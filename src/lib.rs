#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate failure;
extern crate rayon;
extern crate reqwest;

mod user;
mod episode;
mod podcast;