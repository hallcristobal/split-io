use {DateTime, User};
use serde_json::{from_str, from_value, Value};

type Duration = f32;


#[derive(Serialize, Deserialize)]
pub struct Runs {
	runs: Vec<Run>
}

impl Runs {
	pub fn parse(raw: &str) -> Result<Run, ()> {
		Ok(from_str(raw).unwrap())
	}
}

#[derive(Serialize, Deserialize)]
pub struct Run {
    id: usize,
    path: String,
    name: String,
    program: String,
    image_url: Option<String>,
    created_at: DateTime,
    updated_at: DateTime,
    video_url: Option<String>,
    #[serde(default)]
    splits: Option<Vec<Segment>>,
    attempts: usize,
    sum_of_best: Duration,
    user: Option<User>,
    game: Option<String>,
    category: Option<String>,
    time: Duration,
}

impl Run {
    pub fn parse(raw: &str) -> Result<Run, ()> {
        let v: Run = from_str(raw)
            .map(|v: Value| from_value(v["run"].clone()).unwrap())
            .unwrap();
        Ok(v)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Segment {
    name: String,
    duration: Duration,
    finish_time: Duration,
    best: Best,
    history: Vec<i32>,
    gold: bool,
    skipped: bool,
    reduced: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Best {
    duration: Duration,
}
