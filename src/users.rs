use DateTime;
use serde_json::from_str;

#[derive(Serialize, Deserialize)]
struct UserObject {
    user: User
}

#[derive(Serialize, Deserialize)]
pub struct User {
    twitch_id: usize,
    name: String,
    avatar: Option<String>,
    created_at: DateTime,
    updated_at: DateTime,
}

impl User {
    pub fn parse(raw: &str) -> Result<User, ()> {
        let v: UserObject = from_str(raw).unwrap();
        Ok(v.user)
    }
}
