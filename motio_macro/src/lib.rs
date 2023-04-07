extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};
#[proc_macro_derive(crud)]
pub fn crud_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Get the name of the struct
    let name = input.ident;
    let str_name = &name.to_string();

    // Get the fields of the struct

    let fields = if let Data::Struct(s) = input.data {
        if let Fields::Named(f) = s.fields {
            f.named
        } else {
            panic!("crud  derive only supports structs with named fields")
        }
    } else {
        panic!("crud derive only supports structs")
    };

    // Extract the names of the fields in the struct
    let _field_names = fields.iter().map(|f| &f.ident);

    // Generate the code for the insert() method
    let insert_code = quote! {
        pub async fn insert(&self, client: &mongodb::Client) -> mongodb::error::Result<mongodb::results::InsertOneResult> {
            let db_name = std::env::var("DB_NAME").expect("DB_NAME is not set in .env file");
            let collection = client.database(&db_name).collection(#str_name);

            let document = bson::to_document(self).unwrap();
            // println!("{:?}",document);
            collection.insert_one(document, None).await
        }
    };

    // Generate the code for the list() method
    let list_code = quote! {
        pub async fn find(client: &mongodb::Client,filter: Option<bson::Document>) -> Result<Vec<Self>, mongodb::error::Error> {
            let db_name = std::env::var("DB_NAME").expect("DB_NAME is not set in .env file");
            let collection = client.database(&db_name).collection(#str_name);
            let mut cursor = collection.find(filter, None).await?;

            let mut results = Vec::new();
            while let Some(doc) = cursor.try_next().await.unwrap() {
                let item = bson::from_document(doc).unwrap();
                results.push(item);
            }

            Ok(results)
        }
    };
    // Generate the code for the update() method
    let update_code = quote! {
        pub async fn update(&self, client: &mongodb::Client, filter: bson::Document, update_doc: bson::Document) -> mongodb::error::Result<mongodb::results::UpdateResult> {
            let db_name = std::env::var("DB_NAME").expect("DB_NAME is not set in .env file");
            let collection = client.database(&db_name).collection::<Self>(#str_name);

            let document = bson::to_document(self).unwrap();
            let update_result = collection.update_one(filter, bson::doc! {"$set": document }, None).await?;

            Ok(update_result)
        }
    };
       // Generate the code for the delete() method
       let delete_code = quote! {
        pub async fn delete(client: &mongodb::Client, id: ObjectId) -> mongodb::error::Result<mongodb::results::DeleteResult> {
            let db_name = std::env::var("DB_NAME").expect("DB_NAME is not set in .env file");
            let collection = client.database(&db_name).collection::<Self>(#str_name);
            let filter = bson::doc! { "_id": id };
            collection.delete_one(filter, None).await
        }};

    // Combine the generated code into a single implementation block

    let expanded = quote! {
            impl #name {
                #insert_code
                #list_code
                #update_code
                #delete_code
            }
        };

    TokenStream::from(expanded)
}


#[proc_macro_derive(MongoAggregate)]
pub fn mongo_aggregate_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let str_name = &name.to_string();

    let pipeline_code = quote! {
        pub async fn aggregate<T>(client: &mongodb::Client, pipeline: Vec<bson::Document>) -> Result<Vec<T>, mongodb::error::Error>
        where
            T: DeserializeOwned,
        {
            let db_name = std::env::var("DB_NAME").expect("DB_NAME is not set in .env file");
            let collection = client.database(&db_name).collection::<T>(#str_name);

            let mut cursor = collection.aggregate(pipeline, None).await.unwrap();
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
            #pipeline_code
        }
    };

    TokenStream::from(expanded)
}


