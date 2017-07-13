extern crate iron;
extern crate router;
extern crate urlencoded;

extern crate csv;
extern crate fnv;
#[macro_use] extern crate lazy_static;

use std::env;
use fnv::FnvHashMap;
use iron::{Iron, IronResult, Plugin, Request, Response, status, headers};
use iron::mime::{Mime, TopLevel, SubLevel};
use router::Router;
use urlencoded::UrlEncodedQuery;

static INDEXHTML: &'static str = include_str!("index.html");
static DATACSV: &'static str = include_str!("../data/cities_canada-usa.tsv");

struct LocationRecord {
    id: u64,
    name: String,
    ascii: String,
    alt_name: String,
    lat: f64,
    long: f64,
    feat_class: String,
    feat_code: String,
    country: String,
    cc2: String,
    admin1: String,
    admin2: String,
    admin3: String,
    admin4: String,
    population: u64,
    elevation: u64,
    dem: u64,
    tz: String,
    modified_at: String,
}

struct LatLong(f64, f64);

lazy_static!{
    static ref GEODATA: FnvHashMap<String, LatLong> = {
        let mut map: FnvHashMap<String, LatLong> = FnvHashMap::default();
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b'\t')
            .from_reader(DATACSV.as_bytes());
        for record in rdr.records().filter_map(|x| x.ok()) {
            map.insert(String::from(&record[1]), LatLong(record[4].parse().unwrap(), record[5].parse().unwrap()));
        }
        map
    };
}

fn index(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::with((status::Ok, INDEXHTML));
    resp.headers.set(headers::ContentType(Mime(TopLevel::Text, SubLevel::Html, Vec::new())));
    Ok(resp)
}

fn suggestions(req: &mut Request) -> IronResult<Response> {
    let resptext = match req.get_ref::<UrlEncodedQuery>() {
        Ok(ref gets) => {
            let gqf = gets.get("q").and_then(|gq| gq.first());
            if let Some(exact) = gqf.and_then(|gqf| GEODATA.get(gqf)) {
                format!("{{suggestions:[{{latitude:\"{}\",longitude:\"{}\"}}]}}", exact.0, exact.1)
            } else {
                format!("GET {}: {:?}", get_server_port(), gets)
            }
        },
        Err(ref e) => format!("Error: {:?}", e),
    };
    let mut resp = Response::with((status::Ok, resptext));
    resp.headers.set(headers::ContentType(Mime(TopLevel::Application, SubLevel::Json, Vec::new())));
    Ok(resp)
}

fn get_server_port() -> u16 {
    let port_str = env::var("PORT").unwrap_or(String::new());
    port_str.parse().unwrap_or(80)
}

fn main() {
    let mut router: Router = Router::new();
    router.get("/", index, "index");
    router.get("/suggestions", suggestions, "suggestions");

    Iron::new(router).http(("0.0.0.0", get_server_port())).unwrap();
}
