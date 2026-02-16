use axum::{
    routing::{get, post},
    Router,
    Json,
    extract::State,
};
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use tower_http::services::ServeDir;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize)]
pub struct AppState {
    pub cpu: f32,
    pub intent: String,
    pub app: String,
    pub pid: i32,
    pub score: i32, 
    pub mode: String, // "Standard", "Focus", "Chill"
}

#[derive(Deserialize)]
pub struct ModeRequest {
    mode: String,
}

pub type SharedState = Arc<RwLock<AppState>>;

pub async fn start_server(state: SharedState) {
    let app = Router::new()
        .route("/api/status", get(get_status))
        .route("/api/mode", post(set_mode))
        .nest_service("/", ServeDir::new("zenith_daemon/static"))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 9999));
    println!("[Daemon] Web Dashboard running at http://{}", addr);

    match axum::Server::bind(&addr).serve(app.into_make_service()).await {
        Ok(_) => {},
        Err(e) => eprintln!("[Server] Error: {}", e),
    }
}

async fn get_status(State(state): State<SharedState>) -> Json<AppState> {
    let data = state.read().unwrap();
    Json(data.clone())
}

async fn set_mode(State(state): State<SharedState>, Json(payload): Json<ModeRequest>) {
    let mut data = state.write().unwrap();
    println!("[Server] Mode switched to: {}", payload.mode);
    data.mode = payload.mode;
}
