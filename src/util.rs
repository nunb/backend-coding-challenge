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
