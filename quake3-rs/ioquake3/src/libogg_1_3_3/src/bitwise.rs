use ::libc;

pub use crate::ogg_h::oggpack_buffer;

static mut mask: [usize; 33] = [
    0 as i32 as usize,
    0x1 as i32 as usize,
    0x3 as i32 as usize,
    0x7 as i32 as usize,
    0xf as i32 as usize,
    0x1f as i32 as usize,
    0x3f as i32 as usize,
    0x7f as i32 as usize,
    0xff as i32 as usize,
    0x1ff as i32 as usize,
    0x3ff as i32 as usize,
    0x7ff as i32 as usize,
    0xfff as i32 as usize,
    0x1fff as i32 as usize,
    0x3fff as i32 as usize,
    0x7fff as i32 as usize,
    0xffff as i32 as usize,
    0x1ffff as i32 as usize,
    0x3ffff as i32 as usize,
    0x7ffff as i32 as usize,
    0xfffff as i32 as usize,
    0x1fffff as i32 as usize,
    0x3fffff as i32 as usize,
    0x7fffff as i32 as usize,
    0xffffff as i32 as usize,
    0x1ffffff as i32 as usize,
    0x3ffffff as i32 as usize,
    0x7ffffff as i32 as usize,
    0xfffffff as i32 as usize,
    0x1fffffff as i32 as usize,
    0x3fffffff as i32 as usize,
    0x7fffffff as i32 as usize,
    0xffffffff as u32 as usize,
];

static mut mask8B: [u32; 9] = [
    0 as i32 as u32,
    0x80 as i32 as u32,
    0xc0 as i32 as u32,
    0xe0 as i32 as u32,
    0xf0 as i32 as u32,
    0xf8 as i32 as u32,
    0xfc as i32 as u32,
    0xfe as i32 as u32,
    0xff as i32 as u32,
];
/* Ogg BITSTREAM PRIMITIVES: bitstream ************************/
#[no_mangle]

pub unsafe extern "C" fn oggpack_writeinit(mut b: *mut oggpack_buffer) {
    crate::stdlib::memset(
        b as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<oggpack_buffer>() as usize,
    );
    (*b).buffer = crate::stdlib::malloc(256 as i32 as usize) as *mut u8;
    (*b).ptr = (*b).buffer;
    *(*b).buffer.offset(0 as i32 as isize) = '\u{0}' as i32 as u8;
    (*b).storage = 256 as i32 as isize;
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_writeinit(mut b: *mut oggpack_buffer) {
    oggpack_writeinit(b);
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_writecheck(mut b: *mut oggpack_buffer) -> i32 {
    if (*b).ptr.is_null() || (*b).storage == 0 {
        return -(1 as i32);
    }
    return 0 as i32;
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_writecheck(mut b: *mut oggpack_buffer) -> i32 {
    return oggpack_writecheck(b);
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_writetrunc(mut b: *mut oggpack_buffer, mut bits: isize) {
    let mut bytes: isize = bits >> 3 as i32;
    if !(*b).ptr.is_null() {
        bits -= bytes * 8 as i32 as isize;
        (*b).ptr = (*b).buffer.offset(bytes as isize);
        (*b).endbit = bits as i32;
        (*b).endbyte = bytes;
        *(*b).ptr = (*(*b).ptr as usize & mask[bits as usize]) as u8
    };
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_writetrunc(mut b: *mut oggpack_buffer, mut bits: isize) {
    let mut bytes: isize = bits >> 3 as i32;
    if !(*b).ptr.is_null() {
        bits -= bytes * 8 as i32 as isize;
        (*b).ptr = (*b).buffer.offset(bytes as isize);
        (*b).endbit = bits as i32;
        (*b).endbyte = bytes;
        *(*b).ptr = (*(*b).ptr as u32 & mask8B[bits as usize]) as u8
    };
}
/* Takes only up to 32 bits. */
#[no_mangle]

pub unsafe extern "C" fn oggpack_write(
    mut b: *mut oggpack_buffer,
    mut value: usize,
    mut bits: i32,
) {
    let mut current_block: u64;
    if !(bits < 0 as i32 || bits > 32 as i32) {
        if (*b).endbyte >= (*b).storage - 4 as i32 as isize {
            let mut ret: *mut libc::c_void = std::ptr::null_mut();
            if (*b).ptr.is_null() {
                return;
            }
            if (*b).storage > 9223372036854775807 as isize - 256 as i32 as isize {
                current_block = 11736846078876452800;
            } else {
                ret = crate::stdlib::realloc(
                    (*b).buffer as *mut libc::c_void,
                    ((*b).storage + 256 as i32 as isize) as usize,
                );
                if ret.is_null() {
                    current_block = 11736846078876452800;
                } else {
                    (*b).buffer = ret as *mut u8;
                    (*b).storage += 256 as i32 as isize;
                    (*b).ptr = (*b).buffer.offset((*b).endbyte as isize);
                    current_block = 13109137661213826276;
                }
            }
        } else {
            current_block = 13109137661213826276;
        }
        match current_block {
            11736846078876452800 => {}
            _ => {
                value &= mask[bits as usize];
                bits += (*b).endbit;
                let ref mut fresh0 = *(*b).ptr.offset(0 as i32 as isize);
                *fresh0 = (*fresh0 as usize | value << (*b).endbit) as u8;
                if bits >= 8 as i32 {
                    *(*b).ptr.offset(1 as i32 as isize) = (value >> 8 as i32 - (*b).endbit) as u8;
                    if bits >= 16 as i32 {
                        *(*b).ptr.offset(2 as i32 as isize) =
                            (value >> 16 as i32 - (*b).endbit) as u8;
                        if bits >= 24 as i32 {
                            *(*b).ptr.offset(3 as i32 as isize) =
                                (value >> 24 as i32 - (*b).endbit) as u8;
                            if bits >= 32 as i32 {
                                if (*b).endbit != 0 {
                                    *(*b).ptr.offset(4 as i32 as isize) =
                                        (value >> 32 as i32 - (*b).endbit) as u8
                                } else {
                                    *(*b).ptr.offset(4 as i32 as isize) = 0 as i32 as u8
                                }
                            }
                        }
                    }
                }
                (*b).endbyte += (bits / 8 as i32) as isize;
                (*b).ptr = (*b).ptr.offset((bits / 8 as i32) as isize);
                (*b).endbit = bits & 7 as i32;
                return;
            }
        }
    }
    oggpack_writeclear(b);
}
/* Takes only up to 32 bits. */
#[no_mangle]

pub unsafe extern "C" fn oggpackB_write(
    mut b: *mut oggpack_buffer,
    mut value: usize,
    mut bits: i32,
) {
    let mut current_block: u64;
    if !(bits < 0 as i32 || bits > 32 as i32) {
        if (*b).endbyte >= (*b).storage - 4 as i32 as isize {
            let mut ret: *mut libc::c_void = std::ptr::null_mut();
            if (*b).ptr.is_null() {
                return;
            }
            if (*b).storage > 9223372036854775807 as isize - 256 as i32 as isize {
                current_block = 2612814309992382393;
            } else {
                ret = crate::stdlib::realloc(
                    (*b).buffer as *mut libc::c_void,
                    ((*b).storage + 256 as i32 as isize) as usize,
                );
                if ret.is_null() {
                    current_block = 2612814309992382393;
                } else {
                    (*b).buffer = ret as *mut u8;
                    (*b).storage += 256 as i32 as isize;
                    (*b).ptr = (*b).buffer.offset((*b).endbyte as isize);
                    current_block = 13109137661213826276;
                }
            }
        } else {
            current_block = 13109137661213826276;
        }
        match current_block {
            2612814309992382393 => {}
            _ => {
                value = (value & mask[bits as usize]) << 32 as i32 - bits;
                bits += (*b).endbit;
                let ref mut fresh1 = *(*b).ptr.offset(0 as i32 as isize);
                *fresh1 = (*fresh1 as usize | value >> 24 as i32 + (*b).endbit) as u8;
                if bits >= 8 as i32 {
                    *(*b).ptr.offset(1 as i32 as isize) = (value >> 16 as i32 + (*b).endbit) as u8;
                    if bits >= 16 as i32 {
                        *(*b).ptr.offset(2 as i32 as isize) =
                            (value >> 8 as i32 + (*b).endbit) as u8;
                        if bits >= 24 as i32 {
                            *(*b).ptr.offset(3 as i32 as isize) = (value >> (*b).endbit) as u8;
                            if bits >= 32 as i32 {
                                if (*b).endbit != 0 {
                                    *(*b).ptr.offset(4 as i32 as isize) =
                                        (value << 8 as i32 - (*b).endbit) as u8
                                } else {
                                    *(*b).ptr.offset(4 as i32 as isize) = 0 as i32 as u8
                                }
                            }
                        }
                    }
                }
                (*b).endbyte += (bits / 8 as i32) as isize;
                (*b).ptr = (*b).ptr.offset((bits / 8 as i32) as isize);
                (*b).endbit = bits & 7 as i32;
                return;
            }
        }
    }
    oggpack_writeclear(b);
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_writealign(mut b: *mut oggpack_buffer) {
    let mut bits: i32 = 8 as i32 - (*b).endbit;
    if bits < 8 as i32 {
        oggpack_write(b, 0 as i32 as usize, bits);
    };
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_writealign(mut b: *mut oggpack_buffer) {
    let mut bits: i32 = 8 as i32 - (*b).endbit;
    if bits < 8 as i32 {
        oggpackB_write(b, 0 as i32 as usize, bits);
    };
}

unsafe extern "C" fn oggpack_writecopy_helper(
    mut b: *mut oggpack_buffer,
    mut source: *mut libc::c_void,
    mut bits: isize,
    mut w: Option<unsafe extern "C" fn(_: *mut oggpack_buffer, _: usize, _: i32) -> ()>,
    mut msb: i32,
) {
    let mut current_block: u64;
    let mut ptr: *mut u8 = source as *mut u8;
    let mut bytes: isize = bits / 8 as i32 as isize;
    let mut pbytes: isize = ((*b).endbit as isize + bits) / 8 as i32 as isize;
    bits -= bytes * 8 as i32 as isize;
    /* expand storage up-front */
    if (*b).endbyte + pbytes >= (*b).storage {
        let mut ret: *mut libc::c_void = std::ptr::null_mut();
        if (*b).ptr.is_null() {
            current_block = 1692384543052803397;
        } else if (*b).storage > (*b).endbyte + pbytes + 256 as i32 as isize {
            current_block = 1692384543052803397;
        } else {
            (*b).storage = (*b).endbyte + pbytes + 256 as i32 as isize;
            ret = crate::stdlib::realloc((*b).buffer as *mut libc::c_void, (*b).storage as usize);
            if ret.is_null() {
                current_block = 1692384543052803397;
            } else {
                (*b).buffer = ret as *mut u8;
                (*b).ptr = (*b).buffer.offset((*b).endbyte as isize);
                current_block = 7746791466490516765;
            }
        }
        match current_block {
            7746791466490516765 => {}
            _ => {
                oggpack_writeclear(b);
                return;
            }
        }
    }
    /* copy whole octets */
    if (*b).endbit != 0 {
        let mut i: i32 = 0;
        /* unaligned copy.  Do it the hard way. */
        i = 0 as i32;
        while (i as isize) < bytes {
            w.expect("non-null function pointer")(b, *ptr.offset(i as isize) as usize, 8 as i32);
            i += 1
        }
    } else {
        /* aligned block copy */
        crate::stdlib::memmove((*b).ptr as *mut libc::c_void, source, bytes as usize);
        (*b).ptr = (*b).ptr.offset(bytes as isize);
        (*b).endbyte += bytes;
        *(*b).ptr = 0 as i32 as u8
    }
    /* copy trailing bits */
    if bits != 0 {
        if msb != 0 {
            w.expect("non-null function pointer")(
                b,
                (*ptr.offset(bytes as isize) as i32 >> 8 as i32 as isize - bits) as usize,
                bits as i32,
            );
        } else {
            w.expect("non-null function pointer")(
                b,
                *ptr.offset(bytes as isize) as usize,
                bits as i32,
            );
        }
    };
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_writecopy(
    mut b: *mut oggpack_buffer,
    mut source: *mut libc::c_void,
    mut bits: isize,
) {
    oggpack_writecopy_helper(
        b,
        source,
        bits,
        Some(oggpack_write as unsafe extern "C" fn(_: *mut oggpack_buffer, _: usize, _: i32) -> ()),
        0 as i32,
    );
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_writecopy(
    mut b: *mut oggpack_buffer,
    mut source: *mut libc::c_void,
    mut bits: isize,
) {
    oggpack_writecopy_helper(
        b,
        source,
        bits,
        Some(
            oggpackB_write as unsafe extern "C" fn(_: *mut oggpack_buffer, _: usize, _: i32) -> (),
        ),
        1 as i32,
    );
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_reset(mut b: *mut oggpack_buffer) {
    if (*b).ptr.is_null() {
        return;
    }
    (*b).ptr = (*b).buffer;
    *(*b).buffer.offset(0 as i32 as isize) = 0 as i32 as u8;
    (*b).endbyte = 0 as i32 as isize;
    (*b).endbit = (*b).endbyte as i32;
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_reset(mut b: *mut oggpack_buffer) {
    oggpack_reset(b);
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_writeclear(mut b: *mut oggpack_buffer) {
    if !(*b).buffer.is_null() {
        libc::free((*b).buffer as *mut libc::c_void);
    }
    crate::stdlib::memset(
        b as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<oggpack_buffer>() as usize,
    );
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_writeclear(mut b: *mut oggpack_buffer) {
    oggpack_writeclear(b);
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_readinit(
    mut b: *mut oggpack_buffer,
    mut buf: *mut u8,
    mut bytes: i32,
) {
    crate::stdlib::memset(
        b as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<oggpack_buffer>() as usize,
    );
    (*b).ptr = buf;
    (*b).buffer = (*b).ptr;
    (*b).storage = bytes as isize;
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_readinit(
    mut b: *mut oggpack_buffer,
    mut buf: *mut u8,
    mut bytes: i32,
) {
    oggpack_readinit(b, buf, bytes);
}
/* Read in bits without advancing the bitptr; bits <= 32 */
#[no_mangle]

pub unsafe extern "C" fn oggpack_look(mut b: *mut oggpack_buffer, mut bits: i32) -> isize {
    let mut ret: usize = 0;
    let mut m: usize = 0;
    if bits < 0 as i32 || bits > 32 as i32 {
        return -(1 as i32) as isize;
    }
    m = mask[bits as usize];
    bits += (*b).endbit;
    if (*b).endbyte >= (*b).storage - 4 as i32 as isize {
        /* not the main path */
        if (*b).endbyte > (*b).storage - (bits + 7 as i32 >> 3 as i32) as isize {
            return -(1 as i32) as isize;
        } else {
            /* special case to avoid reading b->ptr[0], which might be past the end of
            the buffer; also skips some useless accounting */
            if bits == 0 {
                return 0 as isize;
            }
        }
    }
    ret = (*(*b).ptr.offset(0 as i32 as isize) as i32 >> (*b).endbit) as usize;
    if bits > 8 as i32 {
        ret |= ((*(*b).ptr.offset(1 as i32 as isize) as i32) << 8 as i32 - (*b).endbit) as usize;
        if bits > 16 as i32 {
            ret |=
                ((*(*b).ptr.offset(2 as i32 as isize) as i32) << 16 as i32 - (*b).endbit) as usize;
            if bits > 24 as i32 {
                ret |= ((*(*b).ptr.offset(3 as i32 as isize) as i32) << 24 as i32 - (*b).endbit)
                    as usize;
                if bits > 32 as i32 && (*b).endbit != 0 {
                    ret |= ((*(*b).ptr.offset(4 as i32 as isize) as i32) << 32 as i32 - (*b).endbit)
                        as usize
                }
            }
        }
    }
    return (m & ret) as isize;
}
/* Read in bits without advancing the bitptr; bits <= 32 */
#[no_mangle]

pub unsafe extern "C" fn oggpackB_look(mut b: *mut oggpack_buffer, mut bits: i32) -> isize {
    let mut ret: usize = 0;
    let mut m: i32 = 32 as i32 - bits;
    if m < 0 as i32 || m > 32 as i32 {
        return -(1 as i32) as isize;
    }
    bits += (*b).endbit;
    if (*b).endbyte >= (*b).storage - 4 as i32 as isize {
        /* not the main path */
        if (*b).endbyte > (*b).storage - (bits + 7 as i32 >> 3 as i32) as isize {
            return -(1 as i32) as isize;
        } else {
            /* special case to avoid reading b->ptr[0], which might be past the end of
            the buffer; also skips some useless accounting */
            if bits == 0 {
                return 0 as isize;
            }
        }
    }
    ret = ((*(*b).ptr.offset(0 as i32 as isize) as i32) << 24 as i32 + (*b).endbit) as usize;
    if bits > 8 as i32 {
        ret |= ((*(*b).ptr.offset(1 as i32 as isize) as i32) << 16 as i32 + (*b).endbit) as usize;
        if bits > 16 as i32 {
            ret |=
                ((*(*b).ptr.offset(2 as i32 as isize) as i32) << 8 as i32 + (*b).endbit) as usize;
            if bits > 24 as i32 {
                ret |= ((*(*b).ptr.offset(3 as i32 as isize) as i32) << (*b).endbit) as usize;
                if bits > 32 as i32 && (*b).endbit != 0 {
                    ret |= (*(*b).ptr.offset(4 as i32 as isize) as i32 >> 8 as i32 - (*b).endbit)
                        as usize
                }
            }
        }
    }
    return ((ret & 0xffffffff as u32 as usize) >> (m >> 1 as i32) >> (m + 1 as i32 >> 1 as i32))
        as isize;
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_look1(mut b: *mut oggpack_buffer) -> isize {
    if (*b).endbyte >= (*b).storage {
        return -(1 as i32) as isize;
    }
    return (*(*b).ptr.offset(0 as i32 as isize) as i32 >> (*b).endbit & 1 as i32) as isize;
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_look1(mut b: *mut oggpack_buffer) -> isize {
    if (*b).endbyte >= (*b).storage {
        return -(1 as i32) as isize;
    }
    return (*(*b).ptr.offset(0 as i32 as isize) as i32 >> 7 as i32 - (*b).endbit & 1 as i32)
        as isize;
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_adv(mut b: *mut oggpack_buffer, mut bits: i32) {
    bits += (*b).endbit;
    if (*b).endbyte > (*b).storage - (bits + 7 as i32 >> 3 as i32) as isize {
        (*b).ptr = std::ptr::null_mut();
        (*b).endbyte = (*b).storage;
        (*b).endbit = 1 as i32;
        return;
    } else {
        (*b).ptr = (*b).ptr.offset((bits / 8 as i32) as isize);
        (*b).endbyte += (bits / 8 as i32) as isize;
        (*b).endbit = bits & 7 as i32;
        return;
    };
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_adv(mut b: *mut oggpack_buffer, mut bits: i32) {
    oggpack_adv(b, bits);
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_adv1(mut b: *mut oggpack_buffer) {
    (*b).endbit += 1;
    if (*b).endbit > 7 as i32 {
        (*b).endbit = 0 as i32;
        (*b).ptr = (*b).ptr.offset(1);
        (*b).endbyte += 1
    };
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_adv1(mut b: *mut oggpack_buffer) {
    oggpack_adv1(b);
}
/* bits <= 32 */
#[no_mangle]

pub unsafe extern "C" fn oggpack_read(mut b: *mut oggpack_buffer, mut bits: i32) -> isize {
    let mut current_block: u64;
    let mut ret: isize = 0;
    let mut m: usize = 0;
    if !(bits < 0 as i32 || bits > 32 as i32) {
        m = mask[bits as usize];
        bits += (*b).endbit;
        if (*b).endbyte >= (*b).storage - 4 as i32 as isize {
            /* not the main path */
            if (*b).endbyte > (*b).storage - (bits + 7 as i32 >> 3 as i32) as isize {
                current_block = 7073085723881536557;
            } else {
                /* special case to avoid reading b->ptr[0], which might be past the end of
                the buffer; also skips some useless accounting */
                if bits == 0 {
                    return 0 as isize;
                }
                current_block = 14523784380283086299;
            }
        } else {
            current_block = 14523784380283086299;
        }
        match current_block {
            7073085723881536557 => {}
            _ => {
                ret = (*(*b).ptr.offset(0 as i32 as isize) as i32 >> (*b).endbit) as isize;
                if bits > 8 as i32 {
                    ret |= ((*(*b).ptr.offset(1 as i32 as isize) as i32) << 8 as i32 - (*b).endbit)
                        as isize;
                    if bits > 16 as i32 {
                        ret |= ((*(*b).ptr.offset(2 as i32 as isize) as i32)
                            << 16 as i32 - (*b).endbit) as isize;
                        if bits > 24 as i32 {
                            ret |= ((*(*b).ptr.offset(3 as i32 as isize) as i32)
                                << 24 as i32 - (*b).endbit)
                                as isize;
                            if bits > 32 as i32 && (*b).endbit != 0 {
                                ret |= ((*(*b).ptr.offset(4 as i32 as isize) as i32)
                                    << 32 as i32 - (*b).endbit)
                                    as isize
                            }
                        }
                    }
                }
                ret = (ret as usize & m) as isize;
                (*b).ptr = (*b).ptr.offset((bits / 8 as i32) as isize);
                (*b).endbyte += (bits / 8 as i32) as isize;
                (*b).endbit = bits & 7 as i32;
                return ret;
            }
        }
    }
    (*b).ptr = std::ptr::null_mut();
    (*b).endbyte = (*b).storage;
    (*b).endbit = 1 as i32;
    return -(1 as isize);
}
/* bits <= 32 */
#[no_mangle]

pub unsafe extern "C" fn oggpackB_read(mut b: *mut oggpack_buffer, mut bits: i32) -> isize {
    let mut current_block: u64;
    let mut ret: isize = 0;
    let mut m: isize = (32 as i32 - bits) as isize;
    if !(m < 0 as i32 as isize || m > 32 as i32 as isize) {
        bits += (*b).endbit;
        if (*b).endbyte + 4 as i32 as isize >= (*b).storage {
            /* not the main path */
            if (*b).endbyte > (*b).storage - (bits + 7 as i32 >> 3 as i32) as isize {
                current_block = 11946596175837608108;
            } else {
                /* special case to avoid reading b->ptr[0], which might be past the end of
                the buffer; also skips some useless accounting */
                if bits == 0 {
                    return 0 as isize;
                }
                current_block = 7351195479953500246;
            }
        } else {
            current_block = 7351195479953500246;
        }
        match current_block {
            11946596175837608108 => {}
            _ => {
                ret = ((*(*b).ptr.offset(0 as i32 as isize) as i32) << 24 as i32 + (*b).endbit)
                    as isize;
                if bits > 8 as i32 {
                    ret |= ((*(*b).ptr.offset(1 as i32 as isize) as i32) << 16 as i32 + (*b).endbit)
                        as isize;
                    if bits > 16 as i32 {
                        ret |= ((*(*b).ptr.offset(2 as i32 as isize) as i32)
                            << 8 as i32 + (*b).endbit) as isize;
                        if bits > 24 as i32 {
                            ret |= ((*(*b).ptr.offset(3 as i32 as isize) as i32) << (*b).endbit)
                                as isize;
                            if bits > 32 as i32 && (*b).endbit != 0 {
                                ret |= (*(*b).ptr.offset(4 as i32 as isize) as i32
                                    >> 8 as i32 - (*b).endbit)
                                    as isize
                            }
                        }
                    }
                }
                ret = ((ret as usize & 0xffffffff as usize)
                    >> (m >> 1 as i32)
                    >> (m + 1 as i32 as isize >> 1 as i32)) as isize;
                (*b).ptr = (*b).ptr.offset((bits / 8 as i32) as isize);
                (*b).endbyte += (bits / 8 as i32) as isize;
                (*b).endbit = bits & 7 as i32;
                return ret;
            }
        }
    }
    (*b).ptr = std::ptr::null_mut();
    (*b).endbyte = (*b).storage;
    (*b).endbit = 1 as i32;
    return -(1 as isize);
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_read1(mut b: *mut oggpack_buffer) -> isize {
    let mut ret: isize = 0;
    if (*b).endbyte >= (*b).storage {
        (*b).ptr = std::ptr::null_mut();
        (*b).endbyte = (*b).storage;
        (*b).endbit = 1 as i32;
        return -(1 as isize);
    } else {
        ret = (*(*b).ptr.offset(0 as i32 as isize) as i32 >> (*b).endbit & 1 as i32) as isize;
        (*b).endbit += 1;
        if (*b).endbit > 7 as i32 {
            (*b).endbit = 0 as i32;
            (*b).ptr = (*b).ptr.offset(1);
            (*b).endbyte += 1
        }
        return ret;
    };
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_read1(mut b: *mut oggpack_buffer) -> isize {
    let mut ret: isize = 0;
    if (*b).endbyte >= (*b).storage {
        (*b).ptr = std::ptr::null_mut();
        (*b).endbyte = (*b).storage;
        (*b).endbit = 1 as i32;
        return -(1 as isize);
    } else {
        ret = (*(*b).ptr.offset(0 as i32 as isize) as i32 >> 7 as i32 - (*b).endbit & 1 as i32)
            as isize;
        (*b).endbit += 1;
        if (*b).endbit > 7 as i32 {
            (*b).endbit = 0 as i32;
            (*b).ptr = (*b).ptr.offset(1);
            (*b).endbyte += 1
        }
        return ret;
    };
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_bytes(mut b: *mut oggpack_buffer) -> isize {
    return (*b).endbyte + (((*b).endbit + 7 as i32) / 8 as i32) as isize;
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_bits(mut b: *mut oggpack_buffer) -> isize {
    return (*b).endbyte * 8 as i32 as isize + (*b).endbit as isize;
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_bytes(mut b: *mut oggpack_buffer) -> isize {
    return oggpack_bytes(b);
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_bits(mut b: *mut oggpack_buffer) -> isize {
    return oggpack_bits(b);
}
#[no_mangle]

pub unsafe extern "C" fn oggpack_get_buffer(mut b: *mut oggpack_buffer) -> *mut u8 {
    return (*b).buffer;
}
#[no_mangle]

pub unsafe extern "C" fn oggpackB_get_buffer(mut b: *mut oggpack_buffer) -> *mut u8 {
    return oggpack_get_buffer(b);
}
/* _V_SELFTEST */
/* Self test of the bitwise routines; everything else is based on
them, so they damned well better be solid. */
