use std::path::PathBuf;
use axum::response::{Html, IntoResponse};

/// index page 
/// 
pub async  fn page_index() -> impl IntoResponse{
   
   #[cfg(windows)]
   let html= include_str!("resources\\index.html");
   #[cfg(unix)]
   let html= include_str!("resources/index.html");

   Html(html)
}