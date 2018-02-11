extern crate serde_json;
extern crate splits_io;

mod parsing {
    use std::fs::File;
    use std::io::{BufReader, Read};
    use splits_io::Parse;
    use splits_io::games::{Game, Games};
    use splits_io::category::Category;
    use splits_io::run::{Run, Runs};
    use splits_io::users::User;
    use splits_io::DateTime;
    use serde_json::from_str;

    fn file(path: &str) -> String {
        let mut r = BufReader::new(File::open(path).unwrap());
        let mut buffer = String::new();
        r.read_to_string(&mut buffer).unwrap();
        buffer
    }

    #[test]
    fn parse_run() {
        let buffer = file("./responses/run.json");
        Run::parse(&buffer).unwrap();
    }

    #[test]
    fn parse_user_runs() {
        let buffer = file("./responses/user_runs.json");
        Runs::parse(&buffer).unwrap();
    }

    #[test]
    fn parse_user() {
        let buffer = file("./responses/user.json");
        User::parse(&buffer).unwrap();
    }

    #[test]
    fn parse_user_pbs() {
        let buffer = file("./responses/user_pbs.json");
        Runs::parse(&buffer).unwrap();
    }

    #[test]
    fn parse_user_category_runs() {
        let buffer = file("./responses/user_category_runs.json");
        Runs::parse(&buffer).unwrap();
    }

    #[test]
    fn parse_user_category_pb() {
        let buffer = file("./responses/user_category_pb.json");
        Run::parse(&buffer).unwrap();
    }

    #[test]
    fn parse_game() {
        let buffer = file("./responses/game.json");
        Game::parse(&buffer).unwrap();
    }

    #[test]
    fn parse_game_search() {
        let buffer = file("./responses/game_search.json");
        Games::parse(&buffer).unwrap();
    }

    #[test]
    fn parse_game_runs() {
        let buffer = file("./responses/game_runs.json");
        Runs::parse(&buffer).unwrap();
    }

    #[test]
    fn parse_category() {
        let buffer = file("./responses/category.json");
        Category::parse(&buffer).unwrap();
    }

    #[test]
    fn parse_category_runs() {
        let buffer = file("./responses/category_runs.json");
        Runs::parse(&buffer).unwrap();
    }

    #[test]
    fn parse_date_time() {
        let s = r#""2017-12-10T18:24:33.781Z""#;
        let _: DateTime = from_str(s).unwrap();
    }
}
