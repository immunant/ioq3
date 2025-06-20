@@
type T;
@@

- 0 as *const T as *mut T
+ std::ptr::null_mut()
