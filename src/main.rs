extern crate iron;
extern crate router;
extern crate urlencoded;

extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

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

#[derive(Deserialize)]
struct LocationRecord {
    // id: u64,
    name: String,
    // ascii: String,
    // alt_name: String,
    lat: f64,
    long: f64,
    // feat_class: String,
    // feat_code: String,
    country: String,
    // cc2: String,
    // admin1: String,
    // admin2: String,
    // admin3: String,
    // admin4: String,
    // population: u64,
    // elevation: Option<i32>,
    // dem: Option<f32>,
    // tz: String,
    // modified_at: String,
}

#[derive(Serialize)]
struct Suggestion {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub score: f64,
}

#[derive(Serialize)]
struct Suggestions {
    pub suggestions: Vec<Suggestion>
}

lazy_static!{
    static ref GEODATA: Vec<LocationRecord> = {
        let mut data = Vec::new();
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .from_reader(DATACSV.as_bytes());
        for record in rdr.deserialize() {
            let record: LocationRecord = record.expect("Invalid record in csv");
            data.push(record);
        }
        data
    };

    static ref GEONAMES: FnvHashMap<&'static str, Vec<&'static LocationRecord>> = {
        let mut map: FnvHashMap<&'static str, Vec<&'static LocationRecord>> = FnvHashMap::default();
        for record in GEODATA.iter() {
            map.entry(&record.name[..]).or_insert_with(Vec::new).push(record);
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
            if let Some(ref exacts) = gqf.and_then(|gqf| GEONAMES.get(gqf.as_str())) {
                serde_json::to_string(&Suggestions {
                    suggestions: exacts.iter().map(|exact|
                        Suggestion {
                            name: format!("{}, {}", exact.name, exact.country),
                            latitude: exact.lat,
                            longitude: exact.long,
                            score: 0.5,
                        }
                    ).collect()
                }).unwrap()
            } else {
                format!("GET: {:?}", gets)
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
