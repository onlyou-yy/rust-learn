use crate::models::*;
use sqlx::MySqlPool;

pub async fn get_courses_for_teacher_db(pool: &MySqlPool, teacher_id: i32) -> Vec<Course> {
    let rows = sqlx::query_as!(
        Course,
        r#"SELECT id as "id!", teacher_id, name, time
        FROM course
        WHERE teacher_id = ?"#,
        teacher_id
    )
    .fetch_all(pool)
    .await
    .unwrap();

    rows
}

pub async fn get_course_details_db(pool: &MySqlPool, teacher_id: i32, course_id: i32) -> Course {
    let row = sqlx::query_as!(
        Course,
        r#"SELECT id, teacher_id, name, time
            FROM course
            WHERE teacher_id = ? and id = ?"#,
        teacher_id,
        course_id
    )
    .fetch_one(pool)
    .await
    .unwrap();

    row
}

pub async fn post_new_course_db(pool: &MySqlPool, new_course: Course) -> Course {
    let _insert_res = sqlx::query_as!(
        Course,
        r#"INSERT INTO course (id, teacher_id, name)
            VALUES (?,?,?)"#,
        new_course.id,
        new_course.teacher_id,
        new_course.name,
    )
    .fetch_one(pool)
    .await
    .unwrap();

    let row = sqlx::query_as!(
        Course,
        "SELECT id,teacher_id,name,time FROM course WHERE id=@@IDENTITY"
    )
    .fetch_one(pool)
    .await
    .unwrap();

    row
}
