// use axum::response::Json;
use serde::{Deserialize, Serialize};
// use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBody<T> {
    pub code: i32,
    pub message: String,
    pub data: T,
}

impl<T> ResponseBody<T> {
    pub fn new(code: i32, message: &str, data: T) -> ResponseBody<T> {
        ResponseBody {
            code: code,
            message: message.to_string(),
            data,
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DataBody<T> {
    pub total: usize,
    pub data: T,
}
impl<T> DataBody<T> {
    pub fn new(total: usize, data: T) -> DataBody<T> {
        DataBody { total: total, data: data }
    }
}

// /// Our app's top level error type.
// enum AppError {
//     /// Something went wrong when calling the user repo.
//     UserRepo(UserRepoError),
// }

// `Json` gives a content-type of `application/json` and works with any type
// that implements `serde::Serialize`
// pub async fn json_model() -> Json<Value> {
//     Json(json!({ "data": 42 }))
// }
