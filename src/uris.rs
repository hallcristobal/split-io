pub static BASE_URI: &'static str = "https://splits.io/api/v3";

pub fn get_uri(e: &str) -> String {
    let e = if !e.starts_with("/") {
        let mut n = "/".to_owned();
        n.push_str(e);
        n
    } else {
        e.to_owned()
    };

    format!("{}{}", BASE_URI, e)
}

/// Returns the endpoint for searching for a game.
///
/// This endpoint returns a `Vec<Game>`.
pub fn get_search_uri(term: &str) -> String {
    get_uri(&format!("/games?search={}", term))
}

/// Returns the endpoint for getting a game with an id or shortname.
///
/// This endpoint returns a `Game`.
pub fn get_id_uri(id: &str) -> String {
    get_uri(&format!("/games/{}", id))
}

/// Returns the endpoint for getting the runs for a game.
///
/// This endpoint returns a `Vec<Run>`.
pub fn get_game_runs_uri(id: &str) -> String {
    get_uri(&format!("/games/{}/runs", id))
}

/// Returns the endpoint for getting a category for a game.
///
/// This endpoint returns a `Category`.
pub fn get_category_uri(game_id: &str, category_id: &str) -> String {
    get_uri(&format!("/games/{}/categories/{}", game_id, category_id))
}

/// Returns the endpoint for getting the runs for a category.
///
/// This endpoint returns a `Vec<Run>`.
pub fn get_category_runs_uri(game_id: &str, category_id: &str) -> String {
    get_uri(&format!(
        "/games/{}/categories/{}/runs",
        game_id, category_id
    ))
}

pub fn get_user_uri<S: Into<String>>(user_id: S) -> String {
    get_uri(&format!("/users/{}", user_id.into()))
}

pub fn get_user_runs_uri<S: Into<String>>(user_id: S) -> String {
    get_uri(&format!("/users/{}/runs", user_id.into()))
}

pub fn get_user_pbs_uri<S: Into<String>>(user_id: S) -> String {
    get_uri(&format!("/users/{}/pbs", user_id.into()))
}

pub fn get_user_game_category_runs_uri<S: Into<String>>(
    user_id: S,
    game_id: S,
    category_id: S,
) -> String {
    get_uri(&format!(
        "/users/{}/games/{}/categories/{}/runs",
        user_id.into(),
        game_id.into(),
        category_id.into()
    ))
}

pub fn get_user_game_category_pbs_uri<S: Into<String>>(
    user_id: S,
    game_id: S,
    category_id: S,
) -> String {
    get_uri(&format!(
        "/users/{}/games/{}/categories/{}/pb",
        user_id.into(),
        game_id.into(),
        category_id.into()
    ))
}

pub fn get_user_game_category_prediction_uri<S: Into<String>>(
    user_id: S,
    game_id: S,
    category_id: S,
) -> String {
    get_uri(&format!(
        "/users/{}/games/{}/categories/{}/prediction",
        user_id.into(),
        game_id.into(),
        category_id.into()
    ))
}
