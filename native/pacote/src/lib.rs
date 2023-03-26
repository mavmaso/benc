mod outro;

use outro::*;
use std::collections::HashMap;
use rustler::{Atom, Error, NifMap, NifResult, types::atom::ok};
use csv::{ReaderBuilder};

#[derive(NifMap)]
pub struct CsvData{
    placeholders: Vec<String>,
    destination_count: usize,
    data: Vec<Vec<String>>,
    example_data: HashMap<String, String>
}

#[rustler::nif]
fn csv_reader(path: String, separator: String) ->  NifResult<(Atom, CsvData)>{
    let mut file = ReaderBuilder::new()
        .delimiter(valid_bit(&separator))
        .from_path(&path)
        .map_err(|_| Error::Atom("file_not_found"))?;

    let placeholders = file.headers()
        .map_err(|_| Error::Atom("header_error"))?
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    if placeholders.is_empty() { return Err(Error::Atom("no_header")) }

    let first_row = get_first_row(&path, valid_bit(&separator));
    let example_data = match first_row {
        Some(Ok(record)) => {
            let keys = placeholders.clone().into_iter();
            let values = record.iter().map(|s| s.to_string()).collect::<Vec<String>>();

            keys.zip(values).collect()
        },
        Some(Err(_)) => return Err(Error::Atom("failed_to_read_first_row")),
        None => return Err(Error::Atom("no_rows_found")),
    };

    let mut data:Vec<Vec<String>> = vec![];

    for row in file.records() {
        match row {
            Ok(r) => {
                let line = r.into_iter()
                    .map(|f| f.to_string())
                    .collect();

                data.push(line);
            },
            Err(_) => return Err(Error::Atom("row_format")),
        }
    };

    let csv_data = CsvData {
        destination_count: data.len(),
        placeholders,
        data,
        example_data
    };

    Ok((ok(), csv_data))
}


#[rustler::nif]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

rustler::init!("Elixir.Pacote", [add, csv_reader]);
