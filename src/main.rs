mod tests;

#[macro_use]
extern crate rocket;
use rocket::fs::FileServer;



#[get("/")]
fn index() -> rocket::response::content::RawHtml<String> {
    let html = std::fs::read_to_string("static/index.html")
        .unwrap_or_else(|_| String::from("Error loading index.html"));
    rocket::response::content::RawHtml(html)
}

#[rocket::main]
#[allow(unused_must_use)]
async fn main() {
    rocket::build().mount("/", routes![index])
        .mount("/static", FileServer::from("static"))
        .launch().await;
}