use libc::{c_void, c_int, c_uint};

pub type AVThreadMessageQueue = c_void;

pub const AV_THREAD_MESSAGE_NONBLOCK: c_uint = 1;

extern {
	pub fn av_thread_message_queue_alloc(mq: *mut *mut AVThreadMessageQueue, nelem: c_uint, elsize: c_uint) -> c_int;
	pub fn av_thread_message_queue_free(mq: *mut *mut AVThreadMessageQueue);
	pub fn av_thread_message_queue_send(mq: *mut AVThreadMessageQueue, msg: *mut c_void, flags: c_uint) -> c_int;
	pub fn av_thread_message_queue_recv(mq: *mut AVThreadMessageQueue, msg: *mut c_void, flags: c_uint) -> c_int;
	pub fn av_thread_message_queue_set_err_send(mq: *mut AVThreadMessageQueue, err: c_int);
	pub fn av_thread_message_queue_set_err_recv(mq: *mut AVThreadMessageQueue, err: c_int);
}
