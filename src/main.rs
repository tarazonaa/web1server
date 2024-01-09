use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde_derive::Serialize;
use tera::{Context, Tera};

#[derive(Serialize)]
struct Contact {
    id: i32,
    first: String,
    last: String,
    phone: String,
    email: String,
}

#[get("/")]
async fn index() -> impl Responder {
    web::Redirect::to("/contacts").permanent()
}

#[get("/contacts")]
async fn contacts(tera: web::Data<Tera>) -> HttpResponse {
    // Don't use JSON as data
    let contacts = vec![
        Contact {
            id: 1,
            first: "John".to_string(),
            last: "Doe".to_string(),
            phone: "555-1234".to_string(),
            email: "asdsasda@aasd.com".to_string(),
        },
        Contact {
            id: 2,
            first: "Jane".to_string(),
            last: "Doe".to_string(),
            phone: "555-4321".to_string(),
            email: "asdsasda@gmail.com".to_string(),
        },
        Contact {
            id: 3,
            first: "John".to_string(),
            last: "Smith".to_string(),
            phone: "555-9876".to_string(),
            email: "asdsasd@gmail.com".to_string(),
        },
    ];
    let mut data = Context::new();
    data.insert("title", "Contacts");
    data.insert("q", "this is the search term");
    data.insert("contacts", &contacts);
    let body = tera.render("index.html", &data).unwrap();
    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new("./static/**/*.html").unwrap();
    let tera_ref = web::Data::new(tera);
    HttpServer::new(move || {
        App::new()
            .app_data(tera_ref.clone())
            .service(index)
            .service(contacts)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
