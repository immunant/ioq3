// =============== BEGIN inftrees_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct code {
    pub op: libc::c_uchar,
    pub bits: libc::c_uchar,
    pub val: libc::c_ushort,
}

pub type codetype = libc::c_uint;

pub const CODES: crate::src::zlib::inftrees::codetype = 0;

pub const LENS: crate::src::zlib::inftrees::codetype = 1;

pub const DISTS: crate::src::zlib::inftrees::codetype = 2;
use ::libc;

#[no_mangle]

pub static mut inflate_copyright: [libc::c_char; 47] = [
    32, 105, 110, 102, 108, 97, 116, 101, 32, 49, 46, 50, 46, 51, 32, 67, 111, 112, 121, 114, 105,
    103, 104, 116, 32, 49, 57, 57, 53, 45, 50, 48, 48, 53, 32, 77, 97, 114, 107, 32, 65, 100, 108,
    101, 114, 32, 0,
];
/*
 If you use the zlib library in a product, an acknowledgment is welcome
 in the documentation of your product. If for some reason you cannot
 include such an acknowledgment, I would appreciate that you keep this
 copyright string in the executable of your product.
*/
/*
  Build a set of tables to decode the provided canonical Huffman code.
  The code lengths are lens[0..codes-1].  The result starts at *table,
  whose indices are 0..2^bits-1.  work is a writable array of at least
  lens shorts, which is used as a work area.  type is the type of code
  to be generated, CODES, LENS, or DISTS.  On return, zero is success,
  -1 is an invalid code, and +1 means that ENOUGH isn't enough.  table
  on return points to the next available entry's address.  bits is the
  requested root table index bits, and on return it is the actual root
  table index bits.  It will differ if the request is greater than the
  longest code or if it is less than the shortest code.
*/
#[no_mangle]

pub unsafe extern "C" fn inflate_table(
    mut type_0: crate::src::zlib::inftrees::codetype,
    mut lens: *mut libc::c_ushort,
    mut codes: libc::c_uint,
    mut table: *mut *mut crate::src::zlib::inftrees::code,
    mut bits: *mut libc::c_uint,
    mut work: *mut libc::c_ushort,
) -> libc::c_int {
    let mut len: libc::c_uint = 0; /* a code's length in bits */
    let mut sym: libc::c_uint = 0; /* index of code symbols */
    let mut min: libc::c_uint = 0; /* minimum and maximum code lengths */
    let mut max: libc::c_uint = 0; /* number of index bits for root table */
    let mut root: libc::c_uint = 0; /* number of index bits for current table */
    let mut curr: libc::c_uint = 0; /* code bits to drop for sub-table */
    let mut drop_0: libc::c_uint = 0; /* number of prefix codes available */
    let mut left: libc::c_int = 0; /* code entries in table used */
    let mut used: libc::c_uint = 0; /* Huffman code */
    let mut huff: libc::c_uint = 0; /* for incrementing code, index */
    let mut incr: libc::c_uint = 0; /* index for replicating entries */
    let mut fill: libc::c_uint = 0; /* low bits for current root entry */
    let mut low: libc::c_uint = 0; /* mask for low root bits */
    let mut mask: libc::c_uint = 0; /* table entry for duplication */
    let mut this: crate::src::zlib::inftrees::code = crate::src::zlib::inftrees::code {
        op: 0,
        bits: 0,
        val: 0,
    }; /* next available space in table */
    let mut next: *mut crate::src::zlib::inftrees::code =
        0 as *mut crate::src::zlib::inftrees::code; /* base value table to use */
    let mut base: *const libc::c_ushort = 0 as *const libc::c_ushort; /* extra bits table to use */
    let mut extra: *const libc::c_ushort = 0 as *const libc::c_ushort; /* use base and extra for symbol > end */
    let mut end: libc::c_int = 0; /* number of codes of each length */
    let mut count: [libc::c_ushort; 16] = [0; 16]; /* offsets in table for each length */
    let mut offs: [libc::c_ushort; 16] = [0; 16];
    static mut lbase: [libc::c_ushort; 31] = [
        3 as libc::c_int as libc::c_ushort,
        4 as libc::c_int as libc::c_ushort,
        5 as libc::c_int as libc::c_ushort,
        6 as libc::c_int as libc::c_ushort,
        7 as libc::c_int as libc::c_ushort,
        8 as libc::c_int as libc::c_ushort,
        9 as libc::c_int as libc::c_ushort,
        10 as libc::c_int as libc::c_ushort,
        11 as libc::c_int as libc::c_ushort,
        13 as libc::c_int as libc::c_ushort,
        15 as libc::c_int as libc::c_ushort,
        17 as libc::c_int as libc::c_ushort,
        19 as libc::c_int as libc::c_ushort,
        23 as libc::c_int as libc::c_ushort,
        27 as libc::c_int as libc::c_ushort,
        31 as libc::c_int as libc::c_ushort,
        35 as libc::c_int as libc::c_ushort,
        43 as libc::c_int as libc::c_ushort,
        51 as libc::c_int as libc::c_ushort,
        59 as libc::c_int as libc::c_ushort,
        67 as libc::c_int as libc::c_ushort,
        83 as libc::c_int as libc::c_ushort,
        99 as libc::c_int as libc::c_ushort,
        115 as libc::c_int as libc::c_ushort,
        131 as libc::c_int as libc::c_ushort,
        163 as libc::c_int as libc::c_ushort,
        195 as libc::c_int as libc::c_ushort,
        227 as libc::c_int as libc::c_ushort,
        258 as libc::c_int as libc::c_ushort,
        0 as libc::c_int as libc::c_ushort,
        0 as libc::c_int as libc::c_ushort,
    ];
    static mut lext: [libc::c_ushort; 31] = [
        16 as libc::c_int as libc::c_ushort,
        16 as libc::c_int as libc::c_ushort,
        16 as libc::c_int as libc::c_ushort,
        16 as libc::c_int as libc::c_ushort,
        16 as libc::c_int as libc::c_ushort,
        16 as libc::c_int as libc::c_ushort,
        16 as libc::c_int as libc::c_ushort,
        16 as libc::c_int as libc::c_ushort,
        17 as libc::c_int as libc::c_ushort,
        17 as libc::c_int as libc::c_ushort,
        17 as libc::c_int as libc::c_ushort,
        17 as libc::c_int as libc::c_ushort,
        18 as libc::c_int as libc::c_ushort,
        18 as libc::c_int as libc::c_ushort,
        18 as libc::c_int as libc::c_ushort,
        18 as libc::c_int as libc::c_ushort,
        19 as libc::c_int as libc::c_ushort,
        19 as libc::c_int as libc::c_ushort,
        19 as libc::c_int as libc::c_ushort,
        19 as libc::c_int as libc::c_ushort,
        20 as libc::c_int as libc::c_ushort,
        20 as libc::c_int as libc::c_ushort,
        20 as libc::c_int as libc::c_ushort,
        20 as libc::c_int as libc::c_ushort,
        21 as libc::c_int as libc::c_ushort,
        21 as libc::c_int as libc::c_ushort,
        21 as libc::c_int as libc::c_ushort,
        21 as libc::c_int as libc::c_ushort,
        16 as libc::c_int as libc::c_ushort,
        201 as libc::c_int as libc::c_ushort,
        196 as libc::c_int as libc::c_ushort,
    ];
    static mut dbase: [libc::c_ushort; 32] = [
        1 as libc::c_int as libc::c_ushort,
        2 as libc::c_int as libc::c_ushort,
        3 as libc::c_int as libc::c_ushort,
        4 as libc::c_int as libc::c_ushort,
        5 as libc::c_int as libc::c_ushort,
        7 as libc::c_int as libc::c_ushort,
        9 as libc::c_int as libc::c_ushort,
        13 as libc::c_int as libc::c_ushort,
        17 as libc::c_int as libc::c_ushort,
        25 as libc::c_int as libc::c_ushort,
        33 as libc::c_int as libc::c_ushort,
        49 as libc::c_int as libc::c_ushort,
        65 as libc::c_int as libc::c_ushort,
        97 as libc::c_int as libc::c_ushort,
        129 as libc::c_int as libc::c_ushort,
        193 as libc::c_int as libc::c_ushort,
        257 as libc::c_int as libc::c_ushort,
        385 as libc::c_int as libc::c_ushort,
        513 as libc::c_int as libc::c_ushort,
        769 as libc::c_int as libc::c_ushort,
        1025 as libc::c_int as libc::c_ushort,
        1537 as libc::c_int as libc::c_ushort,
        2049 as libc::c_int as libc::c_ushort,
        3073 as libc::c_int as libc::c_ushort,
        4097 as libc::c_int as libc::c_ushort,
        6145 as libc::c_int as libc::c_ushort,
        8193 as libc::c_int as libc::c_ushort,
        12289 as libc::c_int as libc::c_ushort,
        16385 as libc::c_int as libc::c_ushort,
        24577 as libc::c_int as libc::c_ushort,
        0 as libc::c_int as libc::c_ushort,
        0 as libc::c_int as libc::c_ushort,
    ];
    static mut dext: [libc::c_ushort; 32] = [
        16 as libc::c_int as libc::c_ushort,
        16 as libc::c_int as libc::c_ushort,
        16 as libc::c_int as libc::c_ushort,
        16 as libc::c_int as libc::c_ushort,
        17 as libc::c_int as libc::c_ushort,
        17 as libc::c_int as libc::c_ushort,
        18 as libc::c_int as libc::c_ushort,
        18 as libc::c_int as libc::c_ushort,
        19 as libc::c_int as libc::c_ushort,
        19 as libc::c_int as libc::c_ushort,
        20 as libc::c_int as libc::c_ushort,
        20 as libc::c_int as libc::c_ushort,
        21 as libc::c_int as libc::c_ushort,
        21 as libc::c_int as libc::c_ushort,
        22 as libc::c_int as libc::c_ushort,
        22 as libc::c_int as libc::c_ushort,
        23 as libc::c_int as libc::c_ushort,
        23 as libc::c_int as libc::c_ushort,
        24 as libc::c_int as libc::c_ushort,
        24 as libc::c_int as libc::c_ushort,
        25 as libc::c_int as libc::c_ushort,
        25 as libc::c_int as libc::c_ushort,
        26 as libc::c_int as libc::c_ushort,
        26 as libc::c_int as libc::c_ushort,
        27 as libc::c_int as libc::c_ushort,
        27 as libc::c_int as libc::c_ushort,
        28 as libc::c_int as libc::c_ushort,
        28 as libc::c_int as libc::c_ushort,
        29 as libc::c_int as libc::c_ushort,
        29 as libc::c_int as libc::c_ushort,
        64 as libc::c_int as libc::c_ushort,
        64 as libc::c_int as libc::c_ushort,
    ];
    /*
      Process a set of code lengths to create a canonical Huffman code.  The
      code lengths are lens[0..codes-1].  Each length corresponds to the
      symbols 0..codes-1.  The Huffman code is generated by first sorting the
      symbols by length from short to long, and retaining the symbol order
      for codes with equal lengths.  Then the code starts with all zero bits
      for the first code of the shortest length, and the codes are integer
      increments for the same length, and zeros are appended as the length
      increases.  For the deflate format, these bits are stored backwards
      from their more natural integer increment ordering, and so when the
      decoding tables are built in the large loop below, the integer codes
      are incremented backwards.

      This routine assumes, but does not check, that all of the entries in
      lens[] are in the range 0..MAXBITS.  The caller must assure this.
      1..MAXBITS is interpreted as that code length.  zero means that that
      symbol does not occur in this code.

      The codes are sorted by computing a count of codes for each length,
      creating from that a table of starting indices for each length in the
      sorted table, and then entering the symbols in order in the sorted
      table.  The sorted table is work[], with that space being provided by
      the caller.

      The length counts are used for other purposes as well, i.e. finding
      the minimum and maximum length codes, determining if there are any
      codes at all, checking for a valid set of lengths, and looking ahead
      at length counts to determine sub-table sizes when building the
      decoding tables.
    */
    /* accumulate lengths for codes (assumes lens[] all in 0..MAXBITS) */
    len = 0 as libc::c_int as libc::c_uint;
    while len <= 15 as libc::c_int as libc::c_uint {
        count[len as usize] = 0 as libc::c_int as libc::c_ushort;
        len = len.wrapping_add(1)
    }
    sym = 0 as libc::c_int as libc::c_uint;
    while sym < codes {
        count[*lens.offset(sym as isize) as usize] =
            count[*lens.offset(sym as isize) as usize].wrapping_add(1);
        sym = sym.wrapping_add(1)
    }
    /* bound code lengths, force root to be within code lengths */
    root = *bits;
    max = 15 as libc::c_int as libc::c_uint;
    while max >= 1 as libc::c_int as libc::c_uint {
        if count[max as usize] as libc::c_int != 0 as libc::c_int {
            break;
        }
        max = max.wrapping_sub(1)
    }
    if root > max {
        root = max
    }
    if max == 0 as libc::c_int as libc::c_uint {
        /* no symbols to code at all */
        this.op = 64 as libc::c_int as libc::c_uchar;
        this.bits = 1 as libc::c_int as libc::c_uchar;
        this.val = 0 as libc::c_int as libc::c_ushort; /* invalid code marker */
        /* no symbols, but wait for decoding to report error */
        let fresh0 = *table; /* make a table to force an error */
        *table = (*table).offset(1);
        *fresh0 = this;
        let fresh1 = *table;
        *table = (*table).offset(1);
        *fresh1 = this;
        *bits = 1 as libc::c_int as libc::c_uint;
        return 0 as libc::c_int;
    }
    min = 1 as libc::c_int as libc::c_uint;
    while min <= 15 as libc::c_int as libc::c_uint {
        if count[min as usize] as libc::c_int != 0 as libc::c_int {
            break;
        }
        min = min.wrapping_add(1)
    }
    if root < min {
        root = min
    }
    /* check for an over-subscribed or incomplete set of lengths */
    left = 1 as libc::c_int;
    len = 1 as libc::c_int as libc::c_uint;
    while len <= 15 as libc::c_int as libc::c_uint {
        left <<= 1 as libc::c_int;
        left -= count[len as usize] as libc::c_int;
        if left < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        len = len.wrapping_add(1)
        /* over-subscribed */
    } /* incomplete set */
    if left > 0 as libc::c_int
        && (type_0 as libc::c_uint
            == crate::src::zlib::inftrees::CODES as libc::c_int as libc::c_uint
            || max != 1 as libc::c_int as libc::c_uint)
    {
        return -(1 as libc::c_int);
    }
    /* generate offsets into symbol table for each length for sorting */
    offs[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
    len = 1 as libc::c_int as libc::c_uint;
    while len < 15 as libc::c_int as libc::c_uint {
        offs[len.wrapping_add(1 as libc::c_int as libc::c_uint) as usize] =
            (offs[len as usize] as libc::c_int + count[len as usize] as libc::c_int)
                as libc::c_ushort;
        len = len.wrapping_add(1)
    }
    /* sort symbols by length, by symbol order within each length */
    sym = 0 as libc::c_int as libc::c_uint;
    while sym < codes {
        if *lens.offset(sym as isize) as libc::c_int != 0 as libc::c_int {
            let fresh2 = offs[*lens.offset(sym as isize) as usize];
            offs[*lens.offset(sym as isize) as usize] =
                offs[*lens.offset(sym as isize) as usize].wrapping_add(1);
            *work.offset(fresh2 as isize) = sym as libc::c_ushort
        }
        sym = sym.wrapping_add(1)
    }
    /*
      Create and fill in decoding tables.  In this loop, the table being
      filled is at next and has curr index bits.  The code being used is huff
      with length len.  That code is converted to an index by dropping drop
      bits off of the bottom.  For codes where len is less than drop + curr,
      those top drop + curr - len bits are incremented through all values to
      fill the table with replicated entries.

      root is the number of index bits for the root table.  When len exceeds
      root, sub-tables are created pointed to by the root entry with an index
      of the low root bits of huff.  This is saved in low to check for when a
      new sub-table should be started.  drop is zero when the root table is
      being filled, and drop is root when sub-tables are being filled.

      When a new sub-table is needed, it is necessary to look ahead in the
      code lengths to determine what size sub-table is needed.  The length
      counts are used for this, and so count[] is decremented as codes are
      entered in the tables.

      used keeps track of how many table entries have been allocated from the
      provided *table space.  It is checked when a LENS table is being made
      against the space in *table, ENOUGH, minus the maximum space needed by
      the worst case distance code, MAXD.  This should never happen, but the
      sufficiency of ENOUGH has not been proven exhaustively, hence the check.
      This assumes that when type == LENS, bits == 9.

      sym increments through all symbols, and the loop terminates when
      all codes of length max, i.e. all codes, have been processed.  This
      routine permits incomplete codes, so another loop after this one fills
      in the rest of the decoding tables with invalid code markers.
    */
    /* set up for code type */
    match type_0 as libc::c_uint {
        0 => {
            extra = work; /* dummy value--not used */
            base = extra;
            end = 19 as libc::c_int
        }
        1 => {
            base = lbase.as_ptr();
            base = base.offset(-(257 as libc::c_int as isize));
            extra = lext.as_ptr();
            extra = extra.offset(-(257 as libc::c_int as isize));
            end = 256 as libc::c_int
        }
        _ => {
            /* DISTS */
            base = dbase.as_ptr();
            extra = dext.as_ptr();
            end = -(1 as libc::c_int)
        }
    }
    /* initialize state for loop */
    huff = 0 as libc::c_int as libc::c_uint; /* starting code */
    sym = 0 as libc::c_int as libc::c_uint; /* starting code symbol */
    len = min; /* starting code length */
    next = *table; /* current table to fill in */
    curr = root; /* current table index bits */
    drop_0 = 0 as libc::c_int as libc::c_uint; /* current bits to drop from code for index */
    low = -(1 as libc::c_int) as libc::c_uint; /* trigger new sub-table when len > root */
    used = (1 as libc::c_uint) << root; /* use root table entries */
    mask = used.wrapping_sub(1 as libc::c_int as libc::c_uint); /* mask for comparing low */
    /* check available table space */
    if type_0 as libc::c_uint == crate::src::zlib::inftrees::LENS as libc::c_int as libc::c_uint
        && used >= (2048 as libc::c_int - 592 as libc::c_int) as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    loop
    /* process all codes and make table entries */
    /* create table entry */
    {
        this.bits = len.wrapping_sub(drop_0) as libc::c_uchar; /* end of block */
        if (*work.offset(sym as isize) as libc::c_int) < end {
            this.op = 0 as libc::c_int as libc::c_uchar;
            this.val = *work.offset(sym as isize)
        } else if *work.offset(sym as isize) as libc::c_int > end {
            this.op = *extra.offset(*work.offset(sym as isize) as isize) as libc::c_uchar;
            this.val = *base.offset(*work.offset(sym as isize) as isize)
        } else {
            this.op = (32 as libc::c_int + 64 as libc::c_int) as libc::c_uchar;
            this.val = 0 as libc::c_int as libc::c_ushort
        }
        /* replicate for those indices with low len bits equal to huff */
        incr = (1 as libc::c_uint) << len.wrapping_sub(drop_0); /* save offset to next table */
        fill = (1 as libc::c_uint) << curr;
        min = fill;
        loop {
            fill = fill.wrapping_sub(incr);
            *next.offset((huff >> drop_0).wrapping_add(fill) as isize) = this;
            if !(fill != 0 as libc::c_int as libc::c_uint) {
                break;
            }
        }
        /* backwards increment the len-bit code huff */
        incr = (1 as libc::c_uint) << len.wrapping_sub(1 as libc::c_int as libc::c_uint);
        while huff & incr != 0 {
            incr >>= 1 as libc::c_int
        }
        if incr != 0 as libc::c_int as libc::c_uint {
            huff &= incr.wrapping_sub(1 as libc::c_int as libc::c_uint);
            huff = huff.wrapping_add(incr)
        } else {
            huff = 0 as libc::c_int as libc::c_uint
        }
        /* go to next symbol, update count, len */
        sym = sym.wrapping_add(1);
        count[len as usize] = count[len as usize].wrapping_sub(1);
        if count[len as usize] as libc::c_int == 0 as libc::c_int {
            if len == max {
                break;
            }
            len = *lens.offset(*work.offset(sym as isize) as isize) as libc::c_uint
        }
        /* create new sub-table if needed */
        if len > root && huff & mask != low {
            /* if first time, transition to sub-tables */
            if drop_0 == 0 as libc::c_int as libc::c_uint {
                drop_0 = root
            }
            /* increment past last table */
            next = next.offset(min as isize); /* here min is 1 << curr */
            /* determine length of next table */
            curr = len.wrapping_sub(drop_0);
            left = (1 as libc::c_int) << curr;
            while curr.wrapping_add(drop_0) < max {
                left -= count[curr.wrapping_add(drop_0) as usize] as libc::c_int;
                if left <= 0 as libc::c_int {
                    break;
                }
                curr = curr.wrapping_add(1);
                left <<= 1 as libc::c_int
            }
            /* check for enough space */
            used = used.wrapping_add((1 as libc::c_uint) << curr);
            if type_0 as libc::c_uint
                == crate::src::zlib::inftrees::LENS as libc::c_int as libc::c_uint
                && used >= (2048 as libc::c_int - 592 as libc::c_int) as libc::c_uint
            {
                return 1 as libc::c_int;
            }
            /* point entry in root table to sub-table */
            low = huff & mask;
            (*(*table).offset(low as isize)).op = curr as libc::c_uchar;
            (*(*table).offset(low as isize)).bits = root as libc::c_uchar;
            (*(*table).offset(low as isize)).val =
                next.offset_from(*table) as libc::c_long as libc::c_ushort
        }
    }
    /*
      Fill in rest of table for incomplete codes.  This loop is similar to the
      loop above in incrementing huff for table indices.  It is assumed that
      len is equal to curr + drop, so there is no loop needed to increment
      through high index bits.  When the current sub-table is filled, the loop
      drops back to the root table to fill in any remaining entries there.
    */
    this.op = 64 as libc::c_int as libc::c_uchar; /* invalid code marker */
    this.bits = len.wrapping_sub(drop_0) as libc::c_uchar;
    this.val = 0 as libc::c_int as libc::c_ushort;
    while huff != 0 as libc::c_int as libc::c_uint {
        /* when done with sub-table, drop back to root table */
        if drop_0 != 0 as libc::c_int as libc::c_uint && huff & mask != low {
            drop_0 = 0 as libc::c_int as libc::c_uint;
            len = root;
            next = *table;
            this.bits = len as libc::c_uchar
        }
        /* put invalid code marker in table */
        *next.offset((huff >> drop_0) as isize) = this;
        /* backwards increment the len-bit code huff */
        incr = (1 as libc::c_uint) << len.wrapping_sub(1 as libc::c_int as libc::c_uint);
        while huff & incr != 0 {
            incr >>= 1 as libc::c_int
        }
        if incr != 0 as libc::c_int as libc::c_uint {
            huff &= incr.wrapping_sub(1 as libc::c_int as libc::c_uint);
            huff = huff.wrapping_add(incr)
        } else {
            huff = 0 as libc::c_int as libc::c_uint
        }
    }
    /* set return parameters */
    *table = (*table).offset(used as isize);
    *bits = root;
    return 0 as libc::c_int;
}
