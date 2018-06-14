use std;

use mongodb;
use mongodb::db::ThreadedDatabase;
use mongodb::{Client, ThreadedClient};

const MONGO_ADDRESS: &'static str = "candidate.41.mongolayer.com";

pub fn establish_connection() -> std::sync::Arc<mongodb::db::DatabaseInner> {
    let client = Client::connect(MONGO_ADDRESS, 10219)
        .expect("Failed to initialize standalone client.");
    let db = client.db("prueba");
    db.auth("prueba", "prueba")
        .ok().expect("Failed to authorize user 'root'.");

    db
}
