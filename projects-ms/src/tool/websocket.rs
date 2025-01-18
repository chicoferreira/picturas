use crate::user::AccessTokenClaims;
use crate::AppState;
use axum::extract::ws::{Message, WebSocket};
use axum::extract::{Path, State, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::routing::any;
use axum::{debug_handler, Router};
use serde::Serialize;
use tokio::sync::mpsc::Sender;
use tracing::{error, info};
use uuid::Uuid;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/projects/{project_id}/ws", any(ws_handler))
        .with_state(state)
}

#[debug_handler]
async fn ws_handler(
    ws: WebSocketUpgrade,
    user: AccessTokenClaims,
    State(state): State<AppState>,
    Path(project_uuid): Path<Uuid>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, project_uuid, user, state))
}

async fn handle_socket(
    mut socket: WebSocket,
    project_uuid: Uuid,
    user: AccessTokenClaims,
    state: AppState,
) {
    let span = tracing::info_span!("ws_handler", ?project_uuid, user_uuid = ?user.sub);
    let _guard = span.enter();

    info!("WS User connected");

    let (sender, mut receiver) = tokio::sync::mpsc::channel(10);

    register_ws_client(&state, project_uuid, user.sub, sender).await;

    while let Some(message) = receiver.recv().await {
        if let Err(err) = socket.send(message).await {
            error!(?err, "Error sending message to WS client");
            break;
        }
    }

    unregister_ws_client(&state, project_uuid, user.sub).await;
    info!("WS client disconnected");
}

pub async fn register_ws_client(
    state: &AppState,
    project_uuid: Uuid,
    user_uuid: Uuid,
    sender: Sender<Message>,
) {
    state
        .connected_ws_clients
        .insert((project_uuid, user_uuid), sender);
}

pub async fn unregister_ws_client(state: &AppState, project_uuid: Uuid, user_uuid: Uuid) {
    state
        .connected_ws_clients
        .remove(&(project_uuid, user_uuid));
}

pub async fn send_ws_message<T: Serialize>(
    state: &AppState,
    project_uuid: Uuid,
    user_uuid: Uuid,
    message: T,
) -> Result<(), serde_json::Error> {
    let message = Message::Text(serde_json::to_string(&message)?.into());

    let key = (project_uuid, user_uuid);

    if let Some(sender) = state.connected_ws_clients.get(&key) {
        if sender.send(message).await.is_err() {
            drop(sender);
            state.connected_ws_clients.remove(&key);
        }
    }

    Ok(())
}
