use crate::ALLOCATOR;
use core::{
    alloc::{GlobalAlloc, Layout},
    sync::atomic::{AtomicUsize, Ordering}, ptr::null_mut,
};

use super::types::*;

extern "C" {
    fn mfree(ptr:*mut c_void)->*mut c_void;
    fn malloc(size:usize)->*mut c_void;
}

pub struct Allocator {
    mstate: AtomicUsize,
}

pub const NEWALLOCATOR: Allocator = Allocator {
    mstate: AtomicUsize::new(0),
};

impl Allocator {
    pub fn set_book_keeper(&self, mstate: usize) {
        dbg!("set book keeper,{}");
        dbg!(mstate);
        self.mstate.store(mstate, Ordering::Relaxed);
        dbg!("have set book keeper");//,mstate);
    }
    pub fn get_book_keeper(&self) -> usize {
        self.mstate.load(Ordering::Relaxed)
    }
}

unsafe impl<'a> GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        malloc(layout.size()) as *mut u8
        //alloc_align(layout.size(), layout.align()) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        mfree(ptr as *mut c_void);
    }
}

pub unsafe fn alloc(size: usize) -> *mut c_void {
    malloc(size)
    //mspace_malloc(ALLOCATOR.get_book_keeper(), size)
}

pub unsafe fn alloc_align(size: usize, alignment: usize) -> *mut c_void {
    malloc(size)
    //mspace_memalign(ALLOCATOR.get_book_keeper(), alignment, size)
}

pub unsafe fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void {
    todo!()
    //null_mut()
    //mspace_realloc(ALLOCATOR.get_book_keeper(), ptr, size)
}

pub unsafe fn free(ptr: *mut c_void) {
    mfree(ptr);
    //mspace_free(ALLOCATOR.get_book_keeper(), ptr)
}

#[cfg(not(target_os = "dragonos"))]
pub fn new_mspace() -> usize {
    unsafe { create_mspace(0, 0) };
}

#[cfg(target_os = "dragonos")]
pub fn new_mspace() -> usize {
    // use core::sync::atomic::AtomicU8;

    // static mut space: [[u8; 128 * 16]; 2] = [[0; 128 * 16]; 2];
    // static cnt: AtomicU8 = AtomicU8::new(0);
    // let x = cnt.fetch_add(1, Ordering::Relaxed);
    // if x > 2 {
    //     panic!("new_mspace: too many mspace");
    // }
    // println!("I am here");
    // println!("{:#?}",unsafe{space[x as usize].as_mut_ptr()});
    // let r=unsafe{malloc(128 * 16)} as usize;
    // //let r = unsafe { create_mspace_with_base(space[x as usize].as_mut_ptr() as *mut c_void, 128 * 16, 0) };
    // println!("new_mspace: {:#018x}", r);
    // dbg!("new mspace");
    // let rsize=core::ptr::addr_of!(r);
    // return unsafe { *rsize };
    1
}