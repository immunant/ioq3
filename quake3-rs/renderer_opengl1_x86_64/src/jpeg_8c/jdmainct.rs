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

pub type my_main_ptr = *mut my_main_controller;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_main_controller {
    pub pub_0: jpeg_d_main_controller,
    pub buffer: [JSAMPARRAY; 10],
    pub buffer_full: boolean,
    pub rowgroup_ctr: JDIMENSION,
    pub xbuffer: [JSAMPIMAGE; 2],
    pub whichptr: i32,
    pub context_state: i32,
    pub rowgroups_avail: JDIMENSION,
    pub iMCU_row_ctr: JDIMENSION,
}

unsafe extern "C" fn alloc_funny_pointers(mut cinfo: j_decompress_ptr)
/* Allocate space for the funny pointer lists.
 * This is done only once, not once per pass.
 */
{
    let mut main_ptr: my_main_ptr = (*cinfo).main as my_main_ptr;
    let mut ci: i32 = 0;
    let mut rgroup: i32 = 0;
    let mut M: i32 = (*cinfo).min_DCT_v_scaled_size;
    let mut compptr: *mut jpeg_component_info = std::ptr::null_mut();
    let mut xbuf: JSAMPARRAY = std::ptr::null_mut();
    /* Get top-level space for component array pointers.
     * We alloc both arrays with one call to save a few cycles.
     */
    (*main_ptr).xbuffer[0 as i32 as usize] = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        1 as i32,
        (((*cinfo).num_components * 2 as i32) as usize)
            .wrapping_mul(::std::mem::size_of::<JSAMPARRAY>() as usize),
    ) as JSAMPIMAGE; /* height of a row group of component */
    (*main_ptr).xbuffer[1 as i32 as usize] =
        (*main_ptr).xbuffer[0 as i32 as usize].offset((*cinfo).num_components as isize);
    ci = 0 as i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        rgroup = (*compptr).v_samp_factor * (*compptr).DCT_v_scaled_size
            / (*cinfo).min_DCT_v_scaled_size;
        /* Get space for pointer lists --- M+4 row groups in each list.
         * We alloc both pointer lists with one call to save a few cycles.
         */
        xbuf = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            1 as i32,
            ((2 as i32 * (rgroup * (M + 4 as i32))) as usize)
                .wrapping_mul(::std::mem::size_of::<JSAMPROW>() as usize),
        ) as JSAMPARRAY; /* want one row group at negative offsets */
        xbuf = xbuf.offset(rgroup as isize);
        let ref mut fresh0 = *(*main_ptr).xbuffer[0 as i32 as usize].offset(ci as isize);
        *fresh0 = xbuf;
        xbuf = xbuf.offset((rgroup * (M + 4 as i32)) as isize);
        let ref mut fresh1 = *(*main_ptr).xbuffer[1 as i32 as usize].offset(ci as isize);
        *fresh1 = xbuf;
        ci += 1;
        compptr = compptr.offset(1)
    }
}

unsafe extern "C" fn make_funny_pointers(mut cinfo: j_decompress_ptr)
/* Create the funny pointer lists discussed in the comments above.
 * The actual workspace is already allocated (in main_ptr->buffer),
 * and the space for the pointer lists is allocated too.
 * This routine just fills in the curiously ordered lists.
 * This will be repeated at the beginning of each pass.
 */
{
    let mut main_ptr: my_main_ptr = (*cinfo).main as my_main_ptr; /* height of a row group of component */
    let mut ci: i32 = 0;
    let mut i: i32 = 0;
    let mut rgroup: i32 = 0;
    let mut M: i32 = (*cinfo).min_DCT_v_scaled_size;
    let mut compptr: *mut jpeg_component_info = std::ptr::null_mut();
    let mut buf: JSAMPARRAY = std::ptr::null_mut();
    let mut xbuf0: JSAMPARRAY = std::ptr::null_mut();
    let mut xbuf1: JSAMPARRAY = std::ptr::null_mut();
    ci = 0 as i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        rgroup = (*compptr).v_samp_factor * (*compptr).DCT_v_scaled_size
            / (*cinfo).min_DCT_v_scaled_size;
        xbuf0 = *(*main_ptr).xbuffer[0 as i32 as usize].offset(ci as isize);
        xbuf1 = *(*main_ptr).xbuffer[1 as i32 as usize].offset(ci as isize);
        /* First copy the workspace pointers as-is */
        buf = (*main_ptr).buffer[ci as usize];
        i = 0 as i32;
        while i < rgroup * (M + 2 as i32) {
            let ref mut fresh2 = *xbuf1.offset(i as isize);
            *fresh2 = *buf.offset(i as isize);
            let ref mut fresh3 = *xbuf0.offset(i as isize);
            *fresh3 = *fresh2;
            i += 1
        }
        /* In the second list, put the last four row groups in swapped order */
        i = 0 as i32;
        while i < rgroup * 2 as i32 {
            let ref mut fresh4 = *xbuf1.offset((rgroup * (M - 2 as i32) + i) as isize);
            *fresh4 = *buf.offset((rgroup * M + i) as isize);
            let ref mut fresh5 = *xbuf1.offset((rgroup * M + i) as isize);
            *fresh5 = *buf.offset((rgroup * (M - 2 as i32) + i) as isize);
            i += 1
        }
        /* The wraparound pointers at top and bottom will be filled later
         * (see set_wraparound_pointers, below).  Initially we want the "above"
         * pointers to duplicate the first actual data line.  This only needs
         * to happen in xbuffer[0].
         */
        i = 0 as i32;
        while i < rgroup {
            let ref mut fresh6 = *xbuf0.offset((i - rgroup) as isize);
            *fresh6 = *xbuf0.offset(0 as i32 as isize);
            i += 1
        }
        ci += 1;
        compptr = compptr.offset(1)
    }
}

unsafe extern "C" fn set_wraparound_pointers(mut cinfo: j_decompress_ptr)
/* Set up the "wraparound" pointers at top and bottom of the pointer lists.
 * This changes the pointer list state from top-of-image to the normal state.
 */
{
    let mut main_ptr: my_main_ptr = (*cinfo).main as my_main_ptr; /* height of a row group of component */
    let mut ci: i32 = 0;
    let mut i: i32 = 0;
    let mut rgroup: i32 = 0;
    let mut M: i32 = (*cinfo).min_DCT_v_scaled_size;
    let mut compptr: *mut jpeg_component_info = std::ptr::null_mut();
    let mut xbuf0: JSAMPARRAY = std::ptr::null_mut();
    let mut xbuf1: JSAMPARRAY = std::ptr::null_mut();
    ci = 0 as i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        rgroup = (*compptr).v_samp_factor * (*compptr).DCT_v_scaled_size
            / (*cinfo).min_DCT_v_scaled_size;
        xbuf0 = *(*main_ptr).xbuffer[0 as i32 as usize].offset(ci as isize);
        xbuf1 = *(*main_ptr).xbuffer[1 as i32 as usize].offset(ci as isize);
        i = 0 as i32;
        while i < rgroup {
            let ref mut fresh7 = *xbuf0.offset((i - rgroup) as isize);
            *fresh7 = *xbuf0.offset((rgroup * (M + 1 as i32) + i) as isize);
            let ref mut fresh8 = *xbuf1.offset((i - rgroup) as isize);
            *fresh8 = *xbuf1.offset((rgroup * (M + 1 as i32) + i) as isize);
            let ref mut fresh9 = *xbuf0.offset((rgroup * (M + 2 as i32) + i) as isize);
            *fresh9 = *xbuf0.offset(i as isize);
            let ref mut fresh10 = *xbuf1.offset((rgroup * (M + 2 as i32) + i) as isize);
            *fresh10 = *xbuf1.offset(i as isize);
            i += 1
        }
        ci += 1;
        compptr = compptr.offset(1)
    }
}

unsafe extern "C" fn set_bottom_pointers(mut cinfo: j_decompress_ptr)
/* Change the pointer lists to duplicate the last sample row at the bottom
 * of the image.  whichptr indicates which xbuffer holds the final iMCU row.
 * Also sets rowgroups_avail to indicate number of nondummy row groups in row.
 */
{
    let mut main_ptr: my_main_ptr = (*cinfo).main as my_main_ptr;
    let mut ci: i32 = 0;
    let mut i: i32 = 0;
    let mut rgroup: i32 = 0;
    let mut iMCUheight: i32 = 0;
    let mut rows_left: i32 = 0;
    let mut compptr: *mut jpeg_component_info = std::ptr::null_mut();
    let mut xbuf: JSAMPARRAY = std::ptr::null_mut();
    ci = 0 as i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        /* Count sample rows in one iMCU row and in one row group */
        iMCUheight = (*compptr).v_samp_factor * (*compptr).DCT_v_scaled_size;
        rgroup = iMCUheight / (*cinfo).min_DCT_v_scaled_size;
        /* Count nondummy sample rows remaining for this component */
        rows_left = (*compptr)
            .downsampled_height
            .wrapping_rem(iMCUheight as JDIMENSION) as i32;
        if rows_left == 0 as i32 {
            rows_left = iMCUheight
        }
        /* Count nondummy row groups.  Should get same answer for each component,
         * so we need only do it once.
         */
        if ci == 0 as i32 {
            (*main_ptr).rowgroups_avail = ((rows_left - 1 as i32) / rgroup + 1 as i32) as JDIMENSION
        }
        /* Duplicate the last real sample row rgroup*2 times; this pads out the
         * last partial rowgroup and ensures at least one full rowgroup of context.
         */
        xbuf = *(*main_ptr).xbuffer[(*main_ptr).whichptr as usize].offset(ci as isize);
        i = 0 as i32;
        while i < rgroup * 2 as i32 {
            let ref mut fresh11 = *xbuf.offset((rows_left + i) as isize);
            *fresh11 = *xbuf.offset((rows_left - 1 as i32) as isize);
            i += 1
        }
        ci += 1;
        compptr = compptr.offset(1)
    }
}
/*
 * Initialize for a processing pass.
 */

unsafe extern "C" fn start_pass_main(mut cinfo: j_decompress_ptr, mut pass_mode: J_BUF_MODE) {
    let mut main_ptr: my_main_ptr = (*cinfo).main as my_main_ptr; /* Create the xbuffer[] lists */
    match pass_mode as u32 {
        0 => {
            if (*(*cinfo).upsample).need_context_rows != 0 {
                (*main_ptr).pub_0.process_data = Some(
                    process_data_context_main
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: JSAMPARRAY,
                            _: *mut JDIMENSION,
                            _: JDIMENSION,
                        ) -> (),
                ); /* Read first iMCU row into xbuffer[0] */
                make_funny_pointers(cinfo);
                (*main_ptr).whichptr = 0 as i32;
                (*main_ptr).context_state = 0 as i32;
                (*main_ptr).iMCU_row_ctr = 0 as i32 as JDIMENSION
            } else {
                /* Simple case with no context needed */
                (*main_ptr).pub_0.process_data = Some(
                    process_data_simple_main
                        as unsafe extern "C" fn(
                            _: j_decompress_ptr,
                            _: JSAMPARRAY,
                            _: *mut JDIMENSION,
                            _: JDIMENSION,
                        ) -> (),
                )
            } /* Mark buffer empty */
            (*main_ptr).buffer_full = 0 as i32;
            (*main_ptr).rowgroup_ctr = 0 as i32 as JDIMENSION
        }
        2 => {
            /* For last pass of 2-pass quantization, just crank the postprocessor */
            (*main_ptr).pub_0.process_data = Some(
                process_data_crank_post
                    as unsafe extern "C" fn(
                        _: j_decompress_ptr,
                        _: JSAMPARRAY,
                        _: *mut JDIMENSION,
                        _: JDIMENSION,
                    ) -> (),
            )
        }
        _ => {
            (*(*cinfo).err).msg_code = JERR_BAD_BUFFER_MODE as i32;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
    };
}
/* feeding postponed row group */
/* Forward declarations */
/*
 * Process some data.
 * This handles the simple case where no context is required.
 */

unsafe extern "C" fn process_data_simple_main(
    mut cinfo: j_decompress_ptr,
    mut output_buf: JSAMPARRAY,
    mut out_row_ctr: *mut JDIMENSION,
    mut out_rows_avail: JDIMENSION,
) {
    let mut main_ptr: my_main_ptr = (*cinfo).main as my_main_ptr;
    let mut rowgroups_avail: JDIMENSION = 0;
    /* Read input data if we haven't filled the main buffer yet */
    if (*main_ptr).buffer_full == 0 {
        if Some(
            (*(*cinfo).coef)
                .decompress_data
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo, (*main_ptr).buffer.as_mut_ptr())
            == 0
        {
            return;
        }
        (*main_ptr).buffer_full = 1 as i32 /* suspension forced, can do nothing more */
        /* OK, we have an iMCU row to work with */
    }
    /* There are always min_DCT_scaled_size row groups in an iMCU row. */
    rowgroups_avail = (*cinfo).min_DCT_v_scaled_size as JDIMENSION;
    /* Note: at the bottom of the image, we may pass extra garbage row groups
     * to the postprocessor.  The postprocessor has to check for bottom
     * of image anyway (at row resolution), so no point in us doing it too.
     */
    /* Feed the postprocessor */
    Some(
        (*(*cinfo).post)
            .post_process_data
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo,
        (*main_ptr).buffer.as_mut_ptr(),
        &mut (*main_ptr).rowgroup_ctr,
        rowgroups_avail,
        output_buf,
        out_row_ctr,
        out_rows_avail,
    );
    /* Has postprocessor consumed all the data yet? If so, mark buffer empty */
    if (*main_ptr).rowgroup_ctr >= rowgroups_avail {
        (*main_ptr).buffer_full = 0 as i32;
        (*main_ptr).rowgroup_ctr = 0 as i32 as JDIMENSION
    };
}
/*
 * Process some data.
 * This handles the case where context rows must be provided.
 */

unsafe extern "C" fn process_data_context_main(
    mut cinfo: j_decompress_ptr,
    mut output_buf: JSAMPARRAY,
    mut out_row_ctr: *mut JDIMENSION,
    mut out_rows_avail: JDIMENSION,
) {
    let mut main_ptr: my_main_ptr = (*cinfo).main as my_main_ptr;
    /* Read input data if we haven't filled the main buffer yet */
    if (*main_ptr).buffer_full == 0 {
        if Some(
            (*(*cinfo).coef)
                .decompress_data
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo,
            (*main_ptr).xbuffer[(*main_ptr).whichptr as usize],
        ) == 0
        {
            return;
        } /* suspension forced, can do nothing more */
        /* count rows received */
        (*main_ptr).buffer_full = 1 as i32; /* OK, we have an iMCU row to work with */
        (*main_ptr).iMCU_row_ctr = (*main_ptr).iMCU_row_ctr.wrapping_add(1)
    }
    let mut current_block_26: u64;
    /* Postprocessor typically will not swallow all the input data it is handed
     * in one call (due to filling the output buffer first).  Must be prepared
     * to exit and restart.  This switch lets us keep track of how far we got.
     * Note that each case falls through to the next on successful completion.
     */
    match (*main_ptr).context_state {
        2 => {
            /* Call postprocessor using previously set pointers for postponed row */
            Some(
                (*(*cinfo).post)
                    .post_process_data
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo,
                (*main_ptr).xbuffer[(*main_ptr).whichptr as usize],
                &mut (*main_ptr).rowgroup_ctr,
                (*main_ptr).rowgroups_avail,
                output_buf,
                out_row_ctr,
                out_rows_avail,
            ); /* Need to suspend */
            if (*main_ptr).rowgroup_ctr < (*main_ptr).rowgroups_avail {
                return;
            } /* Postprocessor exactly filled output buf */
            (*main_ptr).context_state = 0 as i32;
            if *out_row_ctr >= out_rows_avail {
                return;
            }
            current_block_26 = 3520159027929658754;
        }
        0 => {
            current_block_26 = 3520159027929658754;
        }
        1 => {
            current_block_26 = 5539595041850004771;
        }
        _ => {
            current_block_26 = 16203760046146113240;
        }
    }
    match current_block_26 {
        3520159027929658754 =>
        /*FALLTHROUGH*/
        /* Prepare to process first M-1 row groups of this iMCU row */
        {
            (*main_ptr).rowgroup_ctr = 0 as i32 as JDIMENSION;
            (*main_ptr).rowgroups_avail = ((*cinfo).min_DCT_v_scaled_size - 1 as i32) as JDIMENSION;
            /* Check for bottom of image: if so, tweak pointers to "duplicate"
             * the last sample row, and adjust rowgroups_avail to ignore padding rows.
             */
            if (*main_ptr).iMCU_row_ctr == (*cinfo).total_iMCU_rows {
                set_bottom_pointers(cinfo);
            }
            (*main_ptr).context_state = 1 as i32;
            current_block_26 = 5539595041850004771;
        }
        _ => {}
    }
    match current_block_26 {
        5539595041850004771 =>
        /*FALLTHROUGH*/
        /* Call postprocessor using previously set pointers */
        {
            Some(
                (*(*cinfo).post)
                    .post_process_data
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo,
                (*main_ptr).xbuffer[(*main_ptr).whichptr as usize],
                &mut (*main_ptr).rowgroup_ctr,
                (*main_ptr).rowgroups_avail,
                output_buf,
                out_row_ctr,
                out_rows_avail,
            ); /* Need to suspend */
            if (*main_ptr).rowgroup_ctr < (*main_ptr).rowgroups_avail {
                return;
            }
            /* After the first iMCU, change wraparound pointers to normal state */
            if (*main_ptr).iMCU_row_ctr == 1 as i32 as u32 {
                set_wraparound_pointers(cinfo);
            }
            /* Prepare to load new iMCU row using other xbuffer list */
            (*main_ptr).whichptr ^= 1 as i32; /* 0=>1 or 1=>0 */
            (*main_ptr).buffer_full = 0 as i32;
            /* Still need to process last row group of this iMCU row, */
            /* which is saved at index M+1 of the other xbuffer */
            (*main_ptr).rowgroup_ctr = ((*cinfo).min_DCT_v_scaled_size + 1 as i32) as JDIMENSION;
            (*main_ptr).rowgroups_avail = ((*cinfo).min_DCT_v_scaled_size + 2 as i32) as JDIMENSION;
            (*main_ptr).context_state = 2 as i32
        }
        _ => {}
    };
}
/*
 * Process some data.
 * Final pass of two-pass quantization: just call the postprocessor.
 * Source data will be the postprocessor controller's internal buffer.
 */

unsafe extern "C" fn process_data_crank_post(
    mut cinfo: j_decompress_ptr,
    mut output_buf: JSAMPARRAY,
    mut out_row_ctr: *mut JDIMENSION,
    mut out_rows_avail: JDIMENSION,
) {
    Some(
        (*(*cinfo).post)
            .post_process_data
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo,
        std::ptr::null_mut() as JSAMPIMAGE,
        std::ptr::null_mut() as *mut JDIMENSION,
        0 as i32 as JDIMENSION,
        output_buf,
        out_row_ctr,
        out_rows_avail,
    );
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
 * Initialize main buffer controller.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_d_main_controller(
    mut cinfo: j_decompress_ptr,
    mut need_full_buffer: boolean,
) {
    let mut main_ptr: my_main_ptr = std::ptr::null_mut();
    let mut ci: i32 = 0;
    let mut rgroup: i32 = 0;
    let mut ngroups: i32 = 0;
    let mut compptr: *mut jpeg_component_info = std::ptr::null_mut();
    main_ptr = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        1 as i32,
        ::std::mem::size_of::<my_main_controller>() as usize,
    ) as my_main_ptr;
    (*cinfo).main = main_ptr as *mut jpeg_d_main_controller;
    (*main_ptr).pub_0.start_pass =
        Some(start_pass_main as unsafe extern "C" fn(_: j_decompress_ptr, _: J_BUF_MODE) -> ());
    if need_full_buffer != 0 {
        /* shouldn't happen */
        (*(*cinfo).err).msg_code = JERR_BAD_BUFFER_MODE as i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* Allocate the workspace.
     * ngroups is the number of row groups we need.
     */
    if (*(*cinfo).upsample).need_context_rows != 0 {
        if (*cinfo).min_DCT_v_scaled_size < 2 as i32 {
            /* unsupported, see comments above */
            (*(*cinfo).err).msg_code = JERR_NOTIMPL as i32; /* Alloc space for xbuffer[] lists */
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr); /* height of a row group of component */
        }
        alloc_funny_pointers(cinfo);
        ngroups = (*cinfo).min_DCT_v_scaled_size + 2 as i32
    } else {
        ngroups = (*cinfo).min_DCT_v_scaled_size
    }
    ci = 0 as i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        rgroup = (*compptr).v_samp_factor * (*compptr).DCT_v_scaled_size
            / (*cinfo).min_DCT_v_scaled_size;
        (*main_ptr).buffer[ci as usize] = Some(
            (*(*cinfo).mem)
                .alloc_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            1 as i32,
            (*compptr)
                .width_in_blocks
                .wrapping_mul((*compptr).DCT_h_scaled_size as u32),
            (rgroup * ngroups) as JDIMENSION,
        );
        ci += 1;
        compptr = compptr.offset(1)
    }
}
