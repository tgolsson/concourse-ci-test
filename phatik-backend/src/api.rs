/*!

*/

use anyhow::Result;
use async_std::prelude::*;
use tide::http::StatusCode;
use tide::Request;
use tide_websockets::{Message, WebSocket};

use database::*;
use models::*;

use crate::Db;

pub async fn handle_phatic_message(
    msg: PhaticMessage,
    database: &Db,
) -> Result<Option<PhaticMessage>> {
    let conn = database.lock().await;
    Ok(match msg {
        PhaticMessage::TagList(..) | PhaticMessage::StatusList(..) => Some(msg),

        PhaticMessage::Request(ListOptions { last_id, limit }) => {
            let (max_id, events) =
                conn.events_after_id(last_id.unwrap_or(-1), limit.unwrap_or(100))?;
            let message = PhaticMessage::StatusList(EventList {
                events,
                last_id: max_id,
            });
            Some(message)
        }

        PhaticMessage::TagRequest(..) => {
            let tags = conn.all_tags()?;
            Some(PhaticMessage::TagList(TagList { tags }))
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

pub fn mount(app: &mut tide::Server<Db>) {
    app.at("/api/websocket").get(WebSocket::new(
        |request: Request<Db>, mut stream| async move {
            let store = request.state().clone();
            while let Some(Ok(Message::Text(incoming))) = stream.next().await {
                let p: PhaticMessage = deserialize(&incoming).unwrap();

                if let Some(res) = handle_phatic_message(p, &store).await? {
                    // let msg = Message::text(&serialize(&res)?);

                    stream.send_json(&res).await?;
                }
            }

            Ok(())
        },
    ));

    app.at("api/status")
        .get(|req: Request<Db>| async move {
            let conn = req.state().lock().await;
            let opts: ListOptions = req.query()?;
            let (max_id, events) =
                conn.events_after_id(opts.last_id.unwrap_or(-1), opts.limit.unwrap_or(100))?;
            let status = EventList {
                events,
                last_id: max_id,
            };

            Ok(serialize(&status)?)
        })
        .post(|mut req: Request<Db>| async move {
            let mut event: Event = req.body_json().await.unwrap();

            let conn = req.state().lock().await;
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
            Ok(StatusCode::Created)
        });
}
