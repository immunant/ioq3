use ::libc;

pub use crate::stddef_h::size_t;

pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::INT16;
pub use crate::jmorecfg_h::INT32;
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
pub use crate::src::jpeg_8c::jutils::jzero_far;

pub type my_cquantize_ptr = *mut my_cquantizer;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_cquantizer {
    pub pub_0: jpeg_color_quantizer,
    pub sv_colormap: JSAMPARRAY,
    pub desired: i32,
    pub histogram: hist3d,
    pub needs_zeroed: boolean,
    pub fserrors: FSERRPTR,
    pub on_odd_row: boolean,
    pub error_limiter: *mut i32,
}
/* use 'int' for calculation temps */

pub type FSERRPTR = *mut FSERROR;
/* type for top-level pointer */
/* Declarations for Floyd-Steinberg dithering.
 *
 * Errors are accumulated into the array fserrors[], at a resolution of
 * 1/16th of a pixel count.  The error at a given pixel is propagated
 * to its not-yet-processed neighbors using the standard F-S fractions,
 *		...	(here)	7/16
 *		3/16	5/16	1/16
 * We work left-to-right on even rows, right-to-left on odd rows.
 *
 * We can get away with a single array (holding one row's worth of errors)
 * by using it to store the current row's errors at pixel columns not yet
 * processed, but the next row's errors at columns already processed.  We
 * need only a few extra variables to hold the errors immediately around the
 * current column.  (If we are lucky, those variables are in registers, but
 * even if not, they're probably cheaper to access than array elements are.)
 *
 * The fserrors[] array has (#columns + 2) entries; the extra entry at
 * each end saves us from special-casing the first and last pixels.
 * Each entry is three values long, one value for each color component.
 *
 * Note: on a wide image, we might not have enough room in a PC's near data
 * segment to hold the error array; so it is allocated with alloc_large.
 */

pub type FSERROR = INT16;
/* type for the 2nd-level pointers */

pub type hist3d = *mut hist2d;
/* typedefs for the array */

pub type hist2d = *mut hist1d;
/* for pointers to histogram cells */

pub type hist1d = [histcell; 32];

pub type histcell = UINT16;

pub type boxptr = *mut box_0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct box_0 {
    pub c0min: i32,
    pub c0max: i32,
    pub c1min: i32,
    pub c1max: i32,
    pub c2min: i32,
    pub c2max: i32,
    pub volume: INT32,
    pub colorcount: isize,
}
/* histogram cell; prefer an unsigned type */

pub type histptr = *mut histcell;
/* 16 bits should be enough */

pub type LOCFSERROR = i32;
/*
 * Prescan some rows of pixels.
 * In this module the prescan simply updates the histogram, which has been
 * initialized to zeroes by start_pass.
 * An output_buf parameter is required by the method signature, but no data
 * is actually output (in fact the buffer controller is probably passing a
 * NULL pointer).
 */

unsafe extern "C" fn prescan_quantize(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPARRAY,
    mut _output_buf: JSAMPARRAY,
    mut num_rows: i32,
) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut ptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut histp: histptr = 0 as *mut histcell;
    let mut histogram: hist3d = (*cquantize).histogram;
    let mut row: i32 = 0;
    let mut col: JDIMENSION = 0;
    let mut width: JDIMENSION = (*cinfo).output_width;
    row = 0 as i32;
    while row < num_rows {
        ptr = *input_buf.offset(row as isize);
        col = width;
        while col > 0 as i32 as u32 {
            /* get pixel value and index into the histogram */
            histp = &mut *(*(*histogram
                .offset((*ptr.offset(0 as i32 as isize) as i32 >> 8 as i32 - 5 as i32) as isize))
            .offset((*ptr.offset(1 as i32 as isize) as i32 >> 8 as i32 - 6 as i32) as isize))
            .as_mut_ptr()
            .offset((*ptr.offset(2 as i32 as isize) as i32 >> 8 as i32 - 5 as i32) as isize)
                as *mut histcell;
            /* increment, check for overflow and undo increment if so. */
            *histp = (*histp).wrapping_add(1);
            if *histp as i32 <= 0 as i32 {
                *histp = (*histp).wrapping_sub(1)
            }
            ptr = ptr.offset(3 as i32 as isize);
            col = col.wrapping_sub(1)
        }
        row += 1
    }
}

unsafe extern "C" fn find_biggest_color_pop(mut boxlist: boxptr, mut numboxes: i32) -> boxptr
/* Find the splittable box with the largest color population */
/* Returns NULL if no splittable boxes remain */ {
    let mut boxp: boxptr = 0 as *mut box_0;
    let mut i: i32 = 0;
    let mut maxc: isize = 0 as i32 as isize;
    let mut which: boxptr = 0 as boxptr;
    i = 0 as i32;
    boxp = boxlist;
    while i < numboxes {
        if (*boxp).colorcount > maxc && (*boxp).volume > 0 as i32 as isize {
            which = boxp;
            maxc = (*boxp).colorcount
        }
        i += 1;
        boxp = boxp.offset(1)
    }
    return which;
}

unsafe extern "C" fn find_biggest_volume(mut boxlist: boxptr, mut numboxes: i32) -> boxptr
/* Find the splittable box with the largest (scaled) volume */
/* Returns NULL if no splittable boxes remain */ {
    let mut boxp: boxptr = 0 as *mut box_0;
    let mut i: i32 = 0;
    let mut maxv: INT32 = 0 as i32 as INT32;
    let mut which: boxptr = 0 as boxptr;
    i = 0 as i32;
    boxp = boxlist;
    while i < numboxes {
        if (*boxp).volume > maxv {
            which = boxp;
            maxv = (*boxp).volume
        }
        i += 1;
        boxp = boxp.offset(1)
    }
    return which;
}

unsafe extern "C" fn update_box(mut cinfo: j_decompress_ptr, mut boxp: boxptr)
/* Shrink the min/max bounds of a box to enclose only nonzero elements, */
/* and recompute its volume and population */
{
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut histogram: hist3d = (*cquantize).histogram;
    let mut histp: histptr = 0 as *mut histcell;
    let mut c0: i32 = 0;
    let mut c1: i32 = 0;
    let mut c2: i32 = 0;
    let mut c0min: i32 = 0;
    let mut c0max: i32 = 0;
    let mut c1min: i32 = 0;
    let mut c1max: i32 = 0;
    let mut c2min: i32 = 0;
    let mut c2max: i32 = 0;
    let mut dist0: INT32 = 0;
    let mut dist1: INT32 = 0;
    let mut dist2: INT32 = 0;
    let mut ccount: isize = 0;
    c0min = (*boxp).c0min;
    c0max = (*boxp).c0max;
    c1min = (*boxp).c1min;
    c1max = (*boxp).c1max;
    c2min = (*boxp).c2min;
    c2max = (*boxp).c2max;
    if c0max > c0min {
        c0 = c0min;
        's_50: while c0 <= c0max {
            c1 = c1min;
            while c1 <= c1max {
                histp = &mut *(*(*histogram.offset(c0 as isize)).offset(c1 as isize))
                    .as_mut_ptr()
                    .offset(c2min as isize) as *mut histcell;
                c2 = c2min;
                while c2 <= c2max {
                    let fresh0 = histp;
                    histp = histp.offset(1);
                    if *fresh0 as i32 != 0 as i32 {
                        c0min = c0;
                        (*boxp).c0min = c0min;
                        break 's_50;
                    } else {
                        c2 += 1
                    }
                }
                c1 += 1
            }
            c0 += 1
        }
    }
    if c0max > c0min {
        c0 = c0max;
        's_105: while c0 >= c0min {
            c1 = c1min;
            while c1 <= c1max {
                histp = &mut *(*(*histogram.offset(c0 as isize)).offset(c1 as isize))
                    .as_mut_ptr()
                    .offset(c2min as isize) as *mut histcell;
                c2 = c2min;
                while c2 <= c2max {
                    let fresh1 = histp;
                    histp = histp.offset(1);
                    if *fresh1 as i32 != 0 as i32 {
                        c0max = c0;
                        (*boxp).c0max = c0max;
                        break 's_105;
                    } else {
                        c2 += 1
                    }
                }
                c1 += 1
            }
            c0 -= 1
        }
    }
    if c1max > c1min {
        c1 = c1min;
        's_162: while c1 <= c1max {
            c0 = c0min;
            while c0 <= c0max {
                histp = &mut *(*(*histogram.offset(c0 as isize)).offset(c1 as isize))
                    .as_mut_ptr()
                    .offset(c2min as isize) as *mut histcell;
                c2 = c2min;
                while c2 <= c2max {
                    let fresh2 = histp;
                    histp = histp.offset(1);
                    if *fresh2 as i32 != 0 as i32 {
                        c1min = c1;
                        (*boxp).c1min = c1min;
                        break 's_162;
                    } else {
                        c2 += 1
                    }
                }
                c0 += 1
            }
            c1 += 1
        }
    }
    if c1max > c1min {
        c1 = c1max;
        's_219: while c1 >= c1min {
            c0 = c0min;
            while c0 <= c0max {
                histp = &mut *(*(*histogram.offset(c0 as isize)).offset(c1 as isize))
                    .as_mut_ptr()
                    .offset(c2min as isize) as *mut histcell;
                c2 = c2min;
                while c2 <= c2max {
                    let fresh3 = histp;
                    histp = histp.offset(1);
                    if *fresh3 as i32 != 0 as i32 {
                        c1max = c1;
                        (*boxp).c1max = c1max;
                        break 's_219;
                    } else {
                        c2 += 1
                    }
                }
                c0 += 1
            }
            c1 -= 1
        }
    }
    if c2max > c2min {
        c2 = c2min;
        's_276: while c2 <= c2max {
            c0 = c0min;
            while c0 <= c0max {
                histp = &mut *(*(*histogram.offset(c0 as isize)).offset(c1min as isize))
                    .as_mut_ptr()
                    .offset(c2 as isize) as *mut histcell;
                c1 = c1min;
                while c1 <= c1max {
                    if *histp as i32 != 0 as i32 {
                        c2min = c2;
                        (*boxp).c2min = c2min;
                        break 's_276;
                    } else {
                        c1 += 1;
                        histp = histp.offset(((1 as i32) << 5 as i32) as isize)
                    }
                }
                c0 += 1
            }
            c2 += 1
        }
    }
    if c2max > c2min {
        c2 = c2max;
        's_333: while c2 >= c2min {
            c0 = c0min;
            while c0 <= c0max {
                histp = &mut *(*(*histogram.offset(c0 as isize)).offset(c1min as isize))
                    .as_mut_ptr()
                    .offset(c2 as isize) as *mut histcell;
                c1 = c1min;
                while c1 <= c1max {
                    if *histp as i32 != 0 as i32 {
                        c2max = c2;
                        (*boxp).c2max = c2max;
                        break 's_333;
                    } else {
                        c1 += 1;
                        histp = histp.offset(((1 as i32) << 5 as i32) as isize)
                    }
                }
                c0 += 1
            }
            c2 -= 1
        }
    }
    /* Update box volume.
     * We use 2-norm rather than real volume here; this biases the method
     * against making long narrow boxes, and it has the side benefit that
     * a box is splittable iff norm > 0.
     * Since the differences are expressed in histogram-cell units,
     * we have to shift back to JSAMPLE units to get consistent distances;
     * after which, we scale according to the selected distance scale factors.
     */
    dist0 = ((c0max - c0min << 8 as i32 - 5 as i32) * 2 as i32) as INT32;
    dist1 = ((c1max - c1min << 8 as i32 - 6 as i32) * 3 as i32) as INT32;
    dist2 = ((c2max - c2min << 8 as i32 - 5 as i32) * 1 as i32) as INT32;
    (*boxp).volume = dist0 * dist0 + dist1 * dist1 + dist2 * dist2;
    /* Now scan remaining volume of box and compute population */
    ccount = 0 as i32 as isize;
    c0 = c0min;
    while c0 <= c0max {
        c1 = c1min;
        while c1 <= c1max {
            histp = &mut *(*(*histogram.offset(c0 as isize)).offset(c1 as isize))
                .as_mut_ptr()
                .offset(c2min as isize) as *mut histcell;
            c2 = c2min;
            while c2 <= c2max {
                if *histp as i32 != 0 as i32 {
                    ccount += 1
                }
                c2 += 1;
                histp = histp.offset(1)
            }
            c1 += 1
        }
        c0 += 1
    }
    (*boxp).colorcount = ccount;
}

unsafe extern "C" fn median_cut(
    mut cinfo: j_decompress_ptr,
    mut boxlist: boxptr,
    mut numboxes: i32,
    mut desired_colors: i32,
) -> i32
/* Repeatedly select and split the largest box until we have enough boxes */ {
    let mut n: i32 = 0;
    let mut lb: i32 = 0;
    let mut c0: i32 = 0;
    let mut c1: i32 = 0;
    let mut c2: i32 = 0;
    let mut cmax: i32 = 0;
    let mut b1: boxptr = 0 as *mut box_0;
    let mut b2: boxptr = 0 as *mut box_0;
    while numboxes < desired_colors {
        /* Select box to split.
         * Current algorithm: by population for first half, then by volume.
         */
        if numboxes * 2 as i32 <= desired_colors {
            b1 = find_biggest_color_pop(boxlist, numboxes)
        } else {
            b1 = find_biggest_volume(boxlist, numboxes)
        } /* where new box will go */
        if b1.is_null() {
            break;
        }
        b2 = &mut *boxlist.offset(numboxes as isize) as *mut box_0;
        /* Copy the color bounds to the new box. */
        (*b2).c0max = (*b1).c0max;
        (*b2).c1max = (*b1).c1max;
        (*b2).c2max = (*b1).c2max;
        (*b2).c0min = (*b1).c0min;
        (*b2).c1min = (*b1).c1min;
        (*b2).c2min = (*b1).c2min;
        /* Choose which axis to split the box on.
         * Current algorithm: longest scaled axis.
         * See notes in update_box about scaling distances.
         */
        c0 = ((*b1).c0max - (*b1).c0min << 8 as i32 - 5 as i32) * 2 as i32;
        c1 = ((*b1).c1max - (*b1).c1min << 8 as i32 - 6 as i32) * 3 as i32;
        c2 = ((*b1).c2max - (*b1).c2min << 8 as i32 - 5 as i32) * 1 as i32;
        /* We want to break any ties in favor of green, then red, blue last.
         * This code does the right thing for R,G,B or B,G,R color orders only.
         */
        cmax = c1;
        n = 1 as i32;
        if c0 > cmax {
            cmax = c0;
            n = 0 as i32
        }
        if c2 > cmax {
            n = 2 as i32
        }
        /* Choose split point along selected axis, and update box bounds.
         * Current algorithm: split at halfway point.
         * (Since the box has been shrunk to minimum volume,
         * any split will produce two nonempty subboxes.)
         * Note that lb value is max for lower box, so must be < old max.
         */
        match n {
            0 => {
                lb = ((*b1).c0max + (*b1).c0min) / 2 as i32;
                (*b1).c0max = lb;
                (*b2).c0min = lb + 1 as i32
            }
            1 => {
                lb = ((*b1).c1max + (*b1).c1min) / 2 as i32;
                (*b1).c1max = lb;
                (*b2).c1min = lb + 1 as i32
            }
            2 => {
                lb = ((*b1).c2max + (*b1).c2min) / 2 as i32;
                (*b1).c2max = lb;
                (*b2).c2min = lb + 1 as i32
            }
            _ => {}
        }
        /* Update stats for boxes */
        update_box(cinfo, b1);
        update_box(cinfo, b2);
        numboxes += 1
    }
    return numboxes;
}

unsafe extern "C" fn compute_color(mut cinfo: j_decompress_ptr, mut boxp: boxptr, mut icolor: i32)
/* Compute representative color for a box, put it in colormap[icolor] */
{
    /* Current algorithm: mean weighted by pixels (not colors) */
    /* Note it is important to get the rounding correct! */
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut histogram: hist3d = (*cquantize).histogram;
    let mut histp: histptr = 0 as *mut histcell;
    let mut c0: i32 = 0;
    let mut c1: i32 = 0;
    let mut c2: i32 = 0;
    let mut c0min: i32 = 0;
    let mut c0max: i32 = 0;
    let mut c1min: i32 = 0;
    let mut c1max: i32 = 0;
    let mut c2min: i32 = 0;
    let mut c2max: i32 = 0;
    let mut count: isize = 0;
    let mut total: isize = 0 as i32 as isize;
    let mut c0total: isize = 0 as i32 as isize;
    let mut c1total: isize = 0 as i32 as isize;
    let mut c2total: isize = 0 as i32 as isize;
    c0min = (*boxp).c0min;
    c0max = (*boxp).c0max;
    c1min = (*boxp).c1min;
    c1max = (*boxp).c1max;
    c2min = (*boxp).c2min;
    c2max = (*boxp).c2max;
    c0 = c0min;
    while c0 <= c0max {
        c1 = c1min;
        while c1 <= c1max {
            histp = &mut *(*(*histogram.offset(c0 as isize)).offset(c1 as isize))
                .as_mut_ptr()
                .offset(c2min as isize) as *mut histcell;
            c2 = c2min;
            while c2 <= c2max {
                let fresh4 = histp;
                histp = histp.offset(1);
                count = *fresh4 as isize;
                if count != 0 as i32 as isize {
                    total += count;
                    c0total += ((c0 << 8 as i32 - 5 as i32)
                        + ((1 as i32) << 8 as i32 - 5 as i32 >> 1 as i32))
                        as isize
                        * count;
                    c1total += ((c1 << 8 as i32 - 6 as i32)
                        + ((1 as i32) << 8 as i32 - 6 as i32 >> 1 as i32))
                        as isize
                        * count;
                    c2total += ((c2 << 8 as i32 - 5 as i32)
                        + ((1 as i32) << 8 as i32 - 5 as i32 >> 1 as i32))
                        as isize
                        * count
                }
                c2 += 1
            }
            c1 += 1
        }
        c0 += 1
    }
    *(*(*cinfo).colormap.offset(0 as i32 as isize)).offset(icolor as isize) =
        ((c0total + (total >> 1 as i32)) / total) as JSAMPLE;
    *(*(*cinfo).colormap.offset(1 as i32 as isize)).offset(icolor as isize) =
        ((c1total + (total >> 1 as i32)) / total) as JSAMPLE;
    *(*(*cinfo).colormap.offset(2 as i32 as isize)).offset(icolor as isize) =
        ((c2total + (total >> 1 as i32)) / total) as JSAMPLE;
}

unsafe extern "C" fn select_colors(mut cinfo: j_decompress_ptr, mut desired_colors: i32)
/* Master routine for color selection */
{
    let mut boxlist: boxptr = 0 as *mut box_0;
    let mut numboxes: i32 = 0;
    let mut i: i32 = 0;
    /* Allocate workspace for box list */
    boxlist = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        1 as i32,
        (desired_colors as usize)
            .wrapping_mul(::std::mem::size_of::<box_0>() as usize),
    ) as boxptr;
    /* Initialize one box containing whole space */
    numboxes = 1 as i32;
    (*boxlist.offset(0 as i32 as isize)).c0min = 0 as i32;
    (*boxlist.offset(0 as i32 as isize)).c0max = 255 as i32 >> 8 as i32 - 5 as i32;
    (*boxlist.offset(0 as i32 as isize)).c1min = 0 as i32;
    (*boxlist.offset(0 as i32 as isize)).c1max = 255 as i32 >> 8 as i32 - 6 as i32;
    (*boxlist.offset(0 as i32 as isize)).c2min = 0 as i32;
    (*boxlist.offset(0 as i32 as isize)).c2max = 255 as i32 >> 8 as i32 - 5 as i32;
    /* Shrink it to actually-used volume and set its statistics */
    update_box(cinfo, &mut *boxlist.offset(0 as i32 as isize));
    /* Perform median-cut to produce final box list */
    numboxes = median_cut(cinfo, boxlist, numboxes, desired_colors);
    /* Compute the representative color for each box, fill colormap */
    i = 0 as i32;
    while i < numboxes {
        compute_color(cinfo, &mut *boxlist.offset(i as isize), i);
        i += 1
    }
    (*cinfo).actual_number_of_colors = numboxes;
    (*(*cinfo).err).msg_code = JTRC_QUANT_SELECTED as i32;
    (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = numboxes;
    Some(
        (*(*cinfo).err)
            .emit_message
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo as j_common_ptr, 1 as i32);
}
/*
 * The next three routines implement inverse colormap filling.  They could
 * all be folded into one big routine, but splitting them up this way saves
 * some stack space (the mindist[] and bestdist[] arrays need not coexist)
 * and may allow some compilers to produce better code by registerizing more
 * inner-loop variables.
 */

unsafe extern "C" fn find_nearby_colors(
    mut cinfo: j_decompress_ptr,
    mut minc0: i32,
    mut minc1: i32,
    mut minc2: i32,
    mut colorlist: *mut JSAMPLE,
) -> i32
/* Locate the colormap entries close enough to an update box to be candidates
 * for the nearest entry to some cell(s) in the update box.  The update box
 * is specified by the center coordinates of its first cell.  The number of
 * candidate colormap entries is returned, and their colormap indexes are
 * placed in colorlist[].
 * This routine uses Heckbert's "locally sorted search" criterion to select
 * the colors that need further consideration.
 */ {
    let mut numcolors: i32 = (*cinfo).actual_number_of_colors; /* min distance to colormap entry i */
    let mut maxc0: i32 = 0;
    let mut maxc1: i32 = 0;
    let mut maxc2: i32 = 0;
    let mut centerc0: i32 = 0;
    let mut centerc1: i32 = 0;
    let mut centerc2: i32 = 0;
    let mut i: i32 = 0;
    let mut x: i32 = 0;
    let mut ncolors: i32 = 0;
    let mut minmaxdist: INT32 = 0;
    let mut min_dist: INT32 = 0;
    let mut max_dist: INT32 = 0;
    let mut tdist: INT32 = 0;
    let mut mindist: [INT32; 256] = [0; 256];
    /* Compute true coordinates of update box's upper corner and center.
     * Actually we compute the coordinates of the center of the upper-corner
     * histogram cell, which are the upper bounds of the volume we care about.
     * Note that since ">>" rounds down, the "center" values may be closer to
     * min than to max; hence comparisons to them must be "<=", not "<".
     */
    maxc0 = minc0
        + (((1 as i32) << 8 as i32 - 5 as i32 + (5 as i32 - 3 as i32))
            - ((1 as i32) << 8 as i32 - 5 as i32));
    centerc0 = minc0 + maxc0 >> 1 as i32;
    maxc1 = minc1
        + (((1 as i32) << 8 as i32 - 6 as i32 + (6 as i32 - 3 as i32))
            - ((1 as i32) << 8 as i32 - 6 as i32));
    centerc1 = minc1 + maxc1 >> 1 as i32;
    maxc2 = minc2
        + (((1 as i32) << 8 as i32 - 5 as i32 + (5 as i32 - 3 as i32))
            - ((1 as i32) << 8 as i32 - 5 as i32));
    centerc2 = minc2 + maxc2 >> 1 as i32;
    /* For each color in colormap, find:
     *  1. its minimum squared-distance to any point in the update box
     *     (zero if color is within update box);
     *  2. its maximum squared-distance to any point in the update box.
     * Both of these can be found by considering only the corners of the box.
     * We save the minimum distance for each color in mindist[];
     * only the smallest maximum distance is of interest.
     */
    minmaxdist = 0x7fffffff as isize;
    i = 0 as i32;
    while i < numcolors {
        /* We compute the squared-c0-distance term, then add in the other two. */
        x = *(*(*cinfo).colormap.offset(0 as i32 as isize)).offset(i as isize) as i32;
        if x < minc0 {
            tdist = ((x - minc0) * 2 as i32) as INT32;
            min_dist = tdist * tdist;
            tdist = ((x - maxc0) * 2 as i32) as INT32;
            max_dist = tdist * tdist
        } else if x > maxc0 {
            tdist = ((x - maxc0) * 2 as i32) as INT32;
            min_dist = tdist * tdist;
            tdist = ((x - minc0) * 2 as i32) as INT32;
            max_dist = tdist * tdist
        } else {
            /* within cell range so no contribution to min_dist */
            min_dist = 0 as i32 as INT32;
            if x <= centerc0 {
                tdist = ((x - maxc0) * 2 as i32) as INT32;
                max_dist = tdist * tdist
            } else {
                tdist = ((x - minc0) * 2 as i32) as INT32;
                max_dist = tdist * tdist
            }
        }
        x = *(*(*cinfo).colormap.offset(1 as i32 as isize)).offset(i as isize) as i32;
        if x < minc1 {
            tdist = ((x - minc1) * 3 as i32) as INT32;
            min_dist += tdist * tdist;
            tdist = ((x - maxc1) * 3 as i32) as INT32;
            max_dist += tdist * tdist
        } else if x > maxc1 {
            tdist = ((x - maxc1) * 3 as i32) as INT32;
            min_dist += tdist * tdist;
            tdist = ((x - minc1) * 3 as i32) as INT32;
            max_dist += tdist * tdist
        } else if x <= centerc1 {
            tdist = ((x - maxc1) * 3 as i32) as INT32;
            max_dist += tdist * tdist
        } else {
            tdist = ((x - minc1) * 3 as i32) as INT32;
            max_dist += tdist * tdist
        }
        x = *(*(*cinfo).colormap.offset(2 as i32 as isize)).offset(i as isize) as i32;
        if x < minc2 {
            tdist = ((x - minc2) * 1 as i32) as INT32;
            min_dist += tdist * tdist;
            tdist = ((x - maxc2) * 1 as i32) as INT32;
            max_dist += tdist * tdist
        } else if x > maxc2 {
            tdist = ((x - maxc2) * 1 as i32) as INT32;
            min_dist += tdist * tdist;
            tdist = ((x - minc2) * 1 as i32) as INT32;
            max_dist += tdist * tdist
        } else if x <= centerc2 {
            tdist = ((x - maxc2) * 1 as i32) as INT32;
            max_dist += tdist * tdist
        } else {
            tdist = ((x - minc2) * 1 as i32) as INT32;
            max_dist += tdist * tdist
        }
        /* within cell range so no contribution to min_dist */
        /* within cell range so no contribution to min_dist */
        mindist[i as usize] = min_dist; /* save away the results */
        if max_dist < minmaxdist {
            minmaxdist = max_dist
        }
        i += 1
    }
    /* Now we know that no cell in the update box is more than minmaxdist
     * away from some colormap entry.  Therefore, only colors that are
     * within minmaxdist of some part of the box need be considered.
     */
    ncolors = 0 as i32;
    i = 0 as i32;
    while i < numcolors {
        if mindist[i as usize] <= minmaxdist {
            let fresh5 = ncolors;
            ncolors = ncolors + 1;
            *colorlist.offset(fresh5 as isize) = i as JSAMPLE
        }
        i += 1
    }
    return ncolors;
}

unsafe extern "C" fn find_best_colors(
    mut cinfo: j_decompress_ptr,
    mut minc0: i32,
    mut minc1: i32,
    mut minc2: i32,
    mut numcolors: i32,
    mut colorlist: *mut JSAMPLE,
    mut bestcolor: *mut JSAMPLE,
)
/* Find the closest colormap entry for each cell in the update box,
 * given the list of candidate colors prepared by find_nearby_colors.
 * Return the indexes of the closest entries in the bestcolor[] array.
 * This routine uses Thomas' incremental distance calculation method to
 * find the distance from a colormap entry to successive cells in the box.
 */
{
    let mut ic0: i32 = 0; /* pointer into bestdist[] array */
    let mut ic1: i32 = 0; /* pointer into bestcolor[] array */
    let mut ic2: i32 = 0; /* initial distance values */
    let mut i: i32 = 0; /* current distance in inner loop */
    let mut icolor: i32 = 0; /* distance increments */
    let mut bptr: *mut INT32 = 0 as *mut INT32; /* initial values for increments */
    let mut cptr: *mut JSAMPLE = 0 as *mut JSAMPLE;
    let mut dist0: INT32 = 0;
    let mut dist1: INT32 = 0;
    let mut dist2: INT32 = 0;
    let mut xx0: INT32 = 0;
    let mut xx1: INT32 = 0;
    let mut xx2: INT32 = 0;
    let mut inc0: INT32 = 0;
    let mut inc1: INT32 = 0;
    let mut inc2: INT32 = 0;
    /* This array holds the distance to the nearest-so-far color for each cell */
    let mut bestdist: [INT32; 128] = [0; 128];
    /* Initialize best-distance for each cell of the update box */
    bptr = bestdist.as_mut_ptr();
    i = ((1 as i32) << 5 as i32 - 3 as i32)
        * ((1 as i32) << 6 as i32 - 3 as i32)
        * ((1 as i32) << 5 as i32 - 3 as i32)
        - 1 as i32;
    while i >= 0 as i32 {
        let fresh6 = bptr;
        bptr = bptr.offset(1);
        *fresh6 = 0x7fffffff as isize;
        i -= 1
    }
    /* For each color selected by find_nearby_colors,
     * compute its distance to the center of each cell in the box.
     * If that's less than best-so-far, update best distance and color number.
     */
    /* Nominal steps between cell centers ("x" in Thomas article) */
    i = 0 as i32;
    while i < numcolors {
        icolor = *colorlist.offset(i as isize) as i32;
        /* Compute (square of) distance from minc0/c1/c2 to this color */
        inc0 = ((minc0
            - *(*(*cinfo).colormap.offset(0 as i32 as isize)).offset(icolor as isize) as i32)
            * 2 as i32) as INT32;
        dist0 = inc0 * inc0;
        inc1 = ((minc1
            - *(*(*cinfo).colormap.offset(1 as i32 as isize)).offset(icolor as isize) as i32)
            * 3 as i32) as INT32;
        dist0 += inc1 * inc1;
        inc2 = ((minc2
            - *(*(*cinfo).colormap.offset(2 as i32 as isize)).offset(icolor as isize) as i32)
            * 1 as i32) as INT32;
        dist0 += inc2 * inc2;
        /* Form the initial difference increments */
        inc0 = inc0 * (2 as i32 * (((1 as i32) << 8 as i32 - 5 as i32) * 2 as i32)) as isize
            + (((1 as i32) << 8 as i32 - 5 as i32)
                * 2 as i32
                * (((1 as i32) << 8 as i32 - 5 as i32) * 2 as i32)) as isize;
        inc1 = inc1 * (2 as i32 * (((1 as i32) << 8 as i32 - 6 as i32) * 3 as i32)) as isize
            + (((1 as i32) << 8 as i32 - 6 as i32)
                * 3 as i32
                * (((1 as i32) << 8 as i32 - 6 as i32) * 3 as i32)) as isize;
        inc2 = inc2 * (2 as i32 * (((1 as i32) << 8 as i32 - 5 as i32) * 1 as i32)) as isize
            + (((1 as i32) << 8 as i32 - 5 as i32)
                * 1 as i32
                * (((1 as i32) << 8 as i32 - 5 as i32) * 1 as i32)) as isize;
        /* Now loop over all cells in box, updating distance per Thomas method */
        bptr = bestdist.as_mut_ptr();
        cptr = bestcolor;
        xx0 = inc0;
        ic0 = ((1 as i32) << 5 as i32 - 3 as i32) - 1 as i32;
        while ic0 >= 0 as i32 {
            dist1 = dist0;
            xx1 = inc1;
            ic1 = ((1 as i32) << 6 as i32 - 3 as i32) - 1 as i32;
            while ic1 >= 0 as i32 {
                dist2 = dist1;
                xx2 = inc2;
                ic2 = ((1 as i32) << 5 as i32 - 3 as i32) - 1 as i32;
                while ic2 >= 0 as i32 {
                    if dist2 < *bptr {
                        *bptr = dist2;
                        *cptr = icolor as JSAMPLE
                    }
                    dist2 += xx2;
                    xx2 += (2 as i32
                        * (((1 as i32) << 8 as i32 - 5 as i32) * 1 as i32)
                        * (((1 as i32) << 8 as i32 - 5 as i32) * 1 as i32))
                        as isize;
                    bptr = bptr.offset(1);
                    cptr = cptr.offset(1);
                    ic2 -= 1
                }
                dist1 += xx1;
                xx1 += (2 as i32
                    * (((1 as i32) << 8 as i32 - 6 as i32) * 3 as i32)
                    * (((1 as i32) << 8 as i32 - 6 as i32) * 3 as i32))
                    as isize;
                ic1 -= 1
            }
            dist0 += xx0;
            xx0 += (2 as i32
                * (((1 as i32) << 8 as i32 - 5 as i32) * 2 as i32)
                * (((1 as i32) << 8 as i32 - 5 as i32) * 2 as i32)) as isize;
            ic0 -= 1
        }
        i += 1
    }
}

unsafe extern "C" fn fill_inverse_cmap(
    mut cinfo: j_decompress_ptr,
    mut c0: i32,
    mut c1: i32,
    mut c2: i32,
)
/* Fill the inverse-colormap entries in the update box that contains */
/* histogram cell c0/c1/c2.  (Only that one cell MUST be filled, but */
/* we can fill as many others as we wish.) */
{
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr; /* lower left corner of update box */
    let mut histogram: hist3d = (*cquantize).histogram; /* pointer into bestcolor[] array */
    let mut minc0: i32 = 0; /* pointer into main cache array */
    let mut minc1: i32 = 0;
    let mut minc2: i32 = 0;
    let mut ic0: i32 = 0;
    let mut ic1: i32 = 0;
    let mut ic2: i32 = 0;
    let mut cptr: *mut JSAMPLE = 0 as *mut JSAMPLE;
    let mut cachep: histptr = 0 as *mut histcell;
    /* This array lists the candidate colormap indexes. */
    let mut colorlist: [JSAMPLE; 256] = [0; 256]; /* number of candidate colors */
    let mut numcolors: i32 = 0;
    /* This array holds the actually closest colormap index for each cell. */
    let mut bestcolor: [JSAMPLE; 128] = [0; 128];
    /* Convert cell coordinates to update box ID */
    c0 >>= 5 as i32 - 3 as i32;
    c1 >>= 6 as i32 - 3 as i32;
    c2 >>= 5 as i32 - 3 as i32;
    /* Compute true coordinates of update box's origin corner.
     * Actually we compute the coordinates of the center of the corner
     * histogram cell, which are the lower bounds of the volume we care about.
     */
    minc0 = (c0 << 8 as i32 - 5 as i32 + (5 as i32 - 3 as i32))
        + ((1 as i32) << 8 as i32 - 5 as i32 >> 1 as i32);
    minc1 = (c1 << 8 as i32 - 6 as i32 + (6 as i32 - 3 as i32))
        + ((1 as i32) << 8 as i32 - 6 as i32 >> 1 as i32);
    minc2 = (c2 << 8 as i32 - 5 as i32 + (5 as i32 - 3 as i32))
        + ((1 as i32) << 8 as i32 - 5 as i32 >> 1 as i32);
    /* Determine which colormap entries are close enough to be candidates
     * for the nearest entry to some cell in the update box.
     */
    numcolors = find_nearby_colors(cinfo, minc0, minc1, minc2, colorlist.as_mut_ptr());
    /* Determine the actually nearest colors. */
    find_best_colors(
        cinfo,
        minc0,
        minc1,
        minc2,
        numcolors,
        colorlist.as_mut_ptr(),
        bestcolor.as_mut_ptr(),
    );
    /* Save the best color numbers (plus 1) in the main cache array */
    c0 <<= 5 as i32 - 3 as i32; /* convert ID back to base cell indexes */
    c1 <<= 6 as i32 - 3 as i32;
    c2 <<= 5 as i32 - 3 as i32;
    cptr = bestcolor.as_mut_ptr();
    ic0 = 0 as i32;
    while ic0 < (1 as i32) << 5 as i32 - 3 as i32 {
        ic1 = 0 as i32;
        while ic1 < (1 as i32) << 6 as i32 - 3 as i32 {
            cachep = &mut *(*(*histogram.offset((c0 + ic0) as isize)).offset((c1 + ic1) as isize))
                .as_mut_ptr()
                .offset(c2 as isize) as *mut histcell;
            ic2 = 0 as i32;
            while ic2 < (1 as i32) << 5 as i32 - 3 as i32 {
                let fresh7 = cptr;
                cptr = cptr.offset(1);
                let fresh8 = cachep;
                cachep = cachep.offset(1);
                *fresh8 = (*fresh7 as i32 + 1 as i32) as histcell;
                ic2 += 1
            }
            ic1 += 1
        }
        ic0 += 1
    }
}
/*
 * Map some rows of pixels to the output colormapped representation.
 */

unsafe extern "C" fn pass2_no_dither(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPARRAY,
    mut num_rows: i32,
)
/* This version performs no dithering */
{
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut histogram: hist3d = (*cquantize).histogram;
    let mut inptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut cachep: histptr = 0 as *mut histcell;
    let mut c0: i32 = 0;
    let mut c1: i32 = 0;
    let mut c2: i32 = 0;
    let mut row: i32 = 0;
    let mut col: JDIMENSION = 0;
    let mut width: JDIMENSION = (*cinfo).output_width;
    row = 0 as i32;
    while row < num_rows {
        inptr = *input_buf.offset(row as isize);
        outptr = *output_buf.offset(row as isize);
        col = width;
        while col > 0 as i32 as u32 {
            /* get pixel value and index into the cache */
            let fresh9 = inptr;
            inptr = inptr.offset(1);
            c0 = *fresh9 as i32 >> 8 as i32 - 5 as i32;
            let fresh10 = inptr;
            inptr = inptr.offset(1);
            c1 = *fresh10 as i32 >> 8 as i32 - 6 as i32;
            let fresh11 = inptr;
            inptr = inptr.offset(1);
            c2 = *fresh11 as i32 >> 8 as i32 - 5 as i32;
            cachep = &mut *(*(*histogram.offset(c0 as isize)).offset(c1 as isize))
                .as_mut_ptr()
                .offset(c2 as isize) as *mut histcell;
            /* If we have not seen this color before, find nearest colormap entry */
            /* and update the cache */
            if *cachep as i32 == 0 as i32 {
                fill_inverse_cmap(cinfo, c0, c1, c2);
            }
            /* Now emit the colormap index for this cell */
            let fresh12 = outptr;
            outptr = outptr.offset(1);
            *fresh12 = (*cachep as i32 - 1 as i32) as JSAMPLE;
            col = col.wrapping_sub(1)
        }
        row += 1
    }
}

unsafe extern "C" fn pass2_fs_dither(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPARRAY,
    mut num_rows: i32,
)
/* This version performs Floyd-Steinberg dithering */
{
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr; /* current error or pixel value */
    let mut histogram: hist3d = (*cquantize).histogram; /* error for pixel below cur */
    let mut cur0: LOCFSERROR = 0; /* error for below/prev col */
    let mut cur1: LOCFSERROR = 0; /* => fserrors[] at column before current */
    let mut cur2: LOCFSERROR = 0; /* => current input pixel */
    let mut belowerr0: LOCFSERROR = 0; /* => current output pixel */
    let mut belowerr1: LOCFSERROR = 0; /* +1 or -1 depending on direction */
    let mut belowerr2: LOCFSERROR = 0; /* 3*dir, for advancing inptr & errorptr */
    let mut bpreverr0: LOCFSERROR = 0;
    let mut bpreverr1: LOCFSERROR = 0;
    let mut bpreverr2: LOCFSERROR = 0;
    let mut errorptr: FSERRPTR = 0 as *mut FSERROR;
    let mut inptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut cachep: histptr = 0 as *mut histcell;
    let mut dir: i32 = 0;
    let mut dir3: i32 = 0;
    let mut row: i32 = 0;
    let mut col: JDIMENSION = 0;
    let mut width: JDIMENSION = (*cinfo).output_width;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit;
    let mut error_limit: *mut i32 = (*cquantize).error_limiter;
    let mut colormap0: JSAMPROW = *(*cinfo).colormap.offset(0 as i32 as isize);
    let mut colormap1: JSAMPROW = *(*cinfo).colormap.offset(1 as i32 as isize);
    let mut colormap2: JSAMPROW = *(*cinfo).colormap.offset(2 as i32 as isize);
    row = 0 as i32;
    while row < num_rows {
        inptr = *input_buf.offset(row as isize);
        outptr = *output_buf.offset(row as isize);
        if (*cquantize).on_odd_row != 0 {
            /* work right to left in this row */
            inptr = inptr.offset(
                width
                    .wrapping_sub(1 as i32 as u32)
                    .wrapping_mul(3 as i32 as u32) as isize,
            );
            outptr = outptr.offset(width.wrapping_sub(1 as i32 as u32) as isize);
            dir = -(1 as i32);
            dir3 = -(3 as i32); /* so point to rightmost pixel */
            /* flip for next time */
            errorptr = (*cquantize).fserrors.offset(
                width
                    .wrapping_add(1 as i32 as u32)
                    .wrapping_mul(3 as i32 as u32) as isize,
            ); /* => entry after last column */
            (*cquantize).on_odd_row = 0 as i32
        } else {
            /* work left to right in this row */
            dir = 1 as i32;
            dir3 = 3 as i32;
            /* flip for next time */
            errorptr = (*cquantize).fserrors; /* => entry before first real column */
            (*cquantize).on_odd_row = 1 as i32
        }
        /* Preset error values: no error propagated to first pixel from left */
        cur2 = 0 as i32;
        cur1 = cur2;
        cur0 = cur1;
        /* and no error propagated to row below yet */
        belowerr2 = 0 as i32;
        belowerr1 = belowerr2;
        belowerr0 = belowerr1;
        bpreverr2 = 0 as i32;
        bpreverr1 = bpreverr2;
        bpreverr0 = bpreverr1;
        col = width;
        while col > 0 as i32 as u32 {
            /* curN holds the error propagated from the previous pixel on the
             * current line.  Add the error propagated from the previous line
             * to form the complete error correction term for this pixel, and
             * round the error term (which is expressed * 16) to an integer.
             * RIGHT_SHIFT rounds towards minus infinity, so adding 8 is correct
             * for either sign of the error value.
             * Note: errorptr points to *previous* column's array entry.
             */
            cur0 =
                cur0 + *errorptr.offset((dir3 + 0 as i32) as isize) as i32 + 8 as i32 >> 4 as i32;
            cur1 =
                cur1 + *errorptr.offset((dir3 + 1 as i32) as isize) as i32 + 8 as i32 >> 4 as i32;
            cur2 =
                cur2 + *errorptr.offset((dir3 + 2 as i32) as isize) as i32 + 8 as i32 >> 4 as i32;
            /* advance errorptr to current column */
            cur0 = *error_limit.offset(cur0 as isize);
            cur1 = *error_limit.offset(cur1 as isize);
            cur2 = *error_limit.offset(cur2 as isize);
            cur0 += *inptr.offset(0 as i32 as isize) as i32;
            cur1 += *inptr.offset(1 as i32 as isize) as i32;
            cur2 += *inptr.offset(2 as i32 as isize) as i32;
            cur0 = *range_limit.offset(cur0 as isize) as i32;
            cur1 = *range_limit.offset(cur1 as isize) as i32;
            cur2 = *range_limit.offset(cur2 as isize) as i32;
            cachep = &mut *(*(*histogram.offset((cur0 >> 8 as i32 - 5 as i32) as isize))
                .offset((cur1 >> 8 as i32 - 6 as i32) as isize))
            .as_mut_ptr()
            .offset((cur2 >> 8 as i32 - 5 as i32) as isize) as *mut histcell;
            if *cachep as i32 == 0 as i32 {
                fill_inverse_cmap(
                    cinfo,
                    cur0 >> 8 as i32 - 5 as i32,
                    cur1 >> 8 as i32 - 6 as i32,
                    cur2 >> 8 as i32 - 5 as i32,
                );
            }
            let mut pixcode: i32 = *cachep as i32 - 1 as i32;
            *outptr = pixcode as JSAMPLE;
            cur0 -= *colormap0.offset(pixcode as isize) as i32;
            cur1 -= *colormap1.offset(pixcode as isize) as i32;
            cur2 -= *colormap2.offset(pixcode as isize) as i32;
            let mut bnexterr: LOCFSERROR = 0;
            let mut delta: LOCFSERROR = 0;
            /* Limit the error using transfer function set by init_error_limit.
             * See comments with init_error_limit for rationale.
             */
            /* Form pixel value + error, and range-limit to 0..MAXJSAMPLE.
             * The maximum error is +- MAXJSAMPLE (or less with error limiting);
             * this sets the required size of the range_limit array.
             */
            /* Index into the cache with adjusted pixel value */
            /* If we have not seen this color before, find nearest colormap */
            /* entry and update the cache */
            /* Now emit the colormap index for this cell */
            /* Compute representation error for this pixel */
            /* Compute error fractions to be propagated to adjacent pixels.
             * Add these into the running sums, and simultaneously shift the
             * next-line error sums left by 1 column.
             */
            /* form error * 7 */
            bnexterr = cur0; /* Process component 0 */
            delta = cur0 * 2 as i32; /* form error * 3 */
            cur0 += delta; /* form error * 5 */
            *errorptr.offset(0 as i32 as isize) = (bpreverr0 + cur0) as FSERROR; /* form error * 7 */
            cur0 += delta; /* Process component 1 */
            bpreverr0 = belowerr0 + cur0; /* form error * 3 */
            belowerr0 = bnexterr; /* form error * 5 */
            cur0 += delta; /* form error * 7 */
            bnexterr = cur1; /* Process component 2 */
            delta = cur1 * 2 as i32; /* form error * 3 */
            cur1 += delta; /* form error * 5 */
            *errorptr.offset(1 as i32 as isize) = (bpreverr1 + cur1) as FSERROR;
            cur1 += delta;
            bpreverr1 = belowerr1 + cur1;
            belowerr1 = bnexterr;
            cur1 += delta;
            bnexterr = cur2;
            delta = cur2 * 2 as i32;
            cur2 += delta;
            *errorptr.offset(2 as i32 as isize) = (bpreverr2 + cur2) as FSERROR;
            cur2 += delta;
            bpreverr2 = belowerr2 + cur2;
            belowerr2 = bnexterr;
            cur2 += delta;
            inptr = inptr.offset(dir3 as isize);
            outptr = outptr.offset(dir as isize);
            errorptr = errorptr.offset(dir3 as isize);
            col = col.wrapping_sub(1)
        }
        /* At this point curN contains the 7/16 error value to be propagated
         * to the next pixel on the current line, and all the errors for the
         * next line have been shifted over.  We are therefore ready to move on.
         */
        /* Advance pixel pointers to next column */
        /* Post-loop cleanup: we must unload the final error values into the
         * final fserrors[] entry.  Note we need not unload belowerrN because
         * it is for the dummy column before or after the actual array.
         */
        *errorptr.offset(0 as i32 as isize) = bpreverr0 as FSERROR; /* unload prev errs into array */
        *errorptr.offset(1 as i32 as isize) = bpreverr1 as FSERROR;
        *errorptr.offset(2 as i32 as isize) = bpreverr2 as FSERROR;
        row += 1
    }
}
/*
 * Initialize the error-limiting transfer function (lookup table).
 * The raw F-S error computation can potentially compute error values of up to
 * +- MAXJSAMPLE.  But we want the maximum correction applied to a pixel to be
 * much less, otherwise obviously wrong pixels will be created.  (Typical
 * effects include weird fringes at color-area boundaries, isolated bright
 * pixels in a dark area, etc.)  The standard advice for avoiding this problem
 * is to ensure that the "corners" of the color cube are allocated as output
 * colors; then repeated errors in the same direction cannot cause cascading
 * error buildup.  However, that only prevents the error from getting
 * completely out of hand; Aaron Giles reports that error limiting improves
 * the results even with corner colors allocated.
 * A simple clamping of the error values to about +- MAXJSAMPLE/8 works pretty
 * well, but the smoother transfer function used below is even better.  Thanks
 * to Aaron Giles for this idea.
 */

unsafe extern "C" fn init_error_limit(mut cinfo: j_decompress_ptr)
/* Allocate and fill in the error_limiter table */
{
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr; /* so can index -MAXJSAMPLE .. +MAXJSAMPLE */
    let mut table: *mut i32 = 0 as *mut i32;
    let mut in_0: i32 = 0;
    let mut out: i32 = 0;
    table = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        1 as i32,
        ((255 as i32 * 2 as i32 + 1 as i32) as usize)
            .wrapping_mul(::std::mem::size_of::<i32>() as usize),
    ) as *mut i32;
    table = table.offset(255 as i32 as isize);
    (*cquantize).error_limiter = table;
    /* Map errors 1:1 up to +- MAXJSAMPLE/16 */
    out = 0 as i32;
    in_0 = 0 as i32;
    while in_0 < (255 as i32 + 1 as i32) / 16 as i32 {
        *table.offset(in_0 as isize) = out;
        *table.offset(-in_0 as isize) = -out;
        in_0 += 1;
        out += 1
    }
    /* Map errors 1:2 up to +- 3*MAXJSAMPLE/16 */
    while in_0 < (255 as i32 + 1 as i32) / 16 as i32 * 3 as i32 {
        *table.offset(in_0 as isize) = out;
        *table.offset(-in_0 as isize) = -out;
        in_0 += 1;
        out += if in_0 & 1 as i32 != 0 {
            0 as i32
        } else {
            1 as i32
        }
    }
    /* Clamp the rest to final out value (which is (MAXJSAMPLE+1)/8) */
    while in_0 <= 255 as i32 {
        *table.offset(in_0 as isize) = out;
        *table.offset(-in_0 as isize) = -out;
        in_0 += 1
    }
}
/*
 * Finish up at the end of each pass.
 */

unsafe extern "C" fn finish_pass1(mut cinfo: j_decompress_ptr) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    /* Select the representative colors and fill in cinfo->colormap */
    (*cinfo).colormap = (*cquantize).sv_colormap;
    select_colors(cinfo, (*cquantize).desired);
    /* Force next pass to zero the color index table */
    (*cquantize).needs_zeroed = 1 as i32;
}

unsafe extern "C" fn finish_pass2(mut _cinfo: j_decompress_ptr) {
    /* no work */
}
/*
 * Initialize for each processing pass.
 */

unsafe extern "C" fn start_pass_2_quant(mut cinfo: j_decompress_ptr, mut is_pre_scan: boolean) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut histogram: hist3d = (*cquantize).histogram;
    let mut i: i32 = 0;
    /* Only F-S dithering or no dithering is supported. */
    /* If user asks for ordered dither, give him F-S. */
    if (*cinfo).dither_mode as u32 != JDITHER_NONE as i32 as u32 {
        (*cinfo).dither_mode = JDITHER_FS
    }
    if is_pre_scan != 0 {
        /* Set up method pointers */
        (*cquantize).pub_0.color_quantize = Some(
            prescan_quantize
                as unsafe extern "C" fn(
                    _: j_decompress_ptr,
                    _: JSAMPARRAY,
                    _: JSAMPARRAY,
                    _: i32,
                ) -> (),
        );
        (*cquantize).pub_0.finish_pass =
            Some(finish_pass1 as unsafe extern "C" fn(_: j_decompress_ptr) -> ());
        (*cquantize).needs_zeroed = 1 as i32
    /* Always zero histogram */
    } else {
        /* Set up method pointers */
        if (*cinfo).dither_mode as u32 == JDITHER_FS as i32 as u32 {
            (*cquantize).pub_0.color_quantize = Some(
                pass2_fs_dither
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: JSAMPARRAY,
                        _: JSAMPARRAY,
                        _: i32,
                    ) -> (),
            )
        } else {
            (*cquantize).pub_0.color_quantize = Some(
                pass2_no_dither
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: JSAMPARRAY,
                        _: JSAMPARRAY,
                        _: i32,
                    ) -> (),
            )
        }
        (*cquantize).pub_0.finish_pass =
            Some(finish_pass2 as unsafe extern "C" fn(_: j_decompress_ptr) -> ());
        /* Make sure color count is acceptable */
        i = (*cinfo).actual_number_of_colors;
        if i < 1 as i32 {
            (*(*cinfo).err).msg_code = JERR_QUANT_FEW_COLORS as i32;
            (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = 1 as i32;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        if i > 255 as i32 + 1 as i32 {
            (*(*cinfo).err).msg_code = JERR_QUANT_MANY_COLORS as i32;
            (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = 255 as i32 + 1 as i32;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        if (*cinfo).dither_mode as u32 == JDITHER_FS as i32 as u32 {
            let mut arraysize: size_t = ((*cinfo).output_width.wrapping_add(2 as i32 as u32)
                as usize)
                .wrapping_mul(
                    (3 as i32 as usize)
                        .wrapping_mul(::std::mem::size_of::<FSERROR>() as usize),
                );
            /* Allocate Floyd-Steinberg workspace if we didn't already. */
            if (*cquantize).fserrors.is_null() {
                (*cquantize).fserrors = Some(
                    (*(*cinfo).mem)
                        .alloc_large
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr, 1 as i32, arraysize
                ) as FSERRPTR
            }
            /* Initialize the propagated errors to zero. */
            jzero_far((*cquantize).fserrors as *mut libc::c_void, arraysize);
            /* Make the error-limit table if we didn't already. */
            if (*cquantize).error_limiter.is_null() {
                init_error_limit(cinfo);
            }
            (*cquantize).on_odd_row = 0 as i32
        }
    }
    /* Zero the histogram or inverse color map, if necessary */
    if (*cquantize).needs_zeroed != 0 {
        i = 0 as i32;
        while i < (1 as i32) << 5 as i32 {
            jzero_far(
                *histogram.offset(i as isize) as *mut libc::c_void,
                ((((1 as i32) << 6 as i32) * ((1 as i32) << 5 as i32)) as usize)
                    .wrapping_mul(::std::mem::size_of::<histcell>() as usize),
            );
            i += 1
        }
        (*cquantize).needs_zeroed = 0 as i32
    };
}
/*
 * Switch to a new external colormap between output passes.
 */

unsafe extern "C" fn new_color_map_2_quant(mut cinfo: j_decompress_ptr) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    /* Reset the inverse color map */
    (*cquantize).needs_zeroed = 1 as i32;
}
/*
 * Module initialization routine for 2-pass color quantization.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_2pass_quantizer(mut cinfo: j_decompress_ptr) {
    let mut cquantize: my_cquantize_ptr = 0 as *mut my_cquantizer; /* flag optional arrays not allocated */
    let mut i: i32 = 0;
    cquantize = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        1 as i32,
        ::std::mem::size_of::<my_cquantizer>() as usize,
    ) as my_cquantize_ptr;
    (*cinfo).cquantize = cquantize as *mut jpeg_color_quantizer;
    (*cquantize).pub_0.start_pass =
        Some(start_pass_2_quant as unsafe extern "C" fn(_: j_decompress_ptr, _: boolean) -> ());
    (*cquantize).pub_0.new_color_map =
        Some(new_color_map_2_quant as unsafe extern "C" fn(_: j_decompress_ptr) -> ());
    (*cquantize).fserrors = 0 as FSERRPTR;
    (*cquantize).error_limiter = 0 as *mut i32;
    /* Make sure jdmaster didn't give me a case I can't handle */
    if (*cinfo).out_color_components != 3 as i32 {
        (*(*cinfo).err).msg_code = JERR_NOTIMPL as i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* Allocate the histogram/inverse colormap storage */
    (*cquantize).histogram = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        1 as i32,
        (((1 as i32) << 5 as i32) as usize)
            .wrapping_mul(::std::mem::size_of::<hist2d>() as usize),
    ) as hist3d; /* histogram is garbage now */
    i = 0 as i32;
    while i < (1 as i32) << 5 as i32 {
        let ref mut fresh13 = *(*cquantize).histogram.offset(i as isize);
        *fresh13 = Some(
            (*(*cinfo).mem)
                .alloc_large
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            1 as i32,
            ((((1 as i32) << 6 as i32) * ((1 as i32) << 5 as i32)) as usize)
                .wrapping_mul(::std::mem::size_of::<histcell>() as usize),
        ) as hist2d;
        i += 1
    }
    (*cquantize).needs_zeroed = 1 as i32;
    /* Allocate storage for the completed colormap, if required.
     * We do this now since it is FAR storage and may affect
     * the memory manager's space calculations.
     */
    if (*cinfo).enable_2pass_quant != 0 {
        /* Make sure color count is acceptable */
        let mut desired: i32 = (*cinfo).desired_number_of_colors;
        /* Lower bound on # of colors ... somewhat arbitrary as long as > 0 */
        if desired < 8 as i32 {
            (*(*cinfo).err).msg_code = JERR_QUANT_FEW_COLORS as i32;
            (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = 8 as i32;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        /* Make sure colormap indexes can be represented by JSAMPLEs */
        if desired > 255 as i32 + 1 as i32 {
            (*(*cinfo).err).msg_code = JERR_QUANT_MANY_COLORS as i32;
            (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = 255 as i32 + 1 as i32;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        (*cquantize).sv_colormap = Some(
            (*(*cinfo).mem)
                .alloc_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            1 as i32,
            desired as JDIMENSION,
            3 as i32 as JDIMENSION,
        );
        (*cquantize).desired = desired
    } else {
        (*cquantize).sv_colormap = 0 as JSAMPARRAY
    }
    /* Only F-S dithering or no dithering is supported. */
    /* If user asks for ordered dither, give him F-S. */
    if (*cinfo).dither_mode as u32 != JDITHER_NONE as i32 as u32 {
        (*cinfo).dither_mode = JDITHER_FS
    }
    /* Allocate Floyd-Steinberg workspace if necessary.
     * This isn't really needed until pass 2, but again it is FAR storage.
     * Although we will cope with a later change in dither_mode,
     * we do not promise to honor max_memory_to_use if dither_mode changes.
     */
    if (*cinfo).dither_mode as u32 == JDITHER_FS as i32 as u32 {
        (*cquantize).fserrors = Some(
            (*(*cinfo).mem)
                .alloc_large
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            1 as i32,
            ((*cinfo).output_width.wrapping_add(2 as i32 as u32) as usize).wrapping_mul(
                (3 as i32 as usize)
                    .wrapping_mul(::std::mem::size_of::<FSERROR>() as usize),
            ),
        ) as FSERRPTR;
        /* Might as well create the error-limiting table too. */
        init_error_limit(cinfo);
    };
}
/* QUANT_2PASS_SUPPORTED */
