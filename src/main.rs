use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use handlebars::{DirectorySourceOptions, Handlebars};
use std::collections::HashMap;

#[get("/")]
async fn index() -> impl Responder {
    web::Redirect::to("/contacts").permanent()
}

#[get("/contacts")]
async fn contacts(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    // Don't use JSON as data
    let mut data = HashMap::new();
    data.insert("project_name", "Contacts");
    data.insert("body", "This is the body");
    let body = hb.render("index", &data).unwrap();
    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut handlebars = Handlebars::new();
    let options = DirectorySourceOptions {
        tpl_extension: ".html".into(),
        ..Default::default()
    };
    handlebars
        .register_templates_directory("./static/", options)
        .unwrap();

    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .service(index)
            .service(contacts)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
