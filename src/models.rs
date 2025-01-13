pub use actix_web::{get, post, web, HttpResponse, Responder};
pub use chrono::NaiveDateTime;
pub use deadpool_postgres::Pool;
pub use serde::{Deserialize, Serialize};
pub use sql_builder::{quote, SqlBuilder};
pub use tokio_postgres::Row;

pub mod operadora;
pub use operadora::*;
pub mod pedagio;
pub use pedagio::*;
pub mod tarifa;
pub use tarifa::*;
