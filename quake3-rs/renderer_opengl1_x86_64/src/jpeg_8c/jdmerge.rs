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
pub use crate::src::jpeg_8c::jutils::jcopy_sample_rows;

pub type my_upsample_ptr = *mut my_upsampler;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_upsampler {
    pub pub_0: crate::jpegint_h::jpeg_upsampler,
    pub upmethod: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_decompress_ptr,
            _: crate::jpeglib_h::JSAMPIMAGE,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jpeglib_h::JSAMPARRAY,
        ) -> (),
    >,
    pub Cr_r_tab: *mut i32,
    pub Cb_b_tab: *mut i32,
    pub Cr_g_tab: *mut crate::jmorecfg_h::INT32,
    pub Cb_g_tab: *mut crate::jmorecfg_h::INT32,
    pub spare_row: crate::jpeglib_h::JSAMPROW,
    pub spare_full: crate::jmorecfg_h::boolean,
    pub out_row_width: crate::jmorecfg_h::JDIMENSION,
    pub rows_to_go: crate::jmorecfg_h::JDIMENSION,
}
/*
 * Initialize tables for YCC->RGB colorspace conversion.
 * This is taken directly from jdcolor.c; see that file for more info.
 */

unsafe extern "C" fn build_ycc_rgb_table(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    let mut i: i32 = 0;
    let mut x: crate::jmorecfg_h::INT32 = 0;
    (*upsample).Cr_r_tab = Some(
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
    (*upsample).Cb_b_tab = Some(
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
    (*upsample).Cr_g_tab = Some(
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
    (*upsample).Cb_g_tab = Some(
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
        *(*upsample).Cr_r_tab.offset(i as isize) =
            ((1.40200f64 * ((1 as isize) << 16 as i32) as f64 + 0.5f64) as crate::jmorecfg_h::INT32
                * x
                + ((1 as i32 as crate::jmorecfg_h::INT32) << 16 as i32 - 1 as i32)
                >> 16 as i32) as i32;
        /* Cb=>B value is nearest int to 1.77200 * x */
        *(*upsample).Cb_b_tab.offset(i as isize) =
            ((1.77200f64 * ((1 as isize) << 16 as i32) as f64 + 0.5f64) as crate::jmorecfg_h::INT32
                * x
                + ((1 as i32 as crate::jmorecfg_h::INT32) << 16 as i32 - 1 as i32)
                >> 16 as i32) as i32;
        /* Cr=>G value is scaled-up -0.71414 * x */
        *(*upsample).Cr_g_tab.offset(i as isize) =
            -((0.71414f64 * ((1 as isize) << 16 as i32) as f64 + 0.5f64)
                as crate::jmorecfg_h::INT32)
                * x;
        /* Cb=>G value is scaled-up -0.34414 * x */
        /* We also add in ONE_HALF so that need not do it in inner loop */
        *(*upsample).Cb_g_tab.offset(i as isize) =
            -((0.34414f64 * ((1 as isize) << 16 as i32) as f64 + 0.5f64)
                as crate::jmorecfg_h::INT32)
                * x
                + ((1 as i32 as crate::jmorecfg_h::INT32) << 16 as i32 - 1 as i32);
        i += 1;
        x += 1
    }
}
/*
 * Initialize for an upsampling pass.
 */

unsafe extern "C" fn start_pass_merged_upsample(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    /* Mark the spare buffer empty */
    (*upsample).spare_full = 0 as i32;
    /* Initialize total-height counter for detecting bottom of image */
    (*upsample).rows_to_go = (*cinfo).output_height;
}
/*
 * Control routine to do upsampling (and color conversion).
 *
 * The control routine just handles the row buffering considerations.
 */

unsafe extern "C" fn merged_2v_upsample(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut in_row_group_ctr: *mut crate::jmorecfg_h::JDIMENSION,
    mut _in_row_groups_avail: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut out_row_ctr: *mut crate::jmorecfg_h::JDIMENSION,
    mut out_rows_avail: crate::jmorecfg_h::JDIMENSION,
)
/* 2:1 vertical sampling case: may need a spare row. */
{
    let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr; /* number of rows returned to caller */
    let mut work_ptrs: [crate::jpeglib_h::JSAMPROW; 2] = [0 as *mut crate::jmorecfg_h::JSAMPLE; 2];
    let mut num_rows: crate::jmorecfg_h::JDIMENSION = 0;
    if (*upsample).spare_full != 0 {
        /* If we have a spare row saved from a previous cycle, just return it. */
        crate::src::jpeg_8c::jutils::jcopy_sample_rows(
            &mut (*upsample).spare_row,
            0 as i32,
            output_buf.offset(*out_row_ctr as isize),
            0 as i32,
            1 as i32,
            (*upsample).out_row_width,
        );
        num_rows = 1 as i32 as crate::jmorecfg_h::JDIMENSION;
        (*upsample).spare_full = 0 as i32
    } else {
        /* Figure number of rows to return to caller. */
        num_rows = 2 as i32 as crate::jmorecfg_h::JDIMENSION;
        /* Not more than the distance to the end of the image. */
        if num_rows > (*upsample).rows_to_go {
            num_rows = (*upsample).rows_to_go
        }
        /* And not more than what the client can accept: */
        out_rows_avail = (out_rows_avail as u32).wrapping_sub(*out_row_ctr)
            as crate::jmorecfg_h::JDIMENSION
            as crate::jmorecfg_h::JDIMENSION;
        if num_rows > out_rows_avail {
            num_rows = out_rows_avail
        }
        /* Create output pointer array for upsampler. */
        work_ptrs[0 as i32 as usize] = *output_buf.offset(*out_row_ctr as isize);
        if num_rows > 1 as i32 as u32 {
            work_ptrs[1 as i32 as usize] =
                *output_buf.offset((*out_row_ctr).wrapping_add(1 as i32 as u32) as isize)
        } else {
            work_ptrs[1 as i32 as usize] = (*upsample).spare_row;
            (*upsample).spare_full = 1 as i32
        }
        /* Now do the upsampling. */
        Some((*upsample).upmethod.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            cinfo,
            input_buf,
            *in_row_group_ctr,
            work_ptrs.as_mut_ptr(),
        );
    }
    /* Adjust counts */
    *out_row_ctr = (*out_row_ctr as u32).wrapping_add(num_rows) as crate::jmorecfg_h::JDIMENSION
        as crate::jmorecfg_h::JDIMENSION;
    (*upsample).rows_to_go = ((*upsample).rows_to_go as u32).wrapping_sub(num_rows)
        as crate::jmorecfg_h::JDIMENSION
        as crate::jmorecfg_h::JDIMENSION;
    /* When the buffer is emptied, declare this input row group consumed */
    if (*upsample).spare_full == 0 {
        *in_row_group_ctr = (*in_row_group_ctr).wrapping_add(1)
    };
}

unsafe extern "C" fn merged_1v_upsample(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut in_row_group_ctr: *mut crate::jmorecfg_h::JDIMENSION,
    mut _in_row_groups_avail: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut out_row_ctr: *mut crate::jmorecfg_h::JDIMENSION,
    mut _out_rows_avail: crate::jmorecfg_h::JDIMENSION,
)
/* 1:1 vertical sampling case: much easier, never need a spare row. */
{
    let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    /* Just do the upsampling. */
    Some((*upsample).upmethod.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        cinfo,
        input_buf,
        *in_row_group_ctr,
        output_buf.offset(*out_row_ctr as isize),
    );
    /* Adjust counts */
    *out_row_ctr = (*out_row_ctr).wrapping_add(1);
    *in_row_group_ctr = (*in_row_group_ctr).wrapping_add(1);
}
/*
 * These are the routines invoked by the control routines to do
 * the actual upsampling/conversion.  One row group is processed per call.
 *
 * Note: since we may be writing directly into application-supplied buffers,
 * we have to be honest about the output width; we can't assume the buffer
 * has been rounded up to an even width.
 */
/*
 * Upsample and color convert for the case of 2:1 horizontal and 1:1 vertical.
 */

unsafe extern "C" fn h2v1_merged_upsample(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
) {
    let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    let mut y: i32 = 0;
    let mut cred: i32 = 0;
    let mut cgreen: i32 = 0;
    let mut cblue: i32 = 0;
    let mut cb: i32 = 0;
    let mut cr: i32 = 0;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut inptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    /* copy these pointers into registers if possible */
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut i32 = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut i32 = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut crate::jmorecfg_h::INT32 = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut crate::jmorecfg_h::INT32 = (*upsample).Cb_g_tab;
    inptr0 = *(*input_buf.offset(0 as i32 as isize)).offset(in_row_group_ctr as isize);
    inptr1 = *(*input_buf.offset(1 as i32 as isize)).offset(in_row_group_ctr as isize);
    inptr2 = *(*input_buf.offset(2 as i32 as isize)).offset(in_row_group_ctr as isize);
    outptr = *output_buf.offset(0 as i32 as isize);
    /* Loop for each pair of output pixels */
    col = (*cinfo).output_width >> 1 as i32;
    while col > 0 as i32 as u32 {
        /* Do the chroma part of the calculation */
        let fresh0 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh0 as i32;
        let fresh1 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh1 as i32;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16 as i32) as i32;
        cblue = *Cbbtab.offset(cb as isize);
        /* Fetch 2 Y values and emit 2 pixels */
        let fresh2 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh2 as i32;
        *outptr.offset(0 as i32 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(1 as i32 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(2 as i32 as isize) = *range_limit.offset((y + cblue) as isize);
        outptr = outptr.offset(3 as i32 as isize);
        let fresh3 = inptr0;
        inptr0 = inptr0.offset(1);
        y = *fresh3 as i32;
        *outptr.offset(0 as i32 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(1 as i32 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(2 as i32 as isize) = *range_limit.offset((y + cblue) as isize);
        outptr = outptr.offset(3 as i32 as isize);
        col = col.wrapping_sub(1)
    }
    /* If image width is odd, do the last output column separately */
    if (*cinfo).output_width & 1 as i32 as u32 != 0 {
        cb = *inptr1 as i32;
        cr = *inptr2 as i32;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16 as i32) as i32;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr0 as i32;
        *outptr.offset(0 as i32 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr.offset(1 as i32 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr.offset(2 as i32 as isize) = *range_limit.offset((y + cblue) as isize)
    };
}
/*
 * Upsample and color convert for the case of 2:1 horizontal and 2:1 vertical.
 */

unsafe extern "C" fn h2v2_merged_upsample(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut in_row_group_ctr: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
) {
    let mut upsample: my_upsample_ptr = (*cinfo).upsample as my_upsample_ptr;
    let mut y: i32 = 0;
    let mut cred: i32 = 0;
    let mut cgreen: i32 = 0;
    let mut cblue: i32 = 0;
    let mut cb: i32 = 0;
    let mut cr: i32 = 0;
    let mut outptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut outptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut inptr00: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut inptr01: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut inptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut inptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    /* copy these pointers into registers if possible */
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
    let mut Crrtab: *mut i32 = (*upsample).Cr_r_tab;
    let mut Cbbtab: *mut i32 = (*upsample).Cb_b_tab;
    let mut Crgtab: *mut crate::jmorecfg_h::INT32 = (*upsample).Cr_g_tab;
    let mut Cbgtab: *mut crate::jmorecfg_h::INT32 = (*upsample).Cb_g_tab;
    inptr00 = *(*input_buf.offset(0 as i32 as isize))
        .offset(in_row_group_ctr.wrapping_mul(2 as i32 as u32) as isize);
    inptr01 = *(*input_buf.offset(0 as i32 as isize)).offset(
        in_row_group_ctr
            .wrapping_mul(2 as i32 as u32)
            .wrapping_add(1 as i32 as u32) as isize,
    );
    inptr1 = *(*input_buf.offset(1 as i32 as isize)).offset(in_row_group_ctr as isize);
    inptr2 = *(*input_buf.offset(2 as i32 as isize)).offset(in_row_group_ctr as isize);
    outptr0 = *output_buf.offset(0 as i32 as isize);
    outptr1 = *output_buf.offset(1 as i32 as isize);
    /* Loop for each group of output pixels */
    col = (*cinfo).output_width >> 1 as i32;
    while col > 0 as i32 as u32 {
        /* Do the chroma part of the calculation */
        let fresh4 = inptr1;
        inptr1 = inptr1.offset(1);
        cb = *fresh4 as i32;
        let fresh5 = inptr2;
        inptr2 = inptr2.offset(1);
        cr = *fresh5 as i32;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16 as i32) as i32;
        cblue = *Cbbtab.offset(cb as isize);
        /* Fetch 4 Y values and emit 4 pixels */
        let fresh6 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh6 as i32;
        *outptr0.offset(0 as i32 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(1 as i32 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(2 as i32 as isize) = *range_limit.offset((y + cblue) as isize);
        outptr0 = outptr0.offset(3 as i32 as isize);
        let fresh7 = inptr00;
        inptr00 = inptr00.offset(1);
        y = *fresh7 as i32;
        *outptr0.offset(0 as i32 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(1 as i32 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(2 as i32 as isize) = *range_limit.offset((y + cblue) as isize);
        outptr0 = outptr0.offset(3 as i32 as isize);
        let fresh8 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh8 as i32;
        *outptr1.offset(0 as i32 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(1 as i32 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(2 as i32 as isize) = *range_limit.offset((y + cblue) as isize);
        outptr1 = outptr1.offset(3 as i32 as isize);
        let fresh9 = inptr01;
        inptr01 = inptr01.offset(1);
        y = *fresh9 as i32;
        *outptr1.offset(0 as i32 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(1 as i32 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(2 as i32 as isize) = *range_limit.offset((y + cblue) as isize);
        outptr1 = outptr1.offset(3 as i32 as isize);
        col = col.wrapping_sub(1)
    }
    /* If image width is odd, do the last output column separately */
    if (*cinfo).output_width & 1 as i32 as u32 != 0 {
        cb = *inptr1 as i32;
        cr = *inptr2 as i32;
        cred = *Crrtab.offset(cr as isize);
        cgreen = (*Cbgtab.offset(cb as isize) + *Crgtab.offset(cr as isize) >> 16 as i32) as i32;
        cblue = *Cbbtab.offset(cb as isize);
        y = *inptr00 as i32;
        *outptr0.offset(0 as i32 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr0.offset(1 as i32 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr0.offset(2 as i32 as isize) = *range_limit.offset((y + cblue) as isize);
        y = *inptr01 as i32;
        *outptr1.offset(0 as i32 as isize) = *range_limit.offset((y + cred) as isize);
        *outptr1.offset(1 as i32 as isize) = *range_limit.offset((y + cgreen) as isize);
        *outptr1.offset(2 as i32 as isize) = *range_limit.offset((y + cblue) as isize)
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
/*
 * Module initialization routine for merged upsampling/color conversion.
 *
 * NB: this is called under the conditions determined by use_merged_upsample()
 * in jdmaster.c.  That routine MUST correspond to the actual capabilities
 * of this module; no safety checks are made here.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_merged_upsampler(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    let mut upsample: my_upsample_ptr = 0 as *mut my_upsampler;
    upsample = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        1 as i32,
        ::std::mem::size_of::<my_upsampler>() as libc::c_ulong,
    ) as my_upsample_ptr;
    (*cinfo).upsample = upsample as *mut crate::jpegint_h::jpeg_upsampler;
    (*upsample).pub_0.start_pass = Some(
        start_pass_merged_upsample
            as unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> (),
    );
    (*upsample).pub_0.need_context_rows = 0 as i32;
    (*upsample).out_row_width = (*cinfo)
        .output_width
        .wrapping_mul((*cinfo).out_color_components as u32);
    if (*cinfo).max_v_samp_factor == 2 as i32 {
        (*upsample).pub_0.upsample = Some(
            merged_2v_upsample
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
        (*upsample).upmethod = Some(
            h2v2_merged_upsample
                as unsafe extern "C" fn(
                    _: crate::jpeglib_h::j_decompress_ptr,
                    _: crate::jpeglib_h::JSAMPIMAGE,
                    _: crate::jmorecfg_h::JDIMENSION,
                    _: crate::jpeglib_h::JSAMPARRAY,
                ) -> (),
        );
        /* Allocate a spare row buffer */
        (*upsample).spare_row = Some(
            (*(*cinfo).mem)
                .alloc_large
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            1 as i32,
            ((*upsample).out_row_width as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::jmorecfg_h::JSAMPLE>() as libc::c_ulong),
        ) as crate::jpeglib_h::JSAMPROW
    } else {
        (*upsample).pub_0.upsample = Some(
            merged_1v_upsample
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
        (*upsample).upmethod = Some(
            h2v1_merged_upsample
                as unsafe extern "C" fn(
                    _: crate::jpeglib_h::j_decompress_ptr,
                    _: crate::jpeglib_h::JSAMPIMAGE,
                    _: crate::jmorecfg_h::JDIMENSION,
                    _: crate::jpeglib_h::JSAMPARRAY,
                ) -> (),
        );
        /* No spare row needed */
        (*upsample).spare_row = 0 as crate::jpeglib_h::JSAMPROW
    }
    build_ycc_rgb_table(cinfo);
}
/* UPSAMPLE_MERGING_SUPPORTED */
