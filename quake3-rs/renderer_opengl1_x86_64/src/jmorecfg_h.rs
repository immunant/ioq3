pub type JSAMPLE = u8;
/* not HAVE_UNSIGNED_CHAR */

/* HAVE_UNSIGNED_CHAR */

/* BITS_IN_JSAMPLE == 8 */

/* BITS_IN_JSAMPLE == 12 */

/* Representation of a DCT frequency coefficient.
 * This should be a signed value of at least 16 bits; "short" is usually OK.
 * Again, we allocate large arrays of these, but you can change to int
 * if you have memory to burn and "short" is really slow.
 */
pub type JCOEF = i16;
/* Compressed datastreams are represented as arrays of JOCTET.
 * These must be EXACTLY 8 bits wide, at least once they are written to
 * external storage.  Note that when using the stdio data source/destination
 * managers, this is also the data type passed to fread/fwrite.
 */
pub type JOCTET = u8;
/* not HAVE_UNSIGNED_CHAR */

/* HAVE_UNSIGNED_CHAR */

/* These typedefs are used for various table entries and so forth.
 * They must be at least as wide as specified; but making them too big
 * won't cost a huge amount of memory, so we don't provide special
 * extraction code like we did for JSAMPLE.  (In other words, these
 * typedefs live at a different point on the speed/space tradeoff curve.)
 */

/* UINT8 must hold at least the values 0..255. */
pub type UINT8 = u8;
pub type UINT16 = u16;
pub type INT16 = i16;
pub type INT32 = libc::c_long;
/* Datatype used for image dimensions.  The JPEG standard only supports
 * images up to 64K*64K due to 16-bit fields in SOF markers.  Therefore
 * "unsigned int" is sufficient on all machines.  However, if you need to
 * handle larger images and you don't mind deviating from the spec, you
 * can change this datatype.
 */
pub type JDIMENSION = u32;
/* a tad under 64K to prevent overflows */

/* These macros are used in all function definitions and extern declarations.
 * You could modify them if you need to change function linkage conventions;
 * in particular, you'll need to do that to make the library a Windows DLL.
 * Another application is to make all functions global for use with debuggers
 * or code profilers that require it.
 */

/* a function called through method pointers: */

/* a function used only in its module: */

/* a function referenced thru EXTERNs: */

/* a reference to a GLOBAL function: */

/* This macro is used to declare a "method", that is, a function pointer.
 * We want to supply prototype parameters if the compiler can cope.
 * Note that the arglist parameter must be parenthesized!
 * Again, you can customize this if you need special linkage keywords.
 */

/* Here is the pseudo-keyword for declaring pointers that must be "far"
 * on 80x86 machines.  Most of the specialized coding for 80x86 is handled
 * by just saying "FAR *" where such a pointer is needed.  In a few places
 * explicit coding is needed; see uses of the NEED_FAR_POINTERS symbol.
 */

/*
 * On a few systems, type boolean and/or its values FALSE, TRUE may appear
 * in standard header files.  Or you may have conflicts with application-
 * specific header files that you want to include together with these files.
 * Defining HAVE_BOOLEAN before including jpeglib.h should make it work.
 */
pub type boolean = i32;
