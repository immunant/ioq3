/* INCOMPLETE_TYPES_BROKEN */

/* Suppress undefined-structure complaints if necessary. */
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
pub use crate::src::jpeg_8c::jcarith::jinit_arith_encoder;
pub use crate::src::jpeg_8c::jccoefct::jinit_c_coef_controller;
pub use crate::src::jpeg_8c::jccolor::jinit_color_converter;
pub use crate::src::jpeg_8c::jcdctmgr::jinit_forward_dct;
pub use crate::src::jpeg_8c::jchuff::jinit_huff_encoder;
pub use crate::src::jpeg_8c::jcmainct::jinit_c_main_controller;
pub use crate::src::jpeg_8c::jcmarker::jinit_marker_writer;
pub use crate::src::jpeg_8c::jcmaster::jinit_c_master_control;
pub use crate::src::jpeg_8c::jcprepct::jinit_c_prep_controller;
pub use crate::src::jpeg_8c::jcsample::jinit_downsampler;
/*
 * jcinit.c
 *
 * Copyright (C) 1991-1997, Thomas G. Lane.
 * This file is part of the Independent JPEG Group's software.
 * For conditions of distribution and use, see the accompanying README file.
 *
 * This file contains initialization logic for the JPEG compressor.
 * This routine is in charge of selecting the modules to be executed and
 * making an initialization call to each one.
 *
 * Logically, this code belongs in jcmaster.c.  It's split out because
 * linking this routine implies linking the entire compression library.
 * For a transcoding-only application, we want to be able to use jcmaster.c
 * without linking in the whole library.
 */
/*
 * Master selection of compression modules.
 * This is done once at the start of processing an image.  We determine
 * which modules will be used and give them appropriate initialization calls.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_compress_master(mut cinfo: crate::jpeglib_h::j_compress_ptr) {
    /* Initialize master control (includes parameter checking/processing) */
    crate::src::jpeg_8c::jcmaster::jinit_c_master_control(
        cinfo as *mut crate::jpeglib_h::jpeg_compress_struct,
        0 as i32,
    );
    /* Preprocessing */
    if (*cinfo).raw_data_in == 0 {
        crate::src::jpeg_8c::jccolor::jinit_color_converter(
            cinfo as *mut crate::jpeglib_h::jpeg_compress_struct,
        );
        crate::src::jpeg_8c::jcsample::jinit_downsampler(
            cinfo as *mut crate::jpeglib_h::jpeg_compress_struct,
        );
        crate::src::jpeg_8c::jcprepct::jinit_c_prep_controller(
            cinfo as *mut crate::jpeglib_h::jpeg_compress_struct,
            0 as i32,
        );
    }
    /* Forward DCT */
    crate::src::jpeg_8c::jcdctmgr::jinit_forward_dct(
        cinfo as *mut crate::jpeglib_h::jpeg_compress_struct,
    );
    /* Entropy encoding: either Huffman or arithmetic coding. */
    if (*cinfo).arith_code != 0 {
        crate::src::jpeg_8c::jcarith::jinit_arith_encoder(
            cinfo as *mut crate::jpeglib_h::jpeg_compress_struct,
        );
    } else {
        crate::src::jpeg_8c::jchuff::jinit_huff_encoder(
            cinfo as *mut crate::jpeglib_h::jpeg_compress_struct,
        );
    }
    /* Need a full-image coefficient buffer in any multi-pass mode. */
    crate::src::jpeg_8c::jccoefct::jinit_c_coef_controller(
        cinfo as *mut crate::jpeglib_h::jpeg_compress_struct,
        ((*cinfo).num_scans > 1 as i32 || (*cinfo).optimize_coding != 0) as i32,
    );
    crate::src::jpeg_8c::jcmainct::jinit_c_main_controller(
        cinfo as *mut crate::jpeglib_h::jpeg_compress_struct,
        0 as i32,
    );
    crate::src::jpeg_8c::jcmarker::jinit_marker_writer(
        cinfo as *mut crate::jpeglib_h::jpeg_compress_struct,
    );
    /* We can now tell the memory manager to allocate virtual arrays. */
    Some(
        (*(*cinfo).mem)
            .realize_virt_arrays
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    /* Write the datastream header (SOI) immediately.
     * Frame and scan headers are postponed till later.
     * This lets application insert special markers after the SOI.
     */
    Some(
        (*(*cinfo).marker)
            .write_file_header
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo);
}
