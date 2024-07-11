use actix_web::{dev::ServiceRequest, get, web, App, Error, HttpResponse, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use actix_web_httpauth::{
    extractors::{
        basic::{self, BasicAuth},
        AuthenticationError,
    },
    headers::www_authenticate,
};

async fn validador(
    req: ServiceRequest,
    auth: BasicAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    if auth.user_id() == "admin" && auth.password().unwrap() == "admin" {
        Ok(req)
    } else {
        Err((
            AuthenticationError::new(www_authenticate::basic::Basic::default()).into(),
            req,
        ))
    }
}

#[get("/publico")]
async fn publico() -> HttpResponse {
    HttpResponse::Ok().body("info publica")
}

#[get("/privado")]
async fn privado(auth: BasicAuth) -> Result<HttpResponse, Error> {
    if auth.user_id() == "miusuario" && auth.password().unwrap() == "12345" {
        Ok(HttpResponse::Ok().body("info privada"))
    } else {
        Err(AuthenticationError::new(www_authenticate::basic::Basic::default()).into())
    }
}

#[get("/confidencial")]
async fn confidencial() -> HttpResponse {
    HttpResponse::Ok().body("info confidencial")
}

#[get("/super-confidencial")]
async fn super_confidencial() -> HttpResponse {
    HttpResponse::Ok().body("info super confidencial")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let auth = HttpAuthentication::basic(validador);
        App::new()
            .service(
                web::scope("/admin")
                    .wrap(auth)
                    .service(confidencial)
                    .service(super_confidencial),
            )
            .app_data(basic::Config::default().realm("privado"))
            .service(publico)
            .service(privado)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
