// #![allow(dead_code)]
// #![feature(proc_macro_hygiene, decl_macro)]
// #[macro_use] extern crate rocket_contrib;
// #[macro_use] extern crate rocket;
use rocket::*;

mod blockchain;
mod server;
mod sha;

// fn main() {
//     println!("Hello, Blockchain!");
//     server::rocket.launch();
// }

#[launch]
fn rocket() -> _ {
    server::rocket::build()
}