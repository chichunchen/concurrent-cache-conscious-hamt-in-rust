extern crate libc;
use std::sync::atomic::{AtomicUsize,Ordering,AtomicPtr};
use std::mem;

pub struct Allocator<T> {
    buf: AtomicPtr<T>,
    capacity: usize,
    n: AtomicUsize 
}

impl<T> Allocator<T> {
    pub fn new(size: usize) -> Self {
        Allocator {
            buf: AtomicPtr::new(unsafe {libc::calloc(size as libc::size_t, mem::size_of::<T>() as libc::size_t) as *mut T}),
            capacity: size,
            n: AtomicUsize::new(0)
        }
    }

    pub fn alloc(&self, obj: T) -> &mut T {
        let i = self.n.fetch_add(1, Ordering::Relaxed);
        assert!(i < self.capacity);
        let buf = self.buf.load(Ordering::Relaxed);
        unsafe {*buf.offset(i as isize) = obj;}
        unsafe {&mut *buf.offset(i as isize)}
    }
}
