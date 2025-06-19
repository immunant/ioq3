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
pub use crate::jpeglib_h::JCOEFPTR;
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
pub use crate::src::jpeg_8c::jcomapi::jpeg_alloc_huff_table;
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct c_derived_tbl {
    pub ehufco: [u32; 256],
    pub ehufsi: [libc::c_char; 256],
}

pub type huff_entropy_ptr = *mut huff_entropy_encoder;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct huff_entropy_encoder {
    pub pub_0: crate::jpegint_h::jpeg_entropy_encoder,
    pub saved: savable_state,
    pub restarts_to_go: u32,
    pub next_restart_num: i32,
    pub dc_derived_tbls: [*mut c_derived_tbl; 4],
    pub ac_derived_tbls: [*mut c_derived_tbl; 4],
    pub dc_count_ptrs: [*mut isize; 4],
    pub ac_count_ptrs: [*mut isize; 4],
    pub gather_statistics: crate::jmorecfg_h::boolean,
    pub next_output_byte: *mut crate::jmorecfg_h::JOCTET,
    pub free_in_buffer: crate::stddef_h::size_t,
    pub cinfo: crate::jpeglib_h::j_compress_ptr,
    pub ac_tbl_no: i32,
    pub EOBRUN: u32,
    pub BE: u32,
    pub bit_buffer: *mut libc::c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct savable_state {
    pub put_buffer: crate::jmorecfg_h::INT32,
    pub put_bits: i32,
    pub last_dc_val: [i32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct working_state {
    pub next_output_byte: *mut crate::jmorecfg_h::JOCTET,
    pub free_in_buffer: crate::stddef_h::size_t,
    pub cur: savable_state,
    pub cinfo: crate::jpeglib_h::j_compress_ptr,
}
/* Max # of correction bits I can buffer */
/* IRIGHT_SHIFT is like RIGHT_SHIFT, but works on int rather than INT32.
 * We assume that int right shift is unsigned if INT32 right shift is,
 * which should be safe.
 */
/*
 * Compute the derived values for a Huffman table.
 * This routine also performs some validation checks on the table.
 */

unsafe extern "C" fn jpeg_make_c_derived_tbl(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut isDC: crate::jmorecfg_h::boolean,
    mut tblno: i32,
    mut pdtbl: *mut *mut c_derived_tbl,
) {
    let mut htbl: *mut crate::jpeglib_h::JHUFF_TBL = 0 as *mut crate::jpeglib_h::JHUFF_TBL;
    let mut dtbl: *mut c_derived_tbl = 0 as *mut c_derived_tbl;
    let mut p: i32 = 0;
    let mut i: i32 = 0;
    let mut l: i32 = 0;
    let mut lastp: i32 = 0;
    let mut si: i32 = 0;
    let mut maxsymbol: i32 = 0;
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
            ::std::mem::size_of::<c_derived_tbl>() as libc::c_ulong,
        ) as *mut c_derived_tbl
    }
    dtbl = *pdtbl;
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
    lastp = p;
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
    /* Figure C.3: generate encoding tables */
    /* These are code and size indexed by symbol value */
    /* Set all codeless symbols to have code length 0;
     * this lets us detect duplicate VAL entries here, and later
     * allows emit_bits to detect any attempt to emit such symbols.
     */
    crate::stdlib::memset(
        (*dtbl).ehufsi.as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
    );
    /* This is also a convenient place to check for out-of-range
     * and duplicated VAL entries.  We allow 0..255 for AC symbols
     * but only 0..15 for DC.  (We could constrain them further
     * based on data depth and mode, but this seems enough.)
     */
    maxsymbol = if isDC != 0 { 15 as i32 } else { 255 as i32 };
    p = 0 as i32;
    while p < lastp {
        i = (*htbl).huffval[p as usize] as i32;
        if i < 0 as i32 || i > maxsymbol || (*dtbl).ehufsi[i as usize] as i32 != 0 {
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
        (*dtbl).ehufco[i as usize] = huffcode[p as usize];
        (*dtbl).ehufsi[i as usize] = huffsize[p as usize];
        p += 1
    }
}
/* Outputting bytes to the file.
 * NB: these must be called only when actually outputting,
 * that is, entropy->gather_statistics == FALSE.
 */
/* Emit a byte, taking 'action' if must suspend. */
/* Emit a byte */

unsafe extern "C" fn dump_buffer_s(mut state: *mut working_state) -> crate::jmorecfg_h::boolean
/* Empty the output buffer; return TRUE if successful, FALSE if must suspend */ {
    let mut dest: *mut crate::jpeglib_h::jpeg_destination_mgr = (*(*state).cinfo).dest;
    if Some(
        (*dest)
            .empty_output_buffer
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")((*state).cinfo)
        == 0
    {
        return 0 as i32;
    }
    /* After a successful buffer dump, must reset buffer pointers */
    (*state).next_output_byte = (*dest).next_output_byte;
    (*state).free_in_buffer = (*dest).free_in_buffer;
    return 1 as i32;
}

unsafe extern "C" fn dump_buffer_e(mut entropy: huff_entropy_ptr)
/* Empty the output buffer; we do not support suspension in this case. */
{
    let mut dest: *mut crate::jpeglib_h::jpeg_destination_mgr = (*(*entropy).cinfo).dest;
    if Some(
        (*dest)
            .empty_output_buffer
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")((*entropy).cinfo)
        == 0
    {
        (*(*(*entropy).cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_CANT_SUSPEND as i32;
        Some(
            (*(*(*entropy).cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            (*entropy).cinfo as crate::jpeglib_h::j_common_ptr
        );
    }
    /* After a successful buffer dump, must reset buffer pointers */
    (*entropy).next_output_byte = (*dest).next_output_byte;
    (*entropy).free_in_buffer = (*dest).free_in_buffer;
}
/* Outputting bits to the file */
/* Only the right 24 bits of put_buffer are used; the valid bits are
 * left-justified in this part.  At most 16 bits can be passed to emit_bits
 * in one call, and we never retain more than 7 bits in put_buffer
 * between calls, so 24 bits are sufficient.
 */
#[inline]

unsafe extern "C" fn emit_bits_s(
    mut state: *mut working_state,
    mut code: u32,
    mut size: i32,
) -> crate::jmorecfg_h::boolean
/* Emit some bits; return TRUE if successful, FALSE if must suspend */ {
    /* This routine is heavily used, so it's worth coding tightly. */
    let mut put_buffer: crate::jmorecfg_h::INT32 = code as crate::jmorecfg_h::INT32;
    let mut put_bits: i32 = (*state).cur.put_bits;
    /* if size is 0, caller used an invalid Huffman table entry */
    if size == 0 as i32 {
        (*(*(*state).cinfo).err).msg_code =
            crate::src::jpeg_8c::jerror::JERR_HUFF_MISSING_CODE as i32; /* mask off any extra bits in code */
        Some(
            (*(*(*state).cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            (*state).cinfo as crate::jpeglib_h::j_common_ptr
        ); /* new number of bits in buffer */
    } /* align incoming bits */
    put_buffer &= ((1 as i32 as crate::jmorecfg_h::INT32) << size) - 1 as i32 as isize; /* and merge with old buffer contents */
    put_bits += size;
    put_buffer <<= 24 as i32 - put_bits;
    put_buffer |= (*state).cur.put_buffer;
    while put_bits >= 8 as i32 {
        let mut c: i32 = (put_buffer >> 16 as i32 & 0xff as i32 as isize) as i32;
        let fresh3 = (*state).next_output_byte;
        (*state).next_output_byte = (*state).next_output_byte.offset(1);
        *fresh3 = c as crate::jmorecfg_h::JOCTET;
        (*state).free_in_buffer = (*state).free_in_buffer.wrapping_sub(1);
        if (*state).free_in_buffer == 0 as i32 as libc::c_ulong {
            if dump_buffer_s(state) == 0 {
                return 0 as i32;
            }
        }
        if c == 0xff as i32 {
            /* need to stuff a zero byte? */
            let fresh4 = (*state).next_output_byte; /* update state variables */
            (*state).next_output_byte = (*state).next_output_byte.offset(1);
            *fresh4 = 0 as i32 as crate::jmorecfg_h::JOCTET;
            (*state).free_in_buffer = (*state).free_in_buffer.wrapping_sub(1);
            if (*state).free_in_buffer == 0 as i32 as libc::c_ulong {
                if dump_buffer_s(state) == 0 {
                    return 0 as i32;
                }
            }
        }
        put_buffer <<= 8 as i32;
        put_bits -= 8 as i32
    }
    (*state).cur.put_buffer = put_buffer;
    (*state).cur.put_bits = put_bits;
    return 1 as i32;
}
#[inline]

unsafe extern "C" fn emit_bits_e(mut entropy: huff_entropy_ptr, mut code: u32, mut size: i32)
/* Emit some bits, unless we are in gather mode */
{
    /* This routine is heavily used, so it's worth coding tightly. */
    let mut put_buffer: crate::jmorecfg_h::INT32 = code as crate::jmorecfg_h::INT32;
    let mut put_bits: i32 = (*entropy).saved.put_bits;
    /* if size is 0, caller used an invalid Huffman table entry */
    if size == 0 as i32 {
        (*(*(*entropy).cinfo).err).msg_code =
            crate::src::jpeg_8c::jerror::JERR_HUFF_MISSING_CODE as i32; /* do nothing if we're only getting stats */
        Some(
            (*(*(*entropy).cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            (*entropy).cinfo as crate::jpeglib_h::j_common_ptr
        ); /* mask off any extra bits in code */
    } /* new number of bits in buffer */
    if (*entropy).gather_statistics != 0 {
        return;
    } /* align incoming bits */
    put_buffer &= ((1 as i32 as crate::jmorecfg_h::INT32) << size) - 1 as i32 as isize;
    put_bits += size;
    put_buffer <<= 24 as i32 - put_bits;
    /* and merge with old buffer contents */
    put_buffer |= (*entropy).saved.put_buffer;
    while put_bits >= 8 as i32 {
        let mut c: i32 = (put_buffer >> 16 as i32 & 0xff as i32 as isize) as i32;
        let fresh5 = (*entropy).next_output_byte;
        (*entropy).next_output_byte = (*entropy).next_output_byte.offset(1);
        *fresh5 = c as crate::jmorecfg_h::JOCTET;
        (*entropy).free_in_buffer = (*entropy).free_in_buffer.wrapping_sub(1);
        if (*entropy).free_in_buffer == 0 as i32 as libc::c_ulong {
            dump_buffer_e(entropy);
        }
        if c == 0xff as i32 {
            /* need to stuff a zero byte? */
            let fresh6 = (*entropy).next_output_byte; /* update variables */
            (*entropy).next_output_byte = (*entropy).next_output_byte.offset(1);
            *fresh6 = 0 as i32 as crate::jmorecfg_h::JOCTET;
            (*entropy).free_in_buffer = (*entropy).free_in_buffer.wrapping_sub(1);
            if (*entropy).free_in_buffer == 0 as i32 as libc::c_ulong {
                dump_buffer_e(entropy);
            }
        }
        put_buffer <<= 8 as i32;
        put_bits -= 8 as i32
    }
    (*entropy).saved.put_buffer = put_buffer;
    (*entropy).saved.put_bits = put_bits;
}

unsafe extern "C" fn flush_bits_s(mut state: *mut working_state) -> crate::jmorecfg_h::boolean {
    if emit_bits_s(state, 0x7f as i32 as u32, 7 as i32) == 0 {
        /* fill any partial byte with ones */
        return 0 as i32;
    } /* and reset bit-buffer to empty */
    (*state).cur.put_buffer = 0 as i32 as crate::jmorecfg_h::INT32; /* fill any partial byte with ones */
    (*state).cur.put_bits = 0 as i32; /* and reset bit-buffer to empty */
    return 1 as i32;
}

unsafe extern "C" fn flush_bits_e(mut entropy: huff_entropy_ptr) {
    emit_bits_e(entropy, 0x7f as i32 as u32, 7 as i32);
    (*entropy).saved.put_buffer = 0 as i32 as crate::jmorecfg_h::INT32;
    (*entropy).saved.put_bits = 0 as i32;
}
/*
 * Emit (or just count) a Huffman symbol.
 */
#[inline]

unsafe extern "C" fn emit_dc_symbol(
    mut entropy: huff_entropy_ptr,
    mut tbl_no: i32,
    mut symbol: i32,
) {
    if (*entropy).gather_statistics != 0 {
        let ref mut fresh7 = *(*entropy).dc_count_ptrs[tbl_no as usize].offset(symbol as isize);
        *fresh7 += 1
    } else {
        let mut tbl: *mut c_derived_tbl = (*entropy).dc_derived_tbls[tbl_no as usize];
        emit_bits_e(
            entropy,
            (*tbl).ehufco[symbol as usize],
            (*tbl).ehufsi[symbol as usize] as i32,
        );
    };
}
#[inline]

unsafe extern "C" fn emit_ac_symbol(
    mut entropy: huff_entropy_ptr,
    mut tbl_no: i32,
    mut symbol: i32,
) {
    if (*entropy).gather_statistics != 0 {
        let ref mut fresh8 = *(*entropy).ac_count_ptrs[tbl_no as usize].offset(symbol as isize);
        *fresh8 += 1
    } else {
        let mut tbl: *mut c_derived_tbl = (*entropy).ac_derived_tbls[tbl_no as usize];
        emit_bits_e(
            entropy,
            (*tbl).ehufco[symbol as usize],
            (*tbl).ehufsi[symbol as usize] as i32,
        );
    };
}
/*
 * Emit bits from a correction bit buffer.
 */

unsafe extern "C" fn emit_buffered_bits(
    mut entropy: huff_entropy_ptr,
    mut bufstart: *mut libc::c_char,
    mut nbits: u32,
) {
    if (*entropy).gather_statistics != 0 {
        return;
    } /* no real work */
    while nbits > 0 as i32 as u32 {
        emit_bits_e(entropy, *bufstart as u32, 1 as i32);
        bufstart = bufstart.offset(1);
        nbits = nbits.wrapping_sub(1)
    }
}
/*
 * Emit any pending EOBRUN symbol.
 */

unsafe extern "C" fn emit_eobrun(mut entropy: huff_entropy_ptr) {
    let mut temp: i32 = 0;
    let mut nbits: i32 = 0;
    if (*entropy).EOBRUN > 0 as i32 as u32 {
        /* if there is any pending EOBRUN */
        temp = (*entropy).EOBRUN as i32;
        nbits = 0 as i32;
        loop {
            temp >>= 1 as i32;
            if !(temp != 0) {
                break;
            }
            nbits += 1
        }
        /* safety check: shouldn't happen given limited correction-bit buffer */
        if nbits > 14 as i32 {
            (*(*(*entropy).cinfo).err).msg_code =
                crate::src::jpeg_8c::jerror::JERR_HUFF_MISSING_CODE as i32;
            Some(
                (*(*(*entropy).cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                (*entropy).cinfo as crate::jpeglib_h::j_common_ptr,
            );
        }
        emit_ac_symbol(entropy, (*entropy).ac_tbl_no, nbits << 4 as i32);
        if nbits != 0 {
            emit_bits_e(entropy, (*entropy).EOBRUN, nbits);
        }
        (*entropy).EOBRUN = 0 as i32 as u32;
        /* Emit any buffered correction bits */
        emit_buffered_bits(entropy, (*entropy).bit_buffer, (*entropy).BE);
        (*entropy).BE = 0 as i32 as u32
    };
}
/*
 * Emit a restart marker & resynchronize predictions.
 */

unsafe extern "C" fn emit_restart_s(
    mut state: *mut working_state,
    mut restart_num: i32,
) -> crate::jmorecfg_h::boolean {
    let mut ci: i32 = 0;
    if flush_bits_s(state) == 0 {
        return 0 as i32;
    }
    let fresh9 = (*state).next_output_byte;
    (*state).next_output_byte = (*state).next_output_byte.offset(1);
    *fresh9 = 0xff as i32 as crate::jmorecfg_h::JOCTET;
    (*state).free_in_buffer = (*state).free_in_buffer.wrapping_sub(1);
    if (*state).free_in_buffer == 0 as i32 as libc::c_ulong {
        if dump_buffer_s(state) == 0 {
            return 0 as i32;
        }
    }
    let fresh10 = (*state).next_output_byte;
    (*state).next_output_byte = (*state).next_output_byte.offset(1);
    *fresh10 = (0xd0 as i32 + restart_num) as crate::jmorecfg_h::JOCTET;
    (*state).free_in_buffer = (*state).free_in_buffer.wrapping_sub(1);
    if (*state).free_in_buffer == 0 as i32 as libc::c_ulong {
        if dump_buffer_s(state) == 0 {
            return 0 as i32;
        }
    }
    /* Re-initialize DC predictions to 0 */
    ci = 0 as i32;
    while ci < (*(*state).cinfo).comps_in_scan {
        (*state).cur.last_dc_val[ci as usize] = 0 as i32;
        ci += 1
    }
    /* The restart counter is not updated until we successfully write the MCU. */
    return 1 as i32;
}

unsafe extern "C" fn emit_restart_e(mut entropy: huff_entropy_ptr, mut restart_num: i32) {
    let mut ci: i32 = 0;
    emit_eobrun(entropy);
    if (*entropy).gather_statistics == 0 {
        flush_bits_e(entropy);
        let fresh11 = (*entropy).next_output_byte;
        (*entropy).next_output_byte = (*entropy).next_output_byte.offset(1);
        *fresh11 = 0xff as i32 as crate::jmorecfg_h::JOCTET;
        (*entropy).free_in_buffer = (*entropy).free_in_buffer.wrapping_sub(1);
        if (*entropy).free_in_buffer == 0 as i32 as libc::c_ulong {
            dump_buffer_e(entropy);
        }
        let fresh12 = (*entropy).next_output_byte;
        (*entropy).next_output_byte = (*entropy).next_output_byte.offset(1);
        *fresh12 = (0xd0 as i32 + restart_num) as crate::jmorecfg_h::JOCTET;
        (*entropy).free_in_buffer = (*entropy).free_in_buffer.wrapping_sub(1);
        if (*entropy).free_in_buffer == 0 as i32 as libc::c_ulong {
            dump_buffer_e(entropy);
        }
    }
    if (*(*entropy).cinfo).Ss == 0 as i32 {
        /* Re-initialize DC predictions to 0 */
        ci = 0 as i32;
        while ci < (*(*entropy).cinfo).comps_in_scan {
            (*entropy).saved.last_dc_val[ci as usize] = 0 as i32;
            ci += 1
        }
    } else {
        /* Re-initialize all AC-related fields to 0 */
        (*entropy).EOBRUN = 0 as i32 as u32;
        (*entropy).BE = 0 as i32 as u32
    };
}
/*
 * MCU encoding for DC initial scan (either spectral selection,
 * or first pass of successive approximation).
 */

unsafe extern "C" fn encode_mcu_DC_first(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut MCU_data: *mut crate::jpeglib_h::JBLOCKROW,
) -> crate::jmorecfg_h::boolean {
    let mut entropy: huff_entropy_ptr = (*cinfo).entropy as huff_entropy_ptr;
    let mut temp: i32 = 0;
    let mut temp2: i32 = 0;
    let mut nbits: i32 = 0;
    let mut blkn: i32 = 0;
    let mut ci: i32 = 0;
    let mut Al: i32 = (*cinfo).Al;
    let mut block: crate::jpeglib_h::JBLOCKROW = 0 as *mut crate::jpeglib_h::JBLOCK;
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        0 as *mut crate::jpeglib_h::jpeg_component_info;
    (*entropy).next_output_byte = (*(*cinfo).dest).next_output_byte;
    (*entropy).free_in_buffer = (*(*cinfo).dest).free_in_buffer;
    /* Emit restart marker if needed */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0 as i32 as u32 {
            emit_restart_e(entropy, (*entropy).next_restart_num);
        }
    }
    /* Encode the MCU data blocks */
    blkn = 0 as i32;
    while blkn < (*cinfo).blocks_in_MCU {
        block = *MCU_data.offset(blkn as isize);
        ci = (*cinfo).MCU_membership[blkn as usize];
        compptr = (*cinfo).cur_comp_info[ci as usize];
        /* Compute the DC value after the required point transform by Al.
         * This is simply an arithmetic right shift.
         */
        temp2 = (*block)[0 as i32 as usize] as i32 >> Al;
        /* DC differences are figured on the point-transformed values. */
        temp = temp2 - (*entropy).saved.last_dc_val[ci as usize];
        (*entropy).saved.last_dc_val[ci as usize] = temp2;
        /* Encode the DC coefficient difference per section G.1.2.1 */
        temp2 = temp; /* temp is abs value of input */
        if temp < 0 as i32 {
            temp = -temp;
            /* For a negative input, want temp2 = bitwise complement of abs(input) */
            /* This code assumes we are on a two's complement machine */
            temp2 -= 1
        }
        /* Find the number of bits needed for the magnitude of the coefficient */
        nbits = 0 as i32;
        while temp != 0 {
            nbits += 1;
            temp >>= 1 as i32
        }
        /* Check for out-of-range coefficient values.
         * Since we're encoding a difference, the range limit is twice as much.
         */
        if nbits > 10 as i32 + 1 as i32 {
            (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_DCT_COEF as i32;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
        /* Count/emit the Huffman-coded symbol for the number of bits */
        emit_dc_symbol(entropy, (*compptr).dc_tbl_no, nbits);
        /* Emit that number of bits of the value, if positive, */
        /* or the complement of its magnitude, if negative. */
        if nbits != 0 {
            /* emit_bits rejects calls with size 0 */
            emit_bits_e(entropy, temp2 as u32, nbits);
        }
        blkn += 1
    }
    (*(*cinfo).dest).next_output_byte = (*entropy).next_output_byte;
    (*(*cinfo).dest).free_in_buffer = (*entropy).free_in_buffer;
    /* Update restart-interval state too */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0 as i32 as u32 {
            (*entropy).restarts_to_go = (*cinfo).restart_interval;
            (*entropy).next_restart_num += 1;
            (*entropy).next_restart_num &= 7 as i32
        }
        (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1)
    }
    return 1 as i32;
}
/*
 * MCU encoding for AC initial scan (either spectral selection,
 * or first pass of successive approximation).
 */

unsafe extern "C" fn encode_mcu_AC_first(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut MCU_data: *mut crate::jpeglib_h::JBLOCKROW,
) -> crate::jmorecfg_h::boolean {
    let mut entropy: huff_entropy_ptr = (*cinfo).entropy as huff_entropy_ptr;
    let mut temp: i32 = 0;
    let mut temp2: i32 = 0;
    let mut nbits: i32 = 0;
    let mut r: i32 = 0;
    let mut k: i32 = 0;
    let mut Se: i32 = 0;
    let mut Al: i32 = 0;
    let mut natural_order: *const i32 = 0 as *const i32;
    let mut block: crate::jpeglib_h::JBLOCKROW = 0 as *mut crate::jpeglib_h::JBLOCK;
    (*entropy).next_output_byte = (*(*cinfo).dest).next_output_byte;
    (*entropy).free_in_buffer = (*(*cinfo).dest).free_in_buffer;
    /* Emit restart marker if needed */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0 as i32 as u32 {
            emit_restart_e(entropy, (*entropy).next_restart_num);
        }
    }
    Se = (*cinfo).Se;
    Al = (*cinfo).Al;
    natural_order = (*cinfo).natural_order;
    /* Encode the MCU data block */
    block = *MCU_data.offset(0 as i32 as isize);
    /* Encode the AC coefficients per section G.1.2.2, fig. G.3 */
    r = 0 as i32; /* r = run length of zeros */
    k = (*cinfo).Ss;
    while k <= Se {
        temp = (*block)[*natural_order.offset(k as isize) as usize] as i32;
        if temp == 0 as i32 {
            r += 1
        } else {
            /* reset zero run length */
            /* We must apply the point transform by Al.  For AC coefficients this
             * is an integer division with rounding towards 0.  To do this portably
             * in C, we shift after obtaining the absolute value; so the code is
             * interwoven with finding the abs value (temp) and output bits (temp2).
             */
            if temp < 0 as i32 {
                temp = -temp; /* temp is abs value of input */
                temp >>= Al; /* apply the point transform */
                /* For a negative coef, want temp2 = bitwise complement of abs(coef) */
                temp2 = !temp
            } else {
                temp >>= Al; /* apply the point transform */
                temp2 = temp
            }
            /* Watch out for case that nonzero coef is zero after point transform */
            if temp == 0 as i32 {
                r += 1
            } else {
                /* Emit any pending EOBRUN */
                if (*entropy).EOBRUN > 0 as i32 as u32 {
                    emit_eobrun(entropy);
                }
                /* if run length > 15, must emit special run-length-16 codes (0xF0) */
                while r > 15 as i32 {
                    emit_ac_symbol(entropy, (*entropy).ac_tbl_no, 0xf0 as i32);
                    r -= 16 as i32
                }
                /* Find the number of bits needed for the magnitude of the coefficient */
                nbits = 1 as i32; /* there must be at least one 1 bit */
                loop {
                    temp >>= 1 as i32;
                    if !(temp != 0) {
                        break;
                    }
                    nbits += 1
                }
                /* Check for out-of-range coefficient values */
                if nbits > 10 as i32 {
                    (*(*cinfo).err).msg_code =
                        crate::src::jpeg_8c::jerror::JERR_BAD_DCT_COEF as i32;
                    Some(
                        (*(*cinfo).err)
                            .error_exit
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as crate::jpeglib_h::j_common_ptr
                    );
                }
                /* Count/emit Huffman symbol for run length / number of bits */
                emit_ac_symbol(entropy, (*entropy).ac_tbl_no, (r << 4 as i32) + nbits);
                /* Emit that number of bits of the value, if positive, */
                /* or the complement of its magnitude, if negative. */
                emit_bits_e(entropy, temp2 as u32, nbits);
                r = 0 as i32
            }
        }
        k += 1
    }
    if r > 0 as i32 {
        /* If there are trailing zeroes, */
        (*entropy).EOBRUN = (*entropy).EOBRUN.wrapping_add(1);
        if (*entropy).EOBRUN == 0x7fff as i32 as u32 {
            emit_eobrun(entropy); /* count an EOB */
        }
        /* force it out to avoid overflow */
    }
    (*(*cinfo).dest).next_output_byte = (*entropy).next_output_byte;
    (*(*cinfo).dest).free_in_buffer = (*entropy).free_in_buffer;
    /* Update restart-interval state too */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0 as i32 as u32 {
            (*entropy).restarts_to_go = (*cinfo).restart_interval;
            (*entropy).next_restart_num += 1;
            (*entropy).next_restart_num &= 7 as i32
        }
        (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1)
    }
    return 1 as i32;
}
/*
 * MCU encoding for DC successive approximation refinement scan.
 * Note: we assume such scans can be multi-component, although the spec
 * is not very clear on the point.
 */

unsafe extern "C" fn encode_mcu_DC_refine(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut MCU_data: *mut crate::jpeglib_h::JBLOCKROW,
) -> crate::jmorecfg_h::boolean {
    let mut entropy: huff_entropy_ptr = (*cinfo).entropy as huff_entropy_ptr;
    let mut temp: i32 = 0;
    let mut blkn: i32 = 0;
    let mut Al: i32 = (*cinfo).Al;
    let mut block: crate::jpeglib_h::JBLOCKROW = 0 as *mut crate::jpeglib_h::JBLOCK;
    (*entropy).next_output_byte = (*(*cinfo).dest).next_output_byte;
    (*entropy).free_in_buffer = (*(*cinfo).dest).free_in_buffer;
    /* Emit restart marker if needed */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0 as i32 as u32 {
            emit_restart_e(entropy, (*entropy).next_restart_num);
        }
    }
    /* Encode the MCU data blocks */
    blkn = 0 as i32;
    while blkn < (*cinfo).blocks_in_MCU {
        block = *MCU_data.offset(blkn as isize);
        /* We simply emit the Al'th bit of the DC coefficient value. */
        temp = (*block)[0 as i32 as usize] as i32;
        emit_bits_e(entropy, (temp >> Al) as u32, 1 as i32);
        blkn += 1
    }
    (*(*cinfo).dest).next_output_byte = (*entropy).next_output_byte;
    (*(*cinfo).dest).free_in_buffer = (*entropy).free_in_buffer;
    /* Update restart-interval state too */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0 as i32 as u32 {
            (*entropy).restarts_to_go = (*cinfo).restart_interval;
            (*entropy).next_restart_num += 1;
            (*entropy).next_restart_num &= 7 as i32
        }
        (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1)
    }
    return 1 as i32;
}
/*
 * MCU encoding for AC successive approximation refinement scan.
 */

unsafe extern "C" fn encode_mcu_AC_refine(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut MCU_data: *mut crate::jpeglib_h::JBLOCKROW,
) -> crate::jmorecfg_h::boolean {
    let mut entropy: huff_entropy_ptr = (*cinfo).entropy as huff_entropy_ptr;
    let mut temp: i32 = 0;
    let mut r: i32 = 0;
    let mut k: i32 = 0;
    let mut EOB: i32 = 0;
    let mut BR_buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut BR: u32 = 0;
    let mut Se: i32 = 0;
    let mut Al: i32 = 0;
    let mut natural_order: *const i32 = 0 as *const i32;
    let mut block: crate::jpeglib_h::JBLOCKROW = 0 as *mut crate::jpeglib_h::JBLOCK;
    let mut absvalues: [i32; 64] = [0; 64];
    (*entropy).next_output_byte = (*(*cinfo).dest).next_output_byte;
    (*entropy).free_in_buffer = (*(*cinfo).dest).free_in_buffer;
    /* Emit restart marker if needed */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0 as i32 as u32 {
            emit_restart_e(entropy, (*entropy).next_restart_num);
        }
    }
    Se = (*cinfo).Se;
    Al = (*cinfo).Al;
    natural_order = (*cinfo).natural_order;
    /* Encode the MCU data block */
    block = *MCU_data.offset(0 as i32 as isize);
    /* It is convenient to make a pre-pass to determine the transformed
     * coefficients' absolute values and the EOB position.
     */
    EOB = 0 as i32;
    k = (*cinfo).Ss;
    while k <= Se {
        temp = (*block)[*natural_order.offset(k as isize) as usize] as i32;
        /* EOB = index of last newly-nonzero coef */
        if temp < 0 as i32 {
            temp = -temp
        }
        temp >>= Al;
        absvalues[k as usize] = temp;
        if temp == 1 as i32 {
            EOB = k
        }
        k += 1
    }
    /* We must apply the point transform by Al.  For AC coefficients this
     * is an integer division with rounding towards 0.  To do this portably
     * in C, we shift after obtaining the absolute value.
     */
    /* temp is abs value of input */
    /* apply the point transform */
    /* save abs value for main pass */
    /* Encode the AC coefficients per section G.1.2.3, fig. G.7 */
    r = 0 as i32; /* r = run length of zeros */
    BR = 0 as i32 as u32; /* BR = count of buffered bits added now */
    BR_buffer = (*entropy).bit_buffer.offset((*entropy).BE as isize); /* Append bits to buffer */
    k = (*cinfo).Ss;
    while k <= Se {
        temp = absvalues[k as usize];
        if temp == 0 as i32 {
            r += 1
        } else {
            /* reset zero run length */
            /* Emit any required ZRLs, but not if they can be folded into EOB */
            while r > 15 as i32 && k <= EOB {
                /* emit any pending EOBRUN and the BE correction bits */
                emit_eobrun(entropy);
                /* Emit ZRL */
                emit_ac_symbol(entropy, (*entropy).ac_tbl_no, 0xf0 as i32);
                r -= 16 as i32;
                /* Emit buffered correction bits that must be associated with ZRL */
                emit_buffered_bits(entropy, BR_buffer, BR); /* BE bits are gone now */
                BR_buffer = (*entropy).bit_buffer;
                BR = 0 as i32 as u32
            }
            /* If the coef was previously nonzero, it only needs a correction bit.
             * NOTE: a straight translation of the spec's figure G.7 would suggest
             * that we also need to test r > 15.  But if r > 15, we can only get here
             * if k > EOB, which implies that this coefficient is not 1.
             */
            if temp > 1 as i32 {
                /* The correction bit is the next bit of the absolute value. */
                let fresh13 = BR;
                BR = BR.wrapping_add(1);
                *BR_buffer.offset(fresh13 as isize) = (temp & 1 as i32) as libc::c_char
            } else {
                /* Emit any pending EOBRUN and the BE correction bits */
                emit_eobrun(entropy);
                /* Count/emit Huffman symbol for run length / number of bits */
                emit_ac_symbol(entropy, (*entropy).ac_tbl_no, (r << 4 as i32) + 1 as i32);
                /* Emit output bit for newly-nonzero coef */
                temp = if ((*block)[*natural_order.offset(k as isize) as usize] as i32) < 0 as i32 {
                    0 as i32
                } else {
                    1 as i32
                };
                emit_bits_e(entropy, temp as u32, 1 as i32);
                /* Emit buffered correction bits that must be associated with this code */
                emit_buffered_bits(entropy, BR_buffer, BR); /* BE bits are gone now */
                BR_buffer = (*entropy).bit_buffer;
                BR = 0 as i32 as u32;
                r = 0 as i32
            }
        }
        k += 1
    }
    if r > 0 as i32 || BR > 0 as i32 as u32 {
        /* If there are trailing zeroes, */
        (*entropy).EOBRUN = (*entropy).EOBRUN.wrapping_add(1); /* count an EOB */
        (*entropy).BE = (*entropy).BE.wrapping_add(BR); /* concat my correction bits to older ones */
        /* We force out the EOB if we risk either:
         * 1. overflow of the EOB counter;
         * 2. overflow of the correction bit buffer during the next MCU.
         */
        if (*entropy).EOBRUN == 0x7fff as i32 as u32
            || (*entropy).BE > (1000 as i32 - 64 as i32 + 1 as i32) as u32
        {
            emit_eobrun(entropy);
        }
    }
    (*(*cinfo).dest).next_output_byte = (*entropy).next_output_byte;
    (*(*cinfo).dest).free_in_buffer = (*entropy).free_in_buffer;
    /* Update restart-interval state too */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0 as i32 as u32 {
            (*entropy).restarts_to_go = (*cinfo).restart_interval;
            (*entropy).next_restart_num += 1;
            (*entropy).next_restart_num &= 7 as i32
        }
        (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1)
    }
    return 1 as i32;
}
/* Encode a single block's worth of coefficients */

unsafe extern "C" fn encode_one_block(
    mut state: *mut working_state,
    mut block: crate::jpeglib_h::JCOEFPTR,
    mut last_dc_val: i32,
    mut dctbl: *mut c_derived_tbl,
    mut actbl: *mut c_derived_tbl,
) -> crate::jmorecfg_h::boolean {
    let mut temp: i32 = 0;
    let mut temp2: i32 = 0;
    let mut nbits: i32 = 0;
    let mut k: i32 = 0;
    let mut r: i32 = 0;
    let mut i: i32 = 0;
    let mut Se: i32 = (*(*state).cinfo).lim_Se;
    let mut natural_order: *const i32 = (*(*state).cinfo).natural_order;
    /* Encode the DC coefficient difference per section F.1.2.1 */
    temp2 = *block.offset(0 as i32 as isize) as i32 - last_dc_val; /* temp is abs value of input */
    temp = temp2;
    if temp < 0 as i32 {
        temp = -temp;
        /* For a negative input, want temp2 = bitwise complement of abs(input) */
        /* This code assumes we are on a two's complement machine */
        temp2 -= 1
    }
    /* Find the number of bits needed for the magnitude of the coefficient */
    nbits = 0 as i32;
    while temp != 0 {
        nbits += 1;
        temp >>= 1 as i32
    }
    /* Check for out-of-range coefficient values.
     * Since we're encoding a difference, the range limit is twice as much.
     */
    if nbits > 10 as i32 + 1 as i32 {
        (*(*(*state).cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_DCT_COEF as i32;
        Some(
            (*(*(*state).cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            (*state).cinfo as crate::jpeglib_h::j_common_ptr
        );
    }
    /* Emit the Huffman-coded symbol for the number of bits */
    if emit_bits_s(
        state,
        (*dctbl).ehufco[nbits as usize],
        (*dctbl).ehufsi[nbits as usize] as i32,
    ) == 0
    {
        return 0 as i32;
    }
    /* Emit that number of bits of the value, if positive, */
    /* or the complement of its magnitude, if negative. */
    if nbits != 0 {
        /* emit_bits rejects calls with size 0 */
        if emit_bits_s(state, temp2 as u32, nbits) == 0 {
            return 0 as i32;
        }
    }
    /* Encode the AC coefficients per section F.1.2.2 */
    r = 0 as i32; /* r = run length of zeros */
    k = 1 as i32;
    while k <= Se {
        temp = *block.offset(*natural_order.offset(k as isize) as isize) as i32;
        if temp == 0 as i32 {
            r += 1
        } else {
            /* if run length > 15, must emit special run-length-16 codes (0xF0) */
            while r > 15 as i32 {
                if emit_bits_s(
                    state,
                    (*actbl).ehufco[0xf0 as i32 as usize],
                    (*actbl).ehufsi[0xf0 as i32 as usize] as i32,
                ) == 0
                {
                    return 0 as i32;
                } /* temp is abs value of input */
                r -= 16 as i32
            }
            temp2 = temp;
            if temp < 0 as i32 {
                temp = -temp;
                /* This code assumes we are on a two's complement machine */
                temp2 -= 1
            }
            /* Find the number of bits needed for the magnitude of the coefficient */
            nbits = 1 as i32; /* there must be at least one 1 bit */
            loop {
                temp >>= 1 as i32;
                if !(temp != 0) {
                    break;
                }
                nbits += 1
            }
            /* Check for out-of-range coefficient values */
            if nbits > 10 as i32 {
                (*(*(*state).cinfo).err).msg_code =
                    crate::src::jpeg_8c::jerror::JERR_BAD_DCT_COEF as i32;
                Some(
                    (*(*(*state).cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    (*state).cinfo as crate::jpeglib_h::j_common_ptr,
                );
            }
            /* Emit Huffman symbol for run length / number of bits */
            i = (r << 4 as i32) + nbits;
            if emit_bits_s(
                state,
                (*actbl).ehufco[i as usize],
                (*actbl).ehufsi[i as usize] as i32,
            ) == 0
            {
                return 0 as i32;
            }
            /* Emit that number of bits of the value, if positive, */
            /* or the complement of its magnitude, if negative. */
            if emit_bits_s(state, temp2 as u32, nbits) == 0 {
                return 0 as i32;
            }
            r = 0 as i32
        }
        k += 1
    }
    /* If the last coef(s) were zero, emit an end-of-block code */
    if r > 0 as i32 {
        if emit_bits_s(
            state,
            (*actbl).ehufco[0 as i32 as usize],
            (*actbl).ehufsi[0 as i32 as usize] as i32,
        ) == 0
        {
            return 0 as i32;
        }
    }
    return 1 as i32;
}
/*
 * Encode and output one MCU's worth of Huffman-compressed coefficients.
 */

unsafe extern "C" fn encode_mcu_huff(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut MCU_data: *mut crate::jpeglib_h::JBLOCKROW,
) -> crate::jmorecfg_h::boolean {
    let mut entropy: huff_entropy_ptr = (*cinfo).entropy as huff_entropy_ptr;
    let mut state: working_state = working_state {
        next_output_byte: 0 as *mut crate::jmorecfg_h::JOCTET,
        free_in_buffer: 0,
        cur: savable_state {
            put_buffer: 0,
            put_bits: 0,
            last_dc_val: [0; 4],
        },
        cinfo: 0 as *mut crate::jpeglib_h::jpeg_compress_struct,
    };
    let mut blkn: i32 = 0;
    let mut ci: i32 = 0;
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        0 as *mut crate::jpeglib_h::jpeg_component_info;
    /* Load up working state */
    state.next_output_byte = (*(*cinfo).dest).next_output_byte;
    state.free_in_buffer = (*(*cinfo).dest).free_in_buffer;
    state.cur = (*entropy).saved;
    state.cinfo = cinfo;
    /* Emit restart marker if needed */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0 as i32 as u32 {
            if emit_restart_s(&mut state, (*entropy).next_restart_num) == 0 {
                return 0 as i32;
            }
        }
    }
    /* Encode the MCU data blocks */
    blkn = 0 as i32;
    while blkn < (*cinfo).blocks_in_MCU {
        ci = (*cinfo).MCU_membership[blkn as usize];
        compptr = (*cinfo).cur_comp_info[ci as usize];
        if encode_one_block(
            &mut state,
            (*(*MCU_data.offset(blkn as isize)).offset(0 as i32 as isize)).as_mut_ptr(),
            state.cur.last_dc_val[ci as usize],
            (*entropy).dc_derived_tbls[(*compptr).dc_tbl_no as usize],
            (*entropy).ac_derived_tbls[(*compptr).ac_tbl_no as usize],
        ) == 0
        {
            return 0 as i32;
        }
        /* Update last_dc_val */
        state.cur.last_dc_val[ci as usize] = (*(*MCU_data.offset(blkn as isize))
            .offset(0 as i32 as isize))[0 as i32 as usize]
            as i32;
        blkn += 1
    }
    /* Completed MCU, so update state */
    (*(*cinfo).dest).next_output_byte = state.next_output_byte;
    (*(*cinfo).dest).free_in_buffer = state.free_in_buffer;
    (*entropy).saved = state.cur;
    /* Update restart-interval state too */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0 as i32 as u32 {
            (*entropy).restarts_to_go = (*cinfo).restart_interval;
            (*entropy).next_restart_num += 1;
            (*entropy).next_restart_num &= 7 as i32
        }
        (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1)
    }
    return 1 as i32;
}
/*
 * Finish up at the end of a Huffman-compressed scan.
 */

unsafe extern "C" fn finish_pass_huff(mut cinfo: crate::jpeglib_h::j_compress_ptr) {
    let mut entropy: huff_entropy_ptr = (*cinfo).entropy as huff_entropy_ptr;
    let mut state: working_state = working_state {
        next_output_byte: 0 as *mut crate::jmorecfg_h::JOCTET,
        free_in_buffer: 0,
        cur: savable_state {
            put_buffer: 0,
            put_bits: 0,
            last_dc_val: [0; 4],
        },
        cinfo: 0 as *mut crate::jpeglib_h::jpeg_compress_struct,
    };
    if (*cinfo).progressive_mode != 0 {
        (*entropy).next_output_byte = (*(*cinfo).dest).next_output_byte;
        (*entropy).free_in_buffer = (*(*cinfo).dest).free_in_buffer;
        /* Flush out any buffered data */
        emit_eobrun(entropy);
        flush_bits_e(entropy);
        (*(*cinfo).dest).next_output_byte = (*entropy).next_output_byte;
        (*(*cinfo).dest).free_in_buffer = (*entropy).free_in_buffer
    } else {
        /* Load up working state ... flush_bits needs it */
        state.next_output_byte = (*(*cinfo).dest).next_output_byte;
        state.free_in_buffer = (*(*cinfo).dest).free_in_buffer;
        state.cur = (*entropy).saved;
        state.cinfo = cinfo;
        /* Flush out the last data */
        if flush_bits_s(&mut state) == 0 {
            (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_CANT_SUSPEND as i32;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
        /* Update state */
        (*(*cinfo).dest).next_output_byte = state.next_output_byte;
        (*(*cinfo).dest).free_in_buffer = state.free_in_buffer;
        (*entropy).saved = state.cur
    };
}
/*
 * Huffman coding optimization.
 *
 * We first scan the supplied data and count the number of uses of each symbol
 * that is to be Huffman-coded. (This process MUST agree with the code above.)
 * Then we build a Huffman coding tree for the observed counts.
 * Symbols which are not needed at all for the particular image are not
 * assigned any code, which saves space in the DHT marker as well as in
 * the compressed data.
 */
/* Process a single block's worth of coefficients */

unsafe extern "C" fn htest_one_block(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut block: crate::jpeglib_h::JCOEFPTR,
    mut last_dc_val: i32,
    mut dc_counts: *mut isize,
    mut ac_counts: *mut isize,
) {
    let mut temp: i32 = 0;
    let mut nbits: i32 = 0;
    let mut k: i32 = 0;
    let mut r: i32 = 0;
    let mut Se: i32 = (*cinfo).lim_Se;
    let mut natural_order: *const i32 = (*cinfo).natural_order;
    /* Encode the DC coefficient difference per section F.1.2.1 */
    temp = *block.offset(0 as i32 as isize) as i32 - last_dc_val;
    if temp < 0 as i32 {
        temp = -temp
    }
    /* Find the number of bits needed for the magnitude of the coefficient */
    nbits = 0 as i32;
    while temp != 0 {
        nbits += 1;
        temp >>= 1 as i32
    }
    /* Check for out-of-range coefficient values.
     * Since we're encoding a difference, the range limit is twice as much.
     */
    if nbits > 10 as i32 + 1 as i32 {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_DCT_COEF as i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Count the Huffman symbol for the number of bits */
    let ref mut fresh14 = *dc_counts.offset(nbits as isize);
    *fresh14 += 1;
    /* Encode the AC coefficients per section F.1.2.2 */
    r = 0 as i32; /* r = run length of zeros */
    k = 1 as i32;
    while k <= Se {
        temp = *block.offset(*natural_order.offset(k as isize) as isize) as i32;
        if temp == 0 as i32 {
            r += 1
        } else {
            /* if run length > 15, must emit special run-length-16 codes (0xF0) */
            while r > 15 as i32 {
                let ref mut fresh15 = *ac_counts.offset(0xf0 as i32 as isize);
                *fresh15 += 1;
                r -= 16 as i32
            }
            /* Find the number of bits needed for the magnitude of the coefficient */
            if temp < 0 as i32 {
                temp = -temp
            }
            /* Find the number of bits needed for the magnitude of the coefficient */
            nbits = 1 as i32; /* there must be at least one 1 bit */
            loop {
                temp >>= 1 as i32;
                if !(temp != 0) {
                    break;
                }
                nbits += 1
            }
            /* Check for out-of-range coefficient values */
            if nbits > 10 as i32 {
                (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_BAD_DCT_COEF as i32;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            /* Count Huffman symbol for run length / number of bits */
            let ref mut fresh16 = *ac_counts.offset(((r << 4 as i32) + nbits) as isize);
            *fresh16 += 1;
            r = 0 as i32
        }
        k += 1
    }
    /* If the last coef(s) were zero, emit an end-of-block code */
    if r > 0 as i32 {
        let ref mut fresh17 = *ac_counts.offset(0 as i32 as isize);
        *fresh17 += 1
    };
}
/*
 * Trial-encode one MCU's worth of Huffman-compressed coefficients.
 * No data is actually output, so no suspension return is possible.
 */

unsafe extern "C" fn encode_mcu_gather(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut MCU_data: *mut crate::jpeglib_h::JBLOCKROW,
) -> crate::jmorecfg_h::boolean {
    let mut entropy: huff_entropy_ptr = (*cinfo).entropy as huff_entropy_ptr;
    let mut blkn: i32 = 0;
    let mut ci: i32 = 0;
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        0 as *mut crate::jpeglib_h::jpeg_component_info;
    /* Take care of restart intervals if needed */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0 as i32 as u32 {
            /* Re-initialize DC predictions to 0 */
            ci = 0 as i32;
            while ci < (*cinfo).comps_in_scan {
                (*entropy).saved.last_dc_val[ci as usize] = 0 as i32;
                ci += 1
            }
            /* Update restart state */
            (*entropy).restarts_to_go = (*cinfo).restart_interval
        }
        (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1)
    }
    blkn = 0 as i32;
    while blkn < (*cinfo).blocks_in_MCU {
        ci = (*cinfo).MCU_membership[blkn as usize];
        compptr = (*cinfo).cur_comp_info[ci as usize];
        htest_one_block(
            cinfo,
            (*(*MCU_data.offset(blkn as isize)).offset(0 as i32 as isize)).as_mut_ptr(),
            (*entropy).saved.last_dc_val[ci as usize],
            (*entropy).dc_count_ptrs[(*compptr).dc_tbl_no as usize],
            (*entropy).ac_count_ptrs[(*compptr).ac_tbl_no as usize],
        );
        (*entropy).saved.last_dc_val[ci as usize] = (*(*MCU_data.offset(blkn as isize))
            .offset(0 as i32 as isize))[0 as i32 as usize]
            as i32;
        blkn += 1
    }
    return 1 as i32;
}
/*
 * Generate the best Huffman code table for the given counts, fill htbl.
 *
 * The JPEG standard requires that no symbol be assigned a codeword of all
 * one bits (so that padding bits added at the end of a compressed segment
 * can't look like a valid code).  Because of the canonical ordering of
 * codewords, this just means that there must be an unused slot in the
 * longest codeword length category.  Section K.2 of the JPEG spec suggests
 * reserving such a slot by pretending that symbol 256 is a valid symbol
 * with count 1.  In theory that's not optimal; giving it count zero but
 * including it in the symbol set anyway should give a better Huffman code.
 * But the theoretically better code actually seems to come out worse in
 * practice, because it produces more all-ones bytes (which incur stuffed
 * zero bytes in the final file).  In any case the difference is tiny.
 *
 * The JPEG standard requires Huffman codes to be no more than 16 bits long.
 * If some symbols have a very small but nonzero probability, the Huffman tree
 * must be adjusted to meet the code length restriction.  We currently use
 * the adjustment method suggested in JPEG section K.2.  This method is *not*
 * optimal; it may not choose the best possible limited-length code.  But
 * typically only very-low-frequency symbols will be given less-than-optimal
 * lengths, so the code is almost optimal.  Experimental comparisons against
 * an optimal limited-length-code algorithm indicate that the difference is
 * microscopic --- usually less than a hundredth of a percent of total size.
 * So the extra complexity of an optimal algorithm doesn't seem worthwhile.
 */

unsafe extern "C" fn jpeg_gen_optimal_table(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut htbl: *mut crate::jpeglib_h::JHUFF_TBL,
    mut freq: *mut isize,
) {
    /* assumed maximum initial code length */
    let mut bits: [crate::jmorecfg_h::UINT8; 33] = [0; 33]; /* bits[k] = # of symbols with code length k */
    let mut codesize: [i32; 257] = [0; 257]; /* codesize[k] = code length of symbol k */
    let mut others: [i32; 257] = [0; 257]; /* next symbol in current branch of tree */
    let mut c1: i32 = 0;
    let mut c2: i32 = 0;
    let mut p: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut v: isize = 0;
    /* This algorithm is explained in section K.2 of the JPEG standard */
    crate::stdlib::memset(
        bits.as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<[crate::jmorecfg_h::UINT8; 33]>() as libc::c_ulong,
    ); /* init links to empty */
    crate::stdlib::memset(
        codesize.as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<[i32; 257]>() as libc::c_ulong,
    ); /* make sure 256 has a nonzero count */
    i = 0 as i32;
    while i < 257 as i32 {
        others[i as usize] = -(1 as i32);
        i += 1
    }
    *freq.offset(256 as i32 as isize) = 1 as i32 as isize;
    loop
    /* Including the pseudo-symbol 256 in the Huffman procedure guarantees
     * that no real symbol is given code-value of all ones, because 256
     * will be placed last in the largest codeword category.
     */
    /* Huffman's basic algorithm to assign optimal code lengths to symbols */
    /* Find the smallest nonzero frequency, set c1 = its symbol */
    /* In case of ties, take the larger symbol number */
    {
        c1 = -(1 as i32);
        v = 1000000000 as isize;
        i = 0 as i32;
        while i <= 256 as i32 {
            if *freq.offset(i as isize) != 0 && *freq.offset(i as isize) <= v {
                v = *freq.offset(i as isize);
                c1 = i
            }
            i += 1
        }
        /* Find the next smallest nonzero frequency, set c2 = its symbol */
        /* In case of ties, take the larger symbol number */
        c2 = -(1 as i32);
        v = 1000000000 as isize;
        i = 0 as i32;
        while i <= 256 as i32 {
            if *freq.offset(i as isize) != 0 && *freq.offset(i as isize) <= v && i != c1 {
                v = *freq.offset(i as isize);
                c2 = i
            }
            i += 1
        }
        /* Done if we've merged everything into one frequency */
        if c2 < 0 as i32 {
            break;
        }
        /* Else merge the two counts/trees */
        *freq.offset(c1 as isize) += *freq.offset(c2 as isize);
        *freq.offset(c2 as isize) = 0 as i32 as isize;
        /* Increment the codesize of everything in c1's tree branch */
        codesize[c1 as usize] += 1; /* chain c2 onto c1's tree branch */
        while others[c1 as usize] >= 0 as i32 {
            c1 = others[c1 as usize];
            codesize[c1 as usize] += 1
        }
        others[c1 as usize] = c2;
        /* Increment the codesize of everything in c2's tree branch */
        codesize[c2 as usize] += 1;
        while others[c2 as usize] >= 0 as i32 {
            c2 = others[c2 as usize];
            codesize[c2 as usize] += 1
        }
    }
    /* Now count the number of symbols of each code length */
    i = 0 as i32;
    while i <= 256 as i32 {
        if codesize[i as usize] != 0 {
            /* The JPEG standard seems to think that this can't happen, */
            /* but I'm paranoid... */
            if codesize[i as usize] > 32 as i32 {
                (*(*cinfo).err).msg_code =
                    crate::src::jpeg_8c::jerror::JERR_HUFF_CLEN_OVERFLOW as i32;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            bits[codesize[i as usize] as usize] =
                bits[codesize[i as usize] as usize].wrapping_add(1)
        }
        i += 1
    }
    /* JPEG doesn't allow symbols with code lengths over 16 bits, so if the pure
     * Huffman procedure assigned any such lengths, we must adjust the coding.
     * Here is what the JPEG spec says about how this next bit works:
     * Since symbols are paired for the longest Huffman code, the symbols are
     * removed from this length category two at a time.  The prefix for the pair
     * (which is one bit shorter) is allocated to one of the pair; then,
     * skipping the BITS entry for that prefix length, a code word from the next
     * shortest nonzero BITS entry is converted into a prefix for two code words
     * one bit longer.
     */
    i = 32 as i32; /* find length of new prefix to be used */
    while i > 16 as i32 {
        while bits[i as usize] as i32 > 0 as i32 {
            j = i - 2 as i32;
            while bits[j as usize] as i32 == 0 as i32 {
                j -= 1
            }
            /* symbol of this length is now a prefix */
            bits[i as usize] = (bits[i as usize] as i32 - 2 as i32) as crate::jmorecfg_h::UINT8; /* remove two symbols */
            bits[(i - 1 as i32) as usize] = bits[(i - 1 as i32) as usize].wrapping_add(1); /* one goes in this length */
            bits[(j + 1 as i32) as usize] =
                (bits[(j + 1 as i32) as usize] as i32 + 2 as i32) as crate::jmorecfg_h::UINT8; /* two new symbols in this length */
            bits[j as usize] = bits[j as usize].wrapping_sub(1)
        }
        i -= 1
    }
    /* Remove the count for the pseudo-symbol 256 from the largest codelength */
    while bits[i as usize] as i32 == 0 as i32 {
        /* find largest codelength still in use */
        i -= 1
    }
    bits[i as usize] = bits[i as usize].wrapping_sub(1);
    /* Return final symbol counts (only for lengths 0..16) */
    crate::stdlib::memcpy(
        (*htbl).bits.as_mut_ptr() as *mut libc::c_void,
        bits.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[crate::jmorecfg_h::UINT8; 17]>() as libc::c_ulong,
    );
    /* Return a list of the symbols sorted by code length */
    /* It's not real clear to me why we don't need to consider the codelength
     * changes made above, but the JPEG spec seems to think this works.
     */
    p = 0 as i32;
    i = 1 as i32;
    while i <= 32 as i32 {
        j = 0 as i32;
        while j <= 255 as i32 {
            if codesize[j as usize] == i {
                (*htbl).huffval[p as usize] = j as crate::jmorecfg_h::UINT8;
                p += 1
            }
            j += 1
        }
        i += 1
    }
    /* Set sent_table FALSE so updated table will be written to JPEG file. */
    (*htbl).sent_table = 0 as i32;
}
/*
 * Finish up a statistics-gathering pass and create the new Huffman tables.
 */

unsafe extern "C" fn finish_pass_gather(mut cinfo: crate::jpeglib_h::j_compress_ptr) {
    let mut entropy: huff_entropy_ptr = (*cinfo).entropy as huff_entropy_ptr;
    let mut ci: i32 = 0;
    let mut tbl: i32 = 0;
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        0 as *mut crate::jpeglib_h::jpeg_component_info;
    let mut htblptr: *mut *mut crate::jpeglib_h::JHUFF_TBL =
        0 as *mut *mut crate::jpeglib_h::JHUFF_TBL;
    let mut did_dc: [crate::jmorecfg_h::boolean; 4] = [0; 4];
    let mut did_ac: [crate::jmorecfg_h::boolean; 4] = [0; 4];
    /* It's important not to apply jpeg_gen_optimal_table more than once
     * per table, because it clobbers the input frequency counts!
     */
    if (*cinfo).progressive_mode != 0 {
        /* Flush out buffered data (all we care about is counting the EOB symbol) */
        emit_eobrun(entropy);
    }
    crate::stdlib::memset(
        did_dc.as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<[crate::jmorecfg_h::boolean; 4]>() as libc::c_ulong,
    );
    crate::stdlib::memset(
        did_ac.as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<[crate::jmorecfg_h::boolean; 4]>() as libc::c_ulong,
    );
    ci = 0 as i32;
    while ci < (*cinfo).comps_in_scan {
        compptr = (*cinfo).cur_comp_info[ci as usize];
        /* DC needs no table for refinement scan */
        if (*cinfo).Ss == 0 as i32 && (*cinfo).Ah == 0 as i32 {
            tbl = (*compptr).dc_tbl_no;
            if did_dc[tbl as usize] == 0 {
                htblptr = &mut *(*cinfo).dc_huff_tbl_ptrs.as_mut_ptr().offset(tbl as isize)
                    as *mut *mut crate::jpeglib_h::JHUFF_TBL;
                if (*htblptr).is_null() {
                    *htblptr = crate::src::jpeg_8c::jcomapi::jpeg_alloc_huff_table(
                        cinfo as crate::jpeglib_h::j_common_ptr
                            as *mut crate::jpeglib_h::jpeg_common_struct,
                    ) as *mut crate::jpeglib_h::JHUFF_TBL
                }
                jpeg_gen_optimal_table(cinfo, *htblptr, (*entropy).dc_count_ptrs[tbl as usize]);
                did_dc[tbl as usize] = 1 as i32
            }
        }
        /* AC needs no table when not present */
        if (*cinfo).Se != 0 {
            tbl = (*compptr).ac_tbl_no;
            if did_ac[tbl as usize] == 0 {
                htblptr = &mut *(*cinfo).ac_huff_tbl_ptrs.as_mut_ptr().offset(tbl as isize)
                    as *mut *mut crate::jpeglib_h::JHUFF_TBL;
                if (*htblptr).is_null() {
                    *htblptr = crate::src::jpeg_8c::jcomapi::jpeg_alloc_huff_table(
                        cinfo as crate::jpeglib_h::j_common_ptr
                            as *mut crate::jpeglib_h::jpeg_common_struct,
                    ) as *mut crate::jpeglib_h::JHUFF_TBL
                }
                jpeg_gen_optimal_table(cinfo, *htblptr, (*entropy).ac_count_ptrs[tbl as usize]);
                did_ac[tbl as usize] = 1 as i32
            }
        }
        ci += 1
    }
}
/*
 * Initialize for a Huffman-compressed scan.
 * If gather_statistics is TRUE, we do not output anything during the scan,
 * just count the Huffman symbols used and generate Huffman code tables.
 */

unsafe extern "C" fn start_pass_huff(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut gather_statistics: crate::jmorecfg_h::boolean,
) {
    let mut entropy: huff_entropy_ptr = (*cinfo).entropy as huff_entropy_ptr;
    let mut ci: i32 = 0;
    let mut tbl: i32 = 0;
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        0 as *mut crate::jpeglib_h::jpeg_component_info;
    if gather_statistics != 0 {
        (*entropy).pub_0.finish_pass = Some(
            finish_pass_gather as unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> (),
        )
    } else {
        (*entropy).pub_0.finish_pass = Some(
            finish_pass_huff as unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> (),
        )
    }
    if (*cinfo).progressive_mode != 0 {
        (*entropy).cinfo = cinfo;
        (*entropy).gather_statistics = gather_statistics;
        /* We assume jcmaster.c already validated the scan parameters. */
        /* Select execution routine */
        if (*cinfo).Ah == 0 as i32 {
            if (*cinfo).Ss == 0 as i32 {
                (*entropy).pub_0.encode_mcu = Some(
                    encode_mcu_DC_first
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_compress_ptr,
                            _: *mut crate::jpeglib_h::JBLOCKROW,
                        )
                            -> crate::jmorecfg_h::boolean,
                )
            } else {
                (*entropy).pub_0.encode_mcu = Some(
                    encode_mcu_AC_first
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_compress_ptr,
                            _: *mut crate::jpeglib_h::JBLOCKROW,
                        )
                            -> crate::jmorecfg_h::boolean,
                )
            }
        } else if (*cinfo).Ss == 0 as i32 {
            (*entropy).pub_0.encode_mcu = Some(
                encode_mcu_DC_refine
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_compress_ptr,
                        _: *mut crate::jpeglib_h::JBLOCKROW,
                    ) -> crate::jmorecfg_h::boolean,
            )
        } else {
            (*entropy).pub_0.encode_mcu = Some(
                encode_mcu_AC_refine
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_compress_ptr,
                        _: *mut crate::jpeglib_h::JBLOCKROW,
                    ) -> crate::jmorecfg_h::boolean,
            );
            /* AC refinement needs a correction bit buffer */
            if (*entropy).bit_buffer.is_null() {
                (*entropy).bit_buffer = Some(
                    (*(*cinfo).mem)
                        .alloc_small
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr,
                    1 as i32,
                    (1000 as i32 as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
                ) as *mut libc::c_char
            }
        }
        /* Initialize AC stuff */
        (*entropy).ac_tbl_no = (*(*cinfo).cur_comp_info[0 as i32 as usize]).ac_tbl_no;
        (*entropy).EOBRUN = 0 as i32 as u32;
        (*entropy).BE = 0 as i32 as u32
    } else if gather_statistics != 0 {
        (*entropy).pub_0.encode_mcu = Some(
            encode_mcu_gather
                as unsafe extern "C" fn(
                    _: crate::jpeglib_h::j_compress_ptr,
                    _: *mut crate::jpeglib_h::JBLOCKROW,
                ) -> crate::jmorecfg_h::boolean,
        )
    } else {
        (*entropy).pub_0.encode_mcu = Some(
            encode_mcu_huff
                as unsafe extern "C" fn(
                    _: crate::jpeglib_h::j_compress_ptr,
                    _: *mut crate::jpeglib_h::JBLOCKROW,
                ) -> crate::jmorecfg_h::boolean,
        )
    }
    ci = 0 as i32;
    while ci < (*cinfo).comps_in_scan {
        compptr = (*cinfo).cur_comp_info[ci as usize];
        /* DC needs no table for refinement scan */
        if (*cinfo).Ss == 0 as i32 && (*cinfo).Ah == 0 as i32 {
            tbl = (*compptr).dc_tbl_no;
            if gather_statistics != 0 {
                /* Check for invalid table index */
                /* (make_c_derived_tbl does this in the other path) */
                if tbl < 0 as i32 || tbl >= 4 as i32 {
                    (*(*cinfo).err).msg_code =
                        crate::src::jpeg_8c::jerror::JERR_NO_HUFF_TABLE as i32;
                    (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = tbl;
                    Some(
                        (*(*cinfo).err)
                            .error_exit
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as crate::jpeglib_h::j_common_ptr
                    );
                }
                /* Allocate and zero the statistics tables */
                /* Note that jpeg_gen_optimal_table expects 257 entries in each table! */
                if (*entropy).dc_count_ptrs[tbl as usize].is_null() {
                    (*entropy).dc_count_ptrs[tbl as usize] = Some(
                        (*(*cinfo).mem)
                            .alloc_small
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as crate::jpeglib_h::j_common_ptr,
                        1 as i32,
                        (257 as i32 as libc::c_ulong)
                            .wrapping_mul(::std::mem::size_of::<isize>() as libc::c_ulong),
                    )
                        as *mut isize
                }
                crate::stdlib::memset(
                    (*entropy).dc_count_ptrs[tbl as usize] as *mut libc::c_void,
                    0 as i32,
                    (257 as i32 as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<isize>() as libc::c_ulong),
                );
            } else {
                /* Compute derived values for Huffman tables */
                /* We may do this more than once for a table, but it's not expensive */
                jpeg_make_c_derived_tbl(
                    cinfo,
                    1 as i32,
                    tbl,
                    &mut *(*entropy).dc_derived_tbls.as_mut_ptr().offset(tbl as isize),
                );
            }
            /* Initialize DC predictions to 0 */
            (*entropy).saved.last_dc_val[ci as usize] = 0 as i32
        }
        /* AC needs no table when not present */
        if (*cinfo).Se != 0 {
            tbl = (*compptr).ac_tbl_no;
            if gather_statistics != 0 {
                if tbl < 0 as i32 || tbl >= 4 as i32 {
                    (*(*cinfo).err).msg_code =
                        crate::src::jpeg_8c::jerror::JERR_NO_HUFF_TABLE as i32;
                    (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = tbl;
                    Some(
                        (*(*cinfo).err)
                            .error_exit
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as crate::jpeglib_h::j_common_ptr
                    );
                }
                if (*entropy).ac_count_ptrs[tbl as usize].is_null() {
                    (*entropy).ac_count_ptrs[tbl as usize] = Some(
                        (*(*cinfo).mem)
                            .alloc_small
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as crate::jpeglib_h::j_common_ptr,
                        1 as i32,
                        (257 as i32 as libc::c_ulong)
                            .wrapping_mul(::std::mem::size_of::<isize>() as libc::c_ulong),
                    )
                        as *mut isize
                }
                crate::stdlib::memset(
                    (*entropy).ac_count_ptrs[tbl as usize] as *mut libc::c_void,
                    0 as i32,
                    (257 as i32 as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<isize>() as libc::c_ulong),
                );
            } else {
                jpeg_make_c_derived_tbl(
                    cinfo,
                    0 as i32,
                    tbl,
                    &mut *(*entropy).ac_derived_tbls.as_mut_ptr().offset(tbl as isize),
                );
            }
        }
        ci += 1
    }
    /* Initialize bit buffer to empty */
    (*entropy).saved.put_buffer = 0 as i32 as crate::jmorecfg_h::INT32;
    (*entropy).saved.put_bits = 0 as i32;
    /* Initialize restart stuff */
    (*entropy).restarts_to_go = (*cinfo).restart_interval;
    (*entropy).next_restart_num = 0 as i32;
}
/*
 * Module initialization routine for Huffman entropy encoding.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_huff_encoder(mut cinfo: crate::jpeglib_h::j_compress_ptr) {
    let mut entropy: huff_entropy_ptr = 0 as *mut huff_entropy_encoder;
    let mut i: i32 = 0;
    entropy = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        1 as i32,
        ::std::mem::size_of::<huff_entropy_encoder>() as libc::c_ulong,
    ) as huff_entropy_ptr;
    (*cinfo).entropy = entropy as *mut crate::jpegint_h::jpeg_entropy_encoder;
    (*entropy).pub_0.start_pass = Some(
        start_pass_huff
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_compress_ptr,
                _: crate::jmorecfg_h::boolean,
            ) -> (),
    );
    /* Mark tables unallocated */
    i = 0 as i32;
    while i < 4 as i32 {
        (*entropy).ac_derived_tbls[i as usize] = 0 as *mut c_derived_tbl;
        (*entropy).dc_derived_tbls[i as usize] = (*entropy).ac_derived_tbls[i as usize];
        (*entropy).ac_count_ptrs[i as usize] = 0 as *mut isize;
        (*entropy).dc_count_ptrs[i as usize] = (*entropy).ac_count_ptrs[i as usize];
        i += 1
    }
    if (*cinfo).progressive_mode != 0 {
        (*entropy).bit_buffer = 0 as *mut libc::c_char
    };
    /* needed only in AC refinement scan */
}
