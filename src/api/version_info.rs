use axum::Json;
use axum::response::IntoResponse;
use satway_build::CompileInfo;
use crate::api::ApiResponse;

/// compile information
///
 pub async fn version_info() -> impl IntoResponse{
    let compile_info=CompileInfo::load_from_str(include_str!("../../target/compile_info.txt"));
    Json(ApiResponse::ok(Box::new(compile_info.unwrap())))
}