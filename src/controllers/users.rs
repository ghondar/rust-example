use bson;
use rocket_contrib::JsonValue;

use meta;
use models;

#[get("/users")]
pub fn get() -> JsonValue {
    let document = models::user::find_all().unwrap();
    let mut users = Vec::new();

    for docu in document {
        let data = bson::from_bson::<meta::user::GetResponse>(bson::Bson::Document(docu.unwrap()));
        match data {
            Ok(user) => {
                users.push(user)
            },
            Err(_e) => {
                json!({
                    "code": 400,
                    "success": false,
                    "data": {},
                    "error": _e.to_string()
                });
            }
        }
    }

    json!({
        "code": 200,
        "success": true,
        "data": users,
        "error": ""
    })
}
