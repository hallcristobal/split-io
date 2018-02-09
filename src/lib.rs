extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod games;
pub mod users;
pub mod run;
pub mod category;
pub mod uris;

pub use users::User;
pub use games::Game;
pub use category::Category;

// TODO: Real DateTime
pub type DateTime = String;
