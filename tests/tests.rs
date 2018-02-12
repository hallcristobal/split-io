extern crate splits_io;
// extern crate tokio;
extern crate futures;
extern crate tokio_core;

mod tests {
    use splits_io::uris::get_uri;

    #[test]
    fn get_uri_test() {
        assert_eq!(
            &get_uri("/games?search=sonic"),
            "https://splits.io/api/v3/games?search=sonic"
        );
        assert_eq!(&get_uri("games/sms"), "https://splits.io/api/v3/games/sms");
    }
}

mod client {
    use splits_io::Client;
    use tokio_core::reactor::Core;
    use futures::{Future, Stream};

    #[test]
    fn get_client() {
        let mut core = Core::new().unwrap();
        let client = Client::new(&core.handle());
        let work = client.get("https://splits.io/api/v3/users/555").map(|res| {
            res.body().concat2().map(|chunk| {
                let s = String::from_utf8(chunk.to_vec()).unwrap();
                println!("{}", s);
            });
        });
        core.run(work).unwrap();
        assert!(false); // Asserting false just so I can see an output
    }
}
