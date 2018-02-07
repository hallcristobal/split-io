extern crate splits_io;

mod parsing {
    use std::fs::File;
    use std::io::{BufReader, Read};
    use splits_io::run::{Run, Runs};

    fn file(path: &str) -> String {
        let mut r = BufReader::new(File::open(path).unwrap());
        let mut buffer = String::new();
		r.read_to_string(&mut buffer)
            .unwrap();
			buffer
    }

    #[test]
    fn parse_run() {
        let buffer = file("./responses/run.json");
        Run::parse(&buffer).unwrap();
    }

	// Failing Currently
	#[test]
	fn parse_user_runs() {
		let buffer = file("./responses/user_runs.json");
		Runs::parse(&buffer).unwrap();
	}
}
