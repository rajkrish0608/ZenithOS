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
    pub telemetry: bool,
    pub is_pro: bool,
}

#[derive(Deserialize)]
pub struct ModeRequest {
    mode: String,
}

#[derive(Deserialize)]
pub struct TelemetryRequest {
    enabled: bool,
}

pub type SharedState = Arc<RwLock<AppState>>;

pub async fn start_server(state: SharedState) {
    let app = Router::new()
        .route("/api/status", get(get_status))
        .route("/api/mode", post(set_mode))
        .route("/api/telemetry", post(set_telemetry))
        .route("/api/upgrade", post(upgrade_pro))
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
    
    // Pro Feature Gate
    if payload.mode == "Focus" && !data.is_pro {
        println!("[Server] Rejected 'Focus' mode request (Requires Pro License)");
        return;
    }

    println!("[Server] Mode switched to: {}", payload.mode);
    data.mode = payload.mode;
}

async fn set_telemetry(State(state): State<SharedState>, Json(payload): Json<TelemetryRequest>) {
    let mut data = state.write().unwrap();
    if payload.enabled != data.telemetry {
        println!("[Server] Telemetry updated: {}", payload.enabled);
        data.telemetry = payload.enabled;
    }
}

async fn upgrade_pro(State(state): State<SharedState>) {
    let mut data = state.write().unwrap();
    println!("[Server] License Upgraded to PRO!");
    data.is_pro = true;
}
