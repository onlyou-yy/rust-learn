use sqlx::MySqlPool;

use crate::errors::MyError;
use crate::models::teacher::{CreateTeacher, Teacher, UpdateTeacher};

pub async fn get_all_teachers_db(pool: &MySqlPool) -> Result<Vec<Teacher>, MyError> {
    let rows = sqlx::query!("SELECT id, name, picture_url, profile FROM teachers")
        .fetch_all(pool)
        .await?;

    let teachers: Vec<Teacher> = rows
        .iter()
        .map(|r| Teacher {
            id: r.id,
            name: r.name.clone(),
            picture_url: r.picture_url.clone(),
            profile: r.profile.clone(),
        })
        .collect();
    match teachers.len() {
        0 => Err(MyError::NotFound("No teachers found".into())),
        _ => Ok(teachers),
    }
}

pub async fn get_teacher_details_db(pool: &MySqlPool, teacher_id: i32) -> Result<Teacher, MyError> {
    let row = sqlx::query!(
        "SELECT id, name, picture_url, profile FROM teachers WHERE id = ?",
        teacher_id,
    )
    .fetch_one(pool)
    .await
    .map(|r| Teacher {
        id: r.id,
        name: r.name,
        picture_url: r.picture_url,
        profile: r.profile,
    })
    .map_err(|_err| MyError::NotFound("Teacher id not found".into()))?;

    Ok(row)
}

pub async fn post_new_teacher_db(
    pool: &MySqlPool,
    new_teacher: CreateTeacher,
) -> Result<Teacher, MyError> {
    let insert_res = sqlx::query!(
        "INSERT INTO teachers (name, picture_url, profile)
        VALUES (?, ?, ?)",
        new_teacher.name,
        new_teacher.picture_url,
        new_teacher.profile
    )
    .execute(pool)
    .await;

    if let Ok(record) = insert_res {
        let row = sqlx::query!(
            "SELECT * FROM teachers WHERE id = ?",
            record.last_insert_id()
        )
        .fetch_one(pool)
        .await?;
        Ok(Teacher {
            id: row.id,
            name: row.name,
            picture_url: row.picture_url,
            profile: row.profile,
        })
    } else {
        Err(MyError::DBError("insert data error".to_string()))
    }
}

pub async fn update_teacher_details_db(
    pool: &MySqlPool,
    teacher_id: i32,
    update_teacher: UpdateTeacher,
) -> Result<Teacher, MyError> {
    let row = sqlx::query!(
        "SELECT id, name, picture_url, profile FROM teachers WHERE id = ?",
        teacher_id,
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| MyError::NotFound("Teacher id not found".into()))?;

    let temp = Teacher {
        id: row.id,
        name: if let Some(name) = Some(update_teacher.name) {
            name
        } else {
            row.name
        },
        picture_url: if let Some(picture_url) = Some(update_teacher.picture_url) {
            picture_url
        } else {
            row.picture_url
        },
        profile: if let Some(profile) = Some(update_teacher.profile) {
            profile
        } else {
            row.profile
        },
    };

    let updated_res = sqlx::query!(
        "UPDATE teachers SET name = ?, picture_url = ?, profile = ? WHERE id = ?",
        temp.name,
        temp.picture_url,
        temp.profile,
        teacher_id,
    )
    .execute(pool)
    .await;

    if let Ok(record) = updated_res {
        let row = sqlx::query!(
            "SELECT * FROM teachers WHERE id = ?",
            record.last_insert_id()
        )
        .fetch_one(pool)
        .await?;
        Ok(Teacher {
            id: row.id,
            name: row.name,
            picture_url: row.picture_url,
            profile: row.profile,
        })
    } else {
        Err(MyError::NotFound("Teacher id not found".into()))
    }
}

pub async fn delete_teacher_db(pool: &MySqlPool, teacher_id: i32) -> Result<String, MyError> {
    let row = sqlx::query!("DELETE FROM teachers WHERE id = ?", teacher_id)
        .execute(pool)
        .await
        .map_err(|_err| MyError::NotFound("Unable to delete teacher".into()))?;

    Ok(format!("Deleted {:?} record", row))
}
