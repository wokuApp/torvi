use mongodb::bson::oid::ObjectId;
use std::sync::Arc;

use crate::modules::websocket::broadcaster::TournamentBroadcaster;
use crate::modules::websocket::model::TournamentEvent;

#[test]
fn test_broadcaster_new_creates_empty_rooms() {
    let broadcaster = TournamentBroadcaster::new();
    assert_eq!(broadcaster.room_count(), 0);
}

#[test]
fn test_subscribe_creates_room() {
    let broadcaster = TournamentBroadcaster::new();
    let tid = ObjectId::new();

    let _rx = broadcaster.subscribe(&tid);

    assert_eq!(broadcaster.room_count(), 1);
    assert_eq!(broadcaster.subscriber_count(&tid), 1);
}

#[test]
fn test_broadcast_delivers_to_subscriber() {
    let broadcaster = TournamentBroadcaster::new();
    let tid = ObjectId::new();
    let mut rx = broadcaster.subscribe(&tid);

    let event = TournamentEvent::TournamentPaused;
    broadcaster.broadcast(&tid, event.clone());

    let received = rx.try_recv().unwrap();
    assert_eq!(received, event);
}

#[test]
fn test_broadcast_delivers_to_multiple_subscribers() {
    let broadcaster = TournamentBroadcaster::new();
    let tid = ObjectId::new();
    let mut rx1 = broadcaster.subscribe(&tid);
    let mut rx2 = broadcaster.subscribe(&tid);

    let event = TournamentEvent::TournamentResumed;
    broadcaster.broadcast(&tid, event.clone());

    assert_eq!(rx1.try_recv().unwrap(), event);
    assert_eq!(rx2.try_recv().unwrap(), event);
}

#[test]
fn test_broadcast_isolation_between_tournaments() {
    let broadcaster = TournamentBroadcaster::new();
    let tid1 = ObjectId::new();
    let tid2 = ObjectId::new();
    let mut rx1 = broadcaster.subscribe(&tid1);
    let mut rx2 = broadcaster.subscribe(&tid2);

    let event = TournamentEvent::TournamentPaused;
    broadcaster.broadcast(&tid1, event.clone());

    assert_eq!(rx1.try_recv().unwrap(), event);
    assert!(rx2.try_recv().is_err());
}

#[test]
fn test_broadcast_to_nonexistent_room_is_noop() {
    let broadcaster = TournamentBroadcaster::new();
    let tid = ObjectId::new();

    // Should not panic
    broadcaster.broadcast(&tid, TournamentEvent::TournamentPaused);
    assert_eq!(broadcaster.room_count(), 0);
}

#[test]
fn test_cleanup_removes_empty_rooms() {
    let broadcaster = TournamentBroadcaster::new();
    let tid = ObjectId::new();

    {
        let _rx = broadcaster.subscribe(&tid);
        assert_eq!(broadcaster.room_count(), 1);
    }
    // rx dropped, room has no receivers

    broadcaster.cleanup();
    assert_eq!(broadcaster.room_count(), 0);
}

#[test]
fn test_cleanup_keeps_active_rooms() {
    let broadcaster = TournamentBroadcaster::new();
    let tid = ObjectId::new();
    let _rx = broadcaster.subscribe(&tid); // kept alive

    broadcaster.cleanup();
    assert_eq!(broadcaster.room_count(), 1);
}

#[test]
fn test_subscriber_count() {
    let broadcaster = TournamentBroadcaster::new();
    let tid = ObjectId::new();

    assert_eq!(broadcaster.subscriber_count(&tid), 0);

    let _rx1 = broadcaster.subscribe(&tid);
    assert_eq!(broadcaster.subscriber_count(&tid), 1);

    let _rx2 = broadcaster.subscribe(&tid);
    assert_eq!(broadcaster.subscriber_count(&tid), 2);

    let _rx3 = broadcaster.subscribe(&tid);
    assert_eq!(broadcaster.subscriber_count(&tid), 3);
}

#[test]
fn test_broadcaster_is_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<TournamentBroadcaster>();
    assert_send_sync::<Arc<TournamentBroadcaster>>();
}

#[test]
fn test_room_count_multiple_tournaments() {
    let broadcaster = TournamentBroadcaster::new();
    let _rx1 = broadcaster.subscribe(&ObjectId::new());
    let _rx2 = broadcaster.subscribe(&ObjectId::new());
    let _rx3 = broadcaster.subscribe(&ObjectId::new());

    assert_eq!(broadcaster.room_count(), 3);
}
