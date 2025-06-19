use ::libc;

pub use crate::stddef_h::size_t;
pub use crate::stdlib::__int64_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int64_t;
pub use crate::stdlib::uint32_t;

pub use crate::config_types_h::ogg_int64_t;
pub use crate::config_types_h::ogg_uint32_t;
pub use crate::ogg_h::ogg_iovec_t;
pub use crate::ogg_h::ogg_packet;
pub use crate::ogg_h::ogg_page;
pub use crate::ogg_h::ogg_stream_state;
pub use crate::ogg_h::ogg_sync_state;

/* *******************************************************************
*                                                                  *
* THIS FILE IS PART OF THE Ogg CONTAINER SOURCE CODE.              *
* USE, DISTRIBUTION AND REPRODUCTION OF THIS LIBRARY SOURCE IS     *
* GOVERNED BY A BSD-STYLE SOURCE LICENSE INCLUDED WITH THIS SOURCE *
* IN 'COPYING'. PLEASE READ THESE TERMS BEFORE DISTRIBUTING.       *
*                                                                  *
* THE OggVorbis SOURCE CODE IS (C) COPYRIGHT 1994-2010             *
* by the Xiph.Org Foundation http://www.xiph.org/                  *
*                                                                  *
********************************************************************

function: code raw packets into framed OggSquish stream and
          decode Ogg streams back into raw packets
last mod: $Id$

note: The CRC code is directly derived from public domain code by
Ross Williams (ross@guest.adelaide.edu.au).  See docs/framing.html
for details.

********************************************************************/
/* A complete description of Ogg framing exists in docs/framing.html */
#[no_mangle]

pub unsafe extern "C" fn ogg_page_version(mut og: *const crate::ogg_h::ogg_page) -> libc::c_int {
    return *(*og).header.offset(4 as libc::c_int as isize) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn ogg_page_continued(mut og: *const crate::ogg_h::ogg_page) -> libc::c_int {
    return *(*og).header.offset(5 as libc::c_int as isize) as libc::c_int & 0x1 as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn ogg_page_bos(mut og: *const crate::ogg_h::ogg_page) -> libc::c_int {
    return *(*og).header.offset(5 as libc::c_int as isize) as libc::c_int & 0x2 as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn ogg_page_eos(mut og: *const crate::ogg_h::ogg_page) -> libc::c_int {
    return *(*og).header.offset(5 as libc::c_int as isize) as libc::c_int & 0x4 as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn ogg_page_granulepos(
    mut og: *const crate::ogg_h::ogg_page,
) -> crate::config_types_h::ogg_int64_t {
    let mut page: *mut libc::c_uchar = (*og).header;
    let mut granulepos: crate::config_types_h::ogg_int64_t =
        (*page.offset(13 as libc::c_int as isize) as libc::c_int & 0xff as libc::c_int)
            as crate::config_types_h::ogg_int64_t;
    granulepos = granulepos << 8 as libc::c_int
        | (*page.offset(12 as libc::c_int as isize) as libc::c_int & 0xff as libc::c_int)
            as libc::c_long;
    granulepos = granulepos << 8 as libc::c_int
        | (*page.offset(11 as libc::c_int as isize) as libc::c_int & 0xff as libc::c_int)
            as libc::c_long;
    granulepos = granulepos << 8 as libc::c_int
        | (*page.offset(10 as libc::c_int as isize) as libc::c_int & 0xff as libc::c_int)
            as libc::c_long;
    granulepos = granulepos << 8 as libc::c_int
        | (*page.offset(9 as libc::c_int as isize) as libc::c_int & 0xff as libc::c_int)
            as libc::c_long;
    granulepos = granulepos << 8 as libc::c_int
        | (*page.offset(8 as libc::c_int as isize) as libc::c_int & 0xff as libc::c_int)
            as libc::c_long;
    granulepos = granulepos << 8 as libc::c_int
        | (*page.offset(7 as libc::c_int as isize) as libc::c_int & 0xff as libc::c_int)
            as libc::c_long;
    granulepos = granulepos << 8 as libc::c_int
        | (*page.offset(6 as libc::c_int as isize) as libc::c_int & 0xff as libc::c_int)
            as libc::c_long;
    return granulepos;
}
#[no_mangle]

pub unsafe extern "C" fn ogg_page_serialno(mut og: *const crate::ogg_h::ogg_page) -> libc::c_int {
    return *(*og).header.offset(14 as libc::c_int as isize) as libc::c_int
        | (*(*og).header.offset(15 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int
        | (*(*og).header.offset(16 as libc::c_int as isize) as libc::c_int) << 16 as libc::c_int
        | (*(*og).header.offset(17 as libc::c_int as isize) as libc::c_int) << 24 as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn ogg_page_pageno(mut og: *const crate::ogg_h::ogg_page) -> libc::c_long {
    return (*(*og).header.offset(18 as libc::c_int as isize) as libc::c_int
        | (*(*og).header.offset(19 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int
        | (*(*og).header.offset(20 as libc::c_int as isize) as libc::c_int) << 16 as libc::c_int
        | (*(*og).header.offset(21 as libc::c_int as isize) as libc::c_int) << 24 as libc::c_int)
        as libc::c_long;
}
/* returns the number of packets that are completed on this page (if
the leading packet is begun on a previous page, but ends on this
page, it's counted */
/* NOTE:
   If a page consists of a packet begun on a previous page, and a new
   packet begun (but not completed) on this page, the return will be:
     ogg_page_packets(page)   ==1,
     ogg_page_continued(page) !=0

   If a page happens to be a single packet that was begun on a
   previous page, and spans to the next page (in the case of a three or
   more page packet), the return will be:
     ogg_page_packets(page)   ==0,
     ogg_page_continued(page) !=0
*/
#[no_mangle]

pub unsafe extern "C" fn ogg_page_packets(mut og: *const crate::ogg_h::ogg_page) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = *(*og).header.offset(26 as libc::c_int as isize) as libc::c_int;
    let mut count: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        if (*(*og).header.offset((27 as libc::c_int + i) as isize) as libc::c_int)
            < 255 as libc::c_int
        {
            count += 1
        }
        i += 1
    }
    return count;
}

static mut crc_lookup: [crate::config_types_h::ogg_uint32_t; 256] = [
    0 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x4c11db7 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x9823b6e as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0xd4326d9 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x130476dc as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x17c56b6b as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x1a864db2 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x1e475005 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x2608edb8 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x22c9f00f as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x2f8ad6d6 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x2b4bcb61 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x350c9b64 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x31cd86d3 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x3c8ea00a as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x384fbdbd as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x4c11db70 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x48d0c6c7 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x4593e01e as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x4152fda9 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x5f15adac as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x5bd4b01b as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x569796c2 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x52568b75 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x6a1936c8 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x6ed82b7f as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x639b0da6 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x675a1011 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x791d4014 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x7ddc5da3 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x709f7b7a as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x745e66cd as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x9823b6e0 as libc::c_uint,
    0x9ce2ab57 as libc::c_uint,
    0x91a18d8e as libc::c_uint,
    0x95609039 as libc::c_uint,
    0x8b27c03c as libc::c_uint,
    0x8fe6dd8b as libc::c_uint,
    0x82a5fb52 as libc::c_uint,
    0x8664e6e5 as libc::c_uint,
    0xbe2b5b58 as libc::c_uint,
    0xbaea46ef as libc::c_uint,
    0xb7a96036 as libc::c_uint,
    0xb3687d81 as libc::c_uint,
    0xad2f2d84 as libc::c_uint,
    0xa9ee3033 as libc::c_uint,
    0xa4ad16ea as libc::c_uint,
    0xa06c0b5d as libc::c_uint,
    0xd4326d90 as libc::c_uint,
    0xd0f37027 as libc::c_uint,
    0xddb056fe as libc::c_uint,
    0xd9714b49 as libc::c_uint,
    0xc7361b4c as libc::c_uint,
    0xc3f706fb as libc::c_uint,
    0xceb42022 as libc::c_uint,
    0xca753d95 as libc::c_uint,
    0xf23a8028 as libc::c_uint,
    0xf6fb9d9f as libc::c_uint,
    0xfbb8bb46 as libc::c_uint,
    0xff79a6f1 as libc::c_uint,
    0xe13ef6f4 as libc::c_uint,
    0xe5ffeb43 as libc::c_uint,
    0xe8bccd9a as libc::c_uint,
    0xec7dd02d as libc::c_uint,
    0x34867077 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x30476dc0 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x3d044b19 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x39c556ae as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x278206ab as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x23431b1c as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x2e003dc5 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x2ac12072 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x128e9dcf as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x164f8078 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x1b0ca6a1 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x1fcdbb16 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x18aeb13 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x54bf6a4 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x808d07d as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0xcc9cdca as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x7897ab07 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x7c56b6b0 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x71159069 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x75d48dde as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x6b93dddb as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x6f52c06c as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x6211e6b5 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x66d0fb02 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x5e9f46bf as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x5a5e5b08 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x571d7dd1 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x53dc6066 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x4d9b3063 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x495a2dd4 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x44190b0d as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x40d816ba as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0xaca5c697 as libc::c_uint,
    0xa864db20 as libc::c_uint,
    0xa527fdf9 as libc::c_uint,
    0xa1e6e04e as libc::c_uint,
    0xbfa1b04b as libc::c_uint,
    0xbb60adfc as libc::c_uint,
    0xb6238b25 as libc::c_uint,
    0xb2e29692 as libc::c_uint,
    0x8aad2b2f as libc::c_uint,
    0x8e6c3698 as libc::c_uint,
    0x832f1041 as libc::c_uint,
    0x87ee0df6 as libc::c_uint,
    0x99a95df3 as libc::c_uint,
    0x9d684044 as libc::c_uint,
    0x902b669d as libc::c_uint,
    0x94ea7b2a as libc::c_uint,
    0xe0b41de7 as libc::c_uint,
    0xe4750050 as libc::c_uint,
    0xe9362689 as libc::c_uint,
    0xedf73b3e as libc::c_uint,
    0xf3b06b3b as libc::c_uint,
    0xf771768c as libc::c_uint,
    0xfa325055 as libc::c_uint,
    0xfef34de2 as libc::c_uint,
    0xc6bcf05f as libc::c_uint,
    0xc27dede8 as libc::c_uint,
    0xcf3ecb31 as libc::c_uint,
    0xcbffd686 as libc::c_uint,
    0xd5b88683 as libc::c_uint,
    0xd1799b34 as libc::c_uint,
    0xdc3abded as libc::c_uint,
    0xd8fba05a as libc::c_uint,
    0x690ce0ee as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x6dcdfd59 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x608edb80 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x644fc637 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x7a089632 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x7ec98b85 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x738aad5c as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x774bb0eb as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x4f040d56 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x4bc510e1 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x46863638 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x42472b8f as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x5c007b8a as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x58c1663d as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x558240e4 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x51435d53 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x251d3b9e as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x21dc2629 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x2c9f00f0 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x285e1d47 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x36194d42 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x32d850f5 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x3f9b762c as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x3b5a6b9b as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x315d626 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x7d4cb91 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0xa97ed48 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0xe56f0ff as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x1011a0fa as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x14d0bd4d as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x19939b94 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x1d528623 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0xf12f560e as libc::c_uint,
    0xf5ee4bb9 as libc::c_uint,
    0xf8ad6d60 as libc::c_uint,
    0xfc6c70d7 as libc::c_uint,
    0xe22b20d2 as libc::c_uint,
    0xe6ea3d65 as libc::c_uint,
    0xeba91bbc as libc::c_uint,
    0xef68060b as libc::c_uint,
    0xd727bbb6 as libc::c_uint,
    0xd3e6a601 as libc::c_uint,
    0xdea580d8 as libc::c_uint,
    0xda649d6f as libc::c_uint,
    0xc423cd6a as libc::c_uint,
    0xc0e2d0dd as libc::c_uint,
    0xcda1f604 as libc::c_uint,
    0xc960ebb3 as libc::c_uint,
    0xbd3e8d7e as libc::c_uint,
    0xb9ff90c9 as libc::c_uint,
    0xb4bcb610 as libc::c_uint,
    0xb07daba7 as libc::c_uint,
    0xae3afba2 as libc::c_uint,
    0xaafbe615 as libc::c_uint,
    0xa7b8c0cc as libc::c_uint,
    0xa379dd7b as libc::c_uint,
    0x9b3660c6 as libc::c_uint,
    0x9ff77d71 as libc::c_uint,
    0x92b45ba8 as libc::c_uint,
    0x9675461f as libc::c_uint,
    0x8832161a as libc::c_uint,
    0x8cf30bad as libc::c_uint,
    0x81b02d74 as libc::c_uint,
    0x857130c3 as libc::c_uint,
    0x5d8a9099 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x594b8d2e as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x5408abf7 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x50c9b640 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x4e8ee645 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x4a4ffbf2 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x470cdd2b as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x43cdc09c as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x7b827d21 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x7f436096 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x7200464f as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x76c15bf8 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x68860bfd as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x6c47164a as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x61043093 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x65c52d24 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x119b4be9 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x155a565e as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x18197087 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x1cd86d30 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x29f3d35 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x65e2082 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0xb1d065b as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0xfdc1bec as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x3793a651 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x3352bbe6 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x3e119d3f as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x3ad08088 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x2497d08d as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x2056cd3a as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x2d15ebe3 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0x29d4f654 as libc::c_int as crate::config_types_h::ogg_uint32_t,
    0xc5a92679 as libc::c_uint,
    0xc1683bce as libc::c_uint,
    0xcc2b1d17 as libc::c_uint,
    0xc8ea00a0 as libc::c_uint,
    0xd6ad50a5 as libc::c_uint,
    0xd26c4d12 as libc::c_uint,
    0xdf2f6bcb as libc::c_uint,
    0xdbee767c as libc::c_uint,
    0xe3a1cbc1 as libc::c_uint,
    0xe760d676 as libc::c_uint,
    0xea23f0af as libc::c_uint,
    0xeee2ed18 as libc::c_uint,
    0xf0a5bd1d as libc::c_uint,
    0xf464a0aa as libc::c_uint,
    0xf9278673 as libc::c_uint,
    0xfde69bc4 as libc::c_uint,
    0x89b8fd09 as libc::c_uint,
    0x8d79e0be as libc::c_uint,
    0x803ac667 as libc::c_uint,
    0x84fbdbd0 as libc::c_uint,
    0x9abc8bd5 as libc::c_uint,
    0x9e7d9662 as libc::c_uint,
    0x933eb0bb as libc::c_uint,
    0x97ffad0c as libc::c_uint,
    0xafb010b1 as libc::c_uint,
    0xab710d06 as libc::c_uint,
    0xa6322bdf as libc::c_uint,
    0xa2f33668 as libc::c_uint,
    0xbcb4666d as libc::c_uint,
    0xb8757bda as libc::c_uint,
    0xb5365d03 as libc::c_uint,
    0xb1f740b4 as libc::c_uint,
];
/* Ogg BITSTREAM PRIMITIVES: general ***************************/
/* init the encode/decode logical stream state */
#[no_mangle]

pub unsafe extern "C" fn ogg_stream_init(
    mut os: *mut crate::ogg_h::ogg_stream_state,
    mut serialno: libc::c_int,
) -> libc::c_int {
    if !os.is_null() {
        crate::stdlib::memset(
            os as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<crate::ogg_h::ogg_stream_state>() as libc::c_ulong,
        );
        (*os).body_storage = (16 as libc::c_int * 1024 as libc::c_int) as libc::c_long;
        (*os).lacing_storage = 1024 as libc::c_int as libc::c_long;
        (*os).body_data = crate::stdlib::malloc(
            ((*os).body_storage as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
        ) as *mut libc::c_uchar;
        (*os).lacing_vals = crate::stdlib::malloc(
            ((*os).lacing_storage as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        (*os).granule_vals =
            crate::stdlib::malloc(((*os).lacing_storage as libc::c_ulong).wrapping_mul(
                ::std::mem::size_of::<crate::config_types_h::ogg_int64_t>() as libc::c_ulong,
            )) as *mut crate::config_types_h::ogg_int64_t;
        if (*os).body_data.is_null() || (*os).lacing_vals.is_null() || (*os).granule_vals.is_null()
        {
            ogg_stream_clear(os);
            return -(1 as libc::c_int);
        }
        (*os).serialno = serialno as libc::c_long;
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
/* async/delayed error detection for the ogg_stream_state */
#[no_mangle]

pub unsafe extern "C" fn ogg_stream_check(
    mut os: *mut crate::ogg_h::ogg_stream_state,
) -> libc::c_int {
    if os.is_null() || (*os).body_data.is_null() {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
/* _clear does not free os, only the non-flat storage within */
#[no_mangle]

pub unsafe extern "C" fn ogg_stream_clear(
    mut os: *mut crate::ogg_h::ogg_stream_state,
) -> libc::c_int {
    if !os.is_null() {
        if !(*os).body_data.is_null() {
            ::libc::free((*os).body_data as *mut libc::c_void);
        }
        if !(*os).lacing_vals.is_null() {
            ::libc::free((*os).lacing_vals as *mut libc::c_void);
        }
        if !(*os).granule_vals.is_null() {
            ::libc::free((*os).granule_vals as *mut libc::c_void);
        }
        crate::stdlib::memset(
            os as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<crate::ogg_h::ogg_stream_state>() as libc::c_ulong,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn ogg_stream_destroy(
    mut os: *mut crate::ogg_h::ogg_stream_state,
) -> libc::c_int {
    if !os.is_null() {
        ogg_stream_clear(os);
        ::libc::free(os as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}
/* Helpers for ogg_stream_encode; this keeps the structure and
what's happening fairly clear */

unsafe extern "C" fn _os_body_expand(
    mut os: *mut crate::ogg_h::ogg_stream_state,
    mut needed: libc::c_long,
) -> libc::c_int {
    if (*os).body_storage - needed <= (*os).body_fill {
        let mut body_storage: libc::c_long = 0;
        let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
        if (*os).body_storage > 9223372036854775807 as libc::c_long - needed {
            ogg_stream_clear(os);
            return -(1 as libc::c_int);
        }
        body_storage = (*os).body_storage + needed;
        if body_storage < 9223372036854775807 as libc::c_long - 1024 as libc::c_int as libc::c_long
        {
            body_storage += 1024 as libc::c_int as libc::c_long
        }
        ret = crate::stdlib::realloc(
            (*os).body_data as *mut libc::c_void,
            (body_storage as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
        );
        if ret.is_null() {
            ogg_stream_clear(os);
            return -(1 as libc::c_int);
        }
        (*os).body_storage = body_storage;
        (*os).body_data = ret as *mut libc::c_uchar
    }
    return 0 as libc::c_int;
}

unsafe extern "C" fn _os_lacing_expand(
    mut os: *mut crate::ogg_h::ogg_stream_state,
    mut needed: libc::c_long,
) -> libc::c_int {
    if (*os).lacing_storage - needed <= (*os).lacing_fill {
        let mut lacing_storage: libc::c_long = 0;
        let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
        if (*os).lacing_storage > 9223372036854775807 as libc::c_long - needed {
            ogg_stream_clear(os);
            return -(1 as libc::c_int);
        }
        lacing_storage = (*os).lacing_storage + needed;
        if lacing_storage < 9223372036854775807 as libc::c_long - 32 as libc::c_int as libc::c_long
        {
            lacing_storage += 32 as libc::c_int as libc::c_long
        }
        ret = crate::stdlib::realloc(
            (*os).lacing_vals as *mut libc::c_void,
            (lacing_storage as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        if ret.is_null() {
            ogg_stream_clear(os);
            return -(1 as libc::c_int);
        }
        (*os).lacing_vals = ret as *mut libc::c_int;
        ret = crate::stdlib::realloc(
            (*os).granule_vals as *mut libc::c_void,
            (lacing_storage as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
                crate::config_types_h::ogg_int64_t,
            >() as libc::c_ulong),
        );
        if ret.is_null() {
            ogg_stream_clear(os);
            return -(1 as libc::c_int);
        }
        (*os).granule_vals = ret as *mut crate::config_types_h::ogg_int64_t;
        (*os).lacing_storage = lacing_storage
    }
    return 0 as libc::c_int;
}
/* checksum the page */
/* Direct table CRC; note that this will be faster in the future if we
perform the checksum simultaneously with other copies */
#[no_mangle]

pub unsafe extern "C" fn ogg_page_checksum_set(mut og: *mut crate::ogg_h::ogg_page) {
    if !og.is_null() {
        let mut crc_reg: crate::config_types_h::ogg_uint32_t =
            0 as libc::c_int as crate::config_types_h::ogg_uint32_t;
        let mut i: libc::c_int = 0;
        /* safety; needed for API behavior, but not framing code */
        *(*og).header.offset(22 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
        *(*og).header.offset(23 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
        *(*og).header.offset(24 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
        *(*og).header.offset(25 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
        i = 0 as libc::c_int;
        while (i as libc::c_long) < (*og).header_len {
            crc_reg = crc_reg << 8 as libc::c_int
                ^ crc_lookup[(crc_reg >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint
                    ^ *(*og).header.offset(i as isize) as libc::c_uint)
                    as usize];
            i += 1
        }
        i = 0 as libc::c_int;
        while (i as libc::c_long) < (*og).body_len {
            crc_reg = crc_reg << 8 as libc::c_int
                ^ crc_lookup[(crc_reg >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint
                    ^ *(*og).body.offset(i as isize) as libc::c_uint)
                    as usize];
            i += 1
        }
        *(*og).header.offset(22 as libc::c_int as isize) =
            (crc_reg & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
        *(*og).header.offset(23 as libc::c_int as isize) =
            (crc_reg >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
        *(*og).header.offset(24 as libc::c_int as isize) =
            (crc_reg >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
        *(*og).header.offset(25 as libc::c_int as isize) =
            (crc_reg >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar
    };
}
/* submit data to the internal buffer of the framing engine */
#[no_mangle]

pub unsafe extern "C" fn ogg_stream_iovecin(
    mut os: *mut crate::ogg_h::ogg_stream_state,
    mut iov: *mut crate::ogg_h::ogg_iovec_t,
    mut count: libc::c_int,
    mut e_o_s: libc::c_long,
    mut granulepos: crate::config_types_h::ogg_int64_t,
) -> libc::c_int {
    let mut bytes: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut lacing_vals: libc::c_long = 0;
    let mut i: libc::c_int = 0;
    if ogg_stream_check(os) != 0 {
        return -(1 as libc::c_int);
    }
    if iov.is_null() {
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < count {
        if (*iov.offset(i as isize)).iov_len > 9223372036854775807 as libc::c_long as libc::c_ulong
        {
            return -(1 as libc::c_int);
        }
        if bytes
            > 9223372036854775807 as libc::c_long
                - (*iov.offset(i as isize)).iov_len as libc::c_long
        {
            return -(1 as libc::c_int);
        }
        bytes += (*iov.offset(i as isize)).iov_len as libc::c_long;
        i += 1
    }
    lacing_vals = bytes / 255 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long;
    if (*os).body_returned != 0 {
        /* advance packet data according to the body_returned pointer. We
        had to keep it around to return a pointer into the buffer last
        call */
        (*os).body_fill -= (*os).body_returned;
        if (*os).body_fill != 0 {
            crate::stdlib::memmove(
                (*os).body_data as *mut libc::c_void,
                (*os).body_data.offset((*os).body_returned as isize) as *const libc::c_void,
                (*os).body_fill as libc::c_ulong,
            );
        }
        (*os).body_returned = 0 as libc::c_int as libc::c_long
    }
    /* make sure we have the buffer storage */
    if _os_body_expand(os, bytes) != 0 || _os_lacing_expand(os, lacing_vals) != 0 {
        return -(1 as libc::c_int);
    }
    /* Copy in the submitted packet.  Yes, the copy is a waste; this is
    the liability of overly clean abstraction for the time being.  It
    will actually be fairly easy to eliminate the extra copy in the
    future */
    i = 0 as libc::c_int;
    while i < count {
        crate::stdlib::memcpy(
            (*os).body_data.offset((*os).body_fill as isize) as *mut libc::c_void,
            (*iov.offset(i as isize)).iov_base,
            (*iov.offset(i as isize)).iov_len,
        );
        (*os).body_fill += (*iov.offset(i as isize)).iov_len as libc::c_int as libc::c_long;
        i += 1
    }
    /* Store lacing vals for this packet */
    i = 0 as libc::c_int;
    while (i as libc::c_long) < lacing_vals - 1 as libc::c_int as libc::c_long {
        *(*os)
            .lacing_vals
            .offset(((*os).lacing_fill + i as libc::c_long) as isize) = 255 as libc::c_int;
        *(*os)
            .granule_vals
            .offset(((*os).lacing_fill + i as libc::c_long) as isize) = (*os).granulepos;
        i += 1
    }
    *(*os)
        .lacing_vals
        .offset(((*os).lacing_fill + i as libc::c_long) as isize) =
        (bytes % 255 as libc::c_int as libc::c_long) as libc::c_int;
    let ref mut fresh0 = *(*os)
        .granule_vals
        .offset(((*os).lacing_fill + i as libc::c_long) as isize);
    *fresh0 = granulepos;
    (*os).granulepos = *fresh0;
    /* flag the first segment as the beginning of the packet */
    *(*os).lacing_vals.offset((*os).lacing_fill as isize) |= 0x100 as libc::c_int;
    (*os).lacing_fill += lacing_vals;
    /* for the sake of completeness */
    (*os).packetno += 1;
    if e_o_s != 0 {
        (*os).e_o_s = 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* Ogg BITSTREAM PRIMITIVES: encoding **************************/
#[no_mangle]

pub unsafe extern "C" fn ogg_stream_packetin(
    mut os: *mut crate::ogg_h::ogg_stream_state,
    mut op: *mut crate::ogg_h::ogg_packet,
) -> libc::c_int {
    let mut iov: crate::ogg_h::ogg_iovec_t = crate::ogg_h::ogg_iovec_t {
        iov_base: 0 as *mut libc::c_void,
        iov_len: 0,
    };
    iov.iov_base = (*op).packet as *mut libc::c_void;
    iov.iov_len = (*op).bytes as crate::stddef_h::size_t;
    return ogg_stream_iovecin(
        os,
        &mut iov,
        1 as libc::c_int,
        (*op).e_o_s,
        (*op).granulepos,
    );
}
/* Conditionally flush a page; force==0 will only flush nominal-size
pages, force==1 forces us to flush a page regardless of page size
so long as there's any data available at all. */

unsafe extern "C" fn ogg_stream_flush_i(
    mut os: *mut crate::ogg_h::ogg_stream_state,
    mut og: *mut crate::ogg_h::ogg_page,
    mut force: libc::c_int,
    mut nfill: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut vals: libc::c_int = 0 as libc::c_int;
    let mut maxvals: libc::c_int = if (*os).lacing_fill > 255 as libc::c_int as libc::c_long {
        255 as libc::c_int as libc::c_long
    } else {
        (*os).lacing_fill
    } as libc::c_int;
    let mut bytes: libc::c_int = 0 as libc::c_int;
    let mut acc: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut granule_pos: crate::config_types_h::ogg_int64_t =
        -(1 as libc::c_int) as crate::config_types_h::ogg_int64_t;
    if ogg_stream_check(os) != 0 {
        return 0 as libc::c_int;
    }
    if maxvals == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    /* construct a page */
    /* decide how many segments to include */
    /* If this is the initial header case, the first page must only include
    the initial header packet */
    if (*os).b_o_s == 0 as libc::c_int {
        /* 'initial header page' case */
        granule_pos = 0 as libc::c_int as crate::config_types_h::ogg_int64_t;
        vals = 0 as libc::c_int;
        while vals < maxvals {
            if (*(*os).lacing_vals.offset(vals as isize) & 0xff as libc::c_int) < 255 as libc::c_int
            {
                vals += 1;
                break;
            } else {
                vals += 1
            }
        }
    } else {
        /* The extra packets_done, packet_just_done logic here attempts to do two things:
        1) Don't unneccessarily span pages.
        2) Unless necessary, don't flush pages if there are less than four packets on
           them; this expands page size to reduce unneccessary overhead if incoming packets
           are large.
        These are not necessary behaviors, just 'always better than naive flushing'
        without requiring an application to explicitly request a specific optimized
        behavior. We'll want an explicit behavior setup pathway eventually as well. */
        let mut packets_done: libc::c_int = 0 as libc::c_int;
        let mut packet_just_done: libc::c_int = 0 as libc::c_int;
        vals = 0 as libc::c_int;
        while vals < maxvals {
            if acc > nfill as libc::c_long && packet_just_done >= 4 as libc::c_int {
                force = 1 as libc::c_int;
                break;
            } else {
                acc += (*(*os).lacing_vals.offset(vals as isize) & 0xff as libc::c_int)
                    as libc::c_long;
                if (*(*os).lacing_vals.offset(vals as isize) & 0xff as libc::c_int)
                    < 255 as libc::c_int
                {
                    granule_pos = *(*os).granule_vals.offset(vals as isize);
                    packets_done += 1;
                    packet_just_done = packets_done
                } else {
                    packet_just_done = 0 as libc::c_int
                }
                vals += 1
            }
        }
        if vals == 255 as libc::c_int {
            force = 1 as libc::c_int
        }
    }
    if force == 0 {
        return 0 as libc::c_int;
    }
    /* construct the header in temp storage */
    crate::stdlib::memcpy(
        (*os).header.as_mut_ptr() as *mut libc::c_void,
        b"OggS\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    /* stream structure version */
    (*os).header[4 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    /* continued packet flag? */
    (*os).header[5 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    if *(*os).lacing_vals.offset(0 as libc::c_int as isize) & 0x100 as libc::c_int
        == 0 as libc::c_int
    {
        (*os).header[5 as libc::c_int as usize] = ((*os).header[5 as libc::c_int as usize]
            as libc::c_int
            | 0x1 as libc::c_int) as libc::c_uchar
    }
    /* first page flag? */
    if (*os).b_o_s == 0 as libc::c_int {
        (*os).header[5 as libc::c_int as usize] = ((*os).header[5 as libc::c_int as usize]
            as libc::c_int
            | 0x2 as libc::c_int) as libc::c_uchar
    }
    /* last page flag? */
    if (*os).e_o_s != 0 && (*os).lacing_fill == vals as libc::c_long {
        (*os).header[5 as libc::c_int as usize] = ((*os).header[5 as libc::c_int as usize]
            as libc::c_int
            | 0x4 as libc::c_int) as libc::c_uchar
    }
    (*os).b_o_s = 1 as libc::c_int;
    /* 64 bits of PCM position */
    i = 6 as libc::c_int;
    while i < 14 as libc::c_int {
        (*os).header[i as usize] =
            (granule_pos & 0xff as libc::c_int as libc::c_long) as libc::c_uchar;
        granule_pos >>= 8 as libc::c_int;
        i += 1
    }
    /* 32 bits of stream serial number */
    let mut serialno: libc::c_long = (*os).serialno;
    i = 14 as libc::c_int;
    while i < 18 as libc::c_int {
        (*os).header[i as usize] =
            (serialno & 0xff as libc::c_int as libc::c_long) as libc::c_uchar;
        serialno >>= 8 as libc::c_int;
        i += 1
    }
    /* 32 bits of page counter (we have both counter and page header
    because this val can roll over) */
    if (*os).pageno == -(1 as libc::c_int) as libc::c_long {
        (*os).pageno = 0 as libc::c_int as libc::c_long
    } /* because someone called
      stream_reset; this would be a
      strange thing to do in an
      encode stream, but it has
      plausible uses */
    let fresh1 = (*os).pageno;
    (*os).pageno = (*os).pageno + 1;
    let mut pageno: libc::c_long = fresh1;
    i = 18 as libc::c_int;
    while i < 22 as libc::c_int {
        (*os).header[i as usize] = (pageno & 0xff as libc::c_int as libc::c_long) as libc::c_uchar;
        pageno >>= 8 as libc::c_int;
        i += 1
    }
    /* zero for computation; filled in later */
    (*os).header[22 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    (*os).header[23 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    (*os).header[24 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    (*os).header[25 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    /* segment table */
    (*os).header[26 as libc::c_int as usize] = (vals & 0xff as libc::c_int) as libc::c_uchar;
    i = 0 as libc::c_int;
    while i < vals {
        (*os).header[(i + 27 as libc::c_int) as usize] =
            (*(*os).lacing_vals.offset(i as isize) & 0xff as libc::c_int) as libc::c_uchar;
        bytes += (*os).header[(i + 27 as libc::c_int) as usize] as libc::c_int;
        i += 1
    }
    /* set pointers in the ogg_page struct */
    (*og).header = (*os).header.as_mut_ptr();
    (*os).header_fill = vals + 27 as libc::c_int;
    (*og).header_len = (*os).header_fill as libc::c_long;
    (*og).body = (*os).body_data.offset((*os).body_returned as isize);
    (*og).body_len = bytes as libc::c_long;
    /* advance the lacing data and set the body_returned pointer */
    (*os).lacing_fill -= vals as libc::c_long;
    crate::stdlib::memmove(
        (*os).lacing_vals as *mut libc::c_void,
        (*os).lacing_vals.offset(vals as isize) as *const libc::c_void,
        ((*os).lacing_fill as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    crate::stdlib::memmove(
        (*os).granule_vals as *mut libc::c_void,
        (*os).granule_vals.offset(vals as isize) as *const libc::c_void,
        ((*os).lacing_fill as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
            crate::config_types_h::ogg_int64_t,
        >() as libc::c_ulong),
    );
    (*os).body_returned += bytes as libc::c_long;
    /* calculate the checksum */
    ogg_page_checksum_set(og);
    /* done */
    return 1 as libc::c_int;
}
/* This will flush remaining packets into a page (returning nonzero),
even if there is not enough data to trigger a flush normally
(undersized page). If there are no packets or partial packets to
flush, ogg_stream_flush returns 0.  Note that ogg_stream_flush will
try to flush a normal sized page like ogg_stream_pageout; a call to
ogg_stream_flush does not guarantee that all packets have flushed.
Only a return value of 0 from ogg_stream_flush indicates all packet
data is flushed into pages.

since ogg_stream_flush will flush the last page in a stream even if
it's undersized, you almost certainly want to use ogg_stream_pageout
(and *not* ogg_stream_flush) unless you specifically need to flush
a page regardless of size in the middle of a stream. */
#[no_mangle]

pub unsafe extern "C" fn ogg_stream_flush(
    mut os: *mut crate::ogg_h::ogg_stream_state,
    mut og: *mut crate::ogg_h::ogg_page,
) -> libc::c_int {
    return ogg_stream_flush_i(os, og, 1 as libc::c_int, 4096 as libc::c_int);
}
/* Like the above, but an argument is provided to adjust the nominal
page size for applications which are smart enough to provide their
own delay based flushing */
#[no_mangle]

pub unsafe extern "C" fn ogg_stream_flush_fill(
    mut os: *mut crate::ogg_h::ogg_stream_state,
    mut og: *mut crate::ogg_h::ogg_page,
    mut nfill: libc::c_int,
) -> libc::c_int {
    return ogg_stream_flush_i(os, og, 1 as libc::c_int, nfill);
}
/* This constructs pages from buffered packet segments.  The pointers
returned are to static buffers; do not free. The returned buffers are
good only until the next call (using the same ogg_stream_state) */
#[no_mangle]

pub unsafe extern "C" fn ogg_stream_pageout(
    mut os: *mut crate::ogg_h::ogg_stream_state,
    mut og: *mut crate::ogg_h::ogg_page,
) -> libc::c_int {
    let mut force: libc::c_int = 0 as libc::c_int;
    if ogg_stream_check(os) != 0 {
        return 0 as libc::c_int;
    }
    if (*os).e_o_s != 0 && (*os).lacing_fill != 0 || (*os).lacing_fill != 0 && (*os).b_o_s == 0 {
        /* 'initial header page' case */
        force = 1 as libc::c_int
    }
    return ogg_stream_flush_i(os, og, force, 4096 as libc::c_int);
}
/* Like the above, but an argument is provided to adjust the nominal
page size for applications which are smart enough to provide their
own delay based flushing */
#[no_mangle]

pub unsafe extern "C" fn ogg_stream_pageout_fill(
    mut os: *mut crate::ogg_h::ogg_stream_state,
    mut og: *mut crate::ogg_h::ogg_page,
    mut nfill: libc::c_int,
) -> libc::c_int {
    let mut force: libc::c_int = 0 as libc::c_int;
    if ogg_stream_check(os) != 0 {
        return 0 as libc::c_int;
    }
    if (*os).e_o_s != 0 && (*os).lacing_fill != 0 || (*os).lacing_fill != 0 && (*os).b_o_s == 0 {
        /* 'initial header page' case */
        force = 1 as libc::c_int
    }
    return ogg_stream_flush_i(os, og, force, nfill);
}
#[no_mangle]

pub unsafe extern "C" fn ogg_stream_eos(
    mut os: *mut crate::ogg_h::ogg_stream_state,
) -> libc::c_int {
    if ogg_stream_check(os) != 0 {
        return 1 as libc::c_int;
    }
    return (*os).e_o_s;
}
/* Ogg BITSTREAM PRIMITIVES: decoding **************************/
/* DECODING PRIMITIVES: packet streaming layer **********************/
/* This has two layers to place more of the multi-serialno and paging
control in the application's hands.  First, we expose a data buffer
using ogg_sync_buffer().  The app either copies into the
buffer, or passes it directly to read(), etc.  We then call
ogg_sync_wrote() to tell how many bytes we just added.

Pages are returned (pointers into the buffer in ogg_sync_state)
by ogg_sync_pageout().  The page is then submitted to
ogg_stream_pagein() along with the appropriate
ogg_stream_state* (ie, matching serialno).  We then get raw
packets out calling ogg_stream_packetout() with a
ogg_stream_state. */
/* initialize the struct to a known state */
#[no_mangle]

pub unsafe extern "C" fn ogg_sync_init(mut oy: *mut crate::ogg_h::ogg_sync_state) -> libc::c_int {
    if !oy.is_null() {
        (*oy).storage = -(1 as libc::c_int); /* used as a readiness flag */
        crate::stdlib::memset(
            oy as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<crate::ogg_h::ogg_sync_state>() as libc::c_ulong,
        );
    }
    return 0 as libc::c_int;
}
/* clear non-flat storage within */
#[no_mangle]

pub unsafe extern "C" fn ogg_sync_clear(mut oy: *mut crate::ogg_h::ogg_sync_state) -> libc::c_int {
    if !oy.is_null() {
        if !(*oy).data.is_null() {
            ::libc::free((*oy).data as *mut libc::c_void);
        }
        crate::stdlib::memset(
            oy as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<crate::ogg_h::ogg_sync_state>() as libc::c_ulong,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn ogg_sync_destroy(
    mut oy: *mut crate::ogg_h::ogg_sync_state,
) -> libc::c_int {
    if !oy.is_null() {
        ogg_sync_clear(oy);
        ::libc::free(oy as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn ogg_sync_check(mut oy: *mut crate::ogg_h::ogg_sync_state) -> libc::c_int {
    if (*oy).storage < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn ogg_sync_buffer(
    mut oy: *mut crate::ogg_h::ogg_sync_state,
    mut size: libc::c_long,
) -> *mut libc::c_char {
    if ogg_sync_check(oy) != 0 {
        return 0 as *mut libc::c_char;
    }
    /* first, clear out any space that has been previously returned */
    if (*oy).returned != 0 {
        (*oy).fill -= (*oy).returned;
        if (*oy).fill > 0 as libc::c_int {
            crate::stdlib::memmove(
                (*oy).data as *mut libc::c_void,
                (*oy).data.offset((*oy).returned as isize) as *const libc::c_void,
                (*oy).fill as libc::c_ulong,
            );
        }
        (*oy).returned = 0 as libc::c_int
    }
    if size > ((*oy).storage - (*oy).fill) as libc::c_long {
        /* We need to extend the internal buffer */
        let mut newsize: libc::c_long =
            size + (*oy).fill as libc::c_long + 4096 as libc::c_int as libc::c_long; /* an extra page to be nice */
        let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
        if !(*oy).data.is_null() {
            ret = crate::stdlib::realloc((*oy).data as *mut libc::c_void, newsize as libc::c_ulong)
        } else {
            ret = crate::stdlib::malloc(newsize as libc::c_ulong)
        }
        if ret.is_null() {
            ogg_sync_clear(oy);
            return 0 as *mut libc::c_char;
        }
        (*oy).data = ret as *mut libc::c_uchar;
        (*oy).storage = newsize as libc::c_int
    }
    /* expose a segment at least as large as requested at the fill mark */
    return ((*oy).data as *mut libc::c_char).offset((*oy).fill as isize);
}
#[no_mangle]

pub unsafe extern "C" fn ogg_sync_wrote(
    mut oy: *mut crate::ogg_h::ogg_sync_state,
    mut bytes: libc::c_long,
) -> libc::c_int {
    if ogg_sync_check(oy) != 0 {
        return -(1 as libc::c_int);
    }
    if (*oy).fill as libc::c_long + bytes > (*oy).storage as libc::c_long {
        return -(1 as libc::c_int);
    }
    (*oy).fill = ((*oy).fill as libc::c_long + bytes) as libc::c_int;
    return 0 as libc::c_int;
}
/* sync the stream.  This is meant to be useful for finding page
   boundaries.

   return values for this:
  -n) skipped n bytes
   0) page not ready; more data (no bytes skipped)
   n) page synced at current location; page length n bytes

*/
#[no_mangle]

pub unsafe extern "C" fn ogg_sync_pageseek(
    mut oy: *mut crate::ogg_h::ogg_sync_state,
    mut og: *mut crate::ogg_h::ogg_page,
) -> libc::c_long {
    let mut current_block: u64; /* not enough for a header */
    let mut page: *mut libc::c_uchar = (*oy).data.offset((*oy).returned as isize);
    let mut next: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut bytes: libc::c_long = ((*oy).fill - (*oy).returned) as libc::c_long;
    if ogg_sync_check(oy) != 0 {
        return 0 as libc::c_int as libc::c_long;
    }
    if (*oy).headerbytes == 0 as libc::c_int {
        let mut headerbytes: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        if bytes < 27 as libc::c_int as libc::c_long {
            return 0 as libc::c_int as libc::c_long;
        }
        /* verify capture pattern */
        if crate::stdlib::memcmp(
            page as *const libc::c_void,
            b"OggS\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        ) != 0
        {
            current_block = 8719873228262180869; /* not enough for header + seg table */
        } else {
            headerbytes =
                *page.offset(26 as libc::c_int as isize) as libc::c_int + 27 as libc::c_int;
            if bytes < headerbytes as libc::c_long {
                return 0 as libc::c_int as libc::c_long;
            }
            /* count up body length in the segment table */
            i = 0 as libc::c_int;
            while i < *page.offset(26 as libc::c_int as isize) as libc::c_int {
                (*oy).bodybytes += *page.offset((27 as libc::c_int + i) as isize) as libc::c_int;
                i += 1
            }
            (*oy).headerbytes = headerbytes;
            current_block = 12349973810996921269;
        }
    } else {
        current_block = 12349973810996921269;
    }
    match current_block {
        12349973810996921269 => {
            if ((*oy).bodybytes + (*oy).headerbytes) as libc::c_long > bytes {
                return 0 as libc::c_int as libc::c_long;
            }
            /* The whole test page is buffered.  Verify the checksum */
            /* Grab the checksum bytes, set the header field to zero */
            let mut chksum: [libc::c_char; 4] = [0; 4];
            let mut log: crate::ogg_h::ogg_page = crate::ogg_h::ogg_page {
                header: 0 as *mut libc::c_uchar,
                header_len: 0,
                body: 0 as *mut libc::c_uchar,
                body_len: 0,
            };
            crate::stdlib::memcpy(
                chksum.as_mut_ptr() as *mut libc::c_void,
                page.offset(22 as libc::c_int as isize) as *const libc::c_void,
                4 as libc::c_int as libc::c_ulong,
            );
            crate::stdlib::memset(
                page.offset(22 as libc::c_int as isize) as *mut libc::c_void,
                0 as libc::c_int,
                4 as libc::c_int as libc::c_ulong,
            );
            /* set up a temp page struct and recompute the checksum */
            log.header = page;
            log.header_len = (*oy).headerbytes as libc::c_long;
            log.body = page.offset((*oy).headerbytes as isize);
            log.body_len = (*oy).bodybytes as libc::c_long;
            ogg_page_checksum_set(&mut log);
            /* Compare */
            if crate::stdlib::memcmp(
                chksum.as_mut_ptr() as *const libc::c_void,
                page.offset(22 as libc::c_int as isize) as *const libc::c_void,
                4 as libc::c_int as libc::c_ulong,
            ) != 0
            {
                /* D'oh.  Mismatch! Corrupt page (or miscapture and not a page
                at all) */
                /* replace the computed checksum with the one actually read in */
                crate::stdlib::memcpy(
                    page.offset(22 as libc::c_int as isize) as *mut libc::c_void,
                    chksum.as_mut_ptr() as *const libc::c_void,
                    4 as libc::c_int as libc::c_ulong,
                );
            } else {
                /* yes, have a whole page all ready to go */
                let mut page_0: *mut libc::c_uchar = (*oy).data.offset((*oy).returned as isize);
                let mut bytes_0: libc::c_long = 0;
                if !og.is_null() {
                    (*og).header = page_0;
                    (*og).header_len = (*oy).headerbytes as libc::c_long;
                    (*og).body = page_0.offset((*oy).headerbytes as isize);
                    (*og).body_len = (*oy).bodybytes as libc::c_long
                }
                (*oy).unsynced = 0 as libc::c_int;
                bytes_0 = ((*oy).headerbytes + (*oy).bodybytes) as libc::c_long;
                (*oy).returned = ((*oy).returned as libc::c_long + bytes_0) as libc::c_int;
                (*oy).headerbytes = 0 as libc::c_int;
                (*oy).bodybytes = 0 as libc::c_int;
                return bytes_0;
            }
        }
        _ => {}
    }
    /* Bad checksum. Lose sync */
    (*oy).headerbytes = 0 as libc::c_int;
    (*oy).bodybytes = 0 as libc::c_int;
    /* search for possible capture */
    next = crate::stdlib::memchr(
        page.offset(1 as libc::c_int as isize) as *const libc::c_void,
        'O' as i32,
        (bytes - 1 as libc::c_int as libc::c_long) as libc::c_ulong,
    ) as *mut libc::c_uchar;
    if next.is_null() {
        next = (*oy).data.offset((*oy).fill as isize)
    }
    (*oy).returned = next.offset_from((*oy).data) as libc::c_long as libc::c_int;
    return -(next.offset_from(page) as libc::c_long);
}
/* sync the stream and get a page.  Keep trying until we find a page.
Suppress 'sync errors' after reporting the first.

return values:
-1) recapture (hole in data)
 0) need more data
 1) page returned

Returns pointers into buffered data; invalidated by next call to
_stream, _clear, _init, or _buffer */
#[no_mangle]

pub unsafe extern "C" fn ogg_sync_pageout(
    mut oy: *mut crate::ogg_h::ogg_sync_state,
    mut og: *mut crate::ogg_h::ogg_page,
) -> libc::c_int {
    if ogg_sync_check(oy) != 0 {
        return 0 as libc::c_int;
    }
    loop
    /* all we need to do is verify a page at the head of the stream
    buffer.  If it doesn't verify, we look for the next potential
    frame */
    {
        let mut ret: libc::c_long = ogg_sync_pageseek(oy, og);
        if ret > 0 as libc::c_int as libc::c_long {
            /* loop. keep looking */
            /* have a page */
            return 1 as libc::c_int;
        }
        if ret == 0 as libc::c_int as libc::c_long {
            /* need more data */
            return 0 as libc::c_int;
        }
        /* head did not start a synced page... skipped some bytes */
        if (*oy).unsynced == 0 {
            (*oy).unsynced = 1 as libc::c_int;
            return -(1 as libc::c_int);
        }
    }
}
/* add the incoming page to the stream state; we decompose the page
into packet segments here as well. */
#[no_mangle]

pub unsafe extern "C" fn ogg_stream_pagein(
    mut os: *mut crate::ogg_h::ogg_stream_state,
    mut og: *mut crate::ogg_h::ogg_page,
) -> libc::c_int {
    let mut header: *mut libc::c_uchar = (*og).header;
    let mut body: *mut libc::c_uchar = (*og).body;
    let mut bodysize: libc::c_long = (*og).body_len;
    let mut segptr: libc::c_int = 0 as libc::c_int;
    let mut version: libc::c_int = ogg_page_version(og);
    let mut continued: libc::c_int = ogg_page_continued(og);
    let mut bos: libc::c_int = ogg_page_bos(og);
    let mut eos: libc::c_int = ogg_page_eos(og);
    let mut granulepos: crate::config_types_h::ogg_int64_t = ogg_page_granulepos(og);
    let mut serialno: libc::c_int = ogg_page_serialno(og);
    let mut pageno: libc::c_long = ogg_page_pageno(og);
    let mut segments: libc::c_int = *header.offset(26 as libc::c_int as isize) as libc::c_int;
    if ogg_stream_check(os) != 0 {
        return -(1 as libc::c_int);
    }
    /* clean up 'returned data' */
    let mut lr: libc::c_long = (*os).lacing_returned;
    let mut br: libc::c_long = (*os).body_returned;
    /* body data */
    if br != 0 {
        (*os).body_fill -= br;
        if (*os).body_fill != 0 {
            crate::stdlib::memmove(
                (*os).body_data as *mut libc::c_void,
                (*os).body_data.offset(br as isize) as *const libc::c_void,
                (*os).body_fill as libc::c_ulong,
            );
        }
        (*os).body_returned = 0 as libc::c_int as libc::c_long
    }
    if lr != 0 {
        /* segment table */
        if (*os).lacing_fill - lr != 0 {
            crate::stdlib::memmove(
                (*os).lacing_vals as *mut libc::c_void,
                (*os).lacing_vals.offset(lr as isize) as *const libc::c_void,
                (((*os).lacing_fill - lr) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
            );
            crate::stdlib::memmove(
                (*os).granule_vals as *mut libc::c_void,
                (*os).granule_vals.offset(lr as isize) as *const libc::c_void,
                (((*os).lacing_fill - lr) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<crate::config_types_h::ogg_int64_t>()
                        as libc::c_ulong),
            );
        }
        (*os).lacing_fill -= lr;
        (*os).lacing_packet -= lr;
        (*os).lacing_returned = 0 as libc::c_int as libc::c_long
    }
    /* check the serial number */
    if serialno as libc::c_long != (*os).serialno {
        return -(1 as libc::c_int);
    }
    if version > 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if _os_lacing_expand(os, (segments + 1 as libc::c_int) as libc::c_long) != 0 {
        return -(1 as libc::c_int);
    }
    /* are we in sequence? */
    if pageno != (*os).pageno {
        let mut i: libc::c_int = 0;
        /* unroll previous partial packet (if any) */
        i = (*os).lacing_packet as libc::c_int;
        while (i as libc::c_long) < (*os).lacing_fill {
            (*os).body_fill -=
                (*(*os).lacing_vals.offset(i as isize) & 0xff as libc::c_int) as libc::c_long;
            i += 1
        }
        (*os).lacing_fill = (*os).lacing_packet;
        /* make a note of dropped data in segment table */
        if (*os).pageno != -(1 as libc::c_int) as libc::c_long {
            let fresh2 = (*os).lacing_fill;
            (*os).lacing_fill = (*os).lacing_fill + 1;
            *(*os).lacing_vals.offset(fresh2 as isize) = 0x400 as libc::c_int;
            (*os).lacing_packet += 1
        }
    }
    /* are we a 'continued packet' page?  If so, we may need to skip
    some segments */
    if continued != 0 {
        if (*os).lacing_fill < 1 as libc::c_int as libc::c_long
            || (*(*os)
                .lacing_vals
                .offset(((*os).lacing_fill - 1 as libc::c_int as libc::c_long) as isize)
                & 0xff as libc::c_int)
                < 255 as libc::c_int
            || *(*os)
                .lacing_vals
                .offset(((*os).lacing_fill - 1 as libc::c_int as libc::c_long) as isize)
                == 0x400 as libc::c_int
        {
            bos = 0 as libc::c_int;
            while segptr < segments {
                let mut val: libc::c_int =
                    *header.offset((27 as libc::c_int + segptr) as isize) as libc::c_int;
                body = body.offset(val as isize);
                bodysize -= val as libc::c_long;
                if val < 255 as libc::c_int {
                    segptr += 1;
                    break;
                } else {
                    segptr += 1
                }
            }
        }
    }
    if bodysize != 0 {
        if _os_body_expand(os, bodysize) != 0 {
            return -(1 as libc::c_int);
        }
        crate::stdlib::memcpy(
            (*os).body_data.offset((*os).body_fill as isize) as *mut libc::c_void,
            body as *const libc::c_void,
            bodysize as libc::c_ulong,
        );
        (*os).body_fill += bodysize
    }
    let mut saved: libc::c_int = -(1 as libc::c_int);
    while segptr < segments {
        let mut val_0: libc::c_int =
            *header.offset((27 as libc::c_int + segptr) as isize) as libc::c_int;
        *(*os).lacing_vals.offset((*os).lacing_fill as isize) = val_0;
        *(*os).granule_vals.offset((*os).lacing_fill as isize) =
            -(1 as libc::c_int) as crate::config_types_h::ogg_int64_t;
        if bos != 0 {
            *(*os).lacing_vals.offset((*os).lacing_fill as isize) |= 0x100 as libc::c_int;
            bos = 0 as libc::c_int
        }
        if val_0 < 255 as libc::c_int {
            saved = (*os).lacing_fill as libc::c_int
        }
        (*os).lacing_fill += 1;
        segptr += 1;
        if val_0 < 255 as libc::c_int {
            (*os).lacing_packet = (*os).lacing_fill
        }
    }
    /* set the granulepos on the last granuleval of the last full packet */
    if saved != -(1 as libc::c_int) {
        *(*os).granule_vals.offset(saved as isize) = granulepos
    }
    if eos != 0 {
        (*os).e_o_s = 1 as libc::c_int;
        if (*os).lacing_fill > 0 as libc::c_int as libc::c_long {
            *(*os)
                .lacing_vals
                .offset(((*os).lacing_fill - 1 as libc::c_int as libc::c_long) as isize) |=
                0x200 as libc::c_int
        }
    }
    (*os).pageno = pageno + 1 as libc::c_int as libc::c_long;
    return 0 as libc::c_int;
}
/* clear things to an initial state.  Good to call, eg, before seeking */
#[no_mangle]

pub unsafe extern "C" fn ogg_sync_reset(mut oy: *mut crate::ogg_h::ogg_sync_state) -> libc::c_int {
    if ogg_sync_check(oy) != 0 {
        return -(1 as libc::c_int);
    }
    (*oy).fill = 0 as libc::c_int;
    (*oy).returned = 0 as libc::c_int;
    (*oy).unsynced = 0 as libc::c_int;
    (*oy).headerbytes = 0 as libc::c_int;
    (*oy).bodybytes = 0 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn ogg_stream_reset(
    mut os: *mut crate::ogg_h::ogg_stream_state,
) -> libc::c_int {
    if ogg_stream_check(os) != 0 {
        return -(1 as libc::c_int);
    }
    (*os).body_fill = 0 as libc::c_int as libc::c_long;
    (*os).body_returned = 0 as libc::c_int as libc::c_long;
    (*os).lacing_fill = 0 as libc::c_int as libc::c_long;
    (*os).lacing_packet = 0 as libc::c_int as libc::c_long;
    (*os).lacing_returned = 0 as libc::c_int as libc::c_long;
    (*os).header_fill = 0 as libc::c_int;
    (*os).e_o_s = 0 as libc::c_int;
    (*os).b_o_s = 0 as libc::c_int;
    (*os).pageno = -(1 as libc::c_int) as libc::c_long;
    (*os).packetno = 0 as libc::c_int as crate::config_types_h::ogg_int64_t;
    (*os).granulepos = 0 as libc::c_int as crate::config_types_h::ogg_int64_t;
    return 0 as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn ogg_stream_reset_serialno(
    mut os: *mut crate::ogg_h::ogg_stream_state,
    mut serialno: libc::c_int,
) -> libc::c_int {
    if ogg_stream_check(os) != 0 {
        return -(1 as libc::c_int);
    }
    ogg_stream_reset(os);
    (*os).serialno = serialno as libc::c_long;
    return 0 as libc::c_int;
}

unsafe extern "C" fn _packetout(
    mut os: *mut crate::ogg_h::ogg_stream_state,
    mut op: *mut crate::ogg_h::ogg_packet,
    mut adv: libc::c_int,
) -> libc::c_int {
    /* The last part of decode. We have the stream broken into packet
    segments.  Now we need to group them into packets (or return the
    out of sync markers) */
    let mut ptr: libc::c_int = (*os).lacing_returned as libc::c_int;
    if (*os).lacing_packet <= ptr as libc::c_long {
        return 0 as libc::c_int;
    }
    if *(*os).lacing_vals.offset(ptr as isize) & 0x400 as libc::c_int != 0 {
        /* we need to tell the codec there's a gap; it might need to
        handle previous packet dependencies. */
        (*os).lacing_returned += 1; /* just using peek as an inexpensive way
                                    to ask if there's a whole packet
                                    waiting */
        (*os).packetno += 1;
        return -(1 as libc::c_int);
    }
    if op.is_null() && adv == 0 {
        return 1 as libc::c_int;
    }
    /* Gather the whole packet. We'll have no holes or a partial packet */
    let mut size: libc::c_int = *(*os).lacing_vals.offset(ptr as isize) & 0xff as libc::c_int; /* last packet of the stream? */
    let mut bytes: libc::c_long = size as libc::c_long; /* first packet of the stream? */
    let mut eos: libc::c_int = *(*os).lacing_vals.offset(ptr as isize) & 0x200 as libc::c_int;
    let mut bos: libc::c_int = *(*os).lacing_vals.offset(ptr as isize) & 0x100 as libc::c_int;
    while size == 255 as libc::c_int {
        ptr += 1;
        let mut val: libc::c_int = *(*os).lacing_vals.offset(ptr as isize);
        size = val & 0xff as libc::c_int;
        if val & 0x200 as libc::c_int != 0 {
            eos = 0x200 as libc::c_int
        }
        bytes += size as libc::c_long
    }
    if !op.is_null() {
        (*op).e_o_s = eos as libc::c_long;
        (*op).b_o_s = bos as libc::c_long;
        (*op).packet = (*os).body_data.offset((*os).body_returned as isize);
        (*op).packetno = (*os).packetno;
        (*op).granulepos = *(*os).granule_vals.offset(ptr as isize);
        (*op).bytes = bytes
    }
    if adv != 0 {
        (*os).body_returned += bytes;
        (*os).lacing_returned = (ptr + 1 as libc::c_int) as libc::c_long;
        (*os).packetno += 1
    }
    return 1 as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn ogg_stream_packetout(
    mut os: *mut crate::ogg_h::ogg_stream_state,
    mut op: *mut crate::ogg_h::ogg_packet,
) -> libc::c_int {
    if ogg_stream_check(os) != 0 {
        return 0 as libc::c_int;
    }
    return _packetout(os, op, 1 as libc::c_int);
}
#[no_mangle]

pub unsafe extern "C" fn ogg_stream_packetpeek(
    mut os: *mut crate::ogg_h::ogg_stream_state,
    mut op: *mut crate::ogg_h::ogg_packet,
) -> libc::c_int {
    if ogg_stream_check(os) != 0 {
        return 0 as libc::c_int;
    }
    return _packetout(os, op, 0 as libc::c_int);
}
#[no_mangle]

pub unsafe extern "C" fn ogg_packet_clear(mut op: *mut crate::ogg_h::ogg_packet) {
    ::libc::free((*op).packet as *mut libc::c_void);
    crate::stdlib::memset(
        op as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::ogg_h::ogg_packet>() as libc::c_ulong,
    );
}
