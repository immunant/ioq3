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
pub use crate::src::jpeg_8c::jutils::jpeg_natural_order;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct d_derived_tbl {
    pub maxcode: [crate::jmorecfg_h::INT32; 18],
    pub valoffset: [crate::jmorecfg_h::INT32; 17],
    pub pub_0: *mut crate::jpeglib_h::JHUFF_TBL,
    pub look_nbits: [i32; 256],
    pub look_sym: [crate::jmorecfg_h::UINT8; 256],
}

pub type huff_entropy_ptr = *mut huff_entropy_decoder;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct huff_entropy_decoder {
    pub pub_0: crate::jpegint_h::jpeg_entropy_decoder,
    pub bitstate: bitread_perm_state,
    pub saved: savable_state,
    pub insufficient_data: crate::jmorecfg_h::boolean,
    pub restarts_to_go: u32,
    pub derived_tbls: [*mut d_derived_tbl; 4],
    pub ac_derived_tbl: *mut d_derived_tbl,
    pub dc_derived_tbls: [*mut d_derived_tbl; 4],
    pub ac_derived_tbls: [*mut d_derived_tbl; 4],
    pub dc_cur_tbls: [*mut d_derived_tbl; 10],
    pub ac_cur_tbls: [*mut d_derived_tbl; 10],
    pub coef_limit: [i32; 10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct savable_state {
    pub EOBRUN: u32,
    pub last_dc_val: [i32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bitread_perm_state {
    pub get_buffer: bit_buf_type,
    pub bits_left: i32,
}
/*
 * Fetching the next N bits from the input stream is a time-critical operation
 * for the Huffman decoders.  We implement it with a combination of inline
 * macros and out-of-line subroutines.  Note that N (the number of bits
 * demanded at one time) never exceeds 15 for JPEG use.
 *
 * We read source bytes into get_buffer and dole out bits as needed.
 * If get_buffer already contains enough bits, they are fetched in-line
 * by the macros CHECK_BIT_BUFFER and GET_BITS.  When there aren't enough
 * bits, jpeg_fill_bit_buffer is called; it will attempt to fill get_buffer
 * as full as possible (not just to the number of bits needed; this
 * prefetching reduces the overhead cost of calling jpeg_fill_bit_buffer).
 * Note that jpeg_fill_bit_buffer may return FALSE to indicate suspension.
 * On TRUE return, jpeg_fill_bit_buffer guarantees that get_buffer contains
 * at least the requested number of bits --- dummy zeroes are inserted if
 * necessary.
 */

pub type bit_buf_type = crate::jmorecfg_h::INT32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bitread_working_state {
    pub next_input_byte: *const crate::jmorecfg_h::JOCTET,
    pub bytes_in_buffer: crate::stddef_h::size_t,
    pub get_buffer: bit_buf_type,
    pub bits_left: i32,
    pub cinfo: crate::jpeglib_h::j_decompress_ptr,
}

static mut jpeg_zigzag_order: [[i32; 8]; 8] = [
    [
        0 as i32, 1 as i32, 5 as i32, 6 as i32, 14 as i32, 15 as i32, 27 as i32, 28 as i32,
    ],
    [
        2 as i32, 4 as i32, 7 as i32, 13 as i32, 16 as i32, 26 as i32, 29 as i32, 42 as i32,
    ],
    [
        3 as i32, 8 as i32, 12 as i32, 17 as i32, 25 as i32, 30 as i32, 41 as i32, 43 as i32,
    ],
    [
        9 as i32, 11 as i32, 18 as i32, 24 as i32, 31 as i32, 40 as i32, 44 as i32, 53 as i32,
    ],
    [
        10 as i32, 19 as i32, 23 as i32, 32 as i32, 39 as i32, 45 as i32, 52 as i32, 54 as i32,
    ],
    [
        20 as i32, 22 as i32, 33 as i32, 38 as i32, 46 as i32, 51 as i32, 55 as i32, 60 as i32,
    ],
    [
        21 as i32, 34 as i32, 37 as i32, 47 as i32, 50 as i32, 56 as i32, 59 as i32, 61 as i32,
    ],
    [
        35 as i32, 36 as i32, 48 as i32, 49 as i32, 57 as i32, 58 as i32, 62 as i32, 63 as i32,
    ],
];

static mut jpeg_zigzag_order7: [[i32; 7]; 7] = [
    [
        0 as i32, 1 as i32, 5 as i32, 6 as i32, 14 as i32, 15 as i32, 27 as i32,
    ],
    [
        2 as i32, 4 as i32, 7 as i32, 13 as i32, 16 as i32, 26 as i32, 28 as i32,
    ],
    [
        3 as i32, 8 as i32, 12 as i32, 17 as i32, 25 as i32, 29 as i32, 38 as i32,
    ],
    [
        9 as i32, 11 as i32, 18 as i32, 24 as i32, 30 as i32, 37 as i32, 39 as i32,
    ],
    [
        10 as i32, 19 as i32, 23 as i32, 31 as i32, 36 as i32, 40 as i32, 45 as i32,
    ],
    [
        20 as i32, 22 as i32, 32 as i32, 35 as i32, 41 as i32, 44 as i32, 46 as i32,
    ],
    [
        21 as i32, 33 as i32, 34 as i32, 42 as i32, 43 as i32, 47 as i32, 48 as i32,
    ],
];

static mut jpeg_zigzag_order6: [[i32; 6]; 6] = [
    [0 as i32, 1 as i32, 5 as i32, 6 as i32, 14 as i32, 15 as i32],
    [
        2 as i32, 4 as i32, 7 as i32, 13 as i32, 16 as i32, 25 as i32,
    ],
    [
        3 as i32, 8 as i32, 12 as i32, 17 as i32, 24 as i32, 26 as i32,
    ],
    [
        9 as i32, 11 as i32, 18 as i32, 23 as i32, 27 as i32, 32 as i32,
    ],
    [
        10 as i32, 19 as i32, 22 as i32, 28 as i32, 31 as i32, 33 as i32,
    ],
    [
        20 as i32, 21 as i32, 29 as i32, 30 as i32, 34 as i32, 35 as i32,
    ],
];

static mut jpeg_zigzag_order5: [[i32; 5]; 5] = [
    [0 as i32, 1 as i32, 5 as i32, 6 as i32, 14 as i32],
    [2 as i32, 4 as i32, 7 as i32, 13 as i32, 15 as i32],
    [3 as i32, 8 as i32, 12 as i32, 16 as i32, 21 as i32],
    [9 as i32, 11 as i32, 17 as i32, 20 as i32, 22 as i32],
    [10 as i32, 18 as i32, 19 as i32, 23 as i32, 24 as i32],
];

static mut jpeg_zigzag_order4: [[i32; 4]; 4] = [
    [0 as i32, 1 as i32, 5 as i32, 6 as i32],
    [2 as i32, 4 as i32, 7 as i32, 12 as i32],
    [3 as i32, 8 as i32, 11 as i32, 13 as i32],
    [9 as i32, 10 as i32, 14 as i32, 15 as i32],
];

static mut jpeg_zigzag_order3: [[i32; 3]; 3] = [
    [0 as i32, 1 as i32, 5 as i32],
    [2 as i32, 4 as i32, 6 as i32],
    [3 as i32, 7 as i32, 8 as i32],
];

static mut jpeg_zigzag_order2: [[i32; 2]; 2] = [[0 as i32, 1 as i32], [2 as i32, 3 as i32]];
/*
 * Compute the derived values for a Huffman table.
 * This routine also performs some validation checks on the table.
 */

unsafe extern "C" fn jpeg_make_d_derived_tbl(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut isDC: crate::jmorecfg_h::boolean,
    mut tblno: i32,
    mut pdtbl: *mut *mut d_derived_tbl,
) {
    let mut htbl: *mut crate::jpeglib_h::JHUFF_TBL = 0 as *mut crate::jpeglib_h::JHUFF_TBL;
    let mut dtbl: *mut d_derived_tbl = 0 as *mut d_derived_tbl;
    let mut p: i32 = 0;
    let mut i: i32 = 0;
    let mut l: i32 = 0;
    let mut si: i32 = 0;
    let mut numsymbols: i32 = 0;
    let mut lookbits: i32 = 0;
    let mut ctr: i32 = 0;
    let mut huffsize: [libc::c_char; 257] = [0; 257];
    let mut huffcode: [u32; 257] = [0; 257];
    let mut code: u32 = 0;
    /* Note that huffsize[] and huffcode[] are filled in code-length order,
     * paralleling the order of the symbols themselves in htbl->huffval[].
     */
    /* Find the input Huffman table */
    if tblno < 0 as i32 || tblno >= 4 as i32 {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_NO_HUFF_TABLE as i32;
        (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = tblno;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    htbl = if isDC != 0 {
        (*cinfo).dc_huff_tbl_ptrs[tblno as usize]
    } else {
        (*cinfo).ac_huff_tbl_ptrs[tblno as usize]
    };
    if htbl.is_null() {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_NO_HUFF_TABLE as i32;
        (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = tblno;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Allocate a workspace if we haven't already done so. */
    if (*pdtbl).is_null() {
        *pdtbl = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            1 as i32,
            ::std::mem::size_of::<d_derived_tbl>() as libc::c_ulong,
        ) as *mut d_derived_tbl
    } /* fill in back link */
    dtbl = *pdtbl;
    (*dtbl).pub_0 = htbl;
    /* Figure C.1: make table of Huffman code length for each symbol */
    p = 0 as i32;
    l = 1 as i32;
    while l <= 16 as i32 {
        i = (*htbl).bits[l as usize] as i32;
        if i < 0 as i32 || p + i > 256 as i32 {
            /* protect against table overrun */
            (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_HUFF_TABLE as i32;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
        loop {
            let fresh0 = i;
            i = i - 1;
            if !(fresh0 != 0) {
                break;
            }
            let fresh1 = p;
            p = p + 1;
            huffsize[fresh1 as usize] = l as libc::c_char
        }
        l += 1
    }
    huffsize[p as usize] = 0 as i32 as libc::c_char;
    numsymbols = p;
    /* Figure C.2: generate the codes themselves */
    /* We also validate that the counts represent a legal Huffman code tree. */
    code = 0 as i32 as u32;
    si = huffsize[0 as i32 as usize] as i32;
    p = 0 as i32;
    while huffsize[p as usize] != 0 {
        while huffsize[p as usize] as i32 == si {
            let fresh2 = p;
            p = p + 1;
            huffcode[fresh2 as usize] = code;
            code = code.wrapping_add(1)
        }
        /* code is now 1 more than the last code used for codelength si; but
         * it must still fit in si bits, since no code is allowed to be all ones.
         */
        if code as crate::jmorecfg_h::INT32 >= (1 as i32 as crate::jmorecfg_h::INT32) << si {
            (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_HUFF_TABLE as i32;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
        code <<= 1 as i32;
        si += 1
    }
    /* Figure F.15: generate decoding tables for bit-sequential decoding */
    p = 0 as i32;
    l = 1 as i32;
    while l <= 16 as i32 {
        if (*htbl).bits[l as usize] != 0 {
            /* valoffset[l] = huffval[] index of 1st symbol of code length l,
             * minus the minimum code of length l
             */
            (*dtbl).valoffset[l as usize] =
                p as crate::jmorecfg_h::INT32 - huffcode[p as usize] as crate::jmorecfg_h::INT32;
            p += (*htbl).bits[l as usize] as i32;
            (*dtbl).maxcode[l as usize] =
                huffcode[(p - 1 as i32) as usize] as crate::jmorecfg_h::INT32
        /* maximum code of length l */
        } else {
            (*dtbl).maxcode[l as usize] = -(1 as i32) as crate::jmorecfg_h::INT32
            /* -1 if no codes of this length */
        } /* ensures jpeg_huff_decode terminates */
        l += 1
    }
    (*dtbl).maxcode[17 as i32 as usize] = 0xfffff as isize;
    /* Compute lookahead tables to speed up decoding.
     * First we set all the table entries to 0, indicating "too long";
     * then we iterate through the Huffman codes that are short enough and
     * fill in all the entries that correspond to bit sequences starting
     * with that code.
     */
    crate::stdlib::memset(
        (*dtbl).look_nbits.as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<[i32; 256]>() as libc::c_ulong,
    );
    p = 0 as i32;
    l = 1 as i32;
    while l <= 8 as i32 {
        i = 1 as i32;
        while i <= (*htbl).bits[l as usize] as i32 {
            /* l = current code's length, p = its index in huffcode[] & huffval[]. */
            /* Generate left-justified code followed by all possible bit sequences */
            lookbits = (huffcode[p as usize] << 8 as i32 - l) as i32;
            ctr = (1 as i32) << 8 as i32 - l;
            while ctr > 0 as i32 {
                (*dtbl).look_nbits[lookbits as usize] = l;
                (*dtbl).look_sym[lookbits as usize] = (*htbl).huffval[p as usize];
                lookbits += 1;
                ctr -= 1
            }
            i += 1;
            p += 1
        }
        l += 1
    }
    /* Validate symbols as being reasonable.
     * For AC tables, we make no check, but accept all byte values 0..255.
     * For DC tables, we require the symbols to be in range 0..15.
     * (Tighter bounds could be applied depending on the data depth and mode,
     * but this is sufficient to ensure safe decoding.)
     */
    if isDC != 0 {
        i = 0 as i32;
        while i < numsymbols {
            let mut sym: i32 = (*htbl).huffval[i as usize] as i32;
            if sym < 0 as i32 || sym > 15 as i32 {
                (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_HUFF_TABLE as i32;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            i += 1
        }
    };
}

unsafe extern "C" fn jpeg_fill_bit_buffer(
    mut state: *mut bitread_working_state,
    mut get_buffer: bit_buf_type,
    mut bits_left: i32,
    mut nbits: i32,
) -> crate::jmorecfg_h::boolean
/* Load up the bit buffer to a depth of at least nbits */ {
    /* Copy heavily used state fields into locals (hopefully registers) */
    let mut next_input_byte: *const crate::jmorecfg_h::JOCTET = (*state).next_input_byte;
    let mut bytes_in_buffer: crate::stddef_h::size_t = (*state).bytes_in_buffer;
    let mut cinfo: crate::jpeglib_h::j_decompress_ptr = (*state).cinfo;
    let mut current_block_30: u64;
    /* Attempt to load at least MIN_GET_BITS bits into get_buffer. */
    /* (It is assumed that no request will be for more than that many bits.) */
    /* We fail to do so only if we hit a marker or are forced to suspend. */
    if (*cinfo).unread_marker == 0 as i32 {
        loop {
            if !(bits_left < 32 as i32 - 7 as i32) {
                current_block_30 = 6417057564578538666;
                break;
                /* end while */
            }
            let mut c: i32 = 0;
            /* Attempt to read a byte */
            if bytes_in_buffer == 0 as i32 as libc::c_ulong {
                if Some(
                    (*(*cinfo).src)
                        .fill_input_buffer
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo)
                    == 0
                {
                    return 0 as i32;
                }
                next_input_byte = (*(*cinfo).src).next_input_byte;
                bytes_in_buffer = (*(*cinfo).src).bytes_in_buffer
            }
            bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
            let fresh3 = next_input_byte;
            next_input_byte = next_input_byte.offset(1);
            c = *fresh3 as i32;
            /* If it's 0xFF, check and discard stuffed zero byte */
            if c == 0xff as i32 {
                loop
                /* Loop here to discard any padding FF's on terminating marker,
                 * so that we can save a valid unread_marker value.  NOTE: we will
                 * accept multiple FF's followed by a 0 as meaning a single FF data
                 * byte.  This data pattern is not valid according to the standard.
                 */
                {
                    if bytes_in_buffer == 0 as i32 as libc::c_ulong {
                        if Some(
                            (*(*cinfo).src)
                                .fill_input_buffer
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(cinfo)
                            == 0
                        {
                            return 0 as i32;
                        }
                        next_input_byte = (*(*cinfo).src).next_input_byte;
                        bytes_in_buffer = (*(*cinfo).src).bytes_in_buffer
                    }
                    bytes_in_buffer = bytes_in_buffer.wrapping_sub(1);
                    let fresh4 = next_input_byte;
                    next_input_byte = next_input_byte.offset(1);
                    c = *fresh4 as i32;
                    if !(c == 0xff as i32) {
                        break;
                    }
                }
                if c == 0 as i32 {
                    /* Found FF/00, which represents an FF data byte */
                    c = 0xff as i32
                } else {
                    /* Oops, it's actually a marker indicating end of compressed data.
                     * Save the marker code for later use.
                     * Fine point: it might appear that we should save the marker into
                     * bitread working state, not straight into permanent state.  But
                     * once we have hit a marker, we cannot need to suspend within the
                     * current MCU, because we will read no more bytes from the data
                     * source.  So it is OK to update permanent state right away.
                     */
                    (*cinfo).unread_marker = c;
                    current_block_30 = 5538656556122186398;
                    break;
                }
            }
            /* OK, load c into get_buffer */
            get_buffer = get_buffer << 8 as i32 | c as isize;
            bits_left += 8 as i32
        }
    } else {
        current_block_30 = 5538656556122186398;
    }
    match current_block_30 {
        5538656556122186398 =>
        /* See if we need to insert some fake zero bits. */
        /* We get here if we've read the marker that terminates the compressed
         * data segment.  There should be enough bits in the buffer register
         * to satisfy the request; if so, no problem.
         */
        {
            if nbits > bits_left {
                /* Uh-oh.  Report corrupted data to user and stuff zeroes into
                 * the data stream, so that we can produce some kind of image.
                 * We use a nonvolatile flag to ensure that only one warning message
                 * appears per data segment.
                 */
                if (*((*cinfo).entropy as huff_entropy_ptr)).insufficient_data == 0 {
                    (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JWRN_HIT_MARKER as i32;
                    Some(
                        (*(*cinfo).err)
                            .emit_message
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as crate::jpeglib_h::j_common_ptr,
                        -(1 as i32),
                    );
                    (*((*cinfo).entropy as huff_entropy_ptr)).insufficient_data = 1 as i32
                }
                /* Fill the buffer with zero bits */
                get_buffer <<= 32 as i32 - 7 as i32 - bits_left;
                bits_left = 32 as i32 - 7 as i32
            }
        }
        _ => {}
    }
    /* Unload the local registers */
    (*state).next_input_byte = next_input_byte;
    (*state).bytes_in_buffer = bytes_in_buffer;
    (*state).get_buffer = get_buffer;
    (*state).bits_left = bits_left;
    return 1 as i32;
}
/*
 * Figure F.12: extend sign bit.
 * On some machines, a shift and sub will be faster than a table lookup.
 */

static mut bmask: [i32; 16] = [
    0 as i32,
    0x1 as i32,
    0x3 as i32,
    0x7 as i32,
    0xf as i32,
    0x1f as i32,
    0x3f as i32,
    0x7f as i32,
    0xff as i32,
    0x1ff as i32,
    0x3ff as i32,
    0x7ff as i32,
    0xfff as i32,
    0x1fff as i32,
    0x3fff as i32,
    0x7fff as i32,
];
/* AVOID_TABLES */
/*
 * Out-of-line code for Huffman code decoding.
 */

unsafe extern "C" fn jpeg_huff_decode(
    mut state: *mut bitread_working_state,
    mut get_buffer: bit_buf_type,
    mut bits_left: i32,
    mut htbl: *mut d_derived_tbl,
    mut min_bits: i32,
) -> i32 {
    let mut l: i32 = min_bits;
    let mut code: crate::jmorecfg_h::INT32 = 0;
    /* HUFF_DECODE has determined that the code is at least min_bits */
    /* bits long, so fetch that many bits in one swoop. */
    if bits_left < l {
        if jpeg_fill_bit_buffer(state, get_buffer, bits_left, l) == 0 {
            return -(1 as i32);
        }
        get_buffer = (*state).get_buffer;
        bits_left = (*state).bits_left
    }
    bits_left -= l;
    code = ((get_buffer >> bits_left) as i32 & bmask[l as usize]) as crate::jmorecfg_h::INT32;
    /* Collect the rest of the Huffman code one bit at a time. */
    /* This is per Figure F.16 in the JPEG spec. */
    while code > (*htbl).maxcode[l as usize] {
        code <<= 1 as i32;
        if bits_left < 1 as i32 {
            if jpeg_fill_bit_buffer(state, get_buffer, bits_left, 1 as i32) == 0 {
                return -(1 as i32);
            }
            get_buffer = (*state).get_buffer;
            bits_left = (*state).bits_left
        }
        bits_left -= 1 as i32;
        code |= ((get_buffer >> bits_left) as i32 & bmask[1 as i32 as usize]) as isize;
        l += 1
    }
    /* Unload the local registers */
    (*state).get_buffer = get_buffer;
    (*state).bits_left = bits_left;
    /* With garbage input we may reach the sentinel value l = 17. */
    if l > 16 as i32 {
        (*(*(*state).cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JWRN_HUFF_BAD_CODE as i32;
        Some(
            (*(*(*state).cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            (*state).cinfo as crate::jpeglib_h::j_common_ptr,
            -(1 as i32),
        );
        return 0 as i32;
        /* fake a zero as the safest result */
    }
    return (*(*htbl).pub_0).huffval[(code + (*htbl).valoffset[l as usize]) as i32 as usize] as i32;
}
/*
 * Check for a restart marker & resynchronize decoder.
 * Returns FALSE if must suspend.
 */

unsafe extern "C" fn process_restart(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
) -> crate::jmorecfg_h::boolean {
    let mut entropy: huff_entropy_ptr = (*cinfo).entropy as huff_entropy_ptr;
    let mut ci: i32 = 0;
    /* Throw away any unused bits remaining in bit buffer; */
    /* include any full bytes in next_marker's count of discarded bytes */
    (*(*cinfo).marker).discarded_bytes = (*(*cinfo).marker)
        .discarded_bytes
        .wrapping_add(((*entropy).bitstate.bits_left / 8 as i32) as u32);
    (*entropy).bitstate.bits_left = 0 as i32;
    /* Advance past the RSTn marker */
    if Some(
        (*(*cinfo).marker)
            .read_restart_marker
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo)
        == 0
    {
        return 0 as i32;
    }
    /* Re-initialize DC predictions to 0 */
    ci = 0 as i32;
    while ci < (*cinfo).comps_in_scan {
        (*entropy).saved.last_dc_val[ci as usize] = 0 as i32;
        ci += 1
    }
    /* Re-init EOB run count, too */
    (*entropy).saved.EOBRUN = 0 as i32 as u32;
    /* Reset restart counter */
    (*entropy).restarts_to_go = (*cinfo).restart_interval;
    /* Reset out-of-data flag, unless read_restart_marker left us smack up
     * against a marker.  In that case we will end up treating the next data
     * segment as empty, and we can avoid producing bogus output pixels by
     * leaving the flag set.
     */
    if (*cinfo).unread_marker == 0 as i32 {
        (*entropy).insufficient_data = 0 as i32
    }
    return 1 as i32;
}
/*
 * Huffman MCU decoding.
 * Each of these routines decodes and returns one MCU's worth of
 * Huffman-compressed coefficients.
 * The coefficients are reordered from zigzag order into natural array order,
 * but are not dequantized.
 *
 * The i'th block of the MCU is stored into the block pointed to by
 * MCU_data[i].  WE ASSUME THIS AREA IS INITIALLY ZEROED BY THE CALLER.
 * (Wholesale zeroing is usually a little faster than retail...)
 *
 * We return FALSE if data source requested suspension.  In that case no
 * changes have been made to permanent state.  (Exception: some output
 * coefficients may already have been assigned.  This is harmless for
 * spectral selection, since we'll just re-assign them on the next call.
 * Successive approximation AC refinement has to be more careful, however.)
 */
/*
 * MCU decoding for DC initial scan (either spectral selection,
 * or first pass of successive approximation).
 */

unsafe extern "C" fn decode_mcu_DC_first(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut MCU_data: *mut crate::jpeglib_h::JBLOCKROW,
) -> crate::jmorecfg_h::boolean {
    let mut entropy: huff_entropy_ptr = (*cinfo).entropy as huff_entropy_ptr;
    let mut Al: i32 = (*cinfo).Al;
    let mut s: i32 = 0;
    let mut r: i32 = 0;
    let mut blkn: i32 = 0;
    let mut ci: i32 = 0;
    let mut block: crate::jpeglib_h::JBLOCKROW = 0 as *mut crate::jpeglib_h::JBLOCK;
    let mut get_buffer: bit_buf_type = 0;
    let mut bits_left: i32 = 0;
    let mut br_state: bitread_working_state = bitread_working_state {
        next_input_byte: 0 as *const crate::jmorecfg_h::JOCTET,
        bytes_in_buffer: 0,
        get_buffer: 0,
        bits_left: 0,
        cinfo: 0 as *mut crate::jpeglib_h::jpeg_decompress_struct,
    };
    let mut state: savable_state = savable_state {
        EOBRUN: 0,
        last_dc_val: [0; 4],
    };
    let mut tbl: *mut d_derived_tbl = 0 as *mut d_derived_tbl;
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        0 as *mut crate::jpeglib_h::jpeg_component_info;
    /* Process restart marker if needed; may have to suspend */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0 as i32 as u32 {
            if process_restart(cinfo) == 0 {
                return 0 as i32;
            }
        }
    }
    /* If we've run out of data, just leave the MCU set to zeroes.
     * This way, we return uniform gray for the remainder of the segment.
     */
    if (*entropy).insufficient_data == 0 {
        /* Load up working state */
        br_state.cinfo = cinfo;
        br_state.next_input_byte = (*(*cinfo).src).next_input_byte;
        br_state.bytes_in_buffer = (*(*cinfo).src).bytes_in_buffer;
        get_buffer = (*entropy).bitstate.get_buffer;
        bits_left = (*entropy).bitstate.bits_left;
        state = (*entropy).saved;
        /* Outer loop handles each block in the MCU */
        blkn = 0 as i32;
        while blkn < (*cinfo).blocks_in_MCU {
            block = *MCU_data.offset(blkn as isize);
            ci = (*cinfo).MCU_membership[blkn as usize];
            compptr = (*cinfo).cur_comp_info[ci as usize];
            tbl = (*entropy).derived_tbls[(*compptr).dc_tbl_no as usize];
            let mut current_block_32: u64;
            /* Decode a single block's worth of coefficients */
            /* Section F.2.2.1: decode the DC coefficient difference */
            let mut nb: i32 = 0;
            let mut look: i32 = 0;
            if bits_left < 8 as i32 {
                if jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, 0 as i32) == 0 {
                    return 0 as i32;
                }
                get_buffer = br_state.get_buffer;
                bits_left = br_state.bits_left;
                if bits_left < 8 as i32 {
                    nb = 1 as i32;
                    current_block_32 = 18265024319667395811;
                } else {
                    current_block_32 = 5494826135382683477;
                }
            } else {
                current_block_32 = 5494826135382683477;
            }
            match current_block_32 {
                5494826135382683477 => {
                    look = (get_buffer >> bits_left - 8 as i32) as i32 & bmask[8 as i32 as usize];
                    nb = (*tbl).look_nbits[look as usize];
                    if nb != 0 as i32 {
                        bits_left -= nb;
                        s = (*tbl).look_sym[look as usize] as i32;
                        current_block_32 = 5330834795799507926;
                    } else {
                        nb = 8 as i32 + 1 as i32;
                        current_block_32 = 18265024319667395811;
                    }
                }
                _ => {}
            }
            match current_block_32 {
                18265024319667395811 => {
                    s = jpeg_huff_decode(&mut br_state, get_buffer, bits_left, tbl, nb);
                    if s < 0 as i32 {
                        return 0 as i32;
                    }
                    get_buffer = br_state.get_buffer;
                    bits_left = br_state.bits_left
                }
                _ => {}
            }
            if s != 0 {
                if bits_left < s {
                    if jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, s) == 0 {
                        return 0 as i32;
                    }
                    get_buffer = br_state.get_buffer;
                    bits_left = br_state.bits_left
                }
                bits_left -= s;
                r = (get_buffer >> bits_left) as i32 & bmask[s as usize];
                s = if r <= bmask[(s - 1 as i32) as usize] {
                    (r) - bmask[s as usize]
                } else {
                    r
                }
            }
            /* Convert DC difference to actual value, update last_dc_val */
            s += state.last_dc_val[ci as usize];
            state.last_dc_val[ci as usize] = s;
            /* Scale and output the coefficient (assumes jpeg_natural_order[0]=0) */
            (*block)[0 as i32 as usize] = (s << Al) as crate::jmorecfg_h::JCOEF;
            blkn += 1
        }
        /* Completed MCU, so update state */
        (*(*cinfo).src).next_input_byte = br_state.next_input_byte;
        (*(*cinfo).src).bytes_in_buffer = br_state.bytes_in_buffer;
        (*entropy).bitstate.get_buffer = get_buffer;
        (*entropy).bitstate.bits_left = bits_left;
        (*entropy).saved = state
    }
    /* Account for restart interval (no-op if not using restarts) */
    (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1);
    return 1 as i32;
}
/*
 * MCU decoding for AC initial scan (either spectral selection,
 * or first pass of successive approximation).
 */

unsafe extern "C" fn decode_mcu_AC_first(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut MCU_data: *mut crate::jpeglib_h::JBLOCKROW,
) -> crate::jmorecfg_h::boolean {
    let mut entropy: huff_entropy_ptr = (*cinfo).entropy as huff_entropy_ptr;
    let mut s: i32 = 0;
    let mut k: i32 = 0;
    let mut r: i32 = 0;
    let mut EOBRUN: u32 = 0;
    let mut Se: i32 = 0;
    let mut Al: i32 = 0;
    let mut natural_order: *const i32 = 0 as *const i32;
    let mut block: crate::jpeglib_h::JBLOCKROW = 0 as *mut crate::jpeglib_h::JBLOCK;
    let mut get_buffer: bit_buf_type = 0;
    let mut bits_left: i32 = 0;
    let mut br_state: bitread_working_state = bitread_working_state {
        next_input_byte: 0 as *const crate::jmorecfg_h::JOCTET,
        bytes_in_buffer: 0,
        get_buffer: 0,
        bits_left: 0,
        cinfo: 0 as *mut crate::jpeglib_h::jpeg_decompress_struct,
    };
    let mut tbl: *mut d_derived_tbl = 0 as *mut d_derived_tbl;
    /* Process restart marker if needed; may have to suspend */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0 as i32 as u32 {
            if process_restart(cinfo) == 0 {
                return 0 as i32;
            }
        }
    }
    /* If we've run out of data, just leave the MCU set to zeroes.
     * This way, we return uniform gray for the remainder of the segment.
     */
    if (*entropy).insufficient_data == 0 {
        Se = (*cinfo).Se;
        Al = (*cinfo).Al;
        natural_order = (*cinfo).natural_order;
        /* only part of saved state we need */
        EOBRUN = (*entropy).saved.EOBRUN;
        if EOBRUN > 0 as i32 as u32 {
            /* Load up working state.
             * We can avoid loading/saving bitread state if in an EOB run.
             */
            /* only part of saved state we need */
            /* There is always only one block per MCU */
            /* if it's a band of zeroes... */
            EOBRUN = EOBRUN.wrapping_sub(1)
        } else {
            br_state.cinfo = cinfo; /* ...process it now (we do nothing) */
            br_state.next_input_byte = (*(*cinfo).src).next_input_byte;
            br_state.bytes_in_buffer = (*(*cinfo).src).bytes_in_buffer;
            get_buffer = (*entropy).bitstate.get_buffer;
            bits_left = (*entropy).bitstate.bits_left;
            block = *MCU_data.offset(0 as i32 as isize);
            tbl = (*entropy).ac_derived_tbl;
            k = (*cinfo).Ss;
            while k <= Se {
                let mut current_block_34: u64;
                let mut nb: i32 = 0;
                let mut look: i32 = 0;
                if bits_left < 8 as i32 {
                    if jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, 0 as i32) == 0 {
                        return 0 as i32;
                    }
                    get_buffer = br_state.get_buffer;
                    bits_left = br_state.bits_left;
                    if bits_left < 8 as i32 {
                        nb = 1 as i32;
                        current_block_34 = 5158729086098965353;
                    } else {
                        current_block_34 = 14136749492126903395;
                    }
                } else {
                    current_block_34 = 14136749492126903395;
                }
                match current_block_34 {
                    14136749492126903395 => {
                        look =
                            (get_buffer >> bits_left - 8 as i32) as i32 & bmask[8 as i32 as usize];
                        nb = (*tbl).look_nbits[look as usize];
                        if nb != 0 as i32 {
                            bits_left -= nb;
                            s = (*tbl).look_sym[look as usize] as i32;
                            current_block_34 = 1847472278776910194;
                        } else {
                            nb = 8 as i32 + 1 as i32;
                            current_block_34 = 5158729086098965353;
                        }
                    }
                    _ => {}
                }
                match current_block_34 {
                    5158729086098965353 => {
                        s = jpeg_huff_decode(&mut br_state, get_buffer, bits_left, tbl, nb);
                        if s < 0 as i32 {
                            return 0 as i32;
                        }
                        get_buffer = br_state.get_buffer;
                        bits_left = br_state.bits_left
                    }
                    _ => {}
                }
                r = s >> 4 as i32;
                s &= 15 as i32;
                if s != 0 {
                    k += r;
                    if bits_left < s {
                        if jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, s) == 0 {
                            return 0 as i32;
                        }
                        get_buffer = br_state.get_buffer;
                        bits_left = br_state.bits_left
                    }
                    bits_left -= s;
                    r = (get_buffer >> bits_left) as i32 & bmask[s as usize];
                    s = if r <= bmask[(s - 1 as i32) as usize] {
                        (r) - bmask[s as usize]
                    } else {
                        r
                    };
                    /* Scale and output coefficient in natural (dezigzagged) order */
                    (*block)[*natural_order.offset(k as isize) as usize] =
                        (s << Al) as crate::jmorecfg_h::JCOEF
                } else if r == 15 as i32 {
                    /* ZRL */
                    k += 15 as i32
                /* skip 15 zeroes in band */
                } else {
                    EOBRUN = ((1 as i32) << r) as u32;
                    if r != 0 {
                        /* force end-of-band */
                        /* EOBr, r > 0 */
                        if bits_left < r {
                            if jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, r) == 0 {
                                return 0 as i32;
                            } /* this band is processed at this moment */
                            get_buffer = br_state.get_buffer;
                            bits_left = br_state.bits_left
                        }
                        bits_left -= r;
                        r = (get_buffer >> bits_left) as i32 & bmask[r as usize];
                        EOBRUN = EOBRUN.wrapping_add(r as u32)
                    }
                    EOBRUN = EOBRUN.wrapping_sub(1);
                    break;
                }
                k += 1
            }
            (*(*cinfo).src).next_input_byte = br_state.next_input_byte;
            (*(*cinfo).src).bytes_in_buffer = br_state.bytes_in_buffer;
            (*entropy).bitstate.get_buffer = get_buffer;
            (*entropy).bitstate.bits_left = bits_left
        }
        (*entropy).saved.EOBRUN = EOBRUN
    }
    /* Completed MCU, so update state */
    /* Account for restart interval (no-op if not using restarts) */
    (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1);
    return 1 as i32;
}
/*
 * MCU decoding for DC successive approximation refinement scan.
 * Note: we assume such scans can be multi-component, although the spec
 * is not very clear on the point.
 */

unsafe extern "C" fn decode_mcu_DC_refine(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut MCU_data: *mut crate::jpeglib_h::JBLOCKROW,
) -> crate::jmorecfg_h::boolean {
    let mut entropy: huff_entropy_ptr = (*cinfo).entropy as huff_entropy_ptr; /* 1 in the bit position being coded */
    let mut p1: i32 = (1 as i32) << (*cinfo).Al;
    let mut blkn: i32 = 0;
    let mut block: crate::jpeglib_h::JBLOCKROW = 0 as *mut crate::jpeglib_h::JBLOCK;
    let mut get_buffer: bit_buf_type = 0;
    let mut bits_left: i32 = 0;
    let mut br_state: bitread_working_state = bitread_working_state {
        next_input_byte: 0 as *const crate::jmorecfg_h::JOCTET,
        bytes_in_buffer: 0,
        get_buffer: 0,
        bits_left: 0,
        cinfo: 0 as *mut crate::jpeglib_h::jpeg_decompress_struct,
    };
    /* Process restart marker if needed; may have to suspend */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0 as i32 as u32 {
            if process_restart(cinfo) == 0 {
                return 0 as i32;
            }
        }
    }
    /* Not worth the cycles to check insufficient_data here,
     * since we will not change the data anyway if we read zeroes.
     */
    /* Load up working state */
    br_state.cinfo = cinfo;
    br_state.next_input_byte = (*(*cinfo).src).next_input_byte;
    br_state.bytes_in_buffer = (*(*cinfo).src).bytes_in_buffer;
    get_buffer = (*entropy).bitstate.get_buffer;
    bits_left = (*entropy).bitstate.bits_left;
    /* Outer loop handles each block in the MCU */
    blkn = 0 as i32;
    while blkn < (*cinfo).blocks_in_MCU {
        block = *MCU_data.offset(blkn as isize);
        /* Note: since we use |=, repeating the assignment later is safe */
        if bits_left < 1 as i32 {
            if jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, 1 as i32) == 0 {
                return 0 as i32;
            }
            get_buffer = br_state.get_buffer;
            bits_left = br_state.bits_left
        }
        bits_left -= 1 as i32;
        if (get_buffer >> bits_left) as i32 & bmask[1 as i32 as usize] != 0 {
            (*block)[0 as i32 as usize] =
                ((*block)[0 as i32 as usize] as i32 | p1) as crate::jmorecfg_h::JCOEF
        }
        blkn += 1
    }
    /* Encoded data is simply the next bit of the two's-complement DC value */
    /* Completed MCU, so update state */
    (*(*cinfo).src).next_input_byte = br_state.next_input_byte;
    (*(*cinfo).src).bytes_in_buffer = br_state.bytes_in_buffer;
    (*entropy).bitstate.get_buffer = get_buffer;
    (*entropy).bitstate.bits_left = bits_left;
    /* Account for restart interval (no-op if not using restarts) */
    (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1);
    return 1 as i32;
}
/*
 * MCU decoding for AC successive approximation refinement scan.
 */

unsafe extern "C" fn decode_mcu_AC_refine(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut MCU_data: *mut crate::jpeglib_h::JBLOCKROW,
) -> crate::jmorecfg_h::boolean {
    let mut current_block: u64;
    let mut entropy: huff_entropy_ptr = (*cinfo).entropy as huff_entropy_ptr;
    let mut s: i32 = 0;
    let mut k: i32 = 0;
    let mut r: i32 = 0;
    let mut EOBRUN: u32 = 0;
    let mut Se: i32 = 0;
    let mut p1: i32 = 0;
    let mut m1: i32 = 0;
    let mut natural_order: *const i32 = 0 as *const i32;
    let mut block: crate::jpeglib_h::JBLOCKROW = 0 as *mut crate::jpeglib_h::JBLOCK;
    let mut thiscoef: crate::jpeglib_h::JCOEFPTR = 0 as *mut crate::jmorecfg_h::JCOEF;
    let mut get_buffer: bit_buf_type = 0;
    let mut bits_left: i32 = 0;
    let mut br_state: bitread_working_state = bitread_working_state {
        next_input_byte: 0 as *const crate::jmorecfg_h::JOCTET,
        bytes_in_buffer: 0,
        get_buffer: 0,
        bits_left: 0,
        cinfo: 0 as *mut crate::jpeglib_h::jpeg_decompress_struct,
    };
    let mut tbl: *mut d_derived_tbl = 0 as *mut d_derived_tbl;
    let mut num_newnz: i32 = 0;
    let mut newnz_pos: [i32; 64] = [0; 64];
    /* Process restart marker if needed; may have to suspend */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0 as i32 as u32 {
            if process_restart(cinfo) == 0 {
                return 0 as i32;
            }
        }
    }
    /* If we've run out of data, don't modify the MCU.
     */
    if (*entropy).insufficient_data == 0 {
        Se = (*cinfo).Se;
        /* only part of saved state we need */
        p1 = (1 as i32) << (*cinfo).Al; /* 1 in the bit position being coded */
        m1 = (-(1 as i32)) << (*cinfo).Al; /* -1 in the bit position being coded */
        natural_order = (*cinfo).natural_order;
        /* Load up working state */
        br_state.cinfo = cinfo; /* only part of saved state we need */
        br_state.next_input_byte = (*(*cinfo).src).next_input_byte;
        br_state.bytes_in_buffer = (*(*cinfo).src).bytes_in_buffer;
        get_buffer = (*entropy).bitstate.get_buffer;
        bits_left = (*entropy).bitstate.bits_left;
        EOBRUN = (*entropy).saved.EOBRUN;
        /* There is always only one block per MCU */
        block = *MCU_data.offset(0 as i32 as isize);
        tbl = (*entropy).ac_derived_tbl;
        /* If we are forced to suspend, we must undo the assignments to any newly
         * nonzero coefficients in the block, because otherwise we'd get confused
         * next time about which coefficients were already nonzero.
         * But we need not undo addition of bits to already-nonzero coefficients;
         * instead, we can test the current bit to see if we already did it.
         */
        num_newnz = 0 as i32;
        /* initialize coefficient loop counter to start of band */
        k = (*cinfo).Ss;
        if EOBRUN == 0 as i32 as u32 {
            current_block = 4068382217303356765;
        } else {
            current_block = 1658462350791934405;
        }
        's_133: loop {
            match current_block {
                1658462350791934405 => {
                    if EOBRUN > 0 as i32 as u32 {
                        current_block = 5700653730392116747;
                        break;
                    } else {
                        current_block = 13505557363059842426;
                        break;
                    }
                }
                _ => {
                    if !(k <= Se) {
                        current_block = 1658462350791934405;
                        continue;
                    }
                    let mut nb: i32 = 0;
                    let mut look: i32 = 0;
                    if bits_left < 8 as i32 {
                        if jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, 0 as i32) == 0
                        {
                            current_block = 6568878844838934825;
                            break;
                        }
                        get_buffer = br_state.get_buffer;
                        bits_left = br_state.bits_left;
                        if bits_left < 8 as i32 {
                            nb = 1 as i32;
                            current_block = 916363241452857900;
                        } else {
                            current_block = 17184638872671510253;
                        }
                    } else {
                        current_block = 17184638872671510253;
                    }
                    match current_block {
                        17184638872671510253 => {
                            look = (get_buffer >> bits_left - 8 as i32) as i32
                                & bmask[8 as i32 as usize];
                            nb = (*tbl).look_nbits[look as usize];
                            if nb != 0 as i32 {
                                bits_left -= nb;
                                s = (*tbl).look_sym[look as usize] as i32;
                                current_block = 1345366029464561491;
                            } else {
                                nb = 8 as i32 + 1 as i32;
                                current_block = 916363241452857900;
                            }
                        }
                        _ => {}
                    }
                    match current_block {
                        916363241452857900 => {
                            s = jpeg_huff_decode(&mut br_state, get_buffer, bits_left, tbl, nb);
                            if s < 0 as i32 {
                                current_block = 6568878844838934825;
                                break;
                            }
                            get_buffer = br_state.get_buffer;
                            bits_left = br_state.bits_left
                        }
                        _ => {}
                    }
                    r = s >> 4 as i32;
                    s &= 15 as i32;
                    if s != 0 {
                        if s != 1 as i32 {
                            /* size of new coef should always be 1 */
                            (*(*cinfo).err).msg_code =
                                crate::src::jpeg_8c::jerror::JWRN_HUFF_BAD_CODE as i32;
                            Some(
                                (*(*cinfo).err)
                                    .emit_message
                                    .expect("non-null function pointer"),
                            )
                            .expect("non-null function pointer")(
                                cinfo as crate::jpeglib_h::j_common_ptr,
                                -(1 as i32),
                            );
                        }
                        if bits_left < 1 as i32 {
                            if jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, 1 as i32)
                                == 0
                            {
                                current_block = 6568878844838934825;
                                break;
                            }
                            get_buffer = br_state.get_buffer;
                            bits_left = br_state.bits_left
                        }
                        bits_left -= 1 as i32;
                        if (get_buffer >> bits_left) as i32 & bmask[1 as i32 as usize] != 0 {
                            /* newly nonzero coef is negative */
                            s = p1
                        } else {
                            s = m1
                        }
                    } else if r != 15 as i32 {
                        /* newly nonzero coef is positive */
                        EOBRUN = ((1 as i32) << r) as u32;
                        if !(r != 0) {
                            current_block = 1658462350791934405; /* EOBr, run length is 2^r + appended bits */
                            continue;
                        }
                        if bits_left < r {
                            if jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, r) == 0 {
                                current_block = 6568878844838934825;
                                break;
                            }
                            get_buffer = br_state.get_buffer;
                            bits_left = br_state.bits_left
                        }
                        bits_left -= r;
                        r = (get_buffer >> bits_left) as i32 & bmask[r as usize];
                        EOBRUN = EOBRUN.wrapping_add(r as u32);
                        current_block = 1658462350791934405;
                        continue;
                        /* rest of block is handled by EOB logic */
                    }
                    loop
                    /* note s = 0 for processing ZRL */
                    /* Advance over already-nonzero coefs and r still-zero coefs,
                     * appending correction bits to the nonzeroes.  A correction bit is 1
                     * if the absolute value of the coefficient must be increased.
                     */
                    {
                        thiscoef = (*block)
                            .as_mut_ptr()
                            .offset(*natural_order.offset(k as isize) as isize);
                        if *thiscoef as i32 != 0 as i32 {
                            if bits_left < 1 as i32 {
                                if jpeg_fill_bit_buffer(
                                    &mut br_state,
                                    get_buffer,
                                    bits_left,
                                    1 as i32,
                                ) == 0
                                {
                                    current_block = 6568878844838934825;
                                    break 's_133;
                                }
                                get_buffer = br_state.get_buffer;
                                bits_left = br_state.bits_left
                            }
                            bits_left -= 1 as i32;
                            if (get_buffer >> bits_left) as i32 & bmask[1 as i32 as usize] != 0 {
                                if *thiscoef as i32 & p1 == 0 as i32 {
                                    /* do nothing if already set it */
                                    if *thiscoef as i32 >= 0 as i32 {
                                        *thiscoef =
                                            (*thiscoef as i32 + p1) as crate::jmorecfg_h::JCOEF
                                    } else {
                                        *thiscoef =
                                            (*thiscoef as i32 + m1) as crate::jmorecfg_h::JCOEF
                                    }
                                }
                            }
                        } else {
                            r -= 1;
                            if r < 0 as i32 {
                                break;
                                /* reached target zero coefficient */
                            }
                        }
                        k += 1;
                        if !(k <= Se) {
                            break;
                        }
                    }
                    if s != 0 {
                        let mut pos: i32 = *natural_order.offset(k as isize);
                        /* Output newly nonzero coefficient */
                        (*block)[pos as usize] = s as crate::jmorecfg_h::JCOEF;
                        /* Remember its position in case we have to suspend */
                        let fresh5 = num_newnz;
                        num_newnz = num_newnz + 1;
                        newnz_pos[fresh5 as usize] = pos
                    }
                    k += 1;
                    current_block = 4068382217303356765;
                }
            }
        }
        loop {
            match current_block {
                6568878844838934825 => {
                    /* Re-zero any output coefficients that we made newly nonzero */
                    while num_newnz > 0 as i32 {
                        num_newnz -= 1;
                        (*block)[newnz_pos[num_newnz as usize] as usize] =
                            0 as i32 as crate::jmorecfg_h::JCOEF
                    }
                    return 0 as i32;
                }
                5700653730392116747 =>
                /* Scan any remaining coefficient positions after the end-of-band
                 * (the last newly nonzero coefficient, if any).  Append a correction
                 * bit to each already-nonzero coefficient.  A correction bit is 1
                 * if the absolute value of the coefficient must be increased.
                 */
                {
                    if k <= Se {
                        thiscoef = (*block)
                            .as_mut_ptr()
                            .offset(*natural_order.offset(k as isize) as isize);
                        if *thiscoef as i32 != 0 as i32 {
                            if bits_left < 1 as i32 {
                                if jpeg_fill_bit_buffer(
                                    &mut br_state,
                                    get_buffer,
                                    bits_left,
                                    1 as i32,
                                ) == 0
                                {
                                    current_block = 6568878844838934825;
                                    continue;
                                }
                                get_buffer = br_state.get_buffer;
                                bits_left = br_state.bits_left
                            }
                            bits_left -= 1 as i32;
                            if (get_buffer >> bits_left) as i32 & bmask[1 as i32 as usize] != 0 {
                                if *thiscoef as i32 & p1 == 0 as i32 {
                                    /* do nothing if already changed it */
                                    if *thiscoef as i32 >= 0 as i32 {
                                        *thiscoef =
                                            (*thiscoef as i32 + p1) as crate::jmorecfg_h::JCOEF
                                    } else {
                                        *thiscoef =
                                            (*thiscoef as i32 + m1) as crate::jmorecfg_h::JCOEF
                                    }
                                }
                            }
                        }
                        k += 1;
                        current_block = 5700653730392116747;
                    } else {
                        /* Count one block completed in EOB run */
                        EOBRUN = EOBRUN.wrapping_sub(1);
                        current_block = 13505557363059842426;
                    }
                }
                _ => {
                    /* Completed MCU, so update state */
                    (*(*cinfo).src).next_input_byte = br_state.next_input_byte;
                    (*(*cinfo).src).bytes_in_buffer = br_state.bytes_in_buffer;
                    (*entropy).bitstate.get_buffer = get_buffer;
                    (*entropy).bitstate.bits_left = bits_left;
                    (*entropy).saved.EOBRUN = EOBRUN;
                    break;
                }
            }
        }
    }
    /* Account for restart interval (no-op if not using restarts) */
    (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1);
    return 1 as i32;
}
/*
 * Decode one MCU's worth of Huffman-compressed coefficients,
 * partial blocks.
 */

unsafe extern "C" fn decode_mcu_sub(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut MCU_data: *mut crate::jpeglib_h::JBLOCKROW,
) -> crate::jmorecfg_h::boolean {
    let mut entropy: huff_entropy_ptr = (*cinfo).entropy as huff_entropy_ptr;
    let mut natural_order: *const i32 = 0 as *const i32;
    let mut Se: i32 = 0;
    let mut blkn: i32 = 0;
    let mut get_buffer: bit_buf_type = 0;
    let mut bits_left: i32 = 0;
    let mut br_state: bitread_working_state = bitread_working_state {
        next_input_byte: 0 as *const crate::jmorecfg_h::JOCTET,
        bytes_in_buffer: 0,
        get_buffer: 0,
        bits_left: 0,
        cinfo: 0 as *mut crate::jpeglib_h::jpeg_decompress_struct,
    };
    let mut state: savable_state = savable_state {
        EOBRUN: 0,
        last_dc_val: [0; 4],
    };
    /* Process restart marker if needed; may have to suspend */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0 as i32 as u32 {
            if process_restart(cinfo) == 0 {
                return 0 as i32;
            }
        }
    }
    /* If we've run out of data, just leave the MCU set to zeroes.
     * This way, we return uniform gray for the remainder of the segment.
     */
    if (*entropy).insufficient_data == 0 {
        natural_order = (*cinfo).natural_order;
        Se = (*cinfo).lim_Se;
        /* Load up working state */
        br_state.cinfo = cinfo;
        br_state.next_input_byte = (*(*cinfo).src).next_input_byte;
        br_state.bytes_in_buffer = (*(*cinfo).src).bytes_in_buffer;
        get_buffer = (*entropy).bitstate.get_buffer;
        bits_left = (*entropy).bitstate.bits_left;
        state = (*entropy).saved;
        /* Outer loop handles each block in the MCU */
        blkn = 0 as i32;
        while blkn < (*cinfo).blocks_in_MCU {
            let mut current_block_136: u64;
            let mut block: crate::jpeglib_h::JBLOCKROW = *MCU_data.offset(blkn as isize);
            let mut htbl: *mut d_derived_tbl = 0 as *mut d_derived_tbl;
            let mut s: i32 = 0;
            let mut k: i32 = 0;
            let mut r: i32 = 0;
            let mut coef_limit: i32 = 0;
            let mut ci: i32 = 0;
            /* Decode a single block's worth of coefficients */
            /* Section F.2.2.1: decode the DC coefficient difference */
            htbl = (*entropy).dc_cur_tbls[blkn as usize];
            let mut current_block_31: u64;
            let mut nb: i32 = 0;
            let mut look: i32 = 0;
            if bits_left < 8 as i32 {
                if jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, 0 as i32) == 0 {
                    return 0 as i32;
                }
                get_buffer = br_state.get_buffer;
                bits_left = br_state.bits_left;
                if bits_left < 8 as i32 {
                    nb = 1 as i32;
                    current_block_31 = 16828568275618539777;
                } else {
                    current_block_31 = 11636175345244025579;
                }
            } else {
                current_block_31 = 11636175345244025579;
            }
            match current_block_31 {
                11636175345244025579 => {
                    look = (get_buffer >> bits_left - 8 as i32) as i32 & bmask[8 as i32 as usize];
                    nb = (*htbl).look_nbits[look as usize];
                    if nb != 0 as i32 {
                        bits_left -= nb;
                        s = (*htbl).look_sym[look as usize] as i32;
                        current_block_31 = 17784502470059252271;
                    } else {
                        nb = 8 as i32 + 1 as i32;
                        current_block_31 = 16828568275618539777;
                    }
                }
                _ => {}
            }
            match current_block_31 {
                16828568275618539777 => {
                    s = jpeg_huff_decode(&mut br_state, get_buffer, bits_left, htbl, nb);
                    if s < 0 as i32 {
                        return 0 as i32;
                    }
                    get_buffer = br_state.get_buffer;
                    bits_left = br_state.bits_left
                }
                _ => {}
            }
            htbl = (*entropy).ac_cur_tbls[blkn as usize];
            k = 1 as i32;
            coef_limit = (*entropy).coef_limit[blkn as usize];
            if coef_limit != 0 {
                /* Convert DC difference to actual value, update last_dc_val */
                if s != 0 {
                    if bits_left < s {
                        if jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, s) == 0 {
                            return 0 as i32;
                        }
                        get_buffer = br_state.get_buffer;
                        bits_left = br_state.bits_left
                    }
                    bits_left -= s;
                    r = (get_buffer >> bits_left) as i32 & bmask[s as usize];
                    s = if r <= bmask[(s - 1 as i32) as usize] {
                        (r) - bmask[s as usize]
                    } else {
                        r
                    }
                }
                ci = (*cinfo).MCU_membership[blkn as usize];
                s += state.last_dc_val[ci as usize];
                state.last_dc_val[ci as usize] = s;
                /* Output the DC coefficient */
                (*block)[0 as i32 as usize] = s as crate::jmorecfg_h::JCOEF;
                loop
                /* Section F.2.2.2: decode the AC coefficients */
                /* Since zeroes are skipped, output area must be cleared beforehand */
                {
                    if !(k < coef_limit) {
                        current_block_136 = 5195798230510548452;
                        break;
                    }
                    let mut current_block_69: u64;
                    let mut nb_0: i32 = 0;
                    let mut look_0: i32 = 0;
                    if bits_left < 8 as i32 {
                        if jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, 0 as i32) == 0
                        {
                            return 0 as i32;
                        }
                        get_buffer = br_state.get_buffer;
                        bits_left = br_state.bits_left;
                        if bits_left < 8 as i32 {
                            nb_0 = 1 as i32;
                            current_block_69 = 8502954685593403254;
                        } else {
                            current_block_69 = 15462640364611497761;
                        }
                    } else {
                        current_block_69 = 15462640364611497761;
                    }
                    match current_block_69 {
                        15462640364611497761 => {
                            look_0 = (get_buffer >> bits_left - 8 as i32) as i32
                                & bmask[8 as i32 as usize];
                            nb_0 = (*htbl).look_nbits[look_0 as usize];
                            if nb_0 != 0 as i32 {
                                bits_left -= nb_0;
                                s = (*htbl).look_sym[look_0 as usize] as i32;
                                current_block_69 = 576355610076403033;
                            } else {
                                nb_0 = 8 as i32 + 1 as i32;
                                current_block_69 = 8502954685593403254;
                            }
                        }
                        _ => {}
                    }
                    match current_block_69 {
                        8502954685593403254 => {
                            s = jpeg_huff_decode(&mut br_state, get_buffer, bits_left, htbl, nb_0);
                            if s < 0 as i32 {
                                return 0 as i32;
                            }
                            get_buffer = br_state.get_buffer;
                            bits_left = br_state.bits_left
                        }
                        _ => {}
                    }
                    r = s >> 4 as i32;
                    s &= 15 as i32;
                    if s != 0 {
                        k += r;
                        if bits_left < s {
                            if jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, s) == 0 {
                                return 0 as i32;
                            }
                            get_buffer = br_state.get_buffer;
                            bits_left = br_state.bits_left
                        }
                        bits_left -= s;
                        r = (get_buffer >> bits_left) as i32 & bmask[s as usize];
                        s = if r <= bmask[(s - 1 as i32) as usize] {
                            (r) - bmask[s as usize]
                        } else {
                            r
                        };
                        /* Output coefficient in natural (dezigzagged) order.
                         * Note: the extra entries in natural_order[] will save us
                         * if k > Se, which could happen if the data is corrupted.
                         */
                        (*block)[*natural_order.offset(k as isize) as usize] =
                            s as crate::jmorecfg_h::JCOEF
                    } else {
                        if r != 15 as i32 {
                            current_block_136 = 2544535129495155983;
                            break;
                        }
                        k += 15 as i32
                    }
                    k += 1
                }
            } else {
                if s != 0 {
                    if bits_left < s {
                        if jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, s) == 0 {
                            return 0 as i32;
                        }
                        get_buffer = br_state.get_buffer;
                        bits_left = br_state.bits_left
                    }
                    bits_left -= s
                }
                current_block_136 = 5195798230510548452;
            }
            match current_block_136 {
                5195798230510548452 => {
                    /* Section F.2.2.2: decode the AC coefficients */
                    /* In this path we just discard the values */
                    while k <= Se {
                        let mut current_block_117: u64;
                        let mut nb_1: i32 = 0;
                        let mut look_1: i32 = 0;
                        if bits_left < 8 as i32 {
                            if jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, 0 as i32)
                                == 0
                            {
                                return 0 as i32;
                            }
                            get_buffer = br_state.get_buffer;
                            bits_left = br_state.bits_left;
                            if bits_left < 8 as i32 {
                                nb_1 = 1 as i32;
                                current_block_117 = 2053931041760441864;
                            } else {
                                current_block_117 = 12696043255897098083;
                            }
                        } else {
                            current_block_117 = 12696043255897098083;
                        }
                        match current_block_117 {
                            12696043255897098083 => {
                                look_1 = (get_buffer >> bits_left - 8 as i32) as i32
                                    & bmask[8 as i32 as usize];
                                nb_1 = (*htbl).look_nbits[look_1 as usize];
                                if nb_1 != 0 as i32 {
                                    bits_left -= nb_1;
                                    s = (*htbl).look_sym[look_1 as usize] as i32;
                                    current_block_117 = 7923086311623215889;
                                } else {
                                    nb_1 = 8 as i32 + 1 as i32;
                                    current_block_117 = 2053931041760441864;
                                }
                            }
                            _ => {}
                        }
                        match current_block_117 {
                            2053931041760441864 => {
                                s = jpeg_huff_decode(
                                    &mut br_state,
                                    get_buffer,
                                    bits_left,
                                    htbl,
                                    nb_1,
                                );
                                if s < 0 as i32 {
                                    return 0 as i32;
                                }
                                get_buffer = br_state.get_buffer;
                                bits_left = br_state.bits_left
                            }
                            _ => {}
                        }
                        r = s >> 4 as i32;
                        s &= 15 as i32;
                        if s != 0 {
                            k += r;
                            if bits_left < s {
                                if jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, s)
                                    == 0
                                {
                                    return 0 as i32;
                                }
                                get_buffer = br_state.get_buffer;
                                bits_left = br_state.bits_left
                            }
                            bits_left -= s
                        } else {
                            if r != 15 as i32 {
                                break;
                            }
                            k += 15 as i32
                        }
                        k += 1
                    }
                }
                _ => {}
            }
            blkn += 1
        }
        /* Completed MCU, so update state */
        (*(*cinfo).src).next_input_byte = br_state.next_input_byte;
        (*(*cinfo).src).bytes_in_buffer = br_state.bytes_in_buffer;
        (*entropy).bitstate.get_buffer = get_buffer;
        (*entropy).bitstate.bits_left = bits_left;
        (*entropy).saved = state
    }
    /* Account for restart interval (no-op if not using restarts) */
    (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1);
    return 1 as i32;
}
/*
 * Decode one MCU's worth of Huffman-compressed coefficients,
 * full-size blocks.
 */

unsafe extern "C" fn decode_mcu(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut MCU_data: *mut crate::jpeglib_h::JBLOCKROW,
) -> crate::jmorecfg_h::boolean {
    let mut entropy: huff_entropy_ptr = (*cinfo).entropy as huff_entropy_ptr;
    let mut blkn: i32 = 0;
    let mut get_buffer: bit_buf_type = 0;
    let mut bits_left: i32 = 0;
    let mut br_state: bitread_working_state = bitread_working_state {
        next_input_byte: 0 as *const crate::jmorecfg_h::JOCTET,
        bytes_in_buffer: 0,
        get_buffer: 0,
        bits_left: 0,
        cinfo: 0 as *mut crate::jpeglib_h::jpeg_decompress_struct,
    };
    let mut state: savable_state = savable_state {
        EOBRUN: 0,
        last_dc_val: [0; 4],
    };
    /* Process restart marker if needed; may have to suspend */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0 as i32 as u32 {
            if process_restart(cinfo) == 0 {
                return 0 as i32;
            }
        }
    }
    /* If we've run out of data, just leave the MCU set to zeroes.
     * This way, we return uniform gray for the remainder of the segment.
     */
    if (*entropy).insufficient_data == 0 {
        /* Load up working state */
        br_state.cinfo = cinfo;
        br_state.next_input_byte = (*(*cinfo).src).next_input_byte;
        br_state.bytes_in_buffer = (*(*cinfo).src).bytes_in_buffer;
        get_buffer = (*entropy).bitstate.get_buffer;
        bits_left = (*entropy).bitstate.bits_left;
        state = (*entropy).saved;
        /* Outer loop handles each block in the MCU */
        blkn = 0 as i32;
        while blkn < (*cinfo).blocks_in_MCU {
            let mut current_block_134: u64;
            let mut block: crate::jpeglib_h::JBLOCKROW = *MCU_data.offset(blkn as isize);
            let mut htbl: *mut d_derived_tbl = 0 as *mut d_derived_tbl;
            let mut s: i32 = 0;
            let mut k: i32 = 0;
            let mut r: i32 = 0;
            let mut coef_limit: i32 = 0;
            let mut ci: i32 = 0;
            /* Decode a single block's worth of coefficients */
            /* Section F.2.2.1: decode the DC coefficient difference */
            htbl = (*entropy).dc_cur_tbls[blkn as usize];
            let mut current_block_29: u64;
            let mut nb: i32 = 0;
            let mut look: i32 = 0;
            if bits_left < 8 as i32 {
                if jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, 0 as i32) == 0 {
                    return 0 as i32;
                }
                get_buffer = br_state.get_buffer;
                bits_left = br_state.bits_left;
                if bits_left < 8 as i32 {
                    nb = 1 as i32;
                    current_block_29 = 14455091952362324934;
                } else {
                    current_block_29 = 14359455889292382949;
                }
            } else {
                current_block_29 = 14359455889292382949;
            }
            match current_block_29 {
                14359455889292382949 => {
                    look = (get_buffer >> bits_left - 8 as i32) as i32 & bmask[8 as i32 as usize];
                    nb = (*htbl).look_nbits[look as usize];
                    if nb != 0 as i32 {
                        bits_left -= nb;
                        s = (*htbl).look_sym[look as usize] as i32;
                        current_block_29 = 13131896068329595644;
                    } else {
                        nb = 8 as i32 + 1 as i32;
                        current_block_29 = 14455091952362324934;
                    }
                }
                _ => {}
            }
            match current_block_29 {
                14455091952362324934 => {
                    s = jpeg_huff_decode(&mut br_state, get_buffer, bits_left, htbl, nb);
                    if s < 0 as i32 {
                        return 0 as i32;
                    }
                    get_buffer = br_state.get_buffer;
                    bits_left = br_state.bits_left
                }
                _ => {}
            }
            htbl = (*entropy).ac_cur_tbls[blkn as usize];
            k = 1 as i32;
            coef_limit = (*entropy).coef_limit[blkn as usize];
            if coef_limit != 0 {
                /* Convert DC difference to actual value, update last_dc_val */
                if s != 0 {
                    if bits_left < s {
                        if jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, s) == 0 {
                            return 0 as i32;
                        }
                        get_buffer = br_state.get_buffer;
                        bits_left = br_state.bits_left
                    }
                    bits_left -= s;
                    r = (get_buffer >> bits_left) as i32 & bmask[s as usize];
                    s = if r <= bmask[(s - 1 as i32) as usize] {
                        (r) - bmask[s as usize]
                    } else {
                        r
                    }
                }
                ci = (*cinfo).MCU_membership[blkn as usize];
                s += state.last_dc_val[ci as usize];
                state.last_dc_val[ci as usize] = s;
                /* Output the DC coefficient */
                (*block)[0 as i32 as usize] = s as crate::jmorecfg_h::JCOEF;
                loop
                /* Section F.2.2.2: decode the AC coefficients */
                /* Since zeroes are skipped, output area must be cleared beforehand */
                {
                    if !(k < coef_limit) {
                        current_block_134 = 10863493864285401582;
                        break;
                    }
                    let mut current_block_67: u64;
                    let mut nb_0: i32 = 0;
                    let mut look_0: i32 = 0;
                    if bits_left < 8 as i32 {
                        if jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, 0 as i32) == 0
                        {
                            return 0 as i32;
                        }
                        get_buffer = br_state.get_buffer;
                        bits_left = br_state.bits_left;
                        if bits_left < 8 as i32 {
                            nb_0 = 1 as i32;
                            current_block_67 = 15097141674430979022;
                        } else {
                            current_block_67 = 10853015579903106591;
                        }
                    } else {
                        current_block_67 = 10853015579903106591;
                    }
                    match current_block_67 {
                        10853015579903106591 => {
                            look_0 = (get_buffer >> bits_left - 8 as i32) as i32
                                & bmask[8 as i32 as usize];
                            nb_0 = (*htbl).look_nbits[look_0 as usize];
                            if nb_0 != 0 as i32 {
                                bits_left -= nb_0;
                                s = (*htbl).look_sym[look_0 as usize] as i32;
                                current_block_67 = 851619935621435220;
                            } else {
                                nb_0 = 8 as i32 + 1 as i32;
                                current_block_67 = 15097141674430979022;
                            }
                        }
                        _ => {}
                    }
                    match current_block_67 {
                        15097141674430979022 => {
                            s = jpeg_huff_decode(&mut br_state, get_buffer, bits_left, htbl, nb_0);
                            if s < 0 as i32 {
                                return 0 as i32;
                            }
                            get_buffer = br_state.get_buffer;
                            bits_left = br_state.bits_left
                        }
                        _ => {}
                    }
                    r = s >> 4 as i32;
                    s &= 15 as i32;
                    if s != 0 {
                        k += r;
                        if bits_left < s {
                            if jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, s) == 0 {
                                return 0 as i32;
                            }
                            get_buffer = br_state.get_buffer;
                            bits_left = br_state.bits_left
                        }
                        bits_left -= s;
                        r = (get_buffer >> bits_left) as i32 & bmask[s as usize];
                        s = if r <= bmask[(s - 1 as i32) as usize] {
                            (r) - bmask[s as usize]
                        } else {
                            r
                        };
                        /* Output coefficient in natural (dezigzagged) order.
                         * Note: the extra entries in jpeg_natural_order[] will save us
                         * if k >= DCTSIZE2, which could happen if the data is corrupted.
                         */
                        (*block)[*crate::src::jpeg_8c::jutils::jpeg_natural_order
                            .as_ptr()
                            .offset(k as isize) as usize] = s as crate::jmorecfg_h::JCOEF
                    } else {
                        if r != 15 as i32 {
                            current_block_134 = 14358540534591340610;
                            break;
                        }
                        k += 15 as i32
                    }
                    k += 1
                }
            } else {
                if s != 0 {
                    if bits_left < s {
                        if jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, s) == 0 {
                            return 0 as i32;
                        }
                        get_buffer = br_state.get_buffer;
                        bits_left = br_state.bits_left
                    }
                    bits_left -= s
                }
                current_block_134 = 10863493864285401582;
            }
            match current_block_134 {
                10863493864285401582 => {
                    /* Section F.2.2.2: decode the AC coefficients */
                    /* In this path we just discard the values */
                    while k < 64 as i32 {
                        let mut current_block_115: u64;
                        let mut nb_1: i32 = 0;
                        let mut look_1: i32 = 0;
                        if bits_left < 8 as i32 {
                            if jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, 0 as i32)
                                == 0
                            {
                                return 0 as i32;
                            }
                            get_buffer = br_state.get_buffer;
                            bits_left = br_state.bits_left;
                            if bits_left < 8 as i32 {
                                nb_1 = 1 as i32;
                                current_block_115 = 15151470646530504543;
                            } else {
                                current_block_115 = 9180031981464905198;
                            }
                        } else {
                            current_block_115 = 9180031981464905198;
                        }
                        match current_block_115 {
                            9180031981464905198 => {
                                look_1 = (get_buffer >> bits_left - 8 as i32) as i32
                                    & bmask[8 as i32 as usize];
                                nb_1 = (*htbl).look_nbits[look_1 as usize];
                                if nb_1 != 0 as i32 {
                                    bits_left -= nb_1;
                                    s = (*htbl).look_sym[look_1 as usize] as i32;
                                    current_block_115 = 12890877304563811856;
                                } else {
                                    nb_1 = 8 as i32 + 1 as i32;
                                    current_block_115 = 15151470646530504543;
                                }
                            }
                            _ => {}
                        }
                        match current_block_115 {
                            15151470646530504543 => {
                                s = jpeg_huff_decode(
                                    &mut br_state,
                                    get_buffer,
                                    bits_left,
                                    htbl,
                                    nb_1,
                                );
                                if s < 0 as i32 {
                                    return 0 as i32;
                                }
                                get_buffer = br_state.get_buffer;
                                bits_left = br_state.bits_left
                            }
                            _ => {}
                        }
                        r = s >> 4 as i32;
                        s &= 15 as i32;
                        if s != 0 {
                            k += r;
                            if bits_left < s {
                                if jpeg_fill_bit_buffer(&mut br_state, get_buffer, bits_left, s)
                                    == 0
                                {
                                    return 0 as i32;
                                }
                                get_buffer = br_state.get_buffer;
                                bits_left = br_state.bits_left
                            }
                            bits_left -= s
                        } else {
                            if r != 15 as i32 {
                                break;
                            }
                            k += 15 as i32
                        }
                        k += 1
                    }
                }
                _ => {}
            }
            blkn += 1
        }
        /* Completed MCU, so update state */
        (*(*cinfo).src).next_input_byte = br_state.next_input_byte;
        (*(*cinfo).src).bytes_in_buffer = br_state.bytes_in_buffer;
        (*entropy).bitstate.get_buffer = get_buffer;
        (*entropy).bitstate.bits_left = bits_left;
        (*entropy).saved = state
    }
    /* Account for restart interval (no-op if not using restarts) */
    (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1);
    return 1 as i32;
}
/*
 * Initialize for a Huffman-compressed scan.
 */

unsafe extern "C" fn start_pass_huff_decoder(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    let mut current_block: u64;
    let mut entropy: huff_entropy_ptr = (*cinfo).entropy as huff_entropy_ptr;
    let mut ci: i32 = 0;
    let mut blkn: i32 = 0;
    let mut tbl: i32 = 0;
    let mut i: i32 = 0;
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        0 as *mut crate::jpeglib_h::jpeg_component_info;
    if (*cinfo).progressive_mode != 0 {
        /* Validate progressive scan parameters */
        if (*cinfo).Ss == 0 as i32 {
            if (*cinfo).Se != 0 as i32 {
                current_block = 5792855011569847122;
            } else {
                current_block = 8515828400728868193;
            }
        } else if (*cinfo).Se < (*cinfo).Ss || (*cinfo).Se > (*cinfo).lim_Se {
            current_block = 5792855011569847122;
        } else if (*cinfo).comps_in_scan != 1 as i32 {
            current_block = 5792855011569847122;
        } else {
            current_block = 8515828400728868193;
        }
        match current_block {
            8515828400728868193 => {
                if (*cinfo).Ah != 0 as i32 {
                    /* need not check Ss/Se < 0 since they came from unsigned bytes */
                    /* AC scans may have only one component */
                    /* Successive approximation refinement scan: must have Al = Ah-1. */
                    if (*cinfo).Ah - 1 as i32 != (*cinfo).Al {
                        current_block = 5792855011569847122;
                    } else {
                        current_block = 7746791466490516765;
                    }
                } else {
                    current_block = 7746791466490516765;
                }
                match current_block {
                    5792855011569847122 => {}
                    _ => {
                        if (*cinfo).Al > 13 as i32 {
                            current_block = 5792855011569847122;
                        } else {
                            current_block = 12599329904712511516;
                        }
                    }
                }
            }
            _ => {}
        }
        match current_block {
            5792855011569847122 =>
            /* need not check for < 0 */
            /* Arguably the maximum Al value should be less than 13 for 8-bit precision,
             * but the spec doesn't say so, and we try to be liberal about what we
             * accept.  Note: large Al values could result in out-of-range DC
             * coefficients during early scans, leading to bizarre displays due to
             * overflows in the IDCT math.  But we won't crash.
             */
            {
                (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_PROGRESSION as i32;
                (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = (*cinfo).Ss;
                (*(*cinfo).err).msg_parm.i[1 as i32 as usize] = (*cinfo).Se;
                (*(*cinfo).err).msg_parm.i[2 as i32 as usize] = (*cinfo).Ah;
                (*(*cinfo).err).msg_parm.i[3 as i32 as usize] = (*cinfo).Al;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            _ => {}
        }
        /* Update progression status, and verify that scan order is legal.
         * Note that inter-scan inconsistencies are treated as warnings
         * not fatal errors ... not clear if this is right way to behave.
         */
        ci = 0 as i32;
        while ci < (*cinfo).comps_in_scan {
            let mut coefi: i32 = 0;
            let mut cindex: i32 = (*(*cinfo).cur_comp_info[ci as usize]).component_index;
            let mut coef_bit_ptr: *mut i32 = &mut *(*(*cinfo).coef_bits.offset(cindex as isize))
                .as_mut_ptr()
                .offset(0 as i32 as isize) as *mut i32;
            if (*cinfo).Ss != 0 && *coef_bit_ptr.offset(0 as i32 as isize) < 0 as i32 {
                /* AC without prior DC scan */
                (*(*cinfo).err).msg_code =
                    crate::src::jpeg_8c::jerror::JWRN_BOGUS_PROGRESSION as i32;
                (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = cindex;
                (*(*cinfo).err).msg_parm.i[1 as i32 as usize] = 0 as i32;
                Some(
                    (*(*cinfo).err)
                        .emit_message
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr,
                    -(1 as i32),
                );
            }
            coefi = (*cinfo).Ss;
            while coefi <= (*cinfo).Se {
                let mut expected: i32 = if *coef_bit_ptr.offset(coefi as isize) < 0 as i32 {
                    0 as i32
                } else {
                    *coef_bit_ptr.offset(coefi as isize)
                };
                if (*cinfo).Ah != expected {
                    (*(*cinfo).err).msg_code =
                        crate::src::jpeg_8c::jerror::JWRN_BOGUS_PROGRESSION as i32;
                    (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = cindex;
                    (*(*cinfo).err).msg_parm.i[1 as i32 as usize] = coefi;
                    Some(
                        (*(*cinfo).err)
                            .emit_message
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as crate::jpeglib_h::j_common_ptr,
                        -(1 as i32),
                    );
                }
                *coef_bit_ptr.offset(coefi as isize) = (*cinfo).Al;
                coefi += 1
            }
            ci += 1
        }
        /* Select MCU decoding routine */
        if (*cinfo).Ah == 0 as i32 {
            if (*cinfo).Ss == 0 as i32 {
                (*entropy).pub_0.decode_mcu = Some(
                    decode_mcu_DC_first
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: *mut crate::jpeglib_h::JBLOCKROW,
                        )
                            -> crate::jmorecfg_h::boolean,
                )
            } else {
                (*entropy).pub_0.decode_mcu = Some(
                    decode_mcu_AC_first
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: *mut crate::jpeglib_h::JBLOCKROW,
                        )
                            -> crate::jmorecfg_h::boolean,
                )
            }
        } else if (*cinfo).Ss == 0 as i32 {
            (*entropy).pub_0.decode_mcu = Some(
                decode_mcu_DC_refine
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_decompress_ptr,
                        _: *mut crate::jpeglib_h::JBLOCKROW,
                    ) -> crate::jmorecfg_h::boolean,
            )
        } else {
            (*entropy).pub_0.decode_mcu = Some(
                decode_mcu_AC_refine
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_decompress_ptr,
                        _: *mut crate::jpeglib_h::JBLOCKROW,
                    ) -> crate::jmorecfg_h::boolean,
            )
        }
        ci = 0 as i32;
        while ci < (*cinfo).comps_in_scan {
            compptr = (*cinfo).cur_comp_info[ci as usize];
            /* Make sure requested tables are present, and compute derived tables.
             * We may build same derived table more than once, but it's not expensive.
             */
            if (*cinfo).Ss == 0 as i32 {
                if (*cinfo).Ah == 0 as i32 {
                    /* DC refinement needs no table */
                    tbl = (*compptr).dc_tbl_no;
                    jpeg_make_d_derived_tbl(
                        cinfo,
                        1 as i32,
                        tbl,
                        &mut *(*entropy).derived_tbls.as_mut_ptr().offset(tbl as isize),
                    );
                }
            } else {
                tbl = (*compptr).ac_tbl_no;
                jpeg_make_d_derived_tbl(
                    cinfo,
                    0 as i32,
                    tbl,
                    &mut *(*entropy).derived_tbls.as_mut_ptr().offset(tbl as isize),
                );
                /* remember the single active table */
                (*entropy).ac_derived_tbl = (*entropy).derived_tbls[tbl as usize]
            }
            /* Initialize DC predictions to 0 */
            (*entropy).saved.last_dc_val[ci as usize] = 0 as i32;
            ci += 1
        }
        /* Initialize private state variables */
        (*entropy).saved.EOBRUN = 0 as i32 as u32
    } else {
        /* Check that the scan parameters Ss, Se, Ah/Al are OK for sequential JPEG.
         * This ought to be an error condition, but we make it a warning because
         * there are some baseline files out there with all zeroes in these bytes.
         */
        if (*cinfo).Ss != 0 as i32
            || (*cinfo).Ah != 0 as i32
            || (*cinfo).Al != 0 as i32
            || ((*cinfo).is_baseline != 0 || (*cinfo).Se < 64 as i32)
                && (*cinfo).Se != (*cinfo).lim_Se
        {
            (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JWRN_NOT_SEQUENTIAL as i32;
            Some(
                (*(*cinfo).err)
                    .emit_message
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr,
                -(1 as i32),
            );
        }
        /* Select MCU decoding routine */
        /* We retain the hard-coded case for full-size blocks.
         * This is not necessary, but it appears that this version is slightly
         * more performant in the given implementation.
         * With an improved implementation we would prefer a single optimized
         * function.
         */
        if (*cinfo).lim_Se != 64 as i32 - 1 as i32 {
            (*entropy).pub_0.decode_mcu = Some(
                decode_mcu_sub
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_decompress_ptr,
                        _: *mut crate::jpeglib_h::JBLOCKROW,
                    ) -> crate::jmorecfg_h::boolean,
            )
        } else {
            (*entropy).pub_0.decode_mcu = Some(
                decode_mcu
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_decompress_ptr,
                        _: *mut crate::jpeglib_h::JBLOCKROW,
                    ) -> crate::jmorecfg_h::boolean,
            )
        }
        ci = 0 as i32;
        while ci < (*cinfo).comps_in_scan {
            compptr = (*cinfo).cur_comp_info[ci as usize];
            /* Compute derived values for Huffman tables */
            /* We may do this more than once for a table, but it's not expensive */
            tbl = (*compptr).dc_tbl_no;
            jpeg_make_d_derived_tbl(
                cinfo,
                1 as i32,
                tbl,
                &mut *(*entropy).dc_derived_tbls.as_mut_ptr().offset(tbl as isize),
            );
            if (*cinfo).lim_Se != 0 {
                /* AC needs no table when not present */
                tbl = (*compptr).ac_tbl_no;
                jpeg_make_d_derived_tbl(
                    cinfo,
                    0 as i32,
                    tbl,
                    &mut *(*entropy).ac_derived_tbls.as_mut_ptr().offset(tbl as isize),
                );
            }
            /* Initialize DC predictions to 0 */
            (*entropy).saved.last_dc_val[ci as usize] = 0 as i32;
            ci += 1
        }
        /* Precalculate decoding info for each block in an MCU of this scan */
        blkn = 0 as i32;
        while blkn < (*cinfo).blocks_in_MCU {
            ci = (*cinfo).MCU_membership[blkn as usize];
            compptr = (*cinfo).cur_comp_info[ci as usize];
            /* Precalculate which table to use for each block */
            (*entropy).dc_cur_tbls[blkn as usize] =
                (*entropy).dc_derived_tbls[(*compptr).dc_tbl_no as usize];
            (*entropy).ac_cur_tbls[blkn as usize] =
                (*entropy).ac_derived_tbls[(*compptr).ac_tbl_no as usize];
            /* Decide whether we really care about the coefficient values */
            if (*compptr).component_needed != 0 {
                ci = (*compptr).DCT_v_scaled_size;
                i = (*compptr).DCT_h_scaled_size;
                match (*cinfo).lim_Se {
                    0 => (*entropy).coef_limit[blkn as usize] = 1 as i32,
                    3 => {
                        if ci <= 0 as i32 || ci > 2 as i32 {
                            ci = 2 as i32
                        }
                        if i <= 0 as i32 || i > 2 as i32 {
                            i = 2 as i32
                        }
                        (*entropy).coef_limit[blkn as usize] = 1 as i32
                            + jpeg_zigzag_order2[(ci - 1 as i32) as usize][(i - 1 as i32) as usize]
                    }
                    8 => {
                        if ci <= 0 as i32 || ci > 3 as i32 {
                            ci = 3 as i32
                        }
                        if i <= 0 as i32 || i > 3 as i32 {
                            i = 3 as i32
                        }
                        (*entropy).coef_limit[blkn as usize] = 1 as i32
                            + jpeg_zigzag_order3[(ci - 1 as i32) as usize][(i - 1 as i32) as usize]
                    }
                    15 => {
                        if ci <= 0 as i32 || ci > 4 as i32 {
                            ci = 4 as i32
                        }
                        if i <= 0 as i32 || i > 4 as i32 {
                            i = 4 as i32
                        }
                        (*entropy).coef_limit[blkn as usize] = 1 as i32
                            + jpeg_zigzag_order4[(ci - 1 as i32) as usize][(i - 1 as i32) as usize]
                    }
                    24 => {
                        if ci <= 0 as i32 || ci > 5 as i32 {
                            ci = 5 as i32
                        }
                        if i <= 0 as i32 || i > 5 as i32 {
                            i = 5 as i32
                        }
                        (*entropy).coef_limit[blkn as usize] = 1 as i32
                            + jpeg_zigzag_order5[(ci - 1 as i32) as usize][(i - 1 as i32) as usize]
                    }
                    35 => {
                        if ci <= 0 as i32 || ci > 6 as i32 {
                            ci = 6 as i32
                        }
                        if i <= 0 as i32 || i > 6 as i32 {
                            i = 6 as i32
                        }
                        (*entropy).coef_limit[blkn as usize] = 1 as i32
                            + jpeg_zigzag_order6[(ci - 1 as i32) as usize][(i - 1 as i32) as usize]
                    }
                    48 => {
                        if ci <= 0 as i32 || ci > 7 as i32 {
                            ci = 7 as i32
                        }
                        if i <= 0 as i32 || i > 7 as i32 {
                            i = 7 as i32
                        }
                        (*entropy).coef_limit[blkn as usize] = 1 as i32
                            + jpeg_zigzag_order7[(ci - 1 as i32) as usize][(i - 1 as i32) as usize]
                    }
                    _ => {
                        if ci <= 0 as i32 || ci > 8 as i32 {
                            ci = 8 as i32
                        }
                        if i <= 0 as i32 || i > 8 as i32 {
                            i = 8 as i32
                        }
                        (*entropy).coef_limit[blkn as usize] = 1 as i32
                            + jpeg_zigzag_order[(ci - 1 as i32) as usize][(i - 1 as i32) as usize]
                    }
                }
            } else {
                (*entropy).coef_limit[blkn as usize] = 0 as i32
            }
            blkn += 1
        }
    }
    /* Initialize bitread state variables */
    (*entropy).bitstate.bits_left = 0 as i32; /* unnecessary, but keeps Purify quiet */
    (*entropy).bitstate.get_buffer = 0 as i32 as bit_buf_type;
    (*entropy).insufficient_data = 0 as i32;
    /* Initialize restart counter */
    (*entropy).restarts_to_go = (*cinfo).restart_interval;
}
/*
 * Module initialization routine for Huffman entropy decoding.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_huff_decoder(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    let mut entropy: huff_entropy_ptr = 0 as *mut huff_entropy_decoder;
    let mut i: i32 = 0;
    entropy = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        1 as i32,
        ::std::mem::size_of::<huff_entropy_decoder>() as libc::c_ulong,
    ) as huff_entropy_ptr;
    (*cinfo).entropy = entropy as *mut crate::jpegint_h::jpeg_entropy_decoder;
    (*entropy).pub_0.start_pass = Some(
        start_pass_huff_decoder
            as unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> (),
    );
    if (*cinfo).progressive_mode != 0 {
        /* Create progression status table */
        let mut coef_bit_ptr: *mut i32 = 0 as *mut i32;
        let mut ci: i32 = 0;
        (*cinfo).coef_bits = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            1 as i32,
            (((*cinfo).num_components * 64 as i32) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<i32>() as libc::c_ulong),
        ) as *mut [i32; 64];
        coef_bit_ptr = &mut *(*(*cinfo).coef_bits.offset(0 as i32 as isize))
            .as_mut_ptr()
            .offset(0 as i32 as isize) as *mut i32;
        ci = 0 as i32;
        while ci < (*cinfo).num_components {
            i = 0 as i32;
            while i < 64 as i32 {
                let fresh6 = coef_bit_ptr;
                coef_bit_ptr = coef_bit_ptr.offset(1);
                *fresh6 = -(1 as i32);
                i += 1
            }
            ci += 1
        }
        /* Mark derived tables unallocated */
        i = 0 as i32;
        while i < 4 as i32 {
            (*entropy).derived_tbls[i as usize] = 0 as *mut d_derived_tbl;
            i += 1
        }
    } else {
        /* Mark tables unallocated */
        i = 0 as i32;
        while i < 4 as i32 {
            (*entropy).ac_derived_tbls[i as usize] = 0 as *mut d_derived_tbl;
            (*entropy).dc_derived_tbls[i as usize] = (*entropy).ac_derived_tbls[i as usize];
            i += 1
        }
    };
}
