use std::{
    fs::{File, DirBuilder},
    io::{Read, Write},
    process::exit,
    time::Instant,
};
//import fat32 module
mod fat32;
mod file_types;

use crate::file_types::{get_file_type, FileType};
fn main() {

    sudo::escalate_if_needed().unwrap();
    let now = Instant::now();
    let _usb = "/dev/sdd";
    DirBuilder::new()
        .recursive(true)
        .create("recovered_files").unwrap();
    let mut f = File::open(_usb).unwrap();
    let mut buf = [0u8; 4096];

    let jpg = get_file_type(FileType::JPG);
    let png = get_file_type(FileType::PNG);

    let recover_type = png;

    let mut recovered_count = 0;
    while let Ok(_) = f.read_exact(&mut buf) {
        if let Some(pos) = compare_headers(&buf, &recover_type.header) {
            println!("pos = {}", pos);
            // println!("{:X?} pattern found", buf);
            let mut fnew = File::create(format!("recovered_files/recovered-{}.png", recovered_count)).unwrap();
            fnew.write_all(&buf[pos..]).unwrap();
            while let Ok(_) = f.read_exact(&mut buf) {
                if let Some(pos) = find_index_by_windowing(&buf, &recover_type.end) {
                    fnew.write_all(&buf[..pos + 2]).unwrap();
                    println!("File recovered in {:.2?}", now.elapsed());
                    recovered_count += 1;
                    break;
                }
                fnew.write_all(&buf).unwrap();
            }
        }
    }
}

fn find_index(array: &[u8], sub_array: &[u8]) -> Option<usize> {
    for i in 0..array.len() - sub_array.len() + 1 {
        if array[i..i + sub_array.len()] == *sub_array {
            return Some(i);
        }
    }
    None
}

fn find_index_by_windowing(array: &[u8], sub_array: &[u8]) -> Option<usize> {
    array
        .windows(sub_array.len())
        .position(|data| data == sub_array)
}
fn compare_headers(array: &[u8], sub_array: &[u8]) -> Option<usize> {
    let mut sub_iter = sub_array.iter();
    for element in &array[0..sub_array.len() - 1] {
        if element != sub_iter.next()? {
            return None;
        }
    }
    Some(0)
}

#[cfg(test)]
mod tests {
    use crate::find_index_by_windowing;

    #[test]
    fn test_indexing() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let sub_arr = [4, 5, 6];

        assert!(find_index_by_windowing(&arr, &sub_arr) == Some(3))
    }
}
