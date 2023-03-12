
use std::collections::HashMap;
use rustler::{Atom, Error, NifMap, NifResult, types::atom::ok};
use csv::{ReaderBuilder};

#[derive(NifMap)]
pub struct CsvData{
    placeholders: Vec<String>,
    destination_count: i32,
    example_data: HashMap<String, String>
}

fn valid_bit(s: &str) -> u8{
    match s {
        ";" => b';',
        "|" => b'|',
        "\t" => b'\t',
        _ => b',',
    }
}


#[rustler::nif]
fn csv_reader(path: String, separator: String) ->  NifResult<(Atom, CsvData)>{
    let mut file = ReaderBuilder::new()
        .delimiter(valid_bit(&separator))
        .from_path(&path)
        .map_err(|_| Error::Atom("file_not_found"))?;

    let placeholders = file.headers()
        .map_err(|_| Error::Atom("no_header"))?
        .iter().map(|s| s.to_string()).collect::<Vec<String>>();

    // let example_data = match file.records().next() {
    //     Some(Ok(record)) => {
    //         let keys = placeholders.clone().into_iter();
    //         let values = record.iter().map(|s| s.to_string()).collect::<Vec<String>>();
    //         if keys.len() == values.len() {
    //             keys.zip(values).collect()
    //         } else {
    //             return Err(Error::Atom("example_data_error"));
    //         }
    //     },
    //     Some(Err(_)) => return Err(Error::Atom("failed_to_read_first_row")),
    //     None => return Err(Error::Atom("no_rows_found")),
    // };

    let example_data = HashMap::new();

    let mut destination_count = 0;
    for row in file.records() {
        match row {
            Ok(_) => destination_count += 1,
            Err(_) => return Err(Error::Atom("row_format")),
        }
    };

    let csv_data = CsvData {
        destination_count,
        placeholders,
        example_data,
    };

    Ok((ok(), csv_data))
}


#[rustler::nif]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

rustler::init!("Elixir.Pacote", [add, csv_reader]);
