extern crate iron;
extern crate router;
extern crate urlencoded;

extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

extern crate csv;
#[macro_use] extern crate lazy_static;
extern crate levenshtein;
extern crate suffix;

mod models;
mod routes;
mod statics;

use std::env;
use iron::Iron;
use router::Router;

fn main() {
    let port: u16 = env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(80);

    let mut router: Router = Router::new();
    router.get("/", routes::index, "index");
    router.get("/suggestions", routes::suggestions, "suggestions");

    Iron::new(router).http(("0.0.0.0", port)).unwrap();
}
