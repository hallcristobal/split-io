use {DateTime, Parse};
use serde_json::from_str;

#[derive(Serialize, Deserialize)]
struct UserObject {
    user: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub twitch_id: usize,
    pub name: String,
    pub avatar: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Parse<User> for User {
    fn parse(raw: &str) -> Result<User, ()> {
        let v: UserObject = from_str(raw).unwrap();
        Ok(v.user)
    }
}
