
mod csv_read;
mod compress;

fn main() {
    let mut csv = csv_read::Csv::default();

    csv.read();

    let mut compr = compress::Compress{
        data: csv.ret_data().to_vec(),
    };

    compr.split_and_compress();

    csv.save(compr.data)

}
