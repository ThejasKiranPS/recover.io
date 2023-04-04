use std::{
    fs::{read, File},
    io::{BufRead, BufReader, Read, Write},
    process::exit,
    time::Instant,
};
fn main() {
    sudo::escalate_if_needed().unwrap();
    let now = Instant::now();
    let _crab = "./crab.jpg";
    let _usb = "/dev/sdd";
    let mut f = File::open(_usb).unwrap();
    let mut buf = [0u8; 256];
    let mut jpg_headers = [0u8; 8];
    let mut jpg_end = [0u8; 2];
    let mut png_headers = [0u8; 8];
    let mut png_end = [0u8; 4];

    hex::decode_to_slice("FFD8FFe000104A46", &mut jpg_headers).unwrap();
    hex::decode_to_slice("FFD9", &mut jpg_end).unwrap();
    hex::decode_to_slice("89504E470D0A1A0A", &mut png_headers).unwrap();
    hex::decode_to_slice("49454E44", &mut png_end).unwrap();

    let header = png_headers;
    let end = png_end;

    let mut recovered_count = 0;
    while let Ok(_) = f.read_exact(&mut buf) {
        if !buf.contains(&header[2]) {
            continue;
        }
        if let Some(pos) = find_index_by_windowing(&buf, &header) {
            println!("pos = {}", pos);
            println!("{:X?} contains jpg headers!", buf);
            let mut fnew = File::create(format!("recovered-{}.jpg", recovered_count)).unwrap();
            fnew.write_all(&buf[pos..]).unwrap();
            while let Ok(_) = f.read_exact(&mut buf) {
                if let Some(pos) = find_index_by_windowing(&buf, &end) {
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
