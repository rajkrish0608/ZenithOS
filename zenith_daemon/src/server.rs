use axum::{
    routing::get,
    Router,
    Json,
    extract::State,
};
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use tower_http::services::ServeDir;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct AppState {
    pub cpu: f32,
    pub intent: String,
    pub app: String,
    pub pid: i32,
    pub score: i32, // For Phase 3.2
}

pub type SharedState = Arc<RwLock<AppState>>;

pub async fn start_server(state: SharedState) {
    let app = Router::new()
        .route("/api/status", get(get_status))
        .nest_service("/", ServeDir::new("zenith_daemon/static"))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 9999));
    println!("[Daemon] Web Dashboard running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_status(State(state): State<SharedState>) -> Json<AppState> {
    let data = state.read().unwrap();
    Json(data.clone())
}
