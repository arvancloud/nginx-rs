use bindings::{
    ngx_http_headers_in_t, ngx_list_part_t, ngx_list_t, ngx_str_t, ngx_table_elt_t, ngx_uint_t,
};
use std::marker::PhantomData;
use std::{slice, str};

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

    pub fn headers_iterator(&self) -> NgxListIterator {
        NgxListIterator::from_ngx_list(&self.headers)
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

#[deprecated(since = "0.7.0", note = "Please use ListIterator struct instead")]
pub struct NgxListIterator {
    done: bool,
    part: *const ngx_list_part_t,
    h: *const ngx_table_elt_t,
    i: ngx_uint_t,
}

impl<'a> ListIterator<'_> {
    pub fn from_ngx_list(list: *const ngx_list_t) -> Self {
        let part: *const ngx_list_part_t = unsafe { &(*list).part };
        ListIterator {
            done: false,
            part: part,
            h: unsafe { (*part).elts as *const ngx_table_elt_t },
            i: 0,
            phantom: PhantomData,
        }
    }
}

impl<'a> Iterator for ListIterator<'a> {
    type Item = (&'a str, &'a str);

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
            Some(unsafe { ((*header).key.to_str(), (*header).value.to_str()) })
        }
    }
}

impl NgxListIterator {
    pub fn from_ngx_list(list: *const ngx_list_t) -> Self {
        let part: *const ngx_list_part_t = unsafe { &(*list).part };
        NgxListIterator {
            done: false,
            part: part,
            h: unsafe { (*part).elts as *const ngx_table_elt_t },
            i: 0,
        }
    }
}

impl Iterator for NgxListIterator {
    type Item = (String, String);

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
            let header_name = unsafe { (*header).key };
            let header_value = unsafe { (*header).value };
            self.i += 1;
            Some((header_name.to_string(), header_value.to_string()))
        }
    }
}
