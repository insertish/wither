use serde::{Serialize, Deserialize};
use wither::Model;

#[derive(Serialize, Deserialize, Model)]
struct BadModel {
    id: Option<String>,
}

fn main() {}
