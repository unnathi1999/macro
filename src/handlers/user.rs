use crate::get_mongodb_client;
use crate::model::user::{User, CreateResponseObject, ResponseObject};
use crate::utilities::common::{is_valid_email, check_empty_fields, };

use reqwest::StatusCode;
use salvo::writer::Json;
use salvo::{Request, Response,handler};




#[handler]
pub async fn user_signup(_req: &mut Request, res: &mut Response) {
    let client = get_mongodb_client();
    let payload_data = _req.parse_json::<User>().await.unwrap();
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

    // Insert new user
    let _result = payload_data.insert(&client).await.unwrap();
    // let inserted_id = result.inserted_id.as_object_id().unwrap();
    
    // Retrieve list of users
    let users = User::list(&client).await.unwrap();

    // Render response with list of users
    let response_obj = CreateResponseObject {
        message: "User is added successfully".to_string(),
        data: users,
    };
    res.render(Json(response_obj));
}
