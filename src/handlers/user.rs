use crate::get_mongodb_client;


use crate::model::user::{User, CreateResponseObject, ResponseObject, Login, AccessToken};
use crate::utilities::common::{is_valid_email, check_empty_fields, generate_refresh_token,generate_access_token };

use bcrypt::{hash, DEFAULT_COST, verify};
use bson::{doc, Document};

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
// pub async fn login(req: &mut Request, res: &mut Response) {
//     let client = get_mongodb_client();

//     // Parse the request body into a Login struct
//     let payload_data = match req.parse_json::<Login>().await {
//         Ok(data) => data,
//         Err(_) => {
//             let response_obj = ResponseObject {
//                 message: "Invalid JSON payload".to_string(),
//             };
//             res.set_status_code(StatusCode::BAD_REQUEST);
//             return res.render(Json(response_obj));
//         }
//     };

//     // Extract the email and password from the Login struct
//     let email = &payload_data.email;
//     let password = &payload_data.password;

//     // Check if any required fields are missing from the Login struct
//     let missing_fields = check_empty_fields(&payload_data, &["email", "password"]).await;
//     if !missing_fields.valid {
//         res.set_status_code(StatusCode::BAD_REQUEST);
//         let response_obj = ResponseObject {
//             message: missing_fields.message,
//         };
//         return res.render(Json(response_obj));
//     }

//     // Build the pipeline for the MongoDB aggregation
//     let pipeline = vec![doc! {
//         "$match": doc! {
//             "$or": [
//                 doc! {
//                     "user_name": &payload_data.user_name
//                 },
//                 doc! {
//                     "email": &payload_data.email
//                 }
//             ]
//         }
//     }];

//     let user =  User::aggregate(&client, pipeline).await.unwrap(); 
//     let password = doc.get_str("password").unwrap_or("");
//         if bcrypt::verify(&payload_data.password, &password).unwrap() {
        
//             let user_id = doc.get_i32("user_id").unwrap_or(0);
//             let access_token = generate_access_token(user_id as u32).unwrap();
//             let refresh_token = generate_refresh_token(user_id as u32).unwrap();
//             let access_token_response = AccessToken {
//                 message: "Login successful".to_string(),
//                 access_token,
//                 refresh_token,
//             };
//             return res.render(Json(access_token_response));
//         }
    
//     let response_obj = ResponseObject {
//         message: "Invalid login ".to_string(),
//     };
//     res.set_status_code(StatusCode::UNAUTHORIZED);
//     res.render(Json(response_obj))
//     }