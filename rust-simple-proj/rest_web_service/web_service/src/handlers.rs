use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;

use crate::{models::Course, state::AppState};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> impl Responder {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

pub async fn new_course(
    new_course_info: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    println!("Received new course");
    let course_count = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.teacher_id == new_course_info.teacher_id)
        .collect::<Vec<Course>>()
        .len();

    let new_course = Course {
        teacher_id: new_course_info.teacher_id,
        id: Some(course_count + 1),
        name: new_course_info.name.clone(),
        time: Some(Utc::now().naive_utc()),
    };
    app_state.courses.lock().unwrap().push(new_course);
    HttpResponse::Ok().json("Course added")
}

pub async fn get_course_for_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<usize>,
) -> HttpResponse {
    let teacher_id: usize = params.0;
    let filtered_courses = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.teacher_id == teacher_id)
        .collect::<Vec<Course>>();

    if filtered_courses.len() > 0 {
        HttpResponse::Ok().json(filtered_courses)
    } else {
        HttpResponse::Ok().json("No courses found for teacher".to_string())
    }
}

pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(usize, usize)>,
) -> HttpResponse {
    let (teacher_id, course_id) = params.0;
    let selected_course = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .find(|course| course.id == Some(course_id) && course.teacher_id == teacher_id)
        .ok_or("course not found");

    if let Ok(course) = selected_course {
        HttpResponse::Ok().json(course)
    } else {
        HttpResponse::Ok().json("Course not found".to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use actix_web::{http::StatusCode, web};

    use crate::{models::Course, state::AppState};

    use super::get_course_detail;
    use super::get_course_for_teacher;
    use super::new_course;

    #[actix_rt::test]
    async fn post_course_test() {
        let course = web::Json(Course {
            teacher_id: 1,
            name: "Test course".into(),
            id: None,
            time: None,
        });
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });

        let resp = new_course(course, app_state).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_courses_test() {
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });
        let teacher_id = web::Path::from(1);
        let resp = get_course_for_teacher(app_state, teacher_id).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[actix_rt::test]
    async fn get_one_course_detail() {
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });
        let params: web::Path<(usize, usize)> = web::Path::from((1, 1));
        let resp = get_course_detail(app_state, params).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
