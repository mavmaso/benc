use csv::{ReaderBuilder};

pub fn valid_bit(s: &str) -> u8{
    match s {
        ";" => b';',
        "|" => b'|',
        "\t" => b'\t',
        _ => b',',
    }
}

pub fn get_first_row(path: &str, delimiter: u8) -> Option<Result<csv::StringRecord, csv::Error>> {
    ReaderBuilder::new()
        .delimiter(delimiter)
        .from_path(&path)
        .unwrap()
        .records()
        .next()
}