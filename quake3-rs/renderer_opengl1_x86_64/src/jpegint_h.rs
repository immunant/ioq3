pub type J_BUF_MODE = u32;
pub const JBUF_PASS_THRU: J_BUF_MODE = 0;
pub const JBUF_SAVE_SOURCE: J_BUF_MODE = 1;
pub const JBUF_CRANK_DEST: J_BUF_MODE = 2;
pub const JBUF_SAVE_AND_PASS: J_BUF_MODE = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_comp_master {
    pub prepare_for_pass: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> ()>,
    pub pass_startup: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> ()>,
    pub finish_pass: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> ()>,
    pub call_pass_startup: crate::jmorecfg_h::boolean,
    pub is_last_pass: crate::jmorecfg_h::boolean,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_c_main_controller {
    pub start_pass:
        Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr, _: J_BUF_MODE) -> ()>,
    pub process_data: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_compress_ptr,
            _: crate::jpeglib_h::JSAMPARRAY,
            _: *mut crate::jmorecfg_h::JDIMENSION,
            _: crate::jmorecfg_h::JDIMENSION,
        ) -> (),
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_c_prep_controller {
    pub start_pass:
        Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr, _: J_BUF_MODE) -> ()>,
    pub pre_process_data: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_compress_ptr,
            _: crate::jpeglib_h::JSAMPARRAY,
            _: *mut crate::jmorecfg_h::JDIMENSION,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jpeglib_h::JSAMPIMAGE,
            _: *mut crate::jmorecfg_h::JDIMENSION,
            _: crate::jmorecfg_h::JDIMENSION,
        ) -> (),
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_c_coef_controller {
    pub start_pass:
        Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr, _: J_BUF_MODE) -> ()>,
    pub compress_data: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_compress_ptr,
            _: crate::jpeglib_h::JSAMPIMAGE,
        ) -> crate::jmorecfg_h::boolean,
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_color_converter {
    pub start_pass: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> ()>,
    pub color_convert: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_compress_ptr,
            _: crate::jpeglib_h::JSAMPARRAY,
            _: crate::jpeglib_h::JSAMPIMAGE,
            _: crate::jmorecfg_h::JDIMENSION,
            _: i32,
        ) -> (),
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_downsampler {
    pub start_pass: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> ()>,
    pub downsample: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_compress_ptr,
            _: crate::jpeglib_h::JSAMPIMAGE,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jpeglib_h::JSAMPIMAGE,
            _: crate::jmorecfg_h::JDIMENSION,
        ) -> (),
    >,
    pub need_context_rows: crate::jmorecfg_h::boolean,
}
pub type forward_DCT_ptr = Option<
    unsafe extern "C" fn(
        _: crate::jpeglib_h::j_compress_ptr,
        _: *mut crate::jpeglib_h::jpeg_component_info,
        _: crate::jpeglib_h::JSAMPARRAY,
        _: crate::jpeglib_h::JBLOCKROW,
        _: crate::jmorecfg_h::JDIMENSION,
        _: crate::jmorecfg_h::JDIMENSION,
        _: crate::jmorecfg_h::JDIMENSION,
    ) -> (),
>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_forward_dct {
    pub start_pass: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> ()>,
    pub forward_DCT: [forward_DCT_ptr; 10],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_entropy_encoder {
    pub start_pass: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_compress_ptr,
            _: crate::jmorecfg_h::boolean,
        ) -> (),
    >,
    pub encode_mcu: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_compress_ptr,
            _: *mut crate::jpeglib_h::JBLOCKROW,
        ) -> crate::jmorecfg_h::boolean,
    >,
    pub finish_pass: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> ()>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_marker_writer {
    pub write_file_header: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> ()>,
    pub write_frame_header: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> ()>,
    pub write_scan_header: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> ()>,
    pub write_file_trailer: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> ()>,
    pub write_tables_only: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> ()>,
    pub write_marker_header:
        Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr, _: i32, _: u32) -> ()>,
    pub write_marker_byte:
        Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr, _: i32) -> ()>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_decomp_master {
    pub prepare_for_output_pass:
        Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> ()>,
    pub finish_output_pass:
        Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> ()>,
    pub is_dummy_pass: crate::jmorecfg_h::boolean,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_input_controller {
    pub consume_input: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> i32>,
    pub reset_input_controller:
        Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> ()>,
    pub start_input_pass: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> ()>,
    pub finish_input_pass:
        Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> ()>,
    pub has_multiple_scans: crate::jmorecfg_h::boolean,
    pub eoi_reached: crate::jmorecfg_h::boolean,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_d_main_controller {
    pub start_pass:
        Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr, _: J_BUF_MODE) -> ()>,
    pub process_data: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_decompress_ptr,
            _: crate::jpeglib_h::JSAMPARRAY,
            _: *mut crate::jmorecfg_h::JDIMENSION,
            _: crate::jmorecfg_h::JDIMENSION,
        ) -> (),
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_d_coef_controller {
    pub start_input_pass: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> ()>,
    pub consume_data: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> i32>,
    pub start_output_pass:
        Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> ()>,
    pub decompress_data: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_decompress_ptr,
            _: crate::jpeglib_h::JSAMPIMAGE,
        ) -> i32,
    >,
    pub coef_arrays: *mut crate::jpeglib_h::jvirt_barray_ptr,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_d_post_controller {
    pub start_pass:
        Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr, _: J_BUF_MODE) -> ()>,
    pub post_process_data: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_decompress_ptr,
            _: crate::jpeglib_h::JSAMPIMAGE,
            _: *mut crate::jmorecfg_h::JDIMENSION,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jpeglib_h::JSAMPARRAY,
            _: *mut crate::jmorecfg_h::JDIMENSION,
            _: crate::jmorecfg_h::JDIMENSION,
        ) -> (),
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_marker_reader {
    pub reset_marker_reader:
        Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> ()>,
    pub read_markers: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> i32>,
    pub read_restart_marker: crate::jpeglib_h::jpeg_marker_parser_method,
    pub saw_SOI: crate::jmorecfg_h::boolean,
    pub saw_SOF: crate::jmorecfg_h::boolean,
    pub next_restart_num: i32,
    pub discarded_bytes: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_entropy_decoder {
    pub start_pass: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> ()>,
    pub decode_mcu: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_decompress_ptr,
            _: *mut crate::jpeglib_h::JBLOCKROW,
        ) -> crate::jmorecfg_h::boolean,
    >,
}
pub type inverse_DCT_method_ptr = Option<
    unsafe extern "C" fn(
        _: crate::jpeglib_h::j_decompress_ptr,
        _: *mut crate::jpeglib_h::jpeg_component_info,
        _: crate::jpeglib_h::JCOEFPTR,
        _: crate::jpeglib_h::JSAMPARRAY,
        _: crate::jmorecfg_h::JDIMENSION,
    ) -> (),
>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_inverse_dct {
    pub start_pass: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> ()>,
    pub inverse_DCT: [inverse_DCT_method_ptr; 10],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_upsampler {
    pub start_pass: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> ()>,
    pub upsample: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_decompress_ptr,
            _: crate::jpeglib_h::JSAMPIMAGE,
            _: *mut crate::jmorecfg_h::JDIMENSION,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jpeglib_h::JSAMPARRAY,
            _: *mut crate::jmorecfg_h::JDIMENSION,
            _: crate::jmorecfg_h::JDIMENSION,
        ) -> (),
    >,
    pub need_context_rows: crate::jmorecfg_h::boolean,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_color_deconverter {
    pub start_pass: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> ()>,
    pub color_convert: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_decompress_ptr,
            _: crate::jpeglib_h::JSAMPIMAGE,
            _: crate::jmorecfg_h::JDIMENSION,
            _: crate::jpeglib_h::JSAMPARRAY,
            _: i32,
        ) -> (),
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_color_quantizer {
    pub start_pass: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_decompress_ptr,
            _: crate::jmorecfg_h::boolean,
        ) -> (),
    >,
    pub color_quantize: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_decompress_ptr,
            _: crate::jpeglib_h::JSAMPARRAY,
            _: crate::jpeglib_h::JSAMPARRAY,
            _: i32,
        ) -> (),
    >,
    pub finish_pass: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> ()>,
    pub new_color_map: Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> ()>,
}
