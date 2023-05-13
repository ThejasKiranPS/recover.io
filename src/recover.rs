use std::{time::Instant, fs::{DirBuilder, File}, io::{Read, Write}};

use crate::{utils::{compare_headers, find_index_by_windowing}, file_types::FileTypeInfo};

pub fn start_recover(device: String, recover_type: FileTypeInfo, output: String) {
    sudo::escalate_if_needed().unwrap();
    let now = Instant::now();
    DirBuilder::new()
        .recursive(true)
        .create("recovered_files")
        .unwrap();
    let mut f = File::open(device).unwrap();
    let mut buf = [0u8; 1024 * 16];

    let mut recovered_count = 0;
    while let Ok(_) = f.read_exact(&mut buf) {
        if let Some(pos) = compare_headers(&buf, &recover_type.header) {
            println!("pos = {}", pos);
            // println!("{:X?} pattern found", buf);
            let mut fnew = File::create(format!(
                "{}/recovered-{}.{}",
                output,recovered_count, recover_type.ext
            ))
            .unwrap();
            fnew.write_all(&buf[pos..]).unwrap();
            while let Ok(_) = f.read_exact(&mut buf) {
                if let Some(pos) = find_index_by_windowing(&buf, &recover_type.end) {
                    fnew.write_all(&buf[..pos + recover_type.end.len()]).unwrap();
                    println!("File recovered in {:.2?}", now.elapsed());
                    recovered_count += 1;
                    break;
                }
                fnew.write_all(&buf).unwrap();
            }
        }
    }
}