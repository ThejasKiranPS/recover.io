pub enum FileType {
    JPG,
    PNG,
}

pub struct FileTypeInfo {
    pub header: Vec<u8>,
    pub end: Vec<u8>,
    pub ext: String,
}


// write get_file_type function that returns a FILE_TYPE_INFO struct and accepts a FILE_TYPE enum as an argument
pub fn get_file_type(file_type: FileType) -> FileTypeInfo {

    match file_type {
        FileType::JPG => FileTypeInfo {
            header: vec![255, 216, 255, 224, 0, 16, 74, 70],
            end: vec![255, 217],
            ext: String::from("jpg"),
        },
        FileType::PNG => FileTypeInfo {
            header: vec![137, 80, 78, 71, 13, 10, 26, 10],
            end: vec![73, 69, 78, 68],
            ext: String::from("png"),
        },
    }
}