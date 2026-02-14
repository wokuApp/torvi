use crate::config::database::MongoDB;
use mongodb::bson::doc;
use mongodb::options::IndexOptions;
use mongodb::IndexModel;
use rocket::fairing::AdHoc;

pub fn init() -> AdHoc {
    AdHoc::on_ignite("MongoDB Indices", |rocket| async {
        let mongodb = rocket
            .state::<MongoDB>()
            .expect("MongoDB must be initialized before indices");
        let db = &mongodb.db;

        // Unique index on users.email
        db.collection::<mongodb::bson::Document>("users")
            .create_index(
                IndexModel::builder()
                    .keys(doc! { "email": 1 })
                    .options(IndexOptions::builder().unique(true).build())
                    .build(),
            )
            .await
            .expect("Failed to create unique index on users.email");

        // Index on tournaments.status + created_at
        db.collection::<mongodb::bson::Document>("tournaments")
            .create_index(
                IndexModel::builder()
                    .keys(doc! { "status": 1, "created_at": -1 })
                    .build(),
            )
            .await
            .expect("Failed to create index on tournaments.status+created_at");

        // Index on tournaments.users.user_id
        db.collection::<mongodb::bson::Document>("tournaments")
            .create_index(
                IndexModel::builder()
                    .keys(doc! { "users.user_id": 1 })
                    .build(),
            )
            .await
            .expect("Failed to create index on tournaments.users.user_id");

        // Index on opponents.created_by
        db.collection::<mongodb::bson::Document>("opponents")
            .create_index(
                IndexModel::builder()
                    .keys(doc! { "created_by": 1 })
                    .build(),
            )
            .await
            .expect("Failed to create index on opponents.created_by");

        // Index on images.created_by + created_at
        db.collection::<mongodb::bson::Document>("images")
            .create_index(
                IndexModel::builder()
                    .keys(doc! { "created_by": 1, "created_at": -1 })
                    .build(),
            )
            .await
            .expect("Failed to create index on images.created_by+created_at");

        rocket
    })
}
