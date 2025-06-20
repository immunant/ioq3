pub use crate::stddef_h::size_t;

pub use crate::jdct_h::FLOAT_MULT_TYPE;
pub use crate::jdct_h::IFAST_MULT_TYPE;
pub use crate::jdct_h::ISLOW_MULT_TYPE;
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
pub use crate::src::jpeg_8c::jidctflt::jpeg_idct_float;
pub use crate::src::jpeg_8c::jidctfst::jpeg_idct_ifast;
pub use crate::src::jpeg_8c::jidctint::jpeg_idct_10x10;
pub use crate::src::jpeg_8c::jidctint::jpeg_idct_10x5;
pub use crate::src::jpeg_8c::jidctint::jpeg_idct_11x11;
pub use crate::src::jpeg_8c::jidctint::jpeg_idct_12x12;
pub use crate::src::jpeg_8c::jidctint::jpeg_idct_12x6;
pub use crate::src::jpeg_8c::jidctint::jpeg_idct_13x13;
pub use crate::src::jpeg_8c::jidctint::jpeg_idct_14x14;
pub use crate::src::jpeg_8c::jidctint::jpeg_idct_14x7;
pub use crate::src::jpeg_8c::jidctint::jpeg_idct_15x15;
pub use crate::src::jpeg_8c::jidctint::jpeg_idct_16x16;
pub use crate::src::jpeg_8c::jidctint::jpeg_idct_16x8;
pub use crate::src::jpeg_8c::jidctint::jpeg_idct_1x1;
pub use crate::src::jpeg_8c::jidctint::jpeg_idct_1x2;
pub use crate::src::jpeg_8c::jidctint::jpeg_idct_2x1;
pub use crate::src::jpeg_8c::jidctint::jpeg_idct_2x2;
pub use crate::src::jpeg_8c::jidctint::jpeg_idct_2x4;
pub use crate::src::jpeg_8c::jidctint::jpeg_idct_3x3;
pub use crate::src::jpeg_8c::jidctint::jpeg_idct_3x6;
pub use crate::src::jpeg_8c::jidctint::jpeg_idct_4x2;
pub use crate::src::jpeg_8c::jidctint::jpeg_idct_4x4;
pub use crate::src::jpeg_8c::jidctint::jpeg_idct_4x8;
pub use crate::src::jpeg_8c::jidctint::jpeg_idct_5x10;
pub use crate::src::jpeg_8c::jidctint::jpeg_idct_5x5;
pub use crate::src::jpeg_8c::jidctint::jpeg_idct_6x12;
pub use crate::src::jpeg_8c::jidctint::jpeg_idct_6x3;
pub use crate::src::jpeg_8c::jidctint::jpeg_idct_6x6;
pub use crate::src::jpeg_8c::jidctint::jpeg_idct_7x14;
pub use crate::src::jpeg_8c::jidctint::jpeg_idct_7x7;
pub use crate::src::jpeg_8c::jidctint::jpeg_idct_8x16;
pub use crate::src::jpeg_8c::jidctint::jpeg_idct_8x4;
pub use crate::src::jpeg_8c::jidctint::jpeg_idct_9x9;
pub use crate::src::jpeg_8c::jidctint::jpeg_idct_islow;

pub type my_idct_ptr = *mut my_idct_controller;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_idct_controller {
    pub pub_0: jpeg_inverse_dct,
    pub cur_method: [i32; 10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union multiplier_table {
    pub islow_array: [ISLOW_MULT_TYPE; 64],
    pub ifast_array: [IFAST_MULT_TYPE; 64],
    pub float_array: [FLOAT_MULT_TYPE; 64],
}
/* The current scaled-IDCT routines require ISLOW-style multiplier tables,
 * so be sure to compile that code if either ISLOW or SCALING is requested.
 */
/*
 * Prepare for an output pass.
 * Here we select the proper IDCT routine for each component and build
 * a matching multiplier table.
 */

unsafe extern "C" fn start_pass(mut cinfo: j_decompress_ptr) {
    let mut idct: my_idct_ptr = (*cinfo).idct as my_idct_ptr;
    let mut ci: i32 = 0;
    let mut i: i32 = 0;
    let mut compptr: *mut jpeg_component_info = std::ptr::null_mut();
    let mut method: i32 = 0 as i32;
    let mut method_ptr: inverse_DCT_method_ptr = None;
    let mut qtbl: *mut JQUANT_TBL = std::ptr::null_mut();
    ci = 0 as i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        /* Select the proper IDCT routine for this component's scaling */
        match ((*compptr).DCT_h_scaled_size << 8 as i32) + (*compptr).DCT_v_scaled_size {
            257 => {
                method_ptr = Some(
                    jpeg_idct_1x1
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            514 => {
                method_ptr = Some(
                    jpeg_idct_2x2
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            771 => {
                method_ptr = Some(
                    jpeg_idct_3x3
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            1028 => {
                method_ptr = Some(
                    jpeg_idct_4x4
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            1285 => {
                method_ptr = Some(
                    jpeg_idct_5x5
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            1542 => {
                method_ptr = Some(
                    jpeg_idct_6x6
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            1799 => {
                method_ptr = Some(
                    jpeg_idct_7x7
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            2313 => {
                method_ptr = Some(
                    jpeg_idct_9x9
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            2570 => {
                method_ptr = Some(
                    jpeg_idct_10x10
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            2827 => {
                method_ptr = Some(
                    jpeg_idct_11x11
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            3084 => {
                method_ptr = Some(
                    jpeg_idct_12x12
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            3341 => {
                method_ptr = Some(
                    jpeg_idct_13x13
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            3598 => {
                method_ptr = Some(
                    jpeg_idct_14x14
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            3855 => {
                method_ptr = Some(
                    jpeg_idct_15x15
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            4112 => {
                method_ptr = Some(
                    jpeg_idct_16x16
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            4104 => {
                method_ptr = Some(
                    jpeg_idct_16x8
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            3591 => {
                method_ptr = Some(
                    jpeg_idct_14x7
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            3078 => {
                method_ptr = Some(
                    jpeg_idct_12x6
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            2565 => {
                method_ptr = Some(
                    jpeg_idct_10x5
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            2052 => {
                method_ptr = Some(
                    jpeg_idct_8x4
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            1539 => {
                method_ptr = Some(
                    jpeg_idct_6x3
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            1026 => {
                method_ptr = Some(
                    jpeg_idct_4x2
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            513 => {
                method_ptr = Some(
                    jpeg_idct_2x1
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            2064 => {
                method_ptr = Some(
                    jpeg_idct_8x16
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            1806 => {
                method_ptr = Some(
                    jpeg_idct_7x14
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            1548 => {
                method_ptr = Some(
                    jpeg_idct_6x12
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            1290 => {
                method_ptr = Some(
                    jpeg_idct_5x10
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            1032 => {
                method_ptr = Some(
                    jpeg_idct_4x8
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            774 => {
                method_ptr = Some(
                    jpeg_idct_3x6
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            516 => {
                method_ptr = Some(
                    jpeg_idct_2x4
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            258 => {
                method_ptr = Some(
                    jpeg_idct_1x2
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: *mut jpeg_component_info,
                            _: JCOEFPTR,
                            _: JSAMPARRAY,
                            _: JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = JDCT_ISLOW as i32
            }
            2056 => match (*cinfo).dct_method as u32 {
                0 => {
                    method_ptr = Some(
                        jpeg_idct_islow
                            as unsafe extern "C" fn(
                                _: j_decompress_ptr,
                                _: *mut jpeg_component_info,
                                _: JCOEFPTR,
                                _: JSAMPARRAY,
                                _: JDIMENSION,
                            ) -> (),
                    );
                    method = JDCT_ISLOW as i32
                }
                1 => {
                    method_ptr = Some(
                        jpeg_idct_ifast
                            as unsafe extern "C" fn(
                                _: j_decompress_ptr,
                                _: *mut jpeg_component_info,
                                _: JCOEFPTR,
                                _: JSAMPARRAY,
                                _: JDIMENSION,
                            ) -> (),
                    );
                    method = JDCT_IFAST as i32
                }
                2 => {
                    method_ptr = Some(
                        jpeg_idct_float
                            as unsafe extern "C" fn(
                                _: j_decompress_ptr,
                                _: *mut jpeg_component_info,
                                _: JCOEFPTR,
                                _: JSAMPARRAY,
                                _: JDIMENSION,
                            ) -> (),
                    );
                    method = JDCT_FLOAT as i32
                }
                _ => {
                    (*(*cinfo).err).msg_code = JERR_NOT_COMPILED as i32;
                    Some(
                        (*(*cinfo).err)
                            .error_exit
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
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
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        (*idct).pub_0.inverse_DCT[ci as usize] = method_ptr;
        /* Create multiplier table from quant table.
         * However, we can skip this if the component is uninteresting
         * or if we already built the table.  Also, if no quant table
         * has yet been saved for the component, we leave the
         * multiplier table all-zero; we'll be reading zeroes from the
         * coefficient controller's buffer anyway.
         */
        if !((*compptr).component_needed == 0 || (*idct).cur_method[ci as usize] == method) {
            qtbl = (*compptr).quant_table;
            if !qtbl.is_null() {
                (*idct).cur_method[ci as usize] = method;
                match method {
                    0 => {
                        /* For LL&M IDCT method, multipliers are equal to raw quantization
                         * coefficients, but are stored as ints to ensure access efficiency.
                         */
                        let mut ismtbl: *mut ISLOW_MULT_TYPE =
                            (*compptr).dct_table as *mut ISLOW_MULT_TYPE;
                        i = 0 as i32;
                        while i < 64 as i32 {
                            *ismtbl.offset(i as isize) =
                                (*qtbl).quantval[i as usize] as ISLOW_MULT_TYPE;
                            i += 1
                        }
                    }
                    1 => {
                        /* For AA&N IDCT method, multipliers are equal to quantization
                         * coefficients scaled by scalefactor[row]*scalefactor[col], where
                         *   scalefactor[0] = 1
                         *   scalefactor[k] = cos(k*PI/16) * sqrt(2)    for k=1..7
                         * For integer operation, the multiplier table is to be scaled by
                         * IFAST_SCALE_BITS.
                         */
                        let mut ifmtbl: *mut IFAST_MULT_TYPE =
                            (*compptr).dct_table as *mut IFAST_MULT_TYPE;
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
                        i = 0 as i32;
                        while i < 64 as i32 {
                            *ifmtbl.offset(i as isize) = ((*qtbl).quantval[i as usize] as INT32
                                * aanscales[i as usize] as INT32
                                + ((1 as i32 as INT32) << 14 as i32 - 2 as i32 - 1 as i32)
                                >> 14 as i32 - 2 as i32)
                                as IFAST_MULT_TYPE;
                            i += 1
                        }
                    }
                    2 => {
                        /* For float AA&N IDCT method, multipliers are equal to quantization
                         * coefficients scaled by scalefactor[row]*scalefactor[col], where
                         *   scalefactor[0] = 1
                         *   scalefactor[k] = cos(k*PI/16) * sqrt(2)    for k=1..7
                         * We apply a further scale factor of 1/8.
                         */
                        let mut fmtbl: *mut FLOAT_MULT_TYPE =
                            (*compptr).dct_table as *mut FLOAT_MULT_TYPE;
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
                        i = 0 as i32;
                        row = 0 as i32;
                        while row < 8 as i32 {
                            col = 0 as i32;
                            while col < 8 as i32 {
                                *fmtbl.offset(i as isize) = ((*qtbl).quantval[i as usize] as f64
                                    * aanscalefactor[row as usize]
                                    * aanscalefactor[col as usize]
                                    * 0.125f64)
                                    as FLOAT_MULT_TYPE;
                                i += 1;
                                col += 1
                            }
                            row += 1
                        }
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
            }
        }
        /* happens if no data yet for component */
        ci += 1;
        compptr = compptr.offset(1)
    }
}
/*
 * Initialize IDCT manager.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_inverse_dct(mut cinfo: j_decompress_ptr) {
    let mut idct: my_idct_ptr = std::ptr::null_mut();
    let mut ci: i32 = 0;
    let mut compptr: *mut jpeg_component_info = std::ptr::null_mut();
    idct = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        1 as i32,
        ::std::mem::size_of::<my_idct_controller>() as usize,
    ) as my_idct_ptr;
    (*cinfo).idct = idct as *mut jpeg_inverse_dct;
    (*idct).pub_0.start_pass = Some(start_pass as unsafe extern "C" fn(_: j_decompress_ptr) -> ());
    ci = 0 as i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        /* Allocate and pre-zero a multiplier table for each component */
        (*compptr).dct_table = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            1 as i32,
            ::std::mem::size_of::<multiplier_table>() as usize,
        );
        crate::stdlib::memset(
            (*compptr).dct_table,
            0 as i32,
            ::std::mem::size_of::<multiplier_table>() as usize,
        );
        /* Mark multiplier table not yet set up for any method */
        (*idct).cur_method[ci as usize] = -(1 as i32);
        ci += 1;
        compptr = compptr.offset(1)
    }
}
