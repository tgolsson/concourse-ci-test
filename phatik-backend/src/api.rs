/*!

*/
use crate::database::{DatabaseApi, DbEvent, DbTag};
use anyhow::Result;
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::ws::{Message, WebSocket};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
enum PhaticMessage {
    Status(Event),
    Request(ListOptions),
    StatusList(EventList),
    TagRequest {},
    TagList { tags: Vec<String> },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub message: String,
    pub tags: Vec<String>,
    pub app: String,

    pub epoch_seconds: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EventList {
    events: Vec<Event>,
    last_id: i64,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ListOptions {
    last_id: Option<i64>,
    limit: Option<i64>,
}

type Db = Arc<Mutex<DatabaseApi>>;

async fn handle_phatic_message(msg: PhaticMessage, database: &Db) -> Result<Option<PhaticMessage>> {
    let conn = database.lock().await;
    Ok(match msg {
        PhaticMessage::TagList { .. } | PhaticMessage::StatusList(..) => Some(msg),

        PhaticMessage::Request(ListOptions { last_id, limit }) => {
            let (max_id, events) =
                conn.events_after_id(last_id.unwrap_or(-1), limit.unwrap_or(100))?;
            let message = PhaticMessage::StatusList(EventList {
                events,
                last_id: max_id,
            });
            Some(message)
        }

        PhaticMessage::TagRequest {} => {
            let tags = conn.all_tags()?;
            Some(PhaticMessage::TagList { tags })
        }
        PhaticMessage::Status(mut event) => {
            let tags = event
                .tags
                .drain(..)
                .map(|v| conn.register_tag(DbTag { text: v }))
                .collect::<Result<Vec<_>>>()?;

            let db_event = DbEvent {
                message: event.message,
                app: event.app,
                tags,
                epoch_seconds: event.epoch_seconds,
            };

            conn.register_event(db_event)?;
            None
        }
    })
}

pub async fn handle_websocket(mut ws: WebSocket, database: Db) {
    let res: Result<()> = async move {
        while let Some(incoming) = ws.next().await {
            match incoming {
                Ok(msg) => {
                    let p: PhaticMessage = serde_json::from_str(msg.to_str().unwrap())?;

                    if let Some(res) = handle_phatic_message(p, &database).await? {
                        let msg = Message::text(&serde_json::to_string(&res)?);
                        ws.send(msg).await?;
                    }
                }
                Err(e) => eprintln!("websocket error: {:?}", e),
            }
        }
        Ok(())
    }
    .await;

    res.expect("success");
}

/// These are our API handlers, the ends of each filter chain.
/// Notice how thanks to using `Filter::and`, we can define a function
/// with the exact arguments we'd expect from each filter in the chain.
/// No tuples are needed, it's auto flattened for the functions.
mod handlers {
    use super::{Db, DbEvent, DbTag, Event, EventList, ListOptions, Result};
    use std::convert::Infallible;
    use warp::http::StatusCode;

    pub async fn list_status(opts: ListOptions, db: Db) -> Result<impl warp::Reply> {
        // Just return a JSON array of status, applying the limit and offset.
        let conn = db.lock().await;

        let (max_id, events) =
            conn.events_after_id(opts.last_id.unwrap_or(-1), opts.limit.unwrap_or(100))?;
        let status = EventList {
            events,
            last_id: max_id,
        };

        Ok(warp::reply::json(&status))
    }

    pub async fn create_status(mut event: Event, db: Db) -> Result<impl warp::Reply> {
        log::debug!("create_status: {:?}", event);

        let mut conn = db.lock().await;

        let tags = event
            .tags
            .drain(..)
            .map(|v| conn.register_tag(DbTag { text: v }))
            .collect::<Result<Vec<_>>>()?;

        let db_event = DbEvent {
            message: event.message,
            app: event.app,
            tags,
            epoch_seconds: event.epoch_seconds,
        };

        conn.register_event(db_event)?;
        Ok(StatusCode::CREATED)
    }
}

pub mod filters {

    use super::handlers;
    use super::{Db, Event, ListOptions};
    use warp::{reject::Reject, Filter};

    #[derive(Debug)]
    struct InternalServerError;

    impl Reject for InternalServerError {}

    /// The 4 Status filters combined.
    pub fn status(
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        status_list(db.clone()).or(status_create(db.clone()))
    }

    /// GET /status?offset=3&limit=5
    pub fn status_list(
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "status")
            .and(warp::get())
            .and(warp::query::<ListOptions>())
            .and(with_db(db))
            .and_then(|lo, db| async {
                match handlers::list_status(lo, db).await {
                    Ok(v) => Ok(v),
                    Err(_) => Err(warp::reject::custom(InternalServerError)),
                }
            })
    }

    /// POST /status with JSON body
    pub fn status_create(
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "status")
            .and(warp::post())
            .and(json_body())
            .and(with_db(db))
            .and_then(|ev, db| async {
                match handlers::create_status(ev, db).await {
                    Ok(v) => Ok(v),
                    Err(_) => Err(warp::reject::custom(InternalServerError)),
                }
            })
    }

    fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || db.clone())
    }

    fn json_body() -> impl Filter<Extract = (Event,), Error = warp::Rejection> + Clone {
        // When accepting a body, we want a JSON body
        // (and to reject huge payloads)...
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
}
