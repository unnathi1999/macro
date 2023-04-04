use crate::handlers::user::{ user_signup};
use salvo::Router;
pub fn get_router() -> Router {
    Router::new()
        .push(Router::with_path("user").post(user_signup))

}