use axum::{
    routing::{delete, get, post, put}, Router,
};

mod endpoints;
mod db;
mod error;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = db::init_db().await;

    let app = Router::new()
        .route("/", get(root))
        // Project routes
        .route("/projects", post(endpoints::create_project))
        .route("/projects", get(endpoints::list_projects))
        .route("/projects/{id}", get(endpoints::get_project))
        .route("/projects/{id}", put(endpoints::update_project))
        .route("/projects/{id}", delete(endpoints::delete_project))
        // Deployment routes
        .route("/projects/{id}/deploy", post(endpoints::deploy))
        .route("/projects/{id}/deployments", get(endpoints::list_deployments))
        .route("/projects/{project_id}/deployments/{deployment_id}", get(endpoints::get_deployment))
        .route("/projects/{project_id}/deployments/{deployment_id}/restart", post(endpoints::restart_deployment))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
