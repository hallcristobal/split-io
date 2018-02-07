extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod games;
pub mod users;
pub mod run;

pub use users::User;

// TODO: Real DateTime
pub type DateTime = String;

pub static BASE_URI: &'static str = "https://splits.io/api/v3";

pub fn get_uri(e: &str) -> String {
    let e = if !e.starts_with("/") {
        let mut n = "/".to_owned();
        n.push_str(e);
        n
    } else {
        e.to_owned()
    };

    format!("{}{}", BASE_URI, e)
}
