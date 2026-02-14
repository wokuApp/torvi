use mongodb::bson::oid::ObjectId;
use std::collections::HashMap;
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

#[test]
fn test_cleanup_on_empty_broadcaster() {
    let broadcaster = TournamentBroadcaster::new();
    broadcaster.cleanup(); // should not panic
    assert_eq!(broadcaster.room_count(), 0);
}

#[test]
fn test_broadcast_after_all_receivers_dropped() {
    let broadcaster = TournamentBroadcaster::new();
    let tid = ObjectId::new();
    let rx = broadcaster.subscribe(&tid);
    drop(rx);

    // Should not panic even though no receivers
    broadcaster.broadcast(&tid, TournamentEvent::TournamentPaused);
}

#[test]
fn test_subscriber_receives_events_in_order() {
    let broadcaster = TournamentBroadcaster::new();
    let tid = ObjectId::new();
    let mut rx = broadcaster.subscribe(&tid);

    broadcaster.broadcast(&tid, TournamentEvent::TournamentPaused);
    broadcaster.broadcast(&tid, TournamentEvent::TournamentResumed);
    broadcaster.broadcast(
        &tid,
        TournamentEvent::VoteCast {
            match_id: "m1".to_string(),
            vote_counts: HashMap::new(),
            total_needed: 3,
        },
    );

    assert_eq!(rx.try_recv().unwrap(), TournamentEvent::TournamentPaused);
    assert_eq!(rx.try_recv().unwrap(), TournamentEvent::TournamentResumed);
    assert!(matches!(
        rx.try_recv().unwrap(),
        TournamentEvent::VoteCast { .. }
    ));
}

#[tokio::test]
async fn test_concurrent_subscribe_and_broadcast() {
    let broadcaster = Arc::new(TournamentBroadcaster::new());
    let tid = ObjectId::new();

    let mut handles = vec![];
    for _ in 0..10 {
        let b = Arc::clone(&broadcaster);
        let t = tid;
        handles.push(tokio::spawn(async move {
            let mut rx = b.subscribe(&t);
            b.broadcast(&t, TournamentEvent::TournamentPaused);
            // Try to receive at least one event
            let _ = tokio::time::timeout(
                tokio::time::Duration::from_millis(100),
                rx.recv(),
            )
            .await;
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }

    // No deadlock occurred
    assert!(broadcaster.room_count() > 0);
}
