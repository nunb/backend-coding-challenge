use csv;
use fnv::FnvHashMap;

use models;

pub static DATACSV: &'static str = include_str!("../data/cities_canada-usa.tsv");

lazy_static!{
    pub static ref GEODATA: Vec<models::LocationRecord> = {
        let mut data = Vec::new();
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .from_reader(DATACSV.as_bytes());
        for record in rdr.deserialize() {
            let record: models::LocationRecordRaw = record.expect("Invalid record in csv");
            data.push(record.into());
        }
        data
    };

    pub static ref PROVADMIN1: FnvHashMap<&'static str, &'static str> = {
        // Admin1 is already state codes for US, so only need to map province codes
        let mut map = FnvHashMap::with_capacity_and_hasher(13, Default::default());
        map.insert("01", "AB");
        map.insert("02", "BC");
        map.insert("03", "MB");
        map.insert("04", "NB");
        map.insert("05", "NL");
        // No 6
        map.insert("07", "NS");
        map.insert("08", "ON");
        map.insert("09", "PE");
        map.insert("10", "QC");
        map.insert("11", "SK");
        map.insert("12", "YK");
        map.insert("13", "NT");
        map.insert("14", "NU");
        map
    };
}
