use rocket_dyn_templates::{Template,context};

#[get("/")]
fn index() -> Template {
    Template::render("about/index", context!{})
}   


pub fn about_router()-> Vec<rocket::Route>{
    routes![index]
}