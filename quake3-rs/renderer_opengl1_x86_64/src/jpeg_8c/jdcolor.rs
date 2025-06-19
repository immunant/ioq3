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
pub use crate::src::jpeg_8c::jutils::jcopy_sample_rows;

pub type my_cconvert_ptr = *mut my_color_deconverter;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_color_deconverter {
    pub pub_0: crate::jpegint_h::jpeg_color_deconverter,
    pub Cr_r_tab: *mut i32,
    pub Cb_b_tab: *mut i32,
    pub Cr_g_tab: *mut crate::jmorecfg_h::INT32,
    pub Cb_g_tab: *mut crate::jmorecfg_h::INT32,
}
/*
 * Initialize tables for YCC->RGB colorspace conversion.
 */

unsafe extern "C" fn build_ycc_rgb_table(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut i: i32 = 0;
    let mut x: crate::jmorecfg_h::INT32 = 0;
    (*cconvert).Cr_r_tab = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        1 as i32,
        ((255 as i32 + 1 as i32) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<i32>() as libc::c_ulong),
    ) as *mut i32;
    (*cconvert).Cb_b_tab = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        1 as i32,
        ((255 as i32 + 1 as i32) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<i32>() as libc::c_ulong),
    ) as *mut i32;
    (*cconvert).Cr_g_tab = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        1 as i32,
        ((255 as i32 + 1 as i32) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::jmorecfg_h::INT32>() as libc::c_ulong),
    ) as *mut crate::jmorecfg_h::INT32;
    (*cconvert).Cb_g_tab = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        1 as i32,
        ((255 as i32 + 1 as i32) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::jmorecfg_h::INT32>() as libc::c_ulong),
    ) as *mut crate::jmorecfg_h::INT32;
    i = 0 as i32;
    x = -(128 as i32) as crate::jmorecfg_h::INT32;
    while i <= 255 as i32 {
        /* i is the actual input pixel value, in the range 0..MAXJSAMPLE */
        /* The Cb or Cr value we are thinking of is x = i - CENTERJSAMPLE */
        /* Cr=>R value is nearest int to 1.40200 * x */
        *(*cconvert).Cr_r_tab.offset(i as isize) =
            ((1.40200f64 * ((1 as isize) << 16 as i32) as f64 + 0.5f64)
                as crate::jmorecfg_h::INT32
                * x
                + ((1 as i32 as crate::jmorecfg_h::INT32) << 16 as i32 - 1 as i32)
                >> 16 as i32) as i32;
        /* Cb=>B value is nearest int to 1.77200 * x */
        *(*cconvert).Cb_b_tab.offset(i as isize) =
            ((1.77200f64 * ((1 as isize) << 16 as i32) as f64 + 0.5f64)
                as crate::jmorecfg_h::INT32
                * x
                + ((1 as i32 as crate::jmorecfg_h::INT32) << 16 as i32 - 1 as i32)
                >> 16 as i32) as i32;
        /* Cr=>G value is scaled-up -0.71414 * x */
        *(*cconvert).Cr_g_tab.offset(i as isize) =
            -((0.71414f64 * ((1 as isize) << 16 as i32) as f64 + 0.5f64)
                as crate::jmorecfg_h::INT32)
                * x;
        /* Cb=>G value is scaled-up -0.34414 * x */
        /* We also add in ONE_HALF so that need not do it in inner loop */
        *(*cconvert).Cb_g_tab.offset(i as isize) =
            -((0.34414f64 * ((1 as isize) << 16 as i32) as f64 + 0.5f64)
                as crate::jmorecfg_h::INT32)
                * x
                + ((1 as i32 as crate::jmorecfg_h::INT32) << 16 as i32 - 1 as i32);
        i += 1;
        x += 1
    }
}
/*
 * Convert some rows of samples to the output colorspace.
 *
 * Note that we change from noninterleaved, one-plane-per-component format
 * to interleaved-pixel format.  The output buffer is therefore three times
 * as wide as the input buffer.
 * A starting row offset is provided only for the input buffer.  The caller
 * can easily adjust the passed output_buf value to accommodate any row
 * offset required on that side.
 */

unsafe extern "C" fn ycc_rgb_convert(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: i32,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut y: i32 = 0;
    let mut cb: i32 = 0;
    let mut cr: i32 = 0;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut inptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    /* copy these pointers into registers if possible */
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut i32 = (*cconvert).Cr_r_tab;
    let mut Cbbtab: *mut i32 = (*cconvert).Cb_b_tab;
    let mut Crgtab: *mut crate::jmorecfg_h::INT32 = (*cconvert).Cr_g_tab;
    let mut Cbgtab: *mut crate::jmorecfg_h::INT32 = (*cconvert).Cb_g_tab;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as i32) {
            break;
        }
        inptr0 = *(*input_buf.offset(0 as i32 as isize)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1 as i32 as isize)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2 as i32 as isize)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh0 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh0;
        col = 0 as i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            y = *inptr0.offset(col as isize) as i32;
            cb = *inptr1.offset(col as isize) as i32;
            cr = *inptr2.offset(col as isize) as i32;
            /* Range-limiting is essential due to noise introduced by DCT losses. */
            *outptr.offset(0 as i32 as isize) =
                *range_limit.offset((y + *Crrtab.offset(cr as isize)) as isize);
            *outptr.offset(1 as i32 as isize) = *range_limit.offset(
                (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16 as i32)
                    as i32) as isize,
            );
            *outptr.offset(2 as i32 as isize) =
                *range_limit.offset((y + *Cbbtab.offset(cb as isize)) as isize);
            outptr = outptr.offset(3 as i32 as isize);
            col = col.wrapping_add(1)
        }
    }
}
/* *************** Cases other than YCbCr -> RGB **************/
/*
 * Color conversion for no colorspace change: just copy the data,
 * converting from separate-planes to interleaved representation.
 */

unsafe extern "C" fn null_convert(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: i32,
) {
    let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE; /* needn't bother with GETJSAMPLE() here */
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut count: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_components: i32 = (*cinfo).num_components;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    let mut ci: i32 = 0;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as i32) {
            break;
        }
        ci = 0 as i32;
        while ci < num_components {
            inptr = *(*input_buf.offset(ci as isize)).offset(input_row as isize);
            outptr = (*output_buf.offset(0 as i32 as isize)).offset(ci as isize);
            count = num_cols;
            while count > 0 as i32 as u32 {
                let fresh1 = inptr;
                inptr = inptr.offset(1);
                *outptr = *fresh1;
                outptr = outptr.offset(num_components as isize);
                count = count.wrapping_sub(1)
            }
            ci += 1
        }
        input_row = input_row.wrapping_add(1);
        output_buf = output_buf.offset(1)
    }
}
/*
 * Color conversion for grayscale: just copy the data.
 * This also works for YCbCr -> grayscale conversion, in which
 * we just copy the Y (luminance) component and ignore chrominance.
 */

unsafe extern "C" fn grayscale_convert(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: i32,
) {
    crate::src::jpeg_8c::jutils::jcopy_sample_rows(
        *input_buf.offset(0 as i32 as isize),
        input_row as i32,
        output_buf,
        0 as i32,
        num_rows,
        (*cinfo).output_width,
    );
}
/*
 * Convert grayscale to RGB: just duplicate the graylevel three times.
 * This is provided to support applications that don't want to cope
 * with grayscale as a separate case.
 */

unsafe extern "C" fn gray_rgb_convert(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: i32,
) {
    let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as i32) {
            break;
        }
        let fresh2 = input_row;
        input_row = input_row.wrapping_add(1);
        inptr = *(*input_buf.offset(0 as i32 as isize)).offset(fresh2 as isize);
        let fresh3 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh3;
        col = 0 as i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            /* We can dispense with GETJSAMPLE() here */
            let ref mut fresh4 = *outptr.offset(2 as i32 as isize);
            *fresh4 = *inptr.offset(col as isize);
            let ref mut fresh5 = *outptr.offset(1 as i32 as isize);
            *fresh5 = *fresh4;
            *outptr.offset(0 as i32 as isize) = *fresh5;
            outptr = outptr.offset(3 as i32 as isize);
            col = col.wrapping_add(1)
        }
    }
}
/*
 * Adobe-style YCCK->CMYK conversion.
 * We convert YCbCr to R=1-C, G=1-M, and B=1-Y using the same
 * conversion as above, while passing K (black) unchanged.
 * We assume build_ycc_rgb_table has been called.
 */

unsafe extern "C" fn ycck_cmyk_convert(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut input_row: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: i32,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut y: i32 = 0;
    let mut cb: i32 = 0;
    let mut cr: i32 = 0;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut inptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut inptr3: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    /* copy these pointers into registers if possible */
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut i32 = (*cconvert).Cr_r_tab;
    let mut Cbbtab: *mut i32 = (*cconvert).Cb_b_tab;
    let mut Crgtab: *mut crate::jmorecfg_h::INT32 = (*cconvert).Cr_g_tab;
    let mut Cbgtab: *mut crate::jmorecfg_h::INT32 = (*cconvert).Cb_g_tab;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as i32) {
            break;
        }
        inptr0 = *(*input_buf.offset(0 as i32 as isize)).offset(input_row as isize);
        inptr1 = *(*input_buf.offset(1 as i32 as isize)).offset(input_row as isize);
        inptr2 = *(*input_buf.offset(2 as i32 as isize)).offset(input_row as isize);
        inptr3 = *(*input_buf.offset(3 as i32 as isize)).offset(input_row as isize);
        input_row = input_row.wrapping_add(1);
        let fresh6 = output_buf;
        output_buf = output_buf.offset(1);
        outptr = *fresh6;
        col = 0 as i32 as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            y = *inptr0.offset(col as isize) as i32;
            cb = *inptr1.offset(col as isize) as i32;
            cr = *inptr2.offset(col as isize) as i32;
            /* Range-limiting is essential due to noise introduced by DCT losses. */
            *outptr.offset(0 as i32 as isize) =
                *range_limit.offset((255 as i32 - (y + *Crrtab.offset(cr as isize))) as isize); /* red */
            *outptr.offset(1 as i32 as isize) = *range_limit.offset(
                (255 as i32
                    - (y + (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16 as i32)
                        as i32)) as isize,
            ); /* blue */
            *outptr.offset(2 as i32 as isize) =
                *range_limit.offset((255 as i32 - (y + *Cbbtab.offset(cb as isize))) as isize);
            /* K passes through unchanged */
            *outptr.offset(3 as i32 as isize) = *inptr3.offset(col as isize); /* don't need GETJSAMPLE here */
            outptr = outptr.offset(4 as i32 as isize);
            col = col.wrapping_add(1)
        }
    }
}
/*
 * Empty method for start_pass.
 */

unsafe extern "C" fn start_pass_dcolor(mut _cinfo: crate::jpeglib_h::j_decompress_ptr) {
    /* no work needed */
}
/*
 * Module initialization routine for output colorspace conversion.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_color_deconverter(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    let mut cconvert: my_cconvert_ptr = 0 as *mut my_color_deconverter;
    let mut ci: i32 = 0;
    cconvert = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        1 as i32,
        ::std::mem::size_of::<my_color_deconverter>() as libc::c_ulong,
    ) as my_cconvert_ptr;
    (*cinfo).cconvert = cconvert as *mut crate::jpegint_h::jpeg_color_deconverter;
    (*cconvert).pub_0.start_pass = Some(
        start_pass_dcolor as unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> (),
    );
    /* Make sure num_components agrees with jpeg_color_space */
    match (*cinfo).jpeg_color_space as u32 {
        1 => {
            if (*cinfo).num_components != 1 as i32 {
                (*(*cinfo).err).msg_code =
                    crate::src::jpeg_8c::jerror::JERR_BAD_J_COLORSPACE as i32;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
        }
        2 | 3 => {
            if (*cinfo).num_components != 3 as i32 {
                (*(*cinfo).err).msg_code =
                    crate::src::jpeg_8c::jerror::JERR_BAD_J_COLORSPACE as i32;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
        }
        4 | 5 => {
            if (*cinfo).num_components != 4 as i32 {
                (*(*cinfo).err).msg_code =
                    crate::src::jpeg_8c::jerror::JERR_BAD_J_COLORSPACE as i32;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
        }
        _ => {
            /* JCS_UNKNOWN can be anything */
            if (*cinfo).num_components < 1 as i32 {
                (*(*cinfo).err).msg_code =
                    crate::src::jpeg_8c::jerror::JERR_BAD_J_COLORSPACE as i32;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
        }
    }
    /* Set out_color_components and conversion method based on requested space.
     * Also clear the component_needed flags for any unused components,
     * so that earlier pipeline stages can avoid useless computation.
     */
    match (*cinfo).out_color_space as u32 {
        1 => {
            (*cinfo).out_color_components = 1 as i32;
            if (*cinfo).jpeg_color_space as u32 == crate::jpeglib_h::JCS_GRAYSCALE as i32 as u32
                || (*cinfo).jpeg_color_space as u32 == crate::jpeglib_h::JCS_YCbCr as i32 as u32
            {
                (*cconvert).pub_0.color_convert = Some(
                    grayscale_convert
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: crate::jpeglib_h::JSAMPIMAGE,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: i32,
                        ) -> (),
                );
                /* For color->grayscale conversion, only the Y (0) component is needed */
                ci = 1 as i32;
                while ci < (*cinfo).num_components {
                    (*(*cinfo).comp_info.offset(ci as isize)).component_needed = 0 as i32;
                    ci += 1
                }
            } else {
                (*(*cinfo).err).msg_code =
                    crate::src::jpeg_8c::jerror::JERR_CONVERSION_NOTIMPL as i32;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
        }
        2 => {
            (*cinfo).out_color_components = 3 as i32;
            if (*cinfo).jpeg_color_space as u32 == crate::jpeglib_h::JCS_YCbCr as i32 as u32 {
                (*cconvert).pub_0.color_convert = Some(
                    ycc_rgb_convert
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: crate::jpeglib_h::JSAMPIMAGE,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: i32,
                        ) -> (),
                );
                build_ycc_rgb_table(cinfo);
            } else if (*cinfo).jpeg_color_space as u32
                == crate::jpeglib_h::JCS_GRAYSCALE as i32 as u32
            {
                (*cconvert).pub_0.color_convert = Some(
                    gray_rgb_convert
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: crate::jpeglib_h::JSAMPIMAGE,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: i32,
                        ) -> (),
                )
            } else if (*cinfo).jpeg_color_space as u32 == crate::jpeglib_h::JCS_RGB as i32 as u32
                && 3 as i32 == 3 as i32
            {
                (*cconvert).pub_0.color_convert = Some(
                    null_convert
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: crate::jpeglib_h::JSAMPIMAGE,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: i32,
                        ) -> (),
                )
            } else {
                (*(*cinfo).err).msg_code =
                    crate::src::jpeg_8c::jerror::JERR_CONVERSION_NOTIMPL as i32;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
        }
        4 => {
            (*cinfo).out_color_components = 4 as i32;
            if (*cinfo).jpeg_color_space as u32 == crate::jpeglib_h::JCS_YCCK as i32 as u32 {
                (*cconvert).pub_0.color_convert = Some(
                    ycck_cmyk_convert
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: crate::jpeglib_h::JSAMPIMAGE,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: i32,
                        ) -> (),
                );
                build_ycc_rgb_table(cinfo);
            } else if (*cinfo).jpeg_color_space as u32 == crate::jpeglib_h::JCS_CMYK as i32 as u32 {
                (*cconvert).pub_0.color_convert = Some(
                    null_convert
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: crate::jpeglib_h::JSAMPIMAGE,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: i32,
                        ) -> (),
                )
            } else {
                (*(*cinfo).err).msg_code =
                    crate::src::jpeg_8c::jerror::JERR_CONVERSION_NOTIMPL as i32;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
        }
        _ => {
            /* Permit null conversion to same output space */
            if (*cinfo).out_color_space as u32 == (*cinfo).jpeg_color_space as u32 {
                (*cinfo).out_color_components = (*cinfo).num_components; /* unsupported non-null conversion */
                (*cconvert).pub_0.color_convert = Some(
                    null_convert
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: crate::jpeglib_h::JSAMPIMAGE,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: i32,
                        ) -> (),
                )
            } else {
                (*(*cinfo).err).msg_code =
                    crate::src::jpeg_8c::jerror::JERR_CONVERSION_NOTIMPL as i32; /* single colormapped output component */
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
        }
    }
    if (*cinfo).quantize_colors != 0 {
        (*cinfo).output_components = 1 as i32
    } else {
        (*cinfo).output_components = (*cinfo).out_color_components
    };
}
