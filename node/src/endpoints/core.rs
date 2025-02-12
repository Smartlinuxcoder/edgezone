use std::{fs, process::Stdio};
use super::super::{db, error::AppError};
use super::Project;
use tokio::io::AsyncBufReadExt;
use super::{STATUS_PENDING, STATUS_INSTALLING, STATUS_BUILDING, STATUS_RUNNING, STATUS_FAILED, STATUS_STOPPED};

pub async fn new_project(path: &String) -> Result<(), AppError> {
    fs::create_dir_all(format!("projects/{}", path))?;
    Ok(())
}

pub async fn stop_deployment(proj_id: i32) -> Result<(), AppError> {
    let db = db::init_db().await;
    let conn = db.db.connect()?;

    // Find and stop all running deployments for this project
    conn.execute(
        "UPDATE deployments SET status = ? WHERE project_id = ? AND status = ?",
        (STATUS_STOPPED, proj_id, STATUS_RUNNING)
    ).await?;

    // Optional: Use process signals to stop the running processes
    // This depends on your OS and requirements
    let output = tokio::process::Command::new("sh")
        .arg("-c")
        .arg(format!("pkill -f \"projects/{}/*/\"", proj_id))
        .output()
        .await?;

    if !output.status.success() {
        println!("Warning: Failed to stop old processes: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}

pub async fn deploy(proj_id: i32, deployment_id: i64) -> Result<(), AppError> {
    // Stop any existing deployments first
    stop_deployment(proj_id).await?;

    let db = db::init_db().await;
    let conn = db.db.connect()?;

    conn.execute(
        "UPDATE deployments SET status = ? WHERE project_id = ? AND status = ?",
        (STATUS_STOPPED, proj_id, STATUS_RUNNING)
    ).await?;
    
    async fn update_logs(conn: &libsql::Connection, deployment_id: i64, new_logs: &str) -> Result<(), AppError> {
        conn.execute(
            "UPDATE deployments SET logs = logs || ? WHERE id = ?",
            (new_logs, deployment_id)
        ).await?;
        Ok(())
    }

    async fn update_status(conn: &libsql::Connection, deployment_id: i64, status: i32) -> Result<(), AppError> {
        conn.execute(
            "UPDATE deployments SET status = ? WHERE id = ?",
            (status, deployment_id)
        ).await?;
        Ok(())
    }

    let mut rows = conn
        .query(
            "SELECT id, name, git_repo, install_cmd, build_cmd, run_cmd, env, healthcheck_endpoint, healthcheck_timeout 
             FROM projects WHERE id = ?",
            [proj_id]
        )
        .await?;

    let row = rows.next().await?.ok_or(AppError::NotFound)?;
    
    let project = Project {
        id: Some(row.get(0)?),
        name: row.get(1)?,
        git_repo: row.get(2)?,
        install_cmd: row.get(3)?,
        build_cmd: row.get(4)?,
        run_cmd: row.get(5)?,
        env: row.get(6)?,
        healthcheck_endpoint: row.get(7)?,
        healthcheck_timeout: row.get(8)?,
    };

    let path = format!("projects/{}/{}", project.name, deployment_id);
    let log_msg = format!("Cloning {} into {}\n", project.git_repo, path);
    update_logs(&conn, deployment_id, &log_msg).await?;
    
    fs::create_dir_all(&path)?;

    let output = tokio::process::Command::new("git")
        .arg("clone")
        .arg(&project.git_repo)
        .arg(&path)
        .output()
        .await?;

    update_logs(&conn, deployment_id, &String::from_utf8_lossy(&output.stdout)).await?;
    if !output.status.success() {
        update_status(&conn, deployment_id, STATUS_FAILED).await?;
        let error = String::from_utf8_lossy(&output.stderr).to_string();
        update_logs(&conn, deployment_id, &format!("Error: {}\n", error)).await?;
        return Err(AppError::Internal(error));
    }

    if let Some(env_vars) = project.env {
        let env_path = format!("{}/{}", path, ".env");
        fs::write(&env_path, env_vars)?;
        update_logs(&conn, deployment_id, "Created .env file\n").await?;
    }

    if let Some(cmd) = project.install_cmd.as_deref() {
        update_status(&conn, deployment_id, STATUS_INSTALLING).await?;
        update_logs(&conn, deployment_id, &format!("Running install command: {}\n", cmd)).await?;
        let install = tokio::process::Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .current_dir(&path)
            .output()
            .await?;

        update_logs(&conn, deployment_id, &String::from_utf8_lossy(&install.stdout)).await?;
        if !install.status.success() {
            update_status(&conn, deployment_id, STATUS_FAILED).await?;
            let error = String::from_utf8_lossy(&install.stderr).to_string();
            update_logs(&conn, deployment_id, &format!("Error: {}\n", error)).await?;
            return Err(AppError::Internal(error));
        }
    }

    if let Some(cmd) = project.build_cmd.as_deref() {
        update_status(&conn, deployment_id, STATUS_BUILDING).await?;
        update_logs(&conn, deployment_id, &format!("Running build command: {}\n", cmd)).await?;
        let build = tokio::process::Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .current_dir(&path)
            .output()
            .await?;

        update_logs(&conn, deployment_id, &String::from_utf8_lossy(&build.stdout)).await?;
        if !build.status.success() {
            update_status(&conn, deployment_id, STATUS_FAILED).await?;
            let error = String::from_utf8_lossy(&build.stderr).to_string();
            update_logs(&conn, deployment_id, &format!("Error: {}\n", error)).await?;
            return Err(AppError::Internal(error));
        }
    }

    update_status(&conn, deployment_id, STATUS_RUNNING).await?;
    let run_cmd = project.run_cmd.ok_or_else(|| AppError::Internal("Run command is required".to_string()))?;
    update_logs(&conn, deployment_id, &format!("Starting service with: {}\n", run_cmd)).await?;
    
    let marker_path = format!("{}/pid", path);
    fs::write(&marker_path, deployment_id.to_string())?;

    let mut run = tokio::process::Command::new("sh")
        .arg("-c")
        .arg(&run_cmd)
        .current_dir(&path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .process_group(0)
        .spawn()?;

    let deployment_id_clone = deployment_id;
    let conn_clone = conn.clone();
    tokio::spawn(async move {
        let stdout = run.stdout.take().unwrap();
        let stderr = run.stderr.take().unwrap();
        
        let mut stdout_reader = tokio::io::BufReader::new(stdout).lines();
        let mut stderr_reader = tokio::io::BufReader::new(stderr).lines();

        loop {
            tokio::select! {
                Ok(Some(line)) = stdout_reader.next_line() => {
                    let _ = update_logs(&conn_clone, deployment_id_clone, &format!("{}\n", line)).await;
                },
                Ok(Some(line)) = stderr_reader.next_line() => {
                    let _ = update_logs(&conn_clone, deployment_id_clone, &format!("Error: {}\n", line)).await;
                },
                else => break,
            }
        }
    });

    Ok(())
}


