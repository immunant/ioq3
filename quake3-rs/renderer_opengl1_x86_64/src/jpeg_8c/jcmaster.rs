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
pub use crate::src::jpeg_8c::jutils::jdiv_round_up;
pub use crate::src::jpeg_8c::jutils::jpeg_natural_order;
pub use crate::src::jpeg_8c::jutils::jpeg_natural_order2;
pub use crate::src::jpeg_8c::jutils::jpeg_natural_order3;
pub use crate::src::jpeg_8c::jutils::jpeg_natural_order4;
pub use crate::src::jpeg_8c::jutils::jpeg_natural_order5;
pub use crate::src::jpeg_8c::jutils::jpeg_natural_order6;
pub use crate::src::jpeg_8c::jutils::jpeg_natural_order7;

pub type my_master_ptr = *mut my_comp_master;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_comp_master {
    pub pub_0: crate::jpegint_h::jpeg_comp_master,
    pub pass_type: c_pass_type,
    pub pass_number: i32,
    pub total_passes: i32,
    pub scan_number: i32,
}

pub type c_pass_type = u32;

pub const output_pass: c_pass_type = 2;

pub const huff_opt_pass: c_pass_type = 1;

pub const main_pass: c_pass_type = 0;
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
/*
 * Support routines that do various essential calculations.
 */
/*
 * Compute JPEG image dimensions and related values.
 * NOTE: this is exported for possible use by application.
 * Hence it mustn't do anything that can't be done twice.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_calc_jpeg_dimensions(mut cinfo: crate::jpeglib_h::j_compress_ptr)
/* Do computations that are needed before master selection phase */
{
    /* Sanity check on input image dimensions to prevent overflow in
     * following calculation.
     * We do check jpeg_width and jpeg_height in initial_setup below,
     * but image_width and image_height can come from arbitrary data,
     * and we need some space for multiplication by block_size.
     */
    if (*cinfo).image_width as isize >> 24 as i32 != 0
        || (*cinfo).image_height as isize >> 24 as i32 != 0
    {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_IMAGE_TOO_BIG as i32;
        (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = 65500 as isize as u32 as i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Compute actual JPEG image dimensions and DCT scaling choices. */
    if (*cinfo).scale_num
        >= (*cinfo)
            .scale_denom
            .wrapping_mul((*cinfo).block_size as u32)
    {
        /* Provide block_size/1 scaling */
        (*cinfo).jpeg_width = (*cinfo)
            .image_width
            .wrapping_mul((*cinfo).block_size as u32);
        (*cinfo).jpeg_height = (*cinfo)
            .image_height
            .wrapping_mul((*cinfo).block_size as u32);
        (*cinfo).min_DCT_h_scaled_size = 1 as i32;
        (*cinfo).min_DCT_v_scaled_size = 1 as i32
    } else if (*cinfo).scale_num.wrapping_mul(2 as i32 as u32)
        >= (*cinfo)
            .scale_denom
            .wrapping_mul((*cinfo).block_size as u32)
    {
        /* Provide block_size/2 scaling */
        (*cinfo).jpeg_width = crate::src::jpeg_8c::jutils::jdiv_round_up(
            (*cinfo).image_width as isize * (*cinfo).block_size as isize,
            2 as isize,
        ) as crate::jmorecfg_h::JDIMENSION;
        (*cinfo).jpeg_height = crate::src::jpeg_8c::jutils::jdiv_round_up(
            (*cinfo).image_height as isize * (*cinfo).block_size as isize,
            2 as isize,
        ) as crate::jmorecfg_h::JDIMENSION;
        (*cinfo).min_DCT_h_scaled_size = 2 as i32;
        (*cinfo).min_DCT_v_scaled_size = 2 as i32
    } else if (*cinfo).scale_num.wrapping_mul(3 as i32 as u32)
        >= (*cinfo)
            .scale_denom
            .wrapping_mul((*cinfo).block_size as u32)
    {
        /* Provide block_size/3 scaling */
        (*cinfo).jpeg_width = crate::src::jpeg_8c::jutils::jdiv_round_up(
            (*cinfo).image_width as isize * (*cinfo).block_size as isize,
            3 as isize,
        ) as crate::jmorecfg_h::JDIMENSION;
        (*cinfo).jpeg_height = crate::src::jpeg_8c::jutils::jdiv_round_up(
            (*cinfo).image_height as isize * (*cinfo).block_size as isize,
            3 as isize,
        ) as crate::jmorecfg_h::JDIMENSION;
        (*cinfo).min_DCT_h_scaled_size = 3 as i32;
        (*cinfo).min_DCT_v_scaled_size = 3 as i32
    } else if (*cinfo).scale_num.wrapping_mul(4 as i32 as u32)
        >= (*cinfo)
            .scale_denom
            .wrapping_mul((*cinfo).block_size as u32)
    {
        /* Provide block_size/4 scaling */
        (*cinfo).jpeg_width = crate::src::jpeg_8c::jutils::jdiv_round_up(
            (*cinfo).image_width as isize * (*cinfo).block_size as isize,
            4 as isize,
        ) as crate::jmorecfg_h::JDIMENSION;
        (*cinfo).jpeg_height = crate::src::jpeg_8c::jutils::jdiv_round_up(
            (*cinfo).image_height as isize * (*cinfo).block_size as isize,
            4 as isize,
        ) as crate::jmorecfg_h::JDIMENSION;
        (*cinfo).min_DCT_h_scaled_size = 4 as i32;
        (*cinfo).min_DCT_v_scaled_size = 4 as i32
    } else if (*cinfo).scale_num.wrapping_mul(5 as i32 as u32)
        >= (*cinfo)
            .scale_denom
            .wrapping_mul((*cinfo).block_size as u32)
    {
        /* Provide block_size/5 scaling */
        (*cinfo).jpeg_width = crate::src::jpeg_8c::jutils::jdiv_round_up(
            (*cinfo).image_width as isize * (*cinfo).block_size as isize,
            5 as isize,
        ) as crate::jmorecfg_h::JDIMENSION;
        (*cinfo).jpeg_height = crate::src::jpeg_8c::jutils::jdiv_round_up(
            (*cinfo).image_height as isize * (*cinfo).block_size as isize,
            5 as isize,
        ) as crate::jmorecfg_h::JDIMENSION;
        (*cinfo).min_DCT_h_scaled_size = 5 as i32;
        (*cinfo).min_DCT_v_scaled_size = 5 as i32
    } else if (*cinfo).scale_num.wrapping_mul(6 as i32 as u32)
        >= (*cinfo)
            .scale_denom
            .wrapping_mul((*cinfo).block_size as u32)
    {
        /* Provide block_size/6 scaling */
        (*cinfo).jpeg_width = crate::src::jpeg_8c::jutils::jdiv_round_up(
            (*cinfo).image_width as isize * (*cinfo).block_size as isize,
            6 as isize,
        ) as crate::jmorecfg_h::JDIMENSION;
        (*cinfo).jpeg_height = crate::src::jpeg_8c::jutils::jdiv_round_up(
            (*cinfo).image_height as isize * (*cinfo).block_size as isize,
            6 as isize,
        ) as crate::jmorecfg_h::JDIMENSION;
        (*cinfo).min_DCT_h_scaled_size = 6 as i32;
        (*cinfo).min_DCT_v_scaled_size = 6 as i32
    } else if (*cinfo).scale_num.wrapping_mul(7 as i32 as u32)
        >= (*cinfo)
            .scale_denom
            .wrapping_mul((*cinfo).block_size as u32)
    {
        /* Provide block_size/7 scaling */
        (*cinfo).jpeg_width = crate::src::jpeg_8c::jutils::jdiv_round_up(
            (*cinfo).image_width as isize * (*cinfo).block_size as isize,
            7 as isize,
        ) as crate::jmorecfg_h::JDIMENSION;
        (*cinfo).jpeg_height = crate::src::jpeg_8c::jutils::jdiv_round_up(
            (*cinfo).image_height as isize * (*cinfo).block_size as isize,
            7 as isize,
        ) as crate::jmorecfg_h::JDIMENSION;
        (*cinfo).min_DCT_h_scaled_size = 7 as i32;
        (*cinfo).min_DCT_v_scaled_size = 7 as i32
    } else if (*cinfo).scale_num.wrapping_mul(8 as i32 as u32)
        >= (*cinfo)
            .scale_denom
            .wrapping_mul((*cinfo).block_size as u32)
    {
        /* Provide block_size/8 scaling */
        (*cinfo).jpeg_width = crate::src::jpeg_8c::jutils::jdiv_round_up(
            (*cinfo).image_width as isize * (*cinfo).block_size as isize,
            8 as isize,
        ) as crate::jmorecfg_h::JDIMENSION;
        (*cinfo).jpeg_height = crate::src::jpeg_8c::jutils::jdiv_round_up(
            (*cinfo).image_height as isize * (*cinfo).block_size as isize,
            8 as isize,
        ) as crate::jmorecfg_h::JDIMENSION;
        (*cinfo).min_DCT_h_scaled_size = 8 as i32;
        (*cinfo).min_DCT_v_scaled_size = 8 as i32
    } else if (*cinfo).scale_num.wrapping_mul(9 as i32 as u32)
        >= (*cinfo)
            .scale_denom
            .wrapping_mul((*cinfo).block_size as u32)
    {
        /* Provide block_size/9 scaling */
        (*cinfo).jpeg_width = crate::src::jpeg_8c::jutils::jdiv_round_up(
            (*cinfo).image_width as isize * (*cinfo).block_size as isize,
            9 as isize,
        ) as crate::jmorecfg_h::JDIMENSION;
        (*cinfo).jpeg_height = crate::src::jpeg_8c::jutils::jdiv_round_up(
            (*cinfo).image_height as isize * (*cinfo).block_size as isize,
            9 as isize,
        ) as crate::jmorecfg_h::JDIMENSION;
        (*cinfo).min_DCT_h_scaled_size = 9 as i32;
        (*cinfo).min_DCT_v_scaled_size = 9 as i32
    } else if (*cinfo).scale_num.wrapping_mul(10 as i32 as u32)
        >= (*cinfo)
            .scale_denom
            .wrapping_mul((*cinfo).block_size as u32)
    {
        /* Provide block_size/10 scaling */
        (*cinfo).jpeg_width = crate::src::jpeg_8c::jutils::jdiv_round_up(
            (*cinfo).image_width as isize * (*cinfo).block_size as isize,
            10 as isize,
        ) as crate::jmorecfg_h::JDIMENSION;
        (*cinfo).jpeg_height = crate::src::jpeg_8c::jutils::jdiv_round_up(
            (*cinfo).image_height as isize * (*cinfo).block_size as isize,
            10 as isize,
        ) as crate::jmorecfg_h::JDIMENSION;
        (*cinfo).min_DCT_h_scaled_size = 10 as i32;
        (*cinfo).min_DCT_v_scaled_size = 10 as i32
    } else if (*cinfo).scale_num.wrapping_mul(11 as i32 as u32)
        >= (*cinfo)
            .scale_denom
            .wrapping_mul((*cinfo).block_size as u32)
    {
        /* Provide block_size/11 scaling */
        (*cinfo).jpeg_width = crate::src::jpeg_8c::jutils::jdiv_round_up(
            (*cinfo).image_width as isize * (*cinfo).block_size as isize,
            11 as isize,
        ) as crate::jmorecfg_h::JDIMENSION;
        (*cinfo).jpeg_height = crate::src::jpeg_8c::jutils::jdiv_round_up(
            (*cinfo).image_height as isize * (*cinfo).block_size as isize,
            11 as isize,
        ) as crate::jmorecfg_h::JDIMENSION;
        (*cinfo).min_DCT_h_scaled_size = 11 as i32;
        (*cinfo).min_DCT_v_scaled_size = 11 as i32
    } else if (*cinfo).scale_num.wrapping_mul(12 as i32 as u32)
        >= (*cinfo)
            .scale_denom
            .wrapping_mul((*cinfo).block_size as u32)
    {
        /* Provide block_size/12 scaling */
        (*cinfo).jpeg_width = crate::src::jpeg_8c::jutils::jdiv_round_up(
            (*cinfo).image_width as isize * (*cinfo).block_size as isize,
            12 as isize,
        ) as crate::jmorecfg_h::JDIMENSION;
        (*cinfo).jpeg_height = crate::src::jpeg_8c::jutils::jdiv_round_up(
            (*cinfo).image_height as isize * (*cinfo).block_size as isize,
            12 as isize,
        ) as crate::jmorecfg_h::JDIMENSION;
        (*cinfo).min_DCT_h_scaled_size = 12 as i32;
        (*cinfo).min_DCT_v_scaled_size = 12 as i32
    } else if (*cinfo).scale_num.wrapping_mul(13 as i32 as u32)
        >= (*cinfo)
            .scale_denom
            .wrapping_mul((*cinfo).block_size as u32)
    {
        /* Provide block_size/13 scaling */
        (*cinfo).jpeg_width = crate::src::jpeg_8c::jutils::jdiv_round_up(
            (*cinfo).image_width as isize * (*cinfo).block_size as isize,
            13 as isize,
        ) as crate::jmorecfg_h::JDIMENSION;
        (*cinfo).jpeg_height = crate::src::jpeg_8c::jutils::jdiv_round_up(
            (*cinfo).image_height as isize * (*cinfo).block_size as isize,
            13 as isize,
        ) as crate::jmorecfg_h::JDIMENSION;
        (*cinfo).min_DCT_h_scaled_size = 13 as i32;
        (*cinfo).min_DCT_v_scaled_size = 13 as i32
    } else if (*cinfo).scale_num.wrapping_mul(14 as i32 as u32)
        >= (*cinfo)
            .scale_denom
            .wrapping_mul((*cinfo).block_size as u32)
    {
        /* Provide block_size/14 scaling */
        (*cinfo).jpeg_width = crate::src::jpeg_8c::jutils::jdiv_round_up(
            (*cinfo).image_width as isize * (*cinfo).block_size as isize,
            14 as isize,
        ) as crate::jmorecfg_h::JDIMENSION;
        (*cinfo).jpeg_height = crate::src::jpeg_8c::jutils::jdiv_round_up(
            (*cinfo).image_height as isize * (*cinfo).block_size as isize,
            14 as isize,
        ) as crate::jmorecfg_h::JDIMENSION;
        (*cinfo).min_DCT_h_scaled_size = 14 as i32;
        (*cinfo).min_DCT_v_scaled_size = 14 as i32
    } else if (*cinfo).scale_num.wrapping_mul(15 as i32 as u32)
        >= (*cinfo)
            .scale_denom
            .wrapping_mul((*cinfo).block_size as u32)
    {
        /* Provide block_size/15 scaling */
        (*cinfo).jpeg_width = crate::src::jpeg_8c::jutils::jdiv_round_up(
            (*cinfo).image_width as isize * (*cinfo).block_size as isize,
            15 as isize,
        ) as crate::jmorecfg_h::JDIMENSION;
        (*cinfo).jpeg_height = crate::src::jpeg_8c::jutils::jdiv_round_up(
            (*cinfo).image_height as isize * (*cinfo).block_size as isize,
            15 as isize,
        ) as crate::jmorecfg_h::JDIMENSION;
        (*cinfo).min_DCT_h_scaled_size = 15 as i32;
        (*cinfo).min_DCT_v_scaled_size = 15 as i32
    } else {
        /* Provide block_size/16 scaling */
        (*cinfo).jpeg_width = crate::src::jpeg_8c::jutils::jdiv_round_up(
            (*cinfo).image_width as isize * (*cinfo).block_size as isize,
            16 as isize,
        ) as crate::jmorecfg_h::JDIMENSION;
        (*cinfo).jpeg_height = crate::src::jpeg_8c::jutils::jdiv_round_up(
            (*cinfo).image_height as isize * (*cinfo).block_size as isize,
            16 as isize,
        ) as crate::jmorecfg_h::JDIMENSION;
        (*cinfo).min_DCT_h_scaled_size = 16 as i32;
        (*cinfo).min_DCT_v_scaled_size = 16 as i32
    };
    /* !DCT_SCALING_SUPPORTED */
    /* DCT_SCALING_SUPPORTED */
}

unsafe extern "C" fn jpeg_calc_trans_dimensions(mut cinfo: crate::jpeglib_h::j_compress_ptr) {
    if (*cinfo).min_DCT_h_scaled_size != (*cinfo).min_DCT_v_scaled_size {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_DCTSIZE as i32;
        (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = (*cinfo).min_DCT_h_scaled_size;
        (*(*cinfo).err).msg_parm.i[1 as i32 as usize] = (*cinfo).min_DCT_v_scaled_size;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    (*cinfo).block_size = (*cinfo).min_DCT_h_scaled_size;
}

unsafe extern "C" fn initial_setup(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut transcode_only: crate::jmorecfg_h::boolean,
)
/* Do computations that are needed before master selection phase */
{
    let mut ci: i32 = 0;
    let mut ssize: i32 = 0;
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        0 as *mut crate::jpeglib_h::jpeg_component_info;
    let mut samplesperrow: isize = 0;
    let mut jd_samplesperrow: crate::jmorecfg_h::JDIMENSION = 0;
    if transcode_only != 0 {
        jpeg_calc_trans_dimensions(cinfo);
    } else {
        jpeg_calc_jpeg_dimensions(cinfo);
    }
    /* Sanity check on block_size */
    if (*cinfo).block_size < 1 as i32 || (*cinfo).block_size > 16 as i32 {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_DCTSIZE as i32;
        (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = (*cinfo).block_size;
        (*(*cinfo).err).msg_parm.i[1 as i32 as usize] = (*cinfo).block_size;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Derive natural_order from block_size */
    match (*cinfo).block_size {
        2 => (*cinfo).natural_order = crate::src::jpeg_8c::jutils::jpeg_natural_order2.as_ptr(),
        3 => (*cinfo).natural_order = crate::src::jpeg_8c::jutils::jpeg_natural_order3.as_ptr(),
        4 => (*cinfo).natural_order = crate::src::jpeg_8c::jutils::jpeg_natural_order4.as_ptr(),
        5 => (*cinfo).natural_order = crate::src::jpeg_8c::jutils::jpeg_natural_order5.as_ptr(),
        6 => (*cinfo).natural_order = crate::src::jpeg_8c::jutils::jpeg_natural_order6.as_ptr(),
        7 => (*cinfo).natural_order = crate::src::jpeg_8c::jutils::jpeg_natural_order7.as_ptr(),
        _ => (*cinfo).natural_order = crate::src::jpeg_8c::jutils::jpeg_natural_order.as_ptr(),
    }
    /* Derive lim_Se from block_size */
    (*cinfo).lim_Se = if (*cinfo).block_size < 8 as i32 {
        ((*cinfo).block_size * (*cinfo).block_size) - 1 as i32
    } else {
        (64 as i32) - 1 as i32
    };
    /* Sanity check on image dimensions */
    if (*cinfo).jpeg_height <= 0 as i32 as u32
        || (*cinfo).jpeg_width <= 0 as i32 as u32
        || (*cinfo).num_components <= 0 as i32
        || (*cinfo).input_components <= 0 as i32
    {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_EMPTY_IMAGE as i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Make sure image isn't bigger than I can handle */
    if (*cinfo).jpeg_height as isize > 65500 as isize
        || (*cinfo).jpeg_width as isize > 65500 as isize
    {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_IMAGE_TOO_BIG as i32;
        (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = 65500 as isize as u32 as i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Width of an input scanline must be representable as JDIMENSION. */
    samplesperrow =
        (*cinfo).image_width as isize * (*cinfo).input_components as isize;
    jd_samplesperrow = samplesperrow as crate::jmorecfg_h::JDIMENSION;
    if jd_samplesperrow as isize != samplesperrow {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_WIDTH_OVERFLOW as i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* For now, precision must match compiled-in value... */
    if (*cinfo).data_precision != 8 as i32 {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_PRECISION as i32;
        (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = (*cinfo).data_precision;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Check that number of components won't exceed internal array sizes */
    if (*cinfo).num_components > 10 as i32 {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_COMPONENT_COUNT as i32;
        (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = (*cinfo).num_components;
        (*(*cinfo).err).msg_parm.i[1 as i32 as usize] = 10 as i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
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
            (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_SAMPLING as i32;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
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
    /* Compute dimensions of components */
    ci = 0 as i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        /* Fill in the correct component_index value; don't rely on application */
        (*compptr).component_index = ci;
        /* In selecting the actual DCT scaling for each component, we try to
         * scale down the chroma components via DCT scaling rather than downsampling.
         * This saves time if the downsampler gets to use 1:1 scaling.
         * Note this code adapts subsampling ratios which are powers of 2.
         */
        ssize = 1 as i32;
        while (*cinfo).min_DCT_h_scaled_size * ssize
            <= (if (*cinfo).do_fancy_downsampling != 0 {
                8 as i32
            } else {
                (8 as i32) / 2 as i32
            })
            && (*cinfo).max_h_samp_factor % ((*compptr).h_samp_factor * ssize * 2 as i32)
                == 0 as i32
        {
            ssize = ssize * 2 as i32
        }
        (*compptr).DCT_h_scaled_size = (*cinfo).min_DCT_h_scaled_size * ssize;
        ssize = 1 as i32;
        while (*cinfo).min_DCT_v_scaled_size * ssize
            <= (if (*cinfo).do_fancy_downsampling != 0 {
                8 as i32
            } else {
                (8 as i32) / 2 as i32
            })
            && (*cinfo).max_v_samp_factor % ((*compptr).v_samp_factor * ssize * 2 as i32)
                == 0 as i32
        {
            ssize = ssize * 2 as i32
        }
        (*compptr).DCT_v_scaled_size = (*cinfo).min_DCT_v_scaled_size * ssize;
        /* We don't support DCT ratios larger than 2. */
        if (*compptr).DCT_h_scaled_size > (*compptr).DCT_v_scaled_size * 2 as i32 {
            (*compptr).DCT_h_scaled_size = (*compptr).DCT_v_scaled_size * 2 as i32
        } else if (*compptr).DCT_v_scaled_size > (*compptr).DCT_h_scaled_size * 2 as i32 {
            (*compptr).DCT_v_scaled_size = (*compptr).DCT_h_scaled_size * 2 as i32
        }
        /* Size in DCT blocks */
        (*compptr).width_in_blocks = crate::src::jpeg_8c::jutils::jdiv_round_up(
            (*cinfo).jpeg_width as isize * (*compptr).h_samp_factor as isize,
            ((*cinfo).max_h_samp_factor * (*cinfo).block_size) as isize,
        ) as crate::jmorecfg_h::JDIMENSION;
        (*compptr).height_in_blocks = crate::src::jpeg_8c::jutils::jdiv_round_up(
            (*cinfo).jpeg_height as isize * (*compptr).v_samp_factor as isize,
            ((*cinfo).max_v_samp_factor * (*cinfo).block_size) as isize,
        ) as crate::jmorecfg_h::JDIMENSION;
        /* Size in samples */
        (*compptr).downsampled_width = crate::src::jpeg_8c::jutils::jdiv_round_up(
            (*cinfo).jpeg_width as isize
                * ((*compptr).h_samp_factor * (*compptr).DCT_h_scaled_size) as isize,
            ((*cinfo).max_h_samp_factor * (*cinfo).block_size) as isize,
        ) as crate::jmorecfg_h::JDIMENSION;
        (*compptr).downsampled_height = crate::src::jpeg_8c::jutils::jdiv_round_up(
            (*cinfo).jpeg_height as isize
                * ((*compptr).v_samp_factor * (*compptr).DCT_v_scaled_size) as isize,
            ((*cinfo).max_v_samp_factor * (*cinfo).block_size) as isize,
        ) as crate::jmorecfg_h::JDIMENSION;
        /* Mark component needed (this flag isn't actually used for compression) */
        (*compptr).component_needed = 1 as i32;
        ci += 1;
        compptr = compptr.offset(1)
    }
    /* Compute number of fully interleaved MCU rows (number of times that
     * main controller will call coefficient controller).
     */
    (*cinfo).total_iMCU_rows = crate::src::jpeg_8c::jutils::jdiv_round_up(
        (*cinfo).jpeg_height as isize,
        ((*cinfo).max_v_samp_factor * (*cinfo).block_size) as isize,
    ) as crate::jmorecfg_h::JDIMENSION;
}

unsafe extern "C" fn validate_script(mut cinfo: crate::jpeglib_h::j_compress_ptr)
/* Verify that the scan script in cinfo->scan_info[] is valid; also
 * determine whether it uses progressive JPEG, and set cinfo->progressive_mode.
 */
{
    let mut scanptr: *const crate::jpeglib_h::jpeg_scan_info =
        0 as *const crate::jpeglib_h::jpeg_scan_info;
    let mut scanno: i32 = 0;
    let mut ncomps: i32 = 0;
    let mut ci: i32 = 0;
    let mut coefi: i32 = 0;
    let mut thisi: i32 = 0;
    let mut Ss: i32 = 0;
    let mut Se: i32 = 0;
    let mut Ah: i32 = 0;
    let mut Al: i32 = 0;
    let mut component_sent: [crate::jmorecfg_h::boolean; 10] = [0; 10];
    let mut last_bitpos_ptr: *mut i32 = 0 as *mut i32;
    let mut last_bitpos: [[i32; 64]; 10] = [[0; 64]; 10];
    /* -1 until that coefficient has been seen; then last Al for it */
    if (*cinfo).num_scans <= 0 as i32 {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_SCAN_SCRIPT as i32;
        (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = 0 as i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* For sequential JPEG, all scans must have Ss=0, Se=DCTSIZE2-1;
     * for progressive JPEG, no scan can have this.
     */
    scanptr = (*cinfo).scan_info;
    if (*scanptr).Ss != 0 as i32 || (*scanptr).Se != 64 as i32 - 1 as i32 {
        (*cinfo).progressive_mode = 1 as i32;
        last_bitpos_ptr = &mut *(*last_bitpos.as_mut_ptr().offset(0 as i32 as isize))
            .as_mut_ptr()
            .offset(0 as i32 as isize) as *mut i32;
        ci = 0 as i32;
        while ci < (*cinfo).num_components {
            coefi = 0 as i32;
            while coefi < 64 as i32 {
                let fresh0 = last_bitpos_ptr;
                last_bitpos_ptr = last_bitpos_ptr.offset(1);
                *fresh0 = -(1 as i32);
                coefi += 1
            }
            ci += 1
        }
    } else {
        (*cinfo).progressive_mode = 0 as i32;
        ci = 0 as i32;
        while ci < (*cinfo).num_components {
            component_sent[ci as usize] = 0 as i32;
            ci += 1
        }
    }
    scanno = 1 as i32;
    while scanno <= (*cinfo).num_scans {
        /* Validate component indexes */
        ncomps = (*scanptr).comps_in_scan;
        if ncomps <= 0 as i32 || ncomps > 4 as i32 {
            (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_COMPONENT_COUNT as i32;
            (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = ncomps;
            (*(*cinfo).err).msg_parm.i[1 as i32 as usize] = 4 as i32;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
        ci = 0 as i32;
        while ci < ncomps {
            thisi = (*scanptr).component_index[ci as usize];
            if thisi < 0 as i32 || thisi >= (*cinfo).num_components {
                (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_SCAN_SCRIPT as i32;
                (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = scanno;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            /* Components must appear in SOF order within each scan */
            if ci > 0 as i32 && thisi <= (*scanptr).component_index[(ci - 1 as i32) as usize] {
                (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_SCAN_SCRIPT as i32;
                (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = scanno;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            ci += 1
        }
        /* Validate progression parameters */
        Ss = (*scanptr).Ss;
        Se = (*scanptr).Se;
        Ah = (*scanptr).Ah;
        Al = (*scanptr).Al;
        if (*cinfo).progressive_mode != 0 {
            /* The JPEG spec simply gives the ranges 0..13 for Ah and Al, but that
             * seems wrong: the upper bound ought to depend on data precision.
             * Perhaps they really meant 0..N+1 for N-bit precision.
             * Here we allow 0..10 for 8-bit data; Al larger than 10 results in
             * out-of-range reconstructed DC values during the first DC scan,
             * which might cause problems for some decoders.
             */
            if Ss < 0 as i32
                || Ss >= 64 as i32
                || Se < Ss
                || Se >= 64 as i32
                || Ah < 0 as i32
                || Ah > 10 as i32
                || Al < 0 as i32
                || Al > 10 as i32
            {
                (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_PROG_SCRIPT as i32;
                (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = scanno;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            if Ss == 0 as i32 {
                if Se != 0 as i32 {
                    /* DC and AC together not OK */
                    (*(*cinfo).err).msg_code =
                        crate::src::jpeg_8c::jerror::JERR_BAD_PROG_SCRIPT as i32;
                    (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = scanno;
                    Some(
                        (*(*cinfo).err)
                            .error_exit
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as crate::jpeglib_h::j_common_ptr
                    );
                }
            } else if ncomps != 1 as i32 {
                /* AC scans must be for only one component */
                (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_PROG_SCRIPT as i32;
                (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = scanno;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            ci = 0 as i32;
            while ci < ncomps {
                last_bitpos_ptr = &mut *(*last_bitpos
                    .as_mut_ptr()
                    .offset(*(*scanptr).component_index.as_ptr().offset(ci as isize) as isize))
                .as_mut_ptr()
                .offset(0 as i32 as isize) as *mut i32;
                if Ss != 0 as i32 && *last_bitpos_ptr.offset(0 as i32 as isize) < 0 as i32 {
                    /* AC without prior DC scan */
                    (*(*cinfo).err).msg_code =
                        crate::src::jpeg_8c::jerror::JERR_BAD_PROG_SCRIPT as i32;
                    (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = scanno;
                    Some(
                        (*(*cinfo).err)
                            .error_exit
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as crate::jpeglib_h::j_common_ptr
                    );
                }
                coefi = Ss;
                while coefi <= Se {
                    if *last_bitpos_ptr.offset(coefi as isize) < 0 as i32 {
                        /* first scan of this coefficient */
                        if Ah != 0 as i32 {
                            (*(*cinfo).err).msg_code =
                                crate::src::jpeg_8c::jerror::JERR_BAD_PROG_SCRIPT as i32;
                            (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = scanno;
                            Some(
                                (*(*cinfo).err)
                                    .error_exit
                                    .expect("non-null function pointer"),
                            )
                            .expect("non-null function pointer")(
                                cinfo as crate::jpeglib_h::j_common_ptr,
                            );
                        }
                    } else if Ah != *last_bitpos_ptr.offset(coefi as isize) || Al != Ah - 1 as i32 {
                        (*(*cinfo).err).msg_code =
                            crate::src::jpeg_8c::jerror::JERR_BAD_PROG_SCRIPT as i32;
                        (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = scanno;
                        Some(
                            (*(*cinfo).err)
                                .error_exit
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            cinfo as crate::jpeglib_h::j_common_ptr,
                        );
                    }
                    *last_bitpos_ptr.offset(coefi as isize) = Al;
                    coefi += 1
                }
                ci += 1
            }
        } else {
            /* not first scan */
            /* For sequential JPEG, all progression parameters must be these: */
            if Ss != 0 as i32 || Se != 64 as i32 - 1 as i32 || Ah != 0 as i32 || Al != 0 as i32 {
                (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_PROG_SCRIPT as i32;
                (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = scanno;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            /* Make sure components are not sent twice */
            ci = 0 as i32;
            while ci < ncomps {
                thisi = (*scanptr).component_index[ci as usize];
                if component_sent[thisi as usize] != 0 {
                    (*(*cinfo).err).msg_code =
                        crate::src::jpeg_8c::jerror::JERR_BAD_SCAN_SCRIPT as i32;
                    (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = scanno;
                    Some(
                        (*(*cinfo).err)
                            .error_exit
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as crate::jpeglib_h::j_common_ptr
                    );
                }
                component_sent[thisi as usize] = 1 as i32;
                ci += 1
            }
        }
        scanptr = scanptr.offset(1);
        scanno += 1
    }
    /* Now verify that everything got sent. */
    if (*cinfo).progressive_mode != 0 {
        /* For progressive mode, we only check that at least some DC data
         * got sent for each component; the spec does not require that all bits
         * of all coefficients be transmitted.  Would it be wiser to enforce
         * transmission of all coefficient bits??
         */
        ci = 0 as i32;
        while ci < (*cinfo).num_components {
            if last_bitpos[ci as usize][0 as i32 as usize] < 0 as i32 {
                (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_MISSING_DATA as i32;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            ci += 1
        }
    } else {
        ci = 0 as i32;
        while ci < (*cinfo).num_components {
            if component_sent[ci as usize] == 0 {
                (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_MISSING_DATA as i32;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            ci += 1
        }
    };
}

unsafe extern "C" fn reduce_script(mut cinfo: crate::jpeglib_h::j_compress_ptr)
/* Adapt scan script for use with reduced block size;
 * assume that script has been validated before.
 */
{
    let mut scanptr: *mut crate::jpeglib_h::jpeg_scan_info =
        0 as *mut crate::jpeglib_h::jpeg_scan_info;
    let mut idxout: i32 = 0;
    let mut idxin: i32 = 0;
    /* Circumvent const declaration for this function */
    scanptr = (*cinfo).scan_info as *mut crate::jpeglib_h::jpeg_scan_info;
    idxout = 0 as i32;
    idxin = 0 as i32;
    while idxin < (*cinfo).num_scans {
        /* After skipping, idxout becomes smaller than idxin */
        if idxin != idxout {
            /* Copy rest of data;
             * note we stay in given chunk of allocated memory.
             */
            *scanptr.offset(idxout as isize) = *scanptr.offset(idxin as isize)
        }
        if !((*scanptr.offset(idxout as isize)).Ss > (*cinfo).lim_Se) {
            if (*scanptr.offset(idxout as isize)).Se > (*cinfo).lim_Se {
                /* Limit scan to end of block */
                (*scanptr.offset(idxout as isize)).Se = (*cinfo).lim_Se
            }
            idxout += 1
        }
        /* Entire scan out of range - skip this entry */
        idxin += 1
    }
    (*cinfo).num_scans = idxout;
}
/* C_MULTISCAN_FILES_SUPPORTED */

unsafe extern "C" fn select_scan_parameters(mut cinfo: crate::jpeglib_h::j_compress_ptr)
/* Set up the scan parameters for the current scan */
{
    let mut ci: i32 = 0;
    if !(*cinfo).scan_info.is_null() {
        /* Prepare for current scan --- the script is already validated */
        let mut master: my_master_ptr = (*cinfo).master as my_master_ptr;
        let mut scanptr: *const crate::jpeglib_h::jpeg_scan_info =
            (*cinfo).scan_info.offset((*master).scan_number as isize);
        (*cinfo).comps_in_scan = (*scanptr).comps_in_scan;
        ci = 0 as i32;
        while ci < (*scanptr).comps_in_scan {
            (*cinfo).cur_comp_info[ci as usize] = &mut *(*cinfo)
                .comp_info
                .offset(*(*scanptr).component_index.as_ptr().offset(ci as isize) as isize)
                as *mut crate::jpeglib_h::jpeg_component_info;
            ci += 1
        }
        if (*cinfo).progressive_mode != 0 {
            (*cinfo).Ss = (*scanptr).Ss;
            (*cinfo).Se = (*scanptr).Se;
            (*cinfo).Ah = (*scanptr).Ah;
            (*cinfo).Al = (*scanptr).Al;
            return;
        }
    } else {
        /* Prepare for single sequential-JPEG scan containing all components */
        if (*cinfo).num_components > 4 as i32 {
            (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_COMPONENT_COUNT as i32;
            (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = (*cinfo).num_components;
            (*(*cinfo).err).msg_parm.i[1 as i32 as usize] = 4 as i32;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
        (*cinfo).comps_in_scan = (*cinfo).num_components;
        ci = 0 as i32;
        while ci < (*cinfo).num_components {
            (*cinfo).cur_comp_info[ci as usize] = &mut *(*cinfo).comp_info.offset(ci as isize)
                as *mut crate::jpeglib_h::jpeg_component_info;
            ci += 1
        }
    }
    (*cinfo).Ss = 0 as i32;
    (*cinfo).Se = (*cinfo).block_size * (*cinfo).block_size - 1 as i32;
    (*cinfo).Ah = 0 as i32;
    (*cinfo).Al = 0 as i32;
}

unsafe extern "C" fn per_scan_setup(mut cinfo: crate::jpeglib_h::j_compress_ptr)
/* Do computations that are needed before processing a JPEG scan */
/* cinfo->comps_in_scan and cinfo->cur_comp_info[] are already set */
{
    let mut ci: i32 = 0;
    let mut mcublks: i32 = 0;
    let mut tmp: i32 = 0;
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        0 as *mut crate::jpeglib_h::jpeg_component_info;
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
            (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_COMPONENT_COUNT as i32;
            (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = (*cinfo).comps_in_scan;
            (*(*cinfo).err).msg_parm.i[1 as i32 as usize] = 4 as i32;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
        /* Overall image size in MCUs */
        (*cinfo).MCUs_per_row = crate::src::jpeg_8c::jutils::jdiv_round_up(
            (*cinfo).jpeg_width as isize,
            ((*cinfo).max_h_samp_factor * (*cinfo).block_size) as isize,
        ) as crate::jmorecfg_h::JDIMENSION;
        (*cinfo).MCU_rows_in_scan = crate::src::jpeg_8c::jutils::jdiv_round_up(
            (*cinfo).jpeg_height as isize,
            ((*cinfo).max_v_samp_factor * (*cinfo).block_size) as isize,
        ) as crate::jmorecfg_h::JDIMENSION;
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
                (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_MCU_SIZE as i32;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            loop {
                let fresh1 = mcublks;
                mcublks = mcublks - 1;
                if !(fresh1 > 0 as i32) {
                    break;
                }
                let fresh2 = (*cinfo).blocks_in_MCU;
                (*cinfo).blocks_in_MCU = (*cinfo).blocks_in_MCU + 1;
                (*cinfo).MCU_membership[fresh2 as usize] = ci
            }
            ci += 1
        }
    }
    /* Convert restart specified in rows to actual MCU count. */
    /* Note that count must fit in 16 bits, so we provide limiting. */
    if (*cinfo).restart_in_rows > 0 as i32 {
        let mut nominal: isize =
            (*cinfo).restart_in_rows as isize * (*cinfo).MCUs_per_row as isize;
        (*cinfo).restart_interval = if nominal < 65535 as isize {
            nominal
        } else {
            65535 as isize
        } as u32
    };
}
/*
 * Per-pass setup.
 * This is called at the beginning of each pass.  We determine which modules
 * will be active during this pass and give them appropriate start_pass calls.
 * We also set is_last_pass to indicate whether any more passes will be
 * required.
 */

unsafe extern "C" fn prepare_for_pass(mut cinfo: crate::jpeglib_h::j_compress_ptr) {
    let mut master: my_master_ptr = (*cinfo).master as my_master_ptr;
    let mut current_block_33: u64;
    match (*master).pass_type as u32 {
        0 => {
            /* Initial pass: will collect input data, and do either Huffman
             * optimization or data output for the first scan.
             */
            select_scan_parameters(cinfo);
            per_scan_setup(cinfo);
            if (*cinfo).raw_data_in == 0 {
                Some(
                    (*(*cinfo).cconvert)
                        .start_pass
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo);
                Some(
                    (*(*cinfo).downsample)
                        .start_pass
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo);
                Some(
                    (*(*cinfo).prep)
                        .start_pass
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo, crate::jpegint_h::JBUF_PASS_THRU
                );
            }
            Some(
                (*(*cinfo).fdct)
                    .start_pass
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo);
            Some(
                (*(*cinfo).entropy)
                    .start_pass
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo, (*cinfo).optimize_coding);
            Some(
                (*(*cinfo).coef)
                    .start_pass
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo,
                if (*master).total_passes > 1 as i32 {
                    crate::jpegint_h::JBUF_SAVE_AND_PASS as i32
                } else {
                    crate::jpegint_h::JBUF_PASS_THRU as i32
                } as crate::jpegint_h::J_BUF_MODE,
            );
            Some(
                (*(*cinfo).main)
                    .start_pass
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo, crate::jpegint_h::JBUF_PASS_THRU
            );
            if (*cinfo).optimize_coding != 0 {
                /* No immediate data output; postpone writing frame/scan headers */
                (*master).pub_0.call_pass_startup = 0 as i32
            } else {
                /* Will write frame/scan headers at first jpeg_write_scanlines call */
                (*master).pub_0.call_pass_startup = 1 as i32
            }
            current_block_33 = 9520865839495247062;
        }
        1 => {
            /* Do Huffman optimization for a scan after the first one. */
            select_scan_parameters(cinfo);
            per_scan_setup(cinfo);
            if (*cinfo).Ss != 0 as i32 || (*cinfo).Ah == 0 as i32 {
                Some(
                    (*(*cinfo).entropy)
                        .start_pass
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo, 1 as i32);
                Some(
                    (*(*cinfo).coef)
                        .start_pass
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo, crate::jpegint_h::JBUF_CRANK_DEST
                );
                (*master).pub_0.call_pass_startup = 0 as i32;
                current_block_33 = 9520865839495247062;
            } else {
                /* Special case: Huffman DC refinement scans need no Huffman table
                 * and therefore we can skip the optimization pass for them.
                 */
                (*master).pass_type = output_pass;
                (*master).pass_number += 1;
                current_block_33 = 9439872330697556915;
            }
        }
        2 => {
            current_block_33 = 9439872330697556915;
        }
        _ => {
            (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_NOT_COMPILED as i32;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
            current_block_33 = 9520865839495247062;
        }
    }
    match current_block_33 {
        9439872330697556915 =>
        /*FALLTHROUGH*/
        /* Do a data-output pass. */
        /* We need not repeat per-scan setup if prior optimization pass did it. */
        {
            if (*cinfo).optimize_coding == 0 {
                select_scan_parameters(cinfo);
                per_scan_setup(cinfo);
            }
            Some(
                (*(*cinfo).entropy)
                    .start_pass
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo, 0 as i32);
            Some(
                (*(*cinfo).coef)
                    .start_pass
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo, crate::jpegint_h::JBUF_CRANK_DEST
            );
            /* We emit frame/scan headers now */
            if (*master).scan_number == 0 as i32 {
                Some(
                    (*(*cinfo).marker)
                        .write_frame_header
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo);
            }
            Some(
                (*(*cinfo).marker)
                    .write_scan_header
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo);
            (*master).pub_0.call_pass_startup = 0 as i32
        }
        _ => {}
    }
    (*master).pub_0.is_last_pass =
        ((*master).pass_number == (*master).total_passes - 1 as i32) as i32;
    /* Set up progress monitor's pass info if present */
    if !(*cinfo).progress.is_null() {
        (*(*cinfo).progress).completed_passes = (*master).pass_number;
        (*(*cinfo).progress).total_passes = (*master).total_passes
    };
}
/*
 * Special start-of-pass hook.
 * This is called by jpeg_write_scanlines if call_pass_startup is TRUE.
 * In single-pass processing, we need this hook because we don't want to
 * write frame/scan headers during jpeg_start_compress; we want to let the
 * application write COM markers etc. between jpeg_start_compress and the
 * jpeg_write_scanlines loop.
 * In multi-pass processing, this routine is not used.
 */

unsafe extern "C" fn pass_startup(mut cinfo: crate::jpeglib_h::j_compress_ptr) {
    (*(*cinfo).master).call_pass_startup = 0 as i32; /* reset flag so call only once */
    Some(
        (*(*cinfo).marker)
            .write_frame_header
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo);
    Some(
        (*(*cinfo).marker)
            .write_scan_header
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo);
}
/*
 * Finish up at end of pass.
 */

unsafe extern "C" fn finish_pass_master(mut cinfo: crate::jpeglib_h::j_compress_ptr) {
    let mut master: my_master_ptr = (*cinfo).master as my_master_ptr;
    /* The entropy coder always needs an end-of-pass call,
     * either to analyze statistics or to flush its output buffer.
     */
    Some(
        (*(*cinfo).entropy)
            .finish_pass
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo);
    /* Update state for next pass */
    match (*master).pass_type as u32 {
        0 => {
            /* next pass is either output of scan 0 (after optimization)
             * or output of scan 1 (if no optimization).
             */
            (*master).pass_type = output_pass;
            if (*cinfo).optimize_coding == 0 {
                (*master).scan_number += 1
            }
        }
        1 => {
            /* next pass is always output of current scan */
            (*master).pass_type = output_pass
        }
        2 => {
            /* next pass is either optimization or output of next scan */
            if (*cinfo).optimize_coding != 0 {
                (*master).pass_type = huff_opt_pass
            }
            (*master).scan_number += 1
        }
        _ => {}
    }
    (*master).pass_number += 1;
}
/*
 * Initialize master compression control.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_c_master_control(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut transcode_only: crate::jmorecfg_h::boolean,
) {
    let mut master: my_master_ptr = 0 as *mut my_comp_master;
    master = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        1 as i32,
        ::std::mem::size_of::<my_comp_master>() as libc::c_ulong,
    ) as my_master_ptr;
    (*cinfo).master = master as *mut crate::jpegint_h::jpeg_comp_master;
    (*master).pub_0.prepare_for_pass =
        Some(prepare_for_pass as unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> ());
    (*master).pub_0.pass_startup =
        Some(pass_startup as unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> ());
    (*master).pub_0.finish_pass =
        Some(finish_pass_master as unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> ());
    (*master).pub_0.is_last_pass = 0 as i32;
    /* Validate parameters, determine derived values */
    initial_setup(cinfo, transcode_only);
    if !(*cinfo).scan_info.is_null() {
        validate_script(cinfo);
        if (*cinfo).block_size < 8 as i32 {
            reduce_script(cinfo);
        }
    } else {
        (*cinfo).progressive_mode = 0 as i32;
        (*cinfo).num_scans = 1 as i32
    }
    if ((*cinfo).progressive_mode != 0 || (*cinfo).block_size < 8 as i32)
        && (*cinfo).arith_code == 0
    {
        /*  TEMPORARY HACK ??? */
        /* assume default tables no good for progressive or downscale mode */
        (*cinfo).optimize_coding = 1 as i32
    }
    /* Initialize my private state */
    if transcode_only != 0 {
        /* no main pass in transcoding */
        if (*cinfo).optimize_coding != 0 {
            (*master).pass_type = huff_opt_pass
        } else {
            (*master).pass_type = output_pass
        }
    } else {
        /* for normal compression, first pass is always this type: */
        (*master).pass_type = main_pass
    }
    (*master).scan_number = 0 as i32;
    (*master).pass_number = 0 as i32;
    if (*cinfo).optimize_coding != 0 {
        (*master).total_passes = (*cinfo).num_scans * 2 as i32
    } else {
        (*master).total_passes = (*cinfo).num_scans
    };
}
