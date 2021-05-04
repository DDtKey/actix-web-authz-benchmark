use actix_web::dev::ServiceRequest;
use actix_web::{get, App, Error, HttpResponse, HttpServer};
use actix_web_grants::{proc_macro::has_any_role, GrantsMiddleware};

#[get("/allowed")]
#[has_any_role("USER")]
async fn allowed_endpoint() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}

#[get("/denied")]
#[has_any_role("ADMIN")]
async fn denied_endpoint() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
// Sample application with grant protection based on extracting by your custom function
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let auth = GrantsMiddleware::with_extractor(fake_extractor);

        App::new()
            .wrap(auth)
            .service(allowed_endpoint)
            .service(denied_endpoint)
    })
    .bind("localhost:8081")?
    .run()
    .await
}

async fn fake_extractor(_req: &ServiceRequest) -> Result<Vec<String>, Error> {
    // Here is a place for your code to get user permissions/grants/permissions from a request (token/database/etc)

    // Stub example
    Ok(vec!["ROLE_USER".to_string()])
}
