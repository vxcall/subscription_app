use crate::utils::{api_response, app_state, jwt::encode_jwt};
use actix_web::{post, web, Responder};
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, Set};
use sha256::digest;

#[derive(serde::Deserialize)]
struct RegisterModel {
    name: String,
    email: String,
    password: String,
}

#[derive(serde::Deserialize)]
struct LoginModel {
    email: String,
    password: String,
}

#[post("/register")]
pub async fn register(
    app_state: web::Data<app_state::AppState>,
    register_json: web::Json<RegisterModel>,
) -> impl Responder {
    let user_model = entity::user::ActiveModel {
        name: Set(register_json.name.clone()),
        email: Set(register_json.email.clone()),
        password: Set(digest(&register_json.password)),
        ..Default::default()
    }
    .insert(&app_state.db)
    .await
    .unwrap();

    api_response::ApiResponse::new(200, format!("{}", user_model.id))
}

#[post("/login")]
pub async fn login(
    app_state: web::Data<app_state::AppState>,
    login_json: web::Json<LoginModel>,
) -> impl Responder {
    let user = entity::user::Entity::find()
        .filter(
            Condition::all()
                .add(entity::user::Column::Email.eq(login_json.email.clone()))
                .add(entity::user::Column::Password.eq(digest(&login_json.password))),
        )
        .one(&app_state.db)
        .await
        .unwrap();

    if user.is_none() {
        return api_response::ApiResponse::new(401, "User not found".to_string());
    }

    let user_data = user.unwrap();
    let token = encode_jwt(user_data.email, user_data.id).unwrap();

    api_response::ApiResponse::new(200, format!("{{'token': '{}'}}", token))
}
