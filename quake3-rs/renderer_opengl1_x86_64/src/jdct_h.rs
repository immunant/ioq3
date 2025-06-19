pub type DCTELEM = libc::c_int;
/* 16 or 32 bits is fine */
pub type forward_DCT_method_ptr = Option<
    unsafe extern "C" fn(
        _: *mut DCTELEM,
        _: crate::jpeglib_h::JSAMPARRAY,
        _: crate::jmorecfg_h::JDIMENSION,
    ) -> (),
>;
pub type float_DCT_method_ptr = Option<
    unsafe extern "C" fn(
        _: *mut libc::c_float,
        _: crate::jpeglib_h::JSAMPARRAY,
        _: crate::jmorecfg_h::JDIMENSION,
    ) -> (),
>;
/*
 * An inverse DCT routine is given a pointer to the input JBLOCK and a pointer
 * to an output sample array.  The routine must dequantize the input data as
 * well as perform the IDCT; for dequantization, it uses the multiplier table
 * pointed to by compptr->dct_table.  The output data is to be placed into the
 * sample array starting at a specified column.  (Any row offset needed will
 * be applied to the array pointer before it is passed to the IDCT code.)
 * Note that the number of samples emitted by the IDCT routine is
 * DCT_h_scaled_size * DCT_v_scaled_size.
 */

/* typedef inverse_DCT_method_ptr is declared in jpegint.h */

/*
 * Each IDCT routine has its own ideas about the best dct_table element type.
 */
pub type ISLOW_MULT_TYPE = libc::c_int;
/* short or int, whichever is faster */
pub type IFAST_MULT_TYPE = libc::c_int;
/* 16 bits is OK, use short if faster */

/* fractional bits in scale factors */
pub type FLOAT_MULT_TYPE = libc::c_float;
