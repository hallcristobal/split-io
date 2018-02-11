extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;

pub mod date_time;
pub mod games;
pub mod users;
pub mod run;
pub mod category;
pub mod uris;
pub mod calls;

pub use users::User;
pub use games::Game;
pub use category::Category;
pub use date_time::DateTime;

pub trait Parse<T> {
    fn parse(raw: &str) -> std::result::Result<T, ()>;
}
