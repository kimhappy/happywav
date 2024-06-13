use core::{ mem::size_of, slice::from_raw_parts_mut };
use super::Pod;

pub trait AsMutU8Slice {
    fn as_u8_slice(what: &mut Self) -> &mut [u8];
}

impl< T: Pod > AsMutU8Slice for T {
    fn as_u8_slice(what: &mut Self) -> &mut [u8] {
        let ptr = what as *const T as *mut u8;
        let len = size_of::< T >();
        unsafe { from_raw_parts_mut(ptr, len) }
    }
}

impl< T: Pod > AsMutU8Slice for [T] {
    fn as_u8_slice(what: &mut Self) -> &mut [u8] {
        let ptr = what.as_ptr() as *mut u8;
        let len = size_of::< T >() * what.len();
        unsafe { from_raw_parts_mut(ptr, len) }
    }
}
