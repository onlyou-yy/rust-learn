use chrono::{DateTime, Utc};
use dotenv::dotenv;
use std::env;

use sqlx::mysql::MySqlPoolOptions;
use std::io;

#[derive(Debug)]
pub struct Course {
    pub id: u64,
    pub teacher_id: i32,
    pub name: String,
    pub time: Option<DateTime<Utc>>,
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件里设置");
    let db_pool = MySqlPoolOptions::new()
        .connect(&database_url)
        .await
        .unwrap();
    let course_rows = sqlx::query_as!(
        Course,
        r#"select id, teacher_id, name, time from course where id = ?"#,
        1
    )
    .fetch_all(&db_pool)
    .await
    .unwrap();

    println!("Courses = {:?}", course_rows);

    Ok(())
}
