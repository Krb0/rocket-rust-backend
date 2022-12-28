mod custom_routes;
mod repository;
use rocket_dyn_templates::Template;
use custom_routes::{profile::profile_router, about::about_router};

#[macro_use] extern crate rocket;


#[get("/")]
fn index() -> &'static str {
    "Hello, El Pepe!"
}

#[launch]
fn rocket() -> _ {

    let profile_routes = profile_router();
    let about_routes = about_router();

    rocket::build().mount("/", routes![index])
    .mount("/profile", profile_routes).mount("/about", about_routes).attach(Template::fairing())
}
