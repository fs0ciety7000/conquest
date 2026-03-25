use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use futures::{SinkExt, StreamExt};
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

use super::events::WsEvent;

type Tx = broadcast::Sender<String>;

#[derive(Clone, Default)]
pub struct WsHub {
    inner: Arc<RwLock<HashMap<Uuid, Tx>>>,
}

impl WsHub {
    pub fn new() -> Self {
        Self::default()
    }

    /// Send a `WsEvent` to a specific empire.
    pub async fn send(&self, empire_id: Uuid, event: &WsEvent) {
        let json = match serde_json::to_string(event) {
            Ok(j) => j,
            Err(e) => {
                tracing::error!("WsHub serialize error: {e}");
                return;
            }
        };

        let hub = self.inner.read().await;
        if let Some(tx) = hub.get(&empire_id) {
            let _ = tx.send(json);
        }
    }

    /// Register a new WebSocket connection for `empire_id`.
    pub async fn register(&self, empire_id: Uuid, socket: WebSocket) {
        let (tx, rx) = broadcast::channel::<String>(64);

        {
            let mut hub = self.inner.write().await;
            hub.insert(empire_id, tx);
        }

        let (mut ws_tx, mut ws_rx) = socket.split();

        // Forward broadcast messages → WebSocket
        let mut rx = rx;
        let send_task = tokio::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                if ws_tx.send(Message::Text(msg.into())).await.is_err() {
                    break;
                }
            }
        });

        // Drain incoming pings (we don't process client messages yet)
        let recv_task = tokio::spawn(async move {
            while let Some(msg) = ws_rx.next().await {
                match msg {
                    Ok(_) => {}
                    Err(_) => break,
                }
            }
        });

        tokio::select! {
            _ = send_task => {},
            _ = recv_task => {},
        }

        // Cleanup
        let mut hub = self.inner.write().await;
        hub.remove(&empire_id);
    }
}
