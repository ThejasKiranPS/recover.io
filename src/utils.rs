use std::fs::DirBuilder;


pub fn create_dir(path: &str) {
    DirBuilder::new()
        .recursive(true)
        .create(path)
        .unwrap();
}

pub fn create_dir_or_default(path: Option<String>) -> String {
    let path = path.unwrap_or("recovered_files".to_string());
    create_dir(&path);
    path
}

pub fn compare_headers(array: &[u8], sub_array: &[u8]) -> Option<usize> {
    let mut sub_iter = sub_array.iter();
    for element in &array[0..sub_array.len() - 1] {
        if element != sub_iter.next()? {
            return None;
        }
    }
    Some(0)
}

pub fn find_index_by_windowing(array: &[u8], sub_array: &[u8]) -> Option<usize> {
    array
        .windows(sub_array.len())
        .position(|data| data == sub_array)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_index_by_windowing_indexing() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let sub_arr = [4, 5, 6];

        assert!(find_index_by_windowing(&arr, &sub_arr) == Some(3))
    }

    #[test]
    fn test_compare_headers() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let sub_arr = [1,2,3];

        assert!(compare_headers(&arr, &sub_arr) == Some(0))
    }
}