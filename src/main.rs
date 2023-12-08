use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

#[post("/echo")]
async fn echo_something(request_body: String) -> impl Responder {
    HttpResponse::Ok().body(request_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("manual hello")
}

#[get("/api/v1/")]
async fn json() -> impl Responder {
    HttpResponse::Ok().json("return json here")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo_something)
            .service(json)
            .route("/manual", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
