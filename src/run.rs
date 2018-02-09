use {DateTime, User, Game, Category};
use serde_json::{from_str, Value};

type Duration = f32;

#[derive(Serialize, Deserialize)]
pub struct Runs {
    runs: Vec<Run>,
}

impl Runs {
    pub fn parse(raw: &str) -> Result<Vec<Run>, ()> {
        let v: Value = from_str(raw).unwrap();
        if let Some(_) = v.get("runs") {
            let v: Runs = from_str(raw).unwrap();
            Ok(v.runs)
        } else {
            let v: PbObject = from_str(raw).unwrap();
            Ok(v.pbs)
        }
    }
}

#[derive(Serialize, Deserialize)]
struct RunObject {
    run: Run
}

#[derive(Serialize, Deserialize)]
struct PbObject {
    pbs: Vec<Run>
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
    game: Option<Game>,
    category: Option<Category>,
    time: Duration,
}

impl Run {
    pub fn parse(raw: &str) -> Result<Run, ()> {
        let v: RunObject = from_str(raw).unwrap();
        Ok(v.run)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Segment {
    name: String,
    duration: Duration,
    finish_time: Duration,
    best: Best,
    history: Vec<f32>,
    gold: bool,
    skipped: bool,
    reduced: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Best {
    duration: Duration,
}
