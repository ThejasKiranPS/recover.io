use std::{
    fs::{read, File},
    io::{BufRead, BufReader, Read, Write},
    process::exit,
    time::Instant,
};
fn main() {
    let now = Instant::now();
    let _crab = "./crab.jpg";
    let _usb = "/dev/sdd";
    let mut f = File::open(_usb).unwrap();
    let mut fnew = File::create("recovered.jpg").unwrap();
    let mut buf = [0u8; 256];
    let mut jpg_headers = [0u8; 8];
    let mut jpg_end = [0u8; 2];
    hex::decode_to_slice("FFD8FFe000104A46", &mut jpg_headers).unwrap();
    hex::decode_to_slice("FFD9", &mut jpg_end).unwrap();
    while let Ok(_) = f.read_exact(&mut buf) {
        if !buf.contains(&jpg_headers[2]) {
            continue;
        }
        if buf
            .windows(jpg_headers.len())
            .any(|data| data == jpg_headers)
        {
            println!("{:X?} contains jpg headers!", buf);
            fnew.write_all(&buf).unwrap();
            while let Ok(_) = f.read_exact(&mut buf) {
                fnew.write_all(&buf).unwrap();
                if buf.windows(jpg_end.len()).any(|data| data == jpg_end) {
                    println!("File recovered in {:.2?}", now.elapsed());
                    exit(0);
                }
            }
        }
    }
}
