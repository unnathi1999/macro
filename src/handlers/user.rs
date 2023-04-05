use crate::get_mongodb_client;


use crate::model::user::{User, CreateResponseObject, ResponseObject, Login, AccessToken};
use crate::utilities::common::{is_valid_email, check_empty_fields,  };

use bcrypt::{hash, DEFAULT_COST};
use bson::doc;

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
    let inserted_id = _result.inserted_id.as_object_id().unwrap();
    let filter=doc! {"_id": inserted_id};
    // Retrieve list of users
    let users = User::list(&client,Some(filter)).await.unwrap();

    // Render response with list of users
    let response_obj = CreateResponseObject {
        message: "User is added successfully".to_string(),
        data: users,
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


// #[handler]
// pub async fn user_update(_req: &mut Request, res: &mut Response) {
//     let client = get_mongodb_client();
//     let payload_data = _req.parse_json::<User>().await.unwrap();
//     let user_id = _req.param("id").unwrap();

//     // Update user
//     let filter = doc! {"_id": ObjectId::parse_str(user_id).unwrap()};
//     let update = doc! {"$set": payload_data.to_bson().unwrap()};
//     match User::update(&client, filter, update).await {
//         Ok(_) => {
//             // Retrieve updated user
//             let updated_user = match User::list(&client, Some(filter)).await {
//                 Ok(mut users) => users.remove(0),
//                 Err(_) => {
//                     let response_obj = ResponseObject {
//                         message: "User updated successfully but failed to retrieve updated user".to_string(),
//                     };
//                     res.set_status_code(StatusCode::OK);
//                     return res.render(Json(response_obj));
//                 }
//             };

//             let response_obj = CreateResponseObject {
//                 message: "User updated successfully".to_string(),
//                 data: vec![updated_user.clone()],
//             };
//             res.render(Json(response_obj));
//         }
//         Err(e) => {
//             let response_obj = ResponseObject {
//                 message: format!("Failed to update user: {}", e.to_string()),
//             };
//             res.set_status_code(StatusCode::INTERNAL_SERVER_ERROR);
//             res.render(Json(response_obj));
//         }
//     }
// // }
//


#[handler]
pub async fn login(req: &mut Request, res: &mut Response) {
    let client = get_mongodb_client();
    let payload_data = match req.parse_json::<Login>().await {
        Ok(data) => data,
        Err(_) => {
            let response_obj = ResponseObject {
                message: "Invalid JSON payload".to_string(),
            };
            res.set_status_code(StatusCode::BAD_REQUEST);
            return res.render(Json(response_obj));
        }
    };
      let email = &payload_data.email;
          let password = &payload_data.password;
    let missing_fields = check_empty_fields(&payload_data, &[""]).await;
    // Check if any required fields are missing from the Login struct
    if !missing_fields.valid {
        res.set_status_code(StatusCode::BAD_REQUEST);
        let response_obj = ResponseObject {
            // status: "failed".to_string(),
            message: missing_fields.message,
        };
        return res.render(Json(response_obj));
    }
    let filter = doc! {"email": email};
    let mut users = User::list(&client, Some(filter)).await.unwrap();
   
    
    for user in users {
        let password_hash = user.password;
        if bcrypt::verify(password, &password_hash).unwrap() {
            let access_token_response = ResponseObject {
                message: "Login successful".to_string(),
            };
            return res.render(Json(access_token_response));
        }
    }

    let response_obj = ResponseObject {
        message: "Invalid login".to_string(),
    };
    res.set_status_code(StatusCode::UNAUTHORIZED);
    res.render(Json(response_obj))
}
