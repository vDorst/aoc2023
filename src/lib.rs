use std::io::Read;

pub mod strings;

pub fn read_input(filename: &str) -> String {
    let mut f = std::fs::File::open(filename).unwrap();

    let m = f.metadata().unwrap().len() as usize;

    let mut buf = Vec::<u8>::with_capacity(m);

    f.read_to_end(&mut buf).unwrap();

    String::from_utf8(buf).unwrap()
}

pub fn input_filename(source_filename: &str) -> &str {
    let (_, fl) = source_filename.rsplit_once('/').unwrap();
    fl.split_once('.').unwrap().0
}
