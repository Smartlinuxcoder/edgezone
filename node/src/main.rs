use axum::{
    routing::{delete, get, post, put}, Router,
};
use clap::Parser;

mod endpoints;
mod db;
mod error;

async fn auto_deploy(state: &db::AppState) {
    let conn = state.db.connect().unwrap();
    
    let mut projects = conn.query(
        "SELECT p.id FROM projects p 
         INNER JOIN deployments d ON d.project_id = p.id 
         WHERE d.status = 3 
         GROUP BY p.id", 
        ()
    ).await.unwrap();

    while let Ok(Some(row)) = projects.next().await {
        let project_id: i32 = row.get(0).unwrap();
        
        if let Err(e) = endpoints::deploy(
            axum::extract::State(state.clone()),
            axum::extract::Path(project_id.to_string())
        ).await {
            eprintln!("Failed to auto-deploy project {}: {:?}", project_id, e);
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value_t = 3000)]
    port: u16,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    let cli = Cli::parse();
    let state = db::init_db().await;
    
    auto_deploy(&state).await;

    let app = Router::new()
        .route("/", get(root))
        .route("/update", post(endpoints::update))
        .route("/info", get(endpoints::info))
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

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", cli.port)).await.unwrap();
    println!("Listening on port {}", cli.port);
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
