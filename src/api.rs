use actix_web::{
    web,
    HttpResponse,
    Responder,
    HttpRequest
};
use mongodb::Database;
use serde::{ Serialize, Deserialize };

use crate::{db::{self, create_recipe, delete_recipe, filter_recipes, read_recipe, update_recipe}, models::{Filters, Recipe}};

#[derive(Deserialize)]
pub struct MultipleQueryParams {
    #[serde(default)]
    limit: Option<usize>,
    #[serde(default)]
    page: Option<usize>
}
#[derive(Deserialize)]
pub struct SingleQueryParams {
    #[serde(default)]
    id: Option<i64>
}
#[derive(Deserialize)]
pub struct Payload {
    recipe: Recipe
}

#[derive(Serialize, Deserialize)]
pub struct PaginatedResult {
    total_pages: usize,
    current_page: usize,
    recipes: Vec<Recipe>
}


pub async fn get_data(
    database: web::Data<Database>,
    req: HttpRequest,
    filters: Option<web::Json<Filters>>,
    params: web::Query<MultipleQueryParams>
) -> impl Responder {
    let auth_header = req.headers().get("Authorization");

    if let Some(header_value) = auth_header {
        if let Ok(header_str) = header_value.to_str() {

            let api_key = header_str.trim();
            let expected_token = "0e89498d5b964f4aa2063ab28eb23a45";

            let limit = params.limit.unwrap_or(5);
            let page = params.page.unwrap_or(1);

            if !api_key.is_empty() && api_key == expected_token {
                let collection = database.collection("Recipes");

                if limit > 0 && page > 0 {
                    return HttpResponse::BadRequest().finish();
                }

                if let Some(filters) = filters {
                    let factored_filters = Filters {
                        query: filters.query.clone(),
                        diets: filters.diets.clone(),
                        max_ready_time: filters.max_ready_time,
                        min_servings: filters.min_servings,
                        cuisines: filters.cuisines.clone(),
                        dish_types: filters.dish_types.clone(),
                        max_calories: filters.max_calories,
                        max_fats: filters.max_carbs,
                        max_carbs: filters.max_carbs,
                        max_glycemic_index: filters.max_glycemic_index,
                        healthy: filters.healthy
                    };

                    let recipes_result = filter_recipes(&collection, factored_filters, page).await;
                    match recipes_result {
                        Ok(recipes) => HttpResponse::Ok().json(recipes),
                        Err(_) => HttpResponse::InternalServerError().finish()
                    };
                } else {
                    if limit > 0 {
                        let recipes_result = db::list_recipes(&collection, Some(limit), None).await;
                        
                        match recipes_result {
                            Ok(recipes) => HttpResponse::Ok().json(recipes),
                            Err(_) => HttpResponse::InternalServerError().finish()
                        };
                    } else {
                        HttpResponse::BadRequest().finish();
                    }
                }

            }

        }
    }

    HttpResponse::Unauthorized().finish()

}

pub async fn get_single_data(
    database: web::Data<Database>,
    req: HttpRequest,
    params: web::Query<SingleQueryParams>
) -> impl Responder {
    let auth_header = req.headers().get("Authorization");

    if let Some(header_value) = auth_header {
        if let Ok(header_str) = header_value.to_str() {

            let api_key = header_str.trim();
            let expected_token = "0e89498d5b964f4aa2063ab28eb23a45";

            if !api_key.is_empty() && api_key == expected_token {
                if params.id.is_some() {
                    let collection = database.collection("Recipes");

                    return match read_recipe(&collection, &params.id.unwrap().to_string()).await {
                        Ok(recipe) => HttpResponse::Ok().json(recipe.unwrap()),
                        Err(_) => HttpResponse::InternalServerError().finish()
                    };
                } else {
                    return HttpResponse::BadRequest().finish();
                }
            }
        }
    }

    HttpResponse::Unauthorized().finish()
}

pub async fn create_data(
    database: web::Data<Database>,
    payload: web::Json<Payload>,
    req: HttpRequest,
) -> impl Responder {
    let auth_header = req.headers().get("Authorization");

    if let Some(header_value) = auth_header {
        if let Ok(header_str) = header_value.to_str() {

            let api_key = header_str.trim();
            let expected_token = "0e89498d5b964f4aa2063ab28eb23a45";

            if !api_key.is_empty() && api_key == expected_token {
                let collection = database.collection("Recipes");

                return match create_recipe(&collection, &payload.recipe).await {
                    Ok(_) => HttpResponse::Created().finish(),
                    Err(_) => HttpResponse::InternalServerError().finish()
                };
            }
        }
    }

    HttpResponse::Unauthorized().finish()
}

pub async fn delete_data(
    database: web::Data<Database>,
    req: HttpRequest,
    params: web::Query<SingleQueryParams>
) -> impl Responder {
    let auth_header = req.headers().get("Authorization");

    if let Some(header_value) = auth_header {
        if let Ok(header_str) = header_value.to_str() {

            let api_key = header_str.trim();
            let expected_token = "0e89498d5b964f4aa2063ab28eb23a45";

            if !api_key.is_empty() && api_key == expected_token {
                if params.id.is_some() {
                    let collection = database.collection("Recipes");

                    return match delete_recipe(&collection, &params.id.unwrap().to_string()).await {
                        Ok(_) => HttpResponse::Accepted().finish(),
                        Err(_) => HttpResponse::InternalServerError().finish()
                    };
                } else {
                    return HttpResponse::BadRequest().finish();
                }
            }
        }
    }

    HttpResponse::Unauthorized().finish()
}

pub async fn update_data(
    database: web::Data<Database>,
    payload: web::Json<Payload>,
    req: HttpRequest,
    params: web::Query<SingleQueryParams>
) -> impl Responder {
    let auth_header = req.headers().get("Authorization");

    if let Some(header_value) = auth_header {
        if let Ok(header_str) = header_value.to_str() {

            let api_key = header_str.trim();
            let expected_token = "0e89498d5b964f4aa2063ab28eb23a45";

            if !api_key.is_empty() && api_key == expected_token {
                if params.id.is_some() {
                    let collection = database.collection("Recipes");
    
                    return match update_recipe(&collection, &params.id.unwrap().to_string(), payload.recipe.to_document()).await {
                        Ok(_) => HttpResponse::Created().finish(),
                        Err(_) => HttpResponse::InternalServerError().finish()
                    };
                } else {
                    return HttpResponse::BadRequest().finish();
                }

            }
        }
    }

    HttpResponse::Unauthorized().finish()
}