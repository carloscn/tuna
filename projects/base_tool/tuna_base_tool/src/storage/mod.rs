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

        return rc;
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

        rc = Ok(file_sz > (self.storage_size_current as u64));

        return rc;
    }

    pub fn set_storage_size(&mut self, current_size:usize) {
        self.storage_size_current = current_size;
    }

    pub fn get_storage_size(&self) -> usize {
        return self.storage_size_current;
    }
}
