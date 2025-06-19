use ::libc;

pub use crate::stddef_h::size_t;

pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;

pub use crate::jmemsys_h::backing_store_info;
pub use crate::jmemsys_h::backing_store_ptr;
pub use crate::jmemsys_h::backing_store_struct;
pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jvirt_barray_ptr;
pub use crate::jpeglib_h::jvirt_sarray_ptr;
pub use crate::jpeglib_h::C2RustUnnamed_0;
pub use crate::jpeglib_h::JBLOCK;
pub use crate::jpeglib_h::JBLOCKARRAY;
pub use crate::jpeglib_h::JBLOCKROW;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::src::jpeg_8c::jerror::C2RustUnnamed_1;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_ALIGN_TYPE;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_ALLOC_CHUNK;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_BUFFER_MODE;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_COMPONENT_ID;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_CROP_SPEC;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_DCTSIZE;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_DCT_COEF;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_DROP_SAMPLING;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_HUFF_TABLE;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_IN_COLORSPACE;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_J_COLORSPACE;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_LENGTH;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_LIB_VERSION;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_MCU_SIZE;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_POOL_ID;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_PRECISION;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_PROGRESSION;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_PROG_SCRIPT;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_SAMPLING;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_SCAN_SCRIPT;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_STATE;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_STRUCT_SIZE;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_VIRTUAL_ACCESS;
pub use crate::src::jpeg_8c::jerror::JERR_BUFFER_SIZE;
pub use crate::src::jpeg_8c::jerror::JERR_CANT_SUSPEND;
pub use crate::src::jpeg_8c::jerror::JERR_CCIR601_NOTIMPL;
pub use crate::src::jpeg_8c::jerror::JERR_COMPONENT_COUNT;
pub use crate::src::jpeg_8c::jerror::JERR_CONVERSION_NOTIMPL;
pub use crate::src::jpeg_8c::jerror::JERR_DAC_INDEX;
pub use crate::src::jpeg_8c::jerror::JERR_DAC_VALUE;
pub use crate::src::jpeg_8c::jerror::JERR_DHT_INDEX;
pub use crate::src::jpeg_8c::jerror::JERR_DQT_INDEX;
pub use crate::src::jpeg_8c::jerror::JERR_EMPTY_IMAGE;
pub use crate::src::jpeg_8c::jerror::JERR_EMS_READ;
pub use crate::src::jpeg_8c::jerror::JERR_EMS_WRITE;
pub use crate::src::jpeg_8c::jerror::JERR_EOI_EXPECTED;
pub use crate::src::jpeg_8c::jerror::JERR_FILE_READ;
pub use crate::src::jpeg_8c::jerror::JERR_FILE_WRITE;
pub use crate::src::jpeg_8c::jerror::JERR_FRACT_SAMPLE_NOTIMPL;
pub use crate::src::jpeg_8c::jerror::JERR_HUFF_CLEN_OVERFLOW;
pub use crate::src::jpeg_8c::jerror::JERR_HUFF_MISSING_CODE;
pub use crate::src::jpeg_8c::jerror::JERR_IMAGE_TOO_BIG;
pub use crate::src::jpeg_8c::jerror::JERR_INPUT_EMPTY;
pub use crate::src::jpeg_8c::jerror::JERR_INPUT_EOF;
pub use crate::src::jpeg_8c::jerror::JERR_MISMATCHED_QUANT_TABLE;
pub use crate::src::jpeg_8c::jerror::JERR_MISSING_DATA;
pub use crate::src::jpeg_8c::jerror::JERR_MODE_CHANGE;
pub use crate::src::jpeg_8c::jerror::JERR_NOTIMPL;
pub use crate::src::jpeg_8c::jerror::JERR_NOT_COMPILED;
pub use crate::src::jpeg_8c::jerror::JERR_NO_ARITH_TABLE;
pub use crate::src::jpeg_8c::jerror::JERR_NO_BACKING_STORE;
pub use crate::src::jpeg_8c::jerror::JERR_NO_HUFF_TABLE;
pub use crate::src::jpeg_8c::jerror::JERR_NO_IMAGE;
pub use crate::src::jpeg_8c::jerror::JERR_NO_QUANT_TABLE;
pub use crate::src::jpeg_8c::jerror::JERR_NO_SOI;
pub use crate::src::jpeg_8c::jerror::JERR_OUT_OF_MEMORY;
pub use crate::src::jpeg_8c::jerror::JERR_QUANT_COMPONENTS;
pub use crate::src::jpeg_8c::jerror::JERR_QUANT_FEW_COLORS;
pub use crate::src::jpeg_8c::jerror::JERR_QUANT_MANY_COLORS;
pub use crate::src::jpeg_8c::jerror::JERR_SOF_DUPLICATE;
pub use crate::src::jpeg_8c::jerror::JERR_SOF_NO_SOS;
pub use crate::src::jpeg_8c::jerror::JERR_SOF_UNSUPPORTED;
pub use crate::src::jpeg_8c::jerror::JERR_SOI_DUPLICATE;
pub use crate::src::jpeg_8c::jerror::JERR_SOS_NO_SOF;
pub use crate::src::jpeg_8c::jerror::JERR_TFILE_CREATE;
pub use crate::src::jpeg_8c::jerror::JERR_TFILE_READ;
pub use crate::src::jpeg_8c::jerror::JERR_TFILE_SEEK;
pub use crate::src::jpeg_8c::jerror::JERR_TFILE_WRITE;
pub use crate::src::jpeg_8c::jerror::JERR_TOO_LITTLE_DATA;
pub use crate::src::jpeg_8c::jerror::JERR_UNKNOWN_MARKER;
pub use crate::src::jpeg_8c::jerror::JERR_VIRTUAL_BUG;
pub use crate::src::jpeg_8c::jerror::JERR_WIDTH_OVERFLOW;
pub use crate::src::jpeg_8c::jerror::JERR_XMS_READ;
pub use crate::src::jpeg_8c::jerror::JERR_XMS_WRITE;
pub use crate::src::jpeg_8c::jerror::JMSG_COPYRIGHT;
pub use crate::src::jpeg_8c::jerror::JMSG_LASTMSGCODE;
pub use crate::src::jpeg_8c::jerror::JMSG_NOMESSAGE;
pub use crate::src::jpeg_8c::jerror::JMSG_VERSION;
pub use crate::src::jpeg_8c::jerror::JTRC_16BIT_TABLES;
pub use crate::src::jpeg_8c::jerror::JTRC_ADOBE;
pub use crate::src::jpeg_8c::jerror::JTRC_APP0;
pub use crate::src::jpeg_8c::jerror::JTRC_APP14;
pub use crate::src::jpeg_8c::jerror::JTRC_DAC;
pub use crate::src::jpeg_8c::jerror::JTRC_DHT;
pub use crate::src::jpeg_8c::jerror::JTRC_DQT;
pub use crate::src::jpeg_8c::jerror::JTRC_DRI;
pub use crate::src::jpeg_8c::jerror::JTRC_EMS_CLOSE;
pub use crate::src::jpeg_8c::jerror::JTRC_EMS_OPEN;
pub use crate::src::jpeg_8c::jerror::JTRC_EOI;
pub use crate::src::jpeg_8c::jerror::JTRC_HUFFBITS;
pub use crate::src::jpeg_8c::jerror::JTRC_JFIF;
pub use crate::src::jpeg_8c::jerror::JTRC_JFIF_BADTHUMBNAILSIZE;
pub use crate::src::jpeg_8c::jerror::JTRC_JFIF_EXTENSION;
pub use crate::src::jpeg_8c::jerror::JTRC_JFIF_THUMBNAIL;
pub use crate::src::jpeg_8c::jerror::JTRC_MISC_MARKER;
pub use crate::src::jpeg_8c::jerror::JTRC_PARMLESS_MARKER;
pub use crate::src::jpeg_8c::jerror::JTRC_QUANTVALS;
pub use crate::src::jpeg_8c::jerror::JTRC_QUANT_3_NCOLORS;
pub use crate::src::jpeg_8c::jerror::JTRC_QUANT_NCOLORS;
pub use crate::src::jpeg_8c::jerror::JTRC_QUANT_SELECTED;
pub use crate::src::jpeg_8c::jerror::JTRC_RECOVERY_ACTION;
pub use crate::src::jpeg_8c::jerror::JTRC_RST;
pub use crate::src::jpeg_8c::jerror::JTRC_SMOOTH_NOTIMPL;
pub use crate::src::jpeg_8c::jerror::JTRC_SOF;
pub use crate::src::jpeg_8c::jerror::JTRC_SOF_COMPONENT;
pub use crate::src::jpeg_8c::jerror::JTRC_SOI;
pub use crate::src::jpeg_8c::jerror::JTRC_SOS;
pub use crate::src::jpeg_8c::jerror::JTRC_SOS_COMPONENT;
pub use crate::src::jpeg_8c::jerror::JTRC_SOS_PARAMS;
pub use crate::src::jpeg_8c::jerror::JTRC_TFILE_CLOSE;
pub use crate::src::jpeg_8c::jerror::JTRC_TFILE_OPEN;
pub use crate::src::jpeg_8c::jerror::JTRC_THUMB_JPEG;
pub use crate::src::jpeg_8c::jerror::JTRC_THUMB_PALETTE;
pub use crate::src::jpeg_8c::jerror::JTRC_THUMB_RGB;
pub use crate::src::jpeg_8c::jerror::JTRC_UNKNOWN_IDS;
pub use crate::src::jpeg_8c::jerror::JTRC_XMS_CLOSE;
pub use crate::src::jpeg_8c::jerror::JTRC_XMS_OPEN;
pub use crate::src::jpeg_8c::jerror::JWRN_ADOBE_XFORM;
pub use crate::src::jpeg_8c::jerror::JWRN_ARITH_BAD_CODE;
pub use crate::src::jpeg_8c::jerror::JWRN_BOGUS_PROGRESSION;
pub use crate::src::jpeg_8c::jerror::JWRN_EXTRANEOUS_DATA;
pub use crate::src::jpeg_8c::jerror::JWRN_HIT_MARKER;
pub use crate::src::jpeg_8c::jerror::JWRN_HUFF_BAD_CODE;
pub use crate::src::jpeg_8c::jerror::JWRN_JFIF_MAJOR;
pub use crate::src::jpeg_8c::jerror::JWRN_JPEG_EOF;
pub use crate::src::jpeg_8c::jerror::JWRN_MUST_RESYNC;
pub use crate::src::jpeg_8c::jerror::JWRN_NOT_SEQUENTIAL;
pub use crate::src::jpeg_8c::jerror::JWRN_TOO_MUCH_DATA;
pub use crate::src::jpeg_8c::jmemnobs::jpeg_free_large;
pub use crate::src::jpeg_8c::jmemnobs::jpeg_free_small;
pub use crate::src::jpeg_8c::jmemnobs::jpeg_get_large;
pub use crate::src::jpeg_8c::jmemnobs::jpeg_get_small;
pub use crate::src::jpeg_8c::jmemnobs::jpeg_mem_available;
pub use crate::src::jpeg_8c::jmemnobs::jpeg_mem_init;
pub use crate::src::jpeg_8c::jmemnobs::jpeg_mem_term;
pub use crate::src::jpeg_8c::jmemnobs::jpeg_open_backing_store;

/* System-dependent control info */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jvirt_barray_control {
    pub mem_buffer: JBLOCKARRAY,
    pub rows_in_array: JDIMENSION,
    pub blocksperrow: JDIMENSION,
    pub maxaccess: JDIMENSION,
    pub rows_in_mem: JDIMENSION,
    pub rowsperchunk: JDIMENSION,
    pub cur_start_row: JDIMENSION,
    pub first_undef_row: JDIMENSION,
    pub pre_zero: boolean,
    pub dirty: boolean,
    pub b_s_open: boolean,
    pub next: jvirt_barray_ptr,
    pub b_s_info: backing_store_info,
}
/*
 * The control blocks for virtual arrays.
 * Note that these blocks are allocated in the "small" pool area.
 * System-dependent info for the associated backing store (if any) is hidden
 * inside the backing_store_info struct.
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jvirt_sarray_control {
    pub mem_buffer: JSAMPARRAY,
    pub rows_in_array: JDIMENSION,
    pub samplesperrow: JDIMENSION,
    pub maxaccess: JDIMENSION,
    pub rows_in_mem: JDIMENSION,
    pub rowsperchunk: JDIMENSION,
    pub cur_start_row: JDIMENSION,
    pub first_undef_row: JDIMENSION,
    pub pre_zero: boolean,
    pub dirty: boolean,
    pub b_s_open: boolean,
    pub next: jvirt_sarray_ptr,
    pub b_s_info: backing_store_info,
}

pub type my_mem_ptr = *mut my_memory_mgr;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_memory_mgr {
    pub pub_0: jpeg_memory_mgr,
    pub small_list: [small_pool_ptr; 2],
    pub large_list: [large_pool_ptr; 2],
    pub virt_sarray_list: jvirt_sarray_ptr,
    pub virt_barray_list: jvirt_barray_ptr,
    pub total_space_allocated: isize,
    pub last_rowsperchunk: JDIMENSION,
}

pub type large_pool_ptr = *mut large_pool_struct;

#[repr(C)]
#[derive(Copy, Clone)]
pub union large_pool_struct {
    pub hdr: C2RustUnnamed_70,
    pub dummy: f64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_70 {
    pub next: large_pool_ptr,
    pub bytes_used: size_t,
    pub bytes_left: size_t,
}
/*
 * jmemmgr.c
 *
 * Copyright (C) 1991-1997, Thomas G. Lane.
 * This file is part of the Independent JPEG Group's software.
 * For conditions of distribution and use, see the accompanying README file.
 *
 * This file contains the JPEG system-independent memory management
 * routines.  This code is usable across a wide variety of machines; most
 * of the system dependencies have been isolated in a separate file.
 * The major functions provided here are:
 *   * pool-based allocation and freeing of memory;
 *   * policy decisions about how to divide available memory among the
 *     virtual arrays;
 *   * control logic for swapping virtual arrays between main memory and
 *     backing storage.
 * The separate system-dependent file provides the actual backing-storage
 * access code, and it contains the policy decision about how much total
 * main memory to use.
 * This file is system-dependent in the sense that some of its functions
 * are unnecessary in some systems.  For example, if there is enough virtual
 * memory so that backing storage will never be used, much of the virtual
 * array control logic could be removed.  (Of course, if you have that much
 * memory then you shouldn't care about a little bit of unused code...)
 */
/* we define jvirt_Xarray_control structs */
/* <stdlib.h> should declare getenv() */
/*
 * Some important notes:
 *   The allocation routines provided here must never return NULL.
 *   They should exit to error_exit if unsuccessful.
 *
 *   It's not a good idea to try to merge the sarray and barray routines,
 *   even though they are textually almost the same, because samples are
 *   usually stored as bytes while coefficients are shorts or ints.  Thus,
 *   in machines where byte pointers have a different representation from
 *   word pointers, the resulting machine code could not be the same.
 */
/*
 * Many machines require storage alignment: longs must start on 4-byte
 * boundaries, doubles on 8-byte boundaries, etc.  On such machines, malloc()
 * always returns pointers that are multiples of the worst-case alignment
 * requirement, and we had better do so too.
 * There isn't any really portable way to determine the worst-case alignment
 * requirement.  This module assumes that the alignment requirement is
 * multiples of sizeof(ALIGN_TYPE).
 * By default, we define ALIGN_TYPE as double.  This is necessary on some
 * workstations (where doubles really do need 8-byte alignment) and will work
 * fine on nearly everything.  If your machine has lesser alignment needs,
 * you can save a few bytes by making ALIGN_TYPE smaller.
 * The only place I know of where this will NOT work is certain Macintosh
 * 680x0 compilers that define double as a 10-byte IEEE extended float.
 * Doing 10-byte alignment is counterproductive because longwords won't be
 * aligned well.  Put "#define ALIGN_TYPE long" in jconfig.h if you have
 * such a compiler.
 */
/* so can override from jconfig.h */
/*
 * We allocate objects from "pools", where each pool is gotten with a single
 * request to jpeg_get_small() or jpeg_get_large().  There is no per-object
 * overhead within a pool, except for alignment padding.  Each pool has a
 * header with a link to the next pool of the same class.
 * Small and large pool headers are identical except that the latter's
 * link pointer must be FAR on 80x86 machines.
 * Notice that the "real" header fields are union'ed with a dummy ALIGN_TYPE
 * field.  This forces the compiler to make SIZEOF(small_pool_hdr) a multiple
 * of the alignment requirement of ALIGN_TYPE.
 */

pub type small_pool_ptr = *mut small_pool_struct;

#[repr(C)]
#[derive(Copy, Clone)]
pub union small_pool_struct {
    pub hdr: C2RustUnnamed_71,
    pub dummy: f64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_71 {
    pub next: small_pool_ptr,
    pub bytes_used: size_t,
    pub bytes_left: size_t,
}

pub type small_pool_hdr = small_pool_struct;

pub type large_pool_hdr = large_pool_struct;
/* next in list of pools */
/* how many bytes already used within pool */
/* bytes still available in this pool */
/* included in union to ensure alignment */
/* next in list of pools */
/* how many bytes already used within pool */
/* bytes still available in this pool */
/* included in union to ensure alignment */
/* System-dependent control info */
/* optional extra stuff for statistics */
/* MEM_STATS */

unsafe extern "C" fn out_of_memory(mut cinfo: j_common_ptr, mut which: i32)
/* Report an out-of-memory error and stop execution */
/* If we compiled MEM_STATS support, report alloc requests before dying */
{
    (*(*cinfo).err).msg_code = JERR_OUT_OF_MEMORY as i32;
    (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = which;
    Some(
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo);
}
/*
 * Allocation of "small" objects.
 *
 * For these, we use pooled storage.  When a new pool must be created,
 * we try to get enough space for the current request plus a "slop" factor,
 * where the slop will be the amount of leftover space in the new pool.
 * The speed vs. space tradeoff is largely determined by the slop values.
 * A different slop value is provided for each pool class (lifetime),
 * and we also distinguish the first pool of a class from later ones.
 * NOTE: the values given work fairly well on both 16- and 32-bit-int
 * machines, but may be too small if longs are 64 bits or more.
 */

static mut first_pool_slop: [size_t; 2] = [1600 as i32 as size_t, 16000 as i32 as size_t];

static mut extra_pool_slop: [size_t; 2] = [0 as i32 as size_t, 5000 as i32 as size_t];
/* greater than 0 to avoid futile looping */

unsafe extern "C" fn alloc_small(
    mut cinfo: j_common_ptr,
    mut pool_id: i32,
    mut sizeofobject: size_t,
) -> *mut libc::c_void
/* Allocate a "small" object */ {
    let mut mem: my_mem_ptr = (*cinfo).mem as my_mem_ptr;
    let mut hdr_ptr: small_pool_ptr = 0 as *mut small_pool_struct;
    let mut prev_hdr_ptr: small_pool_ptr = 0 as *mut small_pool_struct;
    let mut data_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut odd_bytes: size_t = 0;
    let mut min_request: size_t = 0;
    let mut slop: size_t = 0;
    /* Check for unsatisfiable request (do now to ensure no overflow below) */
    if sizeofobject
        > (1000000000 as isize as usize)
            .wrapping_sub(::std::mem::size_of::<small_pool_hdr>() as usize)
    {
        out_of_memory(cinfo, 1 as i32); /* request exceeds malloc's ability */
    }
    /* Round up the requested size to a multiple of SIZEOF(ALIGN_TYPE) */
    odd_bytes = sizeofobject.wrapping_rem(::std::mem::size_of::<f64>() as usize);
    if odd_bytes > 0 as i32 as usize {
        sizeofobject = (sizeofobject as usize)
            .wrapping_add((::std::mem::size_of::<f64>() as usize).wrapping_sub(odd_bytes))
            as size_t
    }
    /* See if space is available in any existing pool */
    if pool_id < 0 as i32 || pool_id >= 2 as i32 {
        (*(*cinfo).err).msg_code = JERR_BAD_POOL_ID as i32; /* safety check */
        (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = pool_id; /* found pool with enough space */
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
    }
    prev_hdr_ptr = 0 as small_pool_ptr;
    hdr_ptr = (*mem).small_list[pool_id as usize];
    while !hdr_ptr.is_null() {
        if (*hdr_ptr).hdr.bytes_left >= sizeofobject {
            break;
        }
        prev_hdr_ptr = hdr_ptr;
        hdr_ptr = (*hdr_ptr).hdr.next
    }
    /* Time to make a new pool? */
    if hdr_ptr.is_null() {
        /* min_request is what we need now, slop is what will be leftover */
        min_request =
            sizeofobject.wrapping_add(::std::mem::size_of::<small_pool_hdr>() as usize);
        if prev_hdr_ptr.is_null() {
            /* first pool in class? */
            slop = first_pool_slop[pool_id as usize]
        } else {
            slop = extra_pool_slop[pool_id as usize]
        }
        /* Don't ask for more than MAX_ALLOC_CHUNK */
        if slop > (1000000000 as isize as usize).wrapping_sub(min_request) {
            slop = (1000000000 as isize as usize).wrapping_sub(min_request)
        }
        loop
        /* Try to get space, if fail reduce slop and try again */
        {
            hdr_ptr = jpeg_get_small(
                cinfo as *mut jpeg_common_struct,
                min_request.wrapping_add(slop),
            ) as small_pool_ptr;
            if !hdr_ptr.is_null() {
                break;
            }
            slop = (slop as usize).wrapping_div(2 as i32 as usize) as size_t;
            if slop < 50 as i32 as usize {
                /* give up when it gets real small */
                out_of_memory(cinfo, 2 as i32);
            }
        }
        (*mem).total_space_allocated = ((*mem).total_space_allocated as usize)
            .wrapping_add(min_request.wrapping_add(slop))
            as isize;
        /* Success, initialize the new pool header and add to end of list */
        (*hdr_ptr).hdr.next = 0 as small_pool_ptr;
        (*hdr_ptr).hdr.bytes_used = 0 as i32 as size_t;
        (*hdr_ptr).hdr.bytes_left = sizeofobject.wrapping_add(slop);
        if prev_hdr_ptr.is_null() {
            /* first pool in class? */
            (*mem).small_list[pool_id as usize] = hdr_ptr
        } else {
            (*prev_hdr_ptr).hdr.next = hdr_ptr
        }
    }
    /* OK, allocate the object from the current pool */
    data_ptr = hdr_ptr.offset(1 as i32 as isize) as *mut libc::c_char; /* point to first data byte in pool */
    data_ptr = data_ptr.offset((*hdr_ptr).hdr.bytes_used as isize); /* point to place for object */
    (*hdr_ptr).hdr.bytes_used =
        ((*hdr_ptr).hdr.bytes_used as usize).wrapping_add(sizeofobject) as size_t;
    (*hdr_ptr).hdr.bytes_left =
        ((*hdr_ptr).hdr.bytes_left as usize).wrapping_sub(sizeofobject) as size_t;
    return data_ptr as *mut libc::c_void;
}
/*
 * Allocation of "large" objects.
 *
 * The external semantics of these are the same as "small" objects,
 * except that FAR pointers are used on 80x86.  However the pool
 * management heuristics are quite different.  We assume that each
 * request is large enough that it may as well be passed directly to
 * jpeg_get_large; the pool management just links everything together
 * so that we can free it all on demand.
 * Note: the major use of "large" objects is in JSAMPARRAY and JBLOCKARRAY
 * structures.  The routines that create these structures (see below)
 * deliberately bunch rows together to ensure a large request size.
 */

unsafe extern "C" fn alloc_large(
    mut cinfo: j_common_ptr,
    mut pool_id: i32,
    mut sizeofobject: size_t,
) -> *mut libc::c_void
/* Allocate a "large" object */ {
    let mut mem: my_mem_ptr = (*cinfo).mem as my_mem_ptr;
    let mut hdr_ptr: large_pool_ptr = 0 as *mut large_pool_struct;
    let mut odd_bytes: size_t = 0;
    /* Check for unsatisfiable request (do now to ensure no overflow below) */
    if sizeofobject
        > (1000000000 as isize as usize)
            .wrapping_sub(::std::mem::size_of::<large_pool_hdr>() as usize)
    {
        out_of_memory(cinfo, 3 as i32); /* request exceeds malloc's ability */
    }
    /* Round up the requested size to a multiple of SIZEOF(ALIGN_TYPE) */
    odd_bytes = sizeofobject.wrapping_rem(::std::mem::size_of::<f64>() as usize);
    if odd_bytes > 0 as i32 as usize {
        sizeofobject = (sizeofobject as usize)
            .wrapping_add((::std::mem::size_of::<f64>() as usize).wrapping_sub(odd_bytes))
            as size_t
    }
    /* Always make a new pool */
    if pool_id < 0 as i32 || pool_id >= 2 as i32 {
        (*(*cinfo).err).msg_code = JERR_BAD_POOL_ID as i32; /* safety check */
        (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = pool_id; /* jpeg_get_large failed */
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
    }
    hdr_ptr = jpeg_get_large(
        cinfo as *mut jpeg_common_struct,
        sizeofobject.wrapping_add(::std::mem::size_of::<large_pool_hdr>() as usize),
    ) as large_pool_ptr;
    if hdr_ptr.is_null() {
        out_of_memory(cinfo, 4 as i32);
    }
    (*mem).total_space_allocated = ((*mem).total_space_allocated as usize).wrapping_add(
        sizeofobject.wrapping_add(::std::mem::size_of::<large_pool_hdr>() as usize),
    ) as isize;
    /* Success, initialize the new pool header and add to list */
    (*hdr_ptr).hdr.next = (*mem).large_list[pool_id as usize];
    /* We maintain space counts in each pool header for statistical purposes,
     * even though they are not needed for allocation.
     */
    (*hdr_ptr).hdr.bytes_used = sizeofobject;
    (*hdr_ptr).hdr.bytes_left = 0 as i32 as size_t;
    (*mem).large_list[pool_id as usize] = hdr_ptr;
    return hdr_ptr.offset(1 as i32 as isize) as *mut libc::c_void;
    /* point to first data byte in pool */
}
/*
 * Creation of 2-D sample arrays.
 * The pointers are in near heap, the samples themselves in FAR heap.
 *
 * To minimize allocation overhead and to allow I/O of large contiguous
 * blocks, we allocate the sample rows in groups of as many rows as possible
 * without exceeding MAX_ALLOC_CHUNK total bytes per allocation request.
 * NB: the virtual array control routines, later in this file, know about
 * this chunking of rows.  The rowsperchunk value is left in the mem manager
 * object so that it can be saved away if this sarray is the workspace for
 * a virtual array.
 */

unsafe extern "C" fn alloc_sarray(
    mut cinfo: j_common_ptr,
    mut pool_id: i32,
    mut samplesperrow: JDIMENSION,
    mut numrows: JDIMENSION,
) -> JSAMPARRAY
/* Allocate a 2-D sample array */ {
    let mut mem: my_mem_ptr = (*cinfo).mem as my_mem_ptr;
    let mut result: JSAMPARRAY = 0 as *mut JSAMPROW;
    let mut workspace: JSAMPROW = 0 as *mut JSAMPLE;
    let mut rowsperchunk: JDIMENSION = 0;
    let mut currow: JDIMENSION = 0;
    let mut i: JDIMENSION = 0;
    let mut ltemp: isize = 0;
    /* Calculate max # of rows allowed in one allocation chunk */
    ltemp = (1000000000 as isize as usize)
        .wrapping_sub(::std::mem::size_of::<large_pool_hdr>() as usize)
        .wrapping_div(
            (samplesperrow as isize as usize)
                .wrapping_mul(::std::mem::size_of::<JSAMPLE>() as usize),
        ) as isize;
    if ltemp <= 0 as i32 as isize {
        (*(*cinfo).err).msg_code = JERR_WIDTH_OVERFLOW as i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
    }
    if ltemp < numrows as isize {
        rowsperchunk = ltemp as JDIMENSION
    } else {
        rowsperchunk = numrows
    }
    (*mem).last_rowsperchunk = rowsperchunk;
    /* Get space for row pointers (small object) */
    result = alloc_small(
        cinfo,
        pool_id,
        (numrows as usize).wrapping_mul(::std::mem::size_of::<JSAMPROW>() as usize),
    ) as JSAMPARRAY;
    /* Get the rows themselves (large objects) */
    currow = 0 as i32 as JDIMENSION;
    while currow < numrows {
        rowsperchunk = if rowsperchunk < numrows.wrapping_sub(currow) {
            rowsperchunk
        } else {
            numrows.wrapping_sub(currow)
        };
        workspace = alloc_large(
            cinfo,
            pool_id,
            (rowsperchunk as size_t)
                .wrapping_mul(samplesperrow as size_t)
                .wrapping_mul(::std::mem::size_of::<JSAMPLE>() as usize),
        ) as JSAMPROW;
        i = rowsperchunk;
        while i > 0 as i32 as u32 {
            let fresh0 = currow;
            currow = currow.wrapping_add(1);
            let ref mut fresh1 = *result.offset(fresh0 as isize);
            *fresh1 = workspace;
            workspace = workspace.offset(samplesperrow as isize);
            i = i.wrapping_sub(1)
        }
    }
    return result;
}
/*
 * Creation of 2-D coefficient-block arrays.
 * This is essentially the same as the code for sample arrays, above.
 */

unsafe extern "C" fn alloc_barray(
    mut cinfo: j_common_ptr,
    mut pool_id: i32,
    mut blocksperrow: JDIMENSION,
    mut numrows: JDIMENSION,
) -> JBLOCKARRAY
/* Allocate a 2-D coefficient-block array */ {
    let mut mem: my_mem_ptr = (*cinfo).mem as my_mem_ptr;
    let mut result: JBLOCKARRAY = 0 as *mut JBLOCKROW;
    let mut workspace: JBLOCKROW = 0 as *mut JBLOCK;
    let mut rowsperchunk: JDIMENSION = 0;
    let mut currow: JDIMENSION = 0;
    let mut i: JDIMENSION = 0;
    let mut ltemp: isize = 0;
    /* Calculate max # of rows allowed in one allocation chunk */
    ltemp = (1000000000 as isize as usize)
        .wrapping_sub(::std::mem::size_of::<large_pool_hdr>() as usize)
        .wrapping_div(
            (blocksperrow as isize as usize)
                .wrapping_mul(::std::mem::size_of::<JBLOCK>() as usize),
        ) as isize;
    if ltemp <= 0 as i32 as isize {
        (*(*cinfo).err).msg_code = JERR_WIDTH_OVERFLOW as i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
    }
    if ltemp < numrows as isize {
        rowsperchunk = ltemp as JDIMENSION
    } else {
        rowsperchunk = numrows
    }
    (*mem).last_rowsperchunk = rowsperchunk;
    /* Get space for row pointers (small object) */
    result = alloc_small(
        cinfo,
        pool_id,
        (numrows as usize)
            .wrapping_mul(::std::mem::size_of::<JBLOCKROW>() as usize),
    ) as JBLOCKARRAY;
    /* Get the rows themselves (large objects) */
    currow = 0 as i32 as JDIMENSION;
    while currow < numrows {
        rowsperchunk = if rowsperchunk < numrows.wrapping_sub(currow) {
            rowsperchunk
        } else {
            numrows.wrapping_sub(currow)
        };
        workspace = alloc_large(
            cinfo,
            pool_id,
            (rowsperchunk as size_t)
                .wrapping_mul(blocksperrow as size_t)
                .wrapping_mul(::std::mem::size_of::<JBLOCK>() as usize),
        ) as JBLOCKROW;
        i = rowsperchunk;
        while i > 0 as i32 as u32 {
            let fresh2 = currow;
            currow = currow.wrapping_add(1);
            let ref mut fresh3 = *result.offset(fresh2 as isize);
            *fresh3 = workspace;
            workspace = workspace.offset(blocksperrow as isize);
            i = i.wrapping_sub(1)
        }
    }
    return result;
}
/*
 * About virtual array management:
 *
 * The above "normal" array routines are only used to allocate strip buffers
 * (as wide as the image, but just a few rows high).  Full-image-sized buffers
 * are handled as "virtual" arrays.  The array is still accessed a strip at a
 * time, but the memory manager must save the whole array for repeated
 * accesses.  The intended implementation is that there is a strip buffer in
 * memory (as high as is possible given the desired memory limit), plus a
 * backing file that holds the rest of the array.
 *
 * The request_virt_array routines are told the total size of the image and
 * the maximum number of rows that will be accessed at once.  The in-memory
 * buffer must be at least as large as the maxaccess value.
 *
 * The request routines create control blocks but not the in-memory buffers.
 * That is postponed until realize_virt_arrays is called.  At that time the
 * total amount of space needed is known (approximately, anyway), so free
 * memory can be divided up fairly.
 *
 * The access_virt_array routines are responsible for making a specific strip
 * area accessible (after reading or writing the backing file, if necessary).
 * Note that the access routines are told whether the caller intends to modify
 * the accessed strip; during a read-only pass this saves having to rewrite
 * data to disk.  The access routines are also responsible for pre-zeroing
 * any newly accessed rows, if pre-zeroing was requested.
 *
 * In current usage, the access requests are usually for nonoverlapping
 * strips; that is, successive access start_row numbers differ by exactly
 * num_rows = maxaccess.  This means we can get good performance with simple
 * buffer dump/reload logic, by making the in-memory buffer be a multiple
 * of the access height; then there will never be accesses across bufferload
 * boundaries.  The code will still work with overlapping access requests,
 * but it doesn't handle bufferload overlaps very efficiently.
 */

unsafe extern "C" fn request_virt_sarray(
    mut cinfo: j_common_ptr,
    mut pool_id: i32,
    mut pre_zero: boolean,
    mut samplesperrow: JDIMENSION,
    mut numrows: JDIMENSION,
    mut maxaccess: JDIMENSION,
) -> jvirt_sarray_ptr
/* Request a virtual 2-D sample array */ {
    let mut mem: my_mem_ptr = (*cinfo).mem as my_mem_ptr;
    let mut result: jvirt_sarray_ptr = 0 as *mut jvirt_sarray_control;
    /* Only IMAGE-lifetime virtual arrays are currently supported */
    if pool_id != 1 as i32 {
        (*(*cinfo).err).msg_code = JERR_BAD_POOL_ID as i32; /* safety check */
        (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = pool_id;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
    }
    /* get control block */
    result = alloc_small(
        cinfo,
        pool_id,
        ::std::mem::size_of::<jvirt_sarray_control>() as usize,
    ) as jvirt_sarray_ptr; /* marks array not yet realized */
    (*result).mem_buffer = 0 as JSAMPARRAY; /* no associated backing-store object */
    (*result).rows_in_array = numrows; /* add to list of virtual arrays */
    (*result).samplesperrow = samplesperrow;
    (*result).maxaccess = maxaccess;
    (*result).pre_zero = pre_zero;
    (*result).b_s_open = 0 as i32;
    (*result).next = (*mem).virt_sarray_list;
    (*mem).virt_sarray_list = result;
    return result;
}

unsafe extern "C" fn request_virt_barray(
    mut cinfo: j_common_ptr,
    mut pool_id: i32,
    mut pre_zero: boolean,
    mut blocksperrow: JDIMENSION,
    mut numrows: JDIMENSION,
    mut maxaccess: JDIMENSION,
) -> jvirt_barray_ptr
/* Request a virtual 2-D coefficient-block array */ {
    let mut mem: my_mem_ptr = (*cinfo).mem as my_mem_ptr;
    let mut result: jvirt_barray_ptr = 0 as *mut jvirt_barray_control;
    /* Only IMAGE-lifetime virtual arrays are currently supported */
    if pool_id != 1 as i32 {
        (*(*cinfo).err).msg_code = JERR_BAD_POOL_ID as i32; /* safety check */
        (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = pool_id;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
    }
    /* get control block */
    result = alloc_small(
        cinfo,
        pool_id,
        ::std::mem::size_of::<jvirt_barray_control>() as usize,
    ) as jvirt_barray_ptr; /* marks array not yet realized */
    (*result).mem_buffer = 0 as JBLOCKARRAY; /* no associated backing-store object */
    (*result).rows_in_array = numrows; /* add to list of virtual arrays */
    (*result).blocksperrow = blocksperrow;
    (*result).maxaccess = maxaccess;
    (*result).pre_zero = pre_zero;
    (*result).b_s_open = 0 as i32;
    (*result).next = (*mem).virt_barray_list;
    (*mem).virt_barray_list = result;
    return result;
}

unsafe extern "C" fn realize_virt_arrays(mut cinfo: j_common_ptr)
/* Allocate the in-memory buffers for any unrealized virtual arrays */
{
    let mut mem: my_mem_ptr = (*cinfo).mem as my_mem_ptr;
    let mut space_per_minheight: isize = 0;
    let mut maximum_space: isize = 0;
    let mut avail_mem: isize = 0;
    let mut minheights: isize = 0;
    let mut max_minheights: isize = 0;
    let mut sptr: jvirt_sarray_ptr = 0 as *mut jvirt_sarray_control;
    let mut bptr: jvirt_barray_ptr = 0 as *mut jvirt_barray_control;
    /* Compute the minimum space needed (maxaccess rows in each buffer)
     * and the maximum space needed (full image height in each buffer).
     * These may be of use to the system-dependent jpeg_mem_available routine.
     */
    space_per_minheight = 0 as i32 as isize;
    maximum_space = 0 as i32 as isize;
    sptr = (*mem).virt_sarray_list;
    while !sptr.is_null() {
        if (*sptr).mem_buffer.is_null() {
            /* if not realized yet */
            space_per_minheight = (space_per_minheight as usize).wrapping_add(
                (((*sptr).maxaccess as isize * (*sptr).samplesperrow as isize) as usize)
                    .wrapping_mul(::std::mem::size_of::<JSAMPLE>() as usize),
            ) as isize;
            maximum_space = (maximum_space as usize).wrapping_add(
                (((*sptr).rows_in_array as isize * (*sptr).samplesperrow as isize)
                    as usize)
                    .wrapping_mul(::std::mem::size_of::<JSAMPLE>() as usize),
            ) as isize
        }
        sptr = (*sptr).next
    }
    bptr = (*mem).virt_barray_list;
    while !bptr.is_null() {
        if (*bptr).mem_buffer.is_null() {
            /* if not realized yet */
            space_per_minheight = (space_per_minheight as usize).wrapping_add(
                (((*bptr).maxaccess as isize * (*bptr).blocksperrow as isize) as usize)
                    .wrapping_mul(::std::mem::size_of::<JBLOCK>() as usize),
            ) as isize; /* no unrealized arrays, no work */
            maximum_space = (maximum_space as usize).wrapping_add(
                (((*bptr).rows_in_array as isize * (*bptr).blocksperrow as isize) as usize)
                    .wrapping_mul(::std::mem::size_of::<JBLOCK>() as usize),
            ) as isize
        }
        bptr = (*bptr).next
    }
    if space_per_minheight <= 0 as i32 as isize {
        return;
    }
    /* Determine amount of memory to actually use; this is system-dependent. */
    avail_mem = jpeg_mem_available(
        cinfo as *mut jpeg_common_struct,
        space_per_minheight,
        maximum_space,
        (*mem).total_space_allocated,
    );
    /* If the maximum space needed is available, make all the buffers full
     * height; otherwise parcel it out with the same number of minheights
     * in each buffer.
     */
    if avail_mem >= maximum_space {
        max_minheights = 1000000000 as isize
    } else {
        max_minheights = avail_mem / space_per_minheight;
        /* If there doesn't seem to be enough space, try to get the minimum
         * anyway.  This allows a "stub" implementation of jpeg_mem_available().
         */
        if max_minheights <= 0 as i32 as isize {
            max_minheights = 1 as i32 as isize
        }
    }
    /* Allocate the in-memory buffers and initialize backing store as needed. */
    sptr = (*mem).virt_sarray_list;
    while !sptr.is_null() {
        if (*sptr).mem_buffer.is_null() {
            /* if not realized yet */
            minheights = ((*sptr).rows_in_array as isize - 1 as isize) / (*sptr).maxaccess as isize
                + 1 as isize;
            if minheights <= max_minheights {
                /* This buffer fits in memory */
                (*sptr).rows_in_mem = (*sptr).rows_in_array
            } else {
                /* It doesn't fit in memory, create backing store. */
                (*sptr).rows_in_mem = (max_minheights * (*sptr).maxaccess as isize) as JDIMENSION;
                jpeg_open_backing_store(
                    cinfo as *mut jpeg_common_struct,
                    &mut (*sptr).b_s_info as *mut _ as *mut backing_store_struct,
                    (*sptr).rows_in_array as isize
                        * (*sptr).samplesperrow as isize
                        * ::std::mem::size_of::<JSAMPLE>() as usize as isize,
                );
                (*sptr).b_s_open = 1 as i32
            }
            (*sptr).mem_buffer =
                alloc_sarray(cinfo, 1 as i32, (*sptr).samplesperrow, (*sptr).rows_in_mem);
            (*sptr).rowsperchunk = (*mem).last_rowsperchunk;
            (*sptr).cur_start_row = 0 as i32 as JDIMENSION;
            (*sptr).first_undef_row = 0 as i32 as JDIMENSION;
            (*sptr).dirty = 0 as i32
        }
        sptr = (*sptr).next
    }
    bptr = (*mem).virt_barray_list;
    while !bptr.is_null() {
        if (*bptr).mem_buffer.is_null() {
            /* if not realized yet */
            minheights = ((*bptr).rows_in_array as isize - 1 as isize) / (*bptr).maxaccess as isize
                + 1 as isize;
            if minheights <= max_minheights {
                /* This buffer fits in memory */
                (*bptr).rows_in_mem = (*bptr).rows_in_array
            } else {
                /* It doesn't fit in memory, create backing store. */
                (*bptr).rows_in_mem = (max_minheights * (*bptr).maxaccess as isize) as JDIMENSION;
                jpeg_open_backing_store(
                    cinfo as *mut jpeg_common_struct,
                    &mut (*bptr).b_s_info as *mut _ as *mut backing_store_struct,
                    (*bptr).rows_in_array as isize
                        * (*bptr).blocksperrow as isize
                        * ::std::mem::size_of::<JBLOCK>() as usize as isize,
                );
                (*bptr).b_s_open = 1 as i32
            }
            (*bptr).mem_buffer =
                alloc_barray(cinfo, 1 as i32, (*bptr).blocksperrow, (*bptr).rows_in_mem);
            (*bptr).rowsperchunk = (*mem).last_rowsperchunk;
            (*bptr).cur_start_row = 0 as i32 as JDIMENSION;
            (*bptr).first_undef_row = 0 as i32 as JDIMENSION;
            (*bptr).dirty = 0 as i32
        }
        bptr = (*bptr).next
    }
}

unsafe extern "C" fn do_sarray_io(
    mut cinfo: j_common_ptr,
    mut ptr: jvirt_sarray_ptr,
    mut writing: boolean,
)
/* Do backing store read or write of a virtual sample array */
{
    let mut bytesperrow: isize = 0;
    let mut file_offset: isize = 0;
    let mut byte_count: isize = 0;
    let mut rows: isize = 0;
    let mut thisrow: isize = 0;
    let mut i: isize = 0;
    bytesperrow = ((*ptr).samplesperrow as isize as usize)
        .wrapping_mul(::std::mem::size_of::<JSAMPLE>() as usize) as isize;
    file_offset = (*ptr).cur_start_row as isize * bytesperrow;
    /* Loop to read or write each allocation chunk in mem_buffer */
    i = 0 as i32 as isize;
    while i < (*ptr).rows_in_mem as isize {
        /* One chunk, but check for short chunk at end of buffer */
        rows = if ((*ptr).rowsperchunk as isize) < (*ptr).rows_in_mem as isize - i {
            (*ptr).rowsperchunk as isize
        } else {
            ((*ptr).rows_in_mem as isize) - i
        };
        /* Transfer no more than is currently defined */
        thisrow = (*ptr).cur_start_row as isize + i;
        rows = if rows < (*ptr).first_undef_row as isize - thisrow {
            rows
        } else {
            ((*ptr).first_undef_row as isize) - thisrow
        };
        /* Transfer no more than fits in file */
        rows = if rows < (*ptr).rows_in_array as isize - thisrow {
            rows
        } else {
            ((*ptr).rows_in_array as isize) - thisrow
        };
        if rows <= 0 as i32 as isize {
            break;
        }
        byte_count = rows * bytesperrow;
        if writing != 0 {
            Some(
                (*ptr)
                    .b_s_info
                    .write_backing_store
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo,
                &mut (*ptr).b_s_info,
                *(*ptr).mem_buffer.offset(i as isize) as *mut libc::c_void,
                file_offset,
                byte_count,
            );
        } else {
            Some(
                (*ptr)
                    .b_s_info
                    .read_backing_store
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo,
                &mut (*ptr).b_s_info,
                *(*ptr).mem_buffer.offset(i as isize) as *mut libc::c_void,
                file_offset,
                byte_count,
            );
        }
        file_offset += byte_count;
        i += (*ptr).rowsperchunk as isize
    }
}

unsafe extern "C" fn do_barray_io(
    mut cinfo: j_common_ptr,
    mut ptr: jvirt_barray_ptr,
    mut writing: boolean,
)
/* Do backing store read or write of a virtual coefficient-block array */
{
    let mut bytesperrow: isize = 0;
    let mut file_offset: isize = 0;
    let mut byte_count: isize = 0;
    let mut rows: isize = 0;
    let mut thisrow: isize = 0;
    let mut i: isize = 0;
    bytesperrow = ((*ptr).blocksperrow as isize as usize)
        .wrapping_mul(::std::mem::size_of::<JBLOCK>() as usize) as isize;
    file_offset = (*ptr).cur_start_row as isize * bytesperrow;
    /* Loop to read or write each allocation chunk in mem_buffer */
    i = 0 as i32 as isize;
    while i < (*ptr).rows_in_mem as isize {
        /* One chunk, but check for short chunk at end of buffer */
        rows = if ((*ptr).rowsperchunk as isize) < (*ptr).rows_in_mem as isize - i {
            (*ptr).rowsperchunk as isize
        } else {
            ((*ptr).rows_in_mem as isize) - i
        };
        /* Transfer no more than is currently defined */
        thisrow = (*ptr).cur_start_row as isize + i;
        rows = if rows < (*ptr).first_undef_row as isize - thisrow {
            rows
        } else {
            ((*ptr).first_undef_row as isize) - thisrow
        };
        /* Transfer no more than fits in file */
        rows = if rows < (*ptr).rows_in_array as isize - thisrow {
            rows
        } else {
            ((*ptr).rows_in_array as isize) - thisrow
        };
        if rows <= 0 as i32 as isize {
            break;
        }
        byte_count = rows * bytesperrow;
        if writing != 0 {
            Some(
                (*ptr)
                    .b_s_info
                    .write_backing_store
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo,
                &mut (*ptr).b_s_info,
                *(*ptr).mem_buffer.offset(i as isize) as *mut libc::c_void,
                file_offset,
                byte_count,
            );
        } else {
            Some(
                (*ptr)
                    .b_s_info
                    .read_backing_store
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo,
                &mut (*ptr).b_s_info,
                *(*ptr).mem_buffer.offset(i as isize) as *mut libc::c_void,
                file_offset,
                byte_count,
            );
        }
        file_offset += byte_count;
        i += (*ptr).rowsperchunk as isize
    }
}

unsafe extern "C" fn access_virt_sarray(
    mut cinfo: j_common_ptr,
    mut ptr: jvirt_sarray_ptr,
    mut start_row: JDIMENSION,
    mut num_rows: JDIMENSION,
    mut writable: boolean,
) -> JSAMPARRAY
/* Access the part of a virtual sample array starting at start_row */
/* and extending for num_rows rows.  writable is true if  */
/* caller intends to modify the accessed area. */ {
    let mut end_row: JDIMENSION = start_row.wrapping_add(num_rows);
    let mut undef_row: JDIMENSION = 0;
    /* debugging check */
    if end_row > (*ptr).rows_in_array || num_rows > (*ptr).maxaccess || (*ptr).mem_buffer.is_null()
    {
        (*(*cinfo).err).msg_code = JERR_BAD_VIRTUAL_ACCESS as i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
    }
    /* Make the desired part of the virtual array accessible */
    if start_row < (*ptr).cur_start_row
        || end_row > (*ptr).cur_start_row.wrapping_add((*ptr).rows_in_mem)
    {
        if (*ptr).b_s_open == 0 {
            (*(*cinfo).err).msg_code = JERR_VIRTUAL_BUG as i32;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo);
        }
        /* Flush old buffer contents if necessary */
        if (*ptr).dirty != 0 {
            do_sarray_io(cinfo, ptr, 1 as i32);
            (*ptr).dirty = 0 as i32
        }
        /* Decide what part of virtual array to access.
         * Algorithm: if target address > current window, assume forward scan,
         * load starting at target address.  If target address < current window,
         * assume backward scan, load so that target area is top of window.
         * Note that when switching from forward write to forward read, will have
         * start_row = 0, so the limiting case applies and we load from 0 anyway.
         */
        if start_row > (*ptr).cur_start_row {
            (*ptr).cur_start_row = start_row
        } else {
            /* use long arithmetic here to avoid overflow & unsigned problems */
            let mut ltemp: isize = 0; /* don't fall off front end of file */
            ltemp = end_row as isize - (*ptr).rows_in_mem as isize;
            if ltemp < 0 as i32 as isize {
                ltemp = 0 as i32 as isize
            }
            (*ptr).cur_start_row = ltemp as JDIMENSION
        }
        /* Read in the selected part of the array.
         * During the initial write pass, we will do no actual read
         * because the selected part is all undefined.
         */
        do_sarray_io(cinfo, ptr, 0 as i32);
    }
    /* Ensure the accessed part of the array is defined; prezero if needed.
     * To improve locality of access, we only prezero the part of the array
     * that the caller is about to access, not the entire in-memory array.
     */
    if (*ptr).first_undef_row < end_row {
        if (*ptr).first_undef_row < start_row {
            if writable != 0 {
                /* writer skipped over a section of array */
                (*(*cinfo).err).msg_code = JERR_BAD_VIRTUAL_ACCESS as i32;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo);
            }
            undef_row = start_row
        /* but reader is allowed to read ahead */
        } else {
            undef_row = (*ptr).first_undef_row
        } /* make indexes relative to buffer */
        if writable != 0 {
            (*ptr).first_undef_row = end_row
        }
        if (*ptr).pre_zero != 0 {
            let mut bytesperrow: size_t = ((*ptr).samplesperrow as size_t)
                .wrapping_mul(::std::mem::size_of::<JSAMPLE>() as usize);
            undef_row = (undef_row as u32).wrapping_sub((*ptr).cur_start_row) as JDIMENSION;
            end_row = (end_row as u32).wrapping_sub((*ptr).cur_start_row) as JDIMENSION;
            while undef_row < end_row {
                crate::src::jpeg_8c::jutils::jzero_far(
                    *(*ptr).mem_buffer.offset(undef_row as isize) as *mut libc::c_void,
                    bytesperrow,
                );
                undef_row = undef_row.wrapping_add(1)
            }
        } else if writable == 0 {
            /* reader looking at undefined data */
            (*(*cinfo).err).msg_code = JERR_BAD_VIRTUAL_ACCESS as i32;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo);
        }
    }
    /* Flag the buffer dirty if caller will write in it */
    if writable != 0 {
        (*ptr).dirty = 1 as i32
    }
    /* Return address of proper part of the buffer */
    return (*ptr)
        .mem_buffer
        .offset(start_row.wrapping_sub((*ptr).cur_start_row) as isize);
}

unsafe extern "C" fn access_virt_barray(
    mut cinfo: j_common_ptr,
    mut ptr: jvirt_barray_ptr,
    mut start_row: JDIMENSION,
    mut num_rows: JDIMENSION,
    mut writable: boolean,
) -> JBLOCKARRAY
/* Access the part of a virtual block array starting at start_row */
/* and extending for num_rows rows.  writable is true if  */
/* caller intends to modify the accessed area. */ {
    let mut end_row: JDIMENSION = start_row.wrapping_add(num_rows);
    let mut undef_row: JDIMENSION = 0;
    /* debugging check */
    if end_row > (*ptr).rows_in_array || num_rows > (*ptr).maxaccess || (*ptr).mem_buffer.is_null()
    {
        (*(*cinfo).err).msg_code = JERR_BAD_VIRTUAL_ACCESS as i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
    }
    /* Make the desired part of the virtual array accessible */
    if start_row < (*ptr).cur_start_row
        || end_row > (*ptr).cur_start_row.wrapping_add((*ptr).rows_in_mem)
    {
        if (*ptr).b_s_open == 0 {
            (*(*cinfo).err).msg_code = JERR_VIRTUAL_BUG as i32;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo);
        }
        /* Flush old buffer contents if necessary */
        if (*ptr).dirty != 0 {
            do_barray_io(cinfo, ptr, 1 as i32);
            (*ptr).dirty = 0 as i32
        }
        /* Decide what part of virtual array to access.
         * Algorithm: if target address > current window, assume forward scan,
         * load starting at target address.  If target address < current window,
         * assume backward scan, load so that target area is top of window.
         * Note that when switching from forward write to forward read, will have
         * start_row = 0, so the limiting case applies and we load from 0 anyway.
         */
        if start_row > (*ptr).cur_start_row {
            (*ptr).cur_start_row = start_row
        } else {
            /* use long arithmetic here to avoid overflow & unsigned problems */
            let mut ltemp: isize = 0; /* don't fall off front end of file */
            ltemp = end_row as isize - (*ptr).rows_in_mem as isize;
            if ltemp < 0 as i32 as isize {
                ltemp = 0 as i32 as isize
            }
            (*ptr).cur_start_row = ltemp as JDIMENSION
        }
        /* Read in the selected part of the array.
         * During the initial write pass, we will do no actual read
         * because the selected part is all undefined.
         */
        do_barray_io(cinfo, ptr, 0 as i32);
    }
    /* Ensure the accessed part of the array is defined; prezero if needed.
     * To improve locality of access, we only prezero the part of the array
     * that the caller is about to access, not the entire in-memory array.
     */
    if (*ptr).first_undef_row < end_row {
        if (*ptr).first_undef_row < start_row {
            if writable != 0 {
                /* writer skipped over a section of array */
                (*(*cinfo).err).msg_code = JERR_BAD_VIRTUAL_ACCESS as i32;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo);
            }
            undef_row = start_row
        /* but reader is allowed to read ahead */
        } else {
            undef_row = (*ptr).first_undef_row
        } /* make indexes relative to buffer */
        if writable != 0 {
            (*ptr).first_undef_row = end_row
        }
        if (*ptr).pre_zero != 0 {
            let mut bytesperrow: size_t = ((*ptr).blocksperrow as size_t)
                .wrapping_mul(::std::mem::size_of::<JBLOCK>() as usize);
            undef_row = (undef_row as u32).wrapping_sub((*ptr).cur_start_row) as JDIMENSION;
            end_row = (end_row as u32).wrapping_sub((*ptr).cur_start_row) as JDIMENSION;
            while undef_row < end_row {
                crate::src::jpeg_8c::jutils::jzero_far(
                    *(*ptr).mem_buffer.offset(undef_row as isize) as *mut libc::c_void,
                    bytesperrow,
                );
                undef_row = undef_row.wrapping_add(1)
            }
        } else if writable == 0 {
            /* reader looking at undefined data */
            (*(*cinfo).err).msg_code = JERR_BAD_VIRTUAL_ACCESS as i32;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo);
        }
    }
    /* Flag the buffer dirty if caller will write in it */
    if writable != 0 {
        (*ptr).dirty = 1 as i32
    }
    /* Return address of proper part of the buffer */
    return (*ptr)
        .mem_buffer
        .offset(start_row.wrapping_sub((*ptr).cur_start_row) as isize);
}
/*
 * Release all objects belonging to a specified pool.
 */

unsafe extern "C" fn free_pool(mut cinfo: j_common_ptr, mut pool_id: i32) {
    let mut mem: my_mem_ptr = (*cinfo).mem as my_mem_ptr; /* safety check */
    let mut shdr_ptr: small_pool_ptr = 0 as *mut small_pool_struct;
    let mut lhdr_ptr: large_pool_ptr = 0 as *mut large_pool_struct;
    let mut space_freed: size_t = 0;
    if pool_id < 0 as i32 || pool_id >= 2 as i32 {
        (*(*cinfo).err).msg_code = JERR_BAD_POOL_ID as i32;
        (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = pool_id;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
    }
    /* If freeing IMAGE pool, close any virtual arrays first */
    if pool_id == 1 as i32 {
        let mut sptr: jvirt_sarray_ptr = 0 as *mut jvirt_sarray_control;
        let mut bptr: jvirt_barray_ptr = 0 as *mut jvirt_barray_control;
        sptr = (*mem).virt_sarray_list;
        while !sptr.is_null() {
            if (*sptr).b_s_open != 0 {
                /* there may be no backing store */
                (*sptr).b_s_open = 0 as i32; /* prevent recursive close if error */
                Some(
                    (*sptr)
                        .b_s_info
                        .close_backing_store
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo, &mut (*sptr).b_s_info);
            }
            sptr = (*sptr).next
        }
        (*mem).virt_sarray_list = 0 as jvirt_sarray_ptr;
        bptr = (*mem).virt_barray_list;
        while !bptr.is_null() {
            if (*bptr).b_s_open != 0 {
                /* there may be no backing store */
                (*bptr).b_s_open = 0 as i32; /* prevent recursive close if error */
                Some(
                    (*bptr)
                        .b_s_info
                        .close_backing_store
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo, &mut (*bptr).b_s_info);
            }
            bptr = (*bptr).next
        }
        (*mem).virt_barray_list = 0 as jvirt_barray_ptr
    }
    /* Release large objects */
    lhdr_ptr = (*mem).large_list[pool_id as usize];
    (*mem).large_list[pool_id as usize] = 0 as large_pool_ptr;
    while !lhdr_ptr.is_null() {
        let mut next_lhdr_ptr: large_pool_ptr = (*lhdr_ptr).hdr.next;
        space_freed = (*lhdr_ptr)
            .hdr
            .bytes_used
            .wrapping_add((*lhdr_ptr).hdr.bytes_left)
            .wrapping_add(::std::mem::size_of::<large_pool_hdr>() as usize);
        jpeg_free_large(
            cinfo as *mut jpeg_common_struct,
            lhdr_ptr as *mut libc::c_void,
            space_freed,
        );
        (*mem).total_space_allocated =
            ((*mem).total_space_allocated as usize).wrapping_sub(space_freed) as isize;
        lhdr_ptr = next_lhdr_ptr
    }
    /* Release small objects */
    shdr_ptr = (*mem).small_list[pool_id as usize];
    (*mem).small_list[pool_id as usize] = 0 as small_pool_ptr;
    while !shdr_ptr.is_null() {
        let mut next_shdr_ptr: small_pool_ptr = (*shdr_ptr).hdr.next;
        space_freed = (*shdr_ptr)
            .hdr
            .bytes_used
            .wrapping_add((*shdr_ptr).hdr.bytes_left)
            .wrapping_add(::std::mem::size_of::<small_pool_hdr>() as usize);
        jpeg_free_small(
            cinfo as *mut jpeg_common_struct,
            shdr_ptr as *mut libc::c_void,
            space_freed,
        );
        (*mem).total_space_allocated =
            ((*mem).total_space_allocated as usize).wrapping_sub(space_freed) as isize;
        shdr_ptr = next_shdr_ptr
    }
}
/*
 * Close up shop entirely.
 * Note that this cannot be called unless cinfo->mem is non-NULL.
 */

unsafe extern "C" fn self_destruct(mut cinfo: j_common_ptr) {
    let mut pool: i32 = 0;
    /* Close all backing store, release all memory.
     * Releasing pools in reverse order might help avoid fragmentation
     * with some (brain-damaged) malloc libraries.
     */
    pool = 2 as i32 - 1 as i32;
    while pool >= 0 as i32 {
        free_pool(cinfo, pool);
        pool -= 1
    }
    /* Release the memory manager control block too. */
    jpeg_free_small(
        cinfo as *mut jpeg_common_struct,
        (*cinfo).mem as *mut libc::c_void,
        ::std::mem::size_of::<my_memory_mgr>() as usize,
    ); /* ensures I will be called only once */
    (*cinfo).mem = 0 as *mut jpeg_memory_mgr;
    jpeg_mem_term(cinfo as *mut jpeg_common_struct);
    /* system-dependent cleanup */
}
/*
 * Memory manager initialization.
 * When this is called, only the error manager pointer is valid in cinfo!
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_memory_mgr(mut cinfo: j_common_ptr) {
    let mut mem: my_mem_ptr = 0 as *mut my_memory_mgr; /* for safety if init fails */
    let mut max_to_use: isize = 0;
    let mut pool: i32 = 0;
    let mut test_mac: size_t = 0;
    (*cinfo).mem = 0 as *mut jpeg_memory_mgr;
    /* Check for configuration errors.
     * SIZEOF(ALIGN_TYPE) should be a power of 2; otherwise, it probably
     * doesn't reflect any real hardware alignment requirement.
     * The test is a little tricky: for X>0, X and X-1 have no one-bits
     * in common if and only if X is a power of 2, ie has only one one-bit.
     * Some compilers may give an "unreachable code" warning here; ignore it.
     */
    if ::std::mem::size_of::<f64>() as usize
        & (::std::mem::size_of::<f64>() as usize).wrapping_sub(1 as i32 as usize)
        != 0 as i32 as usize
    {
        (*(*cinfo).err).msg_code = JERR_BAD_ALIGN_TYPE as i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
    }
    /* MAX_ALLOC_CHUNK must be representable as type size_t, and must be
     * a multiple of SIZEOF(ALIGN_TYPE).
     * Again, an "unreachable code" warning may be ignored here.
     * But a "constant too large" warning means you need to fix MAX_ALLOC_CHUNK.
     */
    test_mac = 1000000000 as isize as size_t; /* system-dependent initialization */
    if test_mac as isize != 1000000000 as isize
        || (1000000000 as isize as usize)
            .wrapping_rem(::std::mem::size_of::<f64>() as usize)
            != 0 as i32 as usize
    {
        (*(*cinfo).err).msg_code = JERR_BAD_ALLOC_CHUNK as i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
    }
    max_to_use = jpeg_mem_init(cinfo as *mut jpeg_common_struct);
    /* Attempt to allocate memory manager's control block */
    mem = jpeg_get_small(
        cinfo as *mut jpeg_common_struct,
        ::std::mem::size_of::<my_memory_mgr>() as usize,
    ) as my_mem_ptr; /* system-dependent cleanup */
    if mem.is_null() {
        jpeg_mem_term(cinfo as *mut jpeg_common_struct);
        (*(*cinfo).err).msg_code = JERR_OUT_OF_MEMORY as i32;
        (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = 0 as i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
    }
    /* OK, fill in the method pointers */
    (*mem).pub_0.alloc_small = Some(
        alloc_small
            as unsafe extern "C" fn(_: j_common_ptr, _: i32, _: size_t) -> *mut libc::c_void,
    );
    (*mem).pub_0.alloc_large = Some(
        alloc_large
            as unsafe extern "C" fn(_: j_common_ptr, _: i32, _: size_t) -> *mut libc::c_void,
    );
    (*mem).pub_0.alloc_sarray = Some(
        alloc_sarray
            as unsafe extern "C" fn(
                _: j_common_ptr,
                _: i32,
                _: JDIMENSION,
                _: JDIMENSION,
            ) -> JSAMPARRAY,
    );
    (*mem).pub_0.alloc_barray = Some(
        alloc_barray
            as unsafe extern "C" fn(
                _: j_common_ptr,
                _: i32,
                _: JDIMENSION,
                _: JDIMENSION,
            ) -> JBLOCKARRAY,
    );
    (*mem).pub_0.request_virt_sarray = Some(
        request_virt_sarray
            as unsafe extern "C" fn(
                _: j_common_ptr,
                _: i32,
                _: boolean,
                _: JDIMENSION,
                _: JDIMENSION,
                _: JDIMENSION,
            ) -> jvirt_sarray_ptr,
    );
    (*mem).pub_0.request_virt_barray = Some(
        request_virt_barray
            as unsafe extern "C" fn(
                _: j_common_ptr,
                _: i32,
                _: boolean,
                _: JDIMENSION,
                _: JDIMENSION,
                _: JDIMENSION,
            ) -> jvirt_barray_ptr,
    );
    (*mem).pub_0.realize_virt_arrays =
        Some(realize_virt_arrays as unsafe extern "C" fn(_: j_common_ptr) -> ());
    (*mem).pub_0.access_virt_sarray = Some(
        access_virt_sarray
            as unsafe extern "C" fn(
                _: j_common_ptr,
                _: jvirt_sarray_ptr,
                _: JDIMENSION,
                _: JDIMENSION,
                _: boolean,
            ) -> JSAMPARRAY,
    );
    (*mem).pub_0.access_virt_barray = Some(
        access_virt_barray
            as unsafe extern "C" fn(
                _: j_common_ptr,
                _: jvirt_barray_ptr,
                _: JDIMENSION,
                _: JDIMENSION,
                _: boolean,
            ) -> JBLOCKARRAY,
    );
    (*mem).pub_0.free_pool = Some(free_pool as unsafe extern "C" fn(_: j_common_ptr, _: i32) -> ());
    (*mem).pub_0.self_destruct = Some(self_destruct as unsafe extern "C" fn(_: j_common_ptr) -> ());
    /* Make MAX_ALLOC_CHUNK accessible to other modules */
    (*mem).pub_0.max_alloc_chunk = 1000000000 as isize;
    /* Initialize working state */
    (*mem).pub_0.max_memory_to_use = max_to_use;
    pool = 2 as i32 - 1 as i32;
    while pool >= 0 as i32 {
        (*mem).small_list[pool as usize] = 0 as small_pool_ptr;
        (*mem).large_list[pool as usize] = 0 as large_pool_ptr;
        pool -= 1
    }
    (*mem).virt_sarray_list = 0 as jvirt_sarray_ptr;
    (*mem).virt_barray_list = 0 as jvirt_barray_ptr;
    (*mem).total_space_allocated = ::std::mem::size_of::<my_memory_mgr>() as usize as isize;
    /* Declare ourselves open for business */
    (*cinfo).mem = &mut (*mem).pub_0;
    /* Check for an environment variable JPEGMEM; if found, override the
     * default max_memory setting from jpeg_mem_init.  Note that the
     * surrounding application may again override this value.
     * If your system doesn't support getenv(), define NO_GETENV to disable
     * this feature.
     */
    let mut memenv: *mut libc::c_char = 0 as *mut libc::c_char;
    memenv = libc::getenv(b"JPEGMEM\x00" as *const u8 as *const libc::c_char);
    if !memenv.is_null() {
        let mut ch: libc::c_char = 'x' as i32 as libc::c_char;
        if libc::sscanf(
            memenv,
            b"%ld%c\x00" as *const u8 as *const libc::c_char,
            &mut max_to_use as *mut isize,
            &mut ch as *mut libc::c_char,
        ) > 0 as i32
        {
            if ch as i32 == 'm' as i32 || ch as i32 == 'M' as i32 {
                max_to_use *= 1000 as isize
            }
            (*mem).pub_0.max_memory_to_use = max_to_use * 1000 as isize
        }
    };
}
