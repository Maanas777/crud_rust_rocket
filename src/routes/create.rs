use rocket::serde::json::Json;
use rocket::post;
use crate::models::item::User;
use crate::db::client::get_db;
use mongodb::Collection;
use rocket::response::status::Created;

#[post("/create_user", format = "json", data = "<item>")]
pub async fn create_user(item: Json<User>) -> Created<Json<User>> {
    let db = get_db().await;
    let collection: Collection<User> = db.collection("users");
       

    let new_item = User {
        id: None,
        name: item.name.clone(),
        location:item.location.clone(),
        description: item.description.clone(),
    };

    let result = collection.insert_one(new_item, None).await
        .expect("Failed to insert item");

    Created::new(format!("/items/{}", result.inserted_id))
}
