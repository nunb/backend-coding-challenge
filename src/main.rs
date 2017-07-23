extern crate iron;
extern crate mount;
extern crate router;
extern crate staticfile;
extern crate urlencoded;

extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

extern crate csv;
#[macro_use] extern crate lazy_static;
extern crate fnv;

mod models;
mod routes;
mod statics;
mod util;

use std::env;
use std::time::Duration;
use mount::Mount;
use router::Router;
use staticfile::Static;

fn main() {
    let port: u16 = env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(80);

    let public = Static::new("public/").cache(Duration::from_secs(60 * 60 * 24));

    let mut router = Router::new();
    router.get("/", public.clone(), "index");
    router.get("/suggestions", routes::suggestions, "suggestions");

    let mut mount = Mount::new();
    mount.mount("/", router);
    mount.mount("/public/", public);

    iron::Iron::new(mount).http(("0.0.0.0", port)).unwrap();
}
