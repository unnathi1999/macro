use std::str::FromStr;

use crate::get_mongodb_client;


use crate::model::user::{User, CreateResponseObject, ResponseObject, Login, UpdateUser};
use crate::utilities::common::{is_valid_email, check_empty_fields, };

use bcrypt::{hash, DEFAULT_COST, verify};
use bson::oid::ObjectId;
use bson::{doc};

use reqwest::StatusCode;
use salvo::writer::Json;
use salvo::{Request, Response,handler};

#[handler]
pub async fn user_signup(_req: &mut Request, res: &mut Response) {
    let client = get_mongodb_client();
    let mut payload_data = _req.parse_json::<User>().await.unwrap();
    let email = &payload_data.email;

    // let date_added = Utc::now();
    // Check for valid email format
    let is_valid = is_valid_email(&email).await;
    if !is_valid {
        let response_obj = ResponseObject {
            message: "Invalid email.".to_string(),
        };
        res.set_status_code(StatusCode::BAD_REQUEST);
        return res.render(Json(response_obj));
    }

    // Check for missing fields
    let missing_fields = check_empty_fields(&payload_data, &["last_name"]).await;
    if !missing_fields.valid {
        res.set_status_code(StatusCode::CONFLICT);
        let response_obj = ResponseObject {
            message: missing_fields.message,
        };
        return res.render(Json(response_obj));
    }

    let password = &payload_data.password;
    let hashed_password = hash(password, DEFAULT_COST).unwrap();   
    payload_data.password = hashed_password;
    let _result = payload_data.insert(&client).await.unwrap();
    println!("{:?}",_result);
    let inserted_id = _result.inserted_id.as_object_id().unwrap();
    let filter=doc! {"_id": inserted_id};
    // Retrieve list of users
    let _users = User::list(&client,Some(filter)).await.unwrap();

    // Render response with list of users
    let response_obj = CreateResponseObject {
        message: "User is added successfully".to_string(),
        data: inserted_id.to_string()
    };
    res.render(Json(response_obj));
}
#[handler]
pub async fn list_users(_req: &mut Request, res: &mut Response) {
    let client = get_mongodb_client();
    match User::list(&client, None).await {
        Ok(users) => {
            let response_obj = CreateResponseObject {
                message: "List of users".to_string(),
                data: users,
            };
            res.render(Json(response_obj));
        }
        Err(e) => {
            let response_obj = ResponseObject {
                message: format!("Failed to retrieve list of users: {}", e.to_string()),
            };
            res.set_status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(response_obj));
        }
    }
}
#[handler]
pub async fn login(_req: &mut Request, res: &mut Response) {
    let client = get_mongodb_client();
    let payload_data = _req.parse_json::<Login>().await.unwrap();
    println!("{:?}",payload_data);
     // Check for missing fields
     let missing_fields = check_empty_fields(&payload_data, &["last_name"]).await;
     if !missing_fields.valid {
         res.set_status_code(StatusCode::CONFLICT);
         let response_obj = ResponseObject {
             message: missing_fields.message,
         };
         return res.render(Json(response_obj));
     }
    let pipeline = vec![doc! {
        "$match": doc! {
            "$or": [
                doc! {
                    "user_name": &payload_data.user_name
                },
                doc! {
                    "email": &payload_data.user_name
                }
            ]
        }
    }];
    let users = User::aggregate::<User>(&client, pipeline).await.unwrap();
    println!("{:?}",users);
    
   
    // Loop through the list of users and verify the password
    for user in users {
        if let Ok(matching_password) = verify(&payload_data.password, &user.password) {

            if matching_password {
                // Password matches, set success message and return response
                let response_obj = ResponseObject {
                    message: "Login successful.".to_string(),
                };
                return res.render(Json(response_obj));
            }
        }
    }

    // No match found, set error message and return response
    let response_obj = ResponseObject {
        message: "Invalid login credentials.".to_string(),
    };
    res.set_status_code(StatusCode::UNAUTHORIZED);
    res.render(Json(response_obj));
}

#[handler]
pub async fn user_update(_req: &mut Request, res: &mut Response) {
    let client = get_mongodb_client();
    let path = _req
        .param::<String>("id")
        .expect("Error: missing user  ID parameter")
        .to_string();  
  

    let payload_data = _req.parse_json::<UpdateUser>().await.unwrap();


    let filter = doc! { "_id": ObjectId::from_str(&path).unwrap() };
   
 

     // Check for missing fields
     let missing_fields = check_empty_fields(&payload_data, &["last_name"]).await;
     if !missing_fields.valid {
         res.set_status_code(StatusCode::CONFLICT);
         let response_obj = ResponseObject {
             message: missing_fields.message,
         };
         return res.render(Json(response_obj));
     }
    let update_doc = bson::to_document(&payload_data).unwrap();
   
    let _cloned_update_doc = update_doc.clone();
    let _update_result = User::update(&User::default(), &client, filter, update_doc).await.unwrap();
   println!("{:?}",_update_result);
    res.set_status_code(StatusCode::NOT_FOUND);
    let response_obj = ResponseObject {
        message: format!("User with ID {} not found", path),
    };
    return res.render(Json(response_obj));
}
   

#[handler]
pub async fn delete_user(_req: &mut Request, res: &mut Response) {
        let client = get_mongodb_client();
        let path = _req
        .param::<String>("id")
        .expect("Error: missing user  ID parameter")
        .to_string();
    let filter = doc! { "_id": ObjectId::from_str(&path).unwrap() };
    println!("{:?}",filter);
    let deleted_user = User::delete( &client, filter.get_object_id("_id").unwrap()).await.unwrap();
    println!("{:?}",deleted_user);

    if deleted_user.deleted_count == 1 {
        let response_obj = ResponseObject {
            message: "User deleted successfully".to_string(),
        };
        res.render(Json(response_obj));
    } else {
        res.set_status_code(StatusCode::NOT_FOUND);
        let response_obj = ResponseObject {
            message: "User not found".to_string(),
        };
        res.render(Json(response_obj));
    }
}




