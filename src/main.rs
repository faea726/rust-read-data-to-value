mod in_out_lib;

fn main() {
    let filename = "data.txt";
    let data = in_out_lib::read_data_from_file(filename, ":=");
    for dt in data {
        print!("{:?}\n", dt);
    }
}
