use anyhow::{Context, Result};
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::{ws::WebSocket, Filter};

mod api;
mod database;

use api::handle_websocket;
use database::DatabaseApi;

async fn inner_main() -> Result<()> {
    let conn = DatabaseApi::new_temporary().and_then(|db| {
        db.init_database().context("when initializing database")?;
        db.add_fake_data().context("when inserting fake data")?;
        Ok(db)
    })?;

    let store = Arc::new(Mutex::new(conn));
    let store2 = store.clone();
    let db_make = warp::any().map(move || store.clone());

    let ws = warp::path!("api" / "websocket")
        .and(warp::ws())
        .and(db_make)
        .map(|ws: warp::ws::Ws, store| {
            ws.on_upgrade(move |sock: WebSocket| handle_websocket(sock, store))
        });

    let rest = api::filters::status(store2);

    warp::serve(rest.or(ws)).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}

#[tokio::main]
async fn main() {
    mowl::init().unwrap();
    inner_main().await.expect("succcess");
}
