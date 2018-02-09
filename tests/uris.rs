extern crate splits_io;

mod uris {
    use splits_io::uris::*;

    #[test]
    fn get_search_uri_test() {
        assert_eq!(&get_search_uri("sonic"), "https://splits.io/api/v3/games?search=sonic");
    }

    #[test]
    fn get_id_uri_test() {
        assert_eq!(&get_id_uri("255"), "https://splits.io/api/v3/games/255");
    }

    #[test]
    fn get_game_runs_uri_test() {
        assert_eq!(&get_game_runs_uri("255"), "https://splits.io/api/v3/games/255/runs");
    }

    #[test]
    fn get_category_uri_test() {
        assert_eq!(&get_category_uri("255", "14"), "https://splits.io/api/v3/games/255/categories/14");
    }

    #[test]
    fn get_category_runs_uri_test() {
        assert_eq!(
            &get_category_runs_uri("255", "14"),
            "https://splits.io/api/v3/games/255/categories/14/runs"
        );
    }

    #[test]
    fn get_user_uri_test() {
        assert_eq!(&get_user_uri(555.to_string()), "https://splits.io/api/v3/users/555");
    }

    #[test]
    fn get_user_runs_uri_test() {
        assert_eq!(&get_user_runs_uri(555.to_string()), "https://splits.io/api/v3/users/555/runs");
    }

    #[test]
    fn get_user_pbs_uri_test() {
        assert_eq!(&get_user_pbs_uri(555.to_string()), "https://splits.io/api/v3/users/555/pbs");
    }

    #[test]
    fn get_user_game_category_runs_uri_test() {
        assert_eq!(
            &get_user_game_category_runs_uri(555.to_string(), String::from("sms"), 6.to_string()),
            "https://splits.io/api/v3/users/555/games/sms/categories/6/runs"
        );
    }

    #[test]
    fn get_user_game_category_pbs_uri_test() {
        assert_eq!(
            &get_user_game_category_pbs_uri(555.to_string(), String::from("sms"), 6.to_string()),
            "https://splits.io/api/v3/users/555/games/sms/categories/6/pb"
        );
    }

    #[test]
    fn get_user_game_category_prediction_uri_test() {
        assert_eq!(
            &get_user_game_category_prediction_uri(
                555.to_string(),
                String::from("sms"),
                6.to_string()
            ),
            "https://splits.io/api/v3/users/555/games/sms/categories/6/prediction"
        );
    }

}
