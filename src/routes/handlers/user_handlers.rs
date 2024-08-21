use actix_web::{get, post, web, Responder};
use sea_orm::ActiveModelTrait;
use sea_orm::{EntityTrait, IntoActiveModel, Set};

use crate::utils::{
    api_response::{self, ApiResponse},
    app_state,
    jwt::Claims,
};

#[derive(serde::Serialize, serde::Deserialize)]
struct UpdateUserModel {
    name: String,
}

#[get("")]
pub async fn user(
    app_state: web::Data<app_state::AppState>,
    claim_data: Claims,
) -> Result<ApiResponse, ApiResponse> {
    let user_model = entity::user::Entity::find_by_id(claim_data.id)
        .one(&app_state.db)
        .await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?
        .ok_or(api_response::ApiResponse::new(
            404,
            "User not found".to_owned(),
        ))?;
    Ok(api_response::ApiResponse::new(
        200,
        format!(
            "{{ 'name': '{}', 'email': '{}'}}",
            user_model.name, user_model.email
        ),
    ))
}

#[post("update")]
pub async fn update_user(
    app_state: web::Data<app_state::AppState>,
    user_data: web::Json<UpdateUserModel>,
    claim_data: Claims,
) -> Result<ApiResponse, ApiResponse> {
    let mut user_model = entity::user::Entity::find_by_id(claim_data.id)
        .one(&app_state.db)
        .await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?
        .ok_or(api_response::ApiResponse::new(
            404,
            "User not found".to_owned(),
        ))?
        .into_active_model();

    user_model.name = Set(user_data.name.clone());
    user_model
        .update(&app_state.db)
        .await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;

    Ok(api_response::ApiResponse::new(
        200,
        "User updated".to_owned(),
    ))
}
