use bindings::{
    ngx_array_t, ngx_buf_t, ngx_chain_t, ngx_http_headers_in_t, ngx_http_headers_out_t,
    ngx_list_part_t, ngx_list_push, ngx_list_t, ngx_palloc, ngx_pool_t, ngx_str_t, ngx_table_elt_t,
    u_char,
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

impl ngx_str_t {
    pub fn from_string(pool: *mut ngx_pool_t, data: String) -> Self {
        ngx_str_t {
            data: str_to_uchar(pool, data.as_str()),
            len: data.len() as _,
        }
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

impl IntoIterator for ngx_http_headers_out_t {
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

impl From<ngx_buf_t> for &[u8] {
    fn from(buf: ngx_buf_t) -> Self {
        if buf.pos.is_null() || buf.last.is_null() || buf.pos == buf.last {
            return Default::default();
        }
        unsafe { slice::from_raw_parts(buf.pos, buf.last.offset_from(buf.pos) as usize) }
    }
}

impl fmt::Display for ngx_buf_t {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "buffer: start.end {:p}/{:p}, pos.last {:p}/{:p}, len {}, file pos {}/{}, tag {:p}, \
             last_in_chain {}, last_buf {}, temporary {}, memory {}, recycled {}, flush {}, sync {} \
             last_shadow {}, temp file {}, content:\n{}",
            self.start,
            self.end,
            self.pos,
            self.last,
            self.size(),
            self.file_pos,
            self.file_last,
            self.tag,
            self.last_in_chain(),
            self.last_buf(),
            self.temporary(),
            self.memory(),
            self.recycled(),
            self.flush(),
            self.sync(),
            self.last_shadow(),
            self.temp_file(),
            String::from_utf8_lossy((*self).into())
        )
    }
}

impl ngx_buf_t {
    pub fn to_str(&self) -> &str {
        unsafe {
            let slice = slice::from_raw_parts(self.pos, self.size());
            return str::from_utf8(slice).unwrap();
        }
    }

    pub fn to_string(&self) -> String {
        return String::from(self.to_str());
    }

    // keep consistent with ngx_buf_in_memory
    pub fn in_memory(&self) -> bool {
        return self.temporary() == 1 || self.memory() == 1 || self.mmap() == 1;
    }

    // keep consistent with ngx_buf_size
    pub fn size(&self) -> usize {
        if self.in_memory() {
            unsafe { self.last.offset_from(self.pos) as usize }
        } else {
            (self.file_last - self.file_pos) as usize
        }
    }
}

impl fmt::Display for ngx_chain_t {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let buf = self.buf;
        unsafe {
            if let Some(val) = buf.as_ref() {
                write!(f, "{}\n", val);
            }
        }

        let mut cur = self.next;
        while !cur.is_null() {
            unsafe {
                if let Some(val) = (*cur).buf.as_ref() {
                    write!(f, "{}\n", val);
                }
                cur = (*cur).next;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn create_buf(content: &mut Vec<u8>, size: usize) -> ngx_buf_t {
        unsafe {
            let buf = ngx_buf_t {
                pos: content.as_mut_ptr(),
                last: content.as_mut_ptr().add(size),
                start: content.as_mut_ptr(),
                end: content.as_mut_ptr().add(size),
                file_pos: 0,
                file_last: 0,
                tag: std::ptr::null_mut(),
                file: std::ptr::null_mut(),
                shadow: std::ptr::null_mut(),
                _bitfield_1: ngx_buf_t::new_bitfield_1(1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0),
                num: 1,
            };
            buf
        }
    }
    #[test]
    fn test_display_buf() {
        let mut str_1 = "it.is.a.dump.string".as_bytes().to_vec();
        let mut buf_1 = Box::new(create_buf(&mut str_1, 10));
        assert_eq!(10, buf_1.size());
        // assert_eq!(6, buf_1.size_from_offset(4));
        let mut str_2 = "it.is.another.string".as_bytes().to_vec();
        let mut buf_2 = Box::new(create_buf(&mut str_2, 20));

        let chain_2 = ngx_chain_t {
            buf: buf_2.as_mut(),
            next: std::ptr::null_mut(),
        };
        let mut chain_2 = Box::new(chain_2);
        let chain = ngx_chain_t {
            buf: buf_1.as_mut(),
            next: chain_2.as_mut(),
        };

        println!("buf_1: {}", buf_1);
        println!("buf_2: {}", buf_2);
        println!("chain 1: \n{}", chain);
        println!("chain 2: \n{}", chain);
    }
}
