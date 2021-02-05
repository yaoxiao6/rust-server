#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("You're {}, {} year old!", name, age)
}

#[post("/", data = "<user_input>")]
fn print_post(user_input: String) -> String{
    println!("Recieved text: {}", user_input);
    user_input
}

fn main() {
    rocket::ignite().mount("/", routes![index, print_post, hello]).launch();
    // rocket::ignite().mount("/rust", routes![hello]).launch();
}
