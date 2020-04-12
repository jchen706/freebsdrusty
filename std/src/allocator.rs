
//#![allow(warnings, unused_variables, dead_code, improper_ctypes, non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::alloc::{GlobalAlloc, Layout};
use core::fmt;
use core::ptr;
//use crate::mutex::Mutex;
use ::os::kernel_malloc as kernel;
use ::os::align_file::MIN_ALIGN;

use ::os::raw;




pub struct FreebsdAllocator;

/// `LocalAlloc` is an analogous trait to the standard library's `GlobalAlloc`,
/// but it takes `&mut self` in `alloc()` and `dealloc()`.
pub trait LocalAlloc {
    unsafe fn alloc(&mut self, layout: Layout) -> *mut u8;
    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout);
}



unsafe impl GlobalAlloc for FreebsdAllocator {

    unsafe fn alloc(&self, _layout: Layout)-> *mut u8 {   
       //check alignment: 





       return  kernel::malloc( _layout.size() as raw::c_size_t, &mut kernel::M_DEVBUF[0], kernel::M_WAITOK as i32) as *mut u8 
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        kernel::free(ptr as *mut raw::c_void, &mut kernel::M_DEVBUF[0]);
    }




}

