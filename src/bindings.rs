#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use libc::*;

type u_char = libc::c_uchar;
type u_short = libc::c_ushort;
type u_long = libc::c_ulong;

// https://www.gnu.org/software/libc/manual/html_node/Atomic-Types.html
pub type sig_atomic_t = libc::c_int;

// https://docs.rs/libpcre-sys/0.2.2/libpcre_sys/enum.pcre.html
#[derive(Debug, Clone, Copy)]
pub enum pcre { }

// https://docs.rs/libpcre-sys/0.2.2/libpcre_sys/struct.pcre_extra.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pcre_extra {
    pub flags: libc::c_ulong,
    pub study_data: *mut libc::c_void,
    pub match_limit_: libc::c_ulong,
    pub callout_data: *mut libc::c_void,
    pub tables: *const libc::c_uchar,
    pub match_limit_recursion_: libc::c_ulong,
    pub mark: *mut *mut libc::c_uchar,
    pub executable_jit: *mut libc::c_void
}

pub type __builtin_va_list = [__va_list_tag; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
