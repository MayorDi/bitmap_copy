use libc::{malloc, memset};

#[inline]
pub unsafe fn malloc_array<'a, T: Copy+Sized>(count: usize) -> &'a [T] {
    let size = std::mem::size_of::<T>() * count;
    let ptr = malloc(size);
    memset(ptr, 0, size);

    *(ptr as *const &'a [T])
}
