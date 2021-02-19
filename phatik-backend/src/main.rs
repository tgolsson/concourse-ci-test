use anyhow::{Context, Result};
use async_std::sync::Mutex;
use database::DatabaseApi;
use std::sync::Arc;

mod api;

type Db = Arc<Mutex<DatabaseApi>>;

async fn inner_main() -> Result<()> {
    let conn = DatabaseApi::new_temporary().and_then(|db| {
        db.init_database().context("when initializing database")?;
        db.add_fake_data().context("when inserting fake data")?;
        Ok(db)
    })?;

    let store = Arc::new(Mutex::new(conn));

    let mut app = tide::with_state(store);

    api::mount(&mut app);
    app.listen("127.0.0.1:3030").await?;

    Ok(())
}

#[async_std::main]
async fn main() {
    use simplelog::*;
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Warn,
        Config::default(),
        TerminalMode::Mixed,
    )])
    .unwrap();

    inner_main().await.expect("succcess");
}
