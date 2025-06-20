use ::libc;

/* JERROR_H */

/* Informational/debugging messages */

/* Nonfatal errors (we can keep going, but the data is probably corrupt) */

/* Fatal errors (print message and exit) */

/* Macros to simplify using the error and trace message stuff */

/* The first parameter is either type of cinfo pointer */

/* Zap JMESSAGE macro so that future re-inclusions do nothing by default */

/* JMAKE_ENUM_LIST */
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

pub type my_marker_ptr = *mut my_marker_writer;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_marker_writer {
    pub pub_0: jpeg_marker_writer,
    pub last_restart_interval: u32,
}

pub type JPEG_MARKER = u32;

pub const M_ERROR: JPEG_MARKER = 256;

pub const M_TEM: JPEG_MARKER = 1;

pub const M_COM: JPEG_MARKER = 254;

pub const M_JPG13: JPEG_MARKER = 253;

pub const M_JPG0: JPEG_MARKER = 240;

pub const M_APP15: JPEG_MARKER = 239;

pub const M_APP14: JPEG_MARKER = 238;

pub const M_APP13: JPEG_MARKER = 237;

pub const M_APP12: JPEG_MARKER = 236;

pub const M_APP11: JPEG_MARKER = 235;

pub const M_APP10: JPEG_MARKER = 234;

pub const M_APP9: JPEG_MARKER = 233;

pub const M_APP8: JPEG_MARKER = 232;

pub const M_APP7: JPEG_MARKER = 231;

pub const M_APP6: JPEG_MARKER = 230;

pub const M_APP5: JPEG_MARKER = 229;

pub const M_APP4: JPEG_MARKER = 228;

pub const M_APP3: JPEG_MARKER = 227;

pub const M_APP2: JPEG_MARKER = 226;

pub const M_APP1: JPEG_MARKER = 225;

pub const M_APP0: JPEG_MARKER = 224;

pub const M_EXP: JPEG_MARKER = 223;

pub const M_DHP: JPEG_MARKER = 222;

pub const M_DRI: JPEG_MARKER = 221;

pub const M_DNL: JPEG_MARKER = 220;

pub const M_DQT: JPEG_MARKER = 219;

pub const M_SOS: JPEG_MARKER = 218;

pub const M_EOI: JPEG_MARKER = 217;

pub const M_SOI: JPEG_MARKER = 216;

pub const M_RST7: JPEG_MARKER = 215;

pub const M_RST6: JPEG_MARKER = 214;

pub const M_RST5: JPEG_MARKER = 213;

pub const M_RST4: JPEG_MARKER = 212;

pub const M_RST3: JPEG_MARKER = 211;

pub const M_RST2: JPEG_MARKER = 210;

pub const M_RST1: JPEG_MARKER = 209;

pub const M_RST0: JPEG_MARKER = 208;

pub const M_DAC: JPEG_MARKER = 204;

pub const M_DHT: JPEG_MARKER = 196;

pub const M_SOF15: JPEG_MARKER = 207;

pub const M_SOF14: JPEG_MARKER = 206;

pub const M_SOF13: JPEG_MARKER = 205;

pub const M_SOF11: JPEG_MARKER = 203;

pub const M_SOF10: JPEG_MARKER = 202;

pub const M_SOF9: JPEG_MARKER = 201;

pub const M_JPG: JPEG_MARKER = 200;

pub const M_SOF7: JPEG_MARKER = 199;

pub const M_SOF6: JPEG_MARKER = 198;

pub const M_SOF5: JPEG_MARKER = 197;

pub const M_SOF3: JPEG_MARKER = 195;

pub const M_SOF2: JPEG_MARKER = 194;

pub const M_SOF1: JPEG_MARKER = 193;

pub const M_SOF0: JPEG_MARKER = 192;
/*
 * Basic output routines.
 *
 * Note that we do not support suspension while writing a marker.
 * Therefore, an application using suspension must ensure that there is
 * enough buffer space for the initial markers (typ. 600-700 bytes) before
 * calling jpeg_start_compress, and enough space to write the trailing EOI
 * (a few bytes) before calling jpeg_finish_compress.  Multipass compression
 * modes are not supported at all with suspension, so those two are the only
 * points where markers will be written.
 */

unsafe extern "C" fn emit_byte(mut cinfo: j_compress_ptr, mut val: i32)
/* Emit a byte */
{
    let mut dest: *mut jpeg_destination_mgr = (*cinfo).dest;
    let fresh0 = (*dest).next_output_byte;
    (*dest).next_output_byte = (*dest).next_output_byte.offset(1);
    *fresh0 = val as JOCTET;
    (*dest).free_in_buffer = (*dest).free_in_buffer.wrapping_sub(1);
    if (*dest).free_in_buffer == 0 as i32 as usize {
        if Some(
            (*dest)
                .empty_output_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            (*(*cinfo).err).msg_code = JERR_CANT_SUSPEND as i32;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
    };
}

unsafe extern "C" fn emit_marker(mut cinfo: j_compress_ptr, mut mark: JPEG_MARKER)
/* Emit a marker code */
{
    emit_byte(cinfo, 0xff as i32);
    emit_byte(cinfo, mark as i32);
}

unsafe extern "C" fn emit_2bytes(mut cinfo: j_compress_ptr, mut value: i32)
/* Emit a 2-byte integer; these are always MSB first in JPEG files */
{
    emit_byte(cinfo, value >> 8 as i32 & 0xff as i32);
    emit_byte(cinfo, value & 0xff as i32);
}
/*
 * Routines to write specific marker types.
 */

unsafe extern "C" fn emit_dqt(mut cinfo: j_compress_ptr, mut index: i32) -> i32
/* Emit a DQT marker */
/* Returns the precision used (0 = 8bits, 1 = 16bits) for baseline checking */ {
    let mut qtbl: *mut JQUANT_TBL = (*cinfo).quant_tbl_ptrs[index as usize];
    let mut prec: i32 = 0;
    let mut i: i32 = 0;
    if qtbl.is_null() {
        (*(*cinfo).err).msg_code = JERR_NO_QUANT_TABLE as i32;
        (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = index;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    prec = 0 as i32;
    i = 0 as i32;
    while i <= (*cinfo).lim_Se {
        if (*qtbl).quantval[*(*cinfo).natural_order.offset(i as isize) as usize] as i32 > 255 as i32
        {
            prec = 1 as i32
        }
        i += 1
    }
    if (*qtbl).sent_table == 0 {
        emit_marker(cinfo, M_DQT);
        emit_2bytes(
            cinfo,
            if prec != 0 {
                ((*cinfo).lim_Se * 2 as i32 + 2 as i32 + 1 as i32) + 2 as i32
            } else {
                ((*cinfo).lim_Se + 1 as i32 + 1 as i32) + 2 as i32
            },
        );
        emit_byte(cinfo, index + (prec << 4 as i32));
        i = 0 as i32;
        while i <= (*cinfo).lim_Se {
            /* The table entries must be emitted in zigzag order. */
            let mut qval: u32 =
                (*qtbl).quantval[*(*cinfo).natural_order.offset(i as isize) as usize] as u32;
            if prec != 0 {
                emit_byte(cinfo, (qval >> 8 as i32) as i32);
            }
            emit_byte(cinfo, (qval & 0xff as i32 as u32) as i32);
            i += 1
        }
        (*qtbl).sent_table = 1 as i32
    }
    return prec;
}

unsafe extern "C" fn emit_dht(mut cinfo: j_compress_ptr, mut index: i32, mut is_ac: boolean)
/* Emit a DHT marker */
{
    let mut htbl: *mut JHUFF_TBL = std::ptr::null_mut();
    let mut length: i32 = 0;
    let mut i: i32 = 0;
    if is_ac != 0 {
        htbl = (*cinfo).ac_huff_tbl_ptrs[index as usize];
        index += 0x10 as i32
    /* output index has AC bit set */
    } else {
        htbl = (*cinfo).dc_huff_tbl_ptrs[index as usize]
    }
    if htbl.is_null() {
        (*(*cinfo).err).msg_code = JERR_NO_HUFF_TABLE as i32;
        (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = index;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if (*htbl).sent_table == 0 {
        emit_marker(cinfo, M_DHT);
        length = 0 as i32;
        i = 1 as i32;
        while i <= 16 as i32 {
            length += (*htbl).bits[i as usize] as i32;
            i += 1
        }
        emit_2bytes(cinfo, length + 2 as i32 + 1 as i32 + 16 as i32);
        emit_byte(cinfo, index);
        i = 1 as i32;
        while i <= 16 as i32 {
            emit_byte(cinfo, (*htbl).bits[i as usize] as i32);
            i += 1
        }
        i = 0 as i32;
        while i < length {
            emit_byte(cinfo, (*htbl).huffval[i as usize] as i32);
            i += 1
        }
        (*htbl).sent_table = 1 as i32
    };
}

unsafe extern "C" fn emit_dac(mut cinfo: j_compress_ptr)
/* Emit a DAC marker */
/* Since the useful info is so small, we want to emit all the tables in */
/* one DAC marker.  Therefore this routine does its own scan of the table. */
{
    let mut dc_in_use: [libc::c_char; 16] = [0; 16];
    let mut ac_in_use: [libc::c_char; 16] = [0; 16];
    let mut length: i32 = 0;
    let mut i: i32 = 0;
    let mut compptr: *mut jpeg_component_info = std::ptr::null_mut();
    i = 0 as i32;
    while i < 16 as i32 {
        ac_in_use[i as usize] = 0 as i32 as libc::c_char;
        dc_in_use[i as usize] = ac_in_use[i as usize];
        i += 1
    }
    i = 0 as i32;
    while i < (*cinfo).comps_in_scan {
        compptr = (*cinfo).cur_comp_info[i as usize];
        /* DC needs no table for refinement scan */
        if (*cinfo).Ss == 0 as i32 && (*cinfo).Ah == 0 as i32 {
            dc_in_use[(*compptr).dc_tbl_no as usize] = 1 as i32 as libc::c_char
        }
        /* AC needs no table when not present */
        if (*cinfo).Se != 0 {
            ac_in_use[(*compptr).ac_tbl_no as usize] = 1 as i32 as libc::c_char
        }
        i += 1
    }
    length = 0 as i32;
    i = 0 as i32;
    while i < 16 as i32 {
        length += dc_in_use[i as usize] as i32 + ac_in_use[i as usize] as i32;
        i += 1
    }
    if length != 0 {
        emit_marker(cinfo, M_DAC);
        emit_2bytes(cinfo, length * 2 as i32 + 2 as i32);
        i = 0 as i32;
        while i < 16 as i32 {
            if dc_in_use[i as usize] != 0 {
                emit_byte(cinfo, i);
                emit_byte(
                    cinfo,
                    (*cinfo).arith_dc_L[i as usize] as i32
                        + (((*cinfo).arith_dc_U[i as usize] as i32) << 4 as i32),
                );
            }
            if ac_in_use[i as usize] != 0 {
                emit_byte(cinfo, i + 0x10 as i32);
                emit_byte(cinfo, (*cinfo).arith_ac_K[i as usize] as i32);
            }
            i += 1
        }
    };
    /* C_ARITH_CODING_SUPPORTED */
}

unsafe extern "C" fn emit_dri(mut cinfo: j_compress_ptr)
/* Emit a DRI marker */
{
    emit_marker(cinfo, M_DRI); /* fixed length */
    emit_2bytes(cinfo, 4 as i32);
    emit_2bytes(cinfo, (*cinfo).restart_interval as i32);
}

unsafe extern "C" fn emit_sof(mut cinfo: j_compress_ptr, mut code: JPEG_MARKER)
/* Emit a SOF marker */
{
    let mut ci: i32 = 0; /* length */
    let mut compptr: *mut jpeg_component_info = std::ptr::null_mut();
    emit_marker(cinfo, code);
    emit_2bytes(
        cinfo,
        3 as i32 * (*cinfo).num_components + 2 as i32 + 5 as i32 + 1 as i32,
    );
    /* Make sure image isn't bigger than SOF field can handle */
    if (*cinfo).jpeg_height as isize > 65535 as isize
        || (*cinfo).jpeg_width as isize > 65535 as isize
    {
        (*(*cinfo).err).msg_code = JERR_IMAGE_TOO_BIG as i32;
        (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = 65535 as i32 as u32 as i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    emit_byte(cinfo, (*cinfo).data_precision);
    emit_2bytes(cinfo, (*cinfo).jpeg_height as i32);
    emit_2bytes(cinfo, (*cinfo).jpeg_width as i32);
    emit_byte(cinfo, (*cinfo).num_components);
    ci = 0 as i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        emit_byte(cinfo, (*compptr).component_id);
        emit_byte(
            cinfo,
            ((*compptr).h_samp_factor << 4 as i32) + (*compptr).v_samp_factor,
        );
        emit_byte(cinfo, (*compptr).quant_tbl_no);
        ci += 1;
        compptr = compptr.offset(1)
    }
}

unsafe extern "C" fn emit_sos(mut cinfo: j_compress_ptr)
/* Emit a SOS marker */
{
    let mut i: i32 = 0; /* length */
    let mut td: i32 = 0;
    let mut ta: i32 = 0;
    let mut compptr: *mut jpeg_component_info = std::ptr::null_mut();
    emit_marker(cinfo, M_SOS);
    emit_2bytes(
        cinfo,
        2 as i32 * (*cinfo).comps_in_scan + 2 as i32 + 1 as i32 + 3 as i32,
    );
    emit_byte(cinfo, (*cinfo).comps_in_scan);
    i = 0 as i32;
    while i < (*cinfo).comps_in_scan {
        compptr = (*cinfo).cur_comp_info[i as usize];
        emit_byte(cinfo, (*compptr).component_id);
        /* We emit 0 for unused field(s); this is recommended by the P&M text
         * but does not seem to be specified in the standard.
         */
        /* DC needs no table for refinement scan */
        td = if (*cinfo).Ss == 0 as i32 && (*cinfo).Ah == 0 as i32 {
            (*compptr).dc_tbl_no
        } else {
            0 as i32
        };
        /* AC needs no table when not present */
        ta = if (*cinfo).Se != 0 {
            (*compptr).ac_tbl_no
        } else {
            0 as i32
        };
        emit_byte(cinfo, (td << 4 as i32) + ta);
        i += 1
    }
    emit_byte(cinfo, (*cinfo).Ss);
    emit_byte(cinfo, (*cinfo).Se);
    emit_byte(cinfo, ((*cinfo).Ah << 4 as i32) + (*cinfo).Al);
}

unsafe extern "C" fn emit_pseudo_sos(mut cinfo: j_compress_ptr)
/* Emit a pseudo SOS marker */
{
    emit_marker(cinfo, M_SOS); /* length */
    emit_2bytes(cinfo, 2 as i32 + 1 as i32 + 3 as i32); /* Ns */
    emit_byte(cinfo, 0 as i32); /* Ss */
    emit_byte(cinfo, 0 as i32); /* Se */
    emit_byte(cinfo, (*cinfo).block_size * (*cinfo).block_size - 1 as i32);
    emit_byte(cinfo, 0 as i32);
    /* Ah/Al */
}

unsafe extern "C" fn emit_jfif_app0(mut cinfo: j_compress_ptr)
/* Emit a JFIF-compliant APP0 marker */
{
    /*
     * Length of APP0 block	(2 bytes)
     * Block ID			(4 bytes - ASCII "JFIF")
     * Zero byte			(1 byte to terminate the ID string)
     * Version Major, Minor	(2 bytes - major first)
     * Units			(1 byte - 0x00 = none, 0x01 = inch, 0x02 = cm)
     * Xdpu			(2 bytes - dots per unit horizontal)
     * Ydpu			(2 bytes - dots per unit vertical)
     * Thumbnail X size		(1 byte)
     * Thumbnail Y size		(1 byte)
     */
    emit_marker(cinfo, M_APP0); /* length */
    emit_2bytes(
        cinfo,
        2 as i32
            + 4 as i32
            + 1 as i32
            + 2 as i32
            + 1 as i32
            + 2 as i32
            + 2 as i32
            + 1 as i32
            + 1 as i32,
    ); /* Identifier: ASCII "JFIF" */
    emit_byte(cinfo, 0x4a as i32); /* Version fields */
    emit_byte(cinfo, 0x46 as i32); /* Pixel size information */
    emit_byte(cinfo, 0x49 as i32); /* No thumbnail image */
    emit_byte(cinfo, 0x46 as i32);
    emit_byte(cinfo, 0 as i32);
    emit_byte(cinfo, (*cinfo).JFIF_major_version as i32);
    emit_byte(cinfo, (*cinfo).JFIF_minor_version as i32);
    emit_byte(cinfo, (*cinfo).density_unit as i32);
    emit_2bytes(cinfo, (*cinfo).X_density as i32);
    emit_2bytes(cinfo, (*cinfo).Y_density as i32);
    emit_byte(cinfo, 0 as i32);
    emit_byte(cinfo, 0 as i32);
}

unsafe extern "C" fn emit_adobe_app14(mut cinfo: j_compress_ptr)
/* Emit an Adobe APP14 marker */
{
    /*
     * Length of APP14 block	(2 bytes)
     * Block ID			(5 bytes - ASCII "Adobe")
     * Version Number		(2 bytes - currently 100)
     * Flags0			(2 bytes - currently 0)
     * Flags1			(2 bytes - currently 0)
     * Color transform		(1 byte)
     *
     * Although Adobe TN 5116 mentions Version = 101, all the Adobe files
     * now in circulation seem to use Version = 100, so that's what we write.
     *
     * We write the color transform byte as 1 if the JPEG color space is
     * YCbCr, 2 if it's YCCK, 0 otherwise.  Adobe's definition has to do with
     * whether the encoder performed a transformation, which is pretty useless.
     */
    emit_marker(cinfo, M_APP14); /* length */
    emit_2bytes(
        cinfo,
        2 as i32 + 5 as i32 + 2 as i32 + 2 as i32 + 2 as i32 + 1 as i32,
    ); /* Identifier: ASCII "Adobe" */
    emit_byte(cinfo, 0x41 as i32); /* Version */
    emit_byte(cinfo, 0x64 as i32); /* Flags0 */
    emit_byte(cinfo, 0x6f as i32); /* Flags1 */
    emit_byte(cinfo, 0x62 as i32); /* Color transform = 1 */
    emit_byte(cinfo, 0x65 as i32); /* Color transform = 2 */
    emit_2bytes(cinfo, 100 as i32); /* Color transform = 0 */
    emit_2bytes(cinfo, 0 as i32);
    emit_2bytes(cinfo, 0 as i32);
    match (*cinfo).jpeg_color_space as u32 {
        3 => {
            emit_byte(cinfo, 1 as i32);
        }
        5 => {
            emit_byte(cinfo, 2 as i32);
        }
        _ => {
            emit_byte(cinfo, 0 as i32);
        }
    };
}
/*
 * These routines allow writing an arbitrary marker with parameters.
 * The only intended use is to emit COM or APPn markers after calling
 * write_file_header and before calling write_frame_header.
 * Other uses are not guaranteed to produce desirable results.
 * Counting the parameter bytes properly is the caller's responsibility.
 */

unsafe extern "C" fn write_marker_header(
    mut cinfo: j_compress_ptr,
    mut marker: i32,
    mut datalen: u32,
)
/* Emit an arbitrary marker header */
{
    if datalen > 65533 as i32 as u32 {
        /* safety check */
        (*(*cinfo).err).msg_code = JERR_BAD_LENGTH as i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    emit_marker(cinfo, marker as JPEG_MARKER);
    emit_2bytes(cinfo, datalen.wrapping_add(2 as i32 as u32) as i32);
    /* total length */
}

unsafe extern "C" fn write_marker_byte(mut cinfo: j_compress_ptr, mut val: i32)
/* Emit one byte of marker parameters following write_marker_header */
{
    emit_byte(cinfo, val);
}
/*
 * Write datastream header.
 * This consists of an SOI and optional APPn markers.
 * We recommend use of the JFIF marker, but not the Adobe marker,
 * when using YCbCr or grayscale data.  The JFIF marker should NOT
 * be used for any other JPEG colorspace.  The Adobe marker is helpful
 * to distinguish RGB, CMYK, and YCCK colorspaces.
 * Note that an application can write additional header markers after
 * jpeg_start_compress returns.
 */

unsafe extern "C" fn write_file_header(mut cinfo: j_compress_ptr) {
    let mut marker: my_marker_ptr = (*cinfo).marker as my_marker_ptr; /* first the SOI */
    emit_marker(cinfo, M_SOI);
    /* SOI is defined to reset restart interval to 0 */
    (*marker).last_restart_interval = 0 as i32 as u32;
    if (*cinfo).write_JFIF_header != 0 {
        /* next an optional JFIF APP0 */
        emit_jfif_app0(cinfo);
    }
    if (*cinfo).write_Adobe_marker != 0 {
        /* next an optional Adobe APP14 */
        emit_adobe_app14(cinfo);
    };
}
/*
 * Write frame header.
 * This consists of DQT and SOFn markers, and a conditional pseudo SOS marker.
 * Note that we do not emit the SOF until we have emitted the DQT(s).
 * This avoids compatibility problems with incorrect implementations that
 * try to error-check the quant table numbers as soon as they see the SOF.
 */

unsafe extern "C" fn write_frame_header(mut cinfo: j_compress_ptr) {
    let mut ci: i32 = 0;
    let mut prec: i32 = 0;
    let mut is_baseline: boolean = 0;
    let mut compptr: *mut jpeg_component_info = std::ptr::null_mut();
    /* Emit DQT for each quantization table.
     * Note that emit_dqt() suppresses any duplicate tables.
     */
    prec = 0 as i32;
    ci = 0 as i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        prec += emit_dqt(cinfo, (*compptr).quant_tbl_no);
        ci += 1;
        compptr = compptr.offset(1)
    }
    /* now prec is nonzero iff there are any 16-bit quant tables. */
    /* Check for a non-baseline specification.
     * Note we assume that Huffman table numbers won't be changed later.
     */
    if (*cinfo).arith_code != 0
        || (*cinfo).progressive_mode != 0
        || (*cinfo).data_precision != 8 as i32
        || (*cinfo).block_size != 8 as i32
    {
        is_baseline = 0 as i32
    } else {
        is_baseline = 1 as i32;
        ci = 0 as i32;
        compptr = (*cinfo).comp_info;
        while ci < (*cinfo).num_components {
            if (*compptr).dc_tbl_no > 1 as i32 || (*compptr).ac_tbl_no > 1 as i32 {
                is_baseline = 0 as i32
            }
            ci += 1;
            compptr = compptr.offset(1)
        }
        if prec != 0 && is_baseline != 0 {
            is_baseline = 0 as i32;
            /* If it's baseline except for quantizer size, warn the user */
            (*(*cinfo).err).msg_code = JTRC_16BIT_TABLES as i32;
            Some(
                (*(*cinfo).err)
                    .emit_message
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr, 0 as i32);
        }
    }
    /* Emit the proper SOF marker */
    if (*cinfo).arith_code != 0 {
        if (*cinfo).progressive_mode != 0 {
            emit_sof(cinfo, M_SOF10); /* SOF code for progressive arithmetic */
        } else {
            emit_sof(cinfo, M_SOF9);
        }
    /* SOF code for sequential arithmetic */
    } else if (*cinfo).progressive_mode != 0 {
        emit_sof(cinfo, M_SOF2); /* SOF code for progressive Huffman */
    } else if is_baseline != 0 {
        emit_sof(cinfo, M_SOF0); /* SOF code for baseline implementation */
    } else {
        emit_sof(cinfo, M_SOF1);
    }
    /* SOF code for non-baseline Huffman file */
    /* Check to emit pseudo SOS marker */
    if (*cinfo).progressive_mode != 0 && (*cinfo).block_size != 8 as i32 {
        emit_pseudo_sos(cinfo);
    };
}
/*
 * Write scan header.
 * This consists of DHT or DAC markers, optional DRI, and SOS.
 * Compressed data will be written following the SOS.
 */

unsafe extern "C" fn write_scan_header(mut cinfo: j_compress_ptr) {
    let mut marker: my_marker_ptr = (*cinfo).marker as my_marker_ptr;
    let mut i: i32 = 0;
    let mut compptr: *mut jpeg_component_info = std::ptr::null_mut();
    if (*cinfo).arith_code != 0 {
        /* Emit arith conditioning info.  We may have some duplication
         * if the file has multiple scans, but it's so small it's hardly
         * worth worrying about.
         */
        emit_dac(cinfo);
    } else {
        /* Emit Huffman tables.
         * Note that emit_dht() suppresses any duplicate tables.
         */
        i = 0 as i32;
        while i < (*cinfo).comps_in_scan {
            compptr = (*cinfo).cur_comp_info[i as usize];
            /* DC needs no table for refinement scan */
            if (*cinfo).Ss == 0 as i32 && (*cinfo).Ah == 0 as i32 {
                emit_dht(cinfo, (*compptr).dc_tbl_no, 0 as i32);
            }
            /* AC needs no table when not present */
            if (*cinfo).Se != 0 {
                emit_dht(cinfo, (*compptr).ac_tbl_no, 1 as i32);
            }
            i += 1
        }
    }
    /* Emit DRI if required --- note that DRI value could change for each scan.
     * We avoid wasting space with unnecessary DRIs, however.
     */
    if (*cinfo).restart_interval != (*marker).last_restart_interval {
        emit_dri(cinfo);
        (*marker).last_restart_interval = (*cinfo).restart_interval
    }
    emit_sos(cinfo);
}
/*
 * Write datastream trailer.
 */

unsafe extern "C" fn write_file_trailer(mut cinfo: j_compress_ptr) {
    emit_marker(cinfo, M_EOI);
}
/*
 * Write an abbreviated table-specification datastream.
 * This consists of SOI, DQT and DHT tables, and EOI.
 * Any table that is defined and not marked sent_table = TRUE will be
 * emitted.  Note that all tables will be marked sent_table = TRUE at exit.
 */

unsafe extern "C" fn write_tables_only(mut cinfo: j_compress_ptr) {
    let mut i: i32 = 0;
    emit_marker(cinfo, M_SOI);
    i = 0 as i32;
    while i < 4 as i32 {
        if !(*cinfo).quant_tbl_ptrs[i as usize].is_null() {
            emit_dqt(cinfo, i);
        }
        i += 1
    }
    if (*cinfo).arith_code == 0 {
        i = 0 as i32;
        while i < 4 as i32 {
            if !(*cinfo).dc_huff_tbl_ptrs[i as usize].is_null() {
                emit_dht(cinfo, i, 0 as i32);
            }
            if !(*cinfo).ac_huff_tbl_ptrs[i as usize].is_null() {
                emit_dht(cinfo, i, 1 as i32);
            }
            i += 1
        }
    }
    emit_marker(cinfo, M_EOI);
}
/*
 * Initialize the marker writer module.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_marker_writer(mut cinfo: j_compress_ptr) {
    let mut marker: my_marker_ptr = std::ptr::null_mut();
    /* Create the subobject */
    marker = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        1 as i32,
        ::std::mem::size_of::<my_marker_writer>() as usize,
    ) as my_marker_ptr;
    (*cinfo).marker = marker as *mut jpeg_marker_writer;
    /* Initialize method pointers */
    (*marker).pub_0.write_file_header =
        Some(write_file_header as unsafe extern "C" fn(_: j_compress_ptr) -> ());
    (*marker).pub_0.write_frame_header =
        Some(write_frame_header as unsafe extern "C" fn(_: j_compress_ptr) -> ());
    (*marker).pub_0.write_scan_header =
        Some(write_scan_header as unsafe extern "C" fn(_: j_compress_ptr) -> ());
    (*marker).pub_0.write_file_trailer =
        Some(write_file_trailer as unsafe extern "C" fn(_: j_compress_ptr) -> ());
    (*marker).pub_0.write_tables_only =
        Some(write_tables_only as unsafe extern "C" fn(_: j_compress_ptr) -> ());
    (*marker).pub_0.write_marker_header =
        Some(write_marker_header as unsafe extern "C" fn(_: j_compress_ptr, _: i32, _: u32) -> ());
    (*marker).pub_0.write_marker_byte =
        Some(write_marker_byte as unsafe extern "C" fn(_: j_compress_ptr, _: i32) -> ());
    /* Initialize private state */
    (*marker).last_restart_interval = 0 as i32 as u32;
}
