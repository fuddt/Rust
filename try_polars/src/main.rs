use polars::prelude::*; // Polars全般の機能をインポート


fn example() -> PolarsResult<DataFrame> {
    CsvReadOptions::default()
            .with_has_header(true)
            .try_into_reader_with_file_path(Some("data.csv".into()))?
            .finish()
}

fn main() -> PolarsResult<()> {
    let df = example()?;
    println!("{:?}", df);
    Ok(())
}