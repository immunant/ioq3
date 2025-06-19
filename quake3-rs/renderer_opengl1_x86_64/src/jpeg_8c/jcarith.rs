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
pub use crate::src::jpeg_8c::jaricom::jpeg_aritab;
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

pub type arith_entropy_ptr = *mut arith_entropy_encoder;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arith_entropy_encoder {
    pub pub_0: jpeg_entropy_encoder,
    pub c: INT32,
    pub a: INT32,
    pub sc: INT32,
    pub zc: INT32,
    pub ct: i32,
    pub buffer: i32,
    pub last_dc_val: [i32; 4],
    pub dc_context: [i32; 4],
    pub restarts_to_go: u32,
    pub next_restart_num: i32,
    pub dc_stats: [*mut u8; 16],
    pub ac_stats: [*mut u8; 16],
    pub fixed_bin: [u8; 4],
}
/* NOTE: Uncomment the following #define if you want to use the
 * given formula for calculating the AC conditioning parameter Kx
 * for spectral selection progressive coding in section G.1.3.2
 * of the spec (Kx = Kmin + SRL (8 + Se - Kmin) 4).
 * Although the spec and P&M authors claim that this "has proven
 * to give good results for 8 bit precision samples", I'm not
 * convinced yet that this is really beneficial.
 * Early tests gave only very marginal compression enhancements
 * (a few - around 5 or so - bytes even for very large files),
 * which would turn out rather negative if we'd suppress the
 * DAC (Define Arithmetic Conditioning) marker segments for
 * the default parameters in the future.
 * Note that currently the marker writing module emits 12-byte
 * DAC segments for a full-component scan in a color image.
 * This is not worth worrying about IMHO. However, since the
 * spec defines the default values to be used if the tables
 * are omitted (unlike Huffman tables, which are required
 * anyway), one might optimize this behaviour in the future,
 * and then it would be disadvantageous to use custom tables if
 * they don't provide sufficient gain to exceed the DAC size.
 *
 * On the other hand, I'd consider it as a reasonable result
 * that the conditioning has no significant influence on the
 * compression performance. This means that the basic
 * statistical model is already rather stable.
 *
 * Thus, at the moment, we use the default conditioning values
 * anyway, and do not use the custom formula.
 *
#define CALCULATE_SPECTRAL_CONDITIONING
 */
/* IRIGHT_SHIFT is like RIGHT_SHIFT, but works on int rather than INT32.
 * We assume that int right shift is unsigned if INT32 right shift is,
 * which should be safe.
 */

unsafe extern "C" fn emit_byte(mut val: i32, mut cinfo: j_compress_ptr)
/* Write next output byte; we do not support suspension in this module. */
{
    let mut dest: *mut jpeg_destination_mgr = (*cinfo).dest;
    let fresh0 = (*dest).next_output_byte;
    (*dest).next_output_byte = (*dest).next_output_byte.offset(1);
    *fresh0 = val as JOCTET;
    (*dest).free_in_buffer = (*dest).free_in_buffer.wrapping_sub(1);
    if (*dest).free_in_buffer == 0 as i32 as libc::c_ulong {
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
/*
 * Finish up at the end of an arithmetic-compressed scan.
 */

unsafe extern "C" fn finish_pass(mut cinfo: j_compress_ptr) {
    let mut e: arith_entropy_ptr = (*cinfo).entropy as arith_entropy_ptr;
    let mut temp: INT32 = 0;
    /* Section D.1.8: Termination of encoding */
    /* Find the e->c in the coding interval with the largest
     * number of trailing zero bits */
    temp = (*e).a - 1 as i32 as isize + (*e).c & 0xffff0000 as isize;
    if temp < (*e).c {
        (*e).c = temp + 0x8000 as isize
    } else {
        (*e).c = temp
    }
    /* Send remaining bytes to output */
    (*e).c <<= (*e).ct;
    if (*e).c & 0xf8000000 as isize != 0 {
        /* One final overflow has to be handled */
        if (*e).buffer >= 0 as i32 {
            if (*e).zc != 0 {
                loop {
                    emit_byte(0 as i32, cinfo); /* carry-over converts stacked 0xFF bytes to 0x00 */
                    (*e).zc -= 1;
                    if !((*e).zc != 0) {
                        break;
                    }
                }
            }
            emit_byte((*e).buffer + 1 as i32, cinfo);
            if (*e).buffer + 1 as i32 == 0xff as i32 {
                emit_byte(0 as i32, cinfo);
            }
        }
        (*e).zc += (*e).sc;
        (*e).sc = 0 as i32 as INT32
    } else {
        if (*e).buffer == 0 as i32 {
            (*e).zc += 1
        } else if (*e).buffer >= 0 as i32 {
            if (*e).zc != 0 {
                loop {
                    emit_byte(0 as i32, cinfo);
                    (*e).zc -= 1;
                    if !((*e).zc != 0) {
                        break;
                    }
                }
            }
            emit_byte((*e).buffer, cinfo);
        }
        if (*e).sc != 0 {
            if (*e).zc != 0 {
                loop {
                    emit_byte(0 as i32, cinfo);
                    (*e).zc -= 1;
                    if !((*e).zc != 0) {
                        break;
                    }
                }
            }
            loop {
                emit_byte(0xff as i32, cinfo);
                emit_byte(0 as i32, cinfo);
                (*e).sc -= 1;
                if !((*e).sc != 0) {
                    break;
                }
            }
        }
    }
    /* Output final bytes only if they are not 0x00 */
    if (*e).c & 0x7fff800 as isize != 0 {
        if (*e).zc != 0 {
            loop
            /* output final pending zero bytes */
            {
                emit_byte(0 as i32, cinfo);
                (*e).zc -= 1;
                if !((*e).zc != 0) {
                    break;
                }
            }
        }
        emit_byte(((*e).c >> 19 as i32 & 0xff as i32 as isize) as i32, cinfo);
        if (*e).c >> 19 as i32 & 0xff as i32 as isize == 0xff as i32 as isize {
            emit_byte(0 as i32, cinfo);
        }
        if (*e).c & 0x7f800 as isize != 0 {
            emit_byte(((*e).c >> 11 as i32 & 0xff as i32 as isize) as i32, cinfo);
            if (*e).c >> 11 as i32 & 0xff as i32 as isize == 0xff as i32 as isize {
                emit_byte(0 as i32, cinfo);
            }
        }
    };
}
/*
 * The core arithmetic encoding routine (common in JPEG and JBIG).
 * This needs to go as fast as possible.
 * Machine-dependent optimization facilities
 * are not utilized in this portable implementation.
 * However, this code should be fairly efficient and
 * may be a good base for further optimizations anyway.
 *
 * Parameter 'val' to be encoded may be 0 or 1 (binary decision).
 *
 * Note: I've added full "Pacman" termination support to the
 * byte output routines, which is equivalent to the optional
 * Discard_final_zeros procedure (Figure D.15) in the spec.
 * Thus, we always produce the shortest possible output
 * stream compliant to the spec (no trailing zero bytes,
 * except for FF stuffing).
 *
 * I've also introduced a new scheme for accessing
 * the probability estimation state machine table,
 * derived from Markus Kuhn's JBIG implementation.
 */

unsafe extern "C" fn arith_encode(mut cinfo: j_compress_ptr, mut st: *mut u8, mut val: i32) {
    let mut e: arith_entropy_ptr = (*cinfo).entropy as arith_entropy_ptr;
    let mut nl: u8 = 0;
    let mut nm: u8 = 0;
    let mut qe: INT32 = 0;
    let mut temp: INT32 = 0;
    let mut sv: i32 = 0;
    /* Fetch values from our compact representation of Table D.2:
     * Qe values and probability estimation state machine
     */
    sv = *st as i32; /* => Qe_Value */
    qe = *jpeg_aritab.as_ptr().offset((sv & 0x7f as i32) as isize); /* Next_Index_LPS + Switch_MPS */
    nl = (qe & 0xff as i32 as isize) as u8; /* Next_Index_MPS */
    qe >>= 8 as i32;
    nm = (qe & 0xff as i32 as isize) as u8;
    qe >>= 8 as i32;
    /* Encode & estimation procedures per sections D.1.4 & D.1.5 */
    (*e).a -= qe;
    if val != sv >> 7 as i32 {
        /* Encode the less probable symbol */
        if (*e).a >= qe {
            /* If the interval size (qe) for the less probable symbol (LPS)
             * is larger than the interval size for the MPS, then exchange
             * the two symbols for coding efficiency, otherwise code the LPS
             * as usual: */
            (*e).c += (*e).a;
            (*e).a = qe
        }
        *st = (sv & 0x80 as i32 ^ nl as i32) as u8
    /* Estimate_after_LPS */
    } else {
        /* Encode the more probable symbol */
        if (*e).a >= 0x8000 as isize {
            return;
        }
        if (*e).a < qe {
            /* A >= 0x8000 -> ready, no renormalization required */
            /* Estimate_after_MPS */
            /* If the interval size (qe) for the less probable symbol (LPS)
             * is larger than the interval size for the MPS, then exchange
             * the two symbols for coding efficiency: */
            (*e).c += (*e).a;
            (*e).a = qe
        }
        *st = (sv & 0x80 as i32 ^ nm as i32) as u8
    }
    loop
    /* Renormalization & data output per section D.1.6 */
    {
        (*e).a <<= 1 as i32;
        (*e).c <<= 1 as i32;
        (*e).ct -= 1;
        if (*e).ct == 0 as i32 {
            /* Another byte is ready for output */
            temp = (*e).c >> 19 as i32;
            if temp > 0xff as i32 as isize {
                /* Handle overflow over all stacked 0xFF bytes */
                if (*e).buffer >= 0 as i32 {
                    if (*e).zc != 0 {
                        loop {
                            emit_byte(0 as i32, cinfo);
                            (*e).zc -= 1;
                            if !((*e).zc != 0) {
                                break;
                            }
                        }
                    }
                    emit_byte((*e).buffer + 1 as i32, cinfo);
                    if (*e).buffer + 1 as i32 == 0xff as i32 {
                        emit_byte(0 as i32, cinfo);
                    }
                }
                /* new output byte, might overflow later */
                (*e).zc += (*e).sc; /* carry-over converts stacked 0xFF bytes to 0x00 */
                (*e).sc = 0 as i32 as INT32;
                (*e).buffer = (temp & 0xff as i32 as isize) as i32
            } else if temp == 0xff as i32 as isize {
                (*e).sc += 1
            /* Note: The 3 spacer bits in the C register guarantee
             * that the new buffer byte can't be 0xFF here
             * (see page 160 in the P&M JPEG book). */
            /* stack 0xFF byte (which might overflow later) */
            } else {
                /* Output all stacked 0xFF bytes, they will not overflow any more */
                if (*e).buffer == 0 as i32 {
                    (*e).zc += 1
                } else if (*e).buffer >= 0 as i32 {
                    if (*e).zc != 0 {
                        loop {
                            emit_byte(0 as i32, cinfo);
                            (*e).zc -= 1;
                            if !((*e).zc != 0) {
                                break;
                            }
                        }
                    }
                    emit_byte((*e).buffer, cinfo);
                }
                if (*e).sc != 0 {
                    if (*e).zc != 0 {
                        loop {
                            emit_byte(0 as i32, cinfo);
                            (*e).zc -= 1;
                            if !((*e).zc != 0) {
                                break;
                            }
                        }
                    }
                    loop {
                        emit_byte(0xff as i32, cinfo);
                        emit_byte(0 as i32, cinfo);
                        (*e).sc -= 1;
                        if !((*e).sc != 0) {
                            break;
                        }
                    }
                }
                (*e).buffer = (temp & 0xff as i32 as isize) as i32
                /* new output byte (can still overflow) */
            }
            (*e).c &= 0x7ffff as isize;
            (*e).ct += 8 as i32
        }
        if !((*e).a < 0x8000 as isize) {
            break;
        }
    }
}
/*
 * Emit a restart marker & resynchronize predictions.
 */

unsafe extern "C" fn emit_restart(mut cinfo: j_compress_ptr, mut restart_num: i32) {
    let mut entropy: arith_entropy_ptr = (*cinfo).entropy as arith_entropy_ptr;
    let mut ci: i32 = 0;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    finish_pass(cinfo);
    emit_byte(0xff as i32, cinfo);
    emit_byte(0xd0 as i32 + restart_num, cinfo);
    /* Re-initialize statistics areas */
    ci = 0 as i32;
    while ci < (*cinfo).comps_in_scan {
        compptr = (*cinfo).cur_comp_info[ci as usize];
        /* DC needs no table for refinement scan */
        if (*cinfo).Ss == 0 as i32 && (*cinfo).Ah == 0 as i32 {
            crate::stdlib::memset(
                (*entropy).dc_stats[(*compptr).dc_tbl_no as usize] as *mut libc::c_void,
                0 as i32,
                64 as i32 as size_t,
            );
            /* Reset DC predictions to 0 */
            (*entropy).last_dc_val[ci as usize] = 0 as i32;
            (*entropy).dc_context[ci as usize] = 0 as i32
        }
        /* AC needs no table when not present */
        if (*cinfo).Se != 0 {
            crate::stdlib::memset(
                (*entropy).ac_stats[(*compptr).ac_tbl_no as usize] as *mut libc::c_void,
                0 as i32,
                256 as i32 as size_t,
            );
        }
        ci += 1
    }
    /* Reset arithmetic encoding variables */
    (*entropy).c = 0 as i32 as INT32;
    (*entropy).a = 0x10000 as isize;
    (*entropy).sc = 0 as i32 as INT32;
    (*entropy).zc = 0 as i32 as INT32;
    (*entropy).ct = 11 as i32;
    (*entropy).buffer = -(1 as i32);
    /* empty */
}
/*
 * MCU encoding for DC initial scan (either spectral selection,
 * or first pass of successive approximation).
 */

unsafe extern "C" fn encode_mcu_DC_first(
    mut cinfo: j_compress_ptr,
    mut MCU_data: *mut JBLOCKROW,
) -> boolean {
    let mut entropy: arith_entropy_ptr = (*cinfo).entropy as arith_entropy_ptr;
    let mut block: JBLOCKROW = 0 as *mut JBLOCK;
    let mut st: *mut u8 = 0 as *mut u8;
    let mut blkn: i32 = 0;
    let mut ci: i32 = 0;
    let mut tbl: i32 = 0;
    let mut v: i32 = 0;
    let mut v2: i32 = 0;
    let mut m: i32 = 0;
    /* Emit restart marker if needed */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0 as i32 as u32 {
            emit_restart(cinfo, (*entropy).next_restart_num);
            (*entropy).restarts_to_go = (*cinfo).restart_interval;
            (*entropy).next_restart_num += 1;
            (*entropy).next_restart_num &= 7 as i32
        }
        (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1)
    }
    /* Encode the MCU data blocks */
    blkn = 0 as i32;
    while blkn < (*cinfo).blocks_in_MCU {
        block = *MCU_data.offset(blkn as isize);
        ci = (*cinfo).MCU_membership[blkn as usize];
        tbl = (*(*cinfo).cur_comp_info[ci as usize]).dc_tbl_no;
        /* Compute the DC value after the required point transform by Al.
         * This is simply an arithmetic right shift.
         */
        m = (*block)[0 as i32 as usize] as i32 >> (*cinfo).Al;
        /* Sections F.1.4.1 & F.1.4.4.1: Encoding of DC coefficients */
        /* Table F.4: Point to statistics bin S0 for DC coefficient coding */
        st = (*entropy).dc_stats[tbl as usize].offset((*entropy).dc_context[ci as usize] as isize);
        /* Figure F.4: Encode_DC_DIFF */
        v = m - (*entropy).last_dc_val[ci as usize];
        if v == 0 as i32 {
            arith_encode(cinfo, st, 0 as i32);
            (*entropy).dc_context[ci as usize] = 0 as i32
        /* zero diff category */
        } else {
            (*entropy).last_dc_val[ci as usize] = m;
            arith_encode(cinfo, st, 1 as i32);
            /* Figure F.6: Encoding nonzero value v */
            /* Figure F.7: Encoding the sign of v */
            if v > 0 as i32 {
                arith_encode(cinfo, st.offset(1 as i32 as isize), 0 as i32); /* Table F.4: SS = S0 + 1 */
                /* small positive diff category */
                st = st.offset(2 as i32 as isize); /* Table F.4: SP = S0 + 2 */
                (*entropy).dc_context[ci as usize] = 4 as i32
            } else {
                v = -v;
                /* small negative diff category */
                arith_encode(cinfo, st.offset(1 as i32 as isize), 1 as i32); /* Table F.4: SS = S0 + 1 */
                st = st.offset(3 as i32 as isize); /* Table F.4: SN = S0 + 3 */
                (*entropy).dc_context[ci as usize] = 8 as i32
            }
            /* Figure F.8: Encoding the magnitude category of v */
            m = 0 as i32; /* Table F.4: X1 = 20 */
            v -= 1 as i32;
            if v != 0 {
                arith_encode(cinfo, st, 1 as i32);
                m = 1 as i32;
                v2 = v;
                st = (*entropy).dc_stats[tbl as usize].offset(20 as i32 as isize);
                loop {
                    v2 >>= 1 as i32;
                    if !(v2 != 0) {
                        break;
                    }
                    arith_encode(cinfo, st, 1 as i32);
                    m <<= 1 as i32;
                    st = st.offset(1 as i32 as isize)
                }
            }
            arith_encode(cinfo, st, 0 as i32);
            /* Section F.1.4.4.1.2: Establish dc_context conditioning category */
            if m < ((1 as isize) << (*cinfo).arith_dc_L[tbl as usize] as i32 >> 1 as i32) as i32 {
                /* large diff category */
                (*entropy).dc_context[ci as usize] = 0 as i32
            } else if m
                > ((1 as isize) << (*cinfo).arith_dc_U[tbl as usize] as i32 >> 1 as i32) as i32
            {
                (*entropy).dc_context[ci as usize] += 8 as i32
            } /* zero diff category */
            /* Figure F.9: Encoding the magnitude bit pattern of v */
            st = st.offset(14 as i32 as isize);
            loop {
                m >>= 1 as i32;
                if !(m != 0) {
                    break;
                }
                arith_encode(cinfo, st, if m & v != 0 { 1 as i32 } else { 0 as i32 });
            }
        }
        blkn += 1
    }
    return 1 as i32;
}
/*
 * MCU encoding for AC initial scan (either spectral selection,
 * or first pass of successive approximation).
 */

unsafe extern "C" fn encode_mcu_AC_first(
    mut cinfo: j_compress_ptr,
    mut MCU_data: *mut JBLOCKROW,
) -> boolean {
    let mut entropy: arith_entropy_ptr = (*cinfo).entropy as arith_entropy_ptr;
    let mut block: JBLOCKROW = 0 as *mut JBLOCK;
    let mut st: *mut u8 = 0 as *mut u8;
    let mut tbl: i32 = 0;
    let mut k: i32 = 0;
    let mut ke: i32 = 0;
    let mut v: i32 = 0;
    let mut v2: i32 = 0;
    let mut m: i32 = 0;
    let mut natural_order: *const i32 = 0 as *const i32;
    /* Emit restart marker if needed */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0 as i32 as u32 {
            emit_restart(cinfo, (*entropy).next_restart_num);
            (*entropy).restarts_to_go = (*cinfo).restart_interval;
            (*entropy).next_restart_num += 1;
            (*entropy).next_restart_num &= 7 as i32
        }
        (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1)
    }
    natural_order = (*cinfo).natural_order;
    /* Encode the MCU data block */
    block = *MCU_data.offset(0 as i32 as isize);
    tbl = (*(*cinfo).cur_comp_info[0 as i32 as usize]).ac_tbl_no;
    /* Sections F.1.4.2 & F.1.4.4.2: Encoding of AC coefficients */
    /* Establish EOB (end-of-block) index */
    ke = (*cinfo).Se;
    while ke > 0 as i32 {
        /* We must apply the point transform by Al.  For AC coefficients this
         * is an integer division with rounding towards 0.  To do this portably
         * in C, we shift after obtaining the absolute value.
         */
        v = (*block)[*natural_order.offset(ke as isize) as usize] as i32;
        if v >= 0 as i32 {
            v >>= (*cinfo).Al;
            if v != 0 {
                break;
            }
        } else {
            v = -v;
            v >>= (*cinfo).Al;
            if v != 0 {
                break;
            }
        }
        ke -= 1
    }
    /* Figure F.5: Encode_AC_Coefficients */
    k = (*cinfo).Ss; /* EOB decision */
    while k <= ke {
        st = (*entropy).ac_stats[tbl as usize].offset((3 as i32 * (k - 1 as i32)) as isize);
        arith_encode(cinfo, st, 0 as i32);
        loop {
            v = (*block)[*natural_order.offset(k as isize) as usize] as i32;
            if v >= 0 as i32 {
                v >>= (*cinfo).Al;
                if v != 0 {
                    arith_encode(cinfo, st.offset(1 as i32 as isize), 1 as i32);
                    arith_encode(cinfo, (*entropy).fixed_bin.as_mut_ptr(), 0 as i32);
                    break;
                }
            } else {
                v = -v;
                v >>= (*cinfo).Al;
                if v != 0 {
                    arith_encode(cinfo, st.offset(1 as i32 as isize), 1 as i32);
                    arith_encode(cinfo, (*entropy).fixed_bin.as_mut_ptr(), 1 as i32);
                    break;
                }
            }
            arith_encode(cinfo, st.offset(1 as i32 as isize), 0 as i32);
            st = st.offset(3 as i32 as isize);
            k += 1
        }
        st = st.offset(2 as i32 as isize);
        /* Figure F.8: Encoding the magnitude category of v */
        m = 0 as i32;
        v -= 1 as i32;
        if v != 0 {
            arith_encode(cinfo, st, 1 as i32);
            m = 1 as i32;
            v2 = v;
            v2 >>= 1 as i32;
            if v2 != 0 {
                arith_encode(cinfo, st, 1 as i32);
                m <<= 1 as i32;
                st = (*entropy).ac_stats[tbl as usize].offset(
                    (if k <= (*cinfo).arith_ac_K[tbl as usize] as i32 {
                        189 as i32
                    } else {
                        217 as i32
                    }) as isize,
                );
                loop {
                    v2 >>= 1 as i32;
                    if !(v2 != 0) {
                        break;
                    }
                    arith_encode(cinfo, st, 1 as i32);
                    m <<= 1 as i32;
                    st = st.offset(1 as i32 as isize)
                }
            }
        }
        arith_encode(cinfo, st, 0 as i32);
        /* Figure F.9: Encoding the magnitude bit pattern of v */
        st = st.offset(14 as i32 as isize);
        loop {
            m >>= 1 as i32;
            if !(m != 0) {
                break;
            }
            arith_encode(cinfo, st, if m & v != 0 { 1 as i32 } else { 0 as i32 });
        }
        k += 1
    }
    /* Encode EOB decision only if k <= cinfo->Se */
    if k <= (*cinfo).Se {
        st = (*entropy).ac_stats[tbl as usize].offset((3 as i32 * (k - 1 as i32)) as isize);
        arith_encode(cinfo, st, 1 as i32);
    }
    return 1 as i32;
}
/*
 * MCU encoding for DC successive approximation refinement scan.
 */

unsafe extern "C" fn encode_mcu_DC_refine(
    mut cinfo: j_compress_ptr,
    mut MCU_data: *mut JBLOCKROW,
) -> boolean {
    let mut entropy: arith_entropy_ptr = (*cinfo).entropy as arith_entropy_ptr;
    let mut st: *mut u8 = 0 as *mut u8;
    let mut Al: i32 = 0;
    let mut blkn: i32 = 0;
    /* Emit restart marker if needed */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0 as i32 as u32 {
            emit_restart(cinfo, (*entropy).next_restart_num); /* use fixed probability estimation */
            (*entropy).restarts_to_go = (*cinfo).restart_interval;
            (*entropy).next_restart_num += 1;
            (*entropy).next_restart_num &= 7 as i32
        }
        (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1)
    }
    st = (*entropy).fixed_bin.as_mut_ptr();
    Al = (*cinfo).Al;
    /* Encode the MCU data blocks */
    blkn = 0 as i32;
    while blkn < (*cinfo).blocks_in_MCU {
        /* We simply emit the Al'th bit of the DC coefficient value. */
        arith_encode(
            cinfo,
            st,
            (*(*MCU_data.offset(blkn as isize)).offset(0 as i32 as isize))[0 as i32 as usize]
                as i32
                >> Al
                & 1 as i32,
        );
        blkn += 1
    }
    return 1 as i32;
}
/*
 * MCU encoding for AC successive approximation refinement scan.
 */

unsafe extern "C" fn encode_mcu_AC_refine(
    mut cinfo: j_compress_ptr,
    mut MCU_data: *mut JBLOCKROW,
) -> boolean {
    let mut entropy: arith_entropy_ptr = (*cinfo).entropy as arith_entropy_ptr;
    let mut block: JBLOCKROW = 0 as *mut JBLOCK;
    let mut st: *mut u8 = 0 as *mut u8;
    let mut tbl: i32 = 0;
    let mut k: i32 = 0;
    let mut ke: i32 = 0;
    let mut kex: i32 = 0;
    let mut v: i32 = 0;
    let mut natural_order: *const i32 = 0 as *const i32;
    /* Emit restart marker if needed */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0 as i32 as u32 {
            emit_restart(cinfo, (*entropy).next_restart_num);
            (*entropy).restarts_to_go = (*cinfo).restart_interval;
            (*entropy).next_restart_num += 1;
            (*entropy).next_restart_num &= 7 as i32
        }
        (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1)
    }
    natural_order = (*cinfo).natural_order;
    /* Encode the MCU data block */
    block = *MCU_data.offset(0 as i32 as isize);
    tbl = (*(*cinfo).cur_comp_info[0 as i32 as usize]).ac_tbl_no;
    /* Section G.1.3.3: Encoding of AC coefficients */
    /* Establish EOB (end-of-block) index */
    ke = (*cinfo).Se;
    while ke > 0 as i32 {
        /* We must apply the point transform by Al.  For AC coefficients this
         * is an integer division with rounding towards 0.  To do this portably
         * in C, we shift after obtaining the absolute value.
         */
        v = (*block)[*natural_order.offset(ke as isize) as usize] as i32;
        if v >= 0 as i32 {
            v >>= (*cinfo).Al;
            if v != 0 {
                break;
            }
        } else {
            v = -v;
            v >>= (*cinfo).Al;
            if v != 0 {
                break;
            }
        }
        ke -= 1
    }
    /* Establish EOBx (previous stage end-of-block) index */
    kex = ke;
    while kex > 0 as i32 {
        v = (*block)[*natural_order.offset(kex as isize) as usize] as i32;
        if v >= 0 as i32 {
            v >>= (*cinfo).Ah;
            if v != 0 {
                break;
            }
        } else {
            v = -v;
            v >>= (*cinfo).Ah;
            if v != 0 {
                break;
            }
        }
        kex -= 1
    }
    /* Figure G.10: Encode_AC_Coefficients_SA */
    k = (*cinfo).Ss; /* EOB decision */
    while k <= ke {
        st = (*entropy).ac_stats[tbl as usize].offset((3 as i32 * (k - 1 as i32)) as isize);
        if k > kex {
            arith_encode(cinfo, st, 0 as i32);
        }
        loop {
            v = (*block)[*natural_order.offset(k as isize) as usize] as i32;
            if v >= 0 as i32 {
                v >>= (*cinfo).Al;
                if v != 0 {
                    if v >> 1 as i32 != 0 {
                        /* previously nonzero coef */
                        arith_encode(cinfo, st.offset(2 as i32 as isize), v & 1 as i32);
                    } else {
                        /* newly nonzero coef */
                        arith_encode(cinfo, st.offset(1 as i32 as isize), 1 as i32);
                        arith_encode(cinfo, (*entropy).fixed_bin.as_mut_ptr(), 0 as i32);
                    }
                    break;
                }
            } else {
                v = -v;
                v >>= (*cinfo).Al;
                if v != 0 {
                    if v >> 1 as i32 != 0 {
                        /* previously nonzero coef */
                        arith_encode(cinfo, st.offset(2 as i32 as isize), v & 1 as i32);
                    } else {
                        /* newly nonzero coef */
                        arith_encode(cinfo, st.offset(1 as i32 as isize), 1 as i32);
                        arith_encode(cinfo, (*entropy).fixed_bin.as_mut_ptr(), 1 as i32);
                    }
                    break;
                }
            }
            arith_encode(cinfo, st.offset(1 as i32 as isize), 0 as i32);
            st = st.offset(3 as i32 as isize);
            k += 1
        }
        k += 1
    }
    /* Encode EOB decision only if k <= cinfo->Se */
    if k <= (*cinfo).Se {
        st = (*entropy).ac_stats[tbl as usize].offset((3 as i32 * (k - 1 as i32)) as isize);
        arith_encode(cinfo, st, 1 as i32);
    }
    return 1 as i32;
}
/*
 * Encode and output one MCU's worth of arithmetic-compressed coefficients.
 */

unsafe extern "C" fn encode_mcu(
    mut cinfo: j_compress_ptr,
    mut MCU_data: *mut JBLOCKROW,
) -> boolean {
    let mut entropy: arith_entropy_ptr = (*cinfo).entropy as arith_entropy_ptr;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    let mut block: JBLOCKROW = 0 as *mut JBLOCK;
    let mut st: *mut u8 = 0 as *mut u8;
    let mut blkn: i32 = 0;
    let mut ci: i32 = 0;
    let mut tbl: i32 = 0;
    let mut k: i32 = 0;
    let mut ke: i32 = 0;
    let mut v: i32 = 0;
    let mut v2: i32 = 0;
    let mut m: i32 = 0;
    let mut natural_order: *const i32 = 0 as *const i32;
    /* Emit restart marker if needed */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0 as i32 as u32 {
            emit_restart(cinfo, (*entropy).next_restart_num);
            (*entropy).restarts_to_go = (*cinfo).restart_interval;
            (*entropy).next_restart_num += 1;
            (*entropy).next_restart_num &= 7 as i32
        }
        (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1)
    }
    natural_order = (*cinfo).natural_order;
    /* Encode the MCU data blocks */
    blkn = 0 as i32;
    while blkn < (*cinfo).blocks_in_MCU {
        block = *MCU_data.offset(blkn as isize);
        ci = (*cinfo).MCU_membership[blkn as usize];
        compptr = (*cinfo).cur_comp_info[ci as usize];
        /* Sections F.1.4.1 & F.1.4.4.1: Encoding of DC coefficients */
        tbl = (*compptr).dc_tbl_no;
        /* Table F.4: Point to statistics bin S0 for DC coefficient coding */
        st = (*entropy).dc_stats[tbl as usize].offset((*entropy).dc_context[ci as usize] as isize);
        /* Figure F.4: Encode_DC_DIFF */
        v = (*block)[0 as i32 as usize] as i32 - (*entropy).last_dc_val[ci as usize];
        if v == 0 as i32 {
            arith_encode(cinfo, st, 0 as i32);
            (*entropy).dc_context[ci as usize] = 0 as i32
        /* zero diff category */
        } else {
            (*entropy).last_dc_val[ci as usize] = (*block)[0 as i32 as usize] as i32;
            arith_encode(cinfo, st, 1 as i32);
            /* Figure F.6: Encoding nonzero value v */
            /* Figure F.7: Encoding the sign of v */
            if v > 0 as i32 {
                arith_encode(cinfo, st.offset(1 as i32 as isize), 0 as i32); /* Table F.4: SS = S0 + 1 */
                /* small positive diff category */
                st = st.offset(2 as i32 as isize); /* Table F.4: SP = S0 + 2 */
                (*entropy).dc_context[ci as usize] = 4 as i32
            } else {
                v = -v;
                /* small negative diff category */
                arith_encode(cinfo, st.offset(1 as i32 as isize), 1 as i32); /* Table F.4: SS = S0 + 1 */
                st = st.offset(3 as i32 as isize); /* Table F.4: SN = S0 + 3 */
                (*entropy).dc_context[ci as usize] = 8 as i32
            }
            /* Figure F.8: Encoding the magnitude category of v */
            m = 0 as i32; /* Table F.4: X1 = 20 */
            v -= 1 as i32;
            if v != 0 {
                arith_encode(cinfo, st, 1 as i32);
                m = 1 as i32;
                v2 = v;
                st = (*entropy).dc_stats[tbl as usize].offset(20 as i32 as isize);
                loop {
                    v2 >>= 1 as i32;
                    if !(v2 != 0) {
                        break;
                    }
                    arith_encode(cinfo, st, 1 as i32);
                    m <<= 1 as i32;
                    st = st.offset(1 as i32 as isize)
                }
            }
            arith_encode(cinfo, st, 0 as i32);
            /* Section F.1.4.4.1.2: Establish dc_context conditioning category */
            if m < ((1 as isize) << (*cinfo).arith_dc_L[tbl as usize] as i32 >> 1 as i32) as i32 {
                /* large diff category */
                (*entropy).dc_context[ci as usize] = 0 as i32
            } else if m
                > ((1 as isize) << (*cinfo).arith_dc_U[tbl as usize] as i32 >> 1 as i32) as i32
            {
                (*entropy).dc_context[ci as usize] += 8 as i32
            } /* zero diff category */
            /* Figure F.9: Encoding the magnitude bit pattern of v */
            st = st.offset(14 as i32 as isize);
            loop {
                m >>= 1 as i32;
                if !(m != 0) {
                    break;
                }
                arith_encode(cinfo, st, if m & v != 0 { 1 as i32 } else { 0 as i32 });
            }
        }
        /* Sections F.1.4.2 & F.1.4.4.2: Encoding of AC coefficients */
        tbl = (*compptr).ac_tbl_no;
        /* Establish EOB (end-of-block) index */
        ke = (*cinfo).lim_Se;
        while ke > 0 as i32 {
            if (*block)[*natural_order.offset(ke as isize) as usize] != 0 {
                break;
            }
            ke -= 1
        }
        /* Figure F.5: Encode_AC_Coefficients */
        k = 1 as i32; /* EOB decision */
        while k <= ke {
            st = (*entropy).ac_stats[tbl as usize].offset((3 as i32 * (k - 1 as i32)) as isize);
            arith_encode(cinfo, st, 0 as i32);
            loop {
                v = (*block)[*natural_order.offset(k as isize) as usize] as i32;
                if !(v == 0 as i32) {
                    break;
                }
                arith_encode(cinfo, st.offset(1 as i32 as isize), 0 as i32);
                st = st.offset(3 as i32 as isize);
                k += 1
            }
            arith_encode(cinfo, st.offset(1 as i32 as isize), 1 as i32);
            /* Figure F.6: Encoding nonzero value v */
            /* Figure F.7: Encoding the sign of v */
            if v > 0 as i32 {
                arith_encode(cinfo, (*entropy).fixed_bin.as_mut_ptr(), 0 as i32);
            } else {
                v = -v;
                arith_encode(cinfo, (*entropy).fixed_bin.as_mut_ptr(), 1 as i32);
            }
            st = st.offset(2 as i32 as isize);
            /* Figure F.8: Encoding the magnitude category of v */
            m = 0 as i32;
            v -= 1 as i32;
            if v != 0 {
                arith_encode(cinfo, st, 1 as i32);
                m = 1 as i32;
                v2 = v;
                v2 >>= 1 as i32;
                if v2 != 0 {
                    arith_encode(cinfo, st, 1 as i32);
                    m <<= 1 as i32;
                    st = (*entropy).ac_stats[tbl as usize].offset(
                        (if k <= (*cinfo).arith_ac_K[tbl as usize] as i32 {
                            189 as i32
                        } else {
                            217 as i32
                        }) as isize,
                    );
                    loop {
                        v2 >>= 1 as i32;
                        if !(v2 != 0) {
                            break;
                        }
                        arith_encode(cinfo, st, 1 as i32);
                        m <<= 1 as i32;
                        st = st.offset(1 as i32 as isize)
                    }
                }
            }
            arith_encode(cinfo, st, 0 as i32);
            /* Figure F.9: Encoding the magnitude bit pattern of v */
            st = st.offset(14 as i32 as isize);
            loop {
                m >>= 1 as i32;
                if !(m != 0) {
                    break;
                }
                arith_encode(cinfo, st, if m & v != 0 { 1 as i32 } else { 0 as i32 });
            }
            k += 1
        }
        /* Encode EOB decision only if k <= cinfo->lim_Se */
        if k <= (*cinfo).lim_Se {
            st = (*entropy).ac_stats[tbl as usize].offset((3 as i32 * (k - 1 as i32)) as isize);
            arith_encode(cinfo, st, 1 as i32);
        }
        blkn += 1
    }
    return 1 as i32;
}
/*
 * Initialize for an arithmetic-compressed scan.
 */

unsafe extern "C" fn start_pass(mut cinfo: j_compress_ptr, mut gather_statistics: boolean) {
    let mut entropy: arith_entropy_ptr = (*cinfo).entropy as arith_entropy_ptr;
    let mut ci: i32 = 0;
    let mut tbl: i32 = 0;
    let mut compptr: *mut jpeg_component_info = 0 as *mut jpeg_component_info;
    if gather_statistics != 0 {
        /* Make sure to avoid that in the master control logic!
         * We are fully adaptive here and need no extra
         * statistics gathering pass!
         */
        (*(*cinfo).err).msg_code = JERR_NOT_COMPILED as i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* We assume jcmaster.c already validated the progressive scan parameters. */
    /* Select execution routines */
    if (*cinfo).progressive_mode != 0 {
        if (*cinfo).Ah == 0 as i32 {
            if (*cinfo).Ss == 0 as i32 {
                (*entropy).pub_0.encode_mcu = Some(
                    encode_mcu_DC_first
                        as unsafe extern "C" fn(_: j_compress_ptr, _: *mut JBLOCKROW) -> boolean,
                )
            } else {
                (*entropy).pub_0.encode_mcu = Some(
                    encode_mcu_AC_first
                        as unsafe extern "C" fn(_: j_compress_ptr, _: *mut JBLOCKROW) -> boolean,
                )
            }
        } else if (*cinfo).Ss == 0 as i32 {
            (*entropy).pub_0.encode_mcu = Some(
                encode_mcu_DC_refine
                    as unsafe extern "C" fn(_: j_compress_ptr, _: *mut JBLOCKROW) -> boolean,
            )
        } else {
            (*entropy).pub_0.encode_mcu = Some(
                encode_mcu_AC_refine
                    as unsafe extern "C" fn(_: j_compress_ptr, _: *mut JBLOCKROW) -> boolean,
            )
        }
    } else {
        (*entropy).pub_0.encode_mcu = Some(
            encode_mcu as unsafe extern "C" fn(_: j_compress_ptr, _: *mut JBLOCKROW) -> boolean,
        )
    }
    /* Allocate & initialize requested statistics areas */
    ci = 0 as i32;
    while ci < (*cinfo).comps_in_scan {
        compptr = (*cinfo).cur_comp_info[ci as usize];
        /* DC needs no table for refinement scan */
        if (*cinfo).Ss == 0 as i32 && (*cinfo).Ah == 0 as i32 {
            tbl = (*compptr).dc_tbl_no;
            if tbl < 0 as i32 || tbl >= 16 as i32 {
                (*(*cinfo).err).msg_code = JERR_NO_ARITH_TABLE as i32;
                (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = tbl;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            if (*entropy).dc_stats[tbl as usize].is_null() {
                (*entropy).dc_stats[tbl as usize] = Some(
                    (*(*cinfo).mem)
                        .alloc_small
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr,
                    1 as i32,
                    64 as i32 as size_t,
                ) as *mut u8
            }
            crate::stdlib::memset(
                (*entropy).dc_stats[tbl as usize] as *mut libc::c_void,
                0 as i32,
                64 as i32 as size_t,
            );
            /* Initialize DC predictions to 0 */
            (*entropy).last_dc_val[ci as usize] = 0 as i32;
            (*entropy).dc_context[ci as usize] = 0 as i32
        }
        /* AC needs no table when not present */
        if (*cinfo).Se != 0 {
            tbl = (*compptr).ac_tbl_no;
            if tbl < 0 as i32 || tbl >= 16 as i32 {
                (*(*cinfo).err).msg_code = JERR_NO_ARITH_TABLE as i32;
                (*(*cinfo).err).msg_parm.i[0 as i32 as usize] = tbl;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            if (*entropy).ac_stats[tbl as usize].is_null() {
                (*entropy).ac_stats[tbl as usize] = Some(
                    (*(*cinfo).mem)
                        .alloc_small
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr,
                    1 as i32,
                    256 as i32 as size_t,
                ) as *mut u8
            }
            crate::stdlib::memset(
                (*entropy).ac_stats[tbl as usize] as *mut libc::c_void,
                0 as i32,
                256 as i32 as size_t,
            );
        }
        ci += 1
    }
    /* Initialize arithmetic encoding variables */
    (*entropy).c = 0 as i32 as INT32; /* empty */
    (*entropy).a = 0x10000 as isize;
    (*entropy).sc = 0 as i32 as INT32;
    (*entropy).zc = 0 as i32 as INT32;
    (*entropy).ct = 11 as i32;
    (*entropy).buffer = -(1 as i32);
    /* Initialize restart stuff */
    (*entropy).restarts_to_go = (*cinfo).restart_interval;
    (*entropy).next_restart_num = 0 as i32;
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
/*
 * Module initialization routine for arithmetic entropy encoding.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_arith_encoder(mut cinfo: j_compress_ptr) {
    let mut entropy: arith_entropy_ptr = 0 as *mut arith_entropy_encoder;
    let mut i: i32 = 0;
    entropy = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        1 as i32,
        ::std::mem::size_of::<arith_entropy_encoder>() as libc::c_ulong,
    ) as arith_entropy_ptr;
    (*cinfo).entropy = entropy as *mut jpeg_entropy_encoder;
    (*entropy).pub_0.start_pass =
        Some(start_pass as unsafe extern "C" fn(_: j_compress_ptr, _: boolean) -> ());
    (*entropy).pub_0.finish_pass =
        Some(finish_pass as unsafe extern "C" fn(_: j_compress_ptr) -> ());
    /* Mark tables unallocated */
    i = 0 as i32;
    while i < 16 as i32 {
        (*entropy).dc_stats[i as usize] = 0 as *mut u8;
        (*entropy).ac_stats[i as usize] = 0 as *mut u8;
        i += 1
    }
    /* Initialize index for fixed probability estimation */
    (*entropy).fixed_bin[0 as i32 as usize] = 113 as i32 as u8;
}
