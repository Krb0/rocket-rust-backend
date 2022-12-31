use rocket::serde::Serialize;
use rocket_dyn_templates::{Template,context};


#[get("/<id>")]
fn get_profile(
    id: String
) -> Template{
    let users:[User;2] = [
        User {
            name: String::from("Juan"),
            id: 1,
        },
        User {
            name: String::from("Pedro"),
            id: 2,
        },
    ];
         let user = find_user(id.parse::<i32>().unwrap(),&users);
         if user.is_none(){
            return Template::render("profile/not_found",context!{})
         }

    Template::render("profile/index",context!{user})
}   

#[post("/")]
fn create_profile() -> &'static str {
    "profile created"
}   


#[derive(Serialize)]
struct User {
    name: String,
    id: i32,
}

fn find_user(id:i32,users:&[User])->Option<&User>{
    for user in users{
        if user.id == id{
            return Some(user);
        }
    }
    /* users.iter().find(|user| user.id == id) */
    None
}




pub fn profile_router()-> Vec<rocket::Route>{
    routes![create_profile,get_profile]
}