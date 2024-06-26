use mongodb::bson::{oid::ObjectId, Bson, Document};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Recipe {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub id: String,
    pub title: String,
    pub summary: String,
    pub image: String,
    pub vegetarian: bool,
    pub vegan: bool,
    pub gluten_free: bool,
    pub dairy_free: bool,
    pub ready_in_minutes: i64,
    pub servings: i64,
    pub ingredients: Vec<Ingredient>,
    pub nutrition: Nutrition,
    pub cuisines: Vec<String>,
    pub dish_types: Vec<String>,
    pub diets: Vec<String>,
    pub instructions: Vec<Step>
}

impl Recipe {
    pub fn to_document(&self) -> Document {
        let bson = mongodb::bson::to_bson(self).expect("Failed to convert to BSON");
        if let Bson::Document(document) = bson {
            document
        } else {
            panic!("Expected a BSON document")
        }
    }
    // pub fn from_document(doc: Document) -> Recipe {
    //     mongodb::bson::from_bson(Bson::Document(doc)).expect("Failed to convert from BSON")
    // }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ingredient {
    pub name: String,
    pub amount: f32,
    pub unit: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Nutrition {
    pub nutrients: Vec<Nutrient>,
    pub properties: Vec<Property>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Nutrient {
    pub name: String,
    pub amount: f32,
    pub unit: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Property {
    pub name: String,
    pub amount: f32
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Step {
    pub number: i64,
    pub step: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Filters {
    pub query: String,
    pub diets: Vec<String>,
    pub max_ready_time: i64,
    pub min_servings: i64,
    pub cuisines: Vec<String>,
    pub dish_types: Vec<String>,
    pub max_calories: f32,
    pub max_fats: f32,
    pub max_carbs: f32,
    pub max_glycemic_index: f32,
    pub healthy: bool
}