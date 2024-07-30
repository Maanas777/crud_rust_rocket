use rocket::serde::json::Json;
use rocket::get;
use crate::models::item::User;
use crate::db::client::get_db;
use mongodb::Collection;
use bson::doc;
use bson::oid::ObjectId;


#[get("/get_user/<id>")]
pub async fn get_user(id:&str)-> Option<Json<User>>{

    let db=get_db().await;
    let collection:Collection<User>=db.collection("users");
   
   let filter=doc! { "_id": ObjectId::parse_str(id).ok()?};
 
 match collection.find_one(filter,None).await.ok()?{

    Some(item)=>Some(Json(item)),
    None=>None,

 }


}
