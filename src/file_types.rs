pub enum FileType {
    JPG,
    PNG,
}
pub struct FileTypeInfo {
    pub header: Vec<u8>,
    pub end: Vec<u8>,
    pub ext: String,
}

impl std::fmt::Debug for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileType::JPG => write!(f, "JPG"),
            FileType::PNG => write!(f, "PNG"),
        }
    }
}

impl Clone for FileType {
    fn clone(&self) -> Self {
        match self {
            FileType::JPG => FileType::JPG,
            FileType::PNG => FileType::PNG,
        }
    }
}

impl From<String> for FileType {
    fn from(file_type: String) -> Self {
        match file_type.as_str() {
            "jpg" => FileType::JPG,
            "png" => FileType::PNG,
            _ => panic!("File type not supported"),
        }
    }
}

impl From<FileType> for FileTypeInfo {
    fn from(file_type: FileType) -> Self {
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
}

impl std::fmt::Debug for FileTypeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ header: {:?}, end: {:?}, ext: {} }}",
            self.header, self.end, self.ext
        )
    }
}

impl Clone for FileTypeInfo {
    fn clone(&self) -> Self {
        FileTypeInfo {
            header: self.header.clone(),
            end: self.end.clone(),
            ext: self.ext.clone(),
        }
    }
}

impl From<String> for FileTypeInfo {
    fn from(file_type: String) -> Self {
        let file_type: FileType = file_type.into();
        file_type.into()
    }
}
