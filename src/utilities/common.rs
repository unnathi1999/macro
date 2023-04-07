
use std::collections::{HashMap};

use regex::Regex;

use serde::Serialize;

// use jsonwebtoken::{ encode, errors::Result, EncodingKey, Header};

// use chrono::{Duration, Utc};

use crate::{model::user::{MissingField}};

pub async fn is_valid_email(email: &str) -> bool {
    let re = Regex::new(r"^([a-zA-Z0-9]+)@([a-zA-Z0-9]+)\.([a-zA-Z]{2,5})$").unwrap();
    if !re.is_match(email) {
        return false;
    }


    true
}
pub async fn check_empty_fields<T: Serialize>(
    payload_data: &T,
    field_names: &[&str],
) -> MissingField {
    let user_dict: HashMap<String, serde_json::Value> =
        serde_json::from_value(serde_json::to_value(payload_data).unwrap()).unwrap();
    let mut missing_fields = Vec::new();
    for (field, value) in user_dict {
        if value.is_null() {
            continue;
        }
        match &value {
            serde_json::Value::String(s) => {
                if s.is_empty() && !field_names.contains(&field.as_str()) {
                    missing_fields.push(field.to_string());
                }
            }
            serde_json::Value::Array(a) => {
                if a.is_empty() && !field_names.contains(&field.as_str()) {
                    missing_fields.push(field.to_string());
                }
            }
            serde_json::Value::Object(o) => {
                if o.is_empty() && !field_names.contains(&field.as_str()) {
                    missing_fields.push(field.to_string());
                }
            }
            _ => {}
        }
    }

    let valid = missing_fields.is_empty();
    if valid {
        MissingField {
            status: "success".to_string(),
            valid,
            message: "All fields are filled".to_string(),
        }
    } else {
        let field_names = missing_fields.join(", ");
        let message = format!("{} are missing ", field_names);
        MissingField {
            status: "failed".to_string(),
            valid,
            message,
        }
    }
}
// pub async fn check_unique_fields<T: Serialize>(
//     payload_data: &T,
//     coll: &Collection<Document>,
//     field_names: &[&str],
// ) -> MissingField 
//     {
//     let user_dict: HashMap<String, String> =
//         serde_json::from_value(serde_json::to_value(payload_data).unwrap()).unwrap();
//     let mut duplicate_fields = Vec::new();


//     for (field, value) in user_dict.iter() {
//         if field_names.contains(&field.as_str()) {
//             let query = doc! {field: &value};
//             if let Some(_existing_user) = coll.find_one(Some(query), None).await.unwrap() {
//                 duplicate_fields.push(field.to_string());
//             }
//         }
//     }
//     let valid = duplicate_fields.is_empty();

//     if valid {
//         MissingField {
//             status: "success".to_string(),
//             valid,
//             message: "unique field".to_string(),
//         }
//     } else {
//         let field_names = duplicate_fields.join(", ");
//         let message = format!("{} already exist ", field_names);
//         MissingField {
//             status: "failed".to_string(),
//             valid,
//             message,
//         }
//     }
// }
// pub fn generate_access_token(user_id: u32) -> Result<String> {
//     let expiration = Utc::now() + Duration::minutes(10);
//     let claims = Claims {
//         user_id,
//         exp: expiration.timestamp() as usize,
//         token_type: "access".to_string(),
//     };
//     let secret = "access_secret".as_bytes();
//     let header = Header::default();
//     encode(&header, &claims, &EncodingKey::from_secret(secret))
// }
// pub fn generate_refresh_token(user_id: u32) -> Result<String> {
//     let expiration = Utc::now() + Duration::days(7);
//     let claims = Claims {
//         user_id,
//         exp: expiration.timestamp() as usize,
//         token_type: "refresh".to_string(),
//     };
//     let secret = "refresh_secret".as_bytes();
//     let header = Header::default();
//     encode(&header, &claims, &EncodingKey::from_secret(secret))
// }
