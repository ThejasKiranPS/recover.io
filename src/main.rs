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
    hex::decode_to_slice("FFD8FFe000104A46", &mut jpg_headers).unwrap();
    hex::decode_to_slice("FFD9", &mut jpg_end).unwrap();

    let mut recovered_count = 0;
    while let Ok(_) = f.read_exact(&mut buf) {
        if !buf.contains(&jpg_headers[2]) {
            continue;
        }
        if let Some(pos) = find_index(&buf, &jpg_headers) {
            println!("pos = {}", pos);
            println!("{:X?} contains jpg headers!", buf);
            let mut fnew = File::create(format!("recovered-{}.jpg", recovered_count)).unwrap();
            fnew.write_all(&buf[pos..]).unwrap();
            while let Ok(_) = f.read_exact(&mut buf) {
                if let Some(pos) = find_index(&buf, &jpg_end) {
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
