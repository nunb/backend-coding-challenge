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

    intersections as f64 / (s1.len() + s2.len()) as f64
}
