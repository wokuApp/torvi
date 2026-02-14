use std::sync::Arc;

use dashmap::DashMap;
use mongodb::bson::oid::ObjectId;
use rocket::fairing::AdHoc;
use tokio::sync::broadcast;
use tokio::time::{interval, Duration};

use crate::modules::websocket::model::TournamentEvent;

const CHANNEL_CAPACITY: usize = 100;
const CLEANUP_INTERVAL_SECS: u64 = 300;

pub struct TournamentBroadcaster {
    rooms: DashMap<ObjectId, broadcast::Sender<TournamentEvent>>,
}

impl TournamentBroadcaster {
    pub fn new() -> Self {
        Self {
            rooms: DashMap::new(),
        }
    }

    pub fn subscribe(&self, tournament_id: &ObjectId) -> broadcast::Receiver<TournamentEvent> {
        let sender = self
            .rooms
            .entry(*tournament_id)
            .or_insert_with(|| broadcast::channel(CHANNEL_CAPACITY).0);
        sender.subscribe()
    }

    pub fn broadcast(&self, tournament_id: &ObjectId, event: TournamentEvent) {
        if let Some(sender) = self.rooms.get(tournament_id) {
            let _ = sender.send(event);
        }
    }

    pub fn cleanup(&self) {
        self.rooms.retain(|_, sender| sender.receiver_count() > 0);
    }

    pub fn subscriber_count(&self, tournament_id: &ObjectId) -> usize {
        self.rooms
            .get(tournament_id)
            .map(|sender| sender.receiver_count())
            .unwrap_or(0)
    }

    pub fn room_count(&self) -> usize {
        self.rooms.len()
    }

    pub fn cleanup_fairing(broadcaster: Arc<TournamentBroadcaster>) -> AdHoc {
        AdHoc::on_liftoff("WebSocket Room Cleanup", move |_| {
            Box::pin(async move {
                tokio::spawn(async move {
                    let mut tick = interval(Duration::from_secs(CLEANUP_INTERVAL_SECS));
                    loop {
                        tick.tick().await;
                        broadcaster.cleanup();
                    }
                });
            })
        })
    }
}
