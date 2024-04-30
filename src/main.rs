use actix_files::Files;
use actix_web::{get, web, App, HttpServer};
use reqwest::Client;
use tera::Tera;

pub mod structs;

#[get("/")]
async fn index(tera: web::Data<Tera>) -> Result<impl actix_web::Responder, actix_web::Error> {
    let context = tera::Context::new();
    let rendered = tera.render("index.html", &context).unwrap();
    Ok(actix_web::HttpResponse::Ok().body(rendered))
}

#[get("/meme/image")]
async fn meme_image(
    search: web::Query<structs::MemeImageParams>,
) -> Result<impl actix_web::Responder, actix_web::Error> {
    let client = Client::new();
    let url = structs::MemeTemplate::get_url(&search.id, search.lines)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Error getting meme image url"))?;

    let response = client.get(&url).send().await.map_err(|e| {
        eprintln!("Error getting meme image from request: {:?}", e);
        actix_web::error::ErrorInternalServerError("Error getting meme image from request")
    })?;

    let bytes = response.bytes().await.map_err(|_| {
        actix_web::error::ErrorInternalServerError("Error getting meme image bytes")
    })?;

    Ok(actix_web::HttpResponse::Ok()
        .content_type("image/png")
        .body(bytes))
}

#[get("/meme")]
async fn meme(
    tera: web::Data<Tera>,
    memes: web::Data<structs::Templates>,
) -> Result<impl actix_web::Responder, actix_web::Error> {
    let mut context = tera::Context::new();

    let meme = memes.get_random();
    let url = format!("/meme/image?id={}&lines={}", meme.id, meme.lines);
    context.insert("meme", &url);

    let rendered = tera.render("meme.html", &context).unwrap();
    Ok(actix_web::HttpResponse::Ok().body(rendered))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = match Tera::new("./src/pages/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let memes = structs::Templates::new().await.unwrap();

    println!("Starting server at port 8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .app_data(web::Data::new(memes.clone()))
            .service(index)
            .service(meme)
            .service(meme_image)
            .service(Files::new("/styles", "./src/styles").show_files_listing())
            .service(Files::new("/images", "./src/images").show_files_listing())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
