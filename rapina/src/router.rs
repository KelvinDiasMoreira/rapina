use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::{Method, Request, Response, StatusCode};
use hyper::body::Incoming;

use crate::extract::{PathParams, extract_path_params};
use crate::response::{BoxBody, IntoResponse};
use crate::state::AppState;

type BoxFuture = Pin<Box<dyn Future<Output = Response<BoxBody>> + Send>>;
type HandlerFn =
    Box<dyn Fn(Request<Incoming>, PathParams, Arc<AppState>) -> BoxFuture + Send + Sync>;

struct Route {
    pattern: String,
    handler: HandlerFn,
}

pub struct Router {
    routes: Vec<(Method, Route)>,
}

impl Router {
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    pub fn route<F, Fut, Out>(mut self, method: Method, pattern: &str, handler: F) -> Self
    where
        F: Fn(Request<Incoming>, PathParams, Arc<AppState>) -> Fut + Send + Sync + Clone + 'static,
        Fut: Future<Output = Out> + Send + 'static,
        Out: IntoResponse + 'static,
    {
        let handler = Box::new(
            move |req: Request<Incoming>, params: PathParams, state: Arc<AppState>| {
                let handler = handler.clone();
                Box::pin(async move {
                    let output = handler(req, params, state).await;
                    output.into_response()
                }) as BoxFuture
            },
        );

        let route = Route {
            pattern: pattern.to_string(),
            handler,
        };

        self.routes.push((method, route));
        self
    }

    pub fn get<F, Fut, Out>(self, pattern: &str, handler: F) -> Self
    where
        F: Fn(Request<Incoming>, PathParams, Arc<AppState>) -> Fut + Send + Sync + Clone + 'static,
        Fut: Future<Output = Out> + Send + 'static,
        Out: IntoResponse + 'static,
    {
        self.route(Method::GET, pattern, handler)
    }

    pub fn post<F, Fut, Out>(self, pattern: &str, handler: F) -> Self
    where
        F: Fn(Request<Incoming>, PathParams, Arc<AppState>) -> Fut + Send + Sync + Clone + 'static,
        Fut: Future<Output = Out> + Send + 'static,
        Out: IntoResponse + 'static,
    {
        self.route(Method::POST, pattern, handler)
    }

    pub async fn handle(&self, req: Request<Incoming>, state: &Arc<AppState>) -> Response<BoxBody> {
        let method = req.method().clone();
        let path = req.uri().path().to_string();

        for (route_method, route) in &self.routes {
            if *route_method != method {
                continue;
            }

            if let Some(params) = extract_path_params(&route.pattern, &path) {
                return (route.handler)(req, params, state.clone()).await;
            }
        }

        StatusCode::NOT_FOUND.into_response()
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}
