use mongodb::{Collection, Database};
use serde::{Serialize, Deserialize};
use reqwest::{self, Client};

use crate::{api_structs::APIRecipe, db::create_recipe, models::Recipe};

#[derive(Debug, Serialize, Deserialize)]
pub struct APIResponse {
    pub results: Vec<APIRecipe>
}

pub async fn fetch_and_store_recipes(api_key: &str, database: &Database) {
    let recipes: &[Recipe] = &fetch_recipes_from_api(api_key).await.expect("Failed");
    store_recipes_in_db(recipes, database).await;
}

async fn fetch_recipes_from_api(api_key: &str) -> Result<Vec<Recipe>, reqwest::Error> {
    let query_arr = vec!["pasta", "mushroom", "stew", "sandwich", "noodles", "soup", "shake", "smoothie", "sweet", "maggi"];
    
    let client = Client::new();
    let mut recipes = Vec::new();
    
    for query in query_arr {
        let endpoint_url = format!("https://api.spoonacular.com/recipes/complexSearch?query={}&instructionsRequired=true&addRecipeInstructions=true&addRecipeNutrition=true&number=10&fillIngredients=true&apiKey={}", &query, &api_key);
        let response = client.get(endpoint_url)
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let json: Result<APIResponse, reqwest::Error> = response.json::<APIResponse>().await;

                match json {
                    Ok(data) => {
                        let converted_recipes: Vec<Recipe> = data.results.into_iter().map(|api_recipe| api_recipe.to_recipe()).collect();
                        recipes.extend(converted_recipes);
                    }
                    Err(err) => {
                        println!("{:?}", err)

                    }
                }
            }
            status => {
                println!("Status code is {:?}", status)
            }
        }
    }

    Ok(recipes)
}

async fn store_recipes_in_db(recipes: &[Recipe], database: &Database) {
    let collection: Collection<Recipe> = database.collection("Recipes");

    for recipe in recipes {
        let _ = create_recipe(&collection, recipe).await;
    }
}