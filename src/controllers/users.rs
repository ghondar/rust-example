use bson;
use rocket_contrib::{Json, JsonValue};
use serde_json::Value;

use meta;
use models;

#[get("/users")]
pub fn get() -> Json<Vec<Cursor>> {
    let document = models::user::find_all();

    json!(vec![document])

}
