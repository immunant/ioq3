use ::libc;

/* INCOMPLETE_TYPES_BROKEN */

/* Suppress undefined-structure complaints if necessary. */
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
pub use crate::src::jpeg_8c::jutils::jcopy_block_row;
pub use crate::src::jpeg_8c::jutils::jround_up;
pub use crate::src::jpeg_8c::jutils::jzero_far;

pub type my_coef_ptr = *mut my_coef_controller;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_coef_controller {
    pub pub_0: jpeg_d_coef_controller,
    pub MCU_ctr: JDIMENSION,
    pub MCU_vert_offset: i32,
    pub MCU_rows_per_iMCU_row: i32,
    pub MCU_buffer: [JBLOCKROW; 10],
    pub whole_image: [jvirt_barray_ptr; 10],
    pub coef_bits_latch: *mut i32,
}

unsafe extern "C" fn start_iMCU_row(mut cinfo: j_decompress_ptr)
/* Reset within-iMCU-row counters for a new row (input side) */
{
    let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr;
    /* In an interleaved scan, an MCU row is the same as an iMCU row.
     * In a noninterleaved scan, an iMCU row has v_samp_factor MCU rows.
     * But at the bottom of the image, process only what's left.
     */
    if (*cinfo).comps_in_scan > 1 as i32 {
        (*coef).MCU_rows_per_iMCU_row = 1 as i32
    } else if (*cinfo).input_iMCU_row < (*cinfo).total_iMCU_rows.wrapping_sub(1 as i32 as u32) {
        (*coef).MCU_rows_per_iMCU_row = (*(*cinfo).cur_comp_info[0 as i32 as usize]).v_samp_factor
    } else {
        (*coef).MCU_rows_per_iMCU_row = (*(*cinfo).cur_comp_info[0 as i32 as usize]).last_row_height
    }
    (*coef).MCU_ctr = 0 as i32 as JDIMENSION;
    (*coef).MCU_vert_offset = 0 as i32;
}
/*
 * Initialize for an input processing pass.
 */

unsafe extern "C" fn start_input_pass(mut cinfo: j_decompress_ptr) {
    (*cinfo).input_iMCU_row = 0 as i32 as JDIMENSION;
    start_iMCU_row(cinfo);
}
/*
 * Initialize for an output processing pass.
 */

unsafe extern "C" fn start_output_pass(mut cinfo: j_decompress_ptr) {
    let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr;
    /* If multipass, check to see whether to use block smoothing on this pass */
    if !(*coef).pub_0.coef_arrays.is_null() {
        if (*cinfo).do_block_smoothing != 0 && smoothing_ok(cinfo) != 0 {
            (*coef).pub_0.decompress_data = Some(
                decompress_smooth_data
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: JSAMPIMAGE,
                    ) -> i32,
            )
        } else {
            (*coef).pub_0.decompress_data = Some(
                decompress_data
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: JSAMPIMAGE,
                    ) -> i32,
            )
        }
    }
    (*cinfo).output_iMCU_row = 0 as i32 as JDIMENSION;
}
/* Forward declarations */
/*
 * Decompress and return some data in the single-pass case.
 * Always attempts to emit one fully interleaved MCU row ("iMCU" row).
 * Input and output must run in lockstep since we have only a one-MCU buffer.
 * Return value is JPEG_ROW_COMPLETED, JPEG_SCAN_COMPLETED, or JPEG_SUSPENDED.
 *
 * NB: output_buf contains a plane for each component in image,
 * which we index according to the component's SOF position.
 */

unsafe extern "C" fn decompress_onepass(
    mut cinfo: j_decompress_ptr,
    mut output_buf: JSAMPIMAGE,
) -> i32 {
    let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr; /* index of current MCU within row */
    let mut MCU_col_num: JDIMENSION = 0;
    let mut last_MCU_col: JDIMENSION =
        (*cinfo).MCUs_per_row.wrapping_sub(1 as i32 as u32);
    let mut last_iMCU_row: JDIMENSION =
        (*cinfo).total_iMCU_rows.wrapping_sub(1 as i32 as u32);
    let mut blkn: i32 = 0;
    let mut ci: i32 = 0;
    let mut xindex: i32 = 0;
    let mut yindex: i32 = 0;
    let mut yoffset: i32 = 0;
    let mut useful_width: i32 = 0;
    let mut output_ptr: JSAMPARRAY = 0 as *mut JSAMPROW;
    let mut start_col: JDIMENSION = 0;
    let mut output_col: JDIMENSION = 0;
    let mut compptr: *mut jpeg_component_info =
        0 as *mut jpeg_component_info;
    let mut inverse_DCT: inverse_DCT_method_ptr = None;
    /* Loop to process as much as one whole iMCU row */
    yoffset = (*coef).MCU_vert_offset;
    while yoffset < (*coef).MCU_rows_per_iMCU_row {
        MCU_col_num = (*coef).MCU_ctr;
        while MCU_col_num <= last_MCU_col {
            /* Try to fetch an MCU.  Entropy decoder expects buffer to be zeroed. */
            jzero_far(
                (*coef).MCU_buffer[0 as i32 as usize] as *mut libc::c_void,
                ((*cinfo).blocks_in_MCU as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<JBLOCK>() as libc::c_ulong
                    ),
            );
            if Some(
                (*(*cinfo).entropy)
                    .decode_mcu
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo, (*coef).MCU_buffer.as_mut_ptr()
            ) == 0
            {
                /* Suspension forced; update state counters and exit */
                (*coef).MCU_vert_offset = yoffset;
                (*coef).MCU_ctr = MCU_col_num;
                return 0 as i32;
            }
            /* Determine where data should go in output_buf and do the IDCT thing.
             * We skip dummy blocks at the right and bottom edges (but blkn gets
             * incremented past them!).  Note the inner loop relies on having
             * allocated the MCU_buffer[] blocks sequentially.
             */
            blkn = 0 as i32; /* index of current DCT block within MCU */
            ci = 0 as i32;
            while ci < (*cinfo).comps_in_scan {
                compptr = (*cinfo).cur_comp_info[ci as usize];
                /* Don't bother to IDCT an uninteresting component. */
                if (*compptr).component_needed == 0 {
                    blkn += (*compptr).MCU_blocks
                } else {
                    inverse_DCT = (*(*cinfo).idct).inverse_DCT[(*compptr).component_index as usize];
                    useful_width = if MCU_col_num < last_MCU_col {
                        (*compptr).MCU_width
                    } else {
                        (*compptr).last_col_width
                    };
                    output_ptr = (*output_buf.offset((*compptr).component_index as isize))
                        .offset((yoffset * (*compptr).DCT_v_scaled_size) as isize);
                    start_col = MCU_col_num.wrapping_mul((*compptr).MCU_sample_width as u32);
                    yindex = 0 as i32;
                    while yindex < (*compptr).MCU_height {
                        if (*cinfo).input_iMCU_row < last_iMCU_row
                            || yoffset + yindex < (*compptr).last_row_height
                        {
                            output_col = start_col;
                            xindex = 0 as i32;
                            while xindex < useful_width {
                                Some(inverse_DCT.expect("non-null function pointer"))
                                    .expect("non-null function pointer")(
                                    cinfo,
                                    compptr,
                                    (*coef).MCU_buffer[(blkn + xindex) as usize]
                                        as JCOEFPTR,
                                    output_ptr,
                                    output_col,
                                );
                                output_col = (output_col as u32)
                                    .wrapping_add((*compptr).DCT_h_scaled_size as u32)
                                    as JDIMENSION
                                    as JDIMENSION;
                                xindex += 1
                            }
                        }
                        blkn += (*compptr).MCU_width;
                        output_ptr = output_ptr.offset((*compptr).DCT_v_scaled_size as isize);
                        yindex += 1
                    }
                }
                ci += 1
            }
            MCU_col_num = MCU_col_num.wrapping_add(1)
        }
        /* Completed an MCU row, but perhaps not an iMCU row */
        (*coef).MCU_ctr = 0 as i32 as JDIMENSION;
        yoffset += 1
    }
    /* Completed the iMCU row, advance counters for next one */
    (*cinfo).output_iMCU_row = (*cinfo).output_iMCU_row.wrapping_add(1);
    (*cinfo).input_iMCU_row = (*cinfo).input_iMCU_row.wrapping_add(1);
    if (*cinfo).input_iMCU_row < (*cinfo).total_iMCU_rows {
        start_iMCU_row(cinfo);
        return 3 as i32;
    }
    /* Completed the scan */
    Some(
        (*(*cinfo).inputctl)
            .finish_input_pass
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo);
    return 4 as i32;
}
/*
 * Dummy consume-input routine for single-pass operation.
 */

unsafe extern "C" fn dummy_consume_data(mut _cinfo: j_decompress_ptr) -> i32 {
    return 0 as i32;
    /* Always indicate nothing was done */
}
/*
 * Consume input data and store it in the full-image coefficient buffer.
 * We read as much as one fully interleaved MCU row ("iMCU" row) per call,
 * ie, v_samp_factor block rows for each component in the scan.
 * Return value is JPEG_ROW_COMPLETED, JPEG_SCAN_COMPLETED, or JPEG_SUSPENDED.
 */

unsafe extern "C" fn consume_data(mut cinfo: j_decompress_ptr) -> i32 {
    let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr; /* index of current MCU within row */
    let mut MCU_col_num: JDIMENSION = 0;
    let mut blkn: i32 = 0;
    let mut ci: i32 = 0;
    let mut xindex: i32 = 0;
    let mut yindex: i32 = 0;
    let mut yoffset: i32 = 0;
    let mut start_col: JDIMENSION = 0;
    let mut buffer: [JBLOCKARRAY; 4] = [0 as *mut JBLOCKROW; 4];
    let mut buffer_ptr: JBLOCKROW = 0 as *mut JBLOCK;
    let mut compptr: *mut jpeg_component_info =
        0 as *mut jpeg_component_info;
    /* Align the virtual buffers for the components used in this scan. */
    ci = 0 as i32;
    while ci < (*cinfo).comps_in_scan {
        compptr = (*cinfo).cur_comp_info[ci as usize];
        buffer[ci as usize] = Some(
            (*(*cinfo).mem)
                .access_virt_barray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            (*coef).whole_image[(*compptr).component_index as usize],
            (*cinfo)
                .input_iMCU_row
                .wrapping_mul((*compptr).v_samp_factor as u32),
            (*compptr).v_samp_factor as JDIMENSION,
            1 as i32,
        );
        ci += 1
        /* Note: entropy decoder expects buffer to be zeroed,
         * but this is handled automatically by the memory manager
         * because we requested a pre-zeroed array.
         */
    }
    /* Loop to process one whole iMCU row */
    yoffset = (*coef).MCU_vert_offset;
    while yoffset < (*coef).MCU_rows_per_iMCU_row {
        MCU_col_num = (*coef).MCU_ctr;
        while MCU_col_num < (*cinfo).MCUs_per_row {
            /* Construct list of pointers to DCT blocks belonging to this MCU */
            blkn = 0 as i32; /* index of current DCT block within MCU */
            ci = 0 as i32;
            while ci < (*cinfo).comps_in_scan {
                compptr = (*cinfo).cur_comp_info[ci as usize];
                start_col = MCU_col_num.wrapping_mul((*compptr).MCU_width as u32);
                yindex = 0 as i32;
                while yindex < (*compptr).MCU_height {
                    buffer_ptr = (*buffer[ci as usize].offset((yindex + yoffset) as isize))
                        .offset(start_col as isize);
                    xindex = 0 as i32;
                    while xindex < (*compptr).MCU_width {
                        let fresh0 = buffer_ptr;
                        buffer_ptr = buffer_ptr.offset(1);
                        let fresh1 = blkn;
                        blkn = blkn + 1;
                        (*coef).MCU_buffer[fresh1 as usize] = fresh0;
                        xindex += 1
                    }
                    yindex += 1
                }
                ci += 1
            }
            /* Try to fetch the MCU. */
            if Some(
                (*(*cinfo).entropy)
                    .decode_mcu
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo, (*coef).MCU_buffer.as_mut_ptr()
            ) == 0
            {
                /* Suspension forced; update state counters and exit */
                (*coef).MCU_vert_offset = yoffset;
                (*coef).MCU_ctr = MCU_col_num;
                return 0 as i32;
            }
            MCU_col_num = MCU_col_num.wrapping_add(1)
        }
        /* Completed an MCU row, but perhaps not an iMCU row */
        (*coef).MCU_ctr = 0 as i32 as JDIMENSION;
        yoffset += 1
    }
    /* Completed the iMCU row, advance counters for next one */
    (*cinfo).input_iMCU_row = (*cinfo).input_iMCU_row.wrapping_add(1);
    if (*cinfo).input_iMCU_row < (*cinfo).total_iMCU_rows {
        start_iMCU_row(cinfo);
        return 3 as i32;
    }
    /* Completed the scan */
    Some(
        (*(*cinfo).inputctl)
            .finish_input_pass
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo);
    return 4 as i32;
}
/*
 * Decompress and return some data in the multi-pass case.
 * Always attempts to emit one fully interleaved MCU row ("iMCU" row).
 * Return value is JPEG_ROW_COMPLETED, JPEG_SCAN_COMPLETED, or JPEG_SUSPENDED.
 *
 * NB: output_buf contains a plane for each component in image.
 */

unsafe extern "C" fn decompress_data(
    mut cinfo: j_decompress_ptr,
    mut output_buf: JSAMPIMAGE,
) -> i32 {
    let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr;
    let mut last_iMCU_row: JDIMENSION =
        (*cinfo).total_iMCU_rows.wrapping_sub(1 as i32 as u32);
    let mut block_num: JDIMENSION = 0;
    let mut ci: i32 = 0;
    let mut block_row: i32 = 0;
    let mut block_rows: i32 = 0;
    let mut buffer: JBLOCKARRAY = 0 as *mut JBLOCKROW;
    let mut buffer_ptr: JBLOCKROW = 0 as *mut JBLOCK;
    let mut output_ptr: JSAMPARRAY = 0 as *mut JSAMPROW;
    let mut output_col: JDIMENSION = 0;
    let mut compptr: *mut jpeg_component_info =
        0 as *mut jpeg_component_info;
    let mut inverse_DCT: inverse_DCT_method_ptr = None;
    /* Force some input to be done if we are getting ahead of the input. */
    while (*cinfo).input_scan_number < (*cinfo).output_scan_number
        || (*cinfo).input_scan_number == (*cinfo).output_scan_number
            && (*cinfo).input_iMCU_row <= (*cinfo).output_iMCU_row
    {
        if Some(
            (*(*cinfo).inputctl)
                .consume_input
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0 as i32
        {
            return 0 as i32;
        }
    }
    /* OK, output from the virtual arrays. */
    ci = 0 as i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        /* Don't bother to IDCT an uninteresting component. */
        if !((*compptr).component_needed == 0) {
            /* Align the virtual buffer for this component. */
            buffer = Some(
                (*(*cinfo).mem)
                    .access_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr,
                (*coef).whole_image[ci as usize],
                (*cinfo)
                    .output_iMCU_row
                    .wrapping_mul((*compptr).v_samp_factor as u32),
                (*compptr).v_samp_factor as JDIMENSION,
                0 as i32,
            );
            /* Count non-dummy DCT block rows in this iMCU row. */
            if (*cinfo).output_iMCU_row < last_iMCU_row {
                block_rows = (*compptr).v_samp_factor
            } else {
                /* NB: can't use last_row_height here; it is input-side-dependent! */
                block_rows = (*compptr)
                    .height_in_blocks
                    .wrapping_rem((*compptr).v_samp_factor as u32)
                    as i32;
                if block_rows == 0 as i32 {
                    block_rows = (*compptr).v_samp_factor
                }
            }
            inverse_DCT = (*(*cinfo).idct).inverse_DCT[ci as usize];
            output_ptr = *output_buf.offset(ci as isize);
            /* Loop over all DCT blocks to be processed. */
            block_row = 0 as i32;
            while block_row < block_rows {
                buffer_ptr = *buffer.offset(block_row as isize);
                output_col = 0 as i32 as JDIMENSION;
                block_num = 0 as i32 as JDIMENSION;
                while block_num < (*compptr).width_in_blocks {
                    Some(inverse_DCT.expect("non-null function pointer"))
                        .expect("non-null function pointer")(
                        cinfo,
                        compptr,
                        buffer_ptr as JCOEFPTR,
                        output_ptr,
                        output_col,
                    );
                    buffer_ptr = buffer_ptr.offset(1);
                    output_col = (output_col as u32)
                        .wrapping_add((*compptr).DCT_h_scaled_size as u32)
                        as JDIMENSION
                        as JDIMENSION;
                    block_num = block_num.wrapping_add(1)
                }
                output_ptr = output_ptr.offset((*compptr).DCT_v_scaled_size as isize);
                block_row += 1
            }
        }
        ci += 1;
        compptr = compptr.offset(1)
    }
    (*cinfo).output_iMCU_row = (*cinfo).output_iMCU_row.wrapping_add(1);
    if (*cinfo).output_iMCU_row < (*cinfo).total_iMCU_rows {
        return 3 as i32;
    }
    return 4 as i32;
}
/*
 * Determine whether block smoothing is applicable and safe.
 * We also latch the current states of the coef_bits[] entries for the
 * AC coefficients; otherwise, if the input side of the decompressor
 * advances into a new scan, we might think the coefficients are known
 * more accurately than they really are.
 */

unsafe extern "C" fn smoothing_ok(
    mut cinfo: j_decompress_ptr,
) -> boolean {
    let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr;
    let mut smoothing_useful: boolean = 0 as i32;
    let mut ci: i32 = 0;
    let mut coefi: i32 = 0;
    let mut compptr: *mut jpeg_component_info =
        0 as *mut jpeg_component_info;
    let mut qtable: *mut JQUANT_TBL = 0 as *mut JQUANT_TBL;
    let mut coef_bits: *mut i32 = 0 as *mut i32;
    let mut coef_bits_latch: *mut i32 = 0 as *mut i32;
    if (*cinfo).progressive_mode == 0 || (*cinfo).coef_bits.is_null() {
        return 0 as i32;
    }
    /* Allocate latch area if not already done */
    if (*coef).coef_bits_latch.is_null() {
        (*coef).coef_bits_latch = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            1 as i32,
            ((*cinfo).num_components as libc::c_ulong).wrapping_mul(
                (6 as i32 as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<i32>() as libc::c_ulong),
            ),
        ) as *mut i32
    }
    coef_bits_latch = (*coef).coef_bits_latch;
    ci = 0 as i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        /* All components' quantization values must already be latched. */
        qtable = (*compptr).quant_table;
        if qtable.is_null() {
            return 0 as i32;
        }
        /* Verify DC & first 5 AC quantizers are nonzero to avoid zero-divide. */
        if (*qtable).quantval[0 as i32 as usize] as i32 == 0 as i32
            || (*qtable).quantval[1 as i32 as usize] as i32 == 0 as i32
            || (*qtable).quantval[8 as i32 as usize] as i32 == 0 as i32
            || (*qtable).quantval[16 as i32 as usize] as i32 == 0 as i32
            || (*qtable).quantval[9 as i32 as usize] as i32 == 0 as i32
            || (*qtable).quantval[2 as i32 as usize] as i32 == 0 as i32
        {
            return 0 as i32;
        }
        /* DC values must be at least partly known for all components. */
        coef_bits = (*(*cinfo).coef_bits.offset(ci as isize)).as_mut_ptr();
        if *coef_bits.offset(0 as i32 as isize) < 0 as i32 {
            return 0 as i32;
        }
        /* Block smoothing is helpful if some AC coefficients remain inaccurate. */
        coefi = 1 as i32;
        while coefi <= 5 as i32 {
            *coef_bits_latch.offset(coefi as isize) = *coef_bits.offset(coefi as isize);
            if *coef_bits.offset(coefi as isize) != 0 as i32 {
                smoothing_useful = 1 as i32
            }
            coefi += 1
        }
        coef_bits_latch = coef_bits_latch.offset(6 as i32 as isize);
        ci += 1;
        compptr = compptr.offset(1)
    }
    return smoothing_useful;
}
/*
 * Variant of decompress_data for use when doing block smoothing.
 */

unsafe extern "C" fn decompress_smooth_data(
    mut cinfo: j_decompress_ptr,
    mut output_buf: JSAMPIMAGE,
) -> i32 {
    let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr;
    let mut last_iMCU_row: JDIMENSION =
        (*cinfo).total_iMCU_rows.wrapping_sub(1 as i32 as u32);
    let mut block_num: JDIMENSION = 0;
    let mut last_block_column: JDIMENSION = 0;
    let mut ci: i32 = 0;
    let mut block_row: i32 = 0;
    let mut block_rows: i32 = 0;
    let mut access_rows: i32 = 0;
    let mut buffer: JBLOCKARRAY = 0 as *mut JBLOCKROW;
    let mut buffer_ptr: JBLOCKROW = 0 as *mut JBLOCK;
    let mut prev_block_row: JBLOCKROW = 0 as *mut JBLOCK;
    let mut next_block_row: JBLOCKROW = 0 as *mut JBLOCK;
    let mut output_ptr: JSAMPARRAY = 0 as *mut JSAMPROW;
    let mut output_col: JDIMENSION = 0;
    let mut compptr: *mut jpeg_component_info =
        0 as *mut jpeg_component_info;
    let mut inverse_DCT: inverse_DCT_method_ptr = None;
    let mut first_row: boolean = 0;
    let mut last_row: boolean = 0;
    let mut workspace: JBLOCK = [0; 64];
    let mut coef_bits: *mut i32 = 0 as *mut i32;
    let mut quanttbl: *mut JQUANT_TBL = 0 as *mut JQUANT_TBL;
    let mut Q00: INT32 = 0;
    let mut Q01: INT32 = 0;
    let mut Q02: INT32 = 0;
    let mut Q10: INT32 = 0;
    let mut Q11: INT32 = 0;
    let mut Q20: INT32 = 0;
    let mut num: INT32 = 0;
    let mut DC1: i32 = 0;
    let mut DC2: i32 = 0;
    let mut DC3: i32 = 0;
    let mut DC4: i32 = 0;
    let mut DC5: i32 = 0;
    let mut DC6: i32 = 0;
    let mut DC7: i32 = 0;
    let mut DC8: i32 = 0;
    let mut DC9: i32 = 0;
    let mut Al: i32 = 0;
    let mut pred: i32 = 0;
    /* Force some input to be done if we are getting ahead of the input. */
    while (*cinfo).input_scan_number <= (*cinfo).output_scan_number
        && (*(*cinfo).inputctl).eoi_reached == 0
    {
        if (*cinfo).input_scan_number == (*cinfo).output_scan_number {
            /* If input is working on current scan, we ordinarily want it to
             * have completed the current row.  But if input scan is DC,
             * we want it to keep one row ahead so that next block row's DC
             * values are up to date.
             */
            let mut delta: JDIMENSION = if (*cinfo).Ss == 0 as i32 {
                1 as i32
            } else {
                0 as i32
            }
                as JDIMENSION;
            if (*cinfo).input_iMCU_row > (*cinfo).output_iMCU_row.wrapping_add(delta) {
                break;
            }
        }
        if Some(
            (*(*cinfo).inputctl)
                .consume_input
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0 as i32
        {
            return 0 as i32;
        }
    }
    /* OK, output from the virtual arrays. */
    ci = 0 as i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        /* Don't bother to IDCT an uninteresting component. */
        if !((*compptr).component_needed == 0) {
            /* Count non-dummy DCT block rows in this iMCU row. */
            if (*cinfo).output_iMCU_row < last_iMCU_row {
                block_rows = (*compptr).v_samp_factor; /* this and next iMCU row */
                access_rows = block_rows * 2 as i32;
                last_row = 0 as i32
            } else {
                /* NB: can't use last_row_height here; it is input-side-dependent! */
                block_rows = (*compptr)
                    .height_in_blocks
                    .wrapping_rem((*compptr).v_samp_factor as u32)
                    as i32; /* this iMCU row only */
                if block_rows == 0 as i32 {
                    block_rows = (*compptr).v_samp_factor
                }
                access_rows = block_rows;
                last_row = 1 as i32
            }
            /* Align the virtual buffer for this component. */
            if (*cinfo).output_iMCU_row > 0 as i32 as u32 {
                access_rows += (*compptr).v_samp_factor; /* prior iMCU row too */
                buffer = Some(
                    (*(*cinfo).mem)
                        .access_virt_barray
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr,
                    (*coef).whole_image[ci as usize],
                    (*cinfo)
                        .output_iMCU_row
                        .wrapping_sub(1 as i32 as u32)
                        .wrapping_mul((*compptr).v_samp_factor as u32),
                    access_rows as JDIMENSION,
                    0 as i32,
                ); /* point to current iMCU row */
                buffer = buffer.offset((*compptr).v_samp_factor as isize);
                first_row = 0 as i32
            } else {
                buffer = Some(
                    (*(*cinfo).mem)
                        .access_virt_barray
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr,
                    (*coef).whole_image[ci as usize],
                    0 as i32 as JDIMENSION,
                    access_rows as JDIMENSION,
                    0 as i32,
                );
                first_row = 1 as i32
            }
            /* Fetch component-dependent info */
            coef_bits = (*coef).coef_bits_latch.offset((ci * 6 as i32) as isize);
            quanttbl = (*compptr).quant_table;
            Q00 = (*quanttbl).quantval[0 as i32 as usize] as INT32;
            Q01 = (*quanttbl).quantval[1 as i32 as usize] as INT32;
            Q10 = (*quanttbl).quantval[8 as i32 as usize] as INT32;
            Q20 = (*quanttbl).quantval[16 as i32 as usize] as INT32;
            Q11 = (*quanttbl).quantval[9 as i32 as usize] as INT32;
            Q02 = (*quanttbl).quantval[2 as i32 as usize] as INT32;
            inverse_DCT = (*(*cinfo).idct).inverse_DCT[ci as usize];
            output_ptr = *output_buf.offset(ci as isize);
            /* Loop over all DCT blocks to be processed. */
            block_row = 0 as i32;
            while block_row < block_rows {
                buffer_ptr = *buffer.offset(block_row as isize);
                if first_row != 0 && block_row == 0 as i32 {
                    prev_block_row = buffer_ptr
                } else {
                    prev_block_row = *buffer.offset((block_row - 1 as i32) as isize)
                }
                if last_row != 0 && block_row == block_rows - 1 as i32 {
                    next_block_row = buffer_ptr
                } else {
                    next_block_row = *buffer.offset((block_row + 1 as i32) as isize)
                }
                /* We fetch the surrounding DC values using a sliding-register approach.
                 * Initialize all nine here so as to do the right thing on narrow pics.
                 */
                DC3 = (*prev_block_row.offset(0 as i32 as isize))[0 as i32 as usize] as i32;
                DC2 = DC3;
                DC1 = DC2;
                DC6 = (*buffer_ptr.offset(0 as i32 as isize))[0 as i32 as usize] as i32;
                DC5 = DC6;
                DC4 = DC5;
                DC9 = (*next_block_row.offset(0 as i32 as isize))[0 as i32 as usize] as i32;
                DC8 = DC9;
                DC7 = DC8;
                output_col = 0 as i32 as JDIMENSION;
                last_block_column = (*compptr).width_in_blocks.wrapping_sub(1 as i32 as u32);
                block_num = 0 as i32 as JDIMENSION;
                while block_num <= last_block_column {
                    /* Fetch current DCT block into workspace so we can modify it. */
                    jcopy_block_row(
                        buffer_ptr,
                        workspace.as_mut_ptr() as JBLOCKROW,
                        1 as i32 as JDIMENSION,
                    );
                    /* Update DC values */
                    if block_num < last_block_column {
                        DC3 = (*prev_block_row.offset(1 as i32 as isize))[0 as i32 as usize] as i32;
                        DC6 = (*buffer_ptr.offset(1 as i32 as isize))[0 as i32 as usize] as i32;
                        DC9 = (*next_block_row.offset(1 as i32 as isize))[0 as i32 as usize] as i32
                    }
                    /* Compute coefficient estimates per K.8.
                     * An estimate is applied only if coefficient is still zero,
                     * and is not known to be fully accurate.
                     */
                    /* AC01 */
                    Al = *coef_bits.offset(1 as i32 as isize);
                    if Al != 0 as i32 && workspace[1 as i32 as usize] as i32 == 0 as i32 {
                        num = 36 as i32 as isize * Q00 * (DC4 - DC6) as isize;
                        if num >= 0 as i32 as isize {
                            pred = (((Q01 << 7 as i32) + num) / (Q01 << 8 as i32)) as i32;
                            if Al > 0 as i32 && pred >= (1 as i32) << Al {
                                pred = ((1 as i32) << Al) - 1 as i32
                            }
                        } else {
                            pred = (((Q01 << 7 as i32) - num) / (Q01 << 8 as i32)) as i32;
                            if Al > 0 as i32 && pred >= (1 as i32) << Al {
                                pred = ((1 as i32) << Al) - 1 as i32
                            }
                            pred = -pred
                        }
                        workspace[1 as i32 as usize] = pred as JCOEF
                    }
                    /* AC10 */
                    Al = *coef_bits.offset(2 as i32 as isize);
                    if Al != 0 as i32 && workspace[8 as i32 as usize] as i32 == 0 as i32 {
                        num = 36 as i32 as isize * Q00 * (DC2 - DC8) as isize;
                        if num >= 0 as i32 as isize {
                            pred = (((Q10 << 7 as i32) + num) / (Q10 << 8 as i32)) as i32;
                            if Al > 0 as i32 && pred >= (1 as i32) << Al {
                                pred = ((1 as i32) << Al) - 1 as i32
                            }
                        } else {
                            pred = (((Q10 << 7 as i32) - num) / (Q10 << 8 as i32)) as i32;
                            if Al > 0 as i32 && pred >= (1 as i32) << Al {
                                pred = ((1 as i32) << Al) - 1 as i32
                            }
                            pred = -pred
                        }
                        workspace[8 as i32 as usize] = pred as JCOEF
                    }
                    /* AC20 */
                    Al = *coef_bits.offset(3 as i32 as isize);
                    if Al != 0 as i32 && workspace[16 as i32 as usize] as i32 == 0 as i32 {
                        num = 9 as i32 as isize * Q00 * (DC2 + DC8 - 2 as i32 * DC5) as isize;
                        if num >= 0 as i32 as isize {
                            pred = (((Q20 << 7 as i32) + num) / (Q20 << 8 as i32)) as i32;
                            if Al > 0 as i32 && pred >= (1 as i32) << Al {
                                pred = ((1 as i32) << Al) - 1 as i32
                            }
                        } else {
                            pred = (((Q20 << 7 as i32) - num) / (Q20 << 8 as i32)) as i32;
                            if Al > 0 as i32 && pred >= (1 as i32) << Al {
                                pred = ((1 as i32) << Al) - 1 as i32
                            }
                            pred = -pred
                        }
                        workspace[16 as i32 as usize] = pred as JCOEF
                    }
                    /* AC11 */
                    Al = *coef_bits.offset(4 as i32 as isize);
                    if Al != 0 as i32 && workspace[9 as i32 as usize] as i32 == 0 as i32 {
                        num = 5 as i32 as isize * Q00 * (DC1 - DC3 - DC7 + DC9) as isize;
                        if num >= 0 as i32 as isize {
                            pred = (((Q11 << 7 as i32) + num) / (Q11 << 8 as i32)) as i32;
                            if Al > 0 as i32 && pred >= (1 as i32) << Al {
                                pred = ((1 as i32) << Al) - 1 as i32
                            }
                        } else {
                            pred = (((Q11 << 7 as i32) - num) / (Q11 << 8 as i32)) as i32;
                            if Al > 0 as i32 && pred >= (1 as i32) << Al {
                                pred = ((1 as i32) << Al) - 1 as i32
                            }
                            pred = -pred
                        }
                        workspace[9 as i32 as usize] = pred as JCOEF
                    }
                    /* AC02 */
                    Al = *coef_bits.offset(5 as i32 as isize);
                    if Al != 0 as i32 && workspace[2 as i32 as usize] as i32 == 0 as i32 {
                        num = 9 as i32 as isize * Q00 * (DC4 + DC6 - 2 as i32 * DC5) as isize;
                        if num >= 0 as i32 as isize {
                            pred = (((Q02 << 7 as i32) + num) / (Q02 << 8 as i32)) as i32;
                            if Al > 0 as i32 && pred >= (1 as i32) << Al {
                                pred = ((1 as i32) << Al) - 1 as i32
                            }
                        } else {
                            pred = (((Q02 << 7 as i32) - num) / (Q02 << 8 as i32)) as i32;
                            if Al > 0 as i32 && pred >= (1 as i32) << Al {
                                pred = ((1 as i32) << Al) - 1 as i32
                            }
                            pred = -pred
                        }
                        workspace[2 as i32 as usize] = pred as JCOEF
                    }
                    /* OK, do the IDCT */
                    Some(inverse_DCT.expect("non-null function pointer"))
                        .expect("non-null function pointer")(
                        cinfo,
                        compptr,
                        workspace.as_mut_ptr(),
                        output_ptr,
                        output_col,
                    );
                    /* Advance for next column */
                    DC1 = DC2;
                    DC2 = DC3;
                    DC4 = DC5;
                    DC5 = DC6;
                    DC7 = DC8;
                    DC8 = DC9;
                    buffer_ptr = buffer_ptr.offset(1);
                    prev_block_row = prev_block_row.offset(1);
                    next_block_row = next_block_row.offset(1);
                    output_col = (output_col as u32)
                        .wrapping_add((*compptr).DCT_h_scaled_size as u32)
                        as JDIMENSION
                        as JDIMENSION;
                    block_num = block_num.wrapping_add(1)
                }
                output_ptr = output_ptr.offset((*compptr).DCT_v_scaled_size as isize);
                block_row += 1
            }
        }
        ci += 1;
        compptr = compptr.offset(1)
    }
    (*cinfo).output_iMCU_row = (*cinfo).output_iMCU_row.wrapping_add(1);
    if (*cinfo).output_iMCU_row < (*cinfo).total_iMCU_rows {
        return 3 as i32;
    }
    return 4 as i32;
}
/* BLOCK_SMOOTHING_SUPPORTED */
/*
 * Initialize coefficient buffer controller.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_d_coef_controller(
    mut cinfo: j_decompress_ptr,
    mut need_full_buffer: boolean,
) {
    let mut coef: my_coef_ptr = 0 as *mut my_coef_controller;
    coef = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        1 as i32,
        ::std::mem::size_of::<my_coef_controller>() as libc::c_ulong,
    ) as my_coef_ptr;
    (*cinfo).coef = coef as *mut jpeg_d_coef_controller;
    (*coef).pub_0.start_input_pass =
        Some(start_input_pass as unsafe extern "C" fn(_: j_decompress_ptr) -> ());
    (*coef).pub_0.start_output_pass = Some(
        start_output_pass as unsafe extern "C" fn(_: j_decompress_ptr) -> (),
    );
    (*coef).coef_bits_latch = 0 as *mut i32;
    /* Create the coefficient buffer. */
    if need_full_buffer != 0 {
        /* Allocate a full-image virtual array for each component, */
        /* padded to a multiple of samp_factor DCT blocks in each direction. */
        /* Note we ask for a pre-zeroed array. */
        let mut ci: i32 = 0;
        let mut access_rows: i32 = 0;
        let mut compptr: *mut jpeg_component_info =
            0 as *mut jpeg_component_info;
        ci = 0 as i32;
        compptr = (*cinfo).comp_info;
        while ci < (*cinfo).num_components {
            access_rows = (*compptr).v_samp_factor;
            /* If block smoothing could be used, need a bigger window */
            if (*cinfo).progressive_mode != 0 {
                access_rows *= 3 as i32
            }
            (*coef).whole_image[ci as usize] = Some(
                (*(*cinfo).mem)
                    .request_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr,
                1 as i32,
                1 as i32,
                jround_up(
                    (*compptr).width_in_blocks as isize,
                    (*compptr).h_samp_factor as isize,
                ) as JDIMENSION,
                jround_up(
                    (*compptr).height_in_blocks as isize,
                    (*compptr).v_samp_factor as isize,
                ) as JDIMENSION,
                access_rows as JDIMENSION,
            );
            ci += 1;
            compptr = compptr.offset(1)
        }
        (*coef).pub_0.consume_data = Some(
            consume_data as unsafe extern "C" fn(_: j_decompress_ptr) -> i32,
        );
        (*coef).pub_0.decompress_data = Some(
            decompress_data
                as unsafe extern "C" fn(
                    _: j_decompress_ptr,
                    _: JSAMPIMAGE,
                ) -> i32,
        );
        (*coef).pub_0.coef_arrays = (*coef).whole_image.as_mut_ptr()
    } else {
        /* We only need a single-MCU buffer. */
        let mut buffer: JBLOCKROW = 0 as *mut JBLOCK;
        let mut i: i32 = 0;
        buffer = Some(
            (*(*cinfo).mem)
                .alloc_large
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            1 as i32,
            (10 as i32 as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<JBLOCK>() as libc::c_ulong),
        ) as JBLOCKROW;
        i = 0 as i32;
        while i < 10 as i32 {
            (*coef).MCU_buffer[i as usize] = buffer.offset(i as isize);
            i += 1
        }
        (*coef).pub_0.consume_data = Some(
            dummy_consume_data
                as unsafe extern "C" fn(_: j_decompress_ptr) -> i32,
        );
        (*coef).pub_0.decompress_data = Some(
            decompress_onepass
                as unsafe extern "C" fn(
                    _: j_decompress_ptr,
                    _: JSAMPIMAGE,
                ) -> i32,
        );
        (*coef).pub_0.coef_arrays = 0 as *mut jvirt_barray_ptr
    };
}
