// use actix_web::{
//     get,
//     web::{Data, Json, Path},
//     Responder, HttpResponse
// };
// use serde::{Deserialize, Serialize};
// use sqlx::{self, FromRow};
// use crate::AppState;

// #[derive(Serialize, FromRow)]
// struct Data<T> {
//     id: i32,
//     data: String,
// }


// #[get("/data")]
// pub async fn fetch_data(state: Data<AppState>) -> impl Responder {
//     match sqlx::query_as::<_, Data>("SELECT id, data FROM data")
//         .fetch_all(&state.db)
//         .await
//     {
//         Ok(data) => HttpResponse::Ok().json(data),
//         Err(_) => HttpResponse::NotFound().json("No data found"),
//     }
// }