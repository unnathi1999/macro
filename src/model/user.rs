use serde::{Serialize, Deserialize};
extern crate bson;
// use mongodb::bson::oid::ObjectId;


use motio_macro::{ MongoInsertable,MongoAggregate,};
use futures_util::stream::TryStreamExt;

use bson::doc;

use serde::de::DeserializeOwned;

use bson::oid::ObjectId;
#[derive(Debug, Serialize, Default, Deserialize, MongoInsertable,MongoAggregate)]

pub struct User {
  
    pub first_name:String,
    pub last_name:Option< String>,
    pub user_name:String,
    pub email: String,
    pub password: String,
    pub phone: String,

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
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: u32,
    pub exp: usize,
    pub token_type: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Login{
   
    pub email:Option<String>,
pub user_name:Option<String>,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct AccessToken {
    pub message:String,
    pub access_token: String,
    pub refresh_token: String,
    
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUser {
    // pub unique_id:String,
    pub first_name:Option< String>,
    pub last_name:Option< String>,
    pub user_name:Option< String>,
    pub email: Option< String>,
    pub about: Option< String>,
    // pub password: String,
    pub phone: Option< String>
  
}