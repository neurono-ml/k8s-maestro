use anyhow::Result;
use notify::{event::EventKind, Event, RecursiveMode, Watcher};
use std::path::Path;
use tokio::signal;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    log::info!("File Observer Sidecar starting...");

    let watch_path = std::env::var("WATCH_PATH").unwrap_or_else(|_| "/data".to_string());
    log::info!("Watching directory: {}", watch_path);

    let (tx, mut rx) = mpsc::unbounded_channel();

    let mut watcher = notify::recommended_watcher(move |res: Result<Event, _>| {
        if let Ok(event) = res {
            log::debug!("Event: {:?}", event);
            if let Err(e) = tx.send(event) {
                log::error!("Failed to send event: {}", e);
            }
        }
    })?;

    watcher.watch(Path::new(&watch_path), RecursiveMode::Recursive)?;

    tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event.kind {
                EventKind::Create(_) => {
                    log::info!("File created: {:?}", event.paths);
                }
                EventKind::Modify(_) => {
                    log::info!("File modified: {:?}", event.paths);
                }
                EventKind::Remove(_) => {
                    log::info!("File removed: {:?}", event.paths);
                }
                _ => {}
            }
        }
    });

    signal::ctrl_c().await.expect("Failed to listen for ctrl+c");
    log::info!("Shutting down file observer sidecar...");

    Ok(())
}
