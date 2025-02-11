use crate::db::AppState;
use crate::error::AppError;
use serde_derive::{Deserialize, Serialize};

use axum::{
    extract::{Path, State},
    http::{StatusCode, HeaderMap, header},
    Json,
};

mod core;

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub id: Option<i32>,
    pub name: String,
    pub git_repo: String,
    pub install_cmd: Option<String>,
    pub build_cmd: Option<String>,
    pub run_cmd: Option<String>,
    pub env: Option<String>,
    pub healthcheck_endpoint: Option<String>,
    pub healthcheck_timeout: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct Deployment {
    pub id: Option<i32>,
    pub project_id: i32,
    pub commit_hash: String,
    pub status: i32,
    pub logs: String,
}

#[derive(Serialize, Deserialize)]
pub struct MiniProj {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct MiniDep {
    pub id: i32,
    pub project_id: i32,
    pub commit_hash: String,
    pub status: i32,
}


pub async fn create_project(
    State(state): State<AppState>,
    Json(project): Json<Project>,
) -> Result<(StatusCode, Json<Project>), AppError> {
    let conn = state.db.connect()?;

    conn.execute(
        "INSERT INTO projects (name, git_repo, install_cmd, build_cmd, run_cmd, healthcheck_endpoint, healthcheck_timeout) VALUES (?, ?, ?, ?, ?, ?, ?)",
        (
            project.name.clone(),
            project.git_repo.clone(),
            project.install_cmd.clone(),
            project.build_cmd.clone(),
            project.run_cmd.clone(),
            project.healthcheck_endpoint.clone(),
            project.healthcheck_timeout,
        ),
    )
    .await?;
    
    core::new_project(&project.name).await?;
    Ok((StatusCode::CREATED, Json(project)))
}

pub async fn list_projects(
    State(state): State<AppState>,
) -> Result<Json<Vec<MiniProj>>, AppError> {
    let conn = state.db.connect()?;
    let mut rows = conn.query("SELECT id, name FROM projects", ()).await?;
    let mut projects = Vec::new();

    while let Some(row) = rows.next().await? {
        let project = MiniProj {
            id: row.get(0)?,
            name: row.get(1)?,
        };
        projects.push(project);
    }

    Ok(Json(projects))
}

pub async fn get_project(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Project>, AppError> {
    let conn = state.db.connect()?;

    let mut rows = conn
        .query(
            "SELECT id, name, git_repo, install_cmd, build_cmd, run_cmd, env, healthcheck_endpoint, healthcheck_timeout FROM projects WHERE id = ?",
            [id],
        )
        .await?;

    let row = rows
        .next()
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(Project {
        id: row.get(0)?,
        name: row.get(1)?,
        git_repo: row.get(2)?,
        install_cmd: row.get(3)?,
        build_cmd: row.get(4)?,
        run_cmd: row.get(5)?,
        env: row.get(6)?,
        healthcheck_endpoint: row.get(7)?,
        healthcheck_timeout: row.get(8)?,
    }))
}

pub async fn update_project(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(project): Json<Project>,
) -> Result<Json<Project>, AppError> {
    let conn = state.db.connect()?;

    conn.execute(
        "UPDATE projects SET name = ?, git_repo = ?, install_cmd = ?, build_cmd = ?, run_cmd = ?, healthcheck_endpoint = ?, healthcheck_timeout = ? WHERE id = ?",
        (
            project.name.clone(),
            project.git_repo.clone(),
            project.install_cmd.clone(),
            project.build_cmd.clone(),
            project.run_cmd.clone(),
            project.healthcheck_endpoint.clone(),
            project.healthcheck_timeout,
            id,
        ),
    )
    .await?;

    Ok(Json(project))
}

pub async fn deploy(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
) -> Result<(StatusCode, HeaderMap, Json<Deployment>), AppError> {
    let conn = state.db.connect()?;
    let deployment = Deployment {
        id: Some(0),  // Set temporary ID
        project_id: project_id.parse()?,
        commit_hash: String::new(),
        status: 0,
        logs: String::from("Starting deployment...\n"),
    };

    conn.execute(
        "INSERT INTO deployments (project_id, commit_hash, status, logs) 
         VALUES (?, ?, ?, ?)",
        (
            deployment.project_id,
            deployment.commit_hash.clone(),
            deployment.status,
            deployment.logs.clone(),
        ),
    )
    .await?;
    let deployment_id = conn.last_insert_rowid();

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());

    let deployment = Deployment {
        id: Some(deployment_id as i32),
        ..deployment
    };

    tokio::spawn(async move {
        if let Err(e) = core::deploy(project_id.parse().unwrap(), deployment_id).await {
            eprintln!("Deployment error: {:?}", e);
        }
    });

    Ok((StatusCode::CREATED, headers, Json(deployment)))
}

pub async fn delete_project(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, AppError> {
    let conn = state.db.connect()?;

    println!("Deleting project with id: {}", id);
    let id_as_int :i32 = id.parse()?;
    conn.execute("DELETE FROM deployments WHERE project_id = ?", [id_as_int])
        .await?;
    conn.execute("DELETE FROM projects WHERE id = ?", [id_as_int])
        .await?;

    Ok(StatusCode::OK)
}

pub async fn list_deployments(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
) -> Result<Json<Vec<MiniDep>>, AppError> {
    let conn = state.db.connect()?;

    let mut rows = conn
        .query(
            "SELECT id, project_id, commit_hash, status
             FROM deployments WHERE project_id = ?",
            [project_id],
        )
        .await?;
    let mut deployments = Vec::new();
    while let Some(row) = rows
        .next()
        .await?
    {
        let deployment = MiniDep {
            id: row.get(0)?,
            project_id: row.get(1)?,
            commit_hash: row.get(2)?,
            status: row.get(3)?,
        };
        deployments.push(deployment);
    }

    Ok(Json(deployments))
}
pub async fn get_deployment(
    (State(state), Path((project_id, deployment_id))): (State<AppState>, Path<(String, String)>),
) -> Result<Json<Deployment>, AppError> {
    let conn = state.db.connect()?;
    let mut rows = conn
        .query(
            "SELECT id, project_id, commit_hash, status, logs
             FROM deployments WHERE project_id = ? AND id = ?",
            [project_id, deployment_id],
        )
        .await?;
    let row = rows
        .next()
        .await?
        .ok_or(AppError::NotFound)?;
    let deployment = Deployment {
        id: row.get(0)?,
        project_id: row.get(1)?,
        commit_hash: row.get(2)?,
        status: row.get(3)?,
        logs: row.get(4)?,
    };
    Ok(Json(deployment))
}
//placeholder
pub async fn restart_deployment(
    Path((_project_id, _deployment_id)): Path<(String, String)>,
) -> StatusCode {
    StatusCode::OK
}
