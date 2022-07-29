use rocket::{get,launch,routes,Rocket,Build};

use rocket::serde::{Serialize, json::Json};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Message { 
    text: String
}

#[get("/")]
fn index() -> Json<Message> {
    Json(Message { text: "Hello World".to_string() })
}

#[launch]
fn rocket() -> Rocket<Build> {
   rocket::build()
       .mount("/", routes![index]) 
}
