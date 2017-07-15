use csv;
use suffix::SuffixTable;

use models;

pub static INDEXHTML: &'static str = include_str!("index.html");
pub static DATACSV: &'static str = include_str!("../data/cities_canada-usa.tsv");

lazy_static!{
    pub static ref GEODATA: Vec<models::LocationRecord> = {
        let mut data = Vec::new();
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .from_reader(DATACSV.as_bytes());
        for record in rdr.deserialize() {
            let record: models::LocationRecord = record.expect("Invalid record in csv");
            data.push(record);
        }
        data
    };

    // BurntSushi hasn't decided how to create a good API for querying a suffix from a set of strings
    // workaround: search for suffix in stringset.join('\0'), maintain a method to lookup nth string based on byte index
    pub static ref SUFFIX_STRINDEX_PAIR: (String, Vec<usize>) = {
        let mut suffixstr = String::new();
        let mut suffixindices = Vec::new();
        for record in GEODATA.iter() {
            suffixindices.push(suffixstr.len());
            suffixstr.push_str(&record.name.to_lowercase());
            suffixstr.push('\0');
        }

        let pop0 = suffixstr.pop();
        assert_eq!(pop0, Some('\0'));

        (suffixstr, suffixindices)
    };

    pub static ref SUFFIXTABLE: SuffixTable<'static, 'static> = SuffixTable::new(SUFFIX_STRINDEX_PAIR.0.as_str());
    pub static ref SUFFIXINDICES: &'static Vec<usize> = &SUFFIX_STRINDEX_PAIR.1;
}
