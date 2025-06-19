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

pub use crate::jdct_h::float_DCT_method_ptr;
pub use crate::jdct_h::forward_DCT_method_ptr;
pub use crate::jdct_h::DCTELEM;
pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::INT16;
pub use crate::jmorecfg_h::INT32;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpegint_h::forward_DCT_ptr;
pub use crate::jpegint_h::jpeg_c_coef_controller;
pub use crate::jpegint_h::jpeg_c_main_controller;
pub use crate::jpegint_h::jpeg_c_prep_controller;
pub use crate::jpegint_h::jpeg_color_converter;
pub use crate::jpegint_h::jpeg_comp_master;
pub use crate::jpegint_h::jpeg_downsampler;
pub use crate::jpegint_h::jpeg_entropy_encoder;
pub use crate::jpegint_h::jpeg_forward_dct;
pub use crate::jpegint_h::jpeg_marker_writer;
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
pub use crate::jpegint_h::J_BUF_MODE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_compress_ptr;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_compress_struct;
pub use crate::jpeglib_h::jpeg_destination_mgr;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_scan_info;
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
pub use crate::jpeglib_h::JHUFF_TBL;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPIMAGE;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
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
pub use crate::src::jpeg_8c::jfdctflt::jpeg_fdct_float;
pub use crate::src::jpeg_8c::jfdctfst::jpeg_fdct_ifast;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_10x10;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_10x5;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_11x11;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_12x12;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_12x6;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_13x13;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_14x14;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_14x7;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_15x15;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_16x16;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_16x8;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_1x1;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_1x2;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_2x1;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_2x2;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_2x4;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_3x3;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_3x6;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_4x2;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_4x4;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_4x8;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_5x10;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_5x5;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_6x12;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_6x3;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_6x6;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_7x14;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_7x7;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_8x16;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_8x4;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_9x9;
pub use crate::src::jpeg_8c::jfdctint::jpeg_fdct_islow;

pub type my_fdct_ptr = *mut my_fdct_controller;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_fdct_controller {
    pub pub_0: jpeg_forward_dct,
    pub do_dct: [forward_DCT_method_ptr; 10],
    pub divisors: [*mut DCTELEM; 4],
    pub do_float_dct: [float_DCT_method_ptr; 10],
    pub float_divisors: [*mut f32; 4],
}
/* The current scaled-DCT routines require ISLOW-style divisor tables,
 * so be sure to compile that code if either ISLOW or SCALING is requested.
 */
/*
 * Perform forward DCT on one or more blocks of a component.
 *
 * The input samples are taken from the sample_data[] array starting at
 * position start_row/start_col, and moving to the right for any additional
 * blocks. The quantized coefficients are returned in coef_blocks[].
 */

unsafe extern "C" fn forward_DCT(
    mut cinfo: j_compress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut sample_data: JSAMPARRAY,
    mut coef_blocks: JBLOCKROW,
    mut start_row: JDIMENSION,
    mut start_col: JDIMENSION,
    mut num_blocks: JDIMENSION,
)
/* This version is used for integer DCT implementations. */
{
    /* This routine is heavily used, so it's worth coding it tightly. */
    let mut fdct: my_fdct_ptr = (*cinfo).fdct as my_fdct_ptr; /* work area for FDCT subroutine */
    let mut do_dct: forward_DCT_method_ptr =
        (*fdct).do_dct[(*compptr).component_index as usize]; /* fold in the vertical offset once */
    let mut divisors: *mut DCTELEM =
        (*fdct).divisors[(*compptr).quant_tbl_no as usize];
    let mut workspace: [DCTELEM; 64] = [0; 64];
    let mut bi: JDIMENSION = 0;
    sample_data = sample_data.offset(start_row as isize);
    bi = 0 as i32 as JDIMENSION;
    while bi < num_blocks {
        /* Perform the DCT */
        Some(do_dct.expect("non-null function pointer")).expect("non-null function pointer")(
            workspace.as_mut_ptr(),
            sample_data,
            start_col,
        );
        /* Quantize/descale the coefficients, and store into coef_blocks[] */
        let mut temp: DCTELEM = 0;
        let mut qval: DCTELEM = 0;
        let mut i: i32 = 0;
        let mut output_ptr: JCOEFPTR =
            (*coef_blocks.offset(bi as isize)).as_mut_ptr();
        i = 0 as i32;
        while i < 64 as i32 {
            qval = *divisors.offset(i as isize);
            temp = workspace[i as usize];
            /* Divide the coefficient value by qval, ensuring proper rounding.
             * Since C does not specify the direction of rounding for negative
             * quotients, we have to force the dividend positive for portability.
             *
             * In most files, at least half of the output values will be zero
             * (at default quantization settings, more like three-quarters...)
             * so we should ensure that this case is fast.  On many machines,
             * a comparison is enough cheaper than a divide to make a special test
             * a win.  Since both inputs will be nonnegative, we need only test
             * for a < b to discover whether a/b is 0.
             * If your machine's division is fast enough, define FAST_DIVIDE.
             */
            if temp < 0 as i32 {
                temp = -temp; /* for rounding */
                temp += qval >> 1 as i32; /* for rounding */
                if temp >= qval {
                    temp /= qval
                } else {
                    temp = 0 as i32
                }
                temp = -temp
            } else {
                temp += qval >> 1 as i32;
                if temp >= qval {
                    temp /= qval
                } else {
                    temp = 0 as i32
                }
            }
            *output_ptr.offset(i as isize) = temp as JCOEF;
            i += 1
        }
        bi = bi.wrapping_add(1);
        start_col = (start_col as u32).wrapping_add((*compptr).DCT_h_scaled_size as u32)
            as JDIMENSION
    }
}

unsafe extern "C" fn forward_DCT_float(
    mut cinfo: j_compress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut sample_data: JSAMPARRAY,
    mut coef_blocks: JBLOCKROW,
    mut start_row: JDIMENSION,
    mut start_col: JDIMENSION,
    mut num_blocks: JDIMENSION,
)
/* This version is used for floating-point DCT implementations. */
{
    /* This routine is heavily used, so it's worth coding it tightly. */
    let mut fdct: my_fdct_ptr = (*cinfo).fdct as my_fdct_ptr; /* work area for FDCT subroutine */
    let mut do_dct: float_DCT_method_ptr =
        (*fdct).do_float_dct[(*compptr).component_index as usize]; /* fold in the vertical offset once */
    let mut divisors: *mut f32 = (*fdct).float_divisors[(*compptr).quant_tbl_no as usize];
    let mut workspace: [f32; 64] = [0.; 64];
    let mut bi: JDIMENSION = 0;
    sample_data = sample_data.offset(start_row as isize);
    bi = 0 as i32 as JDIMENSION;
    while bi < num_blocks {
        /* Perform the DCT */
        Some(do_dct.expect("non-null function pointer")).expect("non-null function pointer")(
            workspace.as_mut_ptr(),
            sample_data,
            start_col,
        );
        /* Quantize/descale the coefficients, and store into coef_blocks[] */
        let mut temp: f32 = 0.;
        let mut i: i32 = 0;
        let mut output_ptr: JCOEFPTR =
            (*coef_blocks.offset(bi as isize)).as_mut_ptr();
        i = 0 as i32;
        while i < 64 as i32 {
            /* Apply the quantization and scaling factor */
            temp = workspace[i as usize] * *divisors.offset(i as isize);
            /* Round to nearest integer.
             * Since C does not specify the direction of rounding for negative
             * quotients, we have to force the dividend positive for portability.
             * The maximum coefficient size is +-16K (for 12-bit data), so this
             * code should work for either 16-bit or 32-bit ints.
             */
            *output_ptr.offset(i as isize) =
                ((temp + 16384.5f64 as f32) as i32 - 16384 as i32) as JCOEF;
            i += 1
        }
        bi = bi.wrapping_add(1);
        start_col = (start_col as u32).wrapping_add((*compptr).DCT_h_scaled_size as u32)
            as JDIMENSION
    }
}
/* DCT_FLOAT_SUPPORTED */
/*
 * Initialize for a processing pass.
 * Verify that all referenced Q-tables are present, and set up
 * the divisor table for each one.
 * In the current implementation, DCT of all components is done during
 * the first pass, even if only some components will be output in the
 * first scan.  Hence all components should be examined here.
 */

unsafe extern "C" fn start_pass_fdctmgr(mut cinfo: j_compress_ptr) {
    let mut fdct: my_fdct_ptr = (*cinfo).fdct as my_fdct_ptr;
    let mut ci: i32 = 0;
    let mut qtblno: i32 = 0;
    let mut i: i32 = 0;
    let mut compptr: *mut jpeg_component_info =
        0 as *mut jpeg_component_info;
    let mut method: i32 = 0 as i32;
    let mut qtbl: *mut JQUANT_TBL = 0 as *mut JQUANT_TBL;
    let mut dtbl: *mut DCTELEM = 0 as *mut DCTELEM;
    ci = 0 as i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        /* Select the proper DCT routine for this component's scaling */
        match ((*compptr).DCT_h_scaled_size << 8 as i32) + (*compptr).DCT_v_scaled_size {
            257 => {
                (*fdct).do_dct[ci as usize] = Some(
                    jpeg_fdct_1x1
                        as unsafe extern "C" fn(
                            _: *mut DCTELEM,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            514 => {
                (*fdct).do_dct[ci as usize] = Some(
                    jpeg_fdct_2x2
                        as unsafe extern "C" fn(
                            _: *mut DCTELEM,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            771 => {
                (*fdct).do_dct[ci as usize] = Some(
                    jpeg_fdct_3x3
                        as unsafe extern "C" fn(
                            _: *mut DCTELEM,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            1028 => {
                (*fdct).do_dct[ci as usize] = Some(
                    jpeg_fdct_4x4
                        as unsafe extern "C" fn(
                            _: *mut DCTELEM,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            1285 => {
                (*fdct).do_dct[ci as usize] = Some(
                    jpeg_fdct_5x5
                        as unsafe extern "C" fn(
                            _: *mut DCTELEM,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            1542 => {
                (*fdct).do_dct[ci as usize] = Some(
                    jpeg_fdct_6x6
                        as unsafe extern "C" fn(
                            _: *mut DCTELEM,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            1799 => {
                (*fdct).do_dct[ci as usize] = Some(
                    jpeg_fdct_7x7
                        as unsafe extern "C" fn(
                            _: *mut DCTELEM,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            2313 => {
                (*fdct).do_dct[ci as usize] = Some(
                    jpeg_fdct_9x9
                        as unsafe extern "C" fn(
                            _: *mut DCTELEM,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            2570 => {
                (*fdct).do_dct[ci as usize] = Some(
                    jpeg_fdct_10x10
                        as unsafe extern "C" fn(
                            _: *mut DCTELEM,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            2827 => {
                (*fdct).do_dct[ci as usize] = Some(
                    jpeg_fdct_11x11
                        as unsafe extern "C" fn(
                            _: *mut DCTELEM,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            3084 => {
                (*fdct).do_dct[ci as usize] = Some(
                    jpeg_fdct_12x12
                        as unsafe extern "C" fn(
                            _: *mut DCTELEM,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            3341 => {
                (*fdct).do_dct[ci as usize] = Some(
                    jpeg_fdct_13x13
                        as unsafe extern "C" fn(
                            _: *mut DCTELEM,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            3598 => {
                (*fdct).do_dct[ci as usize] = Some(
                    jpeg_fdct_14x14
                        as unsafe extern "C" fn(
                            _: *mut DCTELEM,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            3855 => {
                (*fdct).do_dct[ci as usize] = Some(
                    jpeg_fdct_15x15
                        as unsafe extern "C" fn(
                            _: *mut DCTELEM,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            4112 => {
                (*fdct).do_dct[ci as usize] = Some(
                    jpeg_fdct_16x16
                        as unsafe extern "C" fn(
                            _: *mut DCTELEM,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            4104 => {
                (*fdct).do_dct[ci as usize] = Some(
                    jpeg_fdct_16x8
                        as unsafe extern "C" fn(
                            _: *mut DCTELEM,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            3591 => {
                (*fdct).do_dct[ci as usize] = Some(
                    jpeg_fdct_14x7
                        as unsafe extern "C" fn(
                            _: *mut DCTELEM,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            3078 => {
                (*fdct).do_dct[ci as usize] = Some(
                    jpeg_fdct_12x6
                        as unsafe extern "C" fn(
                            _: *mut DCTELEM,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            2565 => {
                (*fdct).do_dct[ci as usize] = Some(
                    jpeg_fdct_10x5
                        as unsafe extern "C" fn(
                            _: *mut DCTELEM,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            2052 => {
                (*fdct).do_dct[ci as usize] = Some(
                    jpeg_fdct_8x4
                        as unsafe extern "C" fn(
                            _: *mut DCTELEM,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            1539 => {
                (*fdct).do_dct[ci as usize] = Some(
                    jpeg_fdct_6x3
                        as unsafe extern "C" fn(
                            _: *mut DCTELEM,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            1026 => {
                (*fdct).do_dct[ci as usize] = Some(
                    jpeg_fdct_4x2
                        as unsafe extern "C" fn(
                            _: *mut DCTELEM,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            513 => {
                (*fdct).do_dct[ci as usize] = Some(
                    jpeg_fdct_2x1
                        as unsafe extern "C" fn(
                            _: *mut DCTELEM,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            2064 => {
                (*fdct).do_dct[ci as usize] = Some(
                    jpeg_fdct_8x16
                        as unsafe extern "C" fn(
                            _: *mut DCTELEM,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            1806 => {
                (*fdct).do_dct[ci as usize] = Some(
                    jpeg_fdct_7x14
                        as unsafe extern "C" fn(
                            _: *mut DCTELEM,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            1548 => {
                (*fdct).do_dct[ci as usize] = Some(
                    jpeg_fdct_6x12
                        as unsafe extern "C" fn(
                            _: *mut DCTELEM,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            1290 => {
                (*fdct).do_dct[ci as usize] = Some(
                    jpeg_fdct_5x10
                        as unsafe extern "C" fn(
                            _: *mut DCTELEM,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            1032 => {
                (*fdct).do_dct[ci as usize] = Some(
                    jpeg_fdct_4x8
                        as unsafe extern "C" fn(
                            _: *mut DCTELEM,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            774 => {
                (*fdct).do_dct[ci as usize] = Some(
                    jpeg_fdct_3x6
                        as unsafe extern "C" fn(
                            _: *mut DCTELEM,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            516 => {
                (*fdct).do_dct[ci as usize] = Some(
                    jpeg_fdct_2x4
                        as unsafe extern "C" fn(
                            _: *mut DCTELEM,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            258 => {
                (*fdct).do_dct[ci as usize] = Some(
                    jpeg_fdct_1x2
                        as unsafe extern "C" fn(
                            _: *mut DCTELEM,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jfdctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            2056 => match (*cinfo).dct_method as u32 {
                0 => {
                    (*fdct).do_dct[ci as usize] = Some(
                        jpeg_fdct_islow
                            as unsafe extern "C" fn(
                                _: *mut DCTELEM,
                                _: JSAMPARRAY,
                                _: JDIMENSION,
                            ) -> (),
                    );
                    method = JDCT_ISLOW as i32
                }
                1 => {
                    (*fdct).do_dct[ci as usize] = Some(
                        jpeg_fdct_ifast
                            as unsafe extern "C" fn(
                                _: *mut DCTELEM,
                                _: JSAMPARRAY,
                                _: JDIMENSION,
                            ) -> (),
                    );
                    method = JDCT_IFAST as i32
                }
                2 => {
                    (*fdct).do_float_dct[ci as usize] = Some(
                        jpeg_fdct_float
                            as unsafe extern "C" fn(
                                _: *mut f32,
                                _: JSAMPARRAY,
                                _: JDIMENSION,
                            ) -> (),
                    );
                    method = JDCT_FLOAT as i32
                }
                _ => {
                    (*(*cinfo).err).msg_code =
                        JERR_NOT_COMPILED as i32;
                    Some(
                        (*(*cinfo).err)
                            .error_exit
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as j_common_ptr
                    );
                }
            },
            _ => {
                (*(*cinfo).err).msg_code = JERR_BAD_DCTSIZE as i32;
                (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = (*compptr).DCT_h_scaled_size;
                (*(*cinfo).err).msg_parm.i[1 as i32 as usize] = (*compptr).DCT_v_scaled_size;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
            }
        }
        qtblno = (*compptr).quant_tbl_no;
        /* Make sure specified quantization table is present */
        if qtblno < 0 as i32
            || qtblno >= 4 as i32
            || (*cinfo).quant_tbl_ptrs[qtblno as usize].is_null()
        {
            (*(*cinfo).err).msg_code = JERR_NO_QUANT_TABLE as i32;
            (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = qtblno;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
        }
        qtbl = (*cinfo).quant_tbl_ptrs[qtblno as usize];
        /* Compute divisors for this quant table */
        /* We may do this more than once for same table, but it's not a big deal */
        match method {
            0 => {
                /* For LL&M IDCT method, divisors are equal to raw quantization
                 * coefficients multiplied by 8 (to counteract scaling).
                 */
                if (*fdct).divisors[qtblno as usize].is_null() {
                    (*fdct).divisors[qtblno as usize] = Some(
                        (*(*cinfo).mem)
                            .alloc_small
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as j_common_ptr,
                        1 as i32,
                        (64 as i32 as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<DCTELEM>() as libc::c_ulong
                            ),
                    )
                        as *mut DCTELEM
                }
                dtbl = (*fdct).divisors[qtblno as usize];
                i = 0 as i32;
                while i < 64 as i32 {
                    *dtbl.offset(i as isize) =
                        ((*qtbl).quantval[i as usize] as DCTELEM) << 3 as i32;
                    i += 1
                }
                (*fdct).pub_0.forward_DCT[ci as usize] = Some(
                    forward_DCT
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: *mut jpeg_component_info,
                            _: JSAMPARRAY,
                            _: JBLOCKROW,
                            _: JDIMENSION,
                            _: JDIMENSION,
                            _: JDIMENSION,
                        ) -> (),
                )
            }
            1 => {
                /* For AA&N IDCT method, divisors are equal to quantization
                 * coefficients scaled by scalefactor[row]*scalefactor[col], where
                 *   scalefactor[0] = 1
                 *   scalefactor[k] = cos(k*PI/16) * sqrt(2)    for k=1..7
                 * We apply a further scale factor of 8.
                 */
                static mut aanscales: [INT16; 64] = [
                    16384 as i32 as INT16,
                    22725 as i32 as INT16,
                    21407 as i32 as INT16,
                    19266 as i32 as INT16,
                    16384 as i32 as INT16,
                    12873 as i32 as INT16,
                    8867 as i32 as INT16,
                    4520 as i32 as INT16,
                    22725 as i32 as INT16,
                    31521 as i32 as INT16,
                    29692 as i32 as INT16,
                    26722 as i32 as INT16,
                    22725 as i32 as INT16,
                    17855 as i32 as INT16,
                    12299 as i32 as INT16,
                    6270 as i32 as INT16,
                    21407 as i32 as INT16,
                    29692 as i32 as INT16,
                    27969 as i32 as INT16,
                    25172 as i32 as INT16,
                    21407 as i32 as INT16,
                    16819 as i32 as INT16,
                    11585 as i32 as INT16,
                    5906 as i32 as INT16,
                    19266 as i32 as INT16,
                    26722 as i32 as INT16,
                    25172 as i32 as INT16,
                    22654 as i32 as INT16,
                    19266 as i32 as INT16,
                    15137 as i32 as INT16,
                    10426 as i32 as INT16,
                    5315 as i32 as INT16,
                    16384 as i32 as INT16,
                    22725 as i32 as INT16,
                    21407 as i32 as INT16,
                    19266 as i32 as INT16,
                    16384 as i32 as INT16,
                    12873 as i32 as INT16,
                    8867 as i32 as INT16,
                    4520 as i32 as INT16,
                    12873 as i32 as INT16,
                    17855 as i32 as INT16,
                    16819 as i32 as INT16,
                    15137 as i32 as INT16,
                    12873 as i32 as INT16,
                    10114 as i32 as INT16,
                    6967 as i32 as INT16,
                    3552 as i32 as INT16,
                    8867 as i32 as INT16,
                    12299 as i32 as INT16,
                    11585 as i32 as INT16,
                    10426 as i32 as INT16,
                    8867 as i32 as INT16,
                    6967 as i32 as INT16,
                    4799 as i32 as INT16,
                    2446 as i32 as INT16,
                    4520 as i32 as INT16,
                    6270 as i32 as INT16,
                    5906 as i32 as INT16,
                    5315 as i32 as INT16,
                    4520 as i32 as INT16,
                    3552 as i32 as INT16,
                    2446 as i32 as INT16,
                    1247 as i32 as INT16,
                ];
                if (*fdct).divisors[qtblno as usize].is_null() {
                    (*fdct).divisors[qtblno as usize] = Some(
                        (*(*cinfo).mem)
                            .alloc_small
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as j_common_ptr,
                        1 as i32,
                        (64 as i32 as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<DCTELEM>() as libc::c_ulong
                            ),
                    )
                        as *mut DCTELEM
                }
                dtbl = (*fdct).divisors[qtblno as usize];
                i = 0 as i32;
                while i < 64 as i32 {
                    *dtbl.offset(i as isize) = ((*qtbl).quantval[i as usize]
                        as INT32
                        * aanscales[i as usize] as INT32
                        + ((1 as i32 as INT32)
                            << 14 as i32 - 3 as i32 - 1 as i32)
                        >> 14 as i32 - 3 as i32)
                        as DCTELEM;
                    i += 1
                }
                (*fdct).pub_0.forward_DCT[ci as usize] = Some(
                    forward_DCT
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: *mut jpeg_component_info,
                            _: JSAMPARRAY,
                            _: JBLOCKROW,
                            _: JDIMENSION,
                            _: JDIMENSION,
                            _: JDIMENSION,
                        ) -> (),
                )
            }
            2 => {
                /* For float AA&N IDCT method, divisors are equal to quantization
                 * coefficients scaled by scalefactor[row]*scalefactor[col], where
                 *   scalefactor[0] = 1
                 *   scalefactor[k] = cos(k*PI/16) * sqrt(2)    for k=1..7
                 * We apply a further scale factor of 8.
                 * What's actually stored is 1/divisor so that the inner loop can
                 * use a multiplication rather than a division.
                 */
                let mut fdtbl: *mut f32 = 0 as *mut f32;
                let mut row: i32 = 0;
                let mut col: i32 = 0;
                static mut aanscalefactor: [f64; 8] = [
                    1.0f64,
                    1.387039845f64,
                    1.306562965f64,
                    1.175875602f64,
                    1.0f64,
                    0.785694958f64,
                    0.541196100f64,
                    0.275899379f64,
                ];
                if (*fdct).float_divisors[qtblno as usize].is_null() {
                    (*fdct).float_divisors[qtblno as usize] = Some(
                        (*(*cinfo).mem)
                            .alloc_small
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as j_common_ptr,
                        1 as i32,
                        (64 as i32 as libc::c_ulong)
                            .wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong),
                    ) as *mut f32
                }
                fdtbl = (*fdct).float_divisors[qtblno as usize];
                i = 0 as i32;
                row = 0 as i32;
                while row < 8 as i32 {
                    col = 0 as i32;
                    while col < 8 as i32 {
                        *fdtbl.offset(i as isize) = (1.0f64
                            / ((*qtbl).quantval[i as usize] as f64
                                * aanscalefactor[row as usize]
                                * aanscalefactor[col as usize]
                                * 8.0f64))
                            as f32;
                        i += 1;
                        col += 1
                    }
                    row += 1
                }
                (*fdct).pub_0.forward_DCT[ci as usize] = Some(
                    forward_DCT_float
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: *mut jpeg_component_info,
                            _: JSAMPARRAY,
                            _: JBLOCKROW,
                            _: JDIMENSION,
                            _: JDIMENSION,
                            _: JDIMENSION,
                        ) -> (),
                )
            }
            _ => {
                (*(*cinfo).err).msg_code = JERR_NOT_COMPILED as i32;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
            }
        }
        ci += 1;
        compptr = compptr.offset(1)
    }
}
/*
 * Initialize FDCT manager.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_forward_dct(mut cinfo: j_compress_ptr) {
    let mut fdct: my_fdct_ptr = 0 as *mut my_fdct_controller;
    let mut i: i32 = 0;
    fdct = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        1 as i32,
        ::std::mem::size_of::<my_fdct_controller>() as libc::c_ulong,
    ) as my_fdct_ptr;
    (*cinfo).fdct = fdct as *mut jpeg_forward_dct;
    (*fdct).pub_0.start_pass =
        Some(start_pass_fdctmgr as unsafe extern "C" fn(_: j_compress_ptr) -> ());
    /* Mark divisor tables unallocated */
    i = 0 as i32;
    while i < 4 as i32 {
        (*fdct).divisors[i as usize] = 0 as *mut DCTELEM;
        (*fdct).float_divisors[i as usize] = 0 as *mut f32;
        i += 1
    }
}
