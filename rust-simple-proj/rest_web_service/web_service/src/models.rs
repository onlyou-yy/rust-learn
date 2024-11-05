use actix_web::web;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Course {
    pub id: Option<u64>,
    pub teacher_id: i32,
    pub name: String,
    pub time: Option<DateTime<Utc>>,
}

// web::Json 和 web::Data 是一种提取器，可以把请求中的数据提取成制定的格式数据
impl From<web::Json<Course>> for Course {
    fn from(value: web::Json<Course>) -> Self {
        Self {
            teacher_id: value.teacher_id,
            id: value.id,
            name: value.name.clone(),
            time: value.time,
        }
    }
}
