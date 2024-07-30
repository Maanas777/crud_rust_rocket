use rocket::delete;
use rocket::http::Status;
use rocket::response::status::NoContent;
use crate::db::client::get_db;
use crate::models::item::User;
use mongodb::Collection;
use bson::doc;
use bson::oid::ObjectId;

#[delete("/delete_user/<id>")]
pub async fn delete_user(id: &str) -> Result<NoContent, Status> {
    let db = get_db().await;
    let collection: Collection<User> = db.collection("users");

    let user_id = match ObjectId::parse_str(&id) {
        Ok(oid) => oid,
        Err(_) => {
            println!("Invalid ID");
            return Err(Status::BadRequest);
        },
    };

    println!("Parsed ObjectId: {}", user_id);

    let filter = doc! { "_id": user_id };

    match collection.delete_one(filter, None).await {
        Ok(delete_result) => {
            if delete_result.deleted_count == 1 {
                println!("User deleted successfully");
                Ok(NoContent)
            } else {
                println!("User not found");
                Err(Status::NotFound)
            }
        },
        Err(e) => {
            println!("Internal Server Error: {:?}", e);
            Err(Status::InternalServerError)
        },
    }
}
