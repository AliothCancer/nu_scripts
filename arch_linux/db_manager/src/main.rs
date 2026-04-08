use std::{
    fs::File,
    io::Error,
    path::{Path, PathBuf},
    str::FromStr,
};
mod log_structure;
use csv::Reader;
use csv_deserializer::{self, NullValues, csv_dataset::CsvDataset};

use crate::log_structure::CsvDataFrame;

fn main() {
    let path_str = "../logs/2026-2-10_h23-1-48_+01:00.csv";
    let csv1 = Path::new(path_str);
    let mut df = load_csv(csv1);
    df.populate_columns_infos();
    let df_s = CsvDataFrame::new(df);
}

fn load_csv(file: &Path) -> CsvDataset<'_> {
    let reader = Reader::from_path(file).unwrap();
    let df = CsvDataset::new(reader, NullValues(vec![]));

    df
}
