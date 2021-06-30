use bindings::{
    ngx_array_t, ngx_http_headers_in_t, ngx_http_headers_out_t, ngx_list_part_t, ngx_list_push,
    ngx_list_t, ngx_palloc, ngx_pool_t, ngx_str_t, ngx_table_elt_t, u_char,
};
use std::convert::{From, TryFrom};
use std::ffi::OsStr;
use std::fmt;
use std::ptr::copy_nonoverlapping;
use std::{slice, str};

pub struct Header(ngx_table_elt_t);

impl From<ngx_str_t> for &[u8] {
    fn from(s: ngx_str_t) -> Self {
        if s.len == 0 || s.data.is_null() {
            return Default::default();
        }
        unsafe { slice::from_raw_parts(s.data, s.len as usize) }
    }
}

#[cfg(any(unix, target_os = "redox"))]
impl From<ngx_str_t> for &OsStr {
    fn from(s: ngx_str_t) -> Self {
        std::os::unix::ffi::OsStrExt::from_bytes(s.into())
    }
}

impl fmt::Display for ngx_str_t {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy((*self).into()))
    }
}

impl TryFrom<ngx_str_t> for &str {
    type Error = str::Utf8Error;

    fn try_from(s: ngx_str_t) -> Result<Self, Self::Error> {
        str::from_utf8(s.into())
    }
}

impl TryFrom<ngx_str_t> for String {
    type Error = std::string::FromUtf8Error;

    fn try_from(s: ngx_str_t) -> Result<Self, Self::Error> {
        let bytes: &[u8] = s.into();
        String::from_utf8(bytes.into())
    }
}

impl ngx_table_elt_t {
    pub fn to_string(&self) -> String {
        self.value.to_string()
    }
}

impl ngx_array_t {
    pub fn table_elt_to_string_vec(&self) -> Vec<String> {
        let mut ret = Vec::new();
        let t = self.elts as *mut *mut ngx_table_elt_t;
        if t.is_null() {
            return ret;
        }
        let mut t = unsafe { *t };
        for _ in 0..self.nelts {
            if !t.is_null() {
                ret.push(unsafe { *t }.value.to_string());
            }
            t = unsafe { t.add(1) };
        }
        ret
    }
}

impl ngx_http_headers_in_t {
    pub fn host_str(&self) -> String {
        if let Some(host) = unsafe { self.host.as_ref() } {
            host.to_string()
        } else {
            Default::default()
        }
    }

    pub fn add(&mut self, pool: *mut ngx_pool_t, key: &str, value: &str) -> Option<()> {
        let table: *mut ngx_table_elt_t = unsafe { ngx_list_push(&mut self.headers) as _ };
        add_to_ngx_table(table, pool, key, value)
    }
}

impl ngx_http_headers_out_t {
    pub fn add(&mut self, pool: *mut ngx_pool_t, key: &str, value: &str) -> Option<()> {
        let table: *mut ngx_table_elt_t = unsafe { ngx_list_push(&mut self.headers) as _ };
        add_to_ngx_table(table, pool, key, value)
    }
}

fn add_to_ngx_table(
    table: *mut ngx_table_elt_t,
    pool: *mut ngx_pool_t,
    key: &str,
    value: &str,
) -> Option<()> {
    if table.is_null() {
        return None;
    }
    unsafe { table.as_mut() }.map(|table| {
        table.hash = 1;
        table.key.len = key.len() as _;
        table.key.data = str_to_uchar(pool, key);
        table.value.len = value.len() as _;
        table.value.data = str_to_uchar(pool, value);
        table.lowcase_key = str_to_uchar(pool, String::from(key).to_ascii_lowercase().as_str());
    })
}

impl IntoIterator for ngx_http_headers_in_t {
    type Item = Header;
    type IntoIter = ListIterator;

    fn into_iter(self) -> Self::IntoIter {
        ListIterator::from_ngx_list(self.headers)
    }
}

pub struct ListIterator {
    part: ngx_list_part_t,
    h: *mut ngx_table_elt_t,
    i: isize,
}

impl ListIterator {
    pub fn from_ngx_list(list: ngx_list_t) -> Self {
        let part = list.part;
        ListIterator {
            part: part,
            h: part.elts as _,
            i: 0,
        }
    }
}

impl Iterator for ListIterator {
    type Item = Header;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.part.nelts as _ {
            if let Some(next) = unsafe { self.part.next.as_ref() } {
                self.part = *next;
                self.h = self.part.elts as _;
                self.i = 0;
            } else {
                return None;
            }
        }
        let header = unsafe { *self.h.offset(self.i) };
        self.i += 1;
        Some(Header::from(header))
    }
}

impl From<ngx_table_elt_t> for Header {
    fn from(table: ngx_table_elt_t) -> Self {
        Self(table)
    }
}

impl Header {
    pub fn key(&self) -> String {
        self.0.key.to_string()
    }

    pub fn value(&self) -> String {
        self.0.value.to_string()
    }

    pub fn into_inner(self) -> ngx_table_elt_t {
        self.0
    }
}

fn str_to_uchar(pool: *mut ngx_pool_t, data: &str) -> *mut u_char {
    let ptr: *mut u_char = unsafe { ngx_palloc(pool, data.len() as _) as _ };
    unsafe {
        copy_nonoverlapping(data.as_ptr(), ptr, data.len());
    }
    ptr
}
