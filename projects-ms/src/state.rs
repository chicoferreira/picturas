use crate::config::Config;
use crate::tool::amqp::rabbit_controller::RabbitMqController;
use crate::tool::queue::QueuedImageApplyTool;
use axum::extract::ws::Message;
use dashmap::DashMap;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub config: Arc<Config>,
    pub rabbit_mq_controller: Arc<RabbitMqController>,
    pub queued_tools: Arc<DashMap<Uuid, (Uuid, QueuedImageApplyTool)>>, // queue_uuid, (current_tool_uuid, tool)
    pub connected_ws_clients: Arc<DashMap<(Uuid, Uuid), Sender<Message>>>, // project_uuid, user_uuid -> Sender<Message>
}
