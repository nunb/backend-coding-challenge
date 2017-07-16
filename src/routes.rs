use std::cmp::Ordering;
use iron::{IronResult, Plugin, Request, Response, status, headers};
use iron::mime::{Mime, TopLevel, SubLevel};
use serde_json;
use urlencoded::UrlEncodedQuery;

use models;
use statics::*;
use util;

pub fn index(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::with((status::Ok, INDEXHTML));
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
                // `matchs` stores references to the records
                // this avoids processing records we won't be returning
                let mut matchs = Vec::new();
                if gqf.len() < 3 {
                    // With less than 3 bytes we can't do much of a query
                    // Specialize to return cities starting with that letter
                    // Ordered by population descending
                    // Score: population / total-population-of-results

                    let mut total_population = 0;
                    for record in GEODATA.iter() {
                        if record.name_lower.starts_with(gqf.as_str()) {
                            total_population += record.population;
                            matchs.push((record, 0.0));
                        }
                    }
                    for &mut (ref r, ref mut score) in matchs.iter_mut() {
                        *score = r.population as f64 / total_population as f64;
                    }
                } else {
                    let indices = SUFFIXTABLE.positions(gqf.as_str());

                    for &idx32 in indices {
                        let idx = idx32 as usize;
                        let geodata_idx = match SUFFIXINDICES.binary_search(&idx) {
                            Ok(x) => x,
                            Err(x) => x-1,
                        };
                        let record = &GEODATA[geodata_idx];
                        matchs.push((record, util::dice_coefficient(record.name_lower.as_str(), gqf.as_str())));
                    }
                }
                matchs.sort_by(|&(_, dista), &(_, distb)| distb.partial_cmp(&dista).unwrap_or(Ordering::Equal));
                let mut result = Vec::new();
                for (data, score) in matchs.into_iter() {
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
