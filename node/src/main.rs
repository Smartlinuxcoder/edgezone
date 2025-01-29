use crate::db::AppState;
use axum::{
    extract::State,
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde_derive::{Deserialize, Serialize};

mod projects;
mod db;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = db::init_db().await;

    let app = Router::new()
        .route("/", get(root))
        // Project routes
        .route("/projects", post(projects::create_project))
        .route("/projects", get(projects::list_projects))
        .route("/projects/{id}", get(projects::get_project))
        .route("/projects/{id}", put(projects::update_project))
        .route("/projects/{id}", delete(projects::delete_project))
        // Deployment routes
        .route("/projects/{id}/deploy", post(projects::deploy))
        .route("/projects/{id}/deployments", get(projects::list_deployments))
        .route("/projects/{project_id}/deployments/{deployment_id}", get(projects::get_deployment))
        .route("/projects/{project_id}/deployments/{deployment_id}/restart", post(projects::restart_deployment))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
