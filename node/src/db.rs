use libsql::Builder;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<libsql::Database>,
}

pub async fn init_db() -> AppState {
    let db = Builder::new_local("./data.db").build().await.unwrap();
    create_tables(&db).await;
    AppState { db: Arc::new(db) }
}

async fn create_tables(db: &libsql::Database) {
    let conn = db.connect().unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS projects (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        git_repo TEXT NOT NULL,
        install_cmd TEXT,
        build_cmd TEXT,
        run_cmd TEXT NOT NULL,
        healthcheck_endpoint TEXT,
        healthcheck_timeout INTEGER DEFAULT 5000,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP

    )",
        (),
    )
    .await
    .unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS deployments (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        project_id INTEGER NOT NULL,
        commit_hash TEXT,
        status INTEGER,
        logs TEXT,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY(project_id) REFERENCES projects(id)
    )",
        (),
    )
    .await
    .unwrap();
}
