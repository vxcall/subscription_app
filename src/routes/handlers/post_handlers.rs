use actix_web::{get, post, web};
use chrono::NaiveDateTime;
use chrono::Utc;
use sea_orm::QueryFilter;
use sea_orm::Set;
use uuid::Uuid;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;

use crate::utils::{
    api_response::{self, ApiResponse},
    app_state,
    jwt::Claims,
};

#[derive(serde::Serialize, serde::Deserialize)]
struct CreatePostModel {
    title: String,
    text: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct PostModel {
    pub id: i32,
    pub title: String,
    pub text: String,
    pub uuid: Uuid,
    pub image: String,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
}

#[post("/create")]
pub async fn create_post(
    app_state: web::Data<app_state::AppState>,
    claim: Claims,
    post_model: web::Json<PostModel>,
) -> Result<ApiResponse, ApiResponse> {
    let post_entity = entity::post::ActiveModel {
        title: Set(post_model.title.clone()) ,
        text: Set(post_model.text.clone()),
        uuid: Set(Uuid::new_v4()),
        user_id: Set(claim.id),
        created_at: Set(Utc::now().naive_local()),
        ..Default::default()
    };

    post_entity.insert(&app_state.db).await
    .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;

    Ok(ApiResponse::new(200, "Post created".to_string()))
}

#[get("my-posts")]
pub async fn my_posts(
    app_state: web::Data<app_state::AppState>,
    claim: Claims,
) -> Result<ApiResponse, ApiResponse> {

    let posts: Vec<PostModel> = entity::post::Entity::find()
    .filter(entity::post::Column::UserId.eq(claim.id)).all(&app_state.db).await
    .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?
    .into_iter().map(|post| PostModel {
        id: post.id,
        title: post.title,
        text: post.text,
        uuid: post.uuid,
        image: post.image,
        user_id: post.user_id,
        created_at: post.created_at,
     }).collect();

    let res_str = serde_json::to_string(&posts)
    .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;
    Ok(ApiResponse::new(200, res_str))
}

#[get("all-posts")]
pub async fn all_posts(
    app_state: web::Data<app_state::AppState>,
    claim: Claims,
) -> Result<ApiResponse, ApiResponse> {

    let posts: Vec<PostModel> = entity::post::Entity::find()
    .all(&app_state.db).await
    .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?
    .into_iter().map(|post| PostModel {
        id: post.id,
        title: post.title,
        text: post.text,
        uuid: post.uuid,
        image: post.image,
        user_id: post.user_id,
        created_at: post.created_at,
     }).collect();

    let res_str = serde_json::to_string(&posts)
    .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;
    Ok(ApiResponse::new(200, res_str))
}