use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct TodolistEntry {
    id: i32,
    complete: bool,
    title: String,
}

#[derive(Deserialize)]
pub struct CreateEntryBody {
    pub title: String
}

#[derive(Deserialize)]
pub struct CompleteEntryBody {
    pub complete: bool,
}
