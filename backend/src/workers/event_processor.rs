use std::time::Duration;

use redis::AsyncCommands;
use sqlx::PgPool;
use tokio::time::sleep;
use uuid::Uuid;

use crate::error::AppError;
use crate::gamedata::GameDataRegistry;
use crate::models::event::{BuildCompletePayload, ResearchCompletePayload};
use crate::websocket::{WsHub, events::WsEvent};

const POLL_INTERVAL_MS: u64 = 500;
const LOCK_TTL_SECS: u64 = 10;

pub async fn run(
    pool: PgPool,
    redis: redis::Client,
    _registry: GameDataRegistry,
    hub: WsHub,
    universe_id: Uuid,
) {
    tracing::info!(%universe_id, "Event worker started");
    loop {
        if let Err(e) = tick(&pool, &redis, &hub, universe_id).await {
            tracing::error!(%universe_id, "Worker tick error: {e}");
        }
        sleep(Duration::from_millis(POLL_INTERVAL_MS)).await;
    }
}

async fn tick(
    pool: &PgPool,
    redis: &redis::Client,
    hub: &WsHub,
    universe_id: Uuid,
) -> Result<(), AppError> {
    let lock_key = format!("worker_lock:{universe_id}");

    let mut conn = redis.get_async_connection().await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let acquired: bool = conn
        .set_nx(&lock_key, "1")
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    if !acquired {
        return Ok(());
    }

    let _: () = conn
        .expire(&lock_key, LOCK_TTL_SECS as i64)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let events = sqlx::query!(
        r#"
        UPDATE event_queue
        SET status = 'PROCESSING'
        WHERE id IN (
            SELECT id FROM event_queue
            WHERE universe_id = $1
              AND status = 'PENDING'
              AND execution_time <= NOW()
            ORDER BY execution_time
            LIMIT 50
            FOR UPDATE SKIP LOCKED
        )
        RETURNING id, event_type, payload
        "#,
        universe_id,
    )
    .fetch_all(pool)
    .await
    .map_err(AppError::from)?;

    for event in events {
        let result = match event.event_type.as_str() {
            "BUILD_COMPLETE"    => handle_build_complete(pool, hub, &event.payload).await,
            "RESEARCH_COMPLETE" => handle_research_complete(pool, hub, &event.payload).await,
            other => {
                tracing::warn!("Unknown event type: {other}");
                Ok(())
            }
        };

        match result {
            Ok(()) => {
                sqlx::query!(
                    "UPDATE event_queue SET status = 'DONE', processed_at = NOW() WHERE id = $1",
                    event.id,
                )
                .execute(pool)
                .await
                .map_err(AppError::from)?;
            }
            Err(e) => {
                tracing::error!(event_id = %event.id, "Event processing failed: {e}");
                sqlx::query!(
                    "UPDATE event_queue SET status = 'FAILED', error_message = $1 WHERE id = $2",
                    e.to_string(), event.id,
                )
                .execute(pool)
                .await
                .map_err(AppError::from)?;
            }
        }
    }

    let _: () = conn.del(&lock_key).await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(())
}

async fn handle_build_complete(
    pool: &PgPool,
    hub: &WsHub,
    payload: &serde_json::Value,
) -> Result<(), AppError> {
    let p: BuildCompletePayload = serde_json::from_value(payload.clone())
        .map_err(|e| AppError::Internal(e.to_string()))?;

    sqlx::query!(
        r#"
        INSERT INTO buildings (planet_id, building_id, level)
        VALUES ($1, $2, $3)
        ON CONFLICT (planet_id, building_id)
        DO UPDATE SET level = EXCLUDED.level, updated_at = NOW()
        "#,
        p.planet_id, p.building_id, p.level,
    )
    .execute(pool)
    .await
    .map_err(AppError::from)?;

    sqlx::query!(
        "DELETE FROM building_queues WHERE planet_id = $1 AND building_id = $2",
        p.planet_id, p.building_id,
    )
    .execute(pool)
    .await
    .map_err(AppError::from)?;

    hub.send(p.empire_id, &WsEvent::BuildComplete {
        planet_id:   p.planet_id,
        building_id: p.building_id,
        level:       p.level,
    }).await;

    Ok(())
}

async fn handle_research_complete(
    pool: &PgPool,
    hub: &WsHub,
    payload: &serde_json::Value,
) -> Result<(), AppError> {
    let p: ResearchCompletePayload = serde_json::from_value(payload.clone())
        .map_err(|e| AppError::Internal(e.to_string()))?;

    sqlx::query!(
        r#"
        INSERT INTO technologies (empire_id, technology_id, level)
        VALUES ($1, $2, $3)
        ON CONFLICT (empire_id, technology_id)
        DO UPDATE SET level = EXCLUDED.level, updated_at = NOW()
        "#,
        p.empire_id, p.technology_id, p.level,
    )
    .execute(pool)
    .await
    .map_err(AppError::from)?;

    sqlx::query!(
        "DELETE FROM research_queues WHERE empire_id = $1 AND technology_id = $2",
        p.empire_id, p.technology_id,
    )
    .execute(pool)
    .await
    .map_err(AppError::from)?;

    hub.send(p.empire_id, &WsEvent::ResearchComplete {
        technology_id: p.technology_id,
        level:         p.level,
    }).await;

    Ok(())
}
