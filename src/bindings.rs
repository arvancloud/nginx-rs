#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const NGX_READ_EVENT: EPOLL_EVENTS = (EPOLL_EVENTS_EPOLLIN | EPOLL_EVENTS_EPOLLRDHUP);
pub const NGX_WRITE_EVENT: EPOLL_EVENTS = EPOLL_EVENTS_EPOLLOUT;
pub const NGX_CLEAR_EVENT: EPOLL_EVENTS = EPOLL_EVENTS_EPOLLET;

extern "C" {
    pub fn ngx_event_add_timer_wrapper(ev: *mut ngx_event_t, timer: ngx_msec_t);
    pub fn ngx_event_del_timer_wrapper(ev: *mut ngx_event_t);
}
