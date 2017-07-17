use models;

/// Takes degrees, returns distance in km
pub fn calcdist_latlong(lat1: f64, long1: f64, lat2: f64, long2: f64) -> f64
{
    // Haversine Formula

    let lat1 = lat1.to_radians();
    let lat2 = lat2.to_radians();
    let latdist = lat2 - lat1;
    let longdist = (long2 - long1).to_radians();
    let latdist_2_sin = (latdist / 2.0).sin();
    let longdist_2_sin = (longdist / 2.0).sin();

    let a = latdist_2_sin * latdist_2_sin +
        lat1.cos() * lat2.cos() *
        longdist_2_sin * longdist_2_sin;
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    c * 6361.0 // 6361km is the average radius of Earth
}

/// Compute Sorensen-Dice coefficient of two strings
/// Returns a similarity index [0..1]
pub fn dice_coefficient(s1: &str, s2: &str) -> f64
{
    // Ideally we'd iterate over character windows
    // Byte windows should be adequate

    // pastintersects is used to not count any pair colliding with multiple pairs multiple times
    // Vec over HashSet as we're dealing with small strings where linear scans are optimal
    let mut intersections = 0usize;
    let mut pastintersects = Vec::new();
    for p1 in s1.as_bytes().windows(2) {
        for (idx2, p2) in s2.as_bytes().windows(2).enumerate() {
            if !pastintersects.contains(&idx2) && p1 == p2 {
                intersections += 2;
                pastintersects.push(idx2);
            }
        }
    }

    // -2 from divisor as the last characters don't have a chance to match
    intersections as f64 / (s1.len() + s2.len() - 2) as f64
}

/// Return a Vec of location references starting with `q`, scored based on population
pub fn find_prefix(q: &str, locations: &'static [models::LocationRecord])
    -> Vec<(&'static models::LocationRecord, f64)>
{
    let mut matches = Vec::new();
    let mut total_population = 0;
    for record in locations.iter() {
        if record.name_lower.starts_with(q) ||
            record.name_ascii_lower.starts_with(q)
        {
            total_population += record.population;
            matches.push((record, 0.0));
        }
    }
    for &mut (ref r, ref mut score) in matches.iter_mut() {
        *score = r.population as f64 / total_population as f64;
    }
    matches
}

/// Return a Vec of location references paired with their Dice's Coefficient to `q`
pub fn find_similar(q: &str, locations: &'static [models::LocationRecord])
    -> Vec<(&'static models::LocationRecord, f64)>
{
    let mut matches = Vec::new();
    for record in locations.iter() {
        let score = f64::max(dice_coefficient(record.name_lower.as_str(), q),
            dice_coefficient(record.name_ascii_lower.as_str(), q));
        if score > 0.1 {
            matches.push((record, score));
        }
    }
    matches
}

#[cfg(test)]
mod test {
    use super::*;

    use std::cmp::Ordering;
    use std::f64::consts::PI;

    use models::LocationRecord;


    lazy_static!{
        static ref GEOMOCK: [LocationRecord; 5] = [
            LocationRecord {
                name: String::from("Asdf"),
                name_lower: String::from("asdf"),
                name_ascii_lower: String::from("asdf"),
                lat: 0.0,
                long: 10.0,
                country: String::from("CA"),
                stateprov: String::from("ON"),
                population: 69105,
            },
            LocationRecord {
                name: String::from("Sadf"),
                name_lower: String::from("sadf"),
                name_ascii_lower: String::from("sadf"),
                lat: 5.0,
                long: 15.0,
                country: String::from("CA"),
                stateprov: String::from("ON"),
                population: 23421,
            },
            LocationRecord {
                name: String::from("Qwèrty"),
                name_lower: String::from("qwèrty"),
                name_ascii_lower: String::from("qwerty"),
                lat: 20.0,
                long: 10.0,
                country: String::from("CA"),
                stateprov: String::from("ON"),
                population: 54321,
            },
            LocationRecord {
                name: String::from("Québec"),
                name_lower: String::from("québec"),
                name_ascii_lower: String::from("quebec"),
                lat: 46.0,
                long: 71.0,
                country: String::from("CA"),
                stateprov: String::from("QC"),
                population: 808080,
            },
            LocationRecord {
                name: String::from("Qi Monte"),
                name_lower: String::from("qi monte"),
                name_ascii_lower: String::from("qi monte"),
                lat: 2.0,
                long: 1.0,
                country: String::from("CA"),
                stateprov: String::from("ON"),
                population: 5432,
            },
        ];
    }

    fn assert_near(a: f64, b: f64, epsilion: f64) {
        assert!(a.is_finite());
        assert!(b.is_finite());
        println!("assert_near: {} {}", a, b);
        assert!((a-b).abs() < epsilion);
    }

    fn sort_match<T>(v: &mut Vec<(T, f64)>) {
        v.sort_by(|&(_, scorea), &(_, scoreb)| scoreb.partial_cmp(&scorea).unwrap_or(Ordering::Equal));
    }

    #[test]
    fn test_calcdist_latlong() {
        assert_near(calcdist_latlong(2.0, 2.0, 2.0, 2.0), 0.0, 0.1);
        assert_near(calcdist_latlong(0.0, 0.0, 0.0, 180.0), 6361.0 * PI, 1.0);
    }

    #[test]
    fn test_dice_coefficient() {
        assert_eq!(dice_coefficient("asdf", "asdf"), 1.0);
        assert_eq!(dice_coefficient("qwerty", "asdf"), 0.0);
    }

    #[test]
    fn test_find_prefix() {
        let mut q = find_prefix("q", &GEOMOCK[..]);
        sort_match(&mut q);
        assert_eq!(q.len(), 3);
        assert_eq!(q[0].0.name.as_str(), "Québec");
        assert_eq!(q[1].0.name.as_str(), "Qwèrty");
        assert_eq!(q[2].0.name.as_str(), "Qi Monte");
    }

    #[test]
    fn test_find_similar() {
        let mut q = find_similar("qwete", &GEOMOCK[..]);
        sort_match(&mut q);
        assert_eq!(q.len(), 2);
        assert_eq!(q[0].0.name.as_str(), "Qwèrty");
        assert_eq!(q[1].0.name.as_str(), "Qi Monte");
    }

}
