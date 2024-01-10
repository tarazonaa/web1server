use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde_derive::Serialize;
use std::collections::HashMap;
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
async fn contacts(req: HttpRequest, tera: web::Data<Tera>) -> HttpResponse {
    // Don't use JSON as data
    let query_string = req.query_string();
    let query_params: HashMap<String, String> = web::Query::from_query(query_string)
        .unwrap_or_else(|_| web::Query(HashMap::new()))
        .into_inner();

    let q = query_params.get("q").cloned().unwrap_or("".to_string());

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

    let contacts_to_show = contacts
        .into_iter()
        .filter(|contact| {
            contact.first.to_lowercase().contains(&q.to_lowercase())
                || contact.last.to_lowercase().contains(&q.to_lowercase())
                || contact.phone.to_lowercase().contains(&q.to_lowercase())
                || contact.email.to_lowercase().contains(&q.to_lowercase())
        })
        .collect::<Vec<_>>();

    let mut data = Context::new();
    data.insert("title", "Contacts");
    data.insert("q", &q);
    data.insert("contacts", &contacts_to_show);
    let body = tera.render("index.html", &data).unwrap();
    HttpResponse::Ok().body(body)
}

#[get("/contacts/new")]
async fn new_contact(tera: web::Data<Tera>) -> HttpResponse {
    let mut data = Context::new();
    println!("new contact");
    data.insert("title", "New Contact");
    let body = tera.render("new.html", &data).unwrap();
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
