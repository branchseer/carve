use std::io::stdout;

pub fn print_resources<'a>(resources: &[Option<&'static [u8]>]) {
    bincode::serialize_into(&mut stdout().lock(), resources).unwrap()
}
