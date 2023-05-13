use std::io::{SeekFrom, Read, Seek};

use std::fs::File;

pub fn read_cluster_size(fat32: &mut File) -> u32 {
    let mut buf = [0u8; 512];
    fat32.seek(SeekFrom::Start(11)).unwrap();
    fat32.read_exact(&mut buf).unwrap();
    let cluster_size = u16::from_le_bytes([buf[0x0B], buf[0x0A]]);
    cluster_size as u32
}

// 1. create a function called read_cluster_size
// 2. the function takes a mutable reference to a File as an argument
// 3. the function returns a u32
// 4. create a mutable array of 512 bytes
// 5. seek to the 11th byte of the file
// 6. read 512 bytes from the file into the array
// 7. create a variable called cluster_size and assign it the value of the 11th and 12th bytes of the array
// 8. return the cluster_size variable as a u32
// 9. end the function

// Create code to read cluster by cluster in a loop for fat32 file system
fn read_cluster(fat32: &mut File, cluster_size: u32, cluster: u32) -> Vec<u8> {
    let mut buf = vec![0u8; cluster_size as usize];
    let offset = cluster_size * cluster;
    fat32.seek(SeekFrom::Start(offset.into())).unwrap();
    fat32.read_exact(&mut buf).unwrap();
    buf
}