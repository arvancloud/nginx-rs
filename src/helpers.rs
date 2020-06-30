use bindings::{
    ngx_http_headers_in_t, ngx_http_request_t, ngx_list_part_t, ngx_list_push, ngx_list_t,
    ngx_palloc, ngx_str_t, ngx_table_elt_t, ngx_uint_t, u_char,
};
use std::convert::From;
use std::marker::PhantomData;
use std::ptr::copy_nonoverlapping;
use std::{slice, str};

pub struct Header(*const ngx_table_elt_t);

impl ngx_str_t {
    pub fn to_str(&self) -> &str {
        let bytes = unsafe { slice::from_raw_parts(self.data, self.len as usize) };
        str::from_utf8(bytes).unwrap_or_default()
    }

    pub fn to_string(&self) -> String {
        String::from(self.to_str())
    }
}

impl ngx_http_headers_in_t {
    pub fn host_str(&self) -> &str {
        if self.host.is_null() {
            return "";
        }
        unsafe { (*self.host).value.to_str() }
    }

    pub fn to_iterator(&self) -> ListIterator<'_> {
        ListIterator::from_ngx_list(&self.headers)
    }
}

pub struct ListIterator<'a> {
    done: bool,
    part: *const ngx_list_part_t,
    h: *const ngx_table_elt_t,
    i: ngx_uint_t,
    phantom: PhantomData<&'a ()>,
}

impl<'a> ListIterator<'_> {
    pub fn from_ngx_list(list: *const ngx_list_t) -> Self {
        let part: *const ngx_list_part_t = unsafe { &(*list).part };
        ListIterator {
            done: false,
            part: part,
            h: unsafe { (*part).elts as *const _ },
            i: 0,
            phantom: PhantomData,
        }
    }
}

impl<'a> Iterator for ListIterator<'a> {
    type Item = Header;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            if self.i >= unsafe { (*self.part).nelts } {
                if unsafe { (*self.part).next.is_null() } {
                    self.done = true;
                    return None;
                }
                self.part = unsafe { (*self.part).next };
                self.h = unsafe { (*self.part).elts as *mut ngx_table_elt_t };
                self.i = 0;
            }
            let header = unsafe { self.h.offset(self.i as isize) };
            self.i += 1;
            Some(Header::from(header))
        }
    }
}

impl From<*const ngx_table_elt_t> for Header {
    fn from(table: *const ngx_table_elt_t) -> Self {
        Self(table)
    }
}

impl Header {
    pub fn new(req: &mut ngx_http_request_t) -> Option<Self> {
        let table: *const ngx_table_elt_t =
            unsafe { ngx_list_push(&mut req.headers_in.headers) as *const _ };
        if table.is_null() {
            return None;
        }
        Some(Self(table))
    }

    pub fn key(&self) -> &str {
        unsafe { (*self.0).key.to_str() }
    }

    pub fn value(&self) -> &str {
        unsafe { (*self.0).value.to_str() }
    }

    fn str_to_uchar(req: &ngx_http_request_t, data: &str) -> *mut u_char {
        let ptr = unsafe { ngx_palloc(req.pool, data.len() as _) };
        unsafe {
            copy_nonoverlapping(data.as_ptr() as *const _, ptr, data.len());
        }
        ptr as *mut _
    }

    pub fn set(&self, req: &ngx_http_request_t, key: &str, lowercase_key: &str, value: &str) {
        unsafe { *self.0 }.hash = 1;
        unsafe { *self.0 }.lowcase_key = Self::str_to_uchar(req, lowercase_key);
        unsafe { *self.0 }.key = ngx_str_t {
            data: Self::str_to_uchar(req, key),
            len: key.len() as _,
        };
        unsafe { *self.0 }.value = ngx_str_t {
            data: Self::str_to_uchar(req, value),
            len: value.len() as _,
        };
    }

    pub fn into_inner(self) -> *const ngx_table_elt_t {
        self.0
    }
}
