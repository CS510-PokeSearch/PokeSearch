#[macro_use] extern crate rocket;

use rocket::response::content::Json;


#[get("/hello")]
fn hello() -> Json<&'static str> {
  Json("{
    'status': 'success',
    'message': 'Hello API!'
  }")
}

#[get("/")]
fn index() -> &'static str {
    "hi tram"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}