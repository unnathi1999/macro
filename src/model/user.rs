use serde::{Serialize, Deserialize};
use motio_macro::{ MongoInsertable,};
use futures_util::stream::TryStreamExt;

#[derive(Debug, Serialize, Deserialize, MongoInsertable)]

pub struct User {
    pub first_name:String,
    pub last_name:Option< String>,
    pub user_name:String,
    pub email: String,
    pub password: String,
    pub phone: String

}
#[derive(Debug, Serialize, Deserialize)]

pub struct CreateResponseObject<T>{
    pub message: String,
     pub data: T
 }
 #[derive(Debug, Serialize, Deserialize)]
pub struct ResponseObject {
    pub message:String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct MissingField{
    pub status: String,
    pub valid:bool,
    pub message: String,
    

}