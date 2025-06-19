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

pub type my_cconvert_ptr = *mut my_color_converter;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_color_converter {
    pub pub_0: jpeg_color_converter,
    pub rgb_ycc_tab: *mut INT32,
}
/*
 * Initialize for RGB->YCC colorspace conversion.
 */

unsafe extern "C" fn rgb_ycc_start(mut cinfo: j_compress_ptr) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut rgb_ycc_tab: *mut INT32 = 0 as *mut INT32;
    let mut i: INT32 = 0;
    /* Allocate and fill in the conversion tables. */
    rgb_ycc_tab = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        1 as i32,
        ((8 as i32 * (255 as i32 + 1 as i32)) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<INT32>() as libc::c_ulong),
    ) as *mut INT32;
    (*cconvert).rgb_ycc_tab = rgb_ycc_tab;
    i = 0 as i32 as INT32;
    while i <= 255 as i32 as isize {
        *rgb_ycc_tab.offset((i + 0 as i32 as isize) as isize) =
            (0.29900f64 * ((1 as isize) << 16 as i32) as f64 + 0.5f64) as INT32 * i;
        *rgb_ycc_tab.offset((i + (1 as i32 * (255 as i32 + 1 as i32)) as isize) as isize) =
            (0.58700f64 * ((1 as isize) << 16 as i32) as f64 + 0.5f64) as INT32 * i;
        *rgb_ycc_tab.offset((i + (2 as i32 * (255 as i32 + 1 as i32)) as isize) as isize) =
            (0.11400f64 * ((1 as isize) << 16 as i32) as f64 + 0.5f64) as INT32 * i
                + ((1 as i32 as INT32) << 16 as i32 - 1 as i32);
        *rgb_ycc_tab.offset((i + (3 as i32 * (255 as i32 + 1 as i32)) as isize) as isize) =
            -((0.16874f64 * ((1 as isize) << 16 as i32) as f64 + 0.5f64) as INT32) * i;
        *rgb_ycc_tab.offset((i + (4 as i32 * (255 as i32 + 1 as i32)) as isize) as isize) =
            -((0.33126f64 * ((1 as isize) << 16 as i32) as f64 + 0.5f64) as INT32) * i;
        /* We use a rounding fudge-factor of 0.5-epsilon for Cb and Cr.
         * This ensures that the maximum output will round to MAXJSAMPLE
         * not MAXJSAMPLE+1, and thus that we don't have to range-limit.
         */
        *rgb_ycc_tab.offset((i + (5 as i32 * (255 as i32 + 1 as i32)) as isize) as isize) =
            (0.50000f64 * ((1 as isize) << 16 as i32) as f64 + 0.5f64) as INT32 * i
                + ((128 as i32 as INT32) << 16 as i32)
                + ((1 as i32 as INT32) << 16 as i32 - 1 as i32)
                - 1 as i32 as isize;
        /*  B=>Cb and R=>Cr tables are the same
            rgb_ycc_tab[i+R_CR_OFF] = FIX(0.50000) * i    + CBCR_OFFSET + ONE_HALF-1;
        */
        *rgb_ycc_tab.offset((i + (6 as i32 * (255 as i32 + 1 as i32)) as isize) as isize) =
            -((0.41869f64 * ((1 as isize) << 16 as i32) as f64 + 0.5f64) as INT32) * i;
        *rgb_ycc_tab.offset((i + (7 as i32 * (255 as i32 + 1 as i32)) as isize) as isize) =
            -((0.08131f64 * ((1 as isize) << 16 as i32) as f64 + 0.5f64) as INT32) * i;
        i += 1
    }
}
/*
 * Convert some rows of samples to the JPEG colorspace.
 *
 * Note that we change from the application's interleaved-pixel format
 * to our internal noninterleaved, one-plane-per-component format.
 * The input buffer is therefore three times as wide as the output buffer.
 *
 * A starting row offset is provided only for the output buffer.  The caller
 * can easily adjust the passed input_buf value to accommodate any row
 * offset required on that side.
 */

unsafe extern "C" fn rgb_ycc_convert(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: i32,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut r: i32 = 0;
    let mut g: i32 = 0;
    let mut b: i32 = 0;
    let mut ctab: *mut INT32 = (*cconvert).rgb_ycc_tab;
    let mut inptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr0: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr1: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr2: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as i32) {
            break;
        }
        let fresh0 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh0;
        outptr0 = *(*output_buf.offset(0 as i32 as isize)).offset(output_row as isize);
        outptr1 = *(*output_buf.offset(1 as i32 as isize)).offset(output_row as isize);
        outptr2 = *(*output_buf.offset(2 as i32 as isize)).offset(output_row as isize);
        output_row = output_row.wrapping_add(1);
        col = 0 as i32 as JDIMENSION;
        while col < num_cols {
            r = *inptr.offset(0 as i32 as isize) as i32;
            g = *inptr.offset(1 as i32 as isize) as i32;
            b = *inptr.offset(2 as i32 as isize) as i32;
            inptr = inptr.offset(3 as i32 as isize);
            /* If the inputs are 0..MAXJSAMPLE, the outputs of these equations
             * must be too; we do not need an explicit range-limiting operation.
             * Hence the value being shifted is never negative, and we don't
             * need the general RIGHT_SHIFT macro.
             */
            /* Y */
            *outptr0.offset(col as isize) = (*ctab.offset((r + 0 as i32) as isize)
                + *ctab.offset((g + 1 as i32 * (255 as i32 + 1 as i32)) as isize)
                + *ctab.offset((b + 2 as i32 * (255 as i32 + 1 as i32)) as isize)
                >> 16 as i32) as JSAMPLE;
            /* Cb */
            *outptr1.offset(col as isize) = (*ctab
                .offset((r + 3 as i32 * (255 as i32 + 1 as i32)) as isize)
                + *ctab.offset((g + 4 as i32 * (255 as i32 + 1 as i32)) as isize)
                + *ctab.offset((b + 5 as i32 * (255 as i32 + 1 as i32)) as isize)
                >> 16 as i32) as JSAMPLE;
            /* Cr */
            *outptr2.offset(col as isize) = (*ctab
                .offset((r + 5 as i32 * (255 as i32 + 1 as i32)) as isize)
                + *ctab.offset((g + 6 as i32 * (255 as i32 + 1 as i32)) as isize)
                + *ctab.offset((b + 7 as i32 * (255 as i32 + 1 as i32)) as isize)
                >> 16 as i32) as JSAMPLE;
            col = col.wrapping_add(1)
        }
    }
}
/* *************** Cases other than RGB -> YCbCr **************/
/*
 * Convert some rows of samples to the JPEG colorspace.
 * This version handles RGB->grayscale conversion, which is the same
 * as the RGB->Y portion of RGB->YCbCr.
 * We assume rgb_ycc_start has been called (we only use the Y tables).
 */

unsafe extern "C" fn rgb_gray_convert(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: i32,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut r: i32 = 0;
    let mut g: i32 = 0;
    let mut b: i32 = 0;
    let mut ctab: *mut INT32 = (*cconvert).rgb_ycc_tab;
    let mut inptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as i32) {
            break;
        }
        let fresh1 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh1;
        outptr = *(*output_buf.offset(0 as i32 as isize)).offset(output_row as isize);
        output_row = output_row.wrapping_add(1);
        col = 0 as i32 as JDIMENSION;
        while col < num_cols {
            r = *inptr.offset(0 as i32 as isize) as i32;
            g = *inptr.offset(1 as i32 as isize) as i32;
            b = *inptr.offset(2 as i32 as isize) as i32;
            inptr = inptr.offset(3 as i32 as isize);
            /* Y */
            *outptr.offset(col as isize) = (*ctab.offset((r + 0 as i32) as isize)
                + *ctab.offset((g + 1 as i32 * (255 as i32 + 1 as i32)) as isize)
                + *ctab.offset((b + 2 as i32 * (255 as i32 + 1 as i32)) as isize)
                >> 16 as i32) as JSAMPLE;
            col = col.wrapping_add(1)
        }
    }
}
/*
 * Convert some rows of samples to the JPEG colorspace.
 * This version handles Adobe-style CMYK->YCCK conversion,
 * where we convert R=1-C, G=1-M, and B=1-Y to YCbCr using the same
 * conversion as above, while passing K (black) unchanged.
 * We assume rgb_ycc_start has been called.
 */

unsafe extern "C" fn cmyk_ycck_convert(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: i32,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut r: i32 = 0;
    let mut g: i32 = 0;
    let mut b: i32 = 0;
    let mut ctab: *mut INT32 = (*cconvert).rgb_ycc_tab;
    let mut inptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr0: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr1: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr2: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr3: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as i32) {
            break;
        }
        let fresh2 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh2;
        outptr0 = *(*output_buf.offset(0 as i32 as isize)).offset(output_row as isize);
        outptr1 = *(*output_buf.offset(1 as i32 as isize)).offset(output_row as isize);
        outptr2 = *(*output_buf.offset(2 as i32 as isize)).offset(output_row as isize);
        outptr3 = *(*output_buf.offset(3 as i32 as isize)).offset(output_row as isize);
        output_row = output_row.wrapping_add(1);
        col = 0 as i32 as JDIMENSION;
        while col < num_cols {
            r = 255 as i32 - *inptr.offset(0 as i32 as isize) as i32;
            g = 255 as i32 - *inptr.offset(1 as i32 as isize) as i32;
            b = 255 as i32 - *inptr.offset(2 as i32 as isize) as i32;
            /* K passes through as-is */
            *outptr3.offset(col as isize) = *inptr.offset(3 as i32 as isize); /* don't need GETJSAMPLE here */
            inptr = inptr.offset(4 as i32 as isize);
            /* If the inputs are 0..MAXJSAMPLE, the outputs of these equations
             * must be too; we do not need an explicit range-limiting operation.
             * Hence the value being shifted is never negative, and we don't
             * need the general RIGHT_SHIFT macro.
             */
            /* Y */
            *outptr0.offset(col as isize) = (*ctab.offset((r + 0 as i32) as isize)
                + *ctab.offset((g + 1 as i32 * (255 as i32 + 1 as i32)) as isize)
                + *ctab.offset((b + 2 as i32 * (255 as i32 + 1 as i32)) as isize)
                >> 16 as i32) as JSAMPLE;
            /* Cb */
            *outptr1.offset(col as isize) = (*ctab
                .offset((r + 3 as i32 * (255 as i32 + 1 as i32)) as isize)
                + *ctab.offset((g + 4 as i32 * (255 as i32 + 1 as i32)) as isize)
                + *ctab.offset((b + 5 as i32 * (255 as i32 + 1 as i32)) as isize)
                >> 16 as i32) as JSAMPLE;
            /* Cr */
            *outptr2.offset(col as isize) = (*ctab
                .offset((r + 5 as i32 * (255 as i32 + 1 as i32)) as isize)
                + *ctab.offset((g + 6 as i32 * (255 as i32 + 1 as i32)) as isize)
                + *ctab.offset((b + 7 as i32 * (255 as i32 + 1 as i32)) as isize)
                >> 16 as i32) as JSAMPLE;
            col = col.wrapping_add(1)
        }
    }
}
/*
 * Convert some rows of samples to the JPEG colorspace.
 * This version handles grayscale output with no conversion.
 * The source can be either plain grayscale or YCbCr (since Y == gray).
 */

unsafe extern "C" fn grayscale_convert(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: i32,
) {
    let mut inptr: JSAMPROW = 0 as *mut JSAMPLE; /* don't need GETJSAMPLE() here */
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    let mut instride: i32 = (*cinfo).input_components;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as i32) {
            break;
        }
        let fresh3 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh3;
        outptr = *(*output_buf.offset(0 as i32 as isize)).offset(output_row as isize);
        output_row = output_row.wrapping_add(1);
        col = 0 as i32 as JDIMENSION;
        while col < num_cols {
            *outptr.offset(col as isize) = *inptr.offset(0 as i32 as isize);
            inptr = inptr.offset(instride as isize);
            col = col.wrapping_add(1)
        }
    }
}
/*
 * Convert some rows of samples to the JPEG colorspace.
 * This version handles multi-component colorspaces without conversion.
 * We assume input_components == num_components.
 */

unsafe extern "C" fn null_convert(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut output_buf: JSAMPIMAGE,
    mut output_row: JDIMENSION,
    mut num_rows: i32,
) {
    let mut inptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    let mut ci: i32 = 0;
    let mut nc: i32 = (*cinfo).num_components;
    let mut num_cols: JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as i32) {
            break;
        }
        /* It seems fastest to make a separate pass for each component. */
        ci = 0 as i32; /* don't need GETJSAMPLE() here */
        while ci < nc {
            inptr = *input_buf;
            outptr = *(*output_buf.offset(ci as isize)).offset(output_row as isize);
            col = 0 as i32 as JDIMENSION;
            while col < num_cols {
                *outptr.offset(col as isize) = *inptr.offset(ci as isize);
                inptr = inptr.offset(nc as isize);
                col = col.wrapping_add(1)
            }
            ci += 1
        }
        input_buf = input_buf.offset(1);
        output_row = output_row.wrapping_add(1)
    }
}
/*
 * Empty method for start_pass.
 */

unsafe extern "C" fn null_method(mut _cinfo: j_compress_ptr) {
    /* no work needed */
}
/*
 * Module initialization routine for input colorspace conversion.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_color_converter(mut cinfo: j_compress_ptr) {
    let mut cconvert: my_cconvert_ptr = 0 as *mut my_color_converter;
    cconvert = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        1 as i32,
        ::std::mem::size_of::<my_color_converter>() as libc::c_ulong,
    ) as my_cconvert_ptr;
    (*cinfo).cconvert = cconvert as *mut jpeg_color_converter;
    /* set start_pass to null method until we find out differently */
    (*cconvert).pub_0.start_pass =
        Some(null_method as unsafe extern "C" fn(_: j_compress_ptr) -> ());
    /* Make sure input_components agrees with in_color_space */
    match (*cinfo).in_color_space as u32 {
        1 => {
            if (*cinfo).input_components != 1 as i32 {
                (*(*cinfo).err).msg_code = JERR_BAD_IN_COLORSPACE as i32;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        2 | 3 => {
            /* else share code with YCbCr */
            if (*cinfo).input_components != 3 as i32 {
                (*(*cinfo).err).msg_code = JERR_BAD_IN_COLORSPACE as i32;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        4 | 5 => {
            if (*cinfo).input_components != 4 as i32 {
                (*(*cinfo).err).msg_code = JERR_BAD_IN_COLORSPACE as i32;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        _ => {
            /* JCS_UNKNOWN can be anything */
            if (*cinfo).input_components < 1 as i32 {
                (*(*cinfo).err).msg_code = JERR_BAD_IN_COLORSPACE as i32;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
    }
    /* Check num_components, set conversion method based on requested space */
    match (*cinfo).jpeg_color_space as u32 {
        1 => {
            if (*cinfo).num_components != 1 as i32 {
                (*(*cinfo).err).msg_code = JERR_BAD_J_COLORSPACE as i32;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            if (*cinfo).in_color_space as u32 == JCS_GRAYSCALE as i32 as u32 {
                (*cconvert).pub_0.color_convert = Some(
                    grayscale_convert
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: JSAMPARRAY,
                            _: JSAMPIMAGE,
                            _: JDIMENSION,
                            _: i32,
                        ) -> (),
                )
            } else if (*cinfo).in_color_space as u32 == JCS_RGB as i32 as u32 {
                (*cconvert).pub_0.start_pass =
                    Some(rgb_ycc_start as unsafe extern "C" fn(_: j_compress_ptr) -> ());
                (*cconvert).pub_0.color_convert = Some(
                    rgb_gray_convert
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: JSAMPARRAY,
                            _: JSAMPIMAGE,
                            _: JDIMENSION,
                            _: i32,
                        ) -> (),
                )
            } else if (*cinfo).in_color_space as u32 == JCS_YCbCr as i32 as u32 {
                (*cconvert).pub_0.color_convert = Some(
                    grayscale_convert
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: JSAMPARRAY,
                            _: JSAMPIMAGE,
                            _: JDIMENSION,
                            _: i32,
                        ) -> (),
                )
            } else {
                (*(*cinfo).err).msg_code = JERR_CONVERSION_NOTIMPL as i32;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        2 => {
            if (*cinfo).num_components != 3 as i32 {
                (*(*cinfo).err).msg_code = JERR_BAD_J_COLORSPACE as i32;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            if (*cinfo).in_color_space as u32 == JCS_RGB as i32 as u32 && 3 as i32 == 3 as i32 {
                (*cconvert).pub_0.color_convert = Some(
                    null_convert
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: JSAMPARRAY,
                            _: JSAMPIMAGE,
                            _: JDIMENSION,
                            _: i32,
                        ) -> (),
                )
            } else {
                (*(*cinfo).err).msg_code = JERR_CONVERSION_NOTIMPL as i32;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        3 => {
            if (*cinfo).num_components != 3 as i32 {
                (*(*cinfo).err).msg_code = JERR_BAD_J_COLORSPACE as i32;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            if (*cinfo).in_color_space as u32 == JCS_RGB as i32 as u32 {
                (*cconvert).pub_0.start_pass =
                    Some(rgb_ycc_start as unsafe extern "C" fn(_: j_compress_ptr) -> ());
                (*cconvert).pub_0.color_convert = Some(
                    rgb_ycc_convert
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: JSAMPARRAY,
                            _: JSAMPIMAGE,
                            _: JDIMENSION,
                            _: i32,
                        ) -> (),
                )
            } else if (*cinfo).in_color_space as u32 == JCS_YCbCr as i32 as u32 {
                (*cconvert).pub_0.color_convert = Some(
                    null_convert
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: JSAMPARRAY,
                            _: JSAMPIMAGE,
                            _: JDIMENSION,
                            _: i32,
                        ) -> (),
                )
            } else {
                (*(*cinfo).err).msg_code = JERR_CONVERSION_NOTIMPL as i32;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        4 => {
            if (*cinfo).num_components != 4 as i32 {
                (*(*cinfo).err).msg_code = JERR_BAD_J_COLORSPACE as i32;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            if (*cinfo).in_color_space as u32 == JCS_CMYK as i32 as u32 {
                (*cconvert).pub_0.color_convert = Some(
                    null_convert
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: JSAMPARRAY,
                            _: JSAMPIMAGE,
                            _: JDIMENSION,
                            _: i32,
                        ) -> (),
                )
            } else {
                (*(*cinfo).err).msg_code = JERR_CONVERSION_NOTIMPL as i32;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        5 => {
            if (*cinfo).num_components != 4 as i32 {
                (*(*cinfo).err).msg_code = JERR_BAD_J_COLORSPACE as i32;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            if (*cinfo).in_color_space as u32 == JCS_CMYK as i32 as u32 {
                (*cconvert).pub_0.start_pass =
                    Some(rgb_ycc_start as unsafe extern "C" fn(_: j_compress_ptr) -> ());
                (*cconvert).pub_0.color_convert = Some(
                    cmyk_ycck_convert
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: JSAMPARRAY,
                            _: JSAMPIMAGE,
                            _: JDIMENSION,
                            _: i32,
                        ) -> (),
                )
            } else if (*cinfo).in_color_space as u32 == JCS_YCCK as i32 as u32 {
                (*cconvert).pub_0.color_convert = Some(
                    null_convert
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: JSAMPARRAY,
                            _: JSAMPIMAGE,
                            _: JDIMENSION,
                            _: i32,
                        ) -> (),
                )
            } else {
                (*(*cinfo).err).msg_code = JERR_CONVERSION_NOTIMPL as i32;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        _ => {
            /* allow null conversion of JCS_UNKNOWN */
            if (*cinfo).jpeg_color_space as u32 != (*cinfo).in_color_space as u32
                || (*cinfo).num_components != (*cinfo).input_components
            {
                (*(*cinfo).err).msg_code = JERR_CONVERSION_NOTIMPL as i32;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            (*cconvert).pub_0.color_convert = Some(
                null_convert
                    as unsafe extern "C" fn(
                        _: j_compress_ptr,
                        _: JSAMPARRAY,
                        _: JSAMPIMAGE,
                        _: JDIMENSION,
                        _: i32,
                    ) -> (),
            )
        }
    };
}
