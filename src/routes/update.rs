// #[macro_use] 
// extern crate rocket;

// use mongodb::{bson::doc, bson::oid::ObjectId, Collection};
// use rocket::serde::json::Json;
// use rocket::http::Status;
// use crate::models::item::User;
// use crate::db::client::get_db;
use bson::oid::ObjectId;
use rocket::serde::json::Json;
use rocket::put;
use rocket::http::Status;
use crate::models::item::User;
use crate::db::client::get_db;
use mongodb::Collection;
use bson::doc;

#[put("/update_user/<id>", format = "json", data = "<user>")]
pub async fn update_user(id: &str, user: Json<User>) -> Result<Json<User>, Status> {
    let db=get_db().await;

    let collection: Collection<User> = db.collection("users");

    let user_id = match ObjectId::parse_str(&id) {
        Ok(oid) => oid,
        Err(_) => return Err(Status::BadRequest),
    };


    println!("{}",user_id);
    
    let updated_user = doc! {
        "$set": {
            "name": &user.name,
            "location": &user.location,
            "description": &user.description,
        }
    };

    println!("{}",updated_user);

    let filter = doc! { "_id": user_id };

    match collection.update_one(filter, updated_user, None).await {
        Ok(update_result) => {
            if update_result.matched_count == 1 {
                println!("User found and updated");
                Ok(user)
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

// pub async fn update_user(
//     id: &str,
//     body: Json<User>,
// ) -> Result<Json<User>, Status> {
//     let db = get_db().await;
//     let collection: Collection<User> = db.collection("user");

//     // Print the incoming ID
//     println!("Incoming ID: {}", id);

//     // Parse the ObjectId
//     let obj_id = match ObjectId::parse_str(id) {
//         Ok(id) => id,
//         Err(_) => {
//             println!("Failed to parse ObjectId");
//             return Err(Status::BadRequest);
//         }
//     };
//     println!("Parsed ObjectId: {:?}", obj_id);

//     // Construct the filter document
//     let filter = doc! {"_id": obj_id};

//     // Check if the document exists
//     match collection.find_one(filter.clone(), None).await {
//         Ok(Some(_)) => {
//             // Document exists, proceed with update
//             let update = doc! {"$set": {
//                 "name": &body.name,
//                 "location": &body.location,
//                 "description": &body.description
//             }};

//             println!("Filter: {:?}", filter);
//             println!("Update: {:?}", update);

//             // Perform the update
//             match collection.update_one(filter.clone(), update, None).await {
//                 Ok(update_result) => {
//                     println!("Update Result: {:?}", update_result);
//                     if update_result.matched_count == 1 {
//                         match collection.find_one(filter, None).await {
//                             Ok(Some(updated_user)) => Ok(Json(updated_user)),
//                             Ok(None) => {
//                                 println!("Updated document not found");
//                                 Err(Status::NotFound)
//                             }
//                             Err(_) => {
//                                 println!("Error finding updated document");
//                                 Err(Status::InternalServerError)
//                             }
//                         }
//                     } else {
//                         println!("No document matched the filter");
//                         Err(Status::NotFound)
//                     }
//                 }
//                 Err(err) => {
//                     println!("Error updating document: {:?}", err);
//                     Err(Status::InternalServerError)
//                 }
//             }
//         }
//         Ok(None) => {
//             println!("Document does not exist");
//             Err(Status::NotFound)
//         }
//         Err(err) => {
//             println!("Error checking document existence: {:?}", err);
//             Err(Status::InternalServerError)
//         }
//     }
// }
