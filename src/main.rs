use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, middleware::Logger};


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/posts/new")]
async fn create_post(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(create_post)
            .route("/hey", web::get().to(manual_hello))
            .wrap(Logger::new("%a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}