extern crate splits_io;

pub mod games {
    use splits_io::games::*;

    #[test]
    fn get_search_uri_test() {
        assert_eq!(&get_search_uri("sonic"), "/games?search=sonic");
    }

    #[test]
    fn get_id_uri_test() {
        assert_eq!(&get_id_uri("255"), "/games/255");
    }

    #[test]
    fn get_game_runs_uri_test() {
        assert_eq!(&get_game_runs_uri("255"), "/games/255/runs");
    }

    #[test]
    fn get_category_uri_test() {
        assert_eq!(&get_category_uri("255", "14"), "/games/255/categories/14");
    }

    #[test]
    fn get_category_runs_uri_test() {
        assert_eq!(
            &get_category_runs_uri("255", "14"),
            "/games/255/categories/14/runs"
        );
    }
}
