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
pub use crate::src::jpeg_8c::jutils::jdiv_round_up;
pub use crate::src::jpeg_8c::jutils::jpeg_natural_order;
pub use crate::src::jpeg_8c::jutils::jpeg_natural_order2;
pub use crate::src::jpeg_8c::jutils::jpeg_natural_order3;
pub use crate::src::jpeg_8c::jutils::jpeg_natural_order4;
pub use crate::src::jpeg_8c::jutils::jpeg_natural_order5;
pub use crate::src::jpeg_8c::jutils::jpeg_natural_order6;
pub use crate::src::jpeg_8c::jutils::jpeg_natural_order7;

pub type my_inputctl_ptr = *mut my_input_controller;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_input_controller {
    pub pub_0: jpeg_input_controller,
    pub inheaders: i32,
}
/*
 * Routines to calculate various quantities related to the size of the image.
 */
/*
 * Compute output image dimensions and related values.
 * NOTE: this is exported for possible use by application.
 * Hence it mustn't do anything that can't be done twice.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_core_output_dimensions(mut cinfo: j_decompress_ptr)
/* Do computations that are needed before master selection phase.
 * This function is used for transcoding and full decompression.
 */
{
    let mut ci: i32 = 0;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    /* Compute actual output image dimensions and DCT scaling choices. */
    if (*cinfo).scale_num.wrapping_mul((*cinfo).block_size as u32) <= (*cinfo).scale_denom {
        /* Provide 1/block_size scaling */
        (*cinfo).output_width =
            jdiv_round_up((*cinfo).image_width as isize, (*cinfo).block_size as isize)
                as JDIMENSION;
        (*cinfo).output_height =
            jdiv_round_up((*cinfo).image_height as isize, (*cinfo).block_size as isize)
                as JDIMENSION;
        (*cinfo).min_DCT_h_scaled_size = 1 as i32;
        (*cinfo).min_DCT_v_scaled_size = 1 as i32
    } else if (*cinfo).scale_num.wrapping_mul((*cinfo).block_size as u32)
        <= (*cinfo).scale_denom.wrapping_mul(2 as i32 as u32)
    {
        /* Provide 2/block_size scaling */
        (*cinfo).output_width = jdiv_round_up(
            (*cinfo).image_width as isize * 2 as isize,
            (*cinfo).block_size as isize,
        ) as JDIMENSION;
        (*cinfo).output_height = jdiv_round_up(
            (*cinfo).image_height as isize * 2 as isize,
            (*cinfo).block_size as isize,
        ) as JDIMENSION;
        (*cinfo).min_DCT_h_scaled_size = 2 as i32;
        (*cinfo).min_DCT_v_scaled_size = 2 as i32
    } else if (*cinfo).scale_num.wrapping_mul((*cinfo).block_size as u32)
        <= (*cinfo).scale_denom.wrapping_mul(3 as i32 as u32)
    {
        /* Provide 3/block_size scaling */
        (*cinfo).output_width = jdiv_round_up(
            (*cinfo).image_width as isize * 3 as isize,
            (*cinfo).block_size as isize,
        ) as JDIMENSION;
        (*cinfo).output_height = jdiv_round_up(
            (*cinfo).image_height as isize * 3 as isize,
            (*cinfo).block_size as isize,
        ) as JDIMENSION;
        (*cinfo).min_DCT_h_scaled_size = 3 as i32;
        (*cinfo).min_DCT_v_scaled_size = 3 as i32
    } else if (*cinfo).scale_num.wrapping_mul((*cinfo).block_size as u32)
        <= (*cinfo).scale_denom.wrapping_mul(4 as i32 as u32)
    {
        /* Provide 4/block_size scaling */
        (*cinfo).output_width = jdiv_round_up(
            (*cinfo).image_width as isize * 4 as isize,
            (*cinfo).block_size as isize,
        ) as JDIMENSION;
        (*cinfo).output_height = jdiv_round_up(
            (*cinfo).image_height as isize * 4 as isize,
            (*cinfo).block_size as isize,
        ) as JDIMENSION;
        (*cinfo).min_DCT_h_scaled_size = 4 as i32;
        (*cinfo).min_DCT_v_scaled_size = 4 as i32
    } else if (*cinfo).scale_num.wrapping_mul((*cinfo).block_size as u32)
        <= (*cinfo).scale_denom.wrapping_mul(5 as i32 as u32)
    {
        /* Provide 5/block_size scaling */
        (*cinfo).output_width = jdiv_round_up(
            (*cinfo).image_width as isize * 5 as isize,
            (*cinfo).block_size as isize,
        ) as JDIMENSION;
        (*cinfo).output_height = jdiv_round_up(
            (*cinfo).image_height as isize * 5 as isize,
            (*cinfo).block_size as isize,
        ) as JDIMENSION;
        (*cinfo).min_DCT_h_scaled_size = 5 as i32;
        (*cinfo).min_DCT_v_scaled_size = 5 as i32
    } else if (*cinfo).scale_num.wrapping_mul((*cinfo).block_size as u32)
        <= (*cinfo).scale_denom.wrapping_mul(6 as i32 as u32)
    {
        /* Provide 6/block_size scaling */
        (*cinfo).output_width = jdiv_round_up(
            (*cinfo).image_width as isize * 6 as isize,
            (*cinfo).block_size as isize,
        ) as JDIMENSION;
        (*cinfo).output_height = jdiv_round_up(
            (*cinfo).image_height as isize * 6 as isize,
            (*cinfo).block_size as isize,
        ) as JDIMENSION;
        (*cinfo).min_DCT_h_scaled_size = 6 as i32;
        (*cinfo).min_DCT_v_scaled_size = 6 as i32
    } else if (*cinfo).scale_num.wrapping_mul((*cinfo).block_size as u32)
        <= (*cinfo).scale_denom.wrapping_mul(7 as i32 as u32)
    {
        /* Provide 7/block_size scaling */
        (*cinfo).output_width = jdiv_round_up(
            (*cinfo).image_width as isize * 7 as isize,
            (*cinfo).block_size as isize,
        ) as JDIMENSION;
        (*cinfo).output_height = jdiv_round_up(
            (*cinfo).image_height as isize * 7 as isize,
            (*cinfo).block_size as isize,
        ) as JDIMENSION;
        (*cinfo).min_DCT_h_scaled_size = 7 as i32;
        (*cinfo).min_DCT_v_scaled_size = 7 as i32
    } else if (*cinfo).scale_num.wrapping_mul((*cinfo).block_size as u32)
        <= (*cinfo).scale_denom.wrapping_mul(8 as i32 as u32)
    {
        /* Provide 8/block_size scaling */
        (*cinfo).output_width = jdiv_round_up(
            (*cinfo).image_width as isize * 8 as isize,
            (*cinfo).block_size as isize,
        ) as JDIMENSION;
        (*cinfo).output_height = jdiv_round_up(
            (*cinfo).image_height as isize * 8 as isize,
            (*cinfo).block_size as isize,
        ) as JDIMENSION;
        (*cinfo).min_DCT_h_scaled_size = 8 as i32;
        (*cinfo).min_DCT_v_scaled_size = 8 as i32
    } else if (*cinfo).scale_num.wrapping_mul((*cinfo).block_size as u32)
        <= (*cinfo).scale_denom.wrapping_mul(9 as i32 as u32)
    {
        /* Provide 9/block_size scaling */
        (*cinfo).output_width = jdiv_round_up(
            (*cinfo).image_width as isize * 9 as isize,
            (*cinfo).block_size as isize,
        ) as JDIMENSION;
        (*cinfo).output_height = jdiv_round_up(
            (*cinfo).image_height as isize * 9 as isize,
            (*cinfo).block_size as isize,
        ) as JDIMENSION;
        (*cinfo).min_DCT_h_scaled_size = 9 as i32;
        (*cinfo).min_DCT_v_scaled_size = 9 as i32
    } else if (*cinfo).scale_num.wrapping_mul((*cinfo).block_size as u32)
        <= (*cinfo).scale_denom.wrapping_mul(10 as i32 as u32)
    {
        /* Provide 10/block_size scaling */
        (*cinfo).output_width = jdiv_round_up(
            (*cinfo).image_width as isize * 10 as isize,
            (*cinfo).block_size as isize,
        ) as JDIMENSION;
        (*cinfo).output_height = jdiv_round_up(
            (*cinfo).image_height as isize * 10 as isize,
            (*cinfo).block_size as isize,
        ) as JDIMENSION;
        (*cinfo).min_DCT_h_scaled_size = 10 as i32;
        (*cinfo).min_DCT_v_scaled_size = 10 as i32
    } else if (*cinfo).scale_num.wrapping_mul((*cinfo).block_size as u32)
        <= (*cinfo).scale_denom.wrapping_mul(11 as i32 as u32)
    {
        /* Provide 11/block_size scaling */
        (*cinfo).output_width = jdiv_round_up(
            (*cinfo).image_width as isize * 11 as isize,
            (*cinfo).block_size as isize,
        ) as JDIMENSION;
        (*cinfo).output_height = jdiv_round_up(
            (*cinfo).image_height as isize * 11 as isize,
            (*cinfo).block_size as isize,
        ) as JDIMENSION;
        (*cinfo).min_DCT_h_scaled_size = 11 as i32;
        (*cinfo).min_DCT_v_scaled_size = 11 as i32
    } else if (*cinfo).scale_num.wrapping_mul((*cinfo).block_size as u32)
        <= (*cinfo).scale_denom.wrapping_mul(12 as i32 as u32)
    {
        /* Provide 12/block_size scaling */
        (*cinfo).output_width = jdiv_round_up(
            (*cinfo).image_width as isize * 12 as isize,
            (*cinfo).block_size as isize,
        ) as JDIMENSION;
        (*cinfo).output_height = jdiv_round_up(
            (*cinfo).image_height as isize * 12 as isize,
            (*cinfo).block_size as isize,
        ) as JDIMENSION;
        (*cinfo).min_DCT_h_scaled_size = 12 as i32;
        (*cinfo).min_DCT_v_scaled_size = 12 as i32
    } else if (*cinfo).scale_num.wrapping_mul((*cinfo).block_size as u32)
        <= (*cinfo).scale_denom.wrapping_mul(13 as i32 as u32)
    {
        /* Provide 13/block_size scaling */
        (*cinfo).output_width = jdiv_round_up(
            (*cinfo).image_width as isize * 13 as isize,
            (*cinfo).block_size as isize,
        ) as JDIMENSION;
        (*cinfo).output_height = jdiv_round_up(
            (*cinfo).image_height as isize * 13 as isize,
            (*cinfo).block_size as isize,
        ) as JDIMENSION;
        (*cinfo).min_DCT_h_scaled_size = 13 as i32;
        (*cinfo).min_DCT_v_scaled_size = 13 as i32
    } else if (*cinfo).scale_num.wrapping_mul((*cinfo).block_size as u32)
        <= (*cinfo).scale_denom.wrapping_mul(14 as i32 as u32)
    {
        /* Provide 14/block_size scaling */
        (*cinfo).output_width = jdiv_round_up(
            (*cinfo).image_width as isize * 14 as isize,
            (*cinfo).block_size as isize,
        ) as JDIMENSION;
        (*cinfo).output_height = jdiv_round_up(
            (*cinfo).image_height as isize * 14 as isize,
            (*cinfo).block_size as isize,
        ) as JDIMENSION;
        (*cinfo).min_DCT_h_scaled_size = 14 as i32;
        (*cinfo).min_DCT_v_scaled_size = 14 as i32
    } else if (*cinfo).scale_num.wrapping_mul((*cinfo).block_size as u32)
        <= (*cinfo).scale_denom.wrapping_mul(15 as i32 as u32)
    {
        /* Provide 15/block_size scaling */
        (*cinfo).output_width = jdiv_round_up(
            (*cinfo).image_width as isize * 15 as isize,
            (*cinfo).block_size as isize,
        ) as JDIMENSION;
        (*cinfo).output_height = jdiv_round_up(
            (*cinfo).image_height as isize * 15 as isize,
            (*cinfo).block_size as isize,
        ) as JDIMENSION;
        (*cinfo).min_DCT_h_scaled_size = 15 as i32;
        (*cinfo).min_DCT_v_scaled_size = 15 as i32
    } else {
        /* Provide 16/block_size scaling */
        (*cinfo).output_width = jdiv_round_up(
            (*cinfo).image_width as isize * 16 as isize,
            (*cinfo).block_size as isize,
        ) as JDIMENSION;
        (*cinfo).output_height = jdiv_round_up(
            (*cinfo).image_height as isize * 16 as isize,
            (*cinfo).block_size as isize,
        ) as JDIMENSION;
        (*cinfo).min_DCT_h_scaled_size = 16 as i32;
        (*cinfo).min_DCT_v_scaled_size = 16 as i32
    }
    /* Recompute dimensions of components */
    ci = 0 as i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        (*compptr).DCT_h_scaled_size = (*cinfo).min_DCT_h_scaled_size;
        (*compptr).DCT_v_scaled_size = (*cinfo).min_DCT_v_scaled_size;
        ci += 1;
        compptr = compptr.offset(1)
    }
    /* !IDCT_SCALING_SUPPORTED */
    /* IDCT_SCALING_SUPPORTED */
}

unsafe extern "C" fn initial_setup(mut cinfo: j_decompress_ptr)
/* Called once, when first SOS marker is reached */
{
    let mut ci: i32 = 0;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    /* Make sure image isn't bigger than I can handle */
    if (*cinfo).image_height as isize > 65500 as isize
        || (*cinfo).image_width as isize > 65500 as isize
    {
        (*(*cinfo).err).msg_code = JERR_IMAGE_TOO_BIG as i32;
        (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = 65500 as isize as u32 as i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* For now, precision must match compiled-in value... */
    if (*cinfo).data_precision != 8 as i32 {
        (*(*cinfo).err).msg_code = JERR_BAD_PRECISION as i32;
        (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = (*cinfo).data_precision;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* Check that number of components won't exceed internal array sizes */
    if (*cinfo).num_components > 10 as i32 {
        (*(*cinfo).err).msg_code = JERR_COMPONENT_COUNT as i32;
        (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = (*cinfo).num_components;
        (*(*cinfo).err).msg_parm.i[1 as i32 as usize] = 10 as i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* Compute maximum sampling factors; check factor validity */
    (*cinfo).max_h_samp_factor = 1 as i32;
    (*cinfo).max_v_samp_factor = 1 as i32;
    ci = 0 as i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        if (*compptr).h_samp_factor <= 0 as i32
            || (*compptr).h_samp_factor > 4 as i32
            || (*compptr).v_samp_factor <= 0 as i32
            || (*compptr).v_samp_factor > 4 as i32
        {
            (*(*cinfo).err).msg_code = JERR_BAD_SAMPLING as i32;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        (*cinfo).max_h_samp_factor = if (*cinfo).max_h_samp_factor > (*compptr).h_samp_factor {
            (*cinfo).max_h_samp_factor
        } else {
            (*compptr).h_samp_factor
        };
        (*cinfo).max_v_samp_factor = if (*cinfo).max_v_samp_factor > (*compptr).v_samp_factor {
            (*cinfo).max_v_samp_factor
        } else {
            (*compptr).v_samp_factor
        };
        ci += 1;
        compptr = compptr.offset(1)
    }
    /* Derive block_size, natural_order, and lim_Se */
    if (*cinfo).is_baseline != 0 || (*cinfo).progressive_mode != 0 && (*cinfo).comps_in_scan != 0 {
        /* no pseudo SOS marker */
        (*cinfo).block_size = 8 as i32; /* not needed */
        (*cinfo).natural_order = jpeg_natural_order.as_ptr();
        (*cinfo).lim_Se = 64 as i32 - 1 as i32
    } else {
        match (*cinfo).Se {
            0 => {
                (*cinfo).block_size = 1 as i32;
                (*cinfo).natural_order = jpeg_natural_order.as_ptr();
                (*cinfo).lim_Se = (*cinfo).Se
            }
            3 => {
                (*cinfo).block_size = 2 as i32;
                (*cinfo).natural_order = jpeg_natural_order2.as_ptr();
                (*cinfo).lim_Se = (*cinfo).Se
            }
            8 => {
                (*cinfo).block_size = 3 as i32;
                (*cinfo).natural_order = jpeg_natural_order3.as_ptr();
                (*cinfo).lim_Se = (*cinfo).Se
            }
            15 => {
                (*cinfo).block_size = 4 as i32;
                (*cinfo).natural_order = jpeg_natural_order4.as_ptr();
                (*cinfo).lim_Se = (*cinfo).Se
            }
            24 => {
                (*cinfo).block_size = 5 as i32;
                (*cinfo).natural_order = jpeg_natural_order5.as_ptr();
                (*cinfo).lim_Se = (*cinfo).Se
            }
            35 => {
                (*cinfo).block_size = 6 as i32;
                (*cinfo).natural_order = jpeg_natural_order6.as_ptr();
                (*cinfo).lim_Se = (*cinfo).Se
            }
            48 => {
                (*cinfo).block_size = 7 as i32;
                (*cinfo).natural_order = jpeg_natural_order7.as_ptr();
                (*cinfo).lim_Se = (*cinfo).Se
            }
            63 => {
                (*cinfo).block_size = 8 as i32;
                (*cinfo).natural_order = jpeg_natural_order.as_ptr();
                (*cinfo).lim_Se = 64 as i32 - 1 as i32
            }
            80 => {
                (*cinfo).block_size = 9 as i32;
                (*cinfo).natural_order = jpeg_natural_order.as_ptr();
                (*cinfo).lim_Se = 64 as i32 - 1 as i32
            }
            99 => {
                (*cinfo).block_size = 10 as i32;
                (*cinfo).natural_order = jpeg_natural_order.as_ptr();
                (*cinfo).lim_Se = 64 as i32 - 1 as i32
            }
            120 => {
                (*cinfo).block_size = 11 as i32;
                (*cinfo).natural_order = jpeg_natural_order.as_ptr();
                (*cinfo).lim_Se = 64 as i32 - 1 as i32
            }
            143 => {
                (*cinfo).block_size = 12 as i32;
                (*cinfo).natural_order = jpeg_natural_order.as_ptr();
                (*cinfo).lim_Se = 64 as i32 - 1 as i32
            }
            168 => {
                (*cinfo).block_size = 13 as i32;
                (*cinfo).natural_order = jpeg_natural_order.as_ptr();
                (*cinfo).lim_Se = 64 as i32 - 1 as i32
            }
            195 => {
                (*cinfo).block_size = 14 as i32;
                (*cinfo).natural_order = jpeg_natural_order.as_ptr();
                (*cinfo).lim_Se = 64 as i32 - 1 as i32
            }
            224 => {
                (*cinfo).block_size = 15 as i32;
                (*cinfo).natural_order = jpeg_natural_order.as_ptr();
                (*cinfo).lim_Se = 64 as i32 - 1 as i32
            }
            255 => {
                (*cinfo).block_size = 16 as i32;
                (*cinfo).natural_order = jpeg_natural_order.as_ptr();
                (*cinfo).lim_Se = 64 as i32 - 1 as i32
            }
            _ => {
                (*(*cinfo).err).msg_code = JERR_BAD_PROGRESSION as i32;
                (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = (*cinfo).Ss;
                (*(*cinfo).err).msg_parm.i[1 as i32 as usize] = (*cinfo).Se;
                (*(*cinfo).err).msg_parm.i[2 as i32 as usize] = (*cinfo).Ah;
                (*(*cinfo).err).msg_parm.i[3 as i32 as usize] = (*cinfo).Al;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
    }
    /* We initialize DCT_scaled_size and min_DCT_scaled_size to block_size.
     * In the full decompressor,
     * this will be overridden by jpeg_calc_output_dimensions in jdmaster.c;
     * but in the transcoder,
     * jpeg_calc_output_dimensions is not used, so we must do it here.
     */
    (*cinfo).min_DCT_h_scaled_size = (*cinfo).block_size;
    (*cinfo).min_DCT_v_scaled_size = (*cinfo).block_size;
    /* Compute dimensions of components */
    ci = 0 as i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        (*compptr).DCT_h_scaled_size = (*cinfo).block_size;
        (*compptr).DCT_v_scaled_size = (*cinfo).block_size;
        /* Size in DCT blocks */
        (*compptr).width_in_blocks = jdiv_round_up(
            (*cinfo).image_width as isize * (*compptr).h_samp_factor as isize,
            ((*cinfo).max_h_samp_factor * (*cinfo).block_size) as isize,
        ) as JDIMENSION;
        (*compptr).height_in_blocks = jdiv_round_up(
            (*cinfo).image_height as isize * (*compptr).v_samp_factor as isize,
            ((*cinfo).max_v_samp_factor * (*cinfo).block_size) as isize,
        ) as JDIMENSION;
        /* downsampled_width and downsampled_height will also be overridden by
         * jdmaster.c if we are doing full decompression.  The transcoder library
         * doesn't use these values, but the calling application might.
         */
        /* Size in samples */
        (*compptr).downsampled_width = jdiv_round_up(
            (*cinfo).image_width as isize * (*compptr).h_samp_factor as isize,
            (*cinfo).max_h_samp_factor as isize,
        ) as JDIMENSION;
        (*compptr).downsampled_height = jdiv_round_up(
            (*cinfo).image_height as isize * (*compptr).v_samp_factor as isize,
            (*cinfo).max_v_samp_factor as isize,
        ) as JDIMENSION;
        /* Mark component needed, until color conversion says otherwise */
        (*compptr).component_needed = 1 as i32;
        /* Mark no quantization table yet saved for component */
        (*compptr).quant_table = 0 as *mut JQUANT_TBL;
        ci += 1;
        compptr = compptr.offset(1)
    }
    /* Compute number of fully interleaved MCU rows. */
    (*cinfo).total_iMCU_rows = jdiv_round_up(
        (*cinfo).image_height as isize,
        ((*cinfo).max_v_samp_factor * (*cinfo).block_size) as isize,
    ) as JDIMENSION;
    /* Decide whether file contains multiple scans */
    if (*cinfo).comps_in_scan < (*cinfo).num_components || (*cinfo).progressive_mode != 0 {
        (*(*cinfo).inputctl).has_multiple_scans = 1 as i32
    } else {
        (*(*cinfo).inputctl).has_multiple_scans = 0 as i32
    };
}

unsafe extern "C" fn per_scan_setup(mut cinfo: j_decompress_ptr)
/* Do computations that are needed before processing a JPEG scan */
/* cinfo->comps_in_scan and cinfo->cur_comp_info[] were set from SOS marker */
{
    let mut ci: i32 = 0;
    let mut mcublks: i32 = 0;
    let mut tmp: i32 = 0;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    if (*cinfo).comps_in_scan == 1 as i32 {
        /* Noninterleaved (single-component) scan */
        compptr = (*cinfo).cur_comp_info[0 as i32 as usize];
        /* Overall image size in MCUs */
        (*cinfo).MCUs_per_row = (*compptr).width_in_blocks;
        (*cinfo).MCU_rows_in_scan = (*compptr).height_in_blocks;
        /* For noninterleaved scan, always one block per MCU */
        (*compptr).MCU_width = 1 as i32;
        (*compptr).MCU_height = 1 as i32;
        (*compptr).MCU_blocks = 1 as i32;
        (*compptr).MCU_sample_width = (*compptr).DCT_h_scaled_size;
        (*compptr).last_col_width = 1 as i32;
        /* For noninterleaved scans, it is convenient to define last_row_height
         * as the number of block rows present in the last iMCU row.
         */
        tmp = (*compptr)
            .height_in_blocks
            .wrapping_rem((*compptr).v_samp_factor as u32) as i32;
        if tmp == 0 as i32 {
            tmp = (*compptr).v_samp_factor
        }
        (*compptr).last_row_height = tmp;
        /* Prepare array describing MCU composition */
        (*cinfo).blocks_in_MCU = 1 as i32;
        (*cinfo).MCU_membership[0 as i32 as usize] = 0 as i32
    } else {
        /* Interleaved (multi-component) scan */
        if (*cinfo).comps_in_scan <= 0 as i32 || (*cinfo).comps_in_scan > 4 as i32 {
            (*(*cinfo).err).msg_code = JERR_COMPONENT_COUNT as i32;
            (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = (*cinfo).comps_in_scan;
            (*(*cinfo).err).msg_parm.i[1 as i32 as usize] = 4 as i32;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        /* Overall image size in MCUs */
        (*cinfo).MCUs_per_row = jdiv_round_up(
            (*cinfo).image_width as isize,
            ((*cinfo).max_h_samp_factor * (*cinfo).block_size) as isize,
        ) as JDIMENSION;
        (*cinfo).MCU_rows_in_scan = jdiv_round_up(
            (*cinfo).image_height as isize,
            ((*cinfo).max_v_samp_factor * (*cinfo).block_size) as isize,
        ) as JDIMENSION;
        (*cinfo).blocks_in_MCU = 0 as i32;
        ci = 0 as i32;
        while ci < (*cinfo).comps_in_scan {
            compptr = (*cinfo).cur_comp_info[ci as usize];
            /* Sampling factors give # of blocks of component in each MCU */
            (*compptr).MCU_width = (*compptr).h_samp_factor;
            (*compptr).MCU_height = (*compptr).v_samp_factor;
            (*compptr).MCU_blocks = (*compptr).MCU_width * (*compptr).MCU_height;
            (*compptr).MCU_sample_width = (*compptr).MCU_width * (*compptr).DCT_h_scaled_size;
            /* Figure number of non-dummy blocks in last MCU column & row */
            tmp = (*compptr)
                .width_in_blocks
                .wrapping_rem((*compptr).MCU_width as u32) as i32;
            if tmp == 0 as i32 {
                tmp = (*compptr).MCU_width
            }
            (*compptr).last_col_width = tmp;
            tmp = (*compptr)
                .height_in_blocks
                .wrapping_rem((*compptr).MCU_height as u32) as i32;
            if tmp == 0 as i32 {
                tmp = (*compptr).MCU_height
            }
            (*compptr).last_row_height = tmp;
            /* Prepare array describing MCU composition */
            mcublks = (*compptr).MCU_blocks;
            if (*cinfo).blocks_in_MCU + mcublks > 10 as i32 {
                (*(*cinfo).err).msg_code = JERR_BAD_MCU_SIZE as i32;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            loop {
                let fresh0 = mcublks;
                mcublks = mcublks - 1;
                if !(fresh0 > 0 as i32) {
                    break;
                }
                let fresh1 = (*cinfo).blocks_in_MCU;
                (*cinfo).blocks_in_MCU = (*cinfo).blocks_in_MCU + 1;
                (*cinfo).MCU_membership[fresh1 as usize] = ci
            }
            ci += 1
        }
    };
}
/*
 * Save away a copy of the Q-table referenced by each component present
 * in the current scan, unless already saved during a prior scan.
 *
 * In a multiple-scan JPEG file, the encoder could assign different components
 * the same Q-table slot number, but change table definitions between scans
 * so that each component uses a different Q-table.  (The IJG encoder is not
 * currently capable of doing this, but other encoders might.)  Since we want
 * to be able to dequantize all the components at the end of the file, this
 * means that we have to save away the table actually used for each component.
 * We do this by copying the table at the start of the first scan containing
 * the component.
 * The JPEG spec prohibits the encoder from changing the contents of a Q-table
 * slot between scans of a component using that slot.  If the encoder does so
 * anyway, this decoder will simply use the Q-table values that were current
 * at the start of the first scan for the component.
 *
 * The decompressor output side looks only at the saved quant tables,
 * not at the current Q-table slots.
 */

unsafe extern "C" fn latch_quant_tables(mut cinfo: j_decompress_ptr) {
    let mut ci: i32 = 0;
    let mut qtblno: i32 = 0;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    let mut qtbl: *mut JQUANT_TBL = 0 as *mut JQUANT_TBL;
    ci = 0 as i32;
    while ci < (*cinfo).comps_in_scan {
        compptr = (*cinfo).cur_comp_info[ci as usize];
        /* No work if we already saved Q-table for this component */
        if (*compptr).quant_table.is_null() {
            /* Make sure specified quantization table is present */
            qtblno = (*compptr).quant_tbl_no;
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
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            /* OK, save away the quantization table */
            qtbl = Some(
                (*(*cinfo).mem)
                    .alloc_small
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr,
                1 as i32,
                ::std::mem::size_of::<JQUANT_TBL>() as usize,
            ) as *mut JQUANT_TBL;
            crate::stdlib::memcpy(
                qtbl as *mut libc::c_void,
                (*cinfo).quant_tbl_ptrs[qtblno as usize] as *const libc::c_void,
                ::std::mem::size_of::<JQUANT_TBL>() as usize,
            );
            (*compptr).quant_table = qtbl
        }
        ci += 1
    }
}
/*
 * Initialize the input modules to read a scan of compressed data.
 * The first call to this is done by jdmaster.c after initializing
 * the entire decompressor (during jpeg_start_decompress).
 * Subsequent calls come from consume_markers, below.
 */

unsafe extern "C" fn start_input_pass(mut cinfo: j_decompress_ptr) {
    per_scan_setup(cinfo);
    latch_quant_tables(cinfo);
    Some(
        (*(*cinfo).entropy)
            .start_pass
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo);
    Some(
        (*(*cinfo).coef)
            .start_input_pass
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo);
    (*(*cinfo).inputctl).consume_input = (*(*cinfo).coef).consume_data;
}
/*
 * Finish up after inputting a compressed-data scan.
 * This is called by the coefficient controller after it's read all
 * the expected data of the scan.
 */

unsafe extern "C" fn finish_input_pass(mut cinfo: j_decompress_ptr) {
    (*(*cinfo).inputctl).consume_input =
        Some(consume_markers as unsafe extern "C" fn(_: j_decompress_ptr) -> i32);
}
/* Forward declarations */
/*
 * Read JPEG markers before, between, or after compressed-data scans.
 * Change state as necessary when a new scan is reached.
 * Return value is JPEG_SUSPENDED, JPEG_REACHED_SOS, or JPEG_REACHED_EOI.
 *
 * The consume_input method pointer points either here or to the
 * coefficient controller's consume_data routine, depending on whether
 * we are reading a compressed data segment or inter-segment markers.
 *
 * Note: This function should NOT return a pseudo SOS marker (with zero
 * component number) to the caller.  A pseudo marker received by
 * read_markers is processed and then skipped for other markers.
 */

unsafe extern "C" fn consume_markers(mut cinfo: j_decompress_ptr) -> i32 {
    let mut inputctl: my_inputctl_ptr = (*cinfo).inputctl as my_inputctl_ptr;
    let mut val: i32 = 0;
    if (*inputctl).pub_0.eoi_reached != 0 {
        /* After hitting EOI, read no further */
        return 2 as i32;
    }
    loop {
        /* Loop to pass pseudo SOS marker */
        val = Some(
            (*(*cinfo).marker)
                .read_markers
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
        let mut current_block_21: u64;
        match val {
            1 => {
                /* Found SOS */
                if (*inputctl).inheaders != 0 {
                    /* 1st SOS */
                    if (*inputctl).inheaders == 1 as i32 {
                        initial_setup(cinfo);
                    }
                    if (*cinfo).comps_in_scan == 0 as i32 {
                        /* Note: start_input_pass must be called by jdmaster.c
                         * before any more input can be consumed.  jdapimin.c is
                         * responsible for enforcing this sequencing.
                         */
                        /* pseudo SOS marker */
                        (*inputctl).inheaders = 2 as i32; /* Oops, I wasn't expecting this! */
                        current_block_21 = 3437258052017859086;
                    } else {
                        (*inputctl).inheaders = 0 as i32;
                        current_block_21 = 15652330335145281839;
                    }
                } else {
                    if (*inputctl).pub_0.has_multiple_scans == 0 {
                        (*(*cinfo).err).msg_code = JERR_EOI_EXPECTED as i32;
                        Some(
                            (*(*cinfo).err)
                                .error_exit
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            cinfo as j_common_ptr
                        );
                    }
                    if (*cinfo).comps_in_scan == 0 as i32 {
                        current_block_21 = 3437258052017859086;
                    } else {
                        start_input_pass(cinfo);
                        current_block_21 = 15652330335145281839;
                    }
                }
                match current_block_21 {
                    15652330335145281839 => return val,
                    _ =>
                        /* unexpected pseudo SOS marker */
                        {}
                }
            }
            2 => {
                /* Found EOI */
                (*inputctl).pub_0.eoi_reached = 1 as i32;
                if (*inputctl).inheaders != 0 {
                    /* Tables-only datastream, apparently */
                    if (*(*cinfo).marker).saw_SOF != 0 {
                        (*(*cinfo).err).msg_code = JERR_SOF_NO_SOS as i32;
                        Some(
                            (*(*cinfo).err)
                                .error_exit
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            cinfo as j_common_ptr
                        );
                    }
                } else if (*cinfo).output_scan_number > (*cinfo).input_scan_number {
                    (*cinfo).output_scan_number = (*cinfo).input_scan_number
                }
                return val;
            }
            0 => return val,
            _ => return val,
        }
    }
}
/* Prevent infinite loop in coef ctlr's decompress_data routine
 * if user set output_scan_number larger than number of scans.
 */
/*
 * Reset state to begin a fresh datastream.
 */

unsafe extern "C" fn reset_input_controller(mut cinfo: j_decompress_ptr) {
    let mut inputctl: my_inputctl_ptr = (*cinfo).inputctl as my_inputctl_ptr; /* "unknown" would be better */
    (*inputctl).pub_0.consume_input =
        Some(consume_markers as unsafe extern "C" fn(_: j_decompress_ptr) -> i32);
    (*inputctl).pub_0.has_multiple_scans = 0 as i32;
    (*inputctl).pub_0.eoi_reached = 0 as i32;
    (*inputctl).inheaders = 1 as i32;
    /* Reset other modules */
    Some(
        (*(*cinfo).err)
            .reset_error_mgr
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo as j_common_ptr);
    Some(
        (*(*cinfo).marker)
            .reset_marker_reader
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo);
    /* Reset progression state -- would be cleaner if entropy decoder did this */
    (*cinfo).coef_bits = 0 as *mut [i32; 64];
}
/* It is useful to allow each component to have a separate IDCT method. */
/* Upsampling (note that upsampler must also call color converter) */
/* TRUE if need rows above & below */
/* Colorspace conversion */
/* Color quantization or color precision reduction */
/* Miscellaneous useful macros */
/* We assume that right shift corresponds to signed division by 2 with
 * rounding towards minus infinity.  This is correct for typical "arithmetic
 * shift" instructions that shift in copies of the sign bit.  But some
 * C compilers implement >> with an unsigned shift.  For these machines you
 * must define RIGHT_SHIFT_IS_UNSIGNED.
 * RIGHT_SHIFT provides a proper signed right shift of an INT32 quantity.
 * It is only applied with constant shift counts.  SHIFT_TEMPS must be
 * included in the variables of any routine using RIGHT_SHIFT.
 */
/* Short forms of external names for systems with brain-damaged linkers. */
/* NEED_SHORT_EXTERNAL_NAMES */
/* Compression module initialization routines */
/* Decompression module initialization routines */
/*
 * Initialize the input controller module.
 * This is called only once, when the decompression object is created.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_input_controller(mut cinfo: j_decompress_ptr) {
    let mut inputctl: my_inputctl_ptr = 0 as *mut my_input_controller;
    /* Create subobject in permanent pool */
    inputctl = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        0 as i32,
        ::std::mem::size_of::<my_input_controller>() as usize,
    ) as my_inputctl_ptr;
    (*cinfo).inputctl = inputctl as *mut jpeg_input_controller;
    /* Initialize method pointers */
    (*inputctl).pub_0.consume_input =
        Some(consume_markers as unsafe extern "C" fn(_: j_decompress_ptr) -> i32);
    (*inputctl).pub_0.reset_input_controller =
        Some(reset_input_controller as unsafe extern "C" fn(_: j_decompress_ptr) -> ());
    (*inputctl).pub_0.start_input_pass =
        Some(start_input_pass as unsafe extern "C" fn(_: j_decompress_ptr) -> ());
    (*inputctl).pub_0.finish_input_pass =
        Some(finish_input_pass as unsafe extern "C" fn(_: j_decompress_ptr) -> ());
    /* Initialize state: can't use reset_input_controller since we don't
     * want to try to reset other modules yet.
     */
    (*inputctl).pub_0.has_multiple_scans = 0 as i32; /* "unknown" would be better */
    (*inputctl).pub_0.eoi_reached = 0 as i32;
    (*inputctl).inheaders = 1 as i32;
}
