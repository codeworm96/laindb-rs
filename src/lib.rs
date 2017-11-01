extern crate libc;
use libc::{c_int, c_void, size_t};
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr::copy_nonoverlapping;

pub enum Mode {
    Open,
    New,
    Create,
}

fn mode_to_int(mode: Mode) -> c_int {
    match mode {
        Mode::Open => 1,
        Mode::New => 2,
        Mode::Create => 3,
    }
}

#[link(name = "laindb", kind = "static")]
extern "C" {
    fn laindb_new(name: *const c_char, mode: c_int) -> *mut c_void;
    fn laindb_get(db: *const c_void, key: *const c_char) -> *mut c_void;
    fn laindb_put(db: *const c_void, key: *const c_char, value: *const u8, len: size_t) -> c_void;
    fn laindb_erase(db: *const c_void, key: *const c_char) -> c_void;
    fn laindb_drop(db: *mut c_void) -> c_void;
    fn laindb_slice_len(slice: *const c_void) -> size_t;
    fn laindb_slice_raw(slice: *const c_void) -> *const u8;
    fn laindb_slice_drop(slice: *mut c_void) -> c_void;
}

pub struct Laindb {
    db: *mut c_void,
}

impl Laindb {
    pub fn new(name: &str, mode: Mode) -> Self {
        let c_name = CString::new(name).unwrap();
        Laindb {
            db: unsafe { laindb_new(c_name.as_ptr(), mode_to_int(mode)) },
        }
    }

    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        let c_key = match CString::new(key) {
            Ok(s) => s,
            Err(_) => return None,
        };
        unsafe {
            let slice = laindb_get(self.db, c_key.as_ptr());
            if !slice.is_null() {
                let len = laindb_slice_len(slice) as usize;
                let mut res = Vec::with_capacity(len);
                let raw = laindb_slice_raw(slice);
                copy_nonoverlapping(raw, res.as_mut_ptr(), len);
                laindb_slice_drop(slice);
                res.set_len(len);
                Some(res)
            } else {
                None
            }
        }
    }

    pub fn put(&self, key: &str, value: &[u8]) {
        let c_key = CString::new(key).unwrap();
        let pval = value.as_ptr();
        let len = value.len() as size_t;
        unsafe {
            laindb_put(self.db, c_key.as_ptr(), pval, len);
        }
    }

    pub fn erase(&self, key: &str) {
        let c_key = CString::new(key).unwrap();
        unsafe {
            laindb_erase(self.db, c_key.as_ptr());
        }
    }
}

impl Drop for Laindb {
    fn drop(&mut self) {
        unsafe {
            laindb_drop(self.db);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn basic() {
        let db = Laindb::new("test", Mode::Create);
        assert_eq!(db.get("1"), None);
        let d1 = vec![1];
        db.put("1", &d1);
        assert_eq!(db.get("1"), Some(d1));
        let d2 = vec![2];
        db.put("1", &d2);
        assert_eq!(db.get("1"), Some(d2));
        db.erase("1");
        assert_eq!(db.get("1"), None);
    }
}
