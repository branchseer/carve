use editres::resource;

fn main() {
    testbin::print_resources(&[resource!("my_res1"), resource!("my_res2")]);
}
