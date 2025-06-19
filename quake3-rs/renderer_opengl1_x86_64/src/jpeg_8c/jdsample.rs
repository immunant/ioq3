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
pub use crate::src::jpeg_8c::jutils::jcopy_sample_rows;
pub use crate::src::jpeg_8c::jutils::jround_up;

pub type my_upsample_ptr = *mut my_upsampler;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_upsampler {
    pub pub_0: jpeg_upsampler,
    pub color_buf: [JSAMPARRAY; 10],
    pub methods: [upsample1_ptr; 10],
    pub next_row_out: i32,
    pub rows_to_go: JDIMENSION,
    pub rowgroup_height: [i32; 10],
    pub h_expand: [UINT8; 10],
    pub v_expand: [UINT8; 10],
}
/*
 * jdsample.c
 *
 * Copyright (C) 1991-1996, Thomas G. Lane.
 * Modified 2002-2008 by Guido Vollbeding.
 * This file is part of the Independent JPEG Group's software.
 * For conditions of distribution and use, see the accompanying README file.
 *
 * This file contains upsampling routines.
 *
 * Upsampling input data is counted in "row groups".  A row group
 * is defined to be (v_samp_factor * DCT_v_scaled_size / min_DCT_v_scaled_size)
 * sample rows of each component.  Upsampling will normally produce
 * max_v_samp_factor pixel rows from each row group (but this could vary
 * if the upsampler is applying a scale factor of its own).
 *
 * An excellent reference for image resampling is
 *   Digital Image Warping, George Wolberg, 1990.
 *   Pub. by IEEE Computer Society Press, Los Alamitos, CA. ISBN 0-8186-8944-7.
 */
/* Pointer to routine to upsample a single component */

pub type upsample1_ptr = Option<
    unsafe extern "C" fn(
        _: j_decompress_ptr,
        _: *mut jpeg_component_info,
        _: JSAMPARRAY,
        _: *mut JSAMPARRAY,
    ) -> (),
>;
/*
 * Initialize for an upsampling pass.
 */

unsafe extern "C" fn start_pass_upsample(mut cinfo: j_decompress_ptr) {
    let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    /* Mark the conversion buffer empty */
    (*upsample).next_row_out = (*cinfo).max_v_samp_factor;
    /* Initialize total-height counter for detecting bottom of image */
    (*upsample).rows_to_go = (*cinfo).output_height;
}
/*
 * Control routine to do upsampling (and color conversion).
 *
 * In this version we upsample each component independently.
 * We upsample one row group into the conversion buffer, then apply
 * color conversion a row at a time.
 */

unsafe extern "C" fn sep_upsample(
    mut cinfo: j_decompress_ptr,
    mut input_buf: JSAMPIMAGE,
    mut in_row_group_ctr: *mut JDIMENSION,
    mut _in_row_groups_avail: JDIMENSION,
    mut output_buf: JSAMPARRAY,
    mut out_row_ctr: *mut JDIMENSION,
    mut out_rows_avail: JDIMENSION,
) {
    let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    let mut ci: i32 = 0;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    let mut num_rows: JDIMENSION = 0;
    /* Fill the conversion buffer, if it's empty */
    if (*upsample).next_row_out >= (*cinfo).max_v_samp_factor {
        ci = 0 as i32;
        compptr = (*cinfo).comp_info;
        while ci < (*cinfo).num_components {
            /* Invoke per-component upsample method.  Notice we pass a POINTER
             * to color_buf[ci], so that fullsize_upsample can change it.
             */
            Some(
                (*(*upsample).methods.as_mut_ptr().offset(ci as isize))
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo,
                compptr,
                (*input_buf.offset(ci as isize)).offset(
                    (*in_row_group_ctr)
                        .wrapping_mul((*upsample).rowgroup_height[ci as usize] as u32)
                        as isize,
                ),
                (*upsample).color_buf.as_mut_ptr().offset(ci as isize),
            );
            ci += 1;
            compptr = compptr.offset(1)
        }
        (*upsample).next_row_out = 0 as i32
    }
    /* Color-convert and emit rows */
    /* How many we have in the buffer: */
    num_rows = ((*cinfo).max_v_samp_factor - (*upsample).next_row_out) as JDIMENSION;
    /* Not more than the distance to the end of the image.  Need this test
     * in case the image height is not a multiple of max_v_samp_factor:
     */
    if num_rows > (*upsample).rows_to_go {
        num_rows = (*upsample).rows_to_go
    }
    /* And not more than what the client can accept: */
    out_rows_avail = (out_rows_avail as u32).wrapping_sub(*out_row_ctr) as JDIMENSION;
    if num_rows > out_rows_avail {
        num_rows = out_rows_avail
    }
    Some(
        (*(*cinfo).cconvert)
            .color_convert
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo,
        (*upsample).color_buf.as_mut_ptr(),
        (*upsample).next_row_out as JDIMENSION,
        output_buf.offset(*out_row_ctr as isize),
        num_rows as i32,
    );
    /* Adjust counts */
    *out_row_ctr = (*out_row_ctr as u32).wrapping_add(num_rows) as JDIMENSION;
    (*upsample).rows_to_go = ((*upsample).rows_to_go as u32).wrapping_sub(num_rows) as JDIMENSION;
    (*upsample).next_row_out = ((*upsample).next_row_out as u32).wrapping_add(num_rows) as i32;
    /* When the buffer is emptied, declare this input row group consumed */
    if (*upsample).next_row_out >= (*cinfo).max_v_samp_factor {
        *in_row_group_ctr = (*in_row_group_ctr).wrapping_add(1)
    };
}
/*
 * These are the routines invoked by sep_upsample to upsample pixel values
 * of a single component.  One row group is processed per call.
 */
/*
 * For full-size components, we just make color_buf[ci] point at the
 * input buffer, and thus avoid copying any data.  Note that this is
 * safe only because sep_upsample doesn't declare the input row group
 * "consumed" until we are done color converting and emitting it.
 */

unsafe extern "C" fn fullsize_upsample(
    mut _cinfo: j_decompress_ptr,
    mut _compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data_ptr: *mut JSAMPARRAY,
) {
    *output_data_ptr = input_data;
}
/*
 * This is a no-op version used for "uninteresting" components.
 * These components will not be referenced by color conversion.
 */

unsafe extern "C" fn noop_upsample(
    mut _cinfo: j_decompress_ptr,
    mut _compptr: *mut jpeg_component_info,
    mut _input_data: JSAMPARRAY,
    mut output_data_ptr: *mut JSAMPARRAY,
) {
    *output_data_ptr = 0 as JSAMPARRAY;
    /* safety check */
}
/*
 * This version handles any integral sampling ratios.
 * This is not used for typical JPEG files, so it need not be fast.
 * Nor, for that matter, is it particularly accurate: the algorithm is
 * simple replication of the input pixel onto the corresponding output
 * pixels.  The hi-falutin sampling literature refers to this as a
 * "box filter".  A box filter tends to introduce visible artifacts,
 * so if you are actually going to use 3:1 or 4:1 sampling ratios
 * you would be well advised to improve this code.
 */

unsafe extern "C" fn int_upsample(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data_ptr: *mut JSAMPARRAY,
) {
    let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    let mut output_data: JSAMPARRAY = *output_data_ptr;
    let mut inptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut invalue: JSAMPLE = 0;
    let mut h: i32 = 0;
    let mut outend: JSAMPROW = 0 as *mut JSAMPLE;
    let mut h_expand: i32 = 0;
    let mut v_expand: i32 = 0;
    let mut inrow: i32 = 0;
    let mut outrow: i32 = 0;
    h_expand = (*upsample).h_expand[(*compptr).component_index as usize] as i32;
    v_expand = (*upsample).v_expand[(*compptr).component_index as usize] as i32;
    outrow = 0 as i32;
    inrow = outrow;
    while outrow < (*cinfo).max_v_samp_factor {
        /* Generate one output row with proper horizontal expansion */
        inptr = *input_data.offset(inrow as isize); /* don't need GETJSAMPLE() here */
        outptr = *output_data.offset(outrow as isize);
        outend = outptr.offset((*cinfo).output_width as isize);
        while outptr < outend {
            let fresh0 = inptr;
            inptr = inptr.offset(1);
            invalue = *fresh0;
            h = h_expand;
            while h > 0 as i32 {
                let fresh1 = outptr;
                outptr = outptr.offset(1);
                *fresh1 = invalue;
                h -= 1
            }
        }
        /* Generate any additional output rows by duplicating the first one */
        if v_expand > 1 as i32 {
            jcopy_sample_rows(
                output_data,
                outrow,
                output_data,
                outrow + 1 as i32,
                v_expand - 1 as i32,
                (*cinfo).output_width,
            );
        }
        inrow += 1;
        outrow += v_expand
    }
}
/*
 * Fast processing for the common case of 2:1 horizontal and 1:1 vertical.
 * It's still a box filter.
 */

unsafe extern "C" fn h2v1_upsample(
    mut cinfo: j_decompress_ptr,
    mut _compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data_ptr: *mut JSAMPARRAY,
) {
    let mut output_data: JSAMPARRAY = *output_data_ptr; /* don't need GETJSAMPLE() here */
    let mut inptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut invalue: JSAMPLE = 0;
    let mut outend: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outrow: i32 = 0;
    outrow = 0 as i32;
    while outrow < (*cinfo).max_v_samp_factor {
        inptr = *input_data.offset(outrow as isize);
        outptr = *output_data.offset(outrow as isize);
        outend = outptr.offset((*cinfo).output_width as isize);
        while outptr < outend {
            let fresh2 = inptr;
            inptr = inptr.offset(1);
            invalue = *fresh2;
            let fresh3 = outptr;
            outptr = outptr.offset(1);
            *fresh3 = invalue;
            let fresh4 = outptr;
            outptr = outptr.offset(1);
            *fresh4 = invalue
        }
        outrow += 1
    }
}
/*
 * Fast processing for the common case of 2:1 horizontal and 2:1 vertical.
 * It's still a box filter.
 */

unsafe extern "C" fn h2v2_upsample(
    mut cinfo: j_decompress_ptr,
    mut _compptr: *mut jpeg_component_info,
    mut input_data: JSAMPARRAY,
    mut output_data_ptr: *mut JSAMPARRAY,
) {
    let mut output_data: JSAMPARRAY = *output_data_ptr; /* don't need GETJSAMPLE() here */
    let mut inptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut invalue: JSAMPLE = 0;
    let mut outend: JSAMPROW = 0 as *mut JSAMPLE;
    let mut inrow: i32 = 0;
    let mut outrow: i32 = 0;
    outrow = 0 as i32;
    inrow = outrow;
    while outrow < (*cinfo).max_v_samp_factor {
        inptr = *input_data.offset(inrow as isize);
        outptr = *output_data.offset(outrow as isize);
        outend = outptr.offset((*cinfo).output_width as isize);
        while outptr < outend {
            let fresh5 = inptr;
            inptr = inptr.offset(1);
            invalue = *fresh5;
            let fresh6 = outptr;
            outptr = outptr.offset(1);
            *fresh6 = invalue;
            let fresh7 = outptr;
            outptr = outptr.offset(1);
            *fresh7 = invalue
        }
        jcopy_sample_rows(
            output_data,
            outrow,
            output_data,
            outrow + 1 as i32,
            1 as i32,
            (*cinfo).output_width,
        );
        inrow += 1;
        outrow += 2 as i32
    }
}
/*
 * Module initialization routine for upsampling.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_upsampler(mut cinfo: j_decompress_ptr) {
    let mut upsample: my_upsample_ptr = 0 as *mut my_upsampler; /* until we find out differently */
    let mut ci: i32 = 0;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    let mut need_buffer: boolean = 0;
    let mut h_in_group: i32 = 0;
    let mut v_in_group: i32 = 0;
    let mut h_out_group: i32 = 0;
    let mut v_out_group: i32 = 0;
    upsample = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        1 as i32,
        ::std::mem::size_of::<my_upsampler>() as libc::c_ulong,
    ) as my_upsample_ptr;
    (*cinfo).upsample = upsample as *mut jpeg_upsampler;
    (*upsample).pub_0.start_pass =
        Some(start_pass_upsample as unsafe extern "C" fn(_: j_decompress_ptr) -> ());
    (*upsample).pub_0.upsample = Some(
        sep_upsample
            as unsafe extern "C" fn(
                _: j_decompress_ptr,
                _: JSAMPIMAGE,
                _: *mut JDIMENSION,
                _: JDIMENSION,
                _: JSAMPARRAY,
                _: *mut JDIMENSION,
                _: JDIMENSION,
            ) -> (),
    );
    (*upsample).pub_0.need_context_rows = 0 as i32;
    if (*cinfo).CCIR601_sampling != 0 {
        /* this isn't supported */
        (*(*cinfo).err).msg_code = JERR_CCIR601_NOTIMPL as i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* Verify we can handle the sampling factors, select per-component methods,
     * and create storage as needed.
     */
    ci = 0 as i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        /* Compute size of an "input group" after IDCT scaling.  This many samples
         * are to be converted to max_h_samp_factor * max_v_samp_factor pixels.
         */
        h_in_group = (*compptr).h_samp_factor * (*compptr).DCT_h_scaled_size
            / (*cinfo).min_DCT_h_scaled_size; /* save for use later */
        v_in_group = (*compptr).v_samp_factor * (*compptr).DCT_v_scaled_size
            / (*cinfo).min_DCT_v_scaled_size;
        h_out_group = (*cinfo).max_h_samp_factor;
        v_out_group = (*cinfo).max_v_samp_factor;
        (*upsample).rowgroup_height[ci as usize] = v_in_group;
        need_buffer = 1 as i32;
        if (*compptr).component_needed == 0 {
            /* Don't bother to upsample an uninteresting component. */
            (*upsample).methods[ci as usize] = Some(
                noop_upsample
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: *mut jpeg_component_info,
                        _: JSAMPARRAY,
                        _: *mut JSAMPARRAY,
                    ) -> (),
            );
            need_buffer = 0 as i32
        } else if h_in_group == h_out_group && v_in_group == v_out_group {
            /* Fullsize components can be processed without any work. */
            (*upsample).methods[ci as usize] = Some(
                fullsize_upsample
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: *mut jpeg_component_info,
                        _: JSAMPARRAY,
                        _: *mut JSAMPARRAY,
                    ) -> (),
            );
            need_buffer = 0 as i32
        } else if h_in_group * 2 as i32 == h_out_group && v_in_group == v_out_group {
            /* Special case for 2h1v upsampling */
            (*upsample).methods[ci as usize] = Some(
                h2v1_upsample
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: *mut jpeg_component_info,
                        _: JSAMPARRAY,
                        _: *mut JSAMPARRAY,
                    ) -> (),
            )
        } else if h_in_group * 2 as i32 == h_out_group && v_in_group * 2 as i32 == v_out_group {
            /* Special case for 2h2v upsampling */
            (*upsample).methods[ci as usize] = Some(
                h2v2_upsample
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: *mut jpeg_component_info,
                        _: JSAMPARRAY,
                        _: *mut JSAMPARRAY,
                    ) -> (),
            )
        } else if h_out_group % h_in_group == 0 as i32 && v_out_group % v_in_group == 0 as i32 {
            /* Generic integral-factors upsampling method */
            (*upsample).methods[ci as usize] = Some(
                int_upsample
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: *mut jpeg_component_info,
                        _: JSAMPARRAY,
                        _: *mut JSAMPARRAY,
                    ) -> (),
            );
            (*upsample).h_expand[ci as usize] = (h_out_group / h_in_group) as UINT8;
            (*upsample).v_expand[ci as usize] = (v_out_group / v_in_group) as UINT8
        } else {
            (*(*cinfo).err).msg_code = JERR_FRACT_SAMPLE_NOTIMPL as i32;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        if need_buffer != 0 {
            (*upsample).color_buf[ci as usize] = Some(
                (*(*cinfo).mem)
                    .alloc_sarray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr,
                1 as i32,
                jround_up(
                    (*cinfo).output_width as isize,
                    (*cinfo).max_h_samp_factor as isize,
                ) as JDIMENSION,
                (*cinfo).max_v_samp_factor as JDIMENSION,
            )
        }
        ci += 1;
        compptr = compptr.offset(1)
    }
}
