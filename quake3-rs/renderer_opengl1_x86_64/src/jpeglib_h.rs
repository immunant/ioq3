pub use crate::src::jpeg_8c::jmemmgr::jvirt_barray_control;
pub use crate::src::jpeg_8c::jmemmgr::jvirt_sarray_control;
/*
 * jpeglib.h
 *
 * Copyright (C) 1991-1998, Thomas G. Lane.
 * Modified 2002-2010 by Guido Vollbeding.
 * This file is part of the Independent JPEG Group's software.
 * For conditions of distribution and use, see the accompanying README file.
 *
 * This file defines the application interface for the JPEG library.
 * Most applications using the library need only include this file,
 * and perhaps jerror.h if they want to know the exact error codes.
 */

/*
 * First we include the configuration files that record how this
 * installation of the JPEG library is set up.  jconfig.h can be
 * generated automatically for many systems.  jmorecfg.h contains
 * manual configuration options that most people need not worry about.
 */

/* in case jinclude.h already did */

/* Version IDs for the JPEG library.
 * Might be useful for tests like "#if JPEG_LIB_VERSION >= 80".
 */

/* Compatibility version 8.0 */

/* Various constants determining the sizes of things.
 * All of these are specified by the JPEG standard, so don't change them
 * if you want to be compatible.
 */

/* The basic DCT block is 8x8 samples */

/* DCTSIZE squared; # of elements in a block */

/* Quantization tables are numbered 0..3 */

/* Huffman tables are numbered 0..3 */

/* Arith-coding tables are numbered 0..15 */

/* JPEG limit on # of components in one scan */

/* JPEG limit on sampling factors */

/* Unfortunately, some bozo at Adobe saw no reason to be bound by the standard;
 * the PostScript DCT filter can emit files with many more than 10 blocks/MCU.
 * If you happen to run across such a file, you can up D_MAX_BLOCKS_IN_MCU
 * to handle it.  We even let you do this from the jconfig.h file.  However,
 * we strongly discourage changing C_MAX_BLOCKS_IN_MCU; just because Adobe
 * sometimes emits noncompliant files doesn't mean you should too.
 */

/* compressor's limit on blocks per MCU */

/* decompressor's limit on blocks per MCU */

/* Data structures for images (arrays of samples and of DCT coefficients).
 * On 80x86 machines, the image arrays are too big for near pointers,
 * but the pointer arrays can fit in near memory.
 */
pub type JSAMPROW = *mut crate::jmorecfg_h::JSAMPLE;
/* ptr to one image row of pixel samples. */
pub type JSAMPARRAY = *mut JSAMPROW;
/* ptr to some rows (a 2-D sample array) */
pub type JSAMPIMAGE = *mut JSAMPARRAY;
/* a 3-D sample array: top index is color */
pub type JBLOCK = [crate::jmorecfg_h::JCOEF; 64];
/* one block of coefficients */
pub type JBLOCKROW = *mut JBLOCK;
/* pointer to one row of coefficient blocks */
pub type JBLOCKARRAY = *mut JBLOCKROW;
pub type JCOEFPTR = *mut crate::jmorecfg_h::JCOEF;
/* useful in a couple of places */

/* Types for JPEG compression parameters and working tables. */

/* DCT coefficient quantization tables. */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct JQUANT_TBL {
    pub quantval: [crate::jmorecfg_h::UINT16; 64],
    pub sent_table: crate::jmorecfg_h::boolean,
}
/* Huffman coding tables. */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct JHUFF_TBL {
    pub bits: [crate::jmorecfg_h::UINT8; 17],
    pub huffval: [crate::jmorecfg_h::UINT8; 256],
    pub sent_table: crate::jmorecfg_h::boolean,
}
/* Basic info about one component (color channel). */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_component_info {
    pub component_id: i32,
    pub component_index: i32,
    pub h_samp_factor: i32,
    pub v_samp_factor: i32,
    pub quant_tbl_no: i32,
    pub dc_tbl_no: i32,
    pub ac_tbl_no: i32,
    pub width_in_blocks: crate::jmorecfg_h::JDIMENSION,
    pub height_in_blocks: crate::jmorecfg_h::JDIMENSION,
    pub DCT_h_scaled_size: i32,
    pub DCT_v_scaled_size: i32,
    pub downsampled_width: crate::jmorecfg_h::JDIMENSION,
    pub downsampled_height: crate::jmorecfg_h::JDIMENSION,
    pub component_needed: crate::jmorecfg_h::boolean,
    pub MCU_width: i32,
    pub MCU_height: i32,
    pub MCU_blocks: i32,
    pub MCU_sample_width: i32,
    pub last_col_width: i32,
    pub last_row_height: i32,
    pub quant_table: *mut JQUANT_TBL,
    pub dct_table: *mut libc::c_void,
}
/* The script for encoding a multiple-scan file is an array of these: */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_scan_info {
    pub comps_in_scan: i32,
    pub component_index: [i32; 4],
    pub Ss: i32,
    pub Se: i32,
    pub Ah: i32,
    pub Al: i32,
}
/* The decompressor can save APPn and COM markers in a list of these: */
pub type jpeg_saved_marker_ptr = *mut jpeg_marker_struct;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_marker_struct {
    pub next: jpeg_saved_marker_ptr,
    pub marker: crate::jmorecfg_h::UINT8,
    pub original_length: u32,
    pub data_length: u32,
    pub data: *mut crate::jmorecfg_h::JOCTET,
}
/* the data contained in the marker */

/* the marker length word is not counted in data_length or original_length */

/* Known color spaces. */
pub type J_COLOR_SPACE = u32;
pub const JCS_UNKNOWN: J_COLOR_SPACE = 0;
/* error/unspecified */
pub const JCS_GRAYSCALE: J_COLOR_SPACE = 1;
/* monochrome */
pub const JCS_RGB: J_COLOR_SPACE = 2;
/* red/green/blue */
pub const JCS_YCbCr: J_COLOR_SPACE = 3;
/* Y/Cb/Cr (also known as YUV) */
pub const JCS_CMYK: J_COLOR_SPACE = 4;
/* Y/Cb/Cr/K */

/* C/M/Y/K */
pub const JCS_YCCK: J_COLOR_SPACE = 5;
/* DCT/IDCT algorithm options. */
pub type J_DCT_METHOD = u32;
pub const JDCT_ISLOW: J_DCT_METHOD = 0;
/* slow but accurate integer algorithm */
pub const JDCT_IFAST: J_DCT_METHOD = 1;
/* floating-point: accurate, fast on fast HW */

/* faster, less accurate integer method */
pub const JDCT_FLOAT: J_DCT_METHOD = 2;
pub type J_DITHER_MODE = u32;
pub const JDITHER_NONE: J_DITHER_MODE = 0;
pub const JDITHER_ORDERED: J_DITHER_MODE = 1;
pub const JDITHER_FS: J_DITHER_MODE = 2;
/* Common fields between JPEG compression and decompression master structs. */

/* Error handler module */

/* Memory manager module */

/* Progress monitor, or NULL if none */

/* Available for use by application */

/* So common code can tell which is which */

/* For checking call sequence validity */

/* Routines that are to be used by both halves of the library are declared
 * to receive a pointer to this structure.  There are no actual instances of
 * jpeg_common_struct, only of jpeg_compress_struct and jpeg_decompress_struct.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_common_struct {
    pub err: *mut jpeg_error_mgr,
    pub mem: *mut jpeg_memory_mgr,
    pub progress: *mut jpeg_progress_mgr,
    pub client_data: *mut libc::c_void,
    pub is_decompressor: crate::jmorecfg_h::boolean,
    pub global_state: i32,
}
/* Fields common to both master struct types */

/* Additional fields follow in an actual jpeg_compress_struct or
 * jpeg_decompress_struct.  All three structs must agree on these
 * initial fields!  (This would be a lot cleaner in C++.)
 */
pub type j_common_ptr = *mut jpeg_common_struct;
pub type j_compress_ptr = *mut jpeg_compress_struct;
pub type j_decompress_ptr = *mut jpeg_decompress_struct;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_compress_struct {
    pub err: *mut jpeg_error_mgr,
    pub mem: *mut jpeg_memory_mgr,
    pub progress: *mut jpeg_progress_mgr,
    pub client_data: *mut libc::c_void,
    pub is_decompressor: crate::jmorecfg_h::boolean,
    pub global_state: i32,
    pub dest: *mut jpeg_destination_mgr,
    pub image_width: crate::jmorecfg_h::JDIMENSION,
    pub image_height: crate::jmorecfg_h::JDIMENSION,
    pub input_components: i32,
    pub in_color_space: J_COLOR_SPACE,
    pub input_gamma: f64,
    pub scale_num: u32,
    pub scale_denom: u32,
    pub jpeg_width: crate::jmorecfg_h::JDIMENSION,
    pub jpeg_height: crate::jmorecfg_h::JDIMENSION,
    pub data_precision: i32,
    pub num_components: i32,
    pub jpeg_color_space: J_COLOR_SPACE,
    pub comp_info: *mut jpeg_component_info,
    pub quant_tbl_ptrs: [*mut JQUANT_TBL; 4],
    pub q_scale_factor: [i32; 4],
    pub dc_huff_tbl_ptrs: [*mut JHUFF_TBL; 4],
    pub ac_huff_tbl_ptrs: [*mut JHUFF_TBL; 4],
    pub arith_dc_L: [crate::jmorecfg_h::UINT8; 16],
    pub arith_dc_U: [crate::jmorecfg_h::UINT8; 16],
    pub arith_ac_K: [crate::jmorecfg_h::UINT8; 16],
    pub num_scans: i32,
    pub scan_info: *const jpeg_scan_info,
    pub raw_data_in: crate::jmorecfg_h::boolean,
    pub arith_code: crate::jmorecfg_h::boolean,
    pub optimize_coding: crate::jmorecfg_h::boolean,
    pub CCIR601_sampling: crate::jmorecfg_h::boolean,
    pub do_fancy_downsampling: crate::jmorecfg_h::boolean,
    pub smoothing_factor: i32,
    pub dct_method: J_DCT_METHOD,
    pub restart_interval: u32,
    pub restart_in_rows: i32,
    pub write_JFIF_header: crate::jmorecfg_h::boolean,
    pub JFIF_major_version: crate::jmorecfg_h::UINT8,
    pub JFIF_minor_version: crate::jmorecfg_h::UINT8,
    pub density_unit: crate::jmorecfg_h::UINT8,
    pub X_density: crate::jmorecfg_h::UINT16,
    pub Y_density: crate::jmorecfg_h::UINT16,
    pub write_Adobe_marker: crate::jmorecfg_h::boolean,
    pub next_scanline: crate::jmorecfg_h::JDIMENSION,
    pub progressive_mode: crate::jmorecfg_h::boolean,
    pub max_h_samp_factor: i32,
    pub max_v_samp_factor: i32,
    pub min_DCT_h_scaled_size: i32,
    pub min_DCT_v_scaled_size: i32,
    pub total_iMCU_rows: crate::jmorecfg_h::JDIMENSION,
    pub comps_in_scan: i32,
    pub cur_comp_info: [*mut jpeg_component_info; 4],
    pub MCUs_per_row: crate::jmorecfg_h::JDIMENSION,
    pub MCU_rows_in_scan: crate::jmorecfg_h::JDIMENSION,
    pub blocks_in_MCU: i32,
    pub MCU_membership: [i32; 10],
    pub Ss: i32,
    pub Se: i32,
    pub Ah: i32,
    pub Al: i32,
    pub block_size: i32,
    pub natural_order: *const i32,
    pub lim_Se: i32,
    pub master: *mut crate::jpegint_h::jpeg_comp_master,
    pub main: *mut crate::jpegint_h::jpeg_c_main_controller,
    pub prep: *mut crate::jpegint_h::jpeg_c_prep_controller,
    pub coef: *mut crate::jpegint_h::jpeg_c_coef_controller,
    pub marker: *mut crate::jpegint_h::jpeg_marker_writer,
    pub cconvert: *mut crate::jpegint_h::jpeg_color_converter,
    pub downsample: *mut crate::jpegint_h::jpeg_downsampler,
    pub fdct: *mut crate::jpegint_h::jpeg_forward_dct,
    pub entropy: *mut crate::jpegint_h::jpeg_entropy_encoder,
    pub script_space: *mut jpeg_scan_info,
    pub script_space_size: i32,
}
/* Master record for a decompression instance */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_decompress_struct {
    pub err: *mut jpeg_error_mgr,
    pub mem: *mut jpeg_memory_mgr,
    pub progress: *mut jpeg_progress_mgr,
    pub client_data: *mut libc::c_void,
    pub is_decompressor: crate::jmorecfg_h::boolean,
    pub global_state: i32,
    pub src: *mut jpeg_source_mgr,
    pub image_width: crate::jmorecfg_h::JDIMENSION,
    pub image_height: crate::jmorecfg_h::JDIMENSION,
    pub num_components: i32,
    pub jpeg_color_space: J_COLOR_SPACE,
    pub out_color_space: J_COLOR_SPACE,
    pub scale_num: u32,
    pub scale_denom: u32,
    pub output_gamma: f64,
    pub buffered_image: crate::jmorecfg_h::boolean,
    pub raw_data_out: crate::jmorecfg_h::boolean,
    pub dct_method: J_DCT_METHOD,
    pub do_fancy_upsampling: crate::jmorecfg_h::boolean,
    pub do_block_smoothing: crate::jmorecfg_h::boolean,
    pub quantize_colors: crate::jmorecfg_h::boolean,
    pub dither_mode: J_DITHER_MODE,
    pub two_pass_quantize: crate::jmorecfg_h::boolean,
    pub desired_number_of_colors: i32,
    pub enable_1pass_quant: crate::jmorecfg_h::boolean,
    pub enable_external_quant: crate::jmorecfg_h::boolean,
    pub enable_2pass_quant: crate::jmorecfg_h::boolean,
    pub output_width: crate::jmorecfg_h::JDIMENSION,
    pub output_height: crate::jmorecfg_h::JDIMENSION,
    pub out_color_components: i32,
    pub output_components: i32,
    pub rec_outbuf_height: i32,
    pub actual_number_of_colors: i32,
    pub colormap: JSAMPARRAY,
    pub output_scanline: crate::jmorecfg_h::JDIMENSION,
    pub input_scan_number: i32,
    pub input_iMCU_row: crate::jmorecfg_h::JDIMENSION,
    pub output_scan_number: i32,
    pub output_iMCU_row: crate::jmorecfg_h::JDIMENSION,
    pub coef_bits: *mut [i32; 64],
    pub quant_tbl_ptrs: [*mut JQUANT_TBL; 4],
    pub dc_huff_tbl_ptrs: [*mut JHUFF_TBL; 4],
    pub ac_huff_tbl_ptrs: [*mut JHUFF_TBL; 4],
    pub data_precision: i32,
    pub comp_info: *mut jpeg_component_info,
    pub is_baseline: crate::jmorecfg_h::boolean,
    pub progressive_mode: crate::jmorecfg_h::boolean,
    pub arith_code: crate::jmorecfg_h::boolean,
    pub arith_dc_L: [crate::jmorecfg_h::UINT8; 16],
    pub arith_dc_U: [crate::jmorecfg_h::UINT8; 16],
    pub arith_ac_K: [crate::jmorecfg_h::UINT8; 16],
    pub restart_interval: u32,
    pub saw_JFIF_marker: crate::jmorecfg_h::boolean,
    pub JFIF_major_version: crate::jmorecfg_h::UINT8,
    pub JFIF_minor_version: crate::jmorecfg_h::UINT8,
    pub density_unit: crate::jmorecfg_h::UINT8,
    pub X_density: crate::jmorecfg_h::UINT16,
    pub Y_density: crate::jmorecfg_h::UINT16,
    pub saw_Adobe_marker: crate::jmorecfg_h::boolean,
    pub Adobe_transform: crate::jmorecfg_h::UINT8,
    pub CCIR601_sampling: crate::jmorecfg_h::boolean,
    pub marker_list: jpeg_saved_marker_ptr,
    pub max_h_samp_factor: i32,
    pub max_v_samp_factor: i32,
    pub min_DCT_h_scaled_size: i32,
    pub min_DCT_v_scaled_size: i32,
    pub total_iMCU_rows: crate::jmorecfg_h::JDIMENSION,
    pub sample_range_limit: *mut crate::jmorecfg_h::JSAMPLE,
    pub comps_in_scan: i32,
    pub cur_comp_info: [*mut jpeg_component_info; 4],
    pub MCUs_per_row: crate::jmorecfg_h::JDIMENSION,
    pub MCU_rows_in_scan: crate::jmorecfg_h::JDIMENSION,
    pub blocks_in_MCU: i32,
    pub MCU_membership: [i32; 10],
    pub Ss: i32,
    pub Se: i32,
    pub Ah: i32,
    pub Al: i32,
    pub block_size: i32,
    pub natural_order: *const i32,
    pub lim_Se: i32,
    pub unread_marker: i32,
    pub master: *mut crate::jpegint_h::jpeg_decomp_master,
    pub main: *mut crate::jpegint_h::jpeg_d_main_controller,
    pub coef: *mut crate::jpegint_h::jpeg_d_coef_controller,
    pub post: *mut crate::jpegint_h::jpeg_d_post_controller,
    pub inputctl: *mut crate::jpegint_h::jpeg_input_controller,
    pub marker: *mut crate::jpegint_h::jpeg_marker_reader,
    pub entropy: *mut crate::jpegint_h::jpeg_entropy_decoder,
    pub idct: *mut crate::jpegint_h::jpeg_inverse_dct,
    pub upsample: *mut crate::jpegint_h::jpeg_upsampler,
    pub cconvert: *mut crate::jpegint_h::jpeg_color_deconverter,
    pub cquantize: *mut crate::jpegint_h::jpeg_color_quantizer,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_error_mgr {
    pub error_exit: Option<unsafe extern "C" fn(_: j_common_ptr) -> ()>,
    pub emit_message: Option<unsafe extern "C" fn(_: j_common_ptr, _: i32) -> ()>,
    pub output_message: Option<unsafe extern "C" fn(_: j_common_ptr) -> ()>,
    pub format_message: Option<unsafe extern "C" fn(_: j_common_ptr, _: *mut libc::c_char) -> ()>,
    pub reset_error_mgr: Option<unsafe extern "C" fn(_: j_common_ptr) -> ()>,
    pub msg_code: i32,
    pub msg_parm: C2RustUnnamed_0,
    pub trace_level: i32,
    pub num_warnings: isize,
    pub jpeg_message_table: *const *const libc::c_char,
    pub last_jpeg_message: i32,
    pub addon_message_table: *const *const libc::c_char,
    pub first_addon_message: i32,
    pub last_addon_message: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_0 {
    pub i: [i32; 8],
    pub s: [libc::c_char; 80],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_progress_mgr {
    pub progress_monitor: Option<unsafe extern "C" fn(_: j_common_ptr) -> ()>,
    pub pass_counter: isize,
    pub pass_limit: isize,
    pub completed_passes: i32,
    pub total_passes: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_destination_mgr {
    pub next_output_byte: *mut crate::jmorecfg_h::JOCTET,
    pub free_in_buffer: crate::stddef_h::size_t,
    pub init_destination: Option<unsafe extern "C" fn(_: j_compress_ptr) -> ()>,
    pub empty_output_buffer:
        Option<unsafe extern "C" fn(_: j_compress_ptr) -> crate::jmorecfg_h::boolean>,
    pub term_destination: Option<unsafe extern "C" fn(_: j_compress_ptr) -> ()>,
}
/* Data source object for decompression */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_source_mgr {
    pub next_input_byte: *const crate::jmorecfg_h::JOCTET,
    pub bytes_in_buffer: crate::stddef_h::size_t,
    pub init_source: Option<unsafe extern "C" fn(_: j_decompress_ptr) -> ()>,
    pub fill_input_buffer:
        Option<unsafe extern "C" fn(_: j_decompress_ptr) -> crate::jmorecfg_h::boolean>,
    pub skip_input_data: Option<unsafe extern "C" fn(_: j_decompress_ptr, _: isize) -> ()>,
    pub resync_to_restart:
        Option<unsafe extern "C" fn(_: j_decompress_ptr, _: i32) -> crate::jmorecfg_h::boolean>,
    pub term_source: Option<unsafe extern "C" fn(_: j_decompress_ptr) -> ()>,
}
pub type jvirt_sarray_ptr = *mut jvirt_sarray_control;
/* Master record for a compression instance */

/* Fields shared with jpeg_decompress_struct */

/* Destination for compressed data */

/* Description of source image --- these fields must be filled in by
 * outer application before starting compression.  in_color_space must
 * be correct before you can even call jpeg_set_defaults().
 */

/* input image width */

/* input image height */

/* # of color components in input image */

/* colorspace of input image */

/* image gamma of input image */

/* Compression parameters --- these fields must be set before calling
 * jpeg_start_compress().  We recommend calling jpeg_set_defaults() to
 * initialize everything to reasonable defaults, then changing anything
 * the application specifically wants to change.  That way you won't get
 * burnt when new parameters are added.  Also note that there are several
 * helper routines to simplify changing parameters.
 */

/* fraction by which to scale image */

/* scaled JPEG image width */

/* scaled JPEG image height */

/* Dimensions of actual JPEG image that will be written to file,
 * derived from input dimensions by scaling factors above.
 * These fields are computed by jpeg_start_compress().
 * You can also use jpeg_calc_jpeg_dimensions() to determine these values
 * in advance of calling jpeg_start_compress().
 */

/* bits of precision in image data */

/* # of color components in JPEG image */

/* colorspace of JPEG image */

/* comp_info[i] describes component that appears i'th in SOF */

/* ptrs to coefficient quantization tables, or NULL if not defined,
 * and corresponding scale factors (percentage, initialized 100).
 */

/* ptrs to Huffman coding tables, or NULL if not defined */

/* L values for DC arith-coding tables */

/* U values for DC arith-coding tables */

/* Kx values for AC arith-coding tables */

/* # of entries in scan_info array */

/* script for multi-scan file, or NULL */

/* The default value of scan_info is NULL, which causes a single-scan
 * sequential JPEG file to be emitted.  To create a multi-scan file,
 * set num_scans and scan_info to point to an array of scan definitions.
 */

/* TRUE=caller supplies downsampled data */

/* TRUE=arithmetic coding, FALSE=Huffman */

/* TRUE=optimize entropy encoding parms */

/* TRUE=first samples are cosited */

/* TRUE=apply fancy downsampling */

/* 1..100, or 0 for no input smoothing */

/* DCT algorithm selector */

/* The restart interval can be specified in absolute MCUs by setting
 * restart_interval, or in MCU rows by setting restart_in_rows
 * (in which case the correct restart_interval will be figured
 * for each scan).
 */

/* MCUs per restart, or 0 for no restart */

/* if > 0, MCU rows per restart interval */

/* Parameters controlling emission of special markers. */

/* should a JFIF marker be written? */

/* What to write for the JFIF version number */

/* These three values are not used by the JPEG code, merely copied */

/* into the JFIF APP0 marker.  density_unit can be 0 for unknown, */

/* 1 for dots/inch, or 2 for dots/cm.  Note that the pixel aspect */

/* ratio is defined by X_density/Y_density even when density_unit=0. */

/* JFIF code for pixel size units */

/* Horizontal pixel density */

/* Vertical pixel density */

/* should an Adobe marker be written? */

/* State variable: index of next scanline to be written to
 * jpeg_write_scanlines().  Application may use this to control its
 * processing loop, e.g., "while (next_scanline < image_height)".
 */

/* 0 .. image_height-1  */

/* Remaining fields are known throughout compressor, but generally
 * should not be touched by a surrounding application.
 */

/*
 * These fields are computed during compression startup
 */

/* TRUE if scan script uses progressive mode */

/* largest h_samp_factor */

/* largest v_samp_factor */

/* smallest DCT_h_scaled_size of any component */

/* smallest DCT_v_scaled_size of any component */

/* # of iMCU rows to be input to coef ctlr */

/* The coefficient controller receives data in units of MCU rows as defined
 * for fully interleaved scans (whether the JPEG file is interleaved or not).
 * There are v_samp_factor * DCTSIZE sample rows of each component in an
 * "iMCU" (interleaved MCU) row.
 */

/*
 * These fields are valid during any one scan.
 * They describe the components and MCUs actually appearing in the scan.
 */

/* # of JPEG components in this scan */

/* *cur_comp_info[i] describes component that appears i'th in SOS */

/* # of MCUs across the image */

/* # of MCU rows in the image */

/* # of DCT blocks per MCU */

/* MCU_membership[i] is index in cur_comp_info of component owning */

/* i'th block in an MCU */

/* progressive JPEG parameters for scan */

/* the basic DCT block size: 1..16 */

/* natural-order position array */

/* min( Se, DCTSIZE2-1 ) */

/*
 * Links to compression subobjects (methods and private variables of modules)
 */

/* workspace for jpeg_simple_progression */

/* Master record for a decompression instance */

/* Fields shared with jpeg_compress_struct */

/* Source of compressed data */

/* Basic description of image --- filled in by jpeg_read_header(). */

/* Application may inspect these values to decide how to process image. */

/* nominal image width (from SOF marker) */

/* nominal image height */

/* # of color components in JPEG image */

/* colorspace of JPEG image */

/* Decompression processing parameters --- these fields must be set before
 * calling jpeg_start_decompress().  Note that jpeg_read_header() initializes
 * them to default values.
 */

/* colorspace for output */

/* fraction by which to scale image */

/* image gamma wanted in output */

/* TRUE=multiple output passes */

/* TRUE=downsampled data wanted */

/* IDCT algorithm selector */

/* TRUE=apply fancy upsampling */

/* TRUE=apply interblock smoothing */

/* TRUE=colormapped output wanted */

/* the following are ignored if not quantize_colors: */

/* type of color dithering to use */

/* TRUE=use two-pass color quantization */

/* max # colors to use in created colormap */

/* these are significant only in buffered-image mode: */

/* enable future use of 1-pass quantizer */

/* enable future use of external colormap */

/* enable future use of 2-pass quantizer */

/* Description of actual output image that will be returned to application.
 * These fields are computed by jpeg_start_decompress().
 * You can also use jpeg_calc_output_dimensions() to determine these values
 * in advance of calling jpeg_start_decompress().
 */

/* scaled image width */

/* scaled image height */

/* # of color components in out_color_space */

/* # of color components returned */

/* output_components is 1 (a colormap index) when quantizing colors;
 * otherwise it equals out_color_components.
 */

/* min recommended height of scanline buffer */

/* If the buffer passed to jpeg_read_scanlines() is less than this many rows
 * high, space and time will be wasted due to unnecessary data copying.
 * Usually rec_outbuf_height will be 1 or 2, at most 4.
 */

/* When quantizing colors, the output colormap is described by these fields.
 * The application can supply a colormap by setting colormap non-NULL before
 * calling jpeg_start_decompress; otherwise a colormap is created during
 * jpeg_start_decompress or jpeg_start_output.
 * The map has out_color_components rows and actual_number_of_colors columns.
 */

/* number of entries in use */

/* The color map as a 2-D pixel array */

/* State variables: these variables indicate the progress of decompression.
 * The application may examine these but must not modify them.
 */

/* Row index of next scanline to be read from jpeg_read_scanlines().
 * Application may use this to control its processing loop, e.g.,
 * "while (output_scanline < output_height)".
 */

/* 0 .. output_height-1  */

/* Current input scan number and number of iMCU rows completed in scan.
 * These indicate the progress of the decompressor input side.
 */

/* Number of SOS markers seen so far */

/* Number of iMCU rows completed */

/* The "output scan number" is the notional scan being displayed by the
 * output side.  The decompressor will not allow output scan/row number
 * to get ahead of input scan/row, but it can fall arbitrarily far behind.
 */

/* Nominal scan number being displayed */

/* Number of iMCU rows read */

/* Current progression status.  coef_bits[c][i] indicates the precision
 * with which component c's DCT coefficient i (in zigzag order) is known.
 * It is -1 when no data has yet been received, otherwise it is the point
 * transform (shift) value for the most recent scan of the coefficient
 * (thus, 0 at completion of the progression).
 * This pointer is NULL when reading a non-progressive file.
 */

/* -1 or current Al value for each coef */

/* Internal JPEG parameters --- the application usually need not look at
 * these fields.  Note that the decompressor output side may not use
 * any parameters that can change between scans.
 */

/* Quantization and Huffman tables are carried forward across input
 * datastreams when processing abbreviated JPEG datastreams.
 */

/* ptrs to coefficient quantization tables, or NULL if not defined */

/* ptrs to Huffman coding tables, or NULL if not defined */

/* These parameters are never carried across datastreams, since they
 * are given in SOF/SOS markers or defined to be reset by SOI.
 */

/* bits of precision in image data */

/* comp_info[i] describes component that appears i'th in SOF */

/* TRUE if Baseline SOF0 encountered */

/* TRUE if SOFn specifies progressive mode */

/* TRUE=arithmetic coding, FALSE=Huffman */

/* L values for DC arith-coding tables */

/* U values for DC arith-coding tables */

/* Kx values for AC arith-coding tables */

/* MCUs per restart interval, or 0 for no restart */

/* These fields record data obtained from optional markers recognized by
 * the JPEG library.
 */

/* TRUE iff a JFIF APP0 marker was found */

/* Data copied from JFIF marker; only valid if saw_JFIF_marker is TRUE: */

/* JFIF version number */

/* JFIF code for pixel size units */

/* Horizontal pixel density */

/* Vertical pixel density */

/* TRUE iff an Adobe APP14 marker was found */

/* Color transform code from Adobe marker */

/* TRUE=first samples are cosited */

/* Aside from the specific data retained from APPn markers known to the
 * library, the uninterpreted contents of any or all APPn and COM markers
 * can be saved in a list for examination by the application.
 */

/* Head of list of saved markers */

/* Remaining fields are known throughout decompressor, but generally
 * should not be touched by a surrounding application.
 */

/*
 * These fields are computed during decompression startup
 */

/* largest h_samp_factor */

/* largest v_samp_factor */

/* smallest DCT_h_scaled_size of any component */

/* smallest DCT_v_scaled_size of any component */

/* # of iMCU rows in image */

/* The coefficient controller's input and output progress is measured in
 * units of "iMCU" (interleaved MCU) rows.  These are the same as MCU rows
 * in fully interleaved JPEG scans, but are used whether the scan is
 * interleaved or not.  We define an iMCU row as v_samp_factor DCT block
 * rows of each component.  Therefore, the IDCT output contains
 * v_samp_factor*DCT_v_scaled_size sample rows of a component per iMCU row.
 */

/* table for fast range-limiting */

/*
 * These fields are valid during any one scan.
 * They describe the components and MCUs actually appearing in the scan.
 * Note that the decompressor output side must not use these fields.
 */

/* # of JPEG components in this scan */

/* *cur_comp_info[i] describes component that appears i'th in SOS */

/* # of MCUs across the image */

/* # of MCU rows in the image */

/* # of DCT blocks per MCU */

/* MCU_membership[i] is index in cur_comp_info of component owning */

/* i'th block in an MCU */

/* progressive JPEG parameters for scan */

/* These fields are derived from Se of first SOS marker.
 */

/* the basic DCT block size: 1..16 */

/* natural-order position array for entropy decode */

/* min( Se, DCTSIZE2-1 ) for entropy decode */

/* This field is shared between entropy decoder and marker parser.
 * It is either zero or the code of a JPEG marker that has been
 * read from the data source, but has not yet been processed.
 */

/*
 * Links to decompression subobjects (methods, private variables of modules)
 */

/* "Object" declarations for JPEG modules that may be supplied or called
 * directly by the surrounding application.
 * As with all objects in the JPEG library, these structs only define the
 * publicly visible methods and state variables of a module.  Additional
 * private fields may exist after the public ones.
 */

/* Error handler object */

/* Error exit handler: does not return to caller */

/* Conditionally emit a trace or warning message */

/* Routine that actually outputs a trace or error message */

/* Format a message string for the most recent JPEG error or message */

/* recommended size of format_message buffer */

/* Reset error state variables at start of a new image */

/* The message ID code and any parameters are saved here.
 * A message can have one string parameter or up to 8 int parameters.
 */

/* Standard state variables for error facility */

/* max msg_level that will be displayed */

/* For recoverable corrupt-data errors, we emit a warning message,
 * but keep going unless emit_message chooses to abort.  emit_message
 * should count warnings in num_warnings.  The surrounding application
 * can check for bad data by seeing if num_warnings is nonzero at the
 * end of processing.
 */

/* number of corrupt-data warnings */

/* These fields point to the table(s) of error message strings.
 * An application can change the table pointer to switch to a different
 * message list (typically, to change the language in which errors are
 * reported).  Some applications may wish to add additional error codes
 * that will be handled by the JPEG library error mechanism; the second
 * table pointer is used for this purpose.
 *
 * First table includes all errors generated by JPEG library itself.
 * Error code 0 is reserved for a "no such error string" message.
 */

/* Library errors */

/* Table contains strings 0..last_jpeg_message */

/* Second table can be added by application (see cjpeg/djpeg for example).
 * It contains strings numbered first_addon_message..last_addon_message.
 */

/* Non-library errors */

/* code for first string in addon table */

/* code for last string in addon table */

/* Progress monitor object */

/* work units completed in this pass */

/* total number of work units in this pass */

/* passes completed so far */

/* total number of passes expected */

/* Data destination object for compression */

/* => next byte to write in buffer */

/* # of byte spaces remaining in buffer */

/* Data source object for decompression */

/* => next byte to read from buffer */

/* # of bytes remaining in buffer */

/* Memory manager object.
 * Allocates "small" objects (a few K total), "large" objects (tens of K),
 * and "really big" objects (virtual arrays with backing store if needed).
 * The memory manager does not allow individual objects to be freed; rather,
 * each created object is assigned to a pool, and whole pools can be freed
 * at once.  This is faster and more convenient than remembering exactly what
 * to free, especially where malloc()/free() are not too speedy.
 * NB: alloc routines never return NULL.  They exit to error_exit if not
 * successful.
 */

/* lasts until master record is destroyed */

/* lasts until done with image/datastream */
pub type jvirt_barray_ptr = *mut jvirt_barray_control;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_memory_mgr {
    pub alloc_small: Option<
        unsafe extern "C" fn(
            _: j_common_ptr,
            _: i32,
            _: crate::stddef_h::size_t,
        ) -> *mut libc::c_void,
    >,
    pub alloc_large: Option<
        unsafe extern "C" fn(
            _: j_common_ptr,
            _: i32,
            _: crate::stddef_h::size_t,
        ) -> *mut libc::c_void,
    >,
    pub alloc_sarray: Option<
        unsafe extern "C" fn(
            _: j_common_ptr,
            _: i32,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jmorecfg_h::JDIMENSION,
        ) -> JSAMPARRAY,
    >,
    pub alloc_barray: Option<
        unsafe extern "C" fn(
            _: j_common_ptr,
            _: i32,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jmorecfg_h::JDIMENSION,
        ) -> JBLOCKARRAY,
    >,
    pub request_virt_sarray: Option<
        unsafe extern "C" fn(
            _: j_common_ptr,
            _: i32,
            _: crate::jmorecfg_h::boolean,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jmorecfg_h::JDIMENSION,
        ) -> jvirt_sarray_ptr,
    >,
    pub request_virt_barray: Option<
        unsafe extern "C" fn(
            _: j_common_ptr,
            _: i32,
            _: crate::jmorecfg_h::boolean,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jmorecfg_h::JDIMENSION,
        ) -> jvirt_barray_ptr,
    >,
    pub realize_virt_arrays: Option<unsafe extern "C" fn(_: j_common_ptr) -> ()>,
    pub access_virt_sarray: Option<
        unsafe extern "C" fn(
            _: j_common_ptr,
            _: jvirt_sarray_ptr,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jmorecfg_h::boolean,
        ) -> JSAMPARRAY,
    >,
    pub access_virt_barray: Option<
        unsafe extern "C" fn(
            _: j_common_ptr,
            _: jvirt_barray_ptr,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jmorecfg_h::boolean,
        ) -> JBLOCKARRAY,
    >,
    pub free_pool: Option<unsafe extern "C" fn(_: j_common_ptr, _: i32) -> ()>,
    pub self_destruct: Option<unsafe extern "C" fn(_: j_common_ptr) -> ()>,
    pub max_memory_to_use: isize,
    pub max_alloc_chunk: isize,
}
pub type jpeg_marker_parser_method =
    Option<unsafe extern "C" fn(_: j_decompress_ptr) -> crate::jmorecfg_h::boolean>;
