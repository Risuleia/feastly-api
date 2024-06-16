use std::env;
use thiserror::Error;
use dotenv::dotenv;
use mongodb::{bson::{self, doc, Document}, options::{ClientOptions, UpdateOptions}, Client, Collection, Database};
use futures::stream::StreamExt;

use crate::models::Recipe;


#[derive(Error, Debug)]
pub enum RecipeError {
    #[error("Database error")]
    DatabaseError(#[from] mongodb::error::Error),
    #[error("Serialization error")]
    SerializationError(#[from] bson::de::Error),
    #[error("Unknown error")]
    Unknown,
}

pub async fn connect() -> Database {
    dotenv().ok();

    let connection_string = env::var("MONGODB_CONNECTION_STRING").expect("MONGODB_CONNECTION_STRING not set");
    let mut client_options = ClientOptions::parse(&connection_string).await.expect("Failed to parse");
    client_options.app_name = Some("Feastly API".to_string());

    let client = Client::with_options(client_options).expect("Failed to initialize MongoDB Client");
    client.database("Recipes")
}


pub async fn create_recipe(collection: &Collection<Recipe>, recipe: &Recipe) -> mongodb::error::Result<()> {
    let filter = doc! { "id": recipe.id.clone() };
    let update = doc! { "$set": recipe.to_document() };

    collection.update_one(filter, update, Some(UpdateOptions::builder().upsert(true).build())).await?;
    Ok(())
}

pub async fn read_recipe(collection: &Collection<Recipe>, id: &str) -> mongodb::error::Result<Option<Recipe>> {
    let filter = doc! { "id": id };
    if let Some(doc) = collection.find_one(filter, None).await? {
        Ok(Some(doc))
    } else {
        Ok(None)
    }
}

pub async fn filter_recipes(collection: &Collection<Recipe>, filter: Document) -> Result<Vec<Recipe>, RecipeError> {
    let mut cursor = collection.find(filter, None).await?;
    let mut recipes: Vec<Recipe> = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(recipe) => {
                recipes.push(recipe)
            },
            Err(e) => return Err(RecipeError::DatabaseError(e))
        }
    }

    Ok(recipes)
}

pub async fn list_recipes(collection: &Collection<Recipe>) -> Result<Vec<Recipe>, RecipeError> {
    let mut cursor = collection.find(None, None).await?;
    let mut recipes: Vec<Recipe> = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(recipe) => {
                recipes.push(recipe)
            },
            Err(e) => return Err(RecipeError::DatabaseError(e))
        }
    }

    Ok(recipes)
}

pub async fn update_recipe(collection: &Collection<Recipe>, id: &str, updated_recipe: Document) -> mongodb::error::Result<()> {
    let filter = doc! { "id": id };
    let update = doc! { "$set": updated_recipe };
    collection.update_one(filter, update, None).await?;
    Ok(())
}

pub async fn delete_recipe(collection: &Collection<Recipe>, id: &str) -> mongodb::error::Result<()> {
    let filter = doc! { "id": id };
    collection.delete_one(filter, None).await?;
    Ok(())
}






// use sqlx::{ migrate::MigrateDatabase, Sqlite, SqlitePool };

// const DB_URL: &str = "sqlite://sqlite.db";

// pub async fn initialize_database() -> Result<SqlitePool, sqlx::Error> {

//     let db = SqlitePool::connect(DB_URL).await.unwrap();

//     if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
//         println!("Creating database {}", DB_URL);

//         match Sqlite::create_database(DB_URL).await {
//             Ok(_) => println!("Database succesfully created"),
//             Err(error) => panic!("Error: {}", error),
//         }
    
//     } else {
//         println!("Database already exists!");
        
//         let result = sqlx::query(
//             "CREATE TABLE IF NOT EXISTS feed (id TEXT PRIMARY KEY NOT NULL, low_res_url TEXT, high_res_url TEXT, caption TEXT, permalink TEXT, timestamp TEXT)"
//         ).execute(&db).await;

//         println!("Create user table result: {:?}", result);
//     }

//     Ok(db)
    
// }

// // pub async fn delete_all_records(pool: &SqlitePool) -> Result<(), sqlx::Error> {
// //     sqlx::query("DELETE FROM feed")
// //         .execute(pool)
// //         .await?;

// //     Ok(())
// // }