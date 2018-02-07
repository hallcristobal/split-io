use DateTime;

#[derive(Serialize, Deserialize)]
pub struct Game {
    id: u32,
    name: String,
    short_name: Option<String>,
    created_at: DateTime,
    updated_at: DateTime,
    categories: Vec<Category>,
}

#[derive(Serialize, Deserialize)]
pub struct Category {
    id: u32,
    name: String,
    created_at: DateTime,
    updated_at: DateTime,
}

/// Returns the endpoint for searching for a game.
///
/// This endpoint returns a `Vec<Game>`.
pub fn get_search_uri(term: &str) -> String {
    format!("/games?search={}", term)
}

/// Returns the endpoint for getting a game with an id or shortname.
///
/// This endpoint returns a `Game`.
pub fn get_id_uri(id: &str) -> String {
    format!("/games/{}", id)
}

/// Returns the endpoint for getting the runs for a game.
///
/// This endpoint returns a `Vec<Run>`.
pub fn get_game_runs_uri(id: &str) -> String {
    format!("/games/{}/runs", id)
}

/// Returns the endpoint for getting a category for a game.
///
/// This endpoint returns a `Category`.
pub fn get_category_uri(game_id: &str, category_id: &str) -> String {
    format!("/games/{}/categories/{}", game_id, category_id)
}

/// Returns the endpoint for getting the runs for a category.
///
/// This endpoint returns a `Vec<Run>`.
pub fn get_category_runs_uri(game_id: &str, category_id: &str) -> String {
    format!("/games/{}/categories/{}/runs", game_id, category_id)
}
