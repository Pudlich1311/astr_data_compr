
mod csv_read;
mod comp;

fn main() {
    let mut csv = csv_read::CsvRead::default();

    csv.read();

    let mut compr = comp::Compress{
        values: csv.ret_values().to_vec(),
    };

    compr.print();
    compr.split();

    compr.print();
}
