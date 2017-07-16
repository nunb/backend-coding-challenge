#[derive(Deserialize)]
pub struct LocationRecordRaw {
    // pub id: u64,
    pub name: String,
    // pub ascii: String,
    // pub alt_name: String,
    pub lat: f64,
    pub long: f64,
    // pub feat_class: String,
    // pub feat_code: String,
    pub country: String,
    // pub cc2: String,
    // pub admin1: String,
    // pub admin2: String,
    // pub admin3: String,
    // pub admin4: String,
    pub population: u64,
    // pub elevation: Option<i32>,
    // pub dem: Option<f32>,
    // pub tz: String,
    // pub modified_at: String,
}

pub struct LocationRecord {
    pub name: String,
    pub name_lower: String,
    pub lat: f64,
    pub long: f64,
    pub country: String,
    pub population: u64,
}

#[derive(Serialize)]
pub struct Suggestion {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub population: u64,
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

impl From<LocationRecordRaw> for LocationRecord {
    fn from(raw: LocationRecordRaw) -> LocationRecord
    {
        LocationRecord {
            name_lower: raw.name.to_lowercase(),
            name: raw.name,
            lat: raw.lat,
            long: raw.long,
            country: raw.country,
            population: raw.population,
        }
    }
}

impl Suggestion {
    pub fn new(record: &LocationRecord, score: f64) -> Suggestion
    {
        Suggestion {
            name: format!("{}, {}", record.name, record.country),
            latitude: record.lat,
            longitude: record.long,
            population: record.population,
            score: score,
        }
    }
}
