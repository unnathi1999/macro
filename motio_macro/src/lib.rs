extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

// use syn::{parse_macro_input, DeriveInput};
use syn::{parse_macro_input, DeriveInput, Data, Fields};
// use bson::doc;

#[proc_macro_derive(MongoInsertable)]
pub fn mongo_insertable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let str_name=&name.to_string();
    let fields = if let Data::Struct(s) = input.data {
        if let Fields::Named(f) = s.fields {
            f.named
        } else {
            panic!("MongoInsertable derive only supports structs with named fields")
        }
    } else {
        panic!("MongoInsertable derive only supports structs")
    };

    // Extract the names of the fields in the struct
    let _field_names = fields.iter().map(|f| &f.ident);

   let insert_code = quote! {
        pub async fn insert(&self, client: &mongodb::Client) -> mongodb::error::Result<mongodb::results::InsertOneResult> {
            let db_name = std::env::var("DB_NAME").expect("DB_NAME is not set in .env file");
            let collection = client.database(&db_name).collection(#str_name);

            let document = bson::to_document(self).unwrap();
            // println!("{:?}",document);
            collection.insert_one(document, None).await
        }
    };
    let list_code = quote! {
        pub async fn list(client: &mongodb::Client) -> Result<Vec<Self>, mongodb::error::Error> {
            let db_name = std::env::var("DB_NAME").expect("DB_NAME is not set in .env file");
            let collection = client.database(&db_name).collection(#str_name);
            let mut cursor = collection.find(None, None).await?;

            let mut results = Vec::new();
            while let Some(doc) = cursor.try_next().await.unwrap() {
                let item = bson::from_document(doc).unwrap();
                results.push(item);
            }

            Ok(results)
        }
    };

    let expanded = quote! {
        impl #name {
            #insert_code
            #list_code
        }
    };

    TokenStream::from(expanded)
}

// #[proc_macro_derive(MongoListable)]
// pub fn mongo_listable_derive(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);

//     let name = input.ident;
//     let str_name = &name.to_string();
//     let list_code = quote! {
//         pub async fn list(client: &mongodb::Client) -> Result<Vec<Self>, mongodb::error::Error> {
//             let db_name = std::env::var("DB_NAME").expect("DB_NAME is not set in .env file");
//             let collection = client.database(&db_name).collection(#str_name);
//             let mut cursor = collection.find(None, None).await?;

//             let mut results = Vec::new();
//             while let Some(doc) = cursor.try_next().await.unwrap() {
//                 let item = bson::from_document(doc).unwrap();
//                 results.push(item);
//             }

//             Ok(results)
//         }
//     };

//     let expanded = quote! {
//         impl #name {
//             #list_code
//         }
//     };

//     TokenStream::from(expanded)
// }
