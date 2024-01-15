use std::io::{Error, Read, Write, self, ErrorKind};
use std::fs::{OpenOptions, read, remove_file, File};
use std::path::Path;

pub struct StorageManager {
    pub storage_size: usize,
    storage_size_current: usize,
    dir_flag: bool,
    file_dir: String,
}

impl StorageManager {
    pub fn new(max_size:usize) -> StorageManager {
        let op = StorageManager {
            storage_size: 0,
            storage_size_current: max_size,
            dir_flag: true,
            file_dir: "/home".to_string(),
        };

        return op;
    }

    pub fn write(&self, file_name:&str, buf: &Vec<u8>) -> Result<usize, Error> {
        let mut ret:usize = 0;
        let mut rc = Ok(ret);

        if file_name.is_empty() {
            return Err(Error::new(ErrorKind::InvalidInput, "[ERR] file name is empty!"));
        }

        let mut ctx = match OpenOptions::new()
                                    .append(true)
                                    .open(file_name) {
            Ok(file) => file,
            Err(err) => match err.kind() {
                ErrorKind::NotFound => match File::create(file_name) {
                    Ok(file) => file,
                    Err(err) => {
                        println!("[ERR] file create failed, {:?}", err);
                        return Err(err);
                    },
                },
                _ => {
                    let ue = Error::new(ErrorKind::Other,
                                    "[ERR] unkown other!");
                    return Err(ue);
                },
            },
        };

        if buf.len() == 0 {
            return rc;
        }

        let buf_slice:&[u8] = &buf;
        rc = match ctx.write_all(buf_slice) {
            Ok(_) => Ok(buf_slice.len()),
            Err(err) => {
                println!("[ERR] Write binary failed, {:?}", err);
                return Err(err);
            },
        };

        return rc;
    }

    pub fn is_dir_exist(&self, dir_name:&str) -> Result<bool, Error> {
        let mut ret:bool = true;
        let mut rc = Ok(ret);

        if dir_name.is_empty() {
            rc = Err(Error::new(ErrorKind::InvalidInput,
                            "[ERR] dir name is emplty!"));
            return rc;
        }

        ret = Path::new(dir_name).exists();

        return Ok(ret);
    }

    pub fn is_file_exist(&self, file_name:&str) -> Result<bool, Error> {
        let mut ret:bool = true;
        let mut rc = Ok(ret);

        if file_name.is_empty() {
            rc = Err(Error::new(ErrorKind::InvalidInput,
                            "[ERR] file name is emplty!"));
            return rc;
        }

        ret = Path::new(file_name).exists() &&
              std::fs::metadata(file_name)
                        .map(|metadata| metadata.is_file())
                        .unwrap_or(false);

        return Ok(ret);
    }

    pub fn is_file_full(&self, file_name:&str) -> Result<bool, Error> {
        let mut ret:bool = false;
        let mut rc = Ok(ret);

        if file_name.is_empty() {
            let ue = Error::new(ErrorKind::InvalidInput,
                        "[ERR] file name is empty!");
            return Err(ue);
        }

        let file_sz = match std::fs::metadata(file_name) {
            Ok(md) => md.len(),
            Err(err) => {
                println!("[ERR] file metadata parsing error! {:?}", err);
                return Err(err);
            },
        };

        rc = Ok(file_sz >= (self.storage_size_current as u64));

        return rc;
    }

    pub fn set_storage_size(&mut self, current_size:usize) {
        self.storage_size_current = current_size;
    }

    pub fn get_storage_size(&self) -> usize {
        return self.storage_size_current;
    }
}

#[cfg(test)]
mod test_storage_manager {
    use std::io::{Error as IOError, ErrorKind};
    use std::env;
    use std::fs::remove_file;
    use std::ops::Add;

    use super::*;

    fn f_test_create_zeros_file(file_name: &str, file_size_bytes: usize) -> Result<usize, Error> {
        let mut ret:usize = file_size_bytes;
        let mut rc = Ok(ret);

        let mut f:File =  File::create(file_name)?;

        let buffer = vec![0; file_size_bytes];
        rc = match f.write_all(&buffer) {
            Ok(_) => Ok(file_size_bytes),
            Err(error) => {
                println!("write failed! {:?}", error);
                return Err(error);
            },
        };

        return rc;
    }

    fn f_test_delete_file(file_name:&str) -> Result<(), Error> {
        match remove_file(file_name) {
            Ok(_) => {
                return Ok(());
            },
            Err(error) => {
                println!("delete file failed! {:?}", error);
                return Err(error);
            }
        };
    }

    #[test]
    fn test_is_dir() {
        let mut np:StorageManager = StorageManager::new(10000);
        let ret = np.is_dir_exist("/home/haochenwei").unwrap();
        assert_eq!(ret, true);

        let ret = np.is_dir_exist("/home/xxx").unwrap();
        assert_eq!(ret, false);

        let ret = np.is_dir_exist("");
        match ret {
            Ok(_) => panic!("Test failed!"),
            Err(e) => {
                match e.kind() {
                    ErrorKind::InvalidInput => {
                        assert!(true);
                    }
                    _ => panic!("test failed!"),
                }
            },
         }

        let ret = np.is_dir_exist("dd").unwrap();
        assert_eq!(ret, false);
    }

    #[test]
    fn test_is_file() {
        let mut np:StorageManager = StorageManager::new(10000);
        let ret = np.is_file_exist("/home/haochenwei/ddr_init.o").unwrap();
        assert_eq!(ret, true);

        let ret = np.is_file_exist("/home/haochenwei/hel.o").unwrap();
        assert_eq!(ret, false);

        let ret = np.is_file_exist("");
        match ret {
            Ok(_) => panic!("Test failed!"),
            Err(e) => {
                match e.kind() {
                    ErrorKind::InvalidInput => {
                        assert!(true);
                    }
                    _ => panic!("test failed!"),
                }
            },
         }

        let ret = np.is_dir_exist("dd").unwrap();
        assert_eq!(ret, false);
    }

    #[test]
    fn test_is_file_full() {

        let max_size:usize = 1225;

        // get home path
        let mut file_path:String = match env::var("HOME") {
            Ok(value) => {
                value.clone()
            }
            Err(e) => {
                println!("Couldn't read HOME ({})", e);
                panic!("test failed!");
            }
        };

        file_path = file_path + "/temp_556677.o";

        // create file
        f_test_create_zeros_file(&file_path, max_size).unwrap();

        // test is file full
        let mut np:StorageManager = StorageManager::new(max_size);

        let mut ret = np.is_file_full(&file_path).unwrap();
        assert_eq!(ret, true);

        np.set_storage_size(max_size + 1);
        ret = np.is_file_full(&file_path).unwrap();
        assert_eq!(ret, false);

        np.set_storage_size(max_size - 1);
        ret = np.is_file_full(&file_path).unwrap();
        assert_eq!(ret, true);

        let _ = f_test_delete_file(&file_path);
    }


}
