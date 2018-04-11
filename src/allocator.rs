use core::mem;
use core::marker::PhantomData;
use std::heap::{Alloc, Layout, Heap};
use core::slice;
use core::ops::{Index, Deref, DerefMut};
use core::ptr::{Unique, self};

const CAP_SIZE: usize = 16;

#[derive(Debug)]
pub struct Allocator<T> {
    ptr: Unique<T>,
    cap: usize,
}

impl<T> Allocator<T> {
    /// Like `with_capacity` but parameterized over the choice of
    /// allocator for the returned Allocator.
    #[inline]
    pub fn with_capacity(cap: usize) -> Self {
        Allocator::allocate_in(cap, false)
    }

    /// Like `with_capacity_zeroed` but parameterized over the choice
    /// of allocator for the returned Allocator.
    #[inline]
    pub fn with_capacity_zeroed(cap: usize) -> Self {
        Allocator::allocate_in(cap, true)
    }

    fn allocate_in(cap: usize, zeroed: bool) -> Self {
        unsafe {
            let elem_size = mem::size_of::<T>();

            // TODO
            let alloc_size = cap.checked_mul(elem_size).expect("capacity overflow");
            //alloc_guard(alloc_size).expect("capacity overflow");

            // handles ZSTs and `cap = 0` alike
            let ptr = if alloc_size == 0 {
                mem::align_of::<T>() as *mut u8
            } else {
                let align = mem::align_of::<T>();
                let result = if zeroed {
                    Heap.alloc_zeroed(Layout::from_size_align(alloc_size, align).unwrap())
                } else {
                    Heap.alloc(Layout::from_size_align(alloc_size, align).unwrap())
                };
                match result {
                    Ok(ptr) => ptr,
                    Err(err) => Heap.oom(err),
                }
            };

            Allocator {
                ptr: Unique::new_unchecked(ptr as *mut _),
                cap,
            }
        }
    }
}

impl<T> Allocator<T> {
    pub unsafe fn from_raw_parts(ptr: *mut T, cap: usize) -> Self {
        Allocator {
            ptr: Unique::new_unchecked(ptr),
            cap,
        }
    }

    /// Converts a `Box<[T]>` into a `Allocator<T>`.
    pub fn from_box(mut slice: Box<[T]>) -> Self {
        unsafe {
            let result = Allocator::from_raw_parts(slice.as_mut_ptr(), slice.len());
            mem::forget(slice);
            result
        }
    }
}

impl<T> Allocator<T> {
    /// Gets a raw pointer to the start of the allocation. Note that this is
    /// Unique::empty() if `cap = 0` or T is zero-sized. In the former case, you must
    /// be careful.
    pub fn ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }

    /// Gets the capacity of the allocation.
    ///
    /// This will always be `usize::MAX` if `T` is zero-sized.
    #[inline(always)]
    pub fn cap(&self) -> usize {
        if mem::size_of::<T>() == 0 {
            !0
        } else {
            self.cap
        }
    }

    fn current_layout(&self) -> Option<Layout> {
        if self.cap == 0 {
            None
        } else {
            // We have an allocated chunk of memory, so we can bypass runtime
            // checks to get our current layout.
            unsafe {
                let align = mem::align_of::<T>();
                let size = mem::size_of::<T>() * self.cap;
                Some(Layout::from_size_align_unchecked(size, align))
            }
        }
    }
}

impl<T> Allocator<T> {
    /// Converts the entire buffer into `Box<[T]>`.
    ///
    /// While it is not *strictly* Undefined Behavior to call
    /// this procedure while some of the RawVec is uninitialized,
    /// it certainly makes it trivial to trigger it.
    ///
    /// Note that this will correctly reconstitute any `cap` changes
    /// that may have been performed. (see description of type for details)

    pub unsafe fn into_box(self) -> Box<[T]> {
        // NOTE: not calling `cap()` here, actually using the real `cap` field!
        let slice = slice::from_raw_parts_mut(self.ptr(), self.cap);
        let output: Box<[T]> = Box::from_raw(slice);
        mem::forget(self);
        output
    }
}


impl<T> Allocator<T> {
    /// Frees the memory owned by the RawVec *without* trying to Drop its contents.
    pub unsafe fn dealloc_buffer(&mut self) {
        let elem_size = mem::size_of::<T>();
        if elem_size != 0 {
            if let Some(layout) = self.current_layout() {
                let ptr = self.ptr() as *mut u8;
                Heap.dealloc(ptr, layout);
            }
        }
    }
}

impl<T> Drop for Allocator<T> {
    /// Frees the memory owned by the RawVec *without* trying to Drop its contents.
    fn drop(&mut self) {
        unsafe { self.dealloc_buffer(); }
    }
}

impl<T> Allocator<T> {
    pub fn update(&mut self, index: i32, value: T) {
        unsafe {
            let end = self.ptr().offset(index as isize);
            ptr::write(end, value);
        }
    }
}

// ---------------------------------------------------------------------------------
//                                      Traits
// ---------------------------------------------------------------------------------

impl<T> Deref for Allocator<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        unsafe {
            let p = self.ptr();
            slice::from_raw_parts(p, self.cap)
        }
    }
}

impl<T, I> Index<I> for Allocator<T>
    where
        I: slice::SliceIndex<[T]>,
{
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        Index::index(&**self, index)
    }
}