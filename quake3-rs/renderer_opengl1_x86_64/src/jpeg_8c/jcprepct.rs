use ::libc;

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
pub use crate::src::jpeg_8c::jutils::jcopy_sample_rows;

pub type my_prep_ptr = *mut my_prep_controller;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_prep_controller {
    pub pub_0: jpeg_c_prep_controller,
    pub color_buf: [JSAMPARRAY; 10],
    pub rows_to_go: JDIMENSION,
    pub next_buf_row: i32,
    pub this_row_group: i32,
    pub next_buf_stop: i32,
}
/*
 * Initialize for a processing pass.
 */

unsafe extern "C" fn start_pass_prep(mut cinfo: j_compress_ptr, mut pass_mode: J_BUF_MODE) {
    let mut prep: my_prep_ptr = (*cinfo).prep as my_prep_ptr;
    if pass_mode as u32 != JBUF_PASS_THRU as i32 as u32 {
        (*(*cinfo).err).msg_code = JERR_BAD_BUFFER_MODE as i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* Initialize total-height counter for detecting bottom of image */
    (*prep).rows_to_go = (*cinfo).image_height;
    /* Mark the conversion buffer empty */
    (*prep).next_buf_row = 0 as i32;
    /* Preset additional state variables for context mode.
     * These aren't used in non-context mode, so we needn't test which mode.
     */
    (*prep).this_row_group = 0 as i32;
    /* Set next_buf_stop to stop after two row groups have been read in. */
    (*prep).next_buf_stop = 2 as i32 * (*cinfo).max_v_samp_factor;
}
/*
 * Expand an image vertically from height input_rows to height output_rows,
 * by duplicating the bottom row.
 */

unsafe extern "C" fn expand_bottom_edge(
    mut image_data: JSAMPARRAY,
    mut num_cols: JDIMENSION,
    mut input_rows: i32,
    mut output_rows: i32,
) {
    let mut row: i32 = 0;
    row = input_rows;
    while row < output_rows {
        jcopy_sample_rows(
            image_data,
            input_rows - 1 as i32,
            image_data,
            row,
            1 as i32,
            num_cols,
        );
        row += 1
    }
}
/*
 * Process some data in the simple no-context case.
 *
 * Preprocessor output data is counted in "row groups".  A row group
 * is defined to be v_samp_factor sample rows of each component.
 * Downsampling will produce this much data from each max_v_samp_factor
 * input rows.
 */

unsafe extern "C" fn pre_process_data(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut in_row_ctr: *mut JDIMENSION,
    mut in_rows_avail: JDIMENSION,
    mut output_buf: JSAMPIMAGE,
    mut out_row_group_ctr: *mut JDIMENSION,
    mut out_row_groups_avail: JDIMENSION,
) {
    let mut prep: my_prep_ptr = (*cinfo).prep as my_prep_ptr;
    let mut numrows: i32 = 0;
    let mut ci: i32 = 0;
    let mut inrows: JDIMENSION = 0;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    while *in_row_ctr < in_rows_avail && *out_row_group_ctr < out_row_groups_avail {
        /* Do color conversion to fill the conversion buffer. */
        inrows = in_rows_avail.wrapping_sub(*in_row_ctr);
        numrows = (*cinfo).max_v_samp_factor - (*prep).next_buf_row;
        numrows = if (numrows as JDIMENSION) < inrows {
            numrows as JDIMENSION
        } else {
            inrows
        } as i32;
        Some(
            (*(*cinfo).cconvert)
                .color_convert
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo,
            input_buf.offset(*in_row_ctr as isize),
            (*prep).color_buf.as_mut_ptr(),
            (*prep).next_buf_row as JDIMENSION,
            numrows,
        );
        *in_row_ctr = (*in_row_ctr as u32).wrapping_add(numrows as u32) as JDIMENSION;
        (*prep).next_buf_row += numrows;
        (*prep).rows_to_go = ((*prep).rows_to_go as u32).wrapping_sub(numrows as u32) as JDIMENSION;
        /* If at bottom of image, pad to fill the conversion buffer. */
        if (*prep).rows_to_go == 0 as i32 as u32
            && (*prep).next_buf_row < (*cinfo).max_v_samp_factor
        {
            ci = 0 as i32;
            while ci < (*cinfo).num_components {
                expand_bottom_edge(
                    (*prep).color_buf[ci as usize],
                    (*cinfo).image_width,
                    (*prep).next_buf_row,
                    (*cinfo).max_v_samp_factor,
                );
                ci += 1
            }
            (*prep).next_buf_row = (*cinfo).max_v_samp_factor
        }
        /* If we've filled the conversion buffer, empty it. */
        if (*prep).next_buf_row == (*cinfo).max_v_samp_factor {
            Some(
                (*(*cinfo).downsample)
                    .downsample
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo,
                (*prep).color_buf.as_mut_ptr(),
                0 as i32 as JDIMENSION,
                output_buf,
                *out_row_group_ctr,
            );
            (*prep).next_buf_row = 0 as i32;
            *out_row_group_ctr = (*out_row_group_ctr).wrapping_add(1)
        }
        /* If at bottom of image, pad the output to a full iMCU height.
         * Note we assume the caller is providing a one-iMCU-height output buffer!
         */
        if !((*prep).rows_to_go == 0 as i32 as u32 && *out_row_group_ctr < out_row_groups_avail) {
            continue;
        }
        ci = 0 as i32;
        compptr = (*cinfo).comp_info;
        while ci < (*cinfo).num_components {
            numrows = (*compptr).v_samp_factor * (*compptr).DCT_v_scaled_size
                / (*cinfo).min_DCT_v_scaled_size;
            expand_bottom_edge(
                *output_buf.offset(ci as isize),
                (*compptr)
                    .width_in_blocks
                    .wrapping_mul((*compptr).DCT_h_scaled_size as u32),
                (*out_row_group_ctr).wrapping_mul(numrows as u32) as i32,
                out_row_groups_avail.wrapping_mul(numrows as u32) as i32,
            );
            ci += 1;
            compptr = compptr.offset(1)
        }
        *out_row_group_ctr = out_row_groups_avail;
        break;
        /* can exit outer loop without test */
    }
}
/*
 * Process some data in the context case.
 */

unsafe extern "C" fn pre_process_context(
    mut cinfo: j_compress_ptr,
    mut input_buf: JSAMPARRAY,
    mut in_row_ctr: *mut JDIMENSION,
    mut in_rows_avail: JDIMENSION,
    mut output_buf: JSAMPIMAGE,
    mut out_row_group_ctr: *mut JDIMENSION,
    mut out_row_groups_avail: JDIMENSION,
) {
    let mut prep: my_prep_ptr = (*cinfo).prep as my_prep_ptr;
    let mut numrows: i32 = 0;
    let mut ci: i32 = 0;
    let mut buf_height: i32 = (*cinfo).max_v_samp_factor * 3 as i32;
    let mut inrows: JDIMENSION = 0;
    while *out_row_group_ctr < out_row_groups_avail {
        if *in_row_ctr < in_rows_avail {
            /* Do color conversion to fill the conversion buffer. */
            inrows = in_rows_avail.wrapping_sub(*in_row_ctr);
            numrows = (*prep).next_buf_stop - (*prep).next_buf_row;
            numrows = if (numrows as JDIMENSION) < inrows {
                numrows as JDIMENSION
            } else {
                inrows
            } as i32;
            Some(
                (*(*cinfo).cconvert)
                    .color_convert
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo,
                input_buf.offset(*in_row_ctr as isize),
                (*prep).color_buf.as_mut_ptr(),
                (*prep).next_buf_row as JDIMENSION,
                numrows,
            );
            /* Pad at top of image, if first time through */
            if (*prep).rows_to_go == (*cinfo).image_height {
                ci = 0 as i32;
                while ci < (*cinfo).num_components {
                    let mut row: i32 = 0;
                    row = 1 as i32;
                    while row <= (*cinfo).max_v_samp_factor {
                        jcopy_sample_rows(
                            (*prep).color_buf[ci as usize],
                            0 as i32,
                            (*prep).color_buf[ci as usize],
                            -row,
                            1 as i32,
                            (*cinfo).image_width,
                        );
                        row += 1
                    }
                    ci += 1
                }
            }
            *in_row_ctr = (*in_row_ctr as u32).wrapping_add(numrows as u32) as JDIMENSION;
            (*prep).next_buf_row += numrows;
            (*prep).rows_to_go =
                ((*prep).rows_to_go as u32).wrapping_sub(numrows as u32) as JDIMENSION
        } else {
            /* Return for more data, unless we are at the bottom of the image. */
            if (*prep).rows_to_go != 0 as i32 as u32 {
                break;
            }
            /* When at bottom of image, pad to fill the conversion buffer. */
            if (*prep).next_buf_row < (*prep).next_buf_stop {
                ci = 0 as i32;
                while ci < (*cinfo).num_components {
                    expand_bottom_edge(
                        (*prep).color_buf[ci as usize],
                        (*cinfo).image_width,
                        (*prep).next_buf_row,
                        (*prep).next_buf_stop,
                    );
                    ci += 1
                }
                (*prep).next_buf_row = (*prep).next_buf_stop
            }
        }
        /* If we've gotten enough data, downsample a row group. */
        if (*prep).next_buf_row == (*prep).next_buf_stop {
            Some(
                (*(*cinfo).downsample)
                    .downsample
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo,
                (*prep).color_buf.as_mut_ptr(),
                (*prep).this_row_group as JDIMENSION,
                output_buf,
                *out_row_group_ctr,
            );
            *out_row_group_ctr = (*out_row_group_ctr).wrapping_add(1);
            /* Advance pointers with wraparound as necessary. */
            (*prep).this_row_group += (*cinfo).max_v_samp_factor;
            if (*prep).this_row_group >= buf_height {
                (*prep).this_row_group = 0 as i32
            }
            if (*prep).next_buf_row >= buf_height {
                (*prep).next_buf_row = 0 as i32
            }
            (*prep).next_buf_stop = (*prep).next_buf_row + (*cinfo).max_v_samp_factor
        }
    }
}
/*
 * Create the wrapped-around downsampling input buffer needed for context mode.
 */

unsafe extern "C" fn create_context_buffer(mut cinfo: j_compress_ptr) {
    let mut prep: my_prep_ptr = (*cinfo).prep as my_prep_ptr;
    let mut rgroup_height: i32 = (*cinfo).max_v_samp_factor;
    let mut ci: i32 = 0;
    let mut i: i32 = 0;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    let mut true_buffer: JSAMPARRAY = 0 as *mut JSAMPROW;
    let mut fake_buffer: JSAMPARRAY = 0 as *mut JSAMPROW;
    /* Grab enough space for fake row pointers for all the components;
     * we need five row groups' worth of pointers for each component.
     */
    fake_buffer = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        1 as i32,
        (((*cinfo).num_components * 5 as i32 * rgroup_height) as usize)
            .wrapping_mul(::std::mem::size_of::<JSAMPROW>() as usize),
    ) as JSAMPARRAY;
    ci = 0 as i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        /* Allocate the actual buffer space (3 row groups) for this component.
         * We make the buffer wide enough to allow the downsampler to edge-expand
         * horizontally within the buffer, if it so chooses.
         */
        true_buffer = Some(
            (*(*cinfo).mem)
                .alloc_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            1 as i32,
            ((*compptr).width_in_blocks as isize
                * (*cinfo).min_DCT_h_scaled_size as isize
                * (*cinfo).max_h_samp_factor as isize
                / (*compptr).h_samp_factor as isize) as JDIMENSION,
            (3 as i32 * rgroup_height) as JDIMENSION,
        );
        /* point to space for next component */
        crate::stdlib::memcpy(
            fake_buffer.offset(rgroup_height as isize) as *mut libc::c_void,
            true_buffer as *const libc::c_void,
            ((3 as i32 * rgroup_height) as usize)
                .wrapping_mul(::std::mem::size_of::<JSAMPROW>() as usize),
        );
        i = 0 as i32;
        while i < rgroup_height {
            let ref mut fresh0 = *fake_buffer.offset(i as isize);
            *fresh0 = *true_buffer.offset((2 as i32 * rgroup_height + i) as isize);
            let ref mut fresh1 = *fake_buffer.offset((4 as i32 * rgroup_height + i) as isize);
            *fresh1 = *true_buffer.offset(i as isize);
            i += 1
        }
        (*prep).color_buf[ci as usize] = fake_buffer.offset(rgroup_height as isize);
        fake_buffer = fake_buffer.offset((5 as i32 * rgroup_height) as isize);
        ci += 1;
        compptr = compptr.offset(1)
    }
}
/* Copy true buffer row pointers into the middle of the fake row array */
/* Fill in the above and below wraparound pointers */
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
/* CONTEXT_ROWS_SUPPORTED */
/*
 * Initialize preprocessing controller.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_c_prep_controller(
    mut cinfo: j_compress_ptr,
    mut need_full_buffer: boolean,
) {
    let mut prep: my_prep_ptr = 0 as *mut my_prep_controller;
    let mut ci: i32 = 0;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    if need_full_buffer != 0 {
        /* safety check */
        (*(*cinfo).err).msg_code = JERR_BAD_BUFFER_MODE as i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    prep = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        1 as i32,
        ::std::mem::size_of::<my_prep_controller>() as usize,
    ) as my_prep_ptr;
    (*cinfo).prep = prep as *mut jpeg_c_prep_controller;
    (*prep).pub_0.start_pass =
        Some(start_pass_prep as unsafe extern "C" fn(_: j_compress_ptr, _: J_BUF_MODE) -> ());
    /* Allocate the color conversion buffer.
     * We make the buffer wide enough to allow the downsampler to edge-expand
     * horizontally within the buffer, if it so chooses.
     */
    if (*(*cinfo).downsample).need_context_rows != 0 {
        /* Set up to provide context rows */
        (*prep).pub_0.pre_process_data = Some(
            pre_process_context
                as unsafe extern "C" fn(
                    _: j_compress_ptr,
                    _: JSAMPARRAY,
                    _: *mut JDIMENSION,
                    _: JDIMENSION,
                    _: JSAMPIMAGE,
                    _: *mut JDIMENSION,
                    _: JDIMENSION,
                ) -> (),
        );
        create_context_buffer(cinfo);
    } else {
        /* No context, just make it tall enough for one row group */
        (*prep).pub_0.pre_process_data = Some(
            pre_process_data
                as unsafe extern "C" fn(
                    _: j_compress_ptr,
                    _: JSAMPARRAY,
                    _: *mut JDIMENSION,
                    _: JDIMENSION,
                    _: JSAMPIMAGE,
                    _: *mut JDIMENSION,
                    _: JDIMENSION,
                ) -> (),
        );
        ci = 0 as i32;
        compptr = (*cinfo).comp_info;
        while ci < (*cinfo).num_components {
            (*prep).color_buf[ci as usize] = Some(
                (*(*cinfo).mem)
                    .alloc_sarray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr,
                1 as i32,
                ((*compptr).width_in_blocks as isize
                    * (*cinfo).min_DCT_h_scaled_size as isize
                    * (*cinfo).max_h_samp_factor as isize
                    / (*compptr).h_samp_factor as isize) as JDIMENSION,
                (*cinfo).max_v_samp_factor as JDIMENSION,
            );
            ci += 1;
            compptr = compptr.offset(1)
        }
    };
}
