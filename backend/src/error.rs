use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde_json::json;
#[derive(Debug,thiserror::Error)] pub enum ApiError{#[error("invalid request")]Validation,#[error("unauthorized")]Unauthorized,#[error("not found")]NotFound,#[error("conflict")]Conflict,#[error("internal error")]Internal}
impl ResponseError for ApiError{fn status_code(&self)->StatusCode{match self{Self::Validation=>StatusCode::BAD_REQUEST,Self::Unauthorized=>StatusCode::UNAUTHORIZED,Self::NotFound=>StatusCode::NOT_FOUND,Self::Conflict=>StatusCode::CONFLICT,Self::Internal=>StatusCode::INTERNAL_SERVER_ERROR}}fn error_response(&self)->HttpResponse{HttpResponse::build(self.status_code()).json(json!({"error":self.to_string()}))}}

