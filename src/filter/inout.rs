use std::marker::PhantomData;

pub struct InOut<'a> {
	ptr: *mut AVFilterInOut,

	_own: bool,
	_marker: PhantomData<&'a ()>,
}

impl<'a> InOut<'a> {
	pub unsafe fn wrap(ptr: *mut AVFilterInOut) -> Self {
		InOut { ptr: ptr, _own: false, _marker: PhantomData }
	}

	pub unsafe fn own(ptr: *mut AVFilterInOut) -> Self {
		InOut { ptr: ptr, _own: true, _marker: PhantomData }
	}

	pub unsafe fn as_ptr(&self) -> *const AVFilterInOut {
		self.ptr as *const _
	}

	pub unsafe fn as_mut_ptr(&mut self) -> *mut AVFilterInOut {
		self.ptr
	}

	pub unsafe fn take(mut self) -> *mut AVFilterInOut {
		self._own = false;
		self.ptr
	}
}
