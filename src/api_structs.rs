use serde::{ Serialize, Deserialize };

use crate::models::{self, Recipe};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct APIRecipe {
    pub id: i64,
    pub title: String,
    pub summary: String,
    pub image: String,
    pub vegetarian: bool,
    pub vegan: bool,
    pub gluten_free: bool,
    pub dairy_free: bool,
    pub ready_in_minutes: i64,
    pub servings: i64,
    pub extended_ingredients: Vec<Ingredient>,
    pub nutrition: Nutrition,
    pub cuisines: Vec<String>,
    pub dish_types: Vec<String>,
    pub diets: Vec<String>,
    pub analyzed_instructions: Vec<Instruction>
}

impl APIRecipe {
    pub fn to_recipe(&self) -> Recipe {
        let ingredients: Vec<models::Ingredient> = self.extended_ingredients.iter().map(|api_ingredient| models::Ingredient {
            name: api_ingredient.name.clone(),
            amount: api_ingredient.amount.clone(),
            unit: api_ingredient.unit.clone()
        }).collect();

        let nutrition: models::Nutrition = models::Nutrition {
            nutrients: self.nutrition.nutrients.iter().map(|api_nutrient| models::Nutrient {
                name: api_nutrient.name.clone(),
                amount: api_nutrient.amount.clone(),
                unit: api_nutrient.unit.clone()
            }).collect(),
            properties: self.nutrition.properties.iter().map(|api_property| models::Property {
                name: api_property.name.clone(),
                amount: api_property.amount.clone(),
            }).collect()
        };

        
        let instructions: Vec<models::Step> = self.analyzed_instructions[0].steps.iter().map(|api_instruction| models::Step {
            number: api_instruction.number.clone(),
            step: api_instruction.step.clone(),
        }).collect();
        
        Recipe {
            _id: None,
            id: self.id.clone().to_string(),
            title: self.title.clone(),
            summary: self.summary.clone(),
            image: self.image.clone(),
            vegetarian: self.vegetarian.clone(),
            vegan: self.vegan.clone(),
            gluten_free: self.gluten_free.clone(),
            dairy_free: self.dairy_free.clone(),
            ready_in_minutes: self.ready_in_minutes.clone(),
            servings: self.servings.clone(),
            ingredients,
            nutrition,
            cuisines: self.cuisines.clone(),
            dish_types: self.dish_types.clone(),
            diets: self.diets.clone(),
            instructions
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ingredient {
    pub name: String,
    pub amount: f32,
    pub unit: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Nutrition {
    pub nutrients: Vec<Nutrient>,
    pub properties: Vec<Property>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Nutrient {
    pub name: String,
    pub amount: f32,
    pub unit: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Property {
    pub name: String,
    pub amount: f32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Instruction {
    pub steps: Vec<Step>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Step {
    pub number: i64,
    pub step: String
}