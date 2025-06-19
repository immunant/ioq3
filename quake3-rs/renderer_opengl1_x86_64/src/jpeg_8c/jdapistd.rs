use ::libc;

/* JERROR_H */

/* Informational/debugging messages */

/* Nonfatal errors (we can keep going, but the data is probably corrupt) */

/* Fatal errors (print message and exit) */

/* Macros to simplify using the error and trace message stuff */

/* The first parameter is either type of cinfo pointer */

/* Zap JMESSAGE macro so that future re-inclusions do nothing by default */

/* JMAKE_ENUM_LIST */
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
pub use crate::src::jpeg_8c::jdmaster::jinit_master_decompress;
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
 * Decompression initialization.
 * jpeg_read_header must be completed before calling this.
 *
 * If a multipass operating mode was selected, this will do all but the
 * last pass, and thus may take a great deal of time.
 *
 * Returns FALSE if suspended.  The return value need be inspected only if
 * a suspending data source is used.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_start_decompress(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
) -> crate::jmorecfg_h::boolean {
    if (*cinfo).global_state == 202 as i32 {
        /* First call: initialize master control, select active modules */
        crate::src::jpeg_8c::jdmaster::jinit_master_decompress(
            cinfo as *mut crate::jpeglib_h::jpeg_decompress_struct,
        );
        if (*cinfo).buffered_image != 0 {
            /* No more work here; expecting jpeg_start_output next */
            (*cinfo).global_state = 207 as i32;
            return 1 as i32;
        }
        (*cinfo).global_state = 203 as i32
    }
    if (*cinfo).global_state == 203 as i32 {
        /* If file has multiple scans, absorb them all into the coef buffer */
        if (*(*cinfo).inputctl).has_multiple_scans != 0 {
            loop {
                let mut retcode: i32 = 0;
                /* Call progress monitor hook if present */
                if !(*cinfo).progress.is_null() {
                    Some(
                        (*(*cinfo).progress)
                            .progress_monitor
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as crate::jpeglib_h::j_common_ptr
                    );
                }
                /* Absorb some more input */
                retcode = Some(
                    (*(*cinfo).inputctl)
                        .consume_input
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo);
                if retcode == 0 as i32 {
                    return 0 as i32;
                }
                if retcode == 2 as i32 {
                    break;
                }
                /* Advance progress counter if appropriate */
                if !(*cinfo).progress.is_null()
                    && (retcode == 3 as i32 || retcode == 1 as i32)
                {
                    (*(*cinfo).progress).pass_counter += 1;
                    if (*(*cinfo).progress).pass_counter >= (*(*cinfo).progress).pass_limit {
                        /* jdmaster underestimated number of scans; ratchet up one scan */
                        (*(*cinfo).progress).pass_limit += (*cinfo).total_iMCU_rows as libc::c_long
                    }
                }
            }
            /* D_MULTISCAN_FILES_SUPPORTED */
        }
        (*cinfo).output_scan_number = (*cinfo).input_scan_number
    } else if (*cinfo).global_state != 204 as i32 {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_STATE as i32;
        (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Perform any dummy output passes, and set up for the final pass */
    return output_pass_setup(cinfo);
}
/*
 * jdapistd.c
 *
 * Copyright (C) 1994-1996, Thomas G. Lane.
 * This file is part of the Independent JPEG Group's software.
 * For conditions of distribution and use, see the accompanying README file.
 *
 * This file contains application interface code for the decompression half
 * of the JPEG library.  These are the "standard" API routines that are
 * used in the normal full-decompression case.  They are not used by a
 * transcoding-only application.  Note that if an application links in
 * jpeg_start_decompress, it will end up linking in the entire decompressor.
 * We thus must separate this file from jdapimin.c to avoid linking the
 * whole decompression library into a transcoder.
 */
/* Forward declarations */
/*
 * Set up for an output pass, and perform any dummy pass(es) needed.
 * Common subroutine for jpeg_start_decompress and jpeg_start_output.
 * Entry: global_state = DSTATE_PRESCAN only if previously suspended.
 * Exit: If done, returns TRUE and sets global_state for proper output mode.
 *       If suspended, returns FALSE and sets global_state = DSTATE_PRESCAN.
 */

unsafe extern "C" fn output_pass_setup(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
) -> crate::jmorecfg_h::boolean {
    if (*cinfo).global_state != 204 as i32 {
        /* First call: do pass setup */
        Some(
            (*(*cinfo).master)
                .prepare_for_output_pass
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
        (*cinfo).output_scanline = 0 as i32 as crate::jmorecfg_h::JDIMENSION;
        (*cinfo).global_state = 204 as i32
    }
    /* Loop over any required dummy passes */
    while (*(*cinfo).master).is_dummy_pass != 0 {
        /* Crank through the dummy pass */
        while (*cinfo).output_scanline < (*cinfo).output_height {
            let mut last_scanline: crate::jmorecfg_h::JDIMENSION = 0;
            /* No progress made, must suspend */
            if !(*cinfo).progress.is_null() {
                (*(*cinfo).progress).pass_counter = (*cinfo).output_scanline as libc::c_long;
                (*(*cinfo).progress).pass_limit = (*cinfo).output_height as libc::c_long;
                Some(
                    (*(*cinfo).progress)
                        .progress_monitor
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            last_scanline = (*cinfo).output_scanline;
            Some(
                (*(*cinfo).main)
                    .process_data
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo,
                0 as *mut libc::c_void as crate::jpeglib_h::JSAMPARRAY,
                &mut (*cinfo).output_scanline,
                0 as i32 as crate::jmorecfg_h::JDIMENSION,
            );
            if (*cinfo).output_scanline == last_scanline {
                return 0 as i32;
            }
        }
        /* Call progress monitor hook if present */
        /* Process some data */
        /* QUANT_2PASS_SUPPORTED */
        Some(
            (*(*cinfo).master)
                .finish_output_pass
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
        Some(
            (*(*cinfo).master)
                .prepare_for_output_pass
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
        (*cinfo).output_scanline = 0 as i32 as crate::jmorecfg_h::JDIMENSION
    }
    /* Finish up dummy pass, and set up for another one */
    /* Ready for application to drive output pass through
     * jpeg_read_scanlines or jpeg_read_raw_data.
     */
    (*cinfo).global_state = if (*cinfo).raw_data_out != 0 {
        206 as i32
    } else {
        205 as i32
    };
    return 1 as i32;
}
/*
 * Read some scanlines of data from the JPEG decompressor.
 *
 * The return value will be the number of lines actually read.
 * This may be less than the number requested in several cases,
 * including bottom of image, data source suspension, and operating
 * modes that emit multiple scanlines at a time.
 *
 * Note: we warn about excess calls to jpeg_read_scanlines() since
 * this likely signals an application programmer error.  However,
 * an oversize buffer (max_lines > scanlines remaining) is not an error.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_read_scanlines(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut scanlines: crate::jpeglib_h::JSAMPARRAY,
    mut max_lines: crate::jmorecfg_h::JDIMENSION,
) -> crate::jmorecfg_h::JDIMENSION {
    let mut row_ctr: crate::jmorecfg_h::JDIMENSION = 0;
    if (*cinfo).global_state != 205 as i32 {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_STATE as i32;
        (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    if (*cinfo).output_scanline >= (*cinfo).output_height {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JWRN_TOO_MUCH_DATA as i32;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            -(1 as i32),
        );
        return 0 as i32 as crate::jmorecfg_h::JDIMENSION;
    }
    /* Call progress monitor hook if present */
    if !(*cinfo).progress.is_null() {
        (*(*cinfo).progress).pass_counter = (*cinfo).output_scanline as libc::c_long;
        (*(*cinfo).progress).pass_limit = (*cinfo).output_height as libc::c_long;
        Some(
            (*(*cinfo).progress)
                .progress_monitor
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Process some data */
    row_ctr = 0 as i32 as crate::jmorecfg_h::JDIMENSION;
    Some(
        (*(*cinfo).main)
            .process_data
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo, scanlines, &mut row_ctr, max_lines);
    (*cinfo).output_scanline = ((*cinfo).output_scanline as u32).wrapping_add(row_ctr)
        as crate::jmorecfg_h::JDIMENSION
        as crate::jmorecfg_h::JDIMENSION;
    return row_ctr;
}
/* Replaces jpeg_read_scanlines when reading raw downsampled data. */
/*
 * Alternate entry point to read raw data.
 * Processes exactly one iMCU row per call, unless suspended.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_read_raw_data(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut data: crate::jpeglib_h::JSAMPIMAGE,
    mut max_lines: crate::jmorecfg_h::JDIMENSION,
) -> crate::jmorecfg_h::JDIMENSION {
    let mut lines_per_iMCU_row: crate::jmorecfg_h::JDIMENSION = 0;
    if (*cinfo).global_state != 206 as i32 {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_STATE as i32;
        (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    if (*cinfo).output_scanline >= (*cinfo).output_height {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JWRN_TOO_MUCH_DATA as i32;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            -(1 as i32),
        );
        return 0 as i32 as crate::jmorecfg_h::JDIMENSION;
    }
    /* Call progress monitor hook if present */
    if !(*cinfo).progress.is_null() {
        (*(*cinfo).progress).pass_counter = (*cinfo).output_scanline as libc::c_long;
        (*(*cinfo).progress).pass_limit = (*cinfo).output_height as libc::c_long;
        Some(
            (*(*cinfo).progress)
                .progress_monitor
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Verify that at least one iMCU row can be returned. */
    lines_per_iMCU_row = ((*cinfo).max_v_samp_factor * (*cinfo).min_DCT_v_scaled_size)
        as crate::jmorecfg_h::JDIMENSION;
    if max_lines < lines_per_iMCU_row {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BUFFER_SIZE as i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Decompress directly into user's buffer. */
    if Some(
        (*(*cinfo).coef)
            .decompress_data
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo, data)
        == 0
    {
        return 0 as i32 as crate::jmorecfg_h::JDIMENSION;
    } /* suspension forced, can do nothing more */
    /* OK, we processed one iMCU row. */
    (*cinfo).output_scanline =
        ((*cinfo).output_scanline as u32).wrapping_add(lines_per_iMCU_row)
            as crate::jmorecfg_h::JDIMENSION as crate::jmorecfg_h::JDIMENSION;
    return lines_per_iMCU_row;
}
/* Additional entry points for buffered-image mode. */
/*
 * Initialize for an output pass in buffered-image mode.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_start_output(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut scan_number: i32,
) -> crate::jmorecfg_h::boolean {
    if (*cinfo).global_state != 207 as i32 && (*cinfo).global_state != 204 as i32 {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_STATE as i32;
        (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Limit scan number to valid range */
    if scan_number <= 0 as i32 {
        scan_number = 1 as i32
    }
    if (*(*cinfo).inputctl).eoi_reached != 0 && scan_number > (*cinfo).input_scan_number {
        scan_number = (*cinfo).input_scan_number
    }
    (*cinfo).output_scan_number = scan_number;
    /* Perform any dummy output passes, and set up for the real pass */
    return output_pass_setup(cinfo);
}
/* Additional entry points for buffered-image mode. */
/*
 * Finish up after an output pass in buffered-image mode.
 *
 * Returns FALSE if suspended.  The return value need be inspected only if
 * a suspending data source is used.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_finish_output(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
) -> crate::jmorecfg_h::boolean {
    if ((*cinfo).global_state == 205 as i32 || (*cinfo).global_state == 206 as i32)
        && (*cinfo).buffered_image != 0
    {
        /* Terminate this pass. */
        /* We do not require the whole pass to have been completed. */
        Some(
            (*(*cinfo).master)
                .finish_output_pass
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
        (*cinfo).global_state = 208 as i32
    } else if (*cinfo).global_state != 208 as i32 {
        /* BUFPOST = repeat call after a suspension, anything else is error */
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_STATE as i32;
        (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = (*cinfo).global_state;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Read markers looking for SOS or EOI */
    while (*cinfo).input_scan_number <= (*cinfo).output_scan_number
        && (*(*cinfo).inputctl).eoi_reached == 0
    {
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
    (*cinfo).global_state = 207 as i32;
    return 1 as i32;
}
/* D_MULTISCAN_FILES_SUPPORTED */
