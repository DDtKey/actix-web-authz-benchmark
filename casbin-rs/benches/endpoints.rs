use actix_casbin::casbin::function_map::key_match2;
use actix_casbin::casbin::{CoreApi, DefaultModel, FileAdapter};
use actix_casbin_auth::CasbinService;
use actix_web::{get, http, test, App, HttpResponse};
use casbin_rs_example::middleware::FakeAuth;
use criterion::{criterion_group, criterion_main, Criterion};
use tokio::runtime::Runtime;

criterion_group!(benches, bench_allowed_endpoint, bench_denied_endpoint);
criterion_main!(benches);

fn bench_allowed_endpoint(c: &mut Criterion) {
    let rt = get_async_runtime();

    c.bench_function("Allowed Endpoint [casbin-rs]", |b| {
        b.to_async(&rt).iter(|| async {
            let casbin_middleware = get_casbin_middleware().await;

            let mut app = test::init_service(
                App::new()
                    .wrap(casbin_middleware.clone())
                    .wrap(FakeAuth)
                    .service(allowed_endpoint),
            )
            .await;

            let req = test::TestRequest::with_uri("/allowed").to_request();
            let resp = test::call_service(&mut app, req).await;

            assert_eq!(resp.status(), http::StatusCode::OK);
        })
    });
}

fn bench_denied_endpoint(c: &mut Criterion) {
    let rt = get_async_runtime();

    c.bench_function("Denied Endpoint [casbin-rs]", |b| {
        b.to_async(&rt).iter(|| async {
            let casbin_middleware = get_casbin_middleware().await;

            let mut app = test::init_service(
                App::new()
                    .wrap(casbin_middleware.clone())
                    .wrap(FakeAuth)
                    .service(denied_endpoint),
            )
            .await;

            let req = test::TestRequest::with_uri("/denied").to_request();
            let resp = test::call_service(&mut app, req).await;

            assert_eq!(resp.status(), http::StatusCode::FORBIDDEN);
        })
    });
}

#[get("/allowed")]
async fn allowed_endpoint() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}

#[get("/denied")]
async fn denied_endpoint() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}

fn get_async_runtime() -> Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
}

async fn get_casbin_middleware() -> CasbinService {
    let m = DefaultModel::from_file("config/model.conf").await.unwrap();
    //You can also use diesel-adapter or sqlx-adapter
    let a = FileAdapter::new("config/policy.csv");

    let casbin_middleware = CasbinService::new(m, a).await.unwrap();

    casbin_middleware
        .write()
        .await
        .get_role_manager()
        .write()
        .unwrap()
        .matching_fn(Some(key_match2), None);

    casbin_middleware
}
