extern crate splits_io;

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
