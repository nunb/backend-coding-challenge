use std::cmp::Ordering;
use iron::{IronResult, Plugin, Request, Response, status, headers};
use iron::mime::{Mime, TopLevel, SubLevel};
use serde_json;
use urlencoded::UrlEncodedQuery;

use models;
use statics;
use util;

pub fn index(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::with((status::Ok, statics::INDEXHTML));
    resp.headers.set(headers::ContentType(Mime(TopLevel::Text, SubLevel::Html, Vec::new())));
    Ok(resp)
}

pub fn suggestions(req: &mut Request) -> IronResult<Response> {
    let resptext = match req.get_ref::<UrlEncodedQuery>() {
        Ok(ref gets) => {
            let latitude: Option<f64> = gets.get("latitude").and_then(|x| x.first()).and_then(|x| x.parse().ok());
            let longitude: Option<f64> = gets.get("longitude").and_then(|x| x.first()).and_then(|x| x.parse().ok());
            let radius: f64 = gets.get("radius").and_then(|x| x.first()).and_then(|x| x.parse().ok()).unwrap_or(500.0);
            if let Some(gqf) = gets.get("q").and_then(|gq| gq.first()).map(|gq| gq.to_lowercase()) {
                // With less than 3 bytes we can't do much of a query
                // Specialize to return cities starting with that letter
                // Ordered by population descending
                // Score: population / total-population-of-results
                let mut matches = if gqf.len() < 3 {
                    util::find_prefix(gqf.as_str(), &statics::GEODATA)
                } else {
                    util::find_similar(gqf.as_str(), &statics::GEODATA)
                };
                matches.sort_by(|&(_, scorea), &(_, scoreb)| scoreb.partial_cmp(&scorea).unwrap_or(Ordering::Equal));
                let mut result = Vec::new();
                for (data, score) in matches.into_iter() {
                    if let (Some(latitude), Some(longitude)) = (latitude, longitude) {
                        if util::calcdist_latlong(data.lat, data.long, latitude, longitude) > radius {
                            continue
                        }
                    }
                    result.push(models::Suggestion::new(data, score));
                    if result.len() == 20 {
                        break
                    }
                }
                serde_json::to_string(&models::Suggestions { suggestions: result }).unwrap()
            } else {
                String::from("{err:\"Missing parameter: q\"}")
            }
        },
        Err(ref e) => serde_json::to_string(&models::Error { err: format!("Error: {:?}", e) }).unwrap(),
    };
    let mut resp = Response::with((status::Ok, resptext));
    resp.headers.set(headers::ContentType(Mime(TopLevel::Application, SubLevel::Json, Vec::new())));
    Ok(resp)
}
