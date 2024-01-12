use editres::resource;

fn main() {
    testbin::print_resources(&[resource!("my_res"), resource!("my_res")]);
}
