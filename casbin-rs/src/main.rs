use actix_casbin_auth::casbin::function_map::key_match2;
use actix_casbin_auth::casbin::{DefaultModel, FileAdapter, Result};
use actix_casbin_auth::CasbinService;
use actix_web::{get, App, HttpResponse, HttpServer};

use actix_casbin::casbin::CoreApi;
use casbin_rs_example::middleware::FakeAuth;

#[get("/allowed")]
async fn allowed_endpoint() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}

#[get("/denied")]
async fn denied_endpoint() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
// Sample application with grant protection based on extracting by your custom function
async fn main() -> Result<()> {
    let m = DefaultModel::from_file("casbin-rs/config/model.conf").await.unwrap();
    //You can also use diesel-adapter or sqlx-adapter
    let a = FileAdapter::new("casbin-rs/config/policy.csv");

    let casbin_middleware = CasbinService::new(m, a).await?;

    casbin_middleware
        .write()
        .await
        .get_role_manager()
        .write()
        .matching_fn(Some(key_match2), None);

    HttpServer::new(move || {
        App::new()
            .wrap(casbin_middleware.clone())
            .wrap(FakeAuth)
            .service(allowed_endpoint)
            .service(denied_endpoint)
    })
    .bind("localhost:8081")?
    .run()
    .await?;

    Ok(())
}
