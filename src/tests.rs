use crate::index;
use rocket::fs::FileServer;

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;

    #[test]
    fn test_index() {
        let rocket = rocket::build().mount("/", routes![index])
            .mount("/static", FileServer::from("static"));
        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::HTML));
        assert!(response.into_string().unwrap().contains("Welcome to my web app!"));
    }
}
