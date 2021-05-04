use actix_web::dev::ServiceRequest;
use actix_web::{get, http, test, App, Error, HttpResponse};
use actix_web_grants::{proc_macro::has_any_role, GrantsMiddleware};
use criterion::{criterion_group, criterion_main, Criterion};
use tokio::runtime::Runtime;

criterion_group!(benches, bench_allowed_endpoint, bench_denied_endpoint);
criterion_main!(benches);

fn bench_allowed_endpoint(c: &mut Criterion) {
    let rt = get_async_runtime();

    c.bench_function("Allowed Endpoint [actix-web-grants]", |b| {
        b.to_async(&rt).iter(|| async {
            let auth = GrantsMiddleware::with_extractor(fake_extractor);
            let mut app = test::init_service(App::new().wrap(auth).service(allowed_endpoint)).await;

            let req = test::TestRequest::with_uri("/allowed").to_request();
            let resp = test::call_service(&mut app, req).await;

            assert_eq!(resp.status(), http::StatusCode::OK);
        })
    });
}

fn bench_denied_endpoint(c: &mut Criterion) {
    let rt = get_async_runtime();

    c.bench_function("Denied Endpoint [actix-web-grants]", |b| {
        b.to_async(&rt).iter(|| async {
            let auth = GrantsMiddleware::with_extractor(fake_extractor);
            let mut app = test::init_service(App::new().wrap(auth).service(denied_endpoint)).await;

            let req = test::TestRequest::with_uri("/denied").to_request();
            let resp = test::call_service(&mut app, req).await;

            assert_eq!(resp.status(), http::StatusCode::FORBIDDEN);
        })
    });
}

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

async fn fake_extractor(_req: &ServiceRequest) -> Result<Vec<String>, Error> {
    Ok(vec!["ROLE_USER".to_string()])
}

fn get_async_runtime() -> Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
}
