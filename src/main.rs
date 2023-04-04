extern crate dotenvy;
use std::env;
mod routers;
use routers::router;
mod handlers;
use salvo::prelude::*;
mod model;
mod utilities;

use mongodb::Client;
use once_cell::sync::OnceCell;
use salvo::__private::once_cell;



static MONGODB_CLIENT: OnceCell<Client> = OnceCell::new();


pub fn get_mongodb_client() -> &'static Client {
    unsafe { MONGODB_CLIENT.get_unchecked() }
}




#[tokio::main]
async fn main() {
    
 

    dotenvy::dotenv().ok();
    let db_username = env::var("DB_USERNAME").expect("DB_USERNAME is not set in .env file");
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD is not set in .env file");
    let db_name = env::var("DB_NAME").expect("DB_NAME is not set in .env file");
    let db_auth_string = env::var("DB_AUTH_STRING").expect("DB_AUTH_STRING is not set in .env file");
    let db_host = env::var("DB_HOST").expect("DB_HOST is not set in .env file");
    let db_protocall = env::var("DB_PROT").expect("DB_PROT is not set in .env file");

    let db_url =
        format!("{db_protocall}://{db_username}:{db_password}@{db_host}/{db_name}{db_auth_string}").to_owned();
    println!("{:?}", db_url);
    let client = Client::with_uri_str(db_url)
        .await
        .expect("Failed to initialize client.");
    MONGODB_CLIENT.set(client).unwrap();


    let router = router::get_router();
    Server::new(TcpListener::bind("127.0.0.1:8000"))
        .serve(router)
        .await
}
