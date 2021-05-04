pub mod middleware {
    use std::cell::RefCell;
    use std::pin::Pin;
    use std::rc::Rc;
    use std::task::{Context, Poll};

    use actix_web::{
        dev::Service, dev::ServiceRequest, dev::ServiceResponse, dev::Transform, Error, HttpMessage,
    };
    use futures::future::{ok, Future, Ready};

    use actix_casbin_auth::CasbinVals;

    pub struct FakeAuth;

    impl<S: 'static, B> Transform<S> for FakeAuth
    where
        S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
    {
        type Request = ServiceRequest;
        type Response = ServiceResponse<B>;
        type Error = Error;
        type Transform = FakeAuthMiddleware<S>;
        type InitError = ();
        type Future = Ready<Result<Self::Transform, Self::InitError>>;

        fn new_transform(&self, service: S) -> Self::Future {
            ok(FakeAuthMiddleware {
                service: Rc::new(RefCell::new(service)),
            })
        }
    }

    pub struct FakeAuthMiddleware<S> {
        service: Rc<RefCell<S>>,
    }

    impl<S, B> Service for FakeAuthMiddleware<S>
    where
        S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>
            + 'static,
        S::Future: 'static,
        B: 'static,
    {
        type Request = ServiceRequest;
        type Response = ServiceResponse<B>;
        type Error = Error;
        type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

        fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
            self.service.poll_ready(cx)
        }

        fn call(&mut self, req: ServiceRequest) -> Self::Future {
            let mut svc = self.service.clone();

            Box::pin(async move {
                let vals = CasbinVals {
                    subject: String::from("user"),
                    domain: None,
                };
                req.extensions_mut().insert(vals);
                svc.call(req).await
            })
        }
    }
}
