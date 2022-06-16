use std::env;
use std::fs::File;

pub fn get_transaction_reader() -> csv::Reader<File> {
    let csv_reader: csv::Reader<File>;
    {
        let fpath: String;
        {
            let args: Vec<String> = env::args().collect();
            if args.len() != 2 {
                panic!("User error. Run toy payment engine as: $ cargo run -- transactions.csv");
            }
            fpath = args[1].to_string();
        }

        let csv_reader_res = csv::ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b',')
            .flexible(true)
            .trim(csv::Trim::All)
            .from_path(&fpath);

        if csv_reader_res.is_ok() {
            csv_reader = csv_reader_res.unwrap();
        } else {
            panic!("file {} not found", &fpath);
        }
    }
    csv_reader
}

