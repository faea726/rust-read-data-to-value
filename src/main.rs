mod in_out_lib;
use in_out_lib::read_data_from_file;

fn main() {
    let filename = "data.txt";
    let data = read_data_from_file(filename, ":=");
    for dt in data {
        print!("{:?}\n", dt);
    }
}
