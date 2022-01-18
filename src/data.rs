use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Property {
    // Handle keyword renaming.
    #[serde(rename = "type")]
    pub ty: String,
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct Object {
    pub name: String,
    pub properties: Option<Vec<Property>>,

    // Rect data is extracted to a separate struct so we can use it for intersection tests.
    #[serde(flatten)]
    pub rect: Rect,
}

// Main `TestInput.json` schema. Some of these fields should probably be enums in real world, but
// we don't care about that here.
#[derive(Serialize, Deserialize)]
pub struct TestInput {
    pub draworder: String,
    pub id: i32,
    pub name: String,
    pub objects: Vec<Object>,
    pub opacity: f32,
    pub visible: bool,
}
