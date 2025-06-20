// =============== BEGIN inftrees_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct code {
    pub op: u8,
    pub bits: u8,
    pub val: u16,
}

pub type codetype = u32;

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
    mut lens: *mut u16,
    mut codes: u32,
    mut table: *mut *mut crate::src::zlib::inftrees::code,
    mut bits: *mut u32,
    mut work: *mut u16,
) -> i32 {
    let mut len: u32 = 0; /* a code's length in bits */
    let mut sym: u32 = 0; /* index of code symbols */
    let mut min: u32 = 0; /* minimum and maximum code lengths */
    let mut max: u32 = 0; /* number of index bits for root table */
    let mut root: u32 = 0; /* number of index bits for current table */
    let mut curr: u32 = 0; /* code bits to drop for sub-table */
    let mut drop_0: u32 = 0; /* number of prefix codes available */
    let mut left: i32 = 0; /* code entries in table used */
    let mut used: u32 = 0; /* Huffman code */
    let mut huff: u32 = 0; /* for incrementing code, index */
    let mut incr: u32 = 0; /* index for replicating entries */
    let mut fill: u32 = 0; /* low bits for current root entry */
    let mut low: u32 = 0; /* mask for low root bits */
    let mut mask: u32 = 0; /* table entry for duplication */
    let mut this: crate::src::zlib::inftrees::code = crate::src::zlib::inftrees::code {
        op: 0,
        bits: 0,
        val: 0,
    }; /* next available space in table */
    let mut next: *mut crate::src::zlib::inftrees::code = std::ptr::null_mut(); /* base value table to use */
    let mut base: *const u16 = std::ptr::null(); /* extra bits table to use */
    let mut extra: *const u16 = std::ptr::null(); /* use base and extra for symbol > end */
    let mut end: i32 = 0; /* number of codes of each length */
    let mut count: [u16; 16] = [0; 16]; /* offsets in table for each length */
    let mut offs: [u16; 16] = [0; 16];
    static mut lbase: [u16; 31] = [
        3 as i32 as u16,
        4 as i32 as u16,
        5 as i32 as u16,
        6 as i32 as u16,
        7 as i32 as u16,
        8 as i32 as u16,
        9 as i32 as u16,
        10 as i32 as u16,
        11 as i32 as u16,
        13 as i32 as u16,
        15 as i32 as u16,
        17 as i32 as u16,
        19 as i32 as u16,
        23 as i32 as u16,
        27 as i32 as u16,
        31 as i32 as u16,
        35 as i32 as u16,
        43 as i32 as u16,
        51 as i32 as u16,
        59 as i32 as u16,
        67 as i32 as u16,
        83 as i32 as u16,
        99 as i32 as u16,
        115 as i32 as u16,
        131 as i32 as u16,
        163 as i32 as u16,
        195 as i32 as u16,
        227 as i32 as u16,
        258 as i32 as u16,
        0 as i32 as u16,
        0 as i32 as u16,
    ];
    static mut lext: [u16; 31] = [
        16 as i32 as u16,
        16 as i32 as u16,
        16 as i32 as u16,
        16 as i32 as u16,
        16 as i32 as u16,
        16 as i32 as u16,
        16 as i32 as u16,
        16 as i32 as u16,
        17 as i32 as u16,
        17 as i32 as u16,
        17 as i32 as u16,
        17 as i32 as u16,
        18 as i32 as u16,
        18 as i32 as u16,
        18 as i32 as u16,
        18 as i32 as u16,
        19 as i32 as u16,
        19 as i32 as u16,
        19 as i32 as u16,
        19 as i32 as u16,
        20 as i32 as u16,
        20 as i32 as u16,
        20 as i32 as u16,
        20 as i32 as u16,
        21 as i32 as u16,
        21 as i32 as u16,
        21 as i32 as u16,
        21 as i32 as u16,
        16 as i32 as u16,
        201 as i32 as u16,
        196 as i32 as u16,
    ];
    static mut dbase: [u16; 32] = [
        1 as i32 as u16,
        2 as i32 as u16,
        3 as i32 as u16,
        4 as i32 as u16,
        5 as i32 as u16,
        7 as i32 as u16,
        9 as i32 as u16,
        13 as i32 as u16,
        17 as i32 as u16,
        25 as i32 as u16,
        33 as i32 as u16,
        49 as i32 as u16,
        65 as i32 as u16,
        97 as i32 as u16,
        129 as i32 as u16,
        193 as i32 as u16,
        257 as i32 as u16,
        385 as i32 as u16,
        513 as i32 as u16,
        769 as i32 as u16,
        1025 as i32 as u16,
        1537 as i32 as u16,
        2049 as i32 as u16,
        3073 as i32 as u16,
        4097 as i32 as u16,
        6145 as i32 as u16,
        8193 as i32 as u16,
        12289 as i32 as u16,
        16385 as i32 as u16,
        24577 as i32 as u16,
        0 as i32 as u16,
        0 as i32 as u16,
    ];
    static mut dext: [u16; 32] = [
        16 as i32 as u16,
        16 as i32 as u16,
        16 as i32 as u16,
        16 as i32 as u16,
        17 as i32 as u16,
        17 as i32 as u16,
        18 as i32 as u16,
        18 as i32 as u16,
        19 as i32 as u16,
        19 as i32 as u16,
        20 as i32 as u16,
        20 as i32 as u16,
        21 as i32 as u16,
        21 as i32 as u16,
        22 as i32 as u16,
        22 as i32 as u16,
        23 as i32 as u16,
        23 as i32 as u16,
        24 as i32 as u16,
        24 as i32 as u16,
        25 as i32 as u16,
        25 as i32 as u16,
        26 as i32 as u16,
        26 as i32 as u16,
        27 as i32 as u16,
        27 as i32 as u16,
        28 as i32 as u16,
        28 as i32 as u16,
        29 as i32 as u16,
        29 as i32 as u16,
        64 as i32 as u16,
        64 as i32 as u16,
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
    len = 0 as i32 as u32;
    while len <= 15 as i32 as u32 {
        count[len as usize] = 0 as i32 as u16;
        len = len.wrapping_add(1)
    }
    sym = 0 as i32 as u32;
    while sym < codes {
        count[*lens.offset(sym as isize) as usize] =
            count[*lens.offset(sym as isize) as usize].wrapping_add(1);
        sym = sym.wrapping_add(1)
    }
    /* bound code lengths, force root to be within code lengths */
    root = *bits;
    max = 15 as i32 as u32;
    while max >= 1 as i32 as u32 {
        if count[max as usize] as i32 != 0 as i32 {
            break;
        }
        max = max.wrapping_sub(1)
    }
    if root > max {
        root = max
    }
    if max == 0 as i32 as u32 {
        /* no symbols to code at all */
        this.op = 64 as i32 as u8;
        this.bits = 1 as i32 as u8;
        this.val = 0 as i32 as u16; /* invalid code marker */
        /* no symbols, but wait for decoding to report error */
        let fresh0 = *table; /* make a table to force an error */
        *table = (*table).offset(1);
        *fresh0 = this;
        let fresh1 = *table;
        *table = (*table).offset(1);
        *fresh1 = this;
        *bits = 1 as i32 as u32;
        return 0 as i32;
    }
    min = 1 as i32 as u32;
    while min <= 15 as i32 as u32 {
        if count[min as usize] as i32 != 0 as i32 {
            break;
        }
        min = min.wrapping_add(1)
    }
    if root < min {
        root = min
    }
    /* check for an over-subscribed or incomplete set of lengths */
    left = 1 as i32;
    len = 1 as i32 as u32;
    while len <= 15 as i32 as u32 {
        left <<= 1 as i32;
        left -= count[len as usize] as i32;
        if left < 0 as i32 {
            return -(1 as i32);
        }
        len = len.wrapping_add(1)
        /* over-subscribed */
    } /* incomplete set */
    if left > 0 as i32
        && (type_0 as u32 == crate::src::zlib::inftrees::CODES as i32 as u32
            || max != 1 as i32 as u32)
    {
        return -(1 as i32);
    }
    /* generate offsets into symbol table for each length for sorting */
    offs[1 as i32 as usize] = 0 as i32 as u16;
    len = 1 as i32 as u32;
    while len < 15 as i32 as u32 {
        offs[len.wrapping_add(1 as i32 as u32) as usize] =
            (offs[len as usize] as i32 + count[len as usize] as i32) as u16;
        len = len.wrapping_add(1)
    }
    /* sort symbols by length, by symbol order within each length */
    sym = 0 as i32 as u32;
    while sym < codes {
        if *lens.offset(sym as isize) as i32 != 0 as i32 {
            let fresh2 = offs[*lens.offset(sym as isize) as usize];
            offs[*lens.offset(sym as isize) as usize] =
                offs[*lens.offset(sym as isize) as usize].wrapping_add(1);
            *work.offset(fresh2 as isize) = sym as u16
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
    match type_0 as u32 {
        0 => {
            extra = work; /* dummy value--not used */
            base = extra;
            end = 19 as i32
        }
        1 => {
            base = lbase.as_ptr();
            base = base.offset(-(257 as i32 as isize));
            extra = lext.as_ptr();
            extra = extra.offset(-(257 as i32 as isize));
            end = 256 as i32
        }
        _ => {
            /* DISTS */
            base = dbase.as_ptr();
            extra = dext.as_ptr();
            end = -(1 as i32)
        }
    }
    /* initialize state for loop */
    huff = 0 as i32 as u32; /* starting code */
    sym = 0 as i32 as u32; /* starting code symbol */
    len = min; /* starting code length */
    next = *table; /* current table to fill in */
    curr = root; /* current table index bits */
    drop_0 = 0 as i32 as u32; /* current bits to drop from code for index */
    low = -(1 as i32) as u32; /* trigger new sub-table when len > root */
    used = (1 as u32) << root; /* use root table entries */
    mask = used.wrapping_sub(1 as i32 as u32); /* mask for comparing low */
    /* check available table space */
    if type_0 as u32 == crate::src::zlib::inftrees::LENS as i32 as u32
        && used >= (2048 as i32 - 592 as i32) as u32
    {
        return 1 as i32;
    }
    loop
    /* process all codes and make table entries */
    /* create table entry */
    {
        this.bits = len.wrapping_sub(drop_0) as u8; /* end of block */
        if (*work.offset(sym as isize) as i32) < end {
            this.op = 0 as i32 as u8;
            this.val = *work.offset(sym as isize)
        } else if *work.offset(sym as isize) as i32 > end {
            this.op = *extra.offset(*work.offset(sym as isize) as isize) as u8;
            this.val = *base.offset(*work.offset(sym as isize) as isize)
        } else {
            this.op = (32 as i32 + 64 as i32) as u8;
            this.val = 0 as i32 as u16
        }
        /* replicate for those indices with low len bits equal to huff */
        incr = (1 as u32) << len.wrapping_sub(drop_0); /* save offset to next table */
        fill = (1 as u32) << curr;
        min = fill;
        loop {
            fill = fill.wrapping_sub(incr);
            *next.offset((huff >> drop_0).wrapping_add(fill) as isize) = this;
            if !(fill != 0 as i32 as u32) {
                break;
            }
        }
        /* backwards increment the len-bit code huff */
        incr = (1 as u32) << len.wrapping_sub(1 as i32 as u32);
        while huff & incr != 0 {
            incr >>= 1 as i32
        }
        if incr != 0 as i32 as u32 {
            huff &= incr.wrapping_sub(1 as i32 as u32);
            huff = huff.wrapping_add(incr)
        } else {
            huff = 0 as i32 as u32
        }
        /* go to next symbol, update count, len */
        sym = sym.wrapping_add(1);
        count[len as usize] = count[len as usize].wrapping_sub(1);
        if count[len as usize] as i32 == 0 as i32 {
            if len == max {
                break;
            }
            len = *lens.offset(*work.offset(sym as isize) as isize) as u32
        }
        /* create new sub-table if needed */
        if len > root && huff & mask != low {
            /* if first time, transition to sub-tables */
            if drop_0 == 0 as i32 as u32 {
                drop_0 = root
            }
            /* increment past last table */
            next = next.offset(min as isize); /* here min is 1 << curr */
            /* determine length of next table */
            curr = len.wrapping_sub(drop_0);
            left = (1 as i32) << curr;
            while curr.wrapping_add(drop_0) < max {
                left -= count[curr.wrapping_add(drop_0) as usize] as i32;
                if left <= 0 as i32 {
                    break;
                }
                curr = curr.wrapping_add(1);
                left <<= 1 as i32
            }
            /* check for enough space */
            used = used.wrapping_add((1 as u32) << curr);
            if type_0 as u32 == crate::src::zlib::inftrees::LENS as i32 as u32
                && used >= (2048 as i32 - 592 as i32) as u32
            {
                return 1 as i32;
            }
            /* point entry in root table to sub-table */
            low = huff & mask;
            (*(*table).offset(low as isize)).op = curr as u8;
            (*(*table).offset(low as isize)).bits = root as u8;
            (*(*table).offset(low as isize)).val = next.offset_from(*table) as isize as u16
        }
    }
    /*
      Fill in rest of table for incomplete codes.  This loop is similar to the
      loop above in incrementing huff for table indices.  It is assumed that
      len is equal to curr + drop, so there is no loop needed to increment
      through high index bits.  When the current sub-table is filled, the loop
      drops back to the root table to fill in any remaining entries there.
    */
    this.op = 64 as i32 as u8; /* invalid code marker */
    this.bits = len.wrapping_sub(drop_0) as u8;
    this.val = 0 as i32 as u16;
    while huff != 0 as i32 as u32 {
        /* when done with sub-table, drop back to root table */
        if drop_0 != 0 as i32 as u32 && huff & mask != low {
            drop_0 = 0 as i32 as u32;
            len = root;
            next = *table;
            this.bits = len as u8
        }
        /* put invalid code marker in table */
        *next.offset((huff >> drop_0) as isize) = this;
        /* backwards increment the len-bit code huff */
        incr = (1 as u32) << len.wrapping_sub(1 as i32 as u32);
        while huff & incr != 0 {
            incr >>= 1 as i32
        }
        if incr != 0 as i32 as u32 {
            huff &= incr.wrapping_sub(1 as i32 as u32);
            huff = huff.wrapping_add(incr)
        } else {
            huff = 0 as i32 as u32
        }
    }
    /* set return parameters */
    *table = (*table).offset(used as isize);
    *bits = root;
    return 0 as i32;
}
