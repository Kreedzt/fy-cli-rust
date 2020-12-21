#[cfg(test)]
mod user_input_tests {
    use fy_cli_rust::parse::{get_user_input, generate_param};
    use fy_cli_rust::model::{Params};

    #[test]
    fn user_input_apple() {
        let v = get_user_input("apple".to_string());
        assert_eq!(v, "apple");
    }

    #[test]
    fn user_input_too_long() {
        let v = get_user_input("This is too long statement".to_string());
        assert_eq!(v, "This is to26 statement");
    }
}

// #[test]
// fn it_generate_param() {
    
// }
