use crate::page::page_index;
use axum::routing::get;
use axum::Router;
///
/// application route configuration
/// 
pub fn create_route() ->Router{
    let  router=Router::new().route("/",get(page_index));
    router
}