use std::{
    fs::{DirBuilder, File},
    io::{Read, Write},
    time::Instant,
};

use ordinal::Ordinal;

use crate::{
    file_types::FileTypeInfo,
    utils::{compare_headers, find_index_by_windowing},
};

const BUFFER_SIZE: usize = 1024 * 2;

pub fn start_recover(device: String, recover_type: FileTypeInfo, output: String) {
    let now = Instant::now();
    DirBuilder::new()
        .recursive(true)
        .create("recovered_files")
        .unwrap();
    let mut f = File::open(device).unwrap();
    let mut buf = [0u8; BUFFER_SIZE];

    let mut recovered_count = 0;
    while let Ok(_) = f.read_exact(&mut buf) {
        if let Some(_start_index) = compare_headers(&buf, &recover_type.header) {
            recover_file(&f, buf, &recover_type, &output, recovered_count);
            recovered_count += 1;
            println!(
                "Recovered {} file at {:.2?}",
                Ordinal(recovered_count),
                now.elapsed()
            );
        }
    }
}

fn recover_file(
    mut f: &File,
    current_buf: [u8; BUFFER_SIZE],
    file_type: &FileTypeInfo,
    output: &str,
    recovered_count: usize,
) {
    let mut recovered_file = File::create(format!(
        "{}/recovered-{}.{}",
        output, recovered_count, file_type.ext
    ))
    .unwrap();
    let mut buf: [u8; BUFFER_SIZE] = current_buf;
    loop {
        match find_index_by_windowing(&buf, &file_type.end) {
            None => {
                recovered_file.write_all(&buf).unwrap();
            }
            Some(end_index) => {
                recovered_file
                    .write_all(&buf[..end_index + file_type.end.len()])
                    .unwrap();
                return;
            }
        }
        f.read_exact(&mut buf).unwrap();
    }
}
