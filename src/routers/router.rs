use crate::handlers::user::{ user_signup,list_users,login,user_update,delete_user};
use salvo::Router;
pub fn get_router() -> Router {
    Router::new()
        .push(Router::with_path("user").post(user_signup).get(list_users).push(
            Router::with_path("<id>").post(user_update)).push(
                Router::with_path("<id>").delete(delete_user)))
        .push(Router::with_path("login").post(login))
}