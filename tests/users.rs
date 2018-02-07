extern crate splits_io;

mod users {
    use splits_io::users::*;

    #[test]
    fn get_user_uri_test() {
        assert_eq!(&get_user_uri(555.to_string()), "/users/555");
    }

    #[test]
    fn get_user_runs_uri_test() {
        assert_eq!(&get_user_runs_uri(555.to_string()), "/users/555/runs");
    }

    #[test]
    fn get_user_pbs_uri_test() {
        assert_eq!(&get_user_pbs_uri(555.to_string()), "/users/555/pbs");
    }

    #[test]
    fn get_user_game_category_runs_uri_test() {
        assert_eq!(
            &get_user_game_category_runs_uri(555.to_string(), String::from("sms"), 6.to_string()),
            "/users/555/games/sms/categories/6/runs"
        );
    }

    #[test]
    fn get_user_game_category_pbs_uri_test() {
        assert_eq!(
            &get_user_game_category_pbs_uri(555.to_string(), String::from("sms"), 6.to_string()),
            "/users/555/games/sms/categories/6/pb"
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
            "/users/555/games/sms/categories/6/prediction"
        );
    }

}
