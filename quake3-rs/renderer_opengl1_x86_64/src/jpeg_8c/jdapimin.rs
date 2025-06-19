use ::libc;

pub use crate::stddef_h::size_t;

pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpegint_h::inverse_DCT_method_ptr;
pub use crate::jpegint_h::jpeg_color_deconverter;
pub use crate::jpegint_h::jpeg_color_quantizer;
pub use crate::jpegint_h::jpeg_d_coef_controller;
pub use crate::jpegint_h::jpeg_d_main_controller;
pub use crate::jpegint_h::jpeg_d_post_controller;
pub use crate::jpegint_h::jpeg_decomp_master;
pub use crate::jpegint_h::jpeg_entropy_decoder;
pub use crate::jpegint_h::jpeg_input_controller;
pub use crate::jpegint_h::jpeg_inverse_dct;
pub use crate::jpegint_h::jpeg_marker_reader;
pub use crate::jpegint_h::jpeg_upsampler;
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
pub use crate::jpegint_h::J_BUF_MODE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_decompress_ptr;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_decompress_struct;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_marker_parser_method;
pub use crate::jpeglib_h::jpeg_marker_struct;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_saved_marker_ptr;
pub use crate::jpeglib_h::jpeg_source_mgr;
pub use crate::jpeglib_h::jvirt_barray_control;
pub use crate::jpeglib_h::jvirt_barray_ptr;
pub use crate::jpeglib_h::jvirt_sarray_control;
pub use crate::jpeglib_h::jvirt_sarray_ptr;
pub use crate::jpeglib_h::C2RustUnnamed_0;
pub use crate::jpeglib_h::JCS_YCbCr;
pub use crate::jpeglib_h::JBLOCK;
pub use crate::jpeglib_h::JBLOCKARRAY;
pub use crate::jpeglib_h::JBLOCKROW;
pub use crate::jpeglib_h::JCOEFPTR;
pub use crate::jpeglib_h::JCS_CMYK;
pub use crate::jpeglib_h::JCS_GRAYSCALE;
pub use crate::jpeglib_h::JCS_RGB;
pub use crate::jpeglib_h::JCS_UNKNOWN;
pub use crate::jpeglib_h::JCS_YCCK;
pub use crate::jpeglib_h::JDCT_FLOAT;
pub use crate::jpeglib_h::JDCT_IFAST;
pub use crate::jpeglib_h::JDCT_ISLOW;
pub use crate::jpeglib_h::JDITHER_FS;
pub use crate::jpeglib_h::JDITHER_NONE;
pub use crate::jpeglib_h::JDITHER_ORDERED;
pub use crate::jpeglib_h::JHUFF_TBL;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPIMAGE;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::jpeglib_h::J_DITHER_MODE;
pub use crate::src::jpeg_8c::jcomapi::jpeg_abort;
pub use crate::src::jpeg_8c::jcomapi::jpeg_destroy;
pub use crate::src::jpeg_8c::jdinput::jinit_input_controller;
pub use crate::src::jpeg_8c::jdmarker::jinit_marker_reader;
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
pub use crate::src::jpeg_8c::jmemmgr::jinit_memory_mgr;

/*
 * jdapimin.c
 *
 * Copyright (C) 1994-1998, Thomas G. Lane.
 * Modified 2009 by Guido Vollbeding.
 * This file is part of the Independent JPEG Group's software.
 * For conditions of distribution and use, see the accompanying README file.
 *
 * This file contains application interface code for the decompression half
 * of the JPEG library.  These are the "minimum" API routines that may be
 * needed in either the normal full-decompression case or the
 * transcoding-only case.
 *
 * Most of the routines intended to be called directly by an application
 * are in this file or in jdapistd.c.  But also see jcomapi.c for routines
 * shared by compression and decompression, and jdtrans.c for the transcoding
 * case.
 */
/*
 * Initialization of a JPEG decompression object.
 * The error manager must already be set up (in case memory manager fails).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_CreateDecompress(
    mut cinfo: j_decompress_ptr,
    mut version: i32,
    mut structsize: size_t,
) {
    let mut i: i32 = 0;
    /* Guard against version mismatches between library and caller. */
    (*cinfo).mem = 0 as *mut jpeg_memory_mgr; /* so jpeg_destroy knows mem mgr not called */
    if version != 80 as i32 {
        (*(*cinfo).err).msg_code = JERR_BAD_LIB_VERSION as i32;
        (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = 80 as i32;
        (*(*cinfo).err).msg_parm.i[1 as i32 as usize] = version;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if structsize
        != ::std::mem::size_of::<jpeg_decompress_struct>() as libc::c_ulong
    {
        (*(*cinfo).err).msg_code = JERR_BAD_STRUCT_SIZE as i32;
        (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = ::std::mem::size_of::<
            jpeg_decompress_struct,
        >() as libc::c_ulong as i32;
        (*(*cinfo).err).msg_parm.i[1 as i32 as usize] = structsize as i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* For debugging purposes, we zero the whole master structure.
     * But the application has already set the err pointer, and may have set
     * client_data, so we have to save and restore those fields.
     * Note: if application hasn't set client_data, tools like Purify may
     * complain here.
     */
    let mut err: *mut jpeg_error_mgr = (*cinfo).err; /* ignore Purify complaint here */
    let mut client_data: *mut libc::c_void = (*cinfo).client_data;
    crate::stdlib::memset(
        cinfo as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<jpeg_decompress_struct>() as libc::c_ulong,
    );
    (*cinfo).err = err;
    (*cinfo).client_data = client_data;
    (*cinfo).is_decompressor = 1 as i32;
    /* Initialize a memory manager instance for this object */
    jinit_memory_mgr(
        cinfo as j_common_ptr as *mut jpeg_common_struct,
    );
    /* Zero out pointers to permanent structures. */
    (*cinfo).progress = 0 as *mut jpeg_progress_mgr;
    (*cinfo).src = 0 as *mut jpeg_source_mgr;
    i = 0 as i32;
    while i < 4 as i32 {
        (*cinfo).quant_tbl_ptrs[i as usize] = 0 as *mut JQUANT_TBL;
        i += 1
    }
    i = 0 as i32;
    while i < 4 as i32 {
        (*cinfo).dc_huff_tbl_ptrs[i as usize] = 0 as *mut JHUFF_TBL;
        (*cinfo).ac_huff_tbl_ptrs[i as usize] = 0 as *mut JHUFF_TBL;
        i += 1
    }
    /* Initialize marker processor so application can override methods
     * for COM, APPn markers before calling jpeg_read_header.
     */
    (*cinfo).marker_list = 0 as jpeg_saved_marker_ptr;
    jinit_marker_reader(
        cinfo as *mut jpeg_decompress_struct,
    );
    /* And initialize the overall input controller. */
    jinit_input_controller(
        cinfo as *mut jpeg_decompress_struct,
    );
    /* OK, I'm ready */
    (*cinfo).global_state = 200 as i32;
}
/*
 * Destruction of a JPEG decompression object
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_destroy_decompress(mut cinfo: j_decompress_ptr) {
    jpeg_destroy(
        cinfo as j_common_ptr as *mut jpeg_common_struct,
    );
    /* use common routine */
}
/* Return value is one of: */
/* #define JPEG_SUSPENDED	0    Suspended due to lack of input data */
/* Reached start of new scan */
/* Reached end of image */
/* Completed one iMCU row */
/* Completed last iMCU row of a scan */
/* Precalculate output dimensions for current decompression parameters. */
/* Control saving of COM and APPn markers into marker_list. */
/* Install a special processing method for COM or APPn markers. */
/* Read or write raw DCT coefficients --- useful for lossless transcoding. */
/* If you choose to abort compression or decompression before completing
 * jpeg_finish_(de)compress, then you need to clean up to release memory,
 * temporary files, etc.  You can just call jpeg_destroy_(de)compress
 * if you're done with the JPEG object, but if you want to clean it up and
 * reuse it, call this:
 */
/*
 * Abort processing of a JPEG decompression operation,
 * but don't destroy the object itself.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_abort_decompress(mut cinfo: j_decompress_ptr) {
    jpeg_abort(
        cinfo as j_common_ptr as *mut jpeg_common_struct,
    );
    /* use common routine */
}
/*
 * Set default decompression parameters.
 */

unsafe extern "C" fn default_decompress_parms(mut cinfo: j_decompress_ptr) {
    /* Guess the input colorspace, and set output colorspace accordingly. */
    /* (Wish JPEG committee had provided a real way to specify this...) */
    /* Note application may override our guesses. */
    match (*cinfo).num_components {
        1 => {
            (*cinfo).jpeg_color_space = JCS_GRAYSCALE;
            (*cinfo).out_color_space = JCS_GRAYSCALE
        }
        3 => {
            if (*cinfo).saw_JFIF_marker != 0 {
                (*cinfo).jpeg_color_space = JCS_YCbCr
            /* JFIF implies YCbCr */
            } else if (*cinfo).saw_Adobe_marker != 0 {
                match (*cinfo).Adobe_transform as i32 {
                    0 => (*cinfo).jpeg_color_space = JCS_RGB,
                    1 => (*cinfo).jpeg_color_space = JCS_YCbCr,
                    _ => {
                        (*(*cinfo).err).msg_code =
                            JWRN_ADOBE_XFORM as i32; /* assume it's YCbCr */
                        (*(*cinfo).err).msg_parm.i[0 as i32 as usize] =
                            (*cinfo).Adobe_transform as i32;
                        Some(
                            (*(*cinfo).err)
                                .emit_message
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            cinfo as j_common_ptr,
                            -(1 as i32),
                        );
                        (*cinfo).jpeg_color_space = JCS_YCbCr
                    }
                }
            } else {
                /* Saw no special markers, try to guess from the component IDs */
                let mut cid0: i32 = (*(*cinfo).comp_info.offset(0 as i32 as isize)).component_id; /* assume JFIF w/out marker */
                let mut cid1: i32 = (*(*cinfo).comp_info.offset(1 as i32 as isize)).component_id; /* ASCII 'R', 'G', 'B' */
                let mut cid2: i32 = (*(*cinfo).comp_info.offset(2 as i32 as isize)).component_id;
                if cid0 == 1 as i32 && cid1 == 2 as i32 && cid2 == 3 as i32 {
                    (*cinfo).jpeg_color_space = JCS_YCbCr
                } else if cid0 == 82 as i32 && cid1 == 71 as i32 && cid2 == 66 as i32 {
                    (*cinfo).jpeg_color_space = JCS_RGB
                } else {
                    let mut _mp: *mut i32 = (*(*cinfo).err).msg_parm.i.as_mut_ptr();
                    *_mp.offset(0 as i32 as isize) = cid0;
                    *_mp.offset(1 as i32 as isize) = cid1;
                    *_mp.offset(2 as i32 as isize) = cid2;
                    (*(*cinfo).err).msg_code = JTRC_UNKNOWN_IDS as i32;
                    Some(
                        (*(*cinfo).err)
                            .emit_message
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as j_common_ptr,
                        1 as i32,
                    );
                    (*cinfo).jpeg_color_space = JCS_YCbCr
                    /* assume it's YCbCr */
                }
            }
            /* Always guess RGB is proper output colorspace. */
            (*cinfo).out_color_space = JCS_RGB
        }
        4 => {
            if (*cinfo).saw_Adobe_marker != 0 {
                match (*cinfo).Adobe_transform as i32 {
                    0 => (*cinfo).jpeg_color_space = JCS_CMYK,
                    2 => (*cinfo).jpeg_color_space = JCS_YCCK,
                    _ => {
                        (*(*cinfo).err).msg_code =
                            JWRN_ADOBE_XFORM as i32; /* assume it's YCCK */
                        (*(*cinfo).err).msg_parm.i[0 as i32 as usize] =
                            (*cinfo).Adobe_transform as i32;
                        Some(
                            (*(*cinfo).err)
                                .emit_message
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            cinfo as j_common_ptr,
                            -(1 as i32),
                        );
                        (*cinfo).jpeg_color_space = JCS_YCCK
                    }
                }
            } else {
                /* No special markers, assume straight CMYK. */
                (*cinfo).jpeg_color_space = JCS_CMYK
            }
            (*cinfo).out_color_space = JCS_CMYK
        }
        _ => {
            (*cinfo).jpeg_color_space = JCS_UNKNOWN;
            (*cinfo).out_color_space = JCS_UNKNOWN
        }
    }
    /* Set defaults for other decompression parameters. */
    (*cinfo).scale_num = (*cinfo).block_size as u32; /* 1:1 scaling */
    (*cinfo).scale_denom = (*cinfo).block_size as u32;
    (*cinfo).output_gamma = 1.0f64;
    (*cinfo).buffered_image = 0 as i32;
    (*cinfo).raw_data_out = 0 as i32;
    (*cinfo).dct_method = JDCT_ISLOW;
    (*cinfo).do_fancy_upsampling = 1 as i32;
    (*cinfo).do_block_smoothing = 1 as i32;
    (*cinfo).quantize_colors = 0 as i32;
    /* We set these in case application only sets quantize_colors. */
    (*cinfo).dither_mode = JDITHER_FS;
    (*cinfo).two_pass_quantize = 1 as i32;
    (*cinfo).desired_number_of_colors = 256 as i32;
    (*cinfo).colormap = 0 as JSAMPARRAY;
    /* Initialize for no mode change in buffered-image mode. */
    (*cinfo).enable_1pass_quant = 0 as i32;
    (*cinfo).enable_external_quant = 0 as i32;
    (*cinfo).enable_2pass_quant = 0 as i32;
}
/*
 * Decompression startup: read start of JPEG datastream to see what's there.
 * Need only initialize JPEG object and supply a data source before calling.
 *
 * This routine will read as far as the first SOS marker (ie, actual start of
 * compressed data), and will save all tables and parameters in the JPEG
 * object.  It will also initialize the decompression parameters to default
 * values, and finally return JPEG_HEADER_OK.  On return, the application may
 * adjust the decompression parameters and then call jpeg_start_decompress.
 * (Or, if the application only wanted to determine the image parameters,
 * the data need not be decompressed.  In that case, call jpeg_abort or
 * jpeg_destroy to release any temporary space.)
 * If an abbreviated (tables only) datastream is presented, the routine will
 * return JPEG_HEADER_TABLES_ONLY upon reaching EOI.  The application may then
 * re-use the JPEG object to read the abbreviated image datastream(s).
 * It is unnecessary (but OK) to call jpeg_abort in this case.
 * The JPEG_SUSPENDED return code only occurs if the data source module
 * requests suspension of the decompressor.  In this case the application
 * should load more source data and then re-call jpeg_read_header to resume
 * processing.
 * If a non-suspending data source is used and require_image is TRUE, then the
 * return code need not be inspected since only JPEG_HEADER_OK is possible.
 *
 * This routine is now just a front end to jpeg_consume_input, with some
 * extra error checking.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_read_header(
    mut cinfo: j_decompress_ptr,
    mut require_image: boolean,
) -> i32 {
    let mut retcode: i32 = 0;
    if (*cinfo).global_state != 200 as i32 && (*cinfo).global_state != 201 as i32 {
        (*(*cinfo).err).msg_code = JERR_BAD_STATE as i32;
        (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    retcode = jpeg_consume_input(cinfo);
    match retcode {
        1 => retcode = 1 as i32,
        2 => {
            if require_image != 0 {
                /* Complain if application wanted an image */
                (*(*cinfo).err).msg_code = JERR_NO_IMAGE as i32;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
            }
            /* Reset to start state; it would be safer to require the application to
             * call jpeg_abort, but we can't change it now for compatibility reasons.
             * A side effect is to free any temporary memory (there shouldn't be any).
             */
            jpeg_abort(
                cinfo as j_common_ptr
                    as *mut jpeg_common_struct,
            ); /* sets state = DSTATE_START */
            retcode = 2 as i32
        }
        0 | _ => {}
    }
    return retcode;
}
/*
 * Consume data in advance of what the decompressor requires.
 * This can be called at any time once the decompressor object has
 * been created and a data source has been set up.
 *
 * This routine is essentially a state machine that handles a couple
 * of critical state-transition actions, namely initial setup and
 * transition from header scanning to ready-for-start_decompress.
 * All the actual input is done via the input controller's consume_input
 * method.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_consume_input(mut cinfo: j_decompress_ptr) -> i32 {
    let mut retcode: i32 = 0 as i32;
    let mut current_block_10: u64;
    /* NB: every possible DSTATE value should be listed in this switch */
    match (*cinfo).global_state {
        200 => {
            /* Start-of-datastream actions: reset appropriate modules */
            Some(
                (*(*cinfo).inputctl)
                    .reset_input_controller
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo);
            /* Initialize application's data source module */
            Some(
                (*(*cinfo).src)
                    .init_source
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo);
            (*cinfo).global_state = 201 as i32;
            current_block_10 = 13504760517129887221;
        }
        201 => {
            current_block_10 = 13504760517129887221;
        }
        202 => {
            /* Can't advance past first SOS until start_decompress is called */
            retcode = 1 as i32;
            current_block_10 = 7149356873433890176;
        }
        203 | 204 | 205 | 206 | 207 | 208 | 210 => {
            retcode = Some(
                (*(*cinfo).inputctl)
                    .consume_input
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo);
            current_block_10 = 7149356873433890176;
        }
        _ => {
            (*(*cinfo).err).msg_code = JERR_BAD_STATE as i32;
            (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = (*cinfo).global_state;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
            current_block_10 = 7149356873433890176;
        }
    }
    match current_block_10 {
        13504760517129887221 =>
        /*FALLTHROUGH*/
        {
            retcode = Some(
                (*(*cinfo).inputctl)
                    .consume_input
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo);
            if retcode == 1 as i32 {
                /* Found SOS, prepare to decompress */
                /* Set up default parameters based on header data */
                default_decompress_parms(cinfo);
                /* Set global state: ready for start_decompress */
                (*cinfo).global_state = 202 as i32
            }
        }
        _ => {}
    }
    return retcode;
}
/*
 * Have we finished reading the input file?
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_input_complete(
    mut cinfo: j_decompress_ptr,
) -> boolean {
    /* Check for valid jpeg object */
    if (*cinfo).global_state < 200 as i32 || (*cinfo).global_state > 210 as i32 {
        (*(*cinfo).err).msg_code = JERR_BAD_STATE as i32;
        (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    return (*(*cinfo).inputctl).eoi_reached;
}
/* Replaces jpeg_read_scanlines when reading raw downsampled data. */
/* Additional entry points for buffered-image mode. */
/*
 * Is there more than one scan?
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_has_multiple_scans(
    mut cinfo: j_decompress_ptr,
) -> boolean {
    /* Only valid after jpeg_read_header completes */
    if (*cinfo).global_state < 202 as i32 || (*cinfo).global_state > 210 as i32 {
        (*(*cinfo).err).msg_code = JERR_BAD_STATE as i32;
        (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    return (*(*cinfo).inputctl).has_multiple_scans;
}
/* Method pointers */
/* Limit on memory allocation for this JPEG object.  (Note that this is
 * merely advisory, not a guaranteed maximum; it only affects the space
 * used for virtual-array buffers.)  May be changed by outer application
 * after creating the JPEG object.
 */
/* Maximum allocation request accepted by alloc_large. */
/* Routine signature for application-supplied marker processing methods.
 * Need not pass marker code since it is stored in cinfo->unread_marker.
 */
/* Declarations for routines called by application.
 * The JPP macro hides prototype parameters from compilers that can't cope.
 * Note JPP requires double parentheses.
 */
/* Short forms of external names for systems with brain-damaged linkers.
 * We shorten external names to be unique in the first six letters, which
 * is good enough for all known systems.
 * (If your compiler itself needs names to be unique in less than 15
 * characters, you are out of luck.  Get a better compiler.)
 */
/* NEED_SHORT_EXTERNAL_NAMES */
/* Default error-management setup */
/* Initialization of JPEG compression objects.
 * jpeg_create_compress() and jpeg_create_decompress() are the exported
 * names that applications should call.  These expand to calls on
 * jpeg_CreateCompress and jpeg_CreateDecompress with additional information
 * passed for version mismatch checking.
 * NB: you must set up the error-manager BEFORE calling jpeg_create_xxx.
 */
/* Destruction of JPEG compression objects */
/* Standard data source and destination managers: stdio streams. */
/* Caller is responsible for opening the file before and closing after. */
/* Data source and destination managers: memory buffers. */
/* Default parameter setup for compression */
/* Compression parameter setup aids */
/* Main entry points for compression */
/* Precalculate JPEG dimensions for current compression parameters. */
/* Replaces jpeg_write_scanlines when writing raw downsampled data. */
/* Write a special marker.  See libjpeg.txt concerning safe usage. */
/* Same, but piecemeal. */
/* Alternate compression function: just write an abbreviated table file */
/* Decompression startup: read start of JPEG datastream to see what's there */
/* Return value is one of: */
/* Suspended due to lack of input data */
/* Found valid image datastream */
/* Found valid table-specs-only datastream */
/* If you pass require_image = TRUE (normal case), you need not check for
 * a TABLES_ONLY return code; an abbreviated file will cause an error exit.
 * JPEG_SUSPENDED is only possible if you use a data source module that can
 * give a suspension return (the stdio source module doesn't).
 */
/* Main entry points for decompression */
/*
 * Finish JPEG decompression.
 *
 * This will normally just verify the file trailer and release temp storage.
 *
 * Returns FALSE if suspended.  The return value need be inspected only if
 * a suspending data source is used.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_finish_decompress(
    mut cinfo: j_decompress_ptr,
) -> boolean {
    if ((*cinfo).global_state == 205 as i32 || (*cinfo).global_state == 206 as i32)
        && (*cinfo).buffered_image == 0
    {
        /* Terminate final pass of non-buffered mode */
        if (*cinfo).output_scanline < (*cinfo).output_height {
            (*(*cinfo).err).msg_code = JERR_TOO_LITTLE_DATA as i32;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
        }
        Some(
            (*(*cinfo).master)
                .finish_output_pass
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
        (*cinfo).global_state = 210 as i32
    } else if (*cinfo).global_state == 207 as i32 {
        /* Finishing after a buffered-image operation */
        (*cinfo).global_state = 210 as i32
    } else if (*cinfo).global_state != 210 as i32 {
        /* STOPPING = repeat call after a suspension, anything else is error */
        (*(*cinfo).err).msg_code = JERR_BAD_STATE as i32;
        (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* Read until EOI */
    while (*(*cinfo).inputctl).eoi_reached == 0 {
        if Some(
            (*(*cinfo).inputctl)
                .consume_input
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0 as i32
        {
            return 0 as i32;
        }
        /* Suspend, come back later */
    }
    /* Do final cleanup */
    Some(
        (*(*cinfo).src)
            .term_source
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo);
    /* We can use jpeg_abort to release memory and reset global_state */
    jpeg_abort(
        cinfo as j_common_ptr as *mut jpeg_common_struct,
    );
    return 1 as i32;
}
