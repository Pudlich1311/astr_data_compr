
mod csv_read;

fn main() {
    let mut csv = csv_read::CsvRead::default();

    csv.read();
    csv.print();
}
