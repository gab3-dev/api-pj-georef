pub use actix_web::{get, post, put, web, HttpResponse, Responder};
pub use chrono::NaiveDateTime;
pub use serde::{Deserialize, Serialize};
pub use sqlx::{FromRow, PgPool, Row};
pub type Pool = PgPool;

pub use crate::operadora::*;
pub use crate::pedagio::*;
pub use crate::tarifa::*;
