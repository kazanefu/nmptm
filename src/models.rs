use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub deadline: Option<DateTime<Utc>>,
    pub priority: Priority,
    pub tags: Vec<String>,
    pub status: TaskStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nutrition {
    pub calories: f32,
    pub protein: f32,
    pub fat: f32,
    pub carbohydrate: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub id: Uuid,
    pub name: String,
    pub ingredients: Vec<String>,
    pub instructions: Vec<String>,
    pub nutrition: Nutrition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MealType {
    Breakfast,
    Lunch,
    Dinner,
    Snack,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Menu {
    pub id: Uuid,
    pub date: DateTime<Utc>,
    pub meal_type: MealType,
    pub recipe_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppDatabase {
    pub tasks: Vec<Task>,
    pub recipes: Vec<Recipe>,
    pub menus: Vec<Menu>,
}
