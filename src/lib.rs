#[macro_use]
extern crate serde_derive;

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

pub mod apis;
pub mod models;
pub mod propelauth;

#[cfg(feature = "axum")]
pub mod axum;

#[cfg(feature = "actix4")]
mod actix;
