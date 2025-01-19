use crate::page::page_index;
use crate::api::version_info;
use axum::routing::get;
use axum::Router;
///
/// application route configuration
/// 
pub fn create_route() ->Router{
    let  router=Router::new()
        .route("/",get(page_index))
        .route("/api/v1/version",get(version_info));
    
    router
}