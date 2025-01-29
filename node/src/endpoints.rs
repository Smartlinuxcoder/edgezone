use crate::db::AppState;
use serde_derive::{Deserialize, Serialize};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub id: Option<i32>,
    pub name: String,
    pub git_repo: String,
    pub install_cmd: String,
    pub build_cmd: String,
    pub run_cmd: String,
    pub env: Option<String>,
    pub healthcheck_endpoint: String,
    pub healthcheck_timeout: i32,
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
) -> Result<(StatusCode, Json<Project>), StatusCode> {
    let conn = state
        .db
        .connect()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

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
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(project)))
}

pub async fn list_projects(
    State(state): State<AppState>,
) -> Result<Json<Vec<MiniProj>>, StatusCode> {
    let conn = state
        .db
        .connect()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut rows = conn
        .query("SELECT id, name FROM projects", ())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut projects = Vec::new();

    while let Some(row) = rows
        .next()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        let project = MiniProj {
            id: row.get(0).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
            name: row.get(1).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        };
        projects.push(project);
    }

    Ok(Json(projects))
}

pub async fn get_project(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Project>, StatusCode> {
    let conn = state
        .db
        .connect()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut rows = conn
        .query(
            "SELECT id, name, git_repo, install_cmd, build_cmd, run_cmd, env, healthcheck_endpoint, healthcheck_timeout FROM projects WHERE id = ?",
            [id],
        )
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let row = rows
        .next()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(Project {
        id: row.get(0).unwrap(),
        name: row.get(1).unwrap(),
        git_repo: row.get(2).unwrap(),
        install_cmd: row.get(3).unwrap(),
        build_cmd: row.get(4).unwrap(),
        run_cmd: row.get(5).unwrap(),
        env: row.get(6).unwrap(),
        healthcheck_endpoint: row.get(7).unwrap(),
        healthcheck_timeout: row.get(8).unwrap(),
    }))
}

pub async fn update_project(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(project): Json<Project>,
) -> Result<Json<Project>, StatusCode> {
    let conn = state
        .db
        .connect()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

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
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(project))
}

pub async fn deploy(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
) -> Result<(StatusCode, Json<Deployment>), StatusCode> {
    let conn = state
        .db
        .connect()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let deployment = Deployment {
        id: None,
        project_id: project_id.parse().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        commit_hash: String::new(),
        status: 0,
        logs: String::from("..."),
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
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(deployment)))
}

pub async fn delete_project(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let conn = state
        .db
        .connect()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    println!("Deleting project with id: {}", id);
    let id_as_int :i32 = id.parse().map_err(|_| StatusCode::BAD_REQUEST)?;
    conn.execute("DELETE FROM deployments WHERE project_id = ?", [id_as_int])
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    conn.execute("DELETE FROM projects WHERE id = ?", [id_as_int])
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

pub async fn list_deployments(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
) -> Result<Json<Vec<MiniDep>>, StatusCode> {
    let conn = state
        .db
        .connect()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut rows = conn
        .query(
            "SELECT id, project_id, commit_hash, status
             FROM deployments WHERE project_id = ?",
            [project_id],
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let mut deployments = Vec::new();
    while let Some(row) = rows
        .next()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        let deployment = MiniDep {
            id: row.get(0).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
            project_id: row.get(1).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
            commit_hash: row.get(2).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
            status: row.get(3).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        };
        deployments.push(deployment);
    }

    Ok(Json(deployments))
}
pub async fn get_deployment(
    (State(state), Path((project_id, deployment_id))): (State<AppState>, Path<(String, String)>),
) -> Result<Json<Deployment>, StatusCode> {
    let conn = state
        .db
        .connect()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let mut rows = conn
        .query(
            "SELECT id, project_id, commit_hash, status, logs
             FROM deployments WHERE project_id = ? AND id = ?",
            [project_id, deployment_id],
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let row = rows
        .next()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    let deployment = Deployment {
        id: row.get(0).unwrap(),
        project_id: row.get(1).unwrap(),
        commit_hash: row.get(2).unwrap(),
        status: row.get(3).unwrap(),
        logs: row.get(4).unwrap(),
    };
    Ok(Json(deployment))
}
//placeholder
pub async fn restart_deployment(
    Path((project_id, deployment_id)): Path<(String, String)>,
) -> StatusCode {
    StatusCode::OK
}
