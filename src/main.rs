#[macro_use] extern crate rocket;

mod db;
mod models;
mod routes;

use rocket::launch;
use routes::{create::create_user,get_user::get_user,update::update_user, delete::delete_user};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![create_user,get_user,update_user,delete_user])
}
