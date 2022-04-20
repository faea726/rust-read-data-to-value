mod in_out_lib;

fn main() {
    let filename = "config.txt";
    let data = in_out_lib::read_data_from_file(filename);
    print!("{:?}", data);
}
