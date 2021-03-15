#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const NGX_READ_EVENT: EPOLL_EVENTS = (EPOLL_EVENTS_EPOLLIN | EPOLL_EVENTS_EPOLLRDHUP);
pub const NGX_WRITE_EVENT: EPOLL_EVENTS = EPOLL_EVENTS_EPOLLOUT;
pub const NGX_CLEAR_EVENT: EPOLL_EVENTS = EPOLL_EVENTS_EPOLLET;

pub fn ngx_add_event(ev: *mut ngx_event_t, event: ngx_int_t, flags: ngx_uint_t) -> ngx_int_t {
    unsafe { ngx_event_actions.add.unwrap()(ev, event, flags) }
}

pub fn ngx_del_event(ev: *mut ngx_event_t, event: ngx_int_t, flags: ngx_uint_t) -> ngx_int_t {
    unsafe { ngx_event_actions.del.unwrap()(ev, event, flags) }
}

#[inline]
pub fn ngx_event_del_timer(ev: *mut ngx_event_t) {
    unsafe {
        ngx_rbtree_delete(&mut ngx_event_timer_rbtree, &mut (*ev).timer);
        (*ev).set_timer_set(0);
    }
}

#[inline]
pub fn ngx_event_add_timer(ev: *mut ngx_event_t, timer: ngx_msec_t) {
    let key: ngx_msec_t = unsafe { ngx_current_msec } + timer;

    if unsafe { *ev }.timer_set() != 0 {
        /*
         * Use a previous timer value if difference between it and a new
         * value is less than NGX_TIMER_LAZY_DELAY milliseconds: this allows
         * to minimize the rbtree operations for fast connections.
         */

        let diff: ngx_msec_int_t = { key - unsafe { *ev }.timer.key } as _;

        if diff.abs() < NGX_TIMER_LAZY_DELAY as _ {
            return;
        }

        ngx_event_del_timer(ev);
    }

    unsafe {
        (*ev).timer.key = key;
        ngx_rbtree_insert(&mut ngx_event_timer_rbtree, &mut (*ev).timer);
        (*ev).set_timer_set(1);
    }
}
