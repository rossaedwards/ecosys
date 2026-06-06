//! f0rg3d in l0v3 by Ross Edwards & Aurphyx
//!
//! AuraFS WebSocket Server
//!
//! Real-time bidirectional communication for live updates, file watching,
//! cluster events, and interactive sessions.

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{debug, error, info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WsEvent {
    FileCreated { path: String },
    FileModified { path: String },
    FileDeleted { path: String },
    NodeJoined { node_id: String },
    NodeLeft { node_id: String },
    ShardReplicated { shard_id: String, node_id: String },
    CacheUpdate { hits: u64, misses: u64 },
}

pub struct WebSocketServer {
    event_tx: broadcast::Sender<WsEvent>,
}

impl WebSocketServer {
    pub fn new() -> Self {
        let (event_tx, _) = broadcast::channel(100);
        
        info!("Initialized WebSocket server");
        
        Self { event_tx }
    }
    
    pub fn broadcast(&self, event: WsEvent) {
        if let Err(e) = self.event_tx.send(event) {
            warn!("Failed to broadcast event: {}", e);
        }
    }
    
    pub fn create_router(self: Arc<Self>) -> Router {
        Router::new()
            .route("/ws", get(ws_handler))
            .with_state(self)
    }
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(ws_server): State<Arc<WebSocketServer>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, ws_server))
}

async fn handle_socket(socket: WebSocket, ws_server: Arc<WebSocketServer>) {
    let (mut sender, mut receiver) = socket.split();
    
    let mut event_rx = ws_server.event_tx.subscribe();
    
    // Spawn task to send events to client
    let mut send_task = tokio::spawn(async move {
        while let Ok(event) = event_rx.recv().await {
            let json = serde_json::to_string(&event).unwrap();
            
            if sender.send(Message::Text(json)).await.is_err() {
                break;
            }
        }
    });
    
    // Handle incoming messages from client
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    debug!("Received: {}", text);
                    
                    // Handle client commands here
                    if let Ok(cmd) = serde_json::from_str::<ClientCommand>(&text) {
                        handle_client_command(cmd).await;
                    }
                }
                Message::Close(_) => {
                    info!("WebSocket connection closed");
                    break;
                }
                _ => {}
            }
        }
    });
    
    // Wait for either task to finish
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }
    
    info!("WebSocket connection terminated");
}

#[derive(Debug, Deserialize)]
enum ClientCommand {
    Subscribe { topic: String },
    Unsubscribe { topic: String },
    Ping,
}

async fn handle_client_command(cmd: ClientCommand) {
    match cmd {
        ClientCommand::Subscribe { topic } => {
            info!("Client subscribed to: {}", topic);
        }
        ClientCommand::Unsubscribe { topic } => {
            info!("Client unsubscribed from: {}", topic);
        }
        ClientCommand::Ping => {
            debug!("Ping received");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_event_serialization() {
        let event = WsEvent::FileCreated {
            path: "/test/file.txt".to_string(),
        };
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("FileCreated"));
    }
}