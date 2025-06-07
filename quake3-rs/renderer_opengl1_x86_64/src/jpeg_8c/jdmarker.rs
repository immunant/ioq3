use ::libc;

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
pub use crate::src::jpeg_8c::jcomapi::jpeg_alloc_huff_table;
pub use crate::src::jpeg_8c::jcomapi::jpeg_alloc_quant_table;
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
pub use crate::src::jpeg_8c::jutils::jpeg_natural_order;
pub use crate::src::jpeg_8c::jutils::jpeg_natural_order2;
pub use crate::src::jpeg_8c::jutils::jpeg_natural_order3;
pub use crate::src::jpeg_8c::jutils::jpeg_natural_order4;
pub use crate::src::jpeg_8c::jutils::jpeg_natural_order5;
pub use crate::src::jpeg_8c::jutils::jpeg_natural_order6;
pub use crate::src::jpeg_8c::jutils::jpeg_natural_order7;

pub const M_APP0: C2RustUnnamed_53 = 224;

pub type my_marker_ptr = *mut my_marker_reader;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_marker_reader {
    pub pub_0: crate::jpegint_h::jpeg_marker_reader,
    pub process_COM: crate::jpeglib_h::jpeg_marker_parser_method,
    pub process_APPn: [crate::jpeglib_h::jpeg_marker_parser_method; 16],
    pub length_limit_COM: libc::c_uint,
    pub length_limit_APPn: [libc::c_uint; 16],
    pub cur_marker: crate::jpeglib_h::jpeg_saved_marker_ptr,
    pub bytes_read: libc::c_uint,
}

pub const M_APP15: C2RustUnnamed_53 = 239;

pub const M_COM: C2RustUnnamed_53 = 254;

pub const M_APP14: C2RustUnnamed_53 = 238;

pub const M_RST0: C2RustUnnamed_53 = 208;

pub const M_RST7: C2RustUnnamed_53 = 215;

pub const M_SOF0: C2RustUnnamed_53 = 192;

pub const M_DNL: C2RustUnnamed_53 = 220;

pub const M_TEM: C2RustUnnamed_53 = 1;

pub const M_RST6: C2RustUnnamed_53 = 214;

pub const M_RST5: C2RustUnnamed_53 = 213;

pub const M_RST4: C2RustUnnamed_53 = 212;

pub const M_RST3: C2RustUnnamed_53 = 211;

pub const M_RST2: C2RustUnnamed_53 = 210;

pub const M_RST1: C2RustUnnamed_53 = 209;

pub const M_APP13: C2RustUnnamed_53 = 237;

pub const M_APP12: C2RustUnnamed_53 = 236;

pub const M_APP11: C2RustUnnamed_53 = 235;

pub const M_APP10: C2RustUnnamed_53 = 234;

pub const M_APP9: C2RustUnnamed_53 = 233;

pub const M_APP8: C2RustUnnamed_53 = 232;

pub const M_APP7: C2RustUnnamed_53 = 231;

pub const M_APP6: C2RustUnnamed_53 = 230;

pub const M_APP5: C2RustUnnamed_53 = 229;

pub const M_APP4: C2RustUnnamed_53 = 228;

pub const M_APP3: C2RustUnnamed_53 = 227;

pub const M_APP2: C2RustUnnamed_53 = 226;

pub const M_APP1: C2RustUnnamed_53 = 225;

pub const M_DRI: C2RustUnnamed_53 = 221;

pub const M_DQT: C2RustUnnamed_53 = 219;

pub const M_DHT: C2RustUnnamed_53 = 196;

pub const M_DAC: C2RustUnnamed_53 = 204;

pub const M_EOI: C2RustUnnamed_53 = 217;

pub const M_SOS: C2RustUnnamed_53 = 218;

pub const M_SOF15: C2RustUnnamed_53 = 207;

pub const M_SOF14: C2RustUnnamed_53 = 206;

pub const M_SOF13: C2RustUnnamed_53 = 205;

pub const M_SOF11: C2RustUnnamed_53 = 203;

pub const M_JPG: C2RustUnnamed_53 = 200;

pub const M_SOF7: C2RustUnnamed_53 = 199;

pub const M_SOF6: C2RustUnnamed_53 = 198;

pub const M_SOF5: C2RustUnnamed_53 = 197;

pub const M_SOF3: C2RustUnnamed_53 = 195;

pub const M_SOF10: C2RustUnnamed_53 = 202;

pub const M_SOF9: C2RustUnnamed_53 = 201;

pub const M_SOF2: C2RustUnnamed_53 = 194;

pub const M_SOF1: C2RustUnnamed_53 = 193;

pub const M_SOI: C2RustUnnamed_53 = 216;

pub type C2RustUnnamed_53 = libc::c_uint;

pub const M_ERROR: C2RustUnnamed_53 = 256;

pub const M_JPG13: C2RustUnnamed_53 = 253;

pub const M_JPG0: C2RustUnnamed_53 = 240;

pub const M_EXP: C2RustUnnamed_53 = 223;

pub const M_DHP: C2RustUnnamed_53 = 222;
/*
 * Macros for fetching data from the data source module.
 *
 * At all times, cinfo->src->next_input_byte and ->bytes_in_buffer reflect
 * the current restart point; we update them only when we have reached a
 * suitable place to restart if a suspension occurs.
 */
/* Declare and initialize local copies of input pointer/count */
/* Unload the local copies --- do this only at a restart boundary */
/* Reload the local copies --- used only in MAKE_BYTE_AVAIL */
/* Internal macro for INPUT_BYTE and INPUT_2BYTES: make a byte available.
 * Note we do *not* do INPUT_SYNC before calling fill_input_buffer,
 * but we must reload the local copies after a successful fill.
 */
/* Read a byte into variable V.
 * If must suspend, take the specified action (typically "return FALSE").
 */
/* As above, but read two bytes interpreted as an unsigned 16-bit integer.
 * V should be declared unsigned int or perhaps INT32.
 */
/*
 * Routines to process JPEG markers.
 *
 * Entry condition: JPEG marker itself has been read and its code saved
 *   in cinfo->unread_marker; input restart point is just after the marker.
 *
 * Exit: if return TRUE, have read and processed any parameters, and have
 *   updated the restart point to point after the parameters.
 *   If return FALSE, was forced to suspend before reaching end of
 *   marker parameters; restart point has not been moved.  Same routine
 *   will be called again after application supplies more input data.
 *
 * This approach to suspension assumes that all of a marker's parameters
 * can fit into a single input bufferload.  This should hold for "normal"
 * markers.  Some COM/APPn markers might have large parameter segments
 * that might not fit.  If we are simply dropping such a marker, we use
 * skip_input_data to get past it, and thereby put the problem on the
 * source manager's shoulders.  If we are saving the marker's contents
 * into memory, we use a slightly different convention: when forced to
 * suspend, the marker processor updates the restart point to the end of
 * what it's consumed (ie, the end of the buffer) before returning FALSE.
 * On resumption, cinfo->unread_marker still contains the marker code,
 * but the data source will point to the next chunk of marker data.
 * The marker processor must retain internal state to deal with this.
 *
 * Note that we don't bother to avoid duplicate trace messages if a
 * suspension occurs within marker parameters.  Other side effects
 * require more care.
 */

unsafe extern "C" fn get_soi(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
) -> crate::jmorecfg_h::boolean
/* Process an SOI marker */ {
    let mut i: libc::c_int = 0;
    (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JTRC_SOI as libc::c_int;
    Some(
        (*(*cinfo).err)
            .emit_message
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr, 1 as libc::c_int
    );
    if (*(*cinfo).marker).saw_SOI != 0 {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_SOI_DUPLICATE as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Reset all parameters that are defined to be reset by SOI */
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        (*cinfo).arith_dc_L[i as usize] = 0 as libc::c_int as crate::jmorecfg_h::UINT8;
        (*cinfo).arith_dc_U[i as usize] = 1 as libc::c_int as crate::jmorecfg_h::UINT8;
        (*cinfo).arith_ac_K[i as usize] = 5 as libc::c_int as crate::jmorecfg_h::UINT8;
        i += 1
    }
    (*cinfo).restart_interval = 0 as libc::c_int as libc::c_uint;
    /* Set initial assumptions for colorspace etc */
    (*cinfo).jpeg_color_space = crate::jpeglib_h::JCS_UNKNOWN; /* Assume non-CCIR sampling??? */
    (*cinfo).CCIR601_sampling = 0 as libc::c_int; /* set default JFIF APP0 values */
    (*cinfo).saw_JFIF_marker = 0 as libc::c_int;
    (*cinfo).JFIF_major_version = 1 as libc::c_int as crate::jmorecfg_h::UINT8;
    (*cinfo).JFIF_minor_version = 1 as libc::c_int as crate::jmorecfg_h::UINT8;
    (*cinfo).density_unit = 0 as libc::c_int as crate::jmorecfg_h::UINT8;
    (*cinfo).X_density = 1 as libc::c_int as crate::jmorecfg_h::UINT16;
    (*cinfo).Y_density = 1 as libc::c_int as crate::jmorecfg_h::UINT16;
    (*cinfo).saw_Adobe_marker = 0 as libc::c_int;
    (*cinfo).Adobe_transform = 0 as libc::c_int as crate::jmorecfg_h::UINT8;
    (*(*cinfo).marker).saw_SOI = 1 as libc::c_int;
    return 1 as libc::c_int;
}

unsafe extern "C" fn get_sof(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut is_baseline: crate::jmorecfg_h::boolean,
    mut is_prog: crate::jmorecfg_h::boolean,
    mut is_arith: crate::jmorecfg_h::boolean,
) -> crate::jmorecfg_h::boolean
/* Process a SOFn marker */ {
    let mut length: crate::jmorecfg_h::INT32 = 0;
    let mut c: libc::c_int = 0;
    let mut ci: libc::c_int = 0;
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        0 as *mut crate::jpeglib_h::jpeg_component_info;
    let mut datasrc: *mut crate::jpeglib_h::jpeg_source_mgr = (*cinfo).src;
    let mut next_input_byte: *const crate::jmorecfg_h::JOCTET = (*datasrc).next_input_byte;
    let mut bytes_in_buffer: crate::stddef_h::size_t = (*datasrc).bytes_in_buffer;
    (*cinfo).is_baseline = is_baseline;
    (*cinfo).progressive_mode = is_prog;
    (*cinfo).arith_code = is_arith;
    if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as libc::c_int;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh0 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length = ((*fresh0 as libc::c_uint) << 8 as libc::c_int) as crate::jmorecfg_h::INT32;
    if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as libc::c_int;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh1 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length += *fresh1 as libc::c_long;
    if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as libc::c_int;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh2 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    (*cinfo).data_precision = *fresh2 as libc::c_int;
    if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as libc::c_int;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh3 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    (*cinfo).image_height = (*fresh3 as libc::c_uint) << 8 as libc::c_int;
    if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as libc::c_int;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh4 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    (*cinfo).image_height =
        ((*cinfo).image_height as libc::c_uint).wrapping_add(*fresh4 as libc::c_uint)
            as crate::jmorecfg_h::JDIMENSION as crate::jmorecfg_h::JDIMENSION;
    if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as libc::c_int;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh5 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    (*cinfo).image_width = (*fresh5 as libc::c_uint) << 8 as libc::c_int;
    if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as libc::c_int;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh6 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    (*cinfo).image_width =
        ((*cinfo).image_width as libc::c_uint).wrapping_add(*fresh6 as libc::c_uint)
            as crate::jmorecfg_h::JDIMENSION as crate::jmorecfg_h::JDIMENSION;
    if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as libc::c_int;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh7 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    (*cinfo).num_components = *fresh7 as libc::c_int;
    length -= 8 as libc::c_int as libc::c_long;
    let mut _mp: *mut libc::c_int = (*(*cinfo).err).msg_parm.i.as_mut_ptr();
    *_mp.offset(0 as libc::c_int as isize) = (*cinfo).unread_marker;
    *_mp.offset(1 as libc::c_int as isize) = (*cinfo).image_width as libc::c_int;
    *_mp.offset(2 as libc::c_int as isize) = (*cinfo).image_height as libc::c_int;
    *_mp.offset(3 as libc::c_int as isize) = (*cinfo).num_components;
    (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JTRC_SOF as libc::c_int;
    Some(
        (*(*cinfo).err)
            .emit_message
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr, 1 as libc::c_int
    );
    if (*(*cinfo).marker).saw_SOF != 0 {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_SOF_DUPLICATE as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* We don't support files in which the image height is initially specified */
    /* as 0 and is later redefined by DNL.  As long as we have to check that,  */
    /* might as well have a general sanity check. */
    if (*cinfo).image_height <= 0 as libc::c_int as libc::c_uint
        || (*cinfo).image_width <= 0 as libc::c_int as libc::c_uint
        || (*cinfo).num_components <= 0 as libc::c_int
    {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_EMPTY_IMAGE as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    if length != ((*cinfo).num_components * 3 as libc::c_int) as libc::c_long {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_LENGTH as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    if (*cinfo).comp_info.is_null() {
        /* do only once, even if suspend */
        (*cinfo).comp_info = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            1 as libc::c_int,
            ((*cinfo).num_components as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<crate::jpeglib_h::jpeg_component_info>() as libc::c_ulong,
                ),
        ) as *mut crate::jpeglib_h::jpeg_component_info
    }
    ci = 0 as libc::c_int;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        (*compptr).component_index = ci;
        if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
            if Some(
                (*datasrc)
                    .fill_input_buffer
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo)
                == 0
            {
                return 0 as libc::c_int;
            }
            next_input_byte = (*datasrc).next_input_byte;
            bytes_in_buffer = (*datasrc).bytes_in_buffer
        }
        bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
        let fresh8 = next_input_byte;
        next_input_byte = next_input_byte.offset(1);
        (*compptr).component_id = *fresh8 as libc::c_int;
        if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
            if Some(
                (*datasrc)
                    .fill_input_buffer
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo)
                == 0
            {
                return 0 as libc::c_int;
            }
            next_input_byte = (*datasrc).next_input_byte;
            bytes_in_buffer = (*datasrc).bytes_in_buffer
        }
        bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
        let fresh9 = next_input_byte;
        next_input_byte = next_input_byte.offset(1);
        c = *fresh9 as libc::c_int;
        (*compptr).h_samp_factor = c >> 4 as libc::c_int & 15 as libc::c_int;
        (*compptr).v_samp_factor = c & 15 as libc::c_int;
        if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
            if Some(
                (*datasrc)
                    .fill_input_buffer
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo)
                == 0
            {
                return 0 as libc::c_int;
            }
            next_input_byte = (*datasrc).next_input_byte;
            bytes_in_buffer = (*datasrc).bytes_in_buffer
        }
        bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
        let fresh10 = next_input_byte;
        next_input_byte = next_input_byte.offset(1);
        (*compptr).quant_tbl_no = *fresh10 as libc::c_int;
        let mut _mp_0: *mut libc::c_int = (*(*cinfo).err).msg_parm.i.as_mut_ptr();
        *_mp_0.offset(0 as libc::c_int as isize) = (*compptr).component_id;
        *_mp_0.offset(1 as libc::c_int as isize) = (*compptr).h_samp_factor;
        *_mp_0.offset(2 as libc::c_int as isize) = (*compptr).v_samp_factor;
        *_mp_0.offset(3 as libc::c_int as isize) = (*compptr).quant_tbl_no;
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JTRC_SOF_COMPONENT as libc::c_int;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            1 as libc::c_int,
        );
        ci += 1;
        compptr = compptr.offset(1)
    }
    (*(*cinfo).marker).saw_SOF = 1 as libc::c_int;
    (*datasrc).next_input_byte = next_input_byte;
    (*datasrc).bytes_in_buffer = bytes_in_buffer;
    return 1 as libc::c_int;
}

unsafe extern "C" fn get_sos(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
) -> crate::jmorecfg_h::boolean
/* Process a SOS marker */ {
    let mut length: crate::jmorecfg_h::INT32 = 0; /* Number of components */
    let mut i: libc::c_int = 0;
    let mut ci: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut cc: libc::c_int = 0;
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        0 as *mut crate::jpeglib_h::jpeg_component_info;
    let mut datasrc: *mut crate::jpeglib_h::jpeg_source_mgr = (*cinfo).src;
    let mut next_input_byte: *const crate::jmorecfg_h::JOCTET = (*datasrc).next_input_byte;
    let mut bytes_in_buffer: crate::stddef_h::size_t = (*datasrc).bytes_in_buffer;
    if (*(*cinfo).marker).saw_SOF == 0 {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_SOS_NO_SOF as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as libc::c_int;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh11 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length = ((*fresh11 as libc::c_uint) << 8 as libc::c_int) as crate::jmorecfg_h::INT32;
    if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as libc::c_int;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh12 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length += *fresh12 as libc::c_long;
    if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as libc::c_int;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh13 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    n = *fresh13 as libc::c_int;
    (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JTRC_SOS as libc::c_int;
    (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = n;
    Some(
        (*(*cinfo).err)
            .emit_message
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr, 1 as libc::c_int
    );
    if length != (n * 2 as libc::c_int + 6 as libc::c_int) as libc::c_long
        || n > 4 as libc::c_int
        || n == 0 as libc::c_int && (*cinfo).progressive_mode == 0
    {
        /* pseudo SOS marker only allowed in progressive mode */
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_LENGTH as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    (*cinfo).comps_in_scan = n;
    /* Collect the component-spec parameters */
    i = 0 as libc::c_int;
    while i < n {
        let mut current_block_71: u64;
        if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
            if Some(
                (*datasrc)
                    .fill_input_buffer
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo)
                == 0
            {
                return 0 as libc::c_int;
            }
            next_input_byte = (*datasrc).next_input_byte;
            bytes_in_buffer = (*datasrc).bytes_in_buffer
        }
        bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
        let fresh14 = next_input_byte;
        next_input_byte = next_input_byte.offset(1);
        cc = *fresh14 as libc::c_int;
        if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
            if Some(
                (*datasrc)
                    .fill_input_buffer
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo)
                == 0
            {
                return 0 as libc::c_int;
            }
            next_input_byte = (*datasrc).next_input_byte;
            bytes_in_buffer = (*datasrc).bytes_in_buffer
        }
        bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
        let fresh15 = next_input_byte;
        next_input_byte = next_input_byte.offset(1);
        c = *fresh15 as libc::c_int;
        ci = 0 as libc::c_int;
        compptr = (*cinfo).comp_info;
        loop {
            if !(ci < (*cinfo).num_components) {
                current_block_71 = 16203797167131938757;
                break;
            }
            if cc == (*compptr).component_id {
                current_block_71 = 14439827023736234030;
                break;
            }
            ci += 1;
            compptr = compptr.offset(1)
        }
        match current_block_71 {
            16203797167131938757 => {
                (*(*cinfo).err).msg_code =
                    crate::src::jpeg_8c::jerror::JERR_BAD_COMPONENT_ID as libc::c_int;
                (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = cc;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            _ => {}
        }
        (*cinfo).cur_comp_info[i as usize] = compptr;
        (*compptr).dc_tbl_no = c >> 4 as libc::c_int & 15 as libc::c_int;
        (*compptr).ac_tbl_no = c & 15 as libc::c_int;
        let mut _mp: *mut libc::c_int = (*(*cinfo).err).msg_parm.i.as_mut_ptr();
        *_mp.offset(0 as libc::c_int as isize) = cc;
        *_mp.offset(1 as libc::c_int as isize) = (*compptr).dc_tbl_no;
        *_mp.offset(2 as libc::c_int as isize) = (*compptr).ac_tbl_no;
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JTRC_SOS_COMPONENT as libc::c_int;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            1 as libc::c_int,
        );
        i += 1
    }
    /* Collect the additional scan parameters Ss, Se, Ah/Al. */
    if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as libc::c_int;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh16 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    c = *fresh16 as libc::c_int;
    (*cinfo).Ss = c;
    if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as libc::c_int;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh17 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    c = *fresh17 as libc::c_int;
    (*cinfo).Se = c;
    if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as libc::c_int;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh18 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    c = *fresh18 as libc::c_int;
    (*cinfo).Ah = c >> 4 as libc::c_int & 15 as libc::c_int;
    (*cinfo).Al = c & 15 as libc::c_int;
    let mut _mp_0: *mut libc::c_int = (*(*cinfo).err).msg_parm.i.as_mut_ptr();
    *_mp_0.offset(0 as libc::c_int as isize) = (*cinfo).Ss;
    *_mp_0.offset(1 as libc::c_int as isize) = (*cinfo).Se;
    *_mp_0.offset(2 as libc::c_int as isize) = (*cinfo).Ah;
    *_mp_0.offset(3 as libc::c_int as isize) = (*cinfo).Al;
    (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JTRC_SOS_PARAMS as libc::c_int;
    Some(
        (*(*cinfo).err)
            .emit_message
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr, 1 as libc::c_int
    );
    /* Prepare to scan data & restart markers */
    (*(*cinfo).marker).next_restart_num = 0 as libc::c_int;
    /* Count another (non-pseudo) SOS marker */
    if n != 0 {
        (*cinfo).input_scan_number += 1
    }
    (*datasrc).next_input_byte = next_input_byte;
    (*datasrc).bytes_in_buffer = bytes_in_buffer;
    return 1 as libc::c_int;
}

unsafe extern "C" fn get_dac(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
) -> crate::jmorecfg_h::boolean
/* Process a DAC marker */ {
    let mut length: crate::jmorecfg_h::INT32 = 0; /* define DC table */
    let mut index: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    let mut datasrc: *mut crate::jpeglib_h::jpeg_source_mgr = (*cinfo).src;
    let mut next_input_byte: *const crate::jmorecfg_h::JOCTET = (*datasrc).next_input_byte;
    let mut bytes_in_buffer: crate::stddef_h::size_t = (*datasrc).bytes_in_buffer;
    if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as libc::c_int;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh19 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length = ((*fresh19 as libc::c_uint) << 8 as libc::c_int) as crate::jmorecfg_h::INT32;
    if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as libc::c_int;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh20 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length += *fresh20 as libc::c_long;
    length -= 2 as libc::c_int as libc::c_long;
    while length > 0 as libc::c_int as libc::c_long {
        if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
            if Some(
                (*datasrc)
                    .fill_input_buffer
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo)
                == 0
            {
                return 0 as libc::c_int;
            }
            next_input_byte = (*datasrc).next_input_byte;
            bytes_in_buffer = (*datasrc).bytes_in_buffer
        }
        bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
        let fresh21 = next_input_byte;
        next_input_byte = next_input_byte.offset(1);
        index = *fresh21 as libc::c_int;
        if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
            if Some(
                (*datasrc)
                    .fill_input_buffer
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo)
                == 0
            {
                return 0 as libc::c_int;
            }
            next_input_byte = (*datasrc).next_input_byte;
            bytes_in_buffer = (*datasrc).bytes_in_buffer
        }
        bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
        let fresh22 = next_input_byte;
        next_input_byte = next_input_byte.offset(1);
        val = *fresh22 as libc::c_int;
        length -= 2 as libc::c_int as libc::c_long;
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JTRC_DAC as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = index;
        (*(*cinfo).err).msg_parm.i[1 as libc::c_int as usize] = val;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            1 as libc::c_int,
        );
        if index < 0 as libc::c_int || index >= 2 as libc::c_int * 16 as libc::c_int {
            (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_DAC_INDEX as libc::c_int;
            (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = index;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
        if index >= 16 as libc::c_int {
            /* define AC table */
            (*cinfo).arith_ac_K[(index - 16 as libc::c_int) as usize] =
                val as crate::jmorecfg_h::UINT8
        } else {
            (*cinfo).arith_dc_L[index as usize] =
                (val & 0xf as libc::c_int) as crate::jmorecfg_h::UINT8;
            (*cinfo).arith_dc_U[index as usize] =
                (val >> 4 as libc::c_int) as crate::jmorecfg_h::UINT8;
            if (*cinfo).arith_dc_L[index as usize] as libc::c_int
                > (*cinfo).arith_dc_U[index as usize] as libc::c_int
            {
                (*(*cinfo).err).msg_code =
                    crate::src::jpeg_8c::jerror::JERR_DAC_VALUE as libc::c_int;
                (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = val;
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
    if length != 0 as libc::c_int as libc::c_long {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_LENGTH as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    (*datasrc).next_input_byte = next_input_byte;
    (*datasrc).bytes_in_buffer = bytes_in_buffer;
    return 1 as libc::c_int;
}
/* ! D_ARITH_CODING_SUPPORTED */
/* D_ARITH_CODING_SUPPORTED */

unsafe extern "C" fn get_dht(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
) -> crate::jmorecfg_h::boolean
/* Process a DHT marker */ {
    let mut length: crate::jmorecfg_h::INT32 = 0;
    let mut bits: [crate::jmorecfg_h::UINT8; 17] = [0; 17];
    let mut huffval: [crate::jmorecfg_h::UINT8; 256] = [0; 256];
    let mut i: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut htblptr: *mut *mut crate::jpeglib_h::JHUFF_TBL =
        0 as *mut *mut crate::jpeglib_h::JHUFF_TBL;
    let mut datasrc: *mut crate::jpeglib_h::jpeg_source_mgr = (*cinfo).src;
    let mut next_input_byte: *const crate::jmorecfg_h::JOCTET = (*datasrc).next_input_byte;
    let mut bytes_in_buffer: crate::stddef_h::size_t = (*datasrc).bytes_in_buffer;
    if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as libc::c_int;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh23 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length = ((*fresh23 as libc::c_uint) << 8 as libc::c_int) as crate::jmorecfg_h::INT32;
    if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as libc::c_int;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh24 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length += *fresh24 as libc::c_long;
    length -= 2 as libc::c_int as libc::c_long;
    while length > 16 as libc::c_int as libc::c_long {
        if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
            if Some(
                (*datasrc)
                    .fill_input_buffer
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo)
                == 0
            {
                return 0 as libc::c_int;
            }
            next_input_byte = (*datasrc).next_input_byte;
            bytes_in_buffer = (*datasrc).bytes_in_buffer
        }
        bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
        let fresh25 = next_input_byte;
        next_input_byte = next_input_byte.offset(1);
        index = *fresh25 as libc::c_int;
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JTRC_DHT as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = index;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            1 as libc::c_int,
        );
        bits[0 as libc::c_int as usize] = 0 as libc::c_int as crate::jmorecfg_h::UINT8;
        count = 0 as libc::c_int;
        i = 1 as libc::c_int;
        while i <= 16 as libc::c_int {
            if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
                if Some(
                    (*datasrc)
                        .fill_input_buffer
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo)
                    == 0
                {
                    return 0 as libc::c_int;
                }
                next_input_byte = (*datasrc).next_input_byte;
                bytes_in_buffer = (*datasrc).bytes_in_buffer
            }
            bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
            let fresh26 = next_input_byte;
            next_input_byte = next_input_byte.offset(1);
            bits[i as usize] = *fresh26;
            count += bits[i as usize] as libc::c_int;
            i += 1
        }
        length -= (1 as libc::c_int + 16 as libc::c_int) as libc::c_long;
        let mut _mp: *mut libc::c_int = (*(*cinfo).err).msg_parm.i.as_mut_ptr();
        *_mp.offset(0 as libc::c_int as isize) = bits[1 as libc::c_int as usize] as libc::c_int;
        *_mp.offset(1 as libc::c_int as isize) = bits[2 as libc::c_int as usize] as libc::c_int;
        *_mp.offset(2 as libc::c_int as isize) = bits[3 as libc::c_int as usize] as libc::c_int;
        *_mp.offset(3 as libc::c_int as isize) = bits[4 as libc::c_int as usize] as libc::c_int;
        *_mp.offset(4 as libc::c_int as isize) = bits[5 as libc::c_int as usize] as libc::c_int;
        *_mp.offset(5 as libc::c_int as isize) = bits[6 as libc::c_int as usize] as libc::c_int;
        *_mp.offset(6 as libc::c_int as isize) = bits[7 as libc::c_int as usize] as libc::c_int;
        *_mp.offset(7 as libc::c_int as isize) = bits[8 as libc::c_int as usize] as libc::c_int;
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JTRC_HUFFBITS as libc::c_int;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            2 as libc::c_int,
        );
        let mut _mp_0: *mut libc::c_int = (*(*cinfo).err).msg_parm.i.as_mut_ptr();
        *_mp_0.offset(0 as libc::c_int as isize) = bits[9 as libc::c_int as usize] as libc::c_int;
        *_mp_0.offset(1 as libc::c_int as isize) = bits[10 as libc::c_int as usize] as libc::c_int;
        *_mp_0.offset(2 as libc::c_int as isize) = bits[11 as libc::c_int as usize] as libc::c_int;
        *_mp_0.offset(3 as libc::c_int as isize) = bits[12 as libc::c_int as usize] as libc::c_int;
        *_mp_0.offset(4 as libc::c_int as isize) = bits[13 as libc::c_int as usize] as libc::c_int;
        *_mp_0.offset(5 as libc::c_int as isize) = bits[14 as libc::c_int as usize] as libc::c_int;
        *_mp_0.offset(6 as libc::c_int as isize) = bits[15 as libc::c_int as usize] as libc::c_int;
        *_mp_0.offset(7 as libc::c_int as isize) = bits[16 as libc::c_int as usize] as libc::c_int;
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JTRC_HUFFBITS as libc::c_int;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            2 as libc::c_int,
        );
        /* Here we just do minimal validation of the counts to avoid walking
         * off the end of our table space.  jdhuff.c will check more carefully.
         */
        if count > 256 as libc::c_int || count as crate::jmorecfg_h::INT32 > length {
            (*(*cinfo).err).msg_code =
                crate::src::jpeg_8c::jerror::JERR_BAD_HUFF_TABLE as libc::c_int; /* DC table definition */
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
        i = 0 as libc::c_int;
        while i < count {
            if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
                if Some(
                    (*datasrc)
                        .fill_input_buffer
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo)
                    == 0
                {
                    return 0 as libc::c_int;
                }
                next_input_byte = (*datasrc).next_input_byte;
                bytes_in_buffer = (*datasrc).bytes_in_buffer
            }
            bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
            let fresh27 = next_input_byte;
            next_input_byte = next_input_byte.offset(1);
            huffval[i as usize] = *fresh27;
            i += 1
        }
        length -= count as libc::c_long;
        if index & 0x10 as libc::c_int != 0 {
            /* AC table definition */
            index -= 0x10 as libc::c_int;
            htblptr = &mut *(*cinfo)
                .ac_huff_tbl_ptrs
                .as_mut_ptr()
                .offset(index as isize)
                as *mut *mut crate::jpeglib_h::JHUFF_TBL
        } else {
            htblptr = &mut *(*cinfo)
                .dc_huff_tbl_ptrs
                .as_mut_ptr()
                .offset(index as isize)
                as *mut *mut crate::jpeglib_h::JHUFF_TBL
        }
        if index < 0 as libc::c_int || index >= 4 as libc::c_int {
            (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_DHT_INDEX as libc::c_int;
            (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = index;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
        if (*htblptr).is_null() {
            *htblptr = crate::src::jpeg_8c::jcomapi::jpeg_alloc_huff_table(
                cinfo as crate::jpeglib_h::j_common_ptr
                    as *mut crate::jpeglib_h::jpeg_common_struct,
            ) as *mut crate::jpeglib_h::JHUFF_TBL
        }
        crate::stdlib::memcpy(
            (**htblptr).bits.as_mut_ptr() as *mut libc::c_void,
            bits.as_mut_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[crate::jmorecfg_h::UINT8; 17]>() as libc::c_ulong,
        );
        crate::stdlib::memcpy(
            (**htblptr).huffval.as_mut_ptr() as *mut libc::c_void,
            huffval.as_mut_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[crate::jmorecfg_h::UINT8; 256]>() as libc::c_ulong,
        );
    }
    if length != 0 as libc::c_int as libc::c_long {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_LENGTH as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    (*datasrc).next_input_byte = next_input_byte;
    (*datasrc).bytes_in_buffer = bytes_in_buffer;
    return 1 as libc::c_int;
}

unsafe extern "C" fn get_dqt(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
) -> crate::jmorecfg_h::boolean
/* Process a DQT marker */ {
    let mut length: crate::jmorecfg_h::INT32 = 0;
    let mut count: crate::jmorecfg_h::INT32 = 0;
    let mut i: crate::jmorecfg_h::INT32 = 0;
    let mut n: libc::c_int = 0;
    let mut prec: libc::c_int = 0;
    let mut tmp: libc::c_uint = 0;
    let mut quant_ptr: *mut crate::jpeglib_h::JQUANT_TBL = 0 as *mut crate::jpeglib_h::JQUANT_TBL;
    let mut natural_order: *const libc::c_int = 0 as *const libc::c_int;
    let mut datasrc: *mut crate::jpeglib_h::jpeg_source_mgr = (*cinfo).src;
    let mut next_input_byte: *const crate::jmorecfg_h::JOCTET = (*datasrc).next_input_byte;
    let mut bytes_in_buffer: crate::stddef_h::size_t = (*datasrc).bytes_in_buffer;
    if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as libc::c_int;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh28 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length = ((*fresh28 as libc::c_uint) << 8 as libc::c_int) as crate::jmorecfg_h::INT32;
    if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as libc::c_int;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh29 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length += *fresh29 as libc::c_long;
    length -= 2 as libc::c_int as libc::c_long;
    while length > 0 as libc::c_int as libc::c_long {
        length -= 1;
        if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
            if Some(
                (*datasrc)
                    .fill_input_buffer
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo)
                == 0
            {
                return 0 as libc::c_int;
            }
            next_input_byte = (*datasrc).next_input_byte;
            bytes_in_buffer = (*datasrc).bytes_in_buffer
        }
        bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
        let fresh30 = next_input_byte;
        next_input_byte = next_input_byte.offset(1);
        n = *fresh30 as libc::c_int;
        prec = n >> 4 as libc::c_int;
        n &= 0xf as libc::c_int;
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JTRC_DQT as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = n;
        (*(*cinfo).err).msg_parm.i[1 as libc::c_int as usize] = prec;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            1 as libc::c_int,
        );
        if n >= 4 as libc::c_int {
            (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_DQT_INDEX as libc::c_int;
            (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = n;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
        if (*cinfo).quant_tbl_ptrs[n as usize].is_null() {
            (*cinfo).quant_tbl_ptrs[n as usize] =
                crate::src::jpeg_8c::jcomapi::jpeg_alloc_quant_table(
                    cinfo as crate::jpeglib_h::j_common_ptr
                        as *mut crate::jpeglib_h::jpeg_common_struct,
                ) as *mut crate::jpeglib_h::JQUANT_TBL
        }
        quant_ptr = (*cinfo).quant_tbl_ptrs[n as usize];
        if prec != 0 {
            if length < (64 as libc::c_int * 2 as libc::c_int) as libc::c_long {
                /* Initialize full table for safety. */
                i = 0 as libc::c_int as crate::jmorecfg_h::INT32;
                while i < 64 as libc::c_int as libc::c_long {
                    (*quant_ptr).quantval[i as usize] =
                        1 as libc::c_int as crate::jmorecfg_h::UINT16;
                    i += 1
                }
                count = length >> 1 as libc::c_int
            } else {
                count = 64 as libc::c_int as crate::jmorecfg_h::INT32
            }
        } else if length < 64 as libc::c_int as libc::c_long {
            /* Initialize full table for safety. */
            i = 0 as libc::c_int as crate::jmorecfg_h::INT32;
            while i < 64 as libc::c_int as libc::c_long {
                (*quant_ptr).quantval[i as usize] = 1 as libc::c_int as crate::jmorecfg_h::UINT16;
                i += 1
            }
            count = length
        } else {
            count = 64 as libc::c_int as crate::jmorecfg_h::INT32
        }
        match count {
            4 => natural_order = crate::src::jpeg_8c::jutils::jpeg_natural_order2.as_ptr(),
            9 => natural_order = crate::src::jpeg_8c::jutils::jpeg_natural_order3.as_ptr(),
            16 => natural_order = crate::src::jpeg_8c::jutils::jpeg_natural_order4.as_ptr(),
            25 => natural_order = crate::src::jpeg_8c::jutils::jpeg_natural_order5.as_ptr(),
            36 => natural_order = crate::src::jpeg_8c::jutils::jpeg_natural_order6.as_ptr(),
            49 => natural_order = crate::src::jpeg_8c::jutils::jpeg_natural_order7.as_ptr(),
            _ => natural_order = crate::src::jpeg_8c::jutils::jpeg_natural_order.as_ptr(),
        }
        i = 0 as libc::c_int as crate::jmorecfg_h::INT32;
        while i < count {
            if prec != 0 {
                if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
                    if Some(
                        (*datasrc)
                            .fill_input_buffer
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(cinfo)
                        == 0
                    {
                        return 0 as libc::c_int;
                    }
                    next_input_byte = (*datasrc).next_input_byte;
                    bytes_in_buffer = (*datasrc).bytes_in_buffer
                }
                bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
                let fresh31 = next_input_byte;
                next_input_byte = next_input_byte.offset(1);
                tmp = (*fresh31 as libc::c_uint) << 8 as libc::c_int;
                if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
                    if Some(
                        (*datasrc)
                            .fill_input_buffer
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(cinfo)
                        == 0
                    {
                        return 0 as libc::c_int;
                    }
                    next_input_byte = (*datasrc).next_input_byte;
                    bytes_in_buffer = (*datasrc).bytes_in_buffer
                }
                bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
                let fresh32 = next_input_byte;
                next_input_byte = next_input_byte.offset(1);
                tmp = tmp.wrapping_add(*fresh32 as libc::c_uint)
            } else {
                if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
                    if Some(
                        (*datasrc)
                            .fill_input_buffer
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(cinfo)
                        == 0
                    {
                        return 0 as libc::c_int;
                    }
                    next_input_byte = (*datasrc).next_input_byte;
                    bytes_in_buffer = (*datasrc).bytes_in_buffer
                }
                bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
                let fresh33 = next_input_byte;
                next_input_byte = next_input_byte.offset(1);
                tmp = *fresh33 as libc::c_uint
            }
            /* We convert the zigzag-order table to natural array order. */
            (*quant_ptr).quantval[*natural_order.offset(i as isize) as usize] =
                tmp as crate::jmorecfg_h::UINT16;
            i += 1
        }
        if (*(*cinfo).err).trace_level >= 2 as libc::c_int {
            i = 0 as libc::c_int as crate::jmorecfg_h::INT32;
            while i < 64 as libc::c_int as libc::c_long {
                let mut _mp: *mut libc::c_int = (*(*cinfo).err).msg_parm.i.as_mut_ptr();
                *_mp.offset(0 as libc::c_int as isize) =
                    (*quant_ptr).quantval[i as usize] as libc::c_int;
                *_mp.offset(1 as libc::c_int as isize) = (*quant_ptr).quantval
                    [(i + 1 as libc::c_int as libc::c_long) as usize]
                    as libc::c_int;
                *_mp.offset(2 as libc::c_int as isize) = (*quant_ptr).quantval
                    [(i + 2 as libc::c_int as libc::c_long) as usize]
                    as libc::c_int;
                *_mp.offset(3 as libc::c_int as isize) = (*quant_ptr).quantval
                    [(i + 3 as libc::c_int as libc::c_long) as usize]
                    as libc::c_int;
                *_mp.offset(4 as libc::c_int as isize) = (*quant_ptr).quantval
                    [(i + 4 as libc::c_int as libc::c_long) as usize]
                    as libc::c_int;
                *_mp.offset(5 as libc::c_int as isize) = (*quant_ptr).quantval
                    [(i + 5 as libc::c_int as libc::c_long) as usize]
                    as libc::c_int;
                *_mp.offset(6 as libc::c_int as isize) = (*quant_ptr).quantval
                    [(i + 6 as libc::c_int as libc::c_long) as usize]
                    as libc::c_int;
                *_mp.offset(7 as libc::c_int as isize) = (*quant_ptr).quantval
                    [(i + 7 as libc::c_int as libc::c_long) as usize]
                    as libc::c_int;
                (*(*cinfo).err).msg_code =
                    crate::src::jpeg_8c::jerror::JTRC_QUANTVALS as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .emit_message
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr,
                    2 as libc::c_int,
                );
                i += 8 as libc::c_int as libc::c_long
            }
        }
        length -= count;
        if prec != 0 {
            length -= count
        }
    }
    if length != 0 as libc::c_int as libc::c_long {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_LENGTH as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    (*datasrc).next_input_byte = next_input_byte;
    (*datasrc).bytes_in_buffer = bytes_in_buffer;
    return 1 as libc::c_int;
}

unsafe extern "C" fn get_dri(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
) -> crate::jmorecfg_h::boolean
/* Process a DRI marker */ {
    let mut length: crate::jmorecfg_h::INT32 = 0;
    let mut tmp: libc::c_uint = 0;
    let mut datasrc: *mut crate::jpeglib_h::jpeg_source_mgr = (*cinfo).src;
    let mut next_input_byte: *const crate::jmorecfg_h::JOCTET = (*datasrc).next_input_byte;
    let mut bytes_in_buffer: crate::stddef_h::size_t = (*datasrc).bytes_in_buffer;
    if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as libc::c_int;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh34 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length = ((*fresh34 as libc::c_uint) << 8 as libc::c_int) as crate::jmorecfg_h::INT32;
    if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as libc::c_int;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh35 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length += *fresh35 as libc::c_long;
    if length != 4 as libc::c_int as libc::c_long {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_LENGTH as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as libc::c_int;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh36 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    tmp = (*fresh36 as libc::c_uint) << 8 as libc::c_int;
    if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as libc::c_int;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh37 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    tmp = tmp.wrapping_add(*fresh37 as libc::c_uint);
    (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JTRC_DRI as libc::c_int;
    (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = tmp as libc::c_int;
    Some(
        (*(*cinfo).err)
            .emit_message
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr, 1 as libc::c_int
    );
    (*cinfo).restart_interval = tmp;
    (*datasrc).next_input_byte = next_input_byte;
    (*datasrc).bytes_in_buffer = bytes_in_buffer;
    return 1 as libc::c_int;
}
/* Must be the largest of the above!! */

unsafe extern "C" fn examine_app0(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut data: *mut crate::jmorecfg_h::JOCTET,
    mut datalen: libc::c_uint,
    mut remaining: crate::jmorecfg_h::INT32,
)
/* Examine first few bytes from an APP0.
 * Take appropriate action if it is a JFIF marker.
 * datalen is # of bytes at data[], remaining is length of rest of marker data.
 */
{
    let mut totallen: crate::jmorecfg_h::INT32 = datalen as crate::jmorecfg_h::INT32 + remaining;
    if datalen >= 14 as libc::c_int as libc::c_uint
        && *data.offset(0 as libc::c_int as isize) as libc::c_int == 0x4a as libc::c_int
        && *data.offset(1 as libc::c_int as isize) as libc::c_int == 0x46 as libc::c_int
        && *data.offset(2 as libc::c_int as isize) as libc::c_int == 0x49 as libc::c_int
        && *data.offset(3 as libc::c_int as isize) as libc::c_int == 0x46 as libc::c_int
        && *data.offset(4 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
    {
        /* Found JFIF APP0 marker: save info */
        (*cinfo).saw_JFIF_marker = 1 as libc::c_int;
        (*cinfo).JFIF_major_version = *data.offset(5 as libc::c_int as isize);
        (*cinfo).JFIF_minor_version = *data.offset(6 as libc::c_int as isize);
        (*cinfo).density_unit = *data.offset(7 as libc::c_int as isize);
        (*cinfo).X_density = (((*data.offset(8 as libc::c_int as isize) as libc::c_int)
            << 8 as libc::c_int)
            + *data.offset(9 as libc::c_int as isize) as libc::c_int)
            as crate::jmorecfg_h::UINT16;
        (*cinfo).Y_density = (((*data.offset(10 as libc::c_int as isize) as libc::c_int)
            << 8 as libc::c_int)
            + *data.offset(11 as libc::c_int as isize) as libc::c_int)
            as crate::jmorecfg_h::UINT16;
        /* Check version.
         * Major version must be 1, anything else signals an incompatible change.
         * (We used to treat this as an error, but now it's a nonfatal warning,
         * because some bozo at Hijaak couldn't read the spec.)
         * Minor version should be 0..2, but process anyway if newer.
         */
        if (*cinfo).JFIF_major_version as libc::c_int != 1 as libc::c_int {
            (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JWRN_JFIF_MAJOR as libc::c_int;
            (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] =
                (*cinfo).JFIF_major_version as libc::c_int;
            (*(*cinfo).err).msg_parm.i[1 as libc::c_int as usize] =
                (*cinfo).JFIF_minor_version as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .emit_message
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr,
                -(1 as libc::c_int),
            );
        }
        /* Generate trace messages */
        let mut _mp: *mut libc::c_int = (*(*cinfo).err).msg_parm.i.as_mut_ptr();
        *_mp.offset(0 as libc::c_int as isize) = (*cinfo).JFIF_major_version as libc::c_int;
        *_mp.offset(1 as libc::c_int as isize) = (*cinfo).JFIF_minor_version as libc::c_int;
        *_mp.offset(2 as libc::c_int as isize) = (*cinfo).X_density as libc::c_int;
        *_mp.offset(3 as libc::c_int as isize) = (*cinfo).Y_density as libc::c_int;
        *_mp.offset(4 as libc::c_int as isize) = (*cinfo).density_unit as libc::c_int;
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JTRC_JFIF as libc::c_int;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            1 as libc::c_int,
        );
        /* Validate thumbnail dimensions and issue appropriate messages */
        if *data.offset(12 as libc::c_int as isize) as libc::c_int
            | *data.offset(13 as libc::c_int as isize) as libc::c_int
            != 0
        {
            (*(*cinfo).err).msg_code =
                crate::src::jpeg_8c::jerror::JTRC_JFIF_THUMBNAIL as libc::c_int;
            (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] =
                *data.offset(12 as libc::c_int as isize) as libc::c_int;
            (*(*cinfo).err).msg_parm.i[1 as libc::c_int as usize] =
                *data.offset(13 as libc::c_int as isize) as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .emit_message
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr,
                1 as libc::c_int,
            );
        }
        totallen -= 14 as libc::c_int as libc::c_long;
        if totallen
            != *data.offset(12 as libc::c_int as isize) as crate::jmorecfg_h::INT32
                * *data.offset(13 as libc::c_int as isize) as crate::jmorecfg_h::INT32
                * 3 as libc::c_int as crate::jmorecfg_h::INT32
        {
            (*(*cinfo).err).msg_code =
                crate::src::jpeg_8c::jerror::JTRC_JFIF_BADTHUMBNAILSIZE as libc::c_int;
            (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = totallen as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .emit_message
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr,
                1 as libc::c_int,
            );
        }
    } else if datalen >= 6 as libc::c_int as libc::c_uint
        && *data.offset(0 as libc::c_int as isize) as libc::c_int == 0x4a as libc::c_int
        && *data.offset(1 as libc::c_int as isize) as libc::c_int == 0x46 as libc::c_int
        && *data.offset(2 as libc::c_int as isize) as libc::c_int == 0x58 as libc::c_int
        && *data.offset(3 as libc::c_int as isize) as libc::c_int == 0x58 as libc::c_int
        && *data.offset(4 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
    {
        /* Found JFIF "JFXX" extension APP0 marker */
        /* The library doesn't actually do anything with these,
         * but we try to produce a helpful trace message.
         */
        match *data.offset(5 as libc::c_int as isize) as libc::c_int {
            16 => {
                (*(*cinfo).err).msg_code =
                    crate::src::jpeg_8c::jerror::JTRC_THUMB_JPEG as libc::c_int;
                (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = totallen as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .emit_message
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr,
                    1 as libc::c_int,
                );
            }
            17 => {
                (*(*cinfo).err).msg_code =
                    crate::src::jpeg_8c::jerror::JTRC_THUMB_PALETTE as libc::c_int;
                (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = totallen as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .emit_message
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr,
                    1 as libc::c_int,
                );
            }
            19 => {
                (*(*cinfo).err).msg_code =
                    crate::src::jpeg_8c::jerror::JTRC_THUMB_RGB as libc::c_int;
                (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = totallen as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .emit_message
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr,
                    1 as libc::c_int,
                );
            }
            _ => {
                (*(*cinfo).err).msg_code =
                    crate::src::jpeg_8c::jerror::JTRC_JFIF_EXTENSION as libc::c_int;
                (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] =
                    *data.offset(5 as libc::c_int as isize) as libc::c_int;
                (*(*cinfo).err).msg_parm.i[1 as libc::c_int as usize] = totallen as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .emit_message
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr,
                    1 as libc::c_int,
                );
            }
        }
    } else {
        /* Start of APP0 does not match "JFIF" or "JFXX", or too short */
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JTRC_APP0 as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = totallen as libc::c_int;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            1 as libc::c_int,
        );
    };
}

unsafe extern "C" fn examine_app14(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut data: *mut crate::jmorecfg_h::JOCTET,
    mut datalen: libc::c_uint,
    mut remaining: crate::jmorecfg_h::INT32,
)
/* Examine first few bytes from an APP14.
 * Take appropriate action if it is an Adobe marker.
 * datalen is # of bytes at data[], remaining is length of rest of marker data.
 */
{
    let mut version: libc::c_uint = 0;
    let mut flags0: libc::c_uint = 0;
    let mut flags1: libc::c_uint = 0;
    let mut transform: libc::c_uint = 0;
    if datalen >= 12 as libc::c_int as libc::c_uint
        && *data.offset(0 as libc::c_int as isize) as libc::c_int == 0x41 as libc::c_int
        && *data.offset(1 as libc::c_int as isize) as libc::c_int == 0x64 as libc::c_int
        && *data.offset(2 as libc::c_int as isize) as libc::c_int == 0x6f as libc::c_int
        && *data.offset(3 as libc::c_int as isize) as libc::c_int == 0x62 as libc::c_int
        && *data.offset(4 as libc::c_int as isize) as libc::c_int == 0x65 as libc::c_int
    {
        /* Found Adobe APP14 marker */
        version = (((*data.offset(5 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int)
            + *data.offset(6 as libc::c_int as isize) as libc::c_int)
            as libc::c_uint;
        flags0 = (((*data.offset(7 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int)
            + *data.offset(8 as libc::c_int as isize) as libc::c_int)
            as libc::c_uint;
        flags1 = (((*data.offset(9 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int)
            + *data.offset(10 as libc::c_int as isize) as libc::c_int)
            as libc::c_uint;
        transform = *data.offset(11 as libc::c_int as isize) as libc::c_uint;
        let mut _mp: *mut libc::c_int = (*(*cinfo).err).msg_parm.i.as_mut_ptr();
        *_mp.offset(0 as libc::c_int as isize) = version as libc::c_int;
        *_mp.offset(1 as libc::c_int as isize) = flags0 as libc::c_int;
        *_mp.offset(2 as libc::c_int as isize) = flags1 as libc::c_int;
        *_mp.offset(3 as libc::c_int as isize) = transform as libc::c_int;
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JTRC_ADOBE as libc::c_int;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            1 as libc::c_int,
        );
        (*cinfo).saw_Adobe_marker = 1 as libc::c_int;
        (*cinfo).Adobe_transform = transform as crate::jmorecfg_h::UINT8
    } else {
        /* Start of APP14 does not match "Adobe", or too short */
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JTRC_APP14 as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] =
            (datalen as libc::c_long + remaining) as libc::c_int;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            1 as libc::c_int,
        );
    };
}

unsafe extern "C" fn get_interesting_appn(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
) -> crate::jmorecfg_h::boolean
/* Process an APP0 or APP14 marker without saving it */ {
    let mut length: crate::jmorecfg_h::INT32 = 0;
    let mut b: [crate::jmorecfg_h::JOCTET; 14] = [0; 14];
    let mut i: libc::c_uint = 0;
    let mut numtoread: libc::c_uint = 0;
    let mut datasrc: *mut crate::jpeglib_h::jpeg_source_mgr = (*cinfo).src;
    let mut next_input_byte: *const crate::jmorecfg_h::JOCTET = (*datasrc).next_input_byte;
    let mut bytes_in_buffer: crate::stddef_h::size_t = (*datasrc).bytes_in_buffer;
    if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as libc::c_int;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh38 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length = ((*fresh38 as libc::c_uint) << 8 as libc::c_int) as crate::jmorecfg_h::INT32;
    if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as libc::c_int;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh39 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length += *fresh39 as libc::c_long;
    length -= 2 as libc::c_int as libc::c_long;
    /* get the interesting part of the marker data */
    if length >= 14 as libc::c_int as libc::c_long {
        numtoread = 14 as libc::c_int as libc::c_uint
    } else if length > 0 as libc::c_int as libc::c_long {
        numtoread = length as libc::c_uint
    } else {
        numtoread = 0 as libc::c_int as libc::c_uint
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < numtoread {
        if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
            if Some(
                (*datasrc)
                    .fill_input_buffer
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo)
                == 0
            {
                return 0 as libc::c_int;
            }
            next_input_byte = (*datasrc).next_input_byte;
            bytes_in_buffer = (*datasrc).bytes_in_buffer
        }
        bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
        let fresh40 = next_input_byte;
        next_input_byte = next_input_byte.offset(1);
        b[i as usize] = *fresh40;
        i = i.wrapping_add(1)
    }
    length -= numtoread as libc::c_long;
    /* process it */
    match (*cinfo).unread_marker {
        224 => {
            examine_app0(cinfo, b.as_mut_ptr(), numtoread, length);
        }
        238 => {
            examine_app14(cinfo, b.as_mut_ptr(), numtoread, length);
        }
        _ => {
            /* can't get here unless jpeg_save_markers chooses wrong processor */
            (*(*cinfo).err).msg_code =
                crate::src::jpeg_8c::jerror::JERR_UNKNOWN_MARKER as libc::c_int;
            (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = (*cinfo).unread_marker;
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
    /* skip any remaining data -- could be lots */
    (*datasrc).next_input_byte = next_input_byte;
    (*datasrc).bytes_in_buffer = bytes_in_buffer;
    if length > 0 as libc::c_int as libc::c_long {
        Some(
            (*(*cinfo).src)
                .skip_input_data
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo, length);
    }
    return 1 as libc::c_int;
}

unsafe extern "C" fn save_marker(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
) -> crate::jmorecfg_h::boolean
/* Save an APPn or COM marker into the marker list */ {
    let mut marker: my_marker_ptr = (*cinfo).marker as my_marker_ptr;
    let mut cur_marker: crate::jpeglib_h::jpeg_saved_marker_ptr = (*marker).cur_marker;
    let mut bytes_read: libc::c_uint = 0;
    let mut data_length: libc::c_uint = 0;
    let mut data: *mut crate::jmorecfg_h::JOCTET = 0 as *mut crate::jmorecfg_h::JOCTET;
    let mut length: crate::jmorecfg_h::INT32 = 0 as libc::c_int as crate::jmorecfg_h::INT32;
    let mut datasrc: *mut crate::jpeglib_h::jpeg_source_mgr = (*cinfo).src;
    let mut next_input_byte: *const crate::jmorecfg_h::JOCTET = (*datasrc).next_input_byte;
    let mut bytes_in_buffer: crate::stddef_h::size_t = (*datasrc).bytes_in_buffer;
    if cur_marker.is_null() {
        /* begin reading a marker */
        if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
            if Some(
                (*datasrc)
                    .fill_input_buffer
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo)
                == 0
            {
                return 0 as libc::c_int;
            }
            next_input_byte = (*datasrc).next_input_byte;
            bytes_in_buffer = (*datasrc).bytes_in_buffer
        }
        bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
        let fresh41 = next_input_byte;
        next_input_byte = next_input_byte.offset(1);
        length = ((*fresh41 as libc::c_uint) << 8 as libc::c_int) as crate::jmorecfg_h::INT32;
        if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
            if Some(
                (*datasrc)
                    .fill_input_buffer
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo)
                == 0
            {
                return 0 as libc::c_int;
            }
            next_input_byte = (*datasrc).next_input_byte;
            bytes_in_buffer = (*datasrc).bytes_in_buffer
        }
        bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
        let fresh42 = next_input_byte;
        next_input_byte = next_input_byte.offset(1);
        length += *fresh42 as libc::c_long;
        length -= 2 as libc::c_int as libc::c_long;
        if length >= 0 as libc::c_int as libc::c_long {
            /* watch out for bogus length word */
            /* figure out how much we want to save */
            let mut limit: libc::c_uint = 0;
            if (*cinfo).unread_marker == M_COM as libc::c_int {
                limit = (*marker).length_limit_COM
            } else {
                limit = (*marker).length_limit_APPn
                    [((*cinfo).unread_marker - M_APP0 as libc::c_int) as usize]
            }
            if (length as libc::c_uint) < limit {
                limit = length as libc::c_uint
            }
            /* allocate and initialize the marker item */
            cur_marker = Some(
                (*(*cinfo).mem)
                    .alloc_large
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr,
                1 as libc::c_int,
                (::std::mem::size_of::<crate::jpeglib_h::jpeg_marker_struct>() as libc::c_ulong)
                    .wrapping_add(limit as libc::c_ulong),
            ) as crate::jpeglib_h::jpeg_saved_marker_ptr;
            (*cur_marker).next = 0 as crate::jpeglib_h::jpeg_saved_marker_ptr;
            (*cur_marker).marker = (*cinfo).unread_marker as crate::jmorecfg_h::UINT8;
            (*cur_marker).original_length = length as libc::c_uint;
            (*cur_marker).data_length = limit;
            /* data area is just beyond the jpeg_marker_struct */
            (*cur_marker).data =
                cur_marker.offset(1 as libc::c_int as isize) as *mut crate::jmorecfg_h::JOCTET;
            data = (*cur_marker).data;
            (*marker).cur_marker = cur_marker;
            (*marker).bytes_read = 0 as libc::c_int as libc::c_uint;
            bytes_read = 0 as libc::c_int as libc::c_uint;
            data_length = limit
        } else {
            /* deal with bogus length word */
            data_length = 0 as libc::c_int as libc::c_uint;
            bytes_read = data_length;
            data = 0 as *mut crate::jmorecfg_h::JOCTET
        }
    } else {
        /* resume reading a marker */
        bytes_read = (*marker).bytes_read; /* move the restart point to here */
        data_length = (*cur_marker).data_length;
        data = (*cur_marker).data.offset(bytes_read as isize)
    }
    while bytes_read < data_length {
        (*datasrc).next_input_byte = next_input_byte;
        (*datasrc).bytes_in_buffer = bytes_in_buffer;
        (*marker).bytes_read = bytes_read;
        /* If there's not at least one byte in buffer, suspend */
        if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
            if Some(
                (*datasrc)
                    .fill_input_buffer
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo)
                == 0
            {
                return 0 as libc::c_int;
            }
            next_input_byte = (*datasrc).next_input_byte;
            bytes_in_buffer = (*datasrc).bytes_in_buffer
        }
        /* Copy bytes with reasonable rapidity */
        while bytes_read < data_length && bytes_in_buffer > 0 as libc::c_int as libc::c_ulong {
            let fresh43 = next_input_byte;
            next_input_byte = next_input_byte.offset(1);
            let fresh44 = data;
            data = data.offset(1);
            *fresh44 = *fresh43;
            bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
            bytes_read = bytes_read.wrapping_add(1)
        }
    }
    /* Done reading what we want to read */
    if !cur_marker.is_null() {
        /* will be NULL if bogus length word */
        /* Add new marker to end of list */
        if (*cinfo).marker_list.is_null() {
            (*cinfo).marker_list = cur_marker
        } else {
            let mut prev: crate::jpeglib_h::jpeg_saved_marker_ptr = (*cinfo).marker_list;
            while !(*prev).next.is_null() {
                prev = (*prev).next
            }
            (*prev).next = cur_marker
        }
        /* Reset pointer & calc remaining data length */
        data = (*cur_marker).data;
        length = (*cur_marker).original_length.wrapping_sub(data_length) as crate::jmorecfg_h::INT32
    }
    /* Reset to initial state for next marker */
    (*marker).cur_marker = 0 as crate::jpeglib_h::jpeg_saved_marker_ptr;
    /* Process the marker if interesting; else just make a generic trace msg */
    match (*cinfo).unread_marker {
        224 => {
            examine_app0(cinfo, data, data_length, length);
        }
        238 => {
            examine_app14(cinfo, data, data_length, length);
        }
        _ => {
            (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JTRC_MISC_MARKER as libc::c_int;
            (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = (*cinfo).unread_marker;
            (*(*cinfo).err).msg_parm.i[1 as libc::c_int as usize] =
                (data_length as libc::c_long + length) as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .emit_message
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr,
                1 as libc::c_int,
            );
        }
    }
    /* skip any remaining data -- could be lots */
    (*datasrc).next_input_byte = next_input_byte; /* do before skip_input_data */
    (*datasrc).bytes_in_buffer = bytes_in_buffer;
    if length > 0 as libc::c_int as libc::c_long {
        Some(
            (*(*cinfo).src)
                .skip_input_data
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo, length);
    }
    return 1 as libc::c_int;
}
/* SAVE_MARKERS_SUPPORTED */

unsafe extern "C" fn skip_variable(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
) -> crate::jmorecfg_h::boolean
/* Skip over an unknown or uninteresting variable-length marker */ {
    let mut length: crate::jmorecfg_h::INT32 = 0; /* do before skip_input_data */
    let mut datasrc: *mut crate::jpeglib_h::jpeg_source_mgr = (*cinfo).src;
    let mut next_input_byte: *const crate::jmorecfg_h::JOCTET = (*datasrc).next_input_byte;
    let mut bytes_in_buffer: crate::stddef_h::size_t = (*datasrc).bytes_in_buffer;
    if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as libc::c_int;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh45 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length = ((*fresh45 as libc::c_uint) << 8 as libc::c_int) as crate::jmorecfg_h::INT32;
    if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as libc::c_int;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh46 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    length += *fresh46 as libc::c_long;
    length -= 2 as libc::c_int as libc::c_long;
    (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JTRC_MISC_MARKER as libc::c_int;
    (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = (*cinfo).unread_marker;
    (*(*cinfo).err).msg_parm.i[1 as libc::c_int as usize] = length as libc::c_int;
    Some(
        (*(*cinfo).err)
            .emit_message
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr, 1 as libc::c_int
    );
    (*datasrc).next_input_byte = next_input_byte;
    (*datasrc).bytes_in_buffer = bytes_in_buffer;
    if length > 0 as libc::c_int as libc::c_long {
        Some(
            (*(*cinfo).src)
                .skip_input_data
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo, length);
    }
    return 1 as libc::c_int;
}
/*
 * Find the next JPEG marker, save it in cinfo->unread_marker.
 * Returns FALSE if had to suspend before reaching a marker;
 * in that case cinfo->unread_marker is unchanged.
 *
 * Note that the result might not be a valid marker code,
 * but it will never be 0 or FF.
 */

unsafe extern "C" fn next_marker(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
) -> crate::jmorecfg_h::boolean {
    let mut c: libc::c_int = 0;
    let mut datasrc: *mut crate::jpeglib_h::jpeg_source_mgr = (*cinfo).src;
    let mut next_input_byte: *const crate::jmorecfg_h::JOCTET = (*datasrc).next_input_byte;
    let mut bytes_in_buffer: crate::stddef_h::size_t = (*datasrc).bytes_in_buffer;
    loop {
        if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
            if Some(
                (*datasrc)
                    .fill_input_buffer
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo)
                == 0
            {
                return 0 as libc::c_int;
            }
            next_input_byte = (*datasrc).next_input_byte;
            bytes_in_buffer = (*datasrc).bytes_in_buffer
        }
        bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
        let fresh47 = next_input_byte;
        next_input_byte = next_input_byte.offset(1);
        c = *fresh47 as libc::c_int;
        /* Skip any non-FF bytes.
         * This may look a bit inefficient, but it will not occur in a valid file.
         * We sync after each discarded byte so that a suspending data source
         * can discard the byte from its buffer.
         */
        while c != 0xff as libc::c_int {
            (*(*cinfo).marker).discarded_bytes = (*(*cinfo).marker).discarded_bytes.wrapping_add(1);
            (*datasrc).next_input_byte = next_input_byte;
            (*datasrc).bytes_in_buffer = bytes_in_buffer;
            if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
                if Some(
                    (*datasrc)
                        .fill_input_buffer
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo)
                    == 0
                {
                    return 0 as libc::c_int;
                }
                next_input_byte = (*datasrc).next_input_byte;
                bytes_in_buffer = (*datasrc).bytes_in_buffer
            }
            bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
            let fresh48 = next_input_byte;
            next_input_byte = next_input_byte.offset(1);
            c = *fresh48 as libc::c_int
        }
        loop
        /* This loop swallows any duplicate FF bytes.  Extra FFs are legal as
         * pad bytes, so don't count them in discarded_bytes.  We assume there
         * will not be so many consecutive FF bytes as to overflow a suspending
         * data source's input buffer.
         */
        {
            if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
                if Some(
                    (*datasrc)
                        .fill_input_buffer
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo)
                    == 0
                {
                    return 0 as libc::c_int;
                } /* found a valid marker, exit loop */
                next_input_byte = (*datasrc).next_input_byte;
                bytes_in_buffer = (*datasrc).bytes_in_buffer
            }
            bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
            let fresh49 = next_input_byte;
            next_input_byte = next_input_byte.offset(1);
            c = *fresh49 as libc::c_int;
            if !(c == 0xff as libc::c_int) {
                break;
            }
        }
        if c != 0 as libc::c_int {
            break;
        }
        /* Reach here if we found a stuffed-zero data sequence (FF/00).
         * Discard it and loop back to try again.
         */
        (*(*cinfo).marker).discarded_bytes = (*(*cinfo).marker)
            .discarded_bytes
            .wrapping_add(2 as libc::c_int as libc::c_uint);
        (*datasrc).next_input_byte = next_input_byte;
        (*datasrc).bytes_in_buffer = bytes_in_buffer
    }
    if (*(*cinfo).marker).discarded_bytes != 0 as libc::c_int as libc::c_uint {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JWRN_EXTRANEOUS_DATA as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] =
            (*(*cinfo).marker).discarded_bytes as libc::c_int;
        (*(*cinfo).err).msg_parm.i[1 as libc::c_int as usize] = c;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            -(1 as libc::c_int),
        );
        (*(*cinfo).marker).discarded_bytes = 0 as libc::c_int as libc::c_uint
    }
    (*cinfo).unread_marker = c;
    (*datasrc).next_input_byte = next_input_byte;
    (*datasrc).bytes_in_buffer = bytes_in_buffer;
    return 1 as libc::c_int;
}

unsafe extern "C" fn first_marker(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
) -> crate::jmorecfg_h::boolean
/* Like next_marker, but used to obtain the initial SOI marker. */
/* For this marker, we do not allow preceding garbage or fill; otherwise,
 * we might well scan an entire input file before realizing it ain't JPEG.
 * If an application wants to process non-JFIF files, it must seek to the
 * SOI before calling the JPEG library.
 */ {
    let mut c: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    let mut datasrc: *mut crate::jpeglib_h::jpeg_source_mgr = (*cinfo).src;
    let mut next_input_byte: *const crate::jmorecfg_h::JOCTET = (*datasrc).next_input_byte;
    let mut bytes_in_buffer: crate::stddef_h::size_t = (*datasrc).bytes_in_buffer;
    if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as libc::c_int;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh50 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    c = *fresh50 as libc::c_int;
    if bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
        if Some(
            (*datasrc)
                .fill_input_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            return 0 as libc::c_int;
        }
        next_input_byte = (*datasrc).next_input_byte;
        bytes_in_buffer = (*datasrc).bytes_in_buffer
    }
    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
    let fresh51 = next_input_byte;
    next_input_byte = next_input_byte.offset(1);
    c2 = *fresh51 as libc::c_int;
    if c != 0xff as libc::c_int || c2 != M_SOI as libc::c_int {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_NO_SOI as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = c;
        (*(*cinfo).err).msg_parm.i[1 as libc::c_int as usize] = c2;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    (*cinfo).unread_marker = c2;
    (*datasrc).next_input_byte = next_input_byte;
    (*datasrc).bytes_in_buffer = bytes_in_buffer;
    return 1 as libc::c_int;
}
/*
 * Read markers until SOS or EOI.
 *
 * Returns same codes as are defined for jpeg_consume_input:
 * JPEG_SUSPENDED, JPEG_REACHED_SOS, or JPEG_REACHED_EOI.
 *
 * Note: This function may return a pseudo SOS marker (with zero
 * component number) for treat by input controller's consume_input.
 * consume_input itself should filter out (skip) the pseudo marker
 * after processing for the caller.
 */

unsafe extern "C" fn read_markers(mut cinfo: crate::jpeglib_h::j_decompress_ptr) -> libc::c_int {
    loop
    /* Outer loop repeats once for each marker. */
    /* Collect the marker proper, unless we already did. */
    /* NB: first_marker() enforces the requirement that SOI appear first. */
    {
        if (*cinfo).unread_marker == 0 as libc::c_int {
            if (*(*cinfo).marker).saw_SOI == 0 {
                if first_marker(cinfo) == 0 {
                    return 0 as libc::c_int;
                }
            } else if next_marker(cinfo) == 0 {
                return 0 as libc::c_int;
            }
        }
        let mut current_block_44: u64;
        /* At this point cinfo->unread_marker contains the marker code and the
         * input point is just past the marker proper, but before any parameters.
         * A suspension will cause us to return with this state still true.
         */
        match (*cinfo).unread_marker {
            216 => {
                if get_soi(cinfo) == 0 {
                    return 0 as libc::c_int;
                }
                current_block_44 = 13707613154239713890;
            }
            192 => {
                /* Baseline */
                if get_sof(cinfo, 1 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int) == 0 {
                    return 0 as libc::c_int;
                }
                current_block_44 = 13707613154239713890;
            }
            193 => {
                /* Extended sequential, Huffman */
                if get_sof(cinfo, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int) == 0 {
                    return 0 as libc::c_int;
                }
                current_block_44 = 13707613154239713890;
            }
            194 => {
                /* Progressive, Huffman */
                if get_sof(cinfo, 0 as libc::c_int, 1 as libc::c_int, 0 as libc::c_int) == 0 {
                    return 0 as libc::c_int;
                }
                current_block_44 = 13707613154239713890;
            }
            201 => {
                /* Extended sequential, arithmetic */
                if get_sof(cinfo, 0 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int) == 0 {
                    return 0 as libc::c_int;
                }
                current_block_44 = 13707613154239713890;
            }
            202 => {
                /* Progressive, arithmetic */
                if get_sof(cinfo, 0 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int) == 0 {
                    return 0 as libc::c_int;
                }
                current_block_44 = 13707613154239713890;
            }
            195 => {
                /* Lossless, Huffman */
                current_block_44 = 9345900496200566726; /* processed the marker */
            }
            197 => {
                current_block_44 = 9345900496200566726; /* processed the marker */
            }
            198 => {
                current_block_44 = 14663562318465450808;
            }
            199 => {
                current_block_44 = 9149589725794138869;
            }
            200 => {
                current_block_44 = 15860528996264011284;
            }
            203 => {
                current_block_44 = 15760560705920806731;
            }
            205 => {
                current_block_44 = 14385744071564360658;
            }
            206 | 207 => {
                current_block_44 = 7958616763515265996;
            }
            218 => {
                if get_sos(cinfo) == 0 {
                    return 0 as libc::c_int;
                }
                (*cinfo).unread_marker = 0 as libc::c_int;
                return 1 as libc::c_int;
            }
            217 => {
                (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JTRC_EOI as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .emit_message
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr,
                    1 as libc::c_int,
                );
                (*cinfo).unread_marker = 0 as libc::c_int;
                return 2 as libc::c_int;
            }
            204 => {
                if get_dac(cinfo) == 0 {
                    return 0 as libc::c_int;
                }
                current_block_44 = 13707613154239713890;
            }
            196 => {
                if get_dht(cinfo) == 0 {
                    return 0 as libc::c_int;
                }
                current_block_44 = 13707613154239713890;
            }
            219 => {
                if get_dqt(cinfo) == 0 {
                    return 0 as libc::c_int;
                }
                current_block_44 = 13707613154239713890;
            }
            221 => {
                if get_dri(cinfo) == 0 {
                    return 0 as libc::c_int;
                }
                current_block_44 = 13707613154239713890;
            }
            224 | 225 | 226 | 227 | 228 | 229 | 230 | 231 | 232 | 233 | 234 | 235 | 236 | 237
            | 238 | 239 => {
                if Some(
                    (*(*((*cinfo).marker as my_marker_ptr))
                        .process_APPn
                        .as_mut_ptr()
                        .offset(((*cinfo).unread_marker - M_APP0 as libc::c_int) as isize))
                    .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo)
                    == 0
                {
                    return 0 as libc::c_int;
                }
                current_block_44 = 13707613154239713890;
            }
            254 => {
                if Some(
                    (*((*cinfo).marker as my_marker_ptr))
                        .process_COM
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo)
                    == 0
                {
                    return 0 as libc::c_int;
                }
                current_block_44 = 13707613154239713890;
            }
            208 | 209 | 210 | 211 | 212 | 213 | 214 | 215 | 1 => {
                /* these are all parameterless */
                (*(*cinfo).err).msg_code =
                    crate::src::jpeg_8c::jerror::JTRC_PARMLESS_MARKER as libc::c_int;
                (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = (*cinfo).unread_marker;
                Some(
                    (*(*cinfo).err)
                        .emit_message
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr,
                    1 as libc::c_int,
                );
                current_block_44 = 13707613154239713890;
            }
            220 => {
                /* Ignore DNL ... perhaps the wrong thing */
                if skip_variable(cinfo) == 0 {
                    return 0 as libc::c_int;
                }
                current_block_44 = 13707613154239713890;
            }
            _ => {
                /* must be DHP, EXP, JPGn, or RESn */
                /* For now, we treat the reserved markers as fatal errors since they are
                 * likely to be used to signal incompatible JPEG Part 3 extensions.
                 * Once the JPEG 3 version-number marker is well defined, this code
                 * ought to change!
                 */
                (*(*cinfo).err).msg_code =
                    crate::src::jpeg_8c::jerror::JERR_UNKNOWN_MARKER as libc::c_int;
                (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = (*cinfo).unread_marker;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
                current_block_44 = 13707613154239713890;
            }
        }
        match current_block_44 {
            9345900496200566726 =>
            /* Differential sequential, Huffman */
            {
                current_block_44 = 14663562318465450808;
            }
            _ => {}
        }
        match current_block_44 {
            14663562318465450808 =>
            /* Differential progressive, Huffman */
            {
                current_block_44 = 9149589725794138869;
            }
            _ => {}
        }
        match current_block_44 {
            9149589725794138869 =>
            /* Differential lossless, Huffman */
            {
                current_block_44 = 15860528996264011284;
            }
            _ => {}
        }
        match current_block_44 {
            15860528996264011284 =>
            /* Reserved for JPEG extensions */
            {
                current_block_44 = 15760560705920806731;
            }
            _ => {}
        }
        match current_block_44 {
            15760560705920806731 =>
            /* Lossless, arithmetic */
            {
                current_block_44 = 14385744071564360658;
            }
            _ => {}
        }
        match current_block_44 {
            14385744071564360658 =>
            /* Differential sequential, arithmetic */
            {
                current_block_44 = 7958616763515265996;
            }
            _ => {}
        }
        match current_block_44 {
            7958616763515265996 =>
            /* Differential progressive, arithmetic */
            /* Differential lossless, arithmetic */
            {
                (*(*cinfo).err).msg_code =
                    crate::src::jpeg_8c::jerror::JERR_SOF_UNSUPPORTED as libc::c_int;
                (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = (*cinfo).unread_marker;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            _ => {}
        }
        /* Successfully processed marker, so reset state variable */
        (*cinfo).unread_marker = 0 as libc::c_int
    }
    /* end loop */
}
/*
 * Read a restart marker, which is expected to appear next in the datastream;
 * if the marker is not there, take appropriate recovery action.
 * Returns FALSE if suspension is required.
 *
 * This is called by the entropy decoder after it has read an appropriate
 * number of MCUs.  cinfo->unread_marker may be nonzero if the entropy decoder
 * has already read a marker from the data source.  Under normal conditions
 * cinfo->unread_marker will be reset to 0 before returning; if not reset,
 * it holds a marker which the decoder will be unable to read past.
 */

unsafe extern "C" fn read_restart_marker(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
) -> crate::jmorecfg_h::boolean {
    /* Obtain a marker unless we already did. */
    /* Note that next_marker will complain if it skips any data. */
    if (*cinfo).unread_marker == 0 as libc::c_int {
        if next_marker(cinfo) == 0 {
            return 0 as libc::c_int;
        }
    }
    if (*cinfo).unread_marker == M_RST0 as libc::c_int + (*(*cinfo).marker).next_restart_num {
        /* Normal case --- swallow the marker and let entropy decoder continue */
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JTRC_RST as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = (*(*cinfo).marker).next_restart_num;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            3 as libc::c_int,
        );
        (*cinfo).unread_marker = 0 as libc::c_int
    } else if Some(
        (*(*cinfo).src)
            .resync_to_restart
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo, (*(*cinfo).marker).next_restart_num)
        == 0
    {
        return 0 as libc::c_int;
    }
    /* Uh-oh, the restart markers have been messed up. */
    /* Let the data source manager determine how to resync. */
    /* Update next-restart state */
    (*(*cinfo).marker).next_restart_num =
        (*(*cinfo).marker).next_restart_num + 1 as libc::c_int & 7 as libc::c_int;
    return 1 as libc::c_int;
}
/* Read or write raw DCT coefficients --- useful for lossless transcoding. */
/* If you choose to abort compression or decompression before completing
 * jpeg_finish_(de)compress, then you need to clean up to release memory,
 * temporary files, etc.  You can just call jpeg_destroy_(de)compress
 * if you're done with the JPEG object, but if you want to clean it up and
 * reuse it, call this:
 */
/* Generic versions of jpeg_abort and jpeg_destroy that work on either
 * flavor of JPEG object.  These may be more convenient in some places.
 */
/* Default restart-marker-resync procedure for use by data source modules */
/*
 * This is the default resync_to_restart method for data source managers
 * to use if they don't have any better approach.  Some data source managers
 * may be able to back up, or may have additional knowledge about the data
 * which permits a more intelligent recovery strategy; such managers would
 * presumably supply their own resync method.
 *
 * read_restart_marker calls resync_to_restart if it finds a marker other than
 * the restart marker it was expecting.  (This code is *not* used unless
 * a nonzero restart interval has been declared.)  cinfo->unread_marker is
 * the marker code actually found (might be anything, except 0 or FF).
 * The desired restart marker number (0..7) is passed as a parameter.
 * This routine is supposed to apply whatever error recovery strategy seems
 * appropriate in order to position the input stream to the next data segment.
 * Note that cinfo->unread_marker is treated as a marker appearing before
 * the current data-source input point; usually it should be reset to zero
 * before returning.
 * Returns FALSE if suspension is required.
 *
 * This implementation is substantially constrained by wanting to treat the
 * input as a data stream; this means we can't back up.  Therefore, we have
 * only the following actions to work with:
 *   1. Simply discard the marker and let the entropy decoder resume at next
 *      byte of file.
 *   2. Read forward until we find another marker, discarding intervening
 *      data.  (In theory we could look ahead within the current bufferload,
 *      without having to discard data if we don't find the desired marker.
 *      This idea is not implemented here, in part because it makes behavior
 *      dependent on buffer size and chance buffer-boundary positions.)
 *   3. Leave the marker unread (by failing to zero cinfo->unread_marker).
 *      This will cause the entropy decoder to process an empty data segment,
 *      inserting dummy zeroes, and then we will reprocess the marker.
 *
 * #2 is appropriate if we think the desired marker lies ahead, while #3 is
 * appropriate if the found marker is a future restart marker (indicating
 * that we have missed the desired restart marker, probably because it got
 * corrupted).
 * We apply #2 or #3 if the found marker is a restart marker no more than
 * two counts behind or ahead of the expected one.  We also apply #2 if the
 * found marker is not a legal JPEG marker code (it's certainly bogus data).
 * If the found marker is a restart marker more than 2 counts away, we do #1
 * (too much risk that the marker is erroneous; with luck we will be able to
 * resync at some future point).
 * For any valid non-restart JPEG marker, we apply #3.  This keeps us from
 * overrunning the end of a scan.  An implementation limited to single-scan
 * files might find it better to apply #2 for markers other than EOI, since
 * any other marker would have to be bogus data in that case.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_resync_to_restart(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut desired: libc::c_int,
) -> crate::jmorecfg_h::boolean {
    let mut marker: libc::c_int = (*cinfo).unread_marker;
    let mut action: libc::c_int = 1 as libc::c_int;
    /* Always put up a warning. */
    (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JWRN_MUST_RESYNC as libc::c_int;
    (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = marker;
    (*(*cinfo).err).msg_parm.i[1 as libc::c_int as usize] = desired;
    Some(
        (*(*cinfo).err)
            .emit_message
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        -(1 as libc::c_int),
    );
    loop
    /* Outer loop handles repeated decision after scanning forward. */
    {
        if marker < M_SOF0 as libc::c_int {
            action = 2 as libc::c_int
        } else if marker < M_RST0 as libc::c_int || marker > M_RST7 as libc::c_int {
            /* invalid marker */
            action = 3 as libc::c_int
        } else if marker == M_RST0 as libc::c_int + (desired + 1 as libc::c_int & 7 as libc::c_int)
            || marker == M_RST0 as libc::c_int + (desired + 2 as libc::c_int & 7 as libc::c_int)
        {
            /* valid non-restart marker */
            action = 3 as libc::c_int
        } else if marker == M_RST0 as libc::c_int + (desired - 1 as libc::c_int & 7 as libc::c_int)
            || marker == M_RST0 as libc::c_int + (desired - 2 as libc::c_int & 7 as libc::c_int)
        {
            /* one of the next two expected restarts */
            action = 2 as libc::c_int
        } else {
            action = 1 as libc::c_int
        } /* a prior restart, so advance */
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JTRC_RECOVERY_ACTION as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = marker;
        (*(*cinfo).err).msg_parm.i[1 as libc::c_int as usize] = action;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            4 as libc::c_int,
        );
        match action {
            1 => {
                /* desired restart or too far away */
                /* Discard marker and let entropy decoder resume processing. */
                (*cinfo).unread_marker = 0 as libc::c_int;
                return 1 as libc::c_int;
            }
            2 => {
                /* Scan to the next marker, and repeat the decision loop. */
                if next_marker(cinfo) == 0 {
                    return 0 as libc::c_int;
                }
                marker = (*cinfo).unread_marker
            }
            3 => {
                /* Return without advancing past this marker. */
                /* Entropy decoder will be forced to process an empty segment. */
                return 1 as libc::c_int;
            }
            _ => {}
        }
    }
    /* end loop */
}
/*
 * Reset marker processing state to begin a fresh datastream.
 */

unsafe extern "C" fn reset_marker_reader(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    let mut marker: my_marker_ptr = (*cinfo).marker as my_marker_ptr; /* until allocated by get_sof */
    (*cinfo).comp_info = 0 as *mut crate::jpeglib_h::jpeg_component_info; /* no SOS seen yet */
    (*cinfo).input_scan_number = 0 as libc::c_int; /* no pending marker */
    (*cinfo).unread_marker = 0 as libc::c_int; /* set internal state too */
    (*marker).pub_0.saw_SOI = 0 as libc::c_int;
    (*marker).pub_0.saw_SOF = 0 as libc::c_int;
    (*marker).pub_0.discarded_bytes = 0 as libc::c_int as libc::c_uint;
    (*marker).cur_marker = 0 as crate::jpeglib_h::jpeg_saved_marker_ptr;
}
/*
 * jpegint.h
 *
 * Copyright (C) 1991-1997, Thomas G. Lane.
 * Modified 1997-2009 by Guido Vollbeding.
 * This file is part of the Independent JPEG Group's software.
 * For conditions of distribution and use, see the accompanying README file.
 *
 * This file provides common declarations for the various JPEG modules.
 * These declarations are considered internal to the JPEG library; most
 * applications using the library shouldn't need to include this file.
 */
/* Declarations for both compression & decompression */
/* Operating modes for buffer controllers */
/* Plain stripwise operation */
/* Remaining modes require a full-image buffer to have been created */
/* Run source subobject only, save output */
/* Run dest subobject only, using saved data */
/* Run both subobjects, save output */
/* Values of global_state field (jdapi.c has some dependencies on ordering!) */
/* after create_compress */
/* start_compress done, write_scanlines OK */
/* start_compress done, write_raw_data OK */
/* jpeg_write_coefficients done */
/* after create_decompress */
/* reading header markers, no SOS yet */
/* found SOS, ready for start_decompress */
/* reading multiscan file in start_decompress*/
/* performing dummy pass for 2-pass quant */
/* start_decompress done, read_scanlines OK */
/* start_decompress done, read_raw_data OK */
/* expecting jpeg_start_output */
/* looking for SOS/EOI in jpeg_finish_output */
/* reading file in jpeg_read_coefficients */
/* looking for EOI in jpeg_finish_decompress */
/* Declarations for compression modules */
/* Master control module */
/* State variables made visible to other modules */
/* True if pass_startup must be called */
/* True during last pass */
/* Main buffer control (downsampled-data buffer) */
/* Compression preprocessing (downsampling input buffer control) */
/* Coefficient buffer control */
/* Colorspace conversion */
/* Downsampling */
/* TRUE if need rows above & below */
/* Forward DCT (also controls coefficient quantization) */
/* It is useful to allow each component to have a separate FDCT method. */
/* Entropy encoding */
/* Marker writing */
/* These routines are exported to allow insertion of extra markers */
/* Probably only COM and APPn markers should be written this way */
/* Declarations for decompression modules */
/* Master control module */
/* State variables made visible to other modules */
/* True during 1st pass for 2-pass quant */
/* Input control module */
/* State variables made visible to other modules */
/* True if file has multiple scans */
/* True when EOI has been consumed */
/* Main buffer control (downsampled-data buffer) */
/* Coefficient buffer control */
/* Pointer to array of coefficient virtual arrays, or NULL if none */
/* Decompression postprocessing (color quantization buffer control) */
/* Marker reading & parsing */
/* Read markers until SOS or EOI.
 * Returns same codes as are defined for jpeg_consume_input:
 * JPEG_SUSPENDED, JPEG_REACHED_SOS, or JPEG_REACHED_EOI.
 */
/* Read a restart marker --- exported for use by entropy decoder only */
/* State of marker reader --- nominally internal, but applications
 * supplying COM or APPn handlers might like to know the state.
 */
/* found SOI? */
/* found SOF? */
/* next restart number expected (0-7) */
/* # of bytes skipped looking for a marker */
/* Entropy decoding */
/* Inverse DCT (also performs dequantization) */
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
 * Initialize the marker reader module.
 * This is called only once, when the decompression object is created.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_marker_reader(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    let mut marker: my_marker_ptr = 0 as *mut my_marker_reader;
    let mut i: libc::c_int = 0;
    /* Create subobject in permanent pool */
    marker = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        0 as libc::c_int,
        ::std::mem::size_of::<my_marker_reader>() as libc::c_ulong,
    ) as my_marker_ptr;
    (*cinfo).marker = marker as *mut crate::jpegint_h::jpeg_marker_reader;
    /* Initialize public method pointers */
    (*marker).pub_0.reset_marker_reader = Some(
        reset_marker_reader as unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> (),
    );
    (*marker).pub_0.read_markers = Some(
        read_markers as unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> libc::c_int,
    );
    (*marker).pub_0.read_restart_marker = Some(
        read_restart_marker
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_decompress_ptr,
            ) -> crate::jmorecfg_h::boolean,
    );
    /* Initialize COM/APPn processing.
     * By default, we examine and then discard APP0 and APP14,
     * but simply discard COM and all other APPn.
     */
    (*marker).process_COM = Some(
        skip_variable
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_decompress_ptr,
            ) -> crate::jmorecfg_h::boolean,
    );
    (*marker).length_limit_COM = 0 as libc::c_int as libc::c_uint;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        (*marker).process_APPn[i as usize] = Some(
            skip_variable
                as unsafe extern "C" fn(
                    _: crate::jpeglib_h::j_decompress_ptr,
                ) -> crate::jmorecfg_h::boolean,
        );
        (*marker).length_limit_APPn[i as usize] = 0 as libc::c_int as libc::c_uint;
        i += 1
    }
    (*marker).process_APPn[0 as libc::c_int as usize] = Some(
        get_interesting_appn
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_decompress_ptr,
            ) -> crate::jmorecfg_h::boolean,
    );
    (*marker).process_APPn[14 as libc::c_int as usize] = Some(
        get_interesting_appn
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_decompress_ptr,
            ) -> crate::jmorecfg_h::boolean,
    );
    /* Reset marker processing state */
    reset_marker_reader(cinfo);
}
/*
 * Control saving of COM and APPn markers into marker_list.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_save_markers(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut marker_code: libc::c_int,
    mut length_limit: libc::c_uint,
) {
    let mut marker: my_marker_ptr = (*cinfo).marker as my_marker_ptr;
    let mut maxlength: libc::c_long = 0;
    let mut processor: crate::jpeglib_h::jpeg_marker_parser_method = None;
    /* Length limit mustn't be larger than what we can allocate
     * (should only be a concern in a 16-bit environment).
     */
    maxlength =
        ((*(*cinfo).mem).max_alloc_chunk as libc::c_ulong)
            .wrapping_sub(
                ::std::mem::size_of::<crate::jpeglib_h::jpeg_marker_struct>() as libc::c_ulong,
            ) as libc::c_long;
    if length_limit as libc::c_long > maxlength {
        length_limit = maxlength as libc::c_uint
    }
    /* Choose processor routine to use.
     * APP0/APP14 have special requirements.
     */
    if length_limit != 0 {
        processor = Some(
            save_marker
                as unsafe extern "C" fn(
                    _: crate::jpeglib_h::j_decompress_ptr,
                ) -> crate::jmorecfg_h::boolean,
        );
        /* If saving APP0/APP14, save at least enough for our internal use. */
        if marker_code == M_APP0 as libc::c_int && length_limit < 14 as libc::c_int as libc::c_uint
        {
            length_limit = 14 as libc::c_int as libc::c_uint
        } else if marker_code == M_APP14 as libc::c_int
            && length_limit < 12 as libc::c_int as libc::c_uint
        {
            length_limit = 12 as libc::c_int as libc::c_uint
        }
    } else {
        processor = Some(
            skip_variable
                as unsafe extern "C" fn(
                    _: crate::jpeglib_h::j_decompress_ptr,
                ) -> crate::jmorecfg_h::boolean,
        );
        /* If discarding APP0/APP14, use our regular on-the-fly processor. */
        if marker_code == M_APP0 as libc::c_int || marker_code == M_APP14 as libc::c_int {
            processor = Some(
                get_interesting_appn
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_decompress_ptr,
                    ) -> crate::jmorecfg_h::boolean,
            )
        }
    }
    if marker_code == M_COM as libc::c_int {
        (*marker).process_COM = processor;
        (*marker).length_limit_COM = length_limit
    } else if marker_code >= M_APP0 as libc::c_int && marker_code <= M_APP15 as libc::c_int {
        (*marker).process_APPn[(marker_code - M_APP0 as libc::c_int) as usize] = processor;
        (*marker).length_limit_APPn[(marker_code - M_APP0 as libc::c_int) as usize] = length_limit
    } else {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_UNKNOWN_MARKER as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = marker_code;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    };
}
/* Control saving of COM and APPn markers into marker_list. */
/* Install a special processing method for COM or APPn markers. */
/* SAVE_MARKERS_SUPPORTED */
/*
 * Install a special processing method for COM or APPn markers.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_set_marker_processor(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut marker_code: libc::c_int,
    mut routine: crate::jpeglib_h::jpeg_marker_parser_method,
) {
    let mut marker: my_marker_ptr = (*cinfo).marker as my_marker_ptr;
    if marker_code == M_COM as libc::c_int {
        (*marker).process_COM = routine
    } else if marker_code >= M_APP0 as libc::c_int && marker_code <= M_APP15 as libc::c_int {
        (*marker).process_APPn[(marker_code - M_APP0 as libc::c_int) as usize] = routine
    } else {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_UNKNOWN_MARKER as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = marker_code;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    };
}
