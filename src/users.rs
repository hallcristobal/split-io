use DateTime;

#[derive(Serialize, Deserialize)]
pub struct User {
    twitch_id: usize,
    name: String,
    avatar: String,
    created_at: DateTime,
    updated_at: DateTime,
}

pub fn get_user_uri<S: Into<String>>(user_id: S) -> String {
    format!("/users/{}", user_id.into())
}

pub fn get_user_runs_uri<S: Into<String>>(user_id: S) -> String {
    format!("/users/{}/runs", user_id.into())
}

pub fn get_user_pbs_uri<S: Into<String>>(user_id: S) -> String {
    format!("/users/{}/pbs", user_id.into())
}

pub fn get_user_game_category_runs_uri<S: Into<String>>(
    user_id: S,
    game_id: S,
    category_id: S,
) -> String {
    format!(
        "/users/{}/games/{}/categories/{}/runs",
        user_id.into(),
        game_id.into(),
        category_id.into()
    )
}

pub fn get_user_game_category_pbs_uri<S: Into<String>>(
    user_id: S,
    game_id: S,
    category_id: S,
) -> String {
    format!(
        "/users/{}/games/{}/categories/{}/pb",
        user_id.into(),
        game_id.into(),
        category_id.into()
    )
}

pub fn get_user_game_category_prediction_uri<S: Into<String>>(
    user_id: S,
    game_id: S,
    category_id: S,
) -> String {
    format!(
        "/users/{}/games/{}/categories/{}/prediction",
        user_id.into(),
        game_id.into(),
        category_id.into()
    )
}
