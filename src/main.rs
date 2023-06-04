
mod csv_read;
mod comp;

fn main() {
    let mut csv = csv_read::Csv::default();

    csv.read();

    let mut compr = comp::Compress{
        data: csv.ret_values().to_vec(),
    };

    compr.split_and_comp();

    csv.save(compr.data)

}
