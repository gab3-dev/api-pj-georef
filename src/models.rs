pub use actix_web::{get, post, put, web, HttpResponse, Responder};
pub use chrono::NaiveDateTime;
pub use deadpool_postgres::Pool;
pub use serde::{Deserialize, Serialize};
pub use sql_builder::{quote, SqlBuilder};
pub use tokio_postgres::Row;

pub use crate::operadora::*;
pub use crate::pedagio::*;
pub use crate::tarifa::*;
