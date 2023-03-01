#[macro_use] extern crate rocket;



fn config_rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![])
}
fn main() {
    config_rocket().launch();
}
