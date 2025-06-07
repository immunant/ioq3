use ::libc;

pub use crate::src::jpeg_8c::jerror::C2RustUnnamed_1;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::e_status;
pub use crate::src::qcommon::q_shared::h_dontcare;
pub use crate::src::qcommon::q_shared::h_high;
pub use crate::src::qcommon::q_shared::h_low;
pub use crate::src::qcommon::q_shared::ha_pref;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;
pub use crate::src::qcommon::q_shared::FMV_EOF;
pub use crate::src::qcommon::q_shared::FMV_IDLE;
pub use crate::src::qcommon::q_shared::FMV_ID_BLT;
pub use crate::src::qcommon::q_shared::FMV_ID_IDLE;
pub use crate::src::qcommon::q_shared::FMV_ID_WAIT;
pub use crate::src::qcommon::q_shared::FMV_LOOPED;
pub use crate::src::qcommon::q_shared::FMV_PLAY;
pub use crate::src::qcommon::q_shared::PRINT_ALL;
pub use crate::src::qcommon::q_shared::PRINT_DEVELOPER;
pub use crate::src::qcommon::q_shared::PRINT_ERROR;
pub use crate::src::qcommon::q_shared::PRINT_WARNING;
pub use crate::stddef_h::size_t;
pub use crate::stdlib::__jmp_buf;
pub use crate::stdlib::__jmp_buf_tag;
pub use crate::stdlib::__sigset_t;
pub use crate::stdlib::_setjmp;
pub use crate::stdlib::jmp_buf;
pub use crate::stdlib::longjmp;
pub use crate::tr_public_h::refimport_t;

pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpegint_h::forward_DCT_ptr;
pub use crate::jpegint_h::inverse_DCT_method_ptr;
pub use crate::jpegint_h::jpeg_c_coef_controller;
pub use crate::jpegint_h::jpeg_c_main_controller;
pub use crate::jpegint_h::jpeg_c_prep_controller;
pub use crate::jpegint_h::jpeg_color_converter;
pub use crate::jpegint_h::jpeg_color_deconverter;
pub use crate::jpegint_h::jpeg_color_quantizer;
pub use crate::jpegint_h::jpeg_comp_master;
pub use crate::jpegint_h::jpeg_d_coef_controller;
pub use crate::jpegint_h::jpeg_d_main_controller;
pub use crate::jpegint_h::jpeg_d_post_controller;
pub use crate::jpegint_h::jpeg_decomp_master;
pub use crate::jpegint_h::jpeg_downsampler;
pub use crate::jpegint_h::jpeg_entropy_decoder;
pub use crate::jpegint_h::jpeg_entropy_encoder;
pub use crate::jpegint_h::jpeg_forward_dct;
pub use crate::jpegint_h::jpeg_input_controller;
pub use crate::jpegint_h::jpeg_inverse_dct;
pub use crate::jpegint_h::jpeg_marker_reader;
pub use crate::jpegint_h::jpeg_marker_writer;
pub use crate::jpegint_h::jpeg_upsampler;
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
pub use crate::jpegint_h::J_BUF_MODE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_compress_ptr;
pub use crate::jpeglib_h::j_decompress_ptr;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_compress_struct;
pub use crate::jpeglib_h::jpeg_decompress_struct;
pub use crate::jpeglib_h::jpeg_destination_mgr;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_marker_parser_method;
pub use crate::jpeglib_h::jpeg_marker_struct;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_saved_marker_ptr;
pub use crate::jpeglib_h::jpeg_scan_info;
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
pub use crate::src::jpeg_8c::jcapimin::jpeg_CreateCompress;
pub use crate::src::jpeg_8c::jcapimin::jpeg_destroy_compress;
pub use crate::src::jpeg_8c::jcapimin::jpeg_finish_compress;
pub use crate::src::jpeg_8c::jcapistd::jpeg_start_compress;
pub use crate::src::jpeg_8c::jcapistd::jpeg_write_scanlines;
pub use crate::src::jpeg_8c::jcparam::jpeg_set_defaults;
pub use crate::src::jpeg_8c::jcparam::jpeg_set_quality;
pub use crate::src::jpeg_8c::jdapimin::jpeg_CreateDecompress;
pub use crate::src::jpeg_8c::jdapimin::jpeg_destroy_decompress;
pub use crate::src::jpeg_8c::jdapimin::jpeg_finish_decompress;
pub use crate::src::jpeg_8c::jdapimin::jpeg_read_header;
pub use crate::src::jpeg_8c::jdapistd::jpeg_read_scanlines;
pub use crate::src::jpeg_8c::jdapistd::jpeg_start_decompress;
pub use crate::src::jpeg_8c::jdatasrc::jpeg_mem_src;
pub use crate::src::jpeg_8c::jerror::jpeg_std_error;

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_86 {
    pub b: *mut crate::src::qcommon::q_shared::byte,
    pub v: *mut libc::c_void,
}
/*
===========================================================================
Copyright (C) 1999-2005 Id Software, Inc.

This file is part of Quake III Arena source code.

Quake III Arena source code is free software; you can redistribute it
and/or modify it under the terms of the GNU General Public License as
published by the Free Software Foundation; either version 2 of the License,
or (at your option) any later version.

Quake III Arena source code is distributed in the hope that it will be
useful, but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Quake III Arena source code; if not, write to the Free Software
Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
===========================================================================
*/
/*
 * Include file for users of JPEG library.
 * You will need to have included system headers that define at least
 * the typedefs FILE and size_t before you can include jpeglib.h.
 * (stdio.h is sufficient on ANSI-conforming systems.)
 * You may also wish to include "jerror.h".
 */
/* Catching errors, as done in libjpeg's example.c */

pub type q_jpeg_error_mgr_t = q_jpeg_error_mgr_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct q_jpeg_error_mgr_s {
    pub pub_0: crate::jpeglib_h::jpeg_error_mgr,
    pub setjmp_buffer: crate::stdlib::jmp_buf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_destination_mgr {
    pub pub_0: crate::jpeglib_h::jpeg_destination_mgr,
    pub outfile: *mut crate::src::qcommon::q_shared::byte,
    pub size: libc::c_int,
}

pub type my_dest_ptr = *mut my_destination_mgr;

unsafe extern "C" fn R_JPGErrorExit(mut cinfo: crate::jpeglib_h::j_common_ptr) {
    let mut buffer: [libc::c_char; 200] = [0; 200];
    /* "public" fields */
    /* for return to caller */
    /* cinfo->err really points to a q_jpeg_error_mgr_s struct, so coerce pointer */
    let mut jerr: *mut q_jpeg_error_mgr_t = (*cinfo).err as *mut q_jpeg_error_mgr_t;
    Some(
        (*(*cinfo).err)
            .format_message
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo, buffer.as_mut_ptr());
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as libc::c_int,
        b"Error: %s\x00" as *const u8 as *const libc::c_char,
        buffer.as_mut_ptr(),
    );
    /* Return control to the setjmp point */
    crate::stdlib::longjmp((*jerr).setjmp_buffer.as_mut_ptr(), 1 as libc::c_int);
}

unsafe extern "C" fn R_JPGOutputMessage(mut cinfo: crate::jpeglib_h::j_common_ptr) {
    let mut buffer: [libc::c_char; 200] = [0; 200];
    /* Create the message */
    Some(
        (*(*cinfo).err)
            .format_message
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo, buffer.as_mut_ptr());
    /* Send it to stderr, adding a newline */
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as libc::c_int,
        b"%s\n\x00" as *const u8 as *const libc::c_char,
        buffer.as_mut_ptr(),
    );
}
/*
===========================================================================
Copyright (C) 1999-2005 Id Software, Inc.

This file is part of Quake III Arena source code.

Quake III Arena source code is free software; you can redistribute it
and/or modify it under the terms of the GNU General Public License as
published by the Free Software Foundation; either version 2 of the License,
or (at your option) any later version.

Quake III Arena source code is distributed in the hope that it will be
useful, but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Quake III Arena source code; if not, write to the Free Software
Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
===========================================================================
*/
// for color, lightmap, diffuse, and specular
// normals are swizzled, deluxe are not
// game path, including extension
// source image
// after power of two and picmip but not including clamp to MAX_TEXTURE_SIZE
// gl texture binding
// for texture usage in frame statistics
// only needed for voodoo2
// any change in the LIGHTMAP_* defines here MUST be reflected in
// R_FindShader() in tr_bsp.c
// shader is for 2D rendering
// pre-lit triangle models
// outside of TR since it shouldn't be cleared during ref re-init
// These variables should live inside glConfig but can't because of
// compatibility issues to the original ID vms.  If you release a stand-alone
// game and your mod uses tr_types.h from this build you can safely move them
// to the glconfig_t struct.
//
// cvars
//
// number of desired stencil bits
// number of desired depth bits
// number of desired color bits, only relevant for fullscreen
// number of desired texture bits
// 0 = use framebuffer depth
// 16 = use 16-bit textures
// 32 = use 32-bit textures
// all else = error
// video mode
// overrides hardware gamma capabilities
// global enable/disable of OpenGL extensions
// these control use of specific extensions
// font stuff
/*
=============================================================

IMAGE LOADERS

=============================================================
*/
#[no_mangle]

pub unsafe extern "C" fn R_LoadJPG(
    mut filename: *const libc::c_char,
    mut pic: *mut *mut libc::c_uchar,
    mut width: *mut libc::c_int,
    mut height: *mut libc::c_int,
) {
    /* This struct contains the JPEG decompression parameters and pointers to
     * working space (which is allocated as needed by the JPEG library).
     */
    let mut cinfo: crate::jpeglib_h::jpeg_decompress_struct = {
        let mut init = crate::jpeglib_h::jpeg_decompress_struct {
            err: 0 as *mut crate::jpeglib_h::jpeg_error_mgr,
            mem: 0 as *mut crate::jpeglib_h::jpeg_memory_mgr,
            progress: 0 as *mut crate::jpeglib_h::jpeg_progress_mgr,
            client_data: 0 as *mut libc::c_void,
            is_decompressor: 0,
            global_state: 0,
            src: 0 as *mut crate::jpeglib_h::jpeg_source_mgr,
            image_width: 0,
            image_height: 0,
            num_components: 0,
            jpeg_color_space: crate::jpeglib_h::JCS_UNKNOWN,
            out_color_space: crate::jpeglib_h::JCS_UNKNOWN,
            scale_num: 0,
            scale_denom: 0,
            output_gamma: 0.,
            buffered_image: 0,
            raw_data_out: 0,
            dct_method: crate::jpeglib_h::JDCT_ISLOW,
            do_fancy_upsampling: 0,
            do_block_smoothing: 0,
            quantize_colors: 0,
            dither_mode: crate::jpeglib_h::JDITHER_NONE,
            two_pass_quantize: 0,
            desired_number_of_colors: 0,
            enable_1pass_quant: 0,
            enable_external_quant: 0,
            enable_2pass_quant: 0,
            output_width: 0,
            output_height: 0,
            out_color_components: 0,
            output_components: 0,
            rec_outbuf_height: 0,
            actual_number_of_colors: 0,
            colormap: 0 as *mut crate::jpeglib_h::JSAMPROW,
            output_scanline: 0,
            input_scan_number: 0,
            input_iMCU_row: 0,
            output_scan_number: 0,
            output_iMCU_row: 0,
            coef_bits: 0 as *mut [libc::c_int; 64],
            quant_tbl_ptrs: [0 as *mut crate::jpeglib_h::JQUANT_TBL; 4],
            dc_huff_tbl_ptrs: [0 as *mut crate::jpeglib_h::JHUFF_TBL; 4],
            ac_huff_tbl_ptrs: [0 as *mut crate::jpeglib_h::JHUFF_TBL; 4],
            data_precision: 0,
            comp_info: 0 as *mut crate::jpeglib_h::jpeg_component_info,
            is_baseline: 0,
            progressive_mode: 0,
            arith_code: 0,
            arith_dc_L: [0; 16],
            arith_dc_U: [0; 16],
            arith_ac_K: [0; 16],
            restart_interval: 0,
            saw_JFIF_marker: 0,
            JFIF_major_version: 0,
            JFIF_minor_version: 0,
            density_unit: 0,
            X_density: 0,
            Y_density: 0,
            saw_Adobe_marker: 0,
            Adobe_transform: 0,
            CCIR601_sampling: 0,
            marker_list: 0 as *mut crate::jpeglib_h::jpeg_marker_struct,
            max_h_samp_factor: 0,
            max_v_samp_factor: 0,
            min_DCT_h_scaled_size: 0,
            min_DCT_v_scaled_size: 0,
            total_iMCU_rows: 0,
            sample_range_limit: 0 as *mut crate::jmorecfg_h::JSAMPLE,
            comps_in_scan: 0,
            cur_comp_info: [0 as *mut crate::jpeglib_h::jpeg_component_info; 4],
            MCUs_per_row: 0,
            MCU_rows_in_scan: 0,
            blocks_in_MCU: 0,
            MCU_membership: [0; 10],
            Ss: 0,
            Se: 0,
            Ah: 0,
            Al: 0,
            block_size: 0,
            natural_order: 0 as *const libc::c_int,
            lim_Se: 0,
            unread_marker: 0,
            master: 0 as *mut crate::jpegint_h::jpeg_decomp_master,
            main: 0 as *mut crate::jpegint_h::jpeg_d_main_controller,
            coef: 0 as *mut crate::jpegint_h::jpeg_d_coef_controller,
            post: 0 as *mut crate::jpegint_h::jpeg_d_post_controller,
            inputctl: 0 as *mut crate::jpegint_h::jpeg_input_controller,
            marker: 0 as *mut crate::jpegint_h::jpeg_marker_reader,
            entropy: 0 as *mut crate::jpegint_h::jpeg_entropy_decoder,
            idct: 0 as *mut crate::jpegint_h::jpeg_inverse_dct,
            upsample: 0 as *mut crate::jpegint_h::jpeg_upsampler,
            cconvert: 0 as *mut crate::jpegint_h::jpeg_color_deconverter,
            cquantize: 0 as *mut crate::jpegint_h::jpeg_color_quantizer,
        };
        init
    };
    /* We use our private extension JPEG error handler.
     * Note that this struct must live as long as the main JPEG parameter
     * struct, to avoid dangling-pointer problems.
     */
    /* This struct represents a JPEG error handler.  It is declared separately
     * because applications often want to supply a specialized error handler
     * (see the second half of this file for an example).  But here we just
     * take the easy way out and use the standard error handler, which will
     * print a message on stderr and call exit() if compression fails.
     * Note that this struct must live as long as the main JPEG parameter
     * struct, to avoid dangling-pointer problems.
     */
    let mut jerr: q_jpeg_error_mgr_t = q_jpeg_error_mgr_t {
        pub_0: crate::jpeglib_h::jpeg_error_mgr {
            error_exit: None,
            emit_message: None,
            output_message: None,
            format_message: None,
            reset_error_mgr: None,
            msg_code: 0,
            msg_parm: crate::jpeglib_h::C2RustUnnamed_0 { i: [0; 8] },
            trace_level: 0,
            num_warnings: 0,
            jpeg_message_table: 0 as *const *const libc::c_char,
            last_jpeg_message: 0,
            addon_message_table: 0 as *const *const libc::c_char,
            first_addon_message: 0,
            last_addon_message: 0,
        },
        setjmp_buffer: [crate::stdlib::__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: crate::stdlib::__sigset_t { __val: [0; 16] },
        }; 1],
    };
    /* More stuff */
    let mut buffer: crate::jpeglib_h::JSAMPARRAY = 0 as *mut crate::jpeglib_h::JSAMPROW; /* Output row buffer */
    let mut row_stride: libc::c_uint = 0; /* physical row width in output buffer */
    let mut pixelcount: libc::c_uint = 0;
    let mut memcount: libc::c_uint = 0;
    let mut sindex: libc::c_uint = 0;
    let mut dindex: libc::c_uint = 0;
    let mut out: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut len: libc::c_int = 0;
    let mut fbuffer: C2RustUnnamed_86 = C2RustUnnamed_86 {
        b: 0 as *mut crate::src::qcommon::q_shared::byte,
    };
    let mut buf: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    /* In this example we want to open the input file before doing anything else,
     * so that the setjmp() error recovery below can assume the file is open.
     * VERY IMPORTANT: use "b" option to fopen() if you are on a machine that
     * requires it in order to read binary files.
     */
    len = crate::src::renderergl1::tr_main::ri
        .FS_ReadFile
        .expect("non-null function pointer")(filename as *mut libc::c_char, &mut fbuffer.v)
        as libc::c_int;
    if fbuffer.b.is_null() || len < 0 as libc::c_int {
        return;
    }
    /* Step 1: allocate and initialize JPEG decompression object */
    /* We have to set up the error handler first, in case the initialization
     * step fails.  (Unlikely, but it could happen if you are out of memory.)
     * This routine fills in the contents of struct jerr, and returns jerr's
     * address which we place into the link field in cinfo.
     */
    cinfo.err = crate::src::jpeg_8c::jerror::jpeg_std_error(
        &mut jerr.pub_0 as *mut _ as *mut crate::jpeglib_h::jpeg_error_mgr,
    ) as *mut crate::jpeglib_h::jpeg_error_mgr;
    (*cinfo.err).error_exit =
        Some(R_JPGErrorExit as unsafe extern "C" fn(_: crate::jpeglib_h::j_common_ptr) -> ());
    (*cinfo.err).output_message =
        Some(R_JPGOutputMessage as unsafe extern "C" fn(_: crate::jpeglib_h::j_common_ptr) -> ());
    /* Establish the setjmp return context for R_JPGErrorExit to use. */
    if crate::stdlib::_setjmp(jerr.setjmp_buffer.as_mut_ptr()) != 0 {
        /* If we get here, the JPEG code has signaled an error.
         * We need to clean up the JPEG object, close the input file, and return.
         */
        crate::src::jpeg_8c::jdapimin::jpeg_destroy_decompress(
            &mut cinfo as *mut _ as *mut crate::jpeglib_h::jpeg_decompress_struct,
        );
        crate::src::renderergl1::tr_main::ri
            .FS_FreeFile
            .expect("non-null function pointer")(fbuffer.v);
        /* Append the filename to the error for easier debugging */
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as libc::c_int,
            b", loading file %s\n\x00" as *const u8 as *const libc::c_char,
            filename,
        );
        return;
    }
    /* Now we can initialize the JPEG decompression object. */
    crate::src::jpeg_8c::jdapimin::jpeg_CreateDecompress(
        &mut cinfo as *mut _ as *mut crate::jpeglib_h::jpeg_decompress_struct,
        80 as libc::c_int,
        ::std::mem::size_of::<crate::jpeglib_h::jpeg_decompress_struct>() as libc::c_ulong,
    );
    /* Step 2: specify data source (eg, a file) */
    crate::src::jpeg_8c::jdatasrc::jpeg_mem_src(
        &mut cinfo as *mut _ as *mut crate::jpeglib_h::jpeg_decompress_struct,
        fbuffer.b,
        len as libc::c_ulong,
    );
    /* Step 3: read file parameters with jpeg_read_header() */
    crate::src::jpeg_8c::jdapimin::jpeg_read_header(
        &mut cinfo as *mut _ as *mut crate::jpeglib_h::jpeg_decompress_struct,
        1 as libc::c_int,
    );
    /* We can ignore the return value from jpeg_read_header since
     *   (a) suspension is not possible with the stdio data source, and
     *   (b) we passed TRUE to reject a tables-only JPEG file as an error.
     * See libjpeg.doc for more info.
     */
    /* Step 4: set parameters for decompression */
    /*
     * Make sure it always converts images to RGB color space. This will
     * automatically convert 8-bit greyscale images to RGB as well.
     */
    cinfo.out_color_space = crate::jpeglib_h::JCS_RGB;
    /* Step 5: Start decompressor */
    crate::src::jpeg_8c::jdapistd::jpeg_start_decompress(
        &mut cinfo as *mut _ as *mut crate::jpeglib_h::jpeg_decompress_struct,
    );
    /* We can ignore the return value since suspension is not possible
     * with the stdio data source.
     */
    /* We may need to do some setup of our own at this point before reading
     * the data.  After jpeg_start_decompress() we have the correct scaled
     * output image dimensions available, as well as the output colormap
     * if we asked for color quantization.
     * In this example, we need to make an output work buffer of the right size.
     */
    /* JSAMPLEs per row in output buffer */
    pixelcount = cinfo.output_width.wrapping_mul(cinfo.output_height);
    if cinfo.output_width == 0
        || cinfo.output_height == 0
        || pixelcount
            .wrapping_mul(4 as libc::c_int as libc::c_uint)
            .wrapping_div(cinfo.output_width)
            .wrapping_div(4 as libc::c_int as libc::c_uint)
            != cinfo.output_height
        || pixelcount > 0x1fffffff as libc::c_int as libc::c_uint
        || cinfo.output_components != 3 as libc::c_int
    {
        // Free the memory to make sure we don't leak memory
        crate::src::renderergl1::tr_main::ri
            .FS_FreeFile
            .expect("non-null function pointer")(fbuffer.v);
        crate::src::jpeg_8c::jdapimin::jpeg_destroy_decompress(
            &mut cinfo as *mut _ as *mut crate::jpeglib_h::jpeg_decompress_struct,
        );
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"LoadJPG: %s has an invalid image format: %dx%d*4=%d, components: %d\x00" as *const u8
                as *const libc::c_char,
            filename,
            cinfo.output_width,
            cinfo.output_height,
            pixelcount.wrapping_mul(4 as libc::c_int as libc::c_uint),
            cinfo.output_components,
        );
    }
    memcount = pixelcount.wrapping_mul(4 as libc::c_int as libc::c_uint);
    row_stride = cinfo
        .output_width
        .wrapping_mul(cinfo.output_components as libc::c_uint);
    out = crate::src::renderergl1::tr_main::ri
        .Malloc
        .expect("non-null function pointer")(memcount as libc::c_int)
        as *mut crate::src::qcommon::q_shared::byte;
    *width = cinfo.output_width as libc::c_int;
    *height = cinfo.output_height as libc::c_int;
    /* Step 6: while (scan lines remain to be read) */
    /*           jpeg_read_scanlines(...); */
    /* Here we use the library's state variable cinfo.output_scanline as the
     * loop counter, so that we don't have to keep track ourselves.
     */
    while cinfo.output_scanline < cinfo.output_height {
        /* jpeg_read_scanlines expects an array of pointers to scanlines.
         * Here the array is only one element long, but you could ask for
         * more than one scanline at a time if that's more convenient.
         */
        buf = out.offset(row_stride.wrapping_mul(cinfo.output_scanline) as isize);
        buffer = &mut buf;
        crate::src::jpeg_8c::jdapistd::jpeg_read_scanlines(
            &mut cinfo as *mut _ as *mut crate::jpeglib_h::jpeg_decompress_struct,
            buffer,
            1 as libc::c_int as crate::jmorecfg_h::JDIMENSION,
        );
    }
    buf = out;
    // Expand from RGB to RGBA
    sindex = pixelcount.wrapping_mul(cinfo.output_components as libc::c_uint);
    dindex = memcount;
    loop {
        dindex = dindex.wrapping_sub(1);
        *buf.offset(dindex as isize) = 255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        sindex = sindex.wrapping_sub(1);
        dindex = dindex.wrapping_sub(1);
        *buf.offset(dindex as isize) = *buf.offset(sindex as isize);
        sindex = sindex.wrapping_sub(1);
        dindex = dindex.wrapping_sub(1);
        *buf.offset(dindex as isize) = *buf.offset(sindex as isize);
        sindex = sindex.wrapping_sub(1);
        dindex = dindex.wrapping_sub(1);
        *buf.offset(dindex as isize) = *buf.offset(sindex as isize);
        if !(sindex != 0) {
            break;
        }
    }
    *pic = out;
    /* Step 7: Finish decompression */
    crate::src::jpeg_8c::jdapimin::jpeg_finish_decompress(
        &mut cinfo as *mut _ as *mut crate::jpeglib_h::jpeg_decompress_struct,
    );
    /* We can ignore the return value since suspension is not possible
     * with the stdio data source.
     */
    /* Step 8: Release JPEG decompression object */
    /* This is an important step since it will release a good deal of memory. */
    crate::src::jpeg_8c::jdapimin::jpeg_destroy_decompress(
        &mut cinfo as *mut _ as *mut crate::jpeglib_h::jpeg_decompress_struct,
    );
    /* After finish_decompress, we can close the input file.
     * Here we postpone it until after no more JPEG errors are possible,
     * so as to simplify the setjmp error logic above.  (Actually, I don't
     * think that jpeg_destroy can do an error exit, but why assume anything...)
     */
    crate::src::renderergl1::tr_main::ri
        .FS_FreeFile
        .expect("non-null function pointer")(fbuffer.v);
    /* At this point you may want to check to see whether any corrupt-data
     * warnings occurred (test whether jerr.pub.num_warnings is nonzero).
     */
    /* And we're done! */
}
/*
 * Initialize destination --- called by jpeg_start_compress
 * before any data is actually written.
 */

unsafe extern "C" fn init_destination(mut cinfo: crate::jpeglib_h::j_compress_ptr) {
    let mut dest: my_dest_ptr = (*cinfo).dest as my_dest_ptr;
    (*dest).pub_0.next_output_byte = (*dest).outfile;
    (*dest).pub_0.free_in_buffer = (*dest).size as crate::stddef_h::size_t;
}
/*
 * Empty the output buffer --- called whenever buffer fills up.
 *
 * In typical applications, this should write the entire output buffer
 * (ignoring the current state of next_output_byte & free_in_buffer),
 * reset the pointer & count to the start of the buffer, and return TRUE
 * indicating that the buffer has been dumped.
 *
 * In applications that need to be able to suspend compression due to output
 * overrun, a FALSE return indicates that the buffer cannot be emptied now.
 * In this situation, the compressor will return to its caller (possibly with
 * an indication that it has not accepted all the supplied scanlines).  The
 * application should resume compression after it has made more room in the
 * output buffer.  Note that there are substantial restrictions on the use of
 * suspension --- see the documentation.
 *
 * When suspending, the compressor will back up to a convenient restart point
 * (typically the start of the current MCU). next_output_byte & free_in_buffer
 * indicate where the restart point will be if the current call returns FALSE.
 * Data beyond this point will be regenerated after resumption, so do not
 * write it out when emptying the buffer externally.
 */

unsafe extern "C" fn empty_output_buffer(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
) -> crate::jmorecfg_h::boolean {
    let mut dest: my_dest_ptr = (*cinfo).dest as my_dest_ptr;
    crate::src::jpeg_8c::jcapimin::jpeg_destroy_compress(
        cinfo as *mut crate::jpeglib_h::jpeg_compress_struct,
    );
    // Make crash fatal or we would probably leak memory.
    crate::src::renderergl1::tr_main::ri
        .Error
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int,
        b"Output buffer for encoded JPEG image has insufficient size of %d bytes\x00" as *const u8
            as *const libc::c_char,
        (*dest).size,
    );

}
/*
 * Terminate destination --- called by jpeg_finish_compress
 * after all data has been written.  Usually needs to flush buffer.
 *
 * NB: *not* called by jpeg_abort or jpeg_destroy; surrounding
 * application must deal with any cleanup that should happen even
 * for error exit.
 */

unsafe extern "C" fn term_destination(mut _cinfo: crate::jpeglib_h::j_compress_ptr) {}
/*
 * Prepare for output to a stdio stream.
 * The caller must have already opened the stream, and is responsible
 * for closing it after finishing compression.
 */

unsafe extern "C" fn jpegDest(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut outfile: *mut crate::src::qcommon::q_shared::byte,
    mut size: libc::c_int,
) {
    let mut dest: my_dest_ptr = 0 as *mut my_destination_mgr;
    /* The destination object is made permanent so that multiple JPEG images
     * can be written to the same file without re-executing jpeg_stdio_dest.
     * This makes it dangerous to use this manager and a different destination
     * manager serially with the same JPEG object, because their private object
     * sizes may be different.  Caveat programmer.
     */
    if (*cinfo).dest.is_null() {
        /* first time for this JPEG object? */
        (*cinfo).dest = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            0 as libc::c_int,
            ::std::mem::size_of::<my_destination_mgr>() as libc::c_ulong,
        ) as *mut crate::jpeglib_h::jpeg_destination_mgr
    }
    dest = (*cinfo).dest as my_dest_ptr;
    (*dest).pub_0.init_destination =
        Some(init_destination as unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> ());
    (*dest).pub_0.empty_output_buffer = Some(
        empty_output_buffer
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_compress_ptr,
            ) -> crate::jmorecfg_h::boolean,
    );
    (*dest).pub_0.term_destination =
        Some(term_destination as unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> ());
    (*dest).outfile = outfile;
    (*dest).size = size;
}
/*
=================
SaveJPGToBuffer

Encodes JPEG from image in image_buffer and writes to buffer.
Expects RGB input data
=================
*/
#[no_mangle]

pub unsafe extern "C" fn RE_SaveJPGToBuffer(
    mut buffer: *mut crate::src::qcommon::q_shared::byte,
    mut bufSize: crate::stddef_h::size_t,
    mut quality: libc::c_int,
    mut image_width: libc::c_int,
    mut image_height: libc::c_int,
    mut image_buffer: *mut crate::src::qcommon::q_shared::byte,
    mut padding: libc::c_int,
) -> crate::stddef_h::size_t {
    let mut cinfo: crate::jpeglib_h::jpeg_compress_struct =
        crate::jpeglib_h::jpeg_compress_struct {
            err: 0 as *mut crate::jpeglib_h::jpeg_error_mgr,
            mem: 0 as *mut crate::jpeglib_h::jpeg_memory_mgr,
            progress: 0 as *mut crate::jpeglib_h::jpeg_progress_mgr,
            client_data: 0 as *mut libc::c_void,
            is_decompressor: 0,
            global_state: 0,
            dest: 0 as *mut crate::jpeglib_h::jpeg_destination_mgr,
            image_width: 0,
            image_height: 0,
            input_components: 0,
            in_color_space: crate::jpeglib_h::JCS_UNKNOWN,
            input_gamma: 0.,
            scale_num: 0,
            scale_denom: 0,
            jpeg_width: 0,
            jpeg_height: 0,
            data_precision: 0,
            num_components: 0,
            jpeg_color_space: crate::jpeglib_h::JCS_UNKNOWN,
            comp_info: 0 as *mut crate::jpeglib_h::jpeg_component_info,
            quant_tbl_ptrs: [0 as *mut crate::jpeglib_h::JQUANT_TBL; 4],
            q_scale_factor: [0; 4],
            dc_huff_tbl_ptrs: [0 as *mut crate::jpeglib_h::JHUFF_TBL; 4],
            ac_huff_tbl_ptrs: [0 as *mut crate::jpeglib_h::JHUFF_TBL; 4],
            arith_dc_L: [0; 16],
            arith_dc_U: [0; 16],
            arith_ac_K: [0; 16],
            num_scans: 0,
            scan_info: 0 as *const crate::jpeglib_h::jpeg_scan_info,
            raw_data_in: 0,
            arith_code: 0,
            optimize_coding: 0,
            CCIR601_sampling: 0,
            do_fancy_downsampling: 0,
            smoothing_factor: 0,
            dct_method: crate::jpeglib_h::JDCT_ISLOW,
            restart_interval: 0,
            restart_in_rows: 0,
            write_JFIF_header: 0,
            JFIF_major_version: 0,
            JFIF_minor_version: 0,
            density_unit: 0,
            X_density: 0,
            Y_density: 0,
            write_Adobe_marker: 0,
            next_scanline: 0,
            progressive_mode: 0,
            max_h_samp_factor: 0,
            max_v_samp_factor: 0,
            min_DCT_h_scaled_size: 0,
            min_DCT_v_scaled_size: 0,
            total_iMCU_rows: 0,
            comps_in_scan: 0,
            cur_comp_info: [0 as *mut crate::jpeglib_h::jpeg_component_info; 4],
            MCUs_per_row: 0,
            MCU_rows_in_scan: 0,
            blocks_in_MCU: 0,
            MCU_membership: [0; 10],
            Ss: 0,
            Se: 0,
            Ah: 0,
            Al: 0,
            block_size: 0,
            natural_order: 0 as *const libc::c_int,
            lim_Se: 0,
            master: 0 as *mut crate::jpegint_h::jpeg_comp_master,
            main: 0 as *mut crate::jpegint_h::jpeg_c_main_controller,
            prep: 0 as *mut crate::jpegint_h::jpeg_c_prep_controller,
            coef: 0 as *mut crate::jpegint_h::jpeg_c_coef_controller,
            marker: 0 as *mut crate::jpegint_h::jpeg_marker_writer,
            cconvert: 0 as *mut crate::jpegint_h::jpeg_color_converter,
            downsample: 0 as *mut crate::jpegint_h::jpeg_downsampler,
            fdct: 0 as *mut crate::jpegint_h::jpeg_forward_dct,
            entropy: 0 as *mut crate::jpegint_h::jpeg_entropy_encoder,
            script_space: 0 as *mut crate::jpeglib_h::jpeg_scan_info,
            script_space_size: 0,
        }; /* pointer to JSAMPLE row[s] */
    let mut jerr: q_jpeg_error_mgr_t = q_jpeg_error_mgr_t {
        pub_0: crate::jpeglib_h::jpeg_error_mgr {
            error_exit: None,
            emit_message: None,
            output_message: None,
            format_message: None,
            reset_error_mgr: None,
            msg_code: 0,
            msg_parm: crate::jpeglib_h::C2RustUnnamed_0 { i: [0; 8] },
            trace_level: 0,
            num_warnings: 0,
            jpeg_message_table: 0 as *const *const libc::c_char,
            last_jpeg_message: 0,
            addon_message_table: 0 as *const *const libc::c_char,
            first_addon_message: 0,
            last_addon_message: 0,
        },
        setjmp_buffer: [crate::stdlib::__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: crate::stdlib::__sigset_t { __val: [0; 16] },
        }; 1],
    }; /* physical row width in image buffer */
    let mut row_pointer: [crate::jpeglib_h::JSAMPROW; 1] =
        [0 as *mut crate::jmorecfg_h::JSAMPLE; 1];
    let mut dest: my_dest_ptr = 0 as *mut my_destination_mgr;
    let mut row_stride: libc::c_int = 0;
    let mut outcount: crate::stddef_h::size_t = 0;
    /* Step 1: allocate and initialize JPEG compression object */
    cinfo.err = crate::src::jpeg_8c::jerror::jpeg_std_error(
        &mut jerr.pub_0 as *mut _ as *mut crate::jpeglib_h::jpeg_error_mgr,
    ) as *mut crate::jpeglib_h::jpeg_error_mgr;
    (*cinfo.err).error_exit =
        Some(R_JPGErrorExit as unsafe extern "C" fn(_: crate::jpeglib_h::j_common_ptr) -> ());
    (*cinfo.err).output_message =
        Some(R_JPGOutputMessage as unsafe extern "C" fn(_: crate::jpeglib_h::j_common_ptr) -> ());
    /* Establish the setjmp return context for R_JPGErrorExit to use. */
    if crate::stdlib::_setjmp(jerr.setjmp_buffer.as_mut_ptr()) != 0 {
        /* If we get here, the JPEG code has signaled an error.
         * We need to clean up the JPEG object and return.
         */
        crate::src::jpeg_8c::jcapimin::jpeg_destroy_compress(
            &mut cinfo as *mut _ as *mut crate::jpeglib_h::jpeg_compress_struct,
        );
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as libc::c_int,
            b"\n\x00" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int as crate::stddef_h::size_t;
    }
    /* Now we can initialize the JPEG compression object. */
    crate::src::jpeg_8c::jcapimin::jpeg_CreateCompress(
        &mut cinfo as *mut _ as *mut crate::jpeglib_h::jpeg_compress_struct,
        80 as libc::c_int,
        ::std::mem::size_of::<crate::jpeglib_h::jpeg_compress_struct>() as libc::c_ulong,
    );
    /* Step 2: specify data destination (eg, a file) */
    /* Note: steps 2 and 3 can be done in either order. */
    jpegDest(&mut cinfo, buffer, bufSize as libc::c_int);
    /* Step 3: set parameters for compression */
    cinfo.image_width = image_width as crate::jmorecfg_h::JDIMENSION; /* image width and height, in pixels */
    cinfo.image_height = image_height as crate::jmorecfg_h::JDIMENSION; /* # of color components per pixel */
    cinfo.input_components = 3 as libc::c_int; /* colorspace of input image */
    cinfo.in_color_space = crate::jpeglib_h::JCS_RGB;
    crate::src::jpeg_8c::jcparam::jpeg_set_defaults(
        &mut cinfo as *mut _ as *mut crate::jpeglib_h::jpeg_compress_struct,
    );
    crate::src::jpeg_8c::jcparam::jpeg_set_quality(
        &mut cinfo as *mut _ as *mut crate::jpeglib_h::jpeg_compress_struct,
        quality,
        1 as libc::c_int,
    );
    /* If quality is set high, disable chroma subsampling */
    if quality >= 85 as libc::c_int {
        (*cinfo.comp_info.offset(0 as libc::c_int as isize)).h_samp_factor = 1 as libc::c_int;
        (*cinfo.comp_info.offset(0 as libc::c_int as isize)).v_samp_factor = 1 as libc::c_int
    }
    /* Step 4: Start compressor */
    crate::src::jpeg_8c::jcapistd::jpeg_start_compress(
        &mut cinfo as *mut _ as *mut crate::jpeglib_h::jpeg_compress_struct,
        1 as libc::c_int,
    );
    /* Step 5: while (scan lines remain to be written) */
    /*           jpeg_write_scanlines(...); */
    row_stride = image_width * cinfo.input_components + padding; /* JSAMPLEs per row in image_buffer */
    while cinfo.next_scanline < cinfo.image_height {
        /* jpeg_write_scanlines expects an array of pointers to scanlines.
         * Here the array is only one element long, but you could pass
         * more than one scanline at a time if that's more convenient.
         */
        row_pointer[0 as libc::c_int as usize] = &mut *image_buffer.offset(
            cinfo
                .image_height
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                .wrapping_mul(row_stride as libc::c_uint)
                .wrapping_sub(cinfo.next_scanline.wrapping_mul(row_stride as libc::c_uint))
                as isize,
        )
            as *mut crate::src::qcommon::q_shared::byte;
        crate::src::jpeg_8c::jcapistd::jpeg_write_scanlines(
            &mut cinfo as *mut _ as *mut crate::jpeglib_h::jpeg_compress_struct,
            row_pointer.as_mut_ptr(),
            1 as libc::c_int as crate::jmorecfg_h::JDIMENSION,
        );
    }
    /* Step 6: Finish compression */
    crate::src::jpeg_8c::jcapimin::jpeg_finish_compress(
        &mut cinfo as *mut _ as *mut crate::jpeglib_h::jpeg_compress_struct,
    );
    dest = cinfo.dest as my_dest_ptr;
    outcount = ((*dest).size as libc::c_ulong).wrapping_sub((*dest).pub_0.free_in_buffer);
    /* Step 7: release JPEG compression object */
    crate::src::jpeg_8c::jcapimin::jpeg_destroy_compress(
        &mut cinfo as *mut _ as *mut crate::jpeglib_h::jpeg_compress_struct,
    );
    /* And we're done! */
    return outcount;
}
#[no_mangle]

pub unsafe extern "C" fn RE_SaveJPG(
    mut filename: *mut libc::c_char,
    mut quality: libc::c_int,
    mut image_width: libc::c_int,
    mut image_height: libc::c_int,
    mut image_buffer: *mut crate::src::qcommon::q_shared::byte,
    mut padding: libc::c_int,
) {
    let mut out: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut bufSize: crate::stddef_h::size_t = 0;
    bufSize = (image_width * image_height * 3 as libc::c_int) as crate::stddef_h::size_t;
    out = crate::src::renderergl1::tr_main::ri
        .Hunk_AllocateTempMemory
        .expect("non-null function pointer")(bufSize as libc::c_int)
        as *mut crate::src::qcommon::q_shared::byte;
    bufSize = RE_SaveJPGToBuffer(
        out,
        bufSize,
        quality,
        image_width,
        image_height,
        image_buffer,
        padding,
    );
    crate::src::renderergl1::tr_main::ri
        .FS_WriteFile
        .expect("non-null function pointer")(
        filename,
        out as *const libc::c_void,
        bufSize as libc::c_int,
    );
    crate::src::renderergl1::tr_main::ri
        .Hunk_FreeTempMemory
        .expect("non-null function pointer")(out as *mut libc::c_void);
}
