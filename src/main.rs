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

use std::env;
use iron::{Iron, IronResult, Plugin, Request, Response, status, headers};
use iron::mime::{Mime, TopLevel, SubLevel};
use levenshtein::levenshtein;
use router::Router;
use suffix::SuffixTable;
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
    pub suggestions: Vec<Suggestion>,
}

#[derive(Serialize)]
struct Error {
    pub err: String,
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

    // BurntSushi hasn't decided how to create a good API for querying a suffix from a set of strings
    // workaround: search for suffix in stringset.join('\0'), maintain a method to lookup nth string based on byte index
    static ref SUFFIX_STRINDEX_PAIR: (String, Vec<usize>) = {
        let mut suffixstr = String::new();
        let mut suffixindices = Vec::new();
        for record in GEODATA.iter() {
            suffixindices.push(suffixstr.len());
            suffixstr.push_str(&record.name);
            suffixstr.push('\0');
        }

        let pop0 = suffixstr.pop();
        assert_eq!(pop0, Some('\0'));

        (suffixstr, suffixindices)
    };

    static ref SUFFIXTABLE: SuffixTable<'static, 'static> = SuffixTable::new(SUFFIX_STRINDEX_PAIR.0.as_str());
    static ref SUFFIXINDICES: &'static Vec<usize> = &SUFFIX_STRINDEX_PAIR.1;
}

fn index(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::with((status::Ok, INDEXHTML));
    resp.headers.set(headers::ContentType(Mime(TopLevel::Text, SubLevel::Html, Vec::new())));
    Ok(resp)
}

fn suggestions(req: &mut Request) -> IronResult<Response> {
    let resptext = match req.get_ref::<UrlEncodedQuery>() {
        Ok(ref gets) => {
            if let Some(gqf) = gets.get("q").and_then(|gq| gq.first()) {
                let indices = SUFFIXTABLE.positions(gqf.as_str());

                let mut result = Vec::new();
                let mut matchs = Vec::new();
                if !indices.is_empty() {
                    let score = 1. / indices.len() as f64;
                    for &idx32 in indices {
                        let idx = idx32 as usize;
                        let geodata_idx = match SUFFIXINDICES.binary_search(&idx) {
                            Ok(x) => x,
                            Err(x) => x-1,
                        };
                        let record = &GEODATA[geodata_idx];
                        matchs.push((record, levenshtein(record.name.as_str(), gqf.as_str())));
                    }
                    matchs.sort_by(|&(_, dista), &(_, distb)| dista.cmp(&distb));

                    for (data, _) in matchs {
                        result.push(Suggestion {
                            name: format!("{}, {}", data.name, data.country),
                            latitude: data.lat,
                            longitude: data.long,
                            score: score,
                        });
                    }
                }

                serde_json::to_string(&Suggestions { suggestions: result }).unwrap()
            } else {
                String::from("{ err: \"Missing parameter: q\" }")
            }
        },
        Err(ref e) => serde_json::to_string(&Error { err: format!("Error: {:?}", e) }).unwrap(),
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
