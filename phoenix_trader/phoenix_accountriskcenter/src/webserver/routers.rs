use std::sync::Arc;

use crate::dataservice::dbsetup::DbConnection;

use super::httpcontroller::*;

use axum::{
  // body::{Body, Bytes},
  extract::Extension,
  handler::Handler,
  http::StatusCode,
  response::IntoResponse,
  routing::{post, MethodRouter},
  Router,
};
// use axum_extra::middleware::{self, Next};

// use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
// use datacontroller::dbsetup::DbConnection;
// use dbopcontroller::entities::prelude::*;
use tower::ServiceBuilder;

pub fn create_route(dbconn: &DbConnection) -> Router {
  let v1_route = create_v1_route();

  // let dbconn = Arc::new(db);
  let app = Router::new()
    .nest("/api", v1_route)
    // .layer(middleware::from_fn(print_request_response))
    .layer(ServiceBuilder::new().layer(Extension(Arc::new(dbconn.to_owned()))));
  // add a fallback service for handling routes to unknown paths
  // let app = app.fallback(handler_404.into_service());
  app
}

fn create_v1_route() -> Router {
  let app_route = Router::new()
    // .merge(query_ua_his())
    .merge(query_aa())
    .merge(query_account_positions())
    .merge(query_aa_his().merge(query_reset_profit_his()));

  let v1_route = Router::new().nest("/v1", app_route);
  v1_route
}

// fn query_ua_his() -> Router {
//   route("/userassets/his", post(query_user_assets_his))
// }

fn query_aa_his() -> Router {
  route("/accountassets/his", post(query_account_assets_his))
}
//query_account_assets
fn query_aa() -> Router {
  route("/accountassets", post(query_account_assets))
}

fn query_account_positions() -> Router {
  route("/accountpositions", post(query_account_stock_positions))
}

fn query_reset_profit_his() -> Router {
  route("/reset/his", post(query_reset_his))
}

fn route(path: &str, method_router: MethodRouter) -> Router {
  Router::new().route(path, method_router)
}

pub async fn handler_404() -> impl IntoResponse {
  (StatusCode::NOT_FOUND, "nothing to see here")
}

// async fn print_request_response(req: Request<Body>, next: Next<Body>) -> Result<impl IntoResponse, (StatusCode, String)> {
//     let (parts, body) = req.into_parts();
//     let bytes = buffer_and_print("request", body).await?;
//     let req = Request::from_parts(parts, Body::from(bytes));

//     let res = next.run(req).await;

//     let (parts, body) = res.into_parts();
//     let bytes = buffer_and_print("response", body).await?;
//     let res = Response::from_parts(parts, Body::from(bytes));

//     Ok(res)
// }

// async fn buffer_and_print<B>(direction: &str, body: B) -> Result<Bytes, (StatusCode, String)>
// where
//     B: axum::body::HttpBody<Data = Bytes>,
//     B::Error: std::fmt::Display,
// {
//     let bytes = match hyper::body::to_bytes(body).await {
//         Ok(bytes) => bytes,
//         Err(err) => {
//             return Err((StatusCode::BAD_REQUEST, format!("failed to read {} body: {}", direction, err)));
//         }
//     };

//     if let Ok(body) = std::str::from_utf8(&bytes) {
//         log::info!("{} body = {:?}", direction, body);
//     }

//     Ok(bytes)
// }
