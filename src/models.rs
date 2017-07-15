#[derive(Deserialize)]
pub struct LocationRecord {
    // id: u64,
    pub name: String,
    // ascii: String,
    // alt_name: String,
    pub lat: f64,
    pub long: f64,
    // feat_class: String,
    // feat_code: String,
    pub country: String,
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
pub struct Suggestion {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub score: f64,
}

#[derive(Serialize)]
pub struct Suggestions {
    pub suggestions: Vec<Suggestion>,
}

#[derive(Serialize)]
pub struct Error {
    pub err: String,
}
