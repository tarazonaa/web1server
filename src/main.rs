use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde_derive::Serialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tera::{Context, Tera};

#[derive(Serialize, Clone)]
struct Contact {
    id: i32,
    first: String,
    last: String,
    phone: String,
    email: String,
}

struct AppState {
    contacts_vec: Arc<Mutex<Vec<Contact>>>,
    tera: Tera,
}

#[get("/")]
async fn index() -> impl Responder {
    web::Redirect::to("/contacts").permanent()
}

#[get("/contacts")]
async fn contacts(req: HttpRequest, data: web::Data<AppState>) -> HttpResponse {
    let query_string = req.query_string();
    let query_params: HashMap<String, String> = web::Query::from_query(query_string)
        .unwrap_or_else(|_| web::Query(HashMap::new()))
        .into_inner();
    let q = query_params.get("q").cloned().unwrap_or("".to_string());
    let contacts = data.contacts_vec.lock().unwrap();
    let contacts_to_show = contacts
        .iter()
        .filter(|contact| {
            contact.first.to_lowercase().contains(&q.to_lowercase())
                || contact.last.to_lowercase().contains(&q.to_lowercase())
                || contact.phone.to_lowercase().contains(&q.to_lowercase())
                || contact.email.to_lowercase().contains(&q.to_lowercase())
        })
        .collect::<Vec<_>>();
    let mut context = Context::new();
    context.insert("title", "Contacts");
    context.insert("q", &q);
    context.insert("contacts", &contacts_to_show);
    let body = data.tera.render("index.html", &context).unwrap();
    HttpResponse::Ok().body(body)
}

#[get("/contacts/new")]
async fn new_contact(data: web::Data<AppState>) -> HttpResponse {
    let mut context = Context::new();
    let contact = Contact {
        id: 0,
        first: "".to_string(),
        last: "".to_string(),
        phone: "".to_string(),
        email: "".to_string(),
    };
    context.insert("title", "New Contact");
    context.insert("contact", &contact);
    let body = data.tera.render("new.html", &context).unwrap();
    HttpResponse::Ok().body(body)
}

#[post("/contacts/new")]
async fn create_contact(
    data: web::Data<AppState>,
    params: web::Form<HashMap<String, String>>,
) -> impl Responder {
    let mut contacts_db = data.contacts_vec.lock().unwrap_or_else(|e| e.into_inner());
    let id = contacts_db.len() as i32 + 1;
    let contact = Contact {
        id,
        first: params
            .get("first")
            .map(|s| s.to_string())
            .unwrap_or_else(|| "DefaultFirstName".to_string()),
        last: params
            .get("last")
            .map(|s| s.to_string())
            .unwrap_or_else(|| "DefaultLastName".to_string()),
        phone: params
            .get("phone")
            .map(|s| s.to_string())
            .unwrap_or_else(|| "DefaultPhone".to_string()),
        email: params
            .get("email")
            .map(|s| s.to_string())
            .unwrap_or_else(|| "DefaultEmail".to_string()),
    };
    contacts_db.push(contact);
    web::Redirect::to("/contacts")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new("./static/**/*.html").unwrap();
    let contacts_db = Arc::new(Mutex::new(vec![
        Contact {
            id: 1,
            first: "John".to_string(),
            last: "Doe".to_string(),
            phone: "555-1234".to_string(),
            email: "john@example.com".to_string(),
        },
        Contact {
            id: 2,
            first: "Jane".to_string(),
            last: "Doe".to_string(),
            phone: "555-4321".to_string(),
            email: "jane@example.com".to_string(),
        },
    ]));
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                contacts_vec: contacts_db.clone(),
                tera: tera.clone(),
            }))
            .service(index)
            .service(contacts)
            .service(new_contact)
            .service(create_contact)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
