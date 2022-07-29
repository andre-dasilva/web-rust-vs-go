use rocket::{get,launch,routes,Rocket,Build};


#[get("/")]
fn index() -> &'static str {
    "Hello World"
}

#[launch]
fn rocket() -> Rocket<Build> {
   rocket::build()
       .mount("/", routes![index]) 
}
