use {DateTime, Parse};
use serde_json::from_str;

#[derive(Serialize, Deserialize)]
struct CategoryObject {
    category: Category,
}

#[derive(Serialize, Deserialize)]
pub struct Category {
    id: u32,
    name: String,
    created_at: DateTime,
    updated_at: DateTime,
}

impl Parse<Category> for Category {
    fn parse(raw: &str) -> Result<Category, ()> {
        let v: CategoryObject = from_str(raw).unwrap();
        Ok(v.category)
    }
}
