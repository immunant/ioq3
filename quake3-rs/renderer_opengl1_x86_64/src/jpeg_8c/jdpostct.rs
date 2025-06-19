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
pub use crate::src::jpeg_8c::jutils::jround_up;

pub type my_post_ptr = *mut my_post_controller;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_post_controller {
    pub pub_0: crate::jpegint_h::jpeg_d_post_controller,
    pub whole_image: crate::jpeglib_h::jvirt_sarray_ptr,
    pub buffer: crate::jpeglib_h::JSAMPARRAY,
    pub strip_height: crate::jmorecfg_h::JDIMENSION,
    pub starting_row: crate::jmorecfg_h::JDIMENSION,
    pub next_row: crate::jmorecfg_h::JDIMENSION,
}
/*
 * Initialize for a processing pass.
 */

unsafe extern "C" fn start_pass_dpost(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut pass_mode: crate::jpegint_h::J_BUF_MODE,
) {
    let mut post: my_post_ptr = (*cinfo).post as my_post_ptr;
    match pass_mode as u32 {
        0 => {
            if (*cinfo).quantize_colors != 0 {
                /* Single-pass processing with color quantization. */
                (*post).pub_0.post_process_data = Some(
                    post_process_1pass
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: crate::jpeglib_h::JSAMPIMAGE,
                            _: *mut crate::jmorecfg_h::JDIMENSION,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: *mut crate::jmorecfg_h::JDIMENSION,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                );
                /* We could be doing buffered-image output before starting a 2-pass
                 * color quantization; in that case, jinit_d_post_controller did not
                 * allocate a strip buffer.  Use the virtual-array buffer as workspace.
                 */
                if (*post).buffer.is_null() {
                    (*post).buffer = Some(
                        (*(*cinfo).mem)
                            .access_virt_sarray
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as crate::jpeglib_h::j_common_ptr,
                        (*post).whole_image,
                        0 as i32 as crate::jmorecfg_h::JDIMENSION,
                        (*post).strip_height,
                        1 as i32,
                    )
                }
            } else {
                /* For single-pass processing without color quantization,
                 * I have no work to do; just call the upsampler directly.
                 */
                (*post).pub_0.post_process_data = (*(*cinfo).upsample).upsample
            }
        }
        3 => {
            /* First pass of 2-pass quantization */
            if (*post).whole_image.is_null() {
                (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_BUFFER_MODE as i32;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            (*post).pub_0.post_process_data = Some(
                post_process_prepass
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_decompress_ptr,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: *mut crate::jmorecfg_h::JDIMENSION,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: *mut crate::jmorecfg_h::JDIMENSION,
                        _: crate::jmorecfg_h::JDIMENSION,
                    ) -> (),
            )
        }
        2 => {
            /* Second pass of 2-pass quantization */
            if (*post).whole_image.is_null() {
                (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_BUFFER_MODE as i32;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            (*post).pub_0.post_process_data = Some(
                post_process_2pass
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_decompress_ptr,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: *mut crate::jmorecfg_h::JDIMENSION,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: *mut crate::jmorecfg_h::JDIMENSION,
                        _: crate::jmorecfg_h::JDIMENSION,
                    ) -> (),
            )
        }
        _ => {
            /* QUANT_2PASS_SUPPORTED */
            (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_BUFFER_MODE as i32;
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
    (*post).next_row = 0 as i32 as crate::jmorecfg_h::JDIMENSION;
    (*post).starting_row = (*post).next_row;
}
/* Forward declarations */
/*
 * Process some data in the one-pass (strip buffer) case.
 * This is used for color precision reduction as well as one-pass quantization.
 */

unsafe extern "C" fn post_process_1pass(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut in_row_group_ctr: *mut crate::jmorecfg_h::JDIMENSION,
    mut in_row_groups_avail: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut out_row_ctr: *mut crate::jmorecfg_h::JDIMENSION,
    mut out_rows_avail: crate::jmorecfg_h::JDIMENSION,
) {
    let mut post: my_post_ptr = (*cinfo).post as my_post_ptr;
    let mut num_rows: crate::jmorecfg_h::JDIMENSION = 0;
    let mut max_rows: crate::jmorecfg_h::JDIMENSION = 0;
    /* Fill the buffer, but not more than what we can dump out in one go. */
    /* Note we rely on the upsampler to detect bottom of image. */
    max_rows = out_rows_avail.wrapping_sub(*out_row_ctr);
    if max_rows > (*post).strip_height {
        max_rows = (*post).strip_height
    }
    num_rows = 0 as i32 as crate::jmorecfg_h::JDIMENSION;
    Some(
        (*(*cinfo).upsample)
            .upsample
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo,
        input_buf,
        in_row_group_ctr,
        in_row_groups_avail,
        (*post).buffer,
        &mut num_rows,
        max_rows,
    );
    /* Quantize and emit data. */
    Some(
        (*(*cinfo).cquantize)
            .color_quantize
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo,
        (*post).buffer,
        output_buf.offset(*out_row_ctr as isize),
        num_rows as i32,
    );
    *out_row_ctr = (*out_row_ctr as u32).wrapping_add(num_rows) as crate::jmorecfg_h::JDIMENSION;
}
/*
 * Process some data in the first pass of 2-pass quantization.
 */

unsafe extern "C" fn post_process_prepass(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut in_row_group_ctr: *mut crate::jmorecfg_h::JDIMENSION,
    mut in_row_groups_avail: crate::jmorecfg_h::JDIMENSION,
    mut _output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut out_row_ctr: *mut crate::jmorecfg_h::JDIMENSION,
    mut _out_rows_avail: crate::jmorecfg_h::JDIMENSION,
) {
    let mut post: my_post_ptr = (*cinfo).post as my_post_ptr;
    let mut old_next_row: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_rows: crate::jmorecfg_h::JDIMENSION = 0;
    /* Reposition virtual buffer if at start of strip. */
    if (*post).next_row == 0 as i32 as u32 {
        (*post).buffer = Some(
            (*(*cinfo).mem)
                .access_virt_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            (*post).whole_image,
            (*post).starting_row,
            (*post).strip_height,
            1 as i32,
        )
    }
    /* Upsample some data (up to a strip height's worth). */
    old_next_row = (*post).next_row;
    Some(
        (*(*cinfo).upsample)
            .upsample
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo,
        input_buf,
        in_row_group_ctr,
        in_row_groups_avail,
        (*post).buffer,
        &mut (*post).next_row,
        (*post).strip_height,
    );
    /* Allow quantizer to scan new data.  No data is emitted, */
    /* but we advance out_row_ctr so outer loop can tell when we're done. */
    if (*post).next_row > old_next_row {
        num_rows = (*post).next_row.wrapping_sub(old_next_row);
        Some(
            (*(*cinfo).cquantize)
                .color_quantize
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo,
            (*post).buffer.offset(old_next_row as isize),
            0 as *mut libc::c_void as crate::jpeglib_h::JSAMPARRAY,
            num_rows as i32,
        );
        *out_row_ctr = (*out_row_ctr as u32).wrapping_add(num_rows) as crate::jmorecfg_h::JDIMENSION
    }
    /* Advance if we filled the strip. */
    if (*post).next_row >= (*post).strip_height {
        (*post).starting_row = ((*post).starting_row as u32).wrapping_add((*post).strip_height)
            as crate::jmorecfg_h::JDIMENSION;
        (*post).next_row = 0 as i32 as crate::jmorecfg_h::JDIMENSION
    };
}
/*
 * Process some data in the second pass of 2-pass quantization.
 */

unsafe extern "C" fn post_process_2pass(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut _input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut _in_row_group_ctr: *mut crate::jmorecfg_h::JDIMENSION,
    mut _in_row_groups_avail: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut out_row_ctr: *mut crate::jmorecfg_h::JDIMENSION,
    mut out_rows_avail: crate::jmorecfg_h::JDIMENSION,
) {
    let mut post: my_post_ptr = (*cinfo).post as my_post_ptr;
    let mut num_rows: crate::jmorecfg_h::JDIMENSION = 0;
    let mut max_rows: crate::jmorecfg_h::JDIMENSION = 0;
    /* Reposition virtual buffer if at start of strip. */
    if (*post).next_row == 0 as i32 as u32 {
        (*post).buffer = Some(
            (*(*cinfo).mem)
                .access_virt_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            (*post).whole_image,
            (*post).starting_row,
            (*post).strip_height,
            0 as i32,
        )
    }
    /* Determine number of rows to emit. */
    num_rows = (*post).strip_height.wrapping_sub((*post).next_row); /* available in strip */
    max_rows = out_rows_avail.wrapping_sub(*out_row_ctr); /* available in output area */
    if num_rows > max_rows {
        num_rows = max_rows
    }
    /* We have to check bottom of image here, can't depend on upsampler. */
    max_rows = (*cinfo).output_height.wrapping_sub((*post).starting_row);
    if num_rows > max_rows {
        num_rows = max_rows
    }
    /* Quantize and emit data. */
    Some(
        (*(*cinfo).cquantize)
            .color_quantize
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo,
        (*post).buffer.offset((*post).next_row as isize),
        output_buf.offset(*out_row_ctr as isize),
        num_rows as i32,
    );
    *out_row_ctr = (*out_row_ctr as u32).wrapping_add(num_rows) as crate::jmorecfg_h::JDIMENSION;
    /* Advance if we filled the strip. */
    (*post).next_row =
        ((*post).next_row as u32).wrapping_add(num_rows) as crate::jmorecfg_h::JDIMENSION;
    if (*post).next_row >= (*post).strip_height {
        (*post).starting_row = ((*post).starting_row as u32).wrapping_add((*post).strip_height)
            as crate::jmorecfg_h::JDIMENSION;
        (*post).next_row = 0 as i32 as crate::jmorecfg_h::JDIMENSION
    };
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
/* QUANT_2PASS_SUPPORTED */
/*
 * Initialize postprocessing controller.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_d_post_controller(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut need_full_buffer: crate::jmorecfg_h::boolean,
) {
    let mut post: my_post_ptr = 0 as *mut my_post_controller; /* flag for no virtual arrays */
    post = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        1 as i32,
        ::std::mem::size_of::<my_post_controller>() as libc::c_ulong,
    ) as my_post_ptr; /* flag for no strip buffer */
    (*cinfo).post = post as *mut crate::jpegint_h::jpeg_d_post_controller;
    (*post).pub_0.start_pass = Some(
        start_pass_dpost
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_decompress_ptr,
                _: crate::jpegint_h::J_BUF_MODE,
            ) -> (),
    );
    (*post).whole_image = 0 as crate::jpeglib_h::jvirt_sarray_ptr;
    (*post).buffer = 0 as crate::jpeglib_h::JSAMPARRAY;
    /* Create the quantization buffer, if needed */
    if (*cinfo).quantize_colors != 0 {
        /* The buffer strip height is max_v_samp_factor, which is typically
         * an efficient number of rows for upsampling to return.
         * (In the presence of output rescaling, we might want to be smarter?)
         */
        (*post).strip_height = (*cinfo).max_v_samp_factor as crate::jmorecfg_h::JDIMENSION;
        if need_full_buffer != 0 {
            /* Two-pass color quantization: need full-image storage. */
            /* We round up the number of rows to a multiple of the strip height. */
            (*post).whole_image = Some(
                (*(*cinfo).mem)
                    .request_virt_sarray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr,
                1 as i32,
                0 as i32,
                (*cinfo)
                    .output_width
                    .wrapping_mul((*cinfo).out_color_components as u32),
                crate::src::jpeg_8c::jutils::jround_up(
                    (*cinfo).output_height as libc::c_long,
                    (*post).strip_height as libc::c_long,
                ) as crate::jmorecfg_h::JDIMENSION,
                (*post).strip_height,
            )
        /* QUANT_2PASS_SUPPORTED */
        } else {
            /* One-pass color quantization: just make a strip buffer. */
            (*post).buffer = Some(
                (*(*cinfo).mem)
                    .alloc_sarray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr,
                1 as i32,
                (*cinfo)
                    .output_width
                    .wrapping_mul((*cinfo).out_color_components as u32),
                (*post).strip_height,
            )
        }
    };
}
