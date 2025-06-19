use ::libc;

pub use crate::qcommon_h::huff_t;
pub use crate::qcommon_h::huffman_t;
pub use crate::qcommon_h::msg_t;
pub use crate::qcommon_h::node_t;
pub use crate::qcommon_h::nodetype;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;

/*
===========================================================================
Copyright (C) 1999-2005 Id Software, Inc.

This file is part of Quake III Arena source code.

Quake III Arena source code is free software; you can redistribute it
and/or modify it under the terms of the GNU General Public License as
published by the Free Software Foundation; either version 2 of the License,
or (at your option) any later version.

Quake III Arena source code is distributed in the hope that it will be
useful, but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Quake III Arena source code; if not, write to the Free Software
Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
===========================================================================
*/
/* This is based on the Adaptive Huffman algorithm described in Sayood's Data
 * Compression book.  The ranks are not actually stored, but implicitly defined
 * by the location of a node within a doubly-linked list */

static mut bloc: i32 = 0 as i32;
#[no_mangle]

pub unsafe extern "C" fn Huff_putBit(
    mut bit: i32,
    mut fout: *mut crate::src::qcommon::q_shared::byte,
    mut offset: *mut i32,
) {
    bloc = *offset;
    if bloc & 7 as i32 == 0 as i32 {
        *fout.offset((bloc >> 3 as i32) as isize) =
            0 as i32 as crate::src::qcommon::q_shared::byte
    }
    let ref mut fresh0 = *fout.offset((bloc >> 3 as i32) as isize);
    *fresh0 = (*fresh0 as i32 | bit << (bloc & 7 as i32))
        as crate::src::qcommon::q_shared::byte;
    bloc += 1;
    *offset = bloc;
}
#[no_mangle]

pub unsafe extern "C" fn Huff_getBloc() -> i32 {
    return bloc;
}
// don't use if you don't know what you're doing.
#[no_mangle]

pub unsafe extern "C" fn Huff_setBloc(mut _bloc: i32) {
    bloc = _bloc;
}
#[no_mangle]

pub unsafe extern "C" fn Huff_getBit(
    mut fin: *mut crate::src::qcommon::q_shared::byte,
    mut offset: *mut i32,
) -> i32 {
    let mut t: i32 = 0;
    bloc = *offset;
    t = *fin.offset((bloc >> 3 as i32) as isize) as i32
        >> (bloc & 7 as i32)
        & 0x1 as i32;
    bloc += 1;
    *offset = bloc;
    return t;
}
/* Add a bit to the output file (buffered) */

unsafe extern "C" fn add_bit(
    mut bit: libc::c_char,
    mut fout: *mut crate::src::qcommon::q_shared::byte,
) {
    if bloc & 7 as i32 == 0 as i32 {
        *fout.offset((bloc >> 3 as i32) as isize) =
            0 as i32 as crate::src::qcommon::q_shared::byte
    }
    let ref mut fresh1 = *fout.offset((bloc >> 3 as i32) as isize);
    *fresh1 = (*fresh1 as i32 | (bit as i32) << (bloc & 7 as i32))
        as crate::src::qcommon::q_shared::byte;
    bloc += 1;
}
/* Receive one bit from the input file (buffered) */

unsafe extern "C" fn get_bit(mut fin: *mut crate::src::qcommon::q_shared::byte) -> i32 {
    let mut t: i32 = 0;
    t = *fin.offset((bloc >> 3 as i32) as isize) as i32
        >> (bloc & 7 as i32)
        & 0x1 as i32;
    bloc += 1;
    return t;
}

unsafe extern "C" fn get_ppnode(
    mut huff: *mut crate::qcommon_h::huff_t,
) -> *mut *mut crate::qcommon_h::node_t {
    let mut tppnode: *mut *mut crate::qcommon_h::node_t = 0 as *mut *mut crate::qcommon_h::node_t;
    if (*huff).freelist.is_null() {
        let fresh2 = (*huff).blocPtrs;
        (*huff).blocPtrs = (*huff).blocPtrs + 1;
        return &mut *(*huff).nodePtrs.as_mut_ptr().offset(fresh2 as isize)
            as *mut *mut crate::qcommon_h::node_t;
    } else {
        tppnode = (*huff).freelist;
        (*huff).freelist = *tppnode as *mut *mut crate::qcommon_h::node_t;
        return tppnode;
    };
}

unsafe extern "C" fn free_ppnode(
    mut huff: *mut crate::qcommon_h::huff_t,
    mut ppnode: *mut *mut crate::qcommon_h::node_t,
) {
    *ppnode = (*huff).freelist as *mut crate::qcommon_h::node_t;
    (*huff).freelist = ppnode;
}
/* Swap the location of these two nodes in the tree */

unsafe extern "C" fn swap(
    mut huff: *mut crate::qcommon_h::huff_t,
    mut node1: *mut crate::qcommon_h::node_t,
    mut node2: *mut crate::qcommon_h::node_t,
) {
    let mut par1: *mut crate::qcommon_h::node_t = 0 as *mut crate::qcommon_h::node_t;
    let mut par2: *mut crate::qcommon_h::node_t = 0 as *mut crate::qcommon_h::node_t;
    par1 = (*node1).parent;
    par2 = (*node2).parent;
    if !par1.is_null() {
        if (*par1).left == node1 {
            (*par1).left = node2
        } else {
            (*par1).right = node2
        }
    } else {
        (*huff).tree = node2
    }
    if !par2.is_null() {
        if (*par2).left == node2 {
            (*par2).left = node1
        } else {
            (*par2).right = node1
        }
    } else {
        (*huff).tree = node1
    }
    (*node1).parent = par2;
    (*node2).parent = par1;
}
/* Swap these two nodes in the linked list (update ranks) */

unsafe extern "C" fn swaplist(
    mut node1: *mut crate::qcommon_h::node_t,
    mut node2: *mut crate::qcommon_h::node_t,
) {
    let mut par1: *mut crate::qcommon_h::node_t = 0 as *mut crate::qcommon_h::node_t;
    par1 = (*node1).next;
    (*node1).next = (*node2).next;
    (*node2).next = par1;
    par1 = (*node1).prev;
    (*node1).prev = (*node2).prev;
    (*node2).prev = par1;
    if (*node1).next == node1 {
        (*node1).next = node2
    }
    if (*node2).next == node2 {
        (*node2).next = node1
    }
    if !(*node1).next.is_null() {
        (*(*node1).next).prev = node1
    }
    if !(*node2).next.is_null() {
        (*(*node2).next).prev = node2
    }
    if !(*node1).prev.is_null() {
        (*(*node1).prev).next = node1
    }
    if !(*node2).prev.is_null() {
        (*(*node2).prev).next = node2
    };
}
/* Do the increments */

unsafe extern "C" fn increment(
    mut huff: *mut crate::qcommon_h::huff_t,
    mut node: *mut crate::qcommon_h::node_t,
) {
    let mut lnode: *mut crate::qcommon_h::node_t = 0 as *mut crate::qcommon_h::node_t;
    if node.is_null() {
        return;
    }
    if !(*node).next.is_null() && (*(*node).next).weight == (*node).weight {
        lnode = *(*node).head;
        if lnode != (*node).parent {
            swap(huff, lnode, node);
        }
        swaplist(lnode, node);
    }
    if !(*node).prev.is_null() && (*(*node).prev).weight == (*node).weight {
        *(*node).head = (*node).prev
    } else {
        *(*node).head = 0 as *mut crate::qcommon_h::nodetype;
        free_ppnode(huff, (*node).head);
    }
    (*node).weight += 1;
    if !(*node).next.is_null() && (*(*node).next).weight == (*node).weight {
        (*node).head = (*(*node).next).head
    } else {
        (*node).head = get_ppnode(huff);
        *(*node).head = node
    }
    if !(*node).parent.is_null() {
        increment(huff, (*node).parent);
        if (*node).prev == (*node).parent {
            swaplist(node, (*node).parent);
            if *(*node).head == node {
                *(*node).head = (*node).parent
            }
        }
    };
}
#[no_mangle]

pub unsafe extern "C" fn Huff_addRef(
    mut huff: *mut crate::qcommon_h::huff_t,
    mut ch: crate::src::qcommon::q_shared::byte,
) {
    let mut tnode: *mut crate::qcommon_h::node_t = 0 as *mut crate::qcommon_h::node_t;
    let mut tnode2: *mut crate::qcommon_h::node_t = 0 as *mut crate::qcommon_h::node_t;
    if (*huff).loc[ch as usize].is_null() {
        /* if this is the first transmission of this node */
        let fresh3 = (*huff).blocNode;
        (*huff).blocNode = (*huff).blocNode + 1;
        tnode = &mut *(*huff).nodeList.as_mut_ptr().offset(fresh3 as isize)
            as *mut crate::qcommon_h::node_t;
        let fresh4 = (*huff).blocNode;
        (*huff).blocNode = (*huff).blocNode + 1;
        tnode2 = &mut *(*huff).nodeList.as_mut_ptr().offset(fresh4 as isize)
            as *mut crate::qcommon_h::node_t;
        (*tnode2).symbol = 256 as i32 + 1 as i32;
        (*tnode2).weight = 1 as i32;
        (*tnode2).next = (*(*huff).lhead).next;
        if !(*(*huff).lhead).next.is_null() {
            (*(*(*huff).lhead).next).prev = tnode2;
            if (*(*(*huff).lhead).next).weight == 1 as i32 {
                (*tnode2).head = (*(*(*huff).lhead).next).head
            } else {
                (*tnode2).head = get_ppnode(huff);
                *(*tnode2).head = tnode2
            }
        } else {
            (*tnode2).head = get_ppnode(huff);
            *(*tnode2).head = tnode2
        }
        (*(*huff).lhead).next = tnode2;
        (*tnode2).prev = (*huff).lhead;
        (*tnode).symbol = ch as i32;
        (*tnode).weight = 1 as i32;
        (*tnode).next = (*(*huff).lhead).next;
        if !(*(*huff).lhead).next.is_null() {
            (*(*(*huff).lhead).next).prev = tnode;
            if (*(*(*huff).lhead).next).weight == 1 as i32 {
                (*tnode).head = (*(*(*huff).lhead).next).head
            } else {
                /* this should never happen */
                (*tnode).head = get_ppnode(huff);
                *(*tnode).head = tnode2
            }
        } else {
            /* this should never happen */
            (*tnode).head = get_ppnode(huff);
            *(*tnode).head = tnode
        }
        (*(*huff).lhead).next = tnode;
        (*tnode).prev = (*huff).lhead;
        (*tnode).right = 0 as *mut crate::qcommon_h::nodetype;
        (*tnode).left = (*tnode).right;
        if !(*(*huff).lhead).parent.is_null() {
            if (*(*(*huff).lhead).parent).left == (*huff).lhead {
                /* lhead is guaranteed to by the NYT */
                (*(*(*huff).lhead).parent).left = tnode2
            } else {
                (*(*(*huff).lhead).parent).right = tnode2
            }
        } else {
            (*huff).tree = tnode2
        }
        (*tnode2).right = tnode;
        (*tnode2).left = (*huff).lhead;
        (*tnode2).parent = (*(*huff).lhead).parent;
        (*tnode).parent = tnode2;
        (*(*huff).lhead).parent = (*tnode).parent;
        (*huff).loc[ch as usize] = tnode;
        increment(huff, (*tnode2).parent);
    } else {
        increment(huff, (*huff).loc[ch as usize]);
    };
}
/* Get a symbol */
#[no_mangle]

pub unsafe extern "C" fn Huff_Receive(
    mut node: *mut crate::qcommon_h::node_t,
    mut ch: *mut i32,
    mut fin: *mut crate::src::qcommon::q_shared::byte,
) -> i32 {
    while !node.is_null() && (*node).symbol == 256 as i32 + 1 as i32 {
        if get_bit(fin) != 0 {
            node = (*node).right
        } else {
            node = (*node).left
        }
    }
    if node.is_null() {
        return 0 as i32;
        //		Com_Error(ERR_DROP, "Illegal tree!");
    }
    *ch = (*node).symbol;
    return *ch;
}
/* Get a symbol */
#[no_mangle]

pub unsafe extern "C" fn Huff_offsetReceive(
    mut node: *mut crate::qcommon_h::node_t,
    mut ch: *mut i32,
    mut fin: *mut crate::src::qcommon::q_shared::byte,
    mut offset: *mut i32,
    mut maxoffset: i32,
) {
    bloc = *offset;
    while !node.is_null() && (*node).symbol == 256 as i32 + 1 as i32 {
        if bloc >= maxoffset {
            *ch = 0 as i32;
            *offset = maxoffset + 1 as i32;
            return;
        }
        if get_bit(fin) != 0 {
            node = (*node).right
        } else {
            node = (*node).left
        }
    }
    if node.is_null() {
        *ch = 0 as i32;
        return;
        //		Com_Error(ERR_DROP, "Illegal tree!");
    }
    *ch = (*node).symbol;
    *offset = bloc;
}
/* Send the prefix code for this node */

unsafe extern "C" fn send(
    mut node: *mut crate::qcommon_h::node_t,
    mut child: *mut crate::qcommon_h::node_t,
    mut fout: *mut crate::src::qcommon::q_shared::byte,
    mut maxoffset: i32,
) {
    if !(*node).parent.is_null() {
        send((*node).parent, node, fout, maxoffset);
    }
    if !child.is_null() {
        if bloc >= maxoffset {
            bloc = maxoffset + 1 as i32;
            return;
        }
        if (*node).right == child {
            add_bit(1 as i32 as libc::c_char, fout);
        } else {
            add_bit(0 as i32 as libc::c_char, fout);
        }
    };
}
/* Send a symbol */
#[no_mangle]

pub unsafe extern "C" fn Huff_transmit(
    mut huff: *mut crate::qcommon_h::huff_t,
    mut ch: i32,
    mut fout: *mut crate::src::qcommon::q_shared::byte,
    mut maxoffset: i32,
) {
    let mut i: i32 = 0;
    if (*huff).loc[ch as usize].is_null() {
        /* node_t hasn't been transmitted, send a NYT, then the symbol */
        Huff_transmit(huff, 256 as i32, fout, maxoffset);
        i = 7 as i32;
        while i >= 0 as i32 {
            add_bit((ch >> i & 0x1 as i32) as libc::c_char, fout);
            i -= 1
        }
    } else {
        send(
            (*huff).loc[ch as usize],
            0 as *mut crate::qcommon_h::node_t,
            fout,
            maxoffset,
        );
    };
}
#[no_mangle]

pub unsafe extern "C" fn Huff_offsetTransmit(
    mut huff: *mut crate::qcommon_h::huff_t,
    mut ch: i32,
    mut fout: *mut crate::src::qcommon::q_shared::byte,
    mut offset: *mut i32,
    mut maxoffset: i32,
) {
    bloc = *offset;
    send(
        (*huff).loc[ch as usize],
        0 as *mut crate::qcommon_h::node_t,
        fout,
        maxoffset,
    );
    *offset = bloc;
}
#[no_mangle]

pub unsafe extern "C" fn Huff_Decompress(
    mut mbuf: *mut crate::qcommon_h::msg_t,
    mut offset: i32,
) {
    let mut ch: i32 = 0;
    let mut cch: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut size: i32 = 0;
    let mut seq: [crate::src::qcommon::q_shared::byte; 65536] = [0; 65536];
    let mut buffer: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut huff: crate::qcommon_h::huff_t = crate::qcommon_h::huff_t {
        blocNode: 0,
        blocPtrs: 0,
        tree: 0 as *mut crate::qcommon_h::node_t,
        lhead: 0 as *mut crate::qcommon_h::node_t,
        ltail: 0 as *mut crate::qcommon_h::node_t,
        loc: [0 as *mut crate::qcommon_h::node_t; 257],
        freelist: 0 as *mut *mut crate::qcommon_h::node_t,
        nodeList: [crate::qcommon_h::node_t {
            left: 0 as *mut crate::qcommon_h::nodetype,
            right: 0 as *mut crate::qcommon_h::nodetype,
            parent: 0 as *mut crate::qcommon_h::nodetype,
            next: 0 as *mut crate::qcommon_h::nodetype,
            prev: 0 as *mut crate::qcommon_h::nodetype,
            head: 0 as *mut *mut crate::qcommon_h::nodetype,
            weight: 0,
            symbol: 0,
        }; 768],
        nodePtrs: [0 as *mut crate::qcommon_h::node_t; 768],
    };
    size = (*mbuf).cursize - offset;
    buffer = (*mbuf).data.offset(offset as isize);
    if size <= 0 as i32 {
        return;
    }
    crate::stdlib::memset(
        &mut huff as *mut crate::qcommon_h::huff_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::qcommon_h::huff_t>() as libc::c_ulong,
    );
    // Initialize the tree & list with the NYT node
    let fresh5 = huff.blocNode;
    huff.blocNode = huff.blocNode + 1;
    huff.loc[256 as i32 as usize] =
        &mut *huff.nodeList.as_mut_ptr().offset(fresh5 as isize) as *mut crate::qcommon_h::node_t;
    huff.ltail = huff.loc[256 as i32 as usize];
    huff.lhead = huff.ltail;
    huff.tree = huff.lhead;
    (*huff.tree).symbol = 256 as i32;
    (*huff.tree).weight = 0 as i32;
    (*huff.lhead).prev = 0 as *mut crate::qcommon_h::nodetype;
    (*huff.lhead).next = (*huff.lhead).prev;
    (*huff.tree).right = 0 as *mut crate::qcommon_h::nodetype;
    (*huff.tree).left = (*huff.tree).right;
    (*huff.tree).parent = (*huff.tree).left;
    cch = *buffer.offset(0 as i32 as isize) as i32 * 256 as i32
        + *buffer.offset(1 as i32 as isize) as i32;
    // don't overflow with bad messages
    if cch > (*mbuf).maxsize - offset {
        cch = (*mbuf).maxsize - offset
    }
    bloc = 16 as i32;
    j = 0 as i32;
    while j < cch {
        ch = 0 as i32;
        /* Increment node */
        if bloc >> 3 as i32 > size {
            seq[j as usize] = 0 as i32 as crate::src::qcommon::q_shared::byte;
            break;
        } else {
            // don't overflow reading from the messages
            // FIXME: would it be better to have an overflow check in get_bit ?
            Huff_Receive(huff.tree, &mut ch, buffer); /* Get a character */
            if ch == 256 as i32 {
                /* We got a NYT, get the symbol associated with it */
                ch = 0 as i32; /* Write symbol */
                i = 0 as i32;
                while i < 8 as i32 {
                    ch = (ch << 1 as i32) + get_bit(buffer);
                    i += 1
                }
            }
            seq[j as usize] = ch as crate::src::qcommon::q_shared::byte;
            Huff_addRef(&mut huff, ch as crate::src::qcommon::q_shared::byte);
            j += 1
        }
    }
    (*mbuf).cursize = cch + offset;
    crate::stdlib::memcpy(
        (*mbuf).data.offset(offset as isize) as *mut libc::c_void,
        seq.as_mut_ptr() as *const libc::c_void,
        cch as libc::c_ulong,
    );
}
#[no_mangle]

pub unsafe extern "C" fn Huff_Compress(
    mut mbuf: *mut crate::qcommon_h::msg_t,
    mut offset: i32,
) {
    let mut i: i32 = 0;
    let mut ch: i32 = 0;
    let mut size: i32 = 0;
    let mut seq: [crate::src::qcommon::q_shared::byte; 65536] = [0; 65536];
    let mut buffer: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut huff: crate::qcommon_h::huff_t = crate::qcommon_h::huff_t {
        blocNode: 0,
        blocPtrs: 0,
        tree: 0 as *mut crate::qcommon_h::node_t,
        lhead: 0 as *mut crate::qcommon_h::node_t,
        ltail: 0 as *mut crate::qcommon_h::node_t,
        loc: [0 as *mut crate::qcommon_h::node_t; 257],
        freelist: 0 as *mut *mut crate::qcommon_h::node_t,
        nodeList: [crate::qcommon_h::node_t {
            left: 0 as *mut crate::qcommon_h::nodetype,
            right: 0 as *mut crate::qcommon_h::nodetype,
            parent: 0 as *mut crate::qcommon_h::nodetype,
            next: 0 as *mut crate::qcommon_h::nodetype,
            prev: 0 as *mut crate::qcommon_h::nodetype,
            head: 0 as *mut *mut crate::qcommon_h::nodetype,
            weight: 0,
            symbol: 0,
        }; 768],
        nodePtrs: [0 as *mut crate::qcommon_h::node_t; 768],
    };
    size = (*mbuf).cursize - offset;
    buffer = (*mbuf).data.offset(offset as isize);
    if size <= 0 as i32 {
        return;
    }
    crate::stdlib::memset(
        &mut huff as *mut crate::qcommon_h::huff_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::qcommon_h::huff_t>() as libc::c_ulong,
    );
    // Add the NYT (not yet transmitted) node into the tree/list */
    let fresh6 = huff.blocNode;
    huff.blocNode = huff.blocNode + 1;
    huff.loc[256 as i32 as usize] =
        &mut *huff.nodeList.as_mut_ptr().offset(fresh6 as isize) as *mut crate::qcommon_h::node_t;
    huff.lhead = huff.loc[256 as i32 as usize];
    huff.tree = huff.lhead;
    (*huff.tree).symbol = 256 as i32;
    (*huff.tree).weight = 0 as i32;
    (*huff.lhead).prev = 0 as *mut crate::qcommon_h::nodetype;
    (*huff.lhead).next = (*huff.lhead).prev;
    (*huff.tree).right = 0 as *mut crate::qcommon_h::nodetype;
    (*huff.tree).left = (*huff.tree).right;
    (*huff.tree).parent = (*huff.tree).left;
    seq[0 as i32 as usize] =
        (size >> 8 as i32) as crate::src::qcommon::q_shared::byte;
    seq[1 as i32 as usize] =
        (size & 0xff as i32) as crate::src::qcommon::q_shared::byte;
    bloc = 16 as i32;
    i = 0 as i32;
    while i < size {
        ch = *buffer.offset(i as isize) as i32;
        /* Do update */
        Huff_transmit(&mut huff, ch, seq.as_mut_ptr(), size << 3 as i32); /* Transmit symbol */
        Huff_addRef(&mut huff, ch as crate::src::qcommon::q_shared::byte); // next byte
        i += 1
    }
    bloc += 8 as i32;
    (*mbuf).cursize = (bloc >> 3 as i32) + offset;
    crate::stdlib::memcpy(
        (*mbuf).data.offset(offset as isize) as *mut libc::c_void,
        seq.as_mut_ptr() as *const libc::c_void,
        (bloc >> 3 as i32) as libc::c_ulong,
    );
}
/* This is based on the Adaptive Huffman algorithm described in Sayood's Data
 * Compression book.  The ranks are not actually stored, but implicitly defined
 * by the location of a node within a doubly-linked list */
/* NYT = Not Yet Transmitted */
/* tree structure */
/* doubly-linked list */
/* highest ranked node in block */
/* Maximum symbol */
#[no_mangle]

pub unsafe extern "C" fn Huff_Init(mut huff: *mut crate::qcommon_h::huffman_t) {
    crate::stdlib::memset(
        &mut (*huff).compressor as *mut crate::qcommon_h::huff_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::qcommon_h::huff_t>() as libc::c_ulong,
    );
    crate::stdlib::memset(
        &mut (*huff).decompressor as *mut crate::qcommon_h::huff_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::qcommon_h::huff_t>() as libc::c_ulong,
    );
    // Initialize the tree & list with the NYT node
    let fresh7 = (*huff).decompressor.blocNode;
    (*huff).decompressor.blocNode = (*huff).decompressor.blocNode + 1;
    (*huff).decompressor.loc[256 as i32 as usize] = &mut *(*huff)
        .decompressor
        .nodeList
        .as_mut_ptr()
        .offset(fresh7 as isize)
        as *mut crate::qcommon_h::node_t;
    (*huff).decompressor.ltail = (*huff).decompressor.loc[256 as i32 as usize];
    (*huff).decompressor.lhead = (*huff).decompressor.ltail;
    (*huff).decompressor.tree = (*huff).decompressor.lhead;
    (*(*huff).decompressor.tree).symbol = 256 as i32;
    (*(*huff).decompressor.tree).weight = 0 as i32;
    (*(*huff).decompressor.lhead).prev = 0 as *mut crate::qcommon_h::nodetype;
    (*(*huff).decompressor.lhead).next = (*(*huff).decompressor.lhead).prev;
    (*(*huff).decompressor.tree).right = 0 as *mut crate::qcommon_h::nodetype;
    (*(*huff).decompressor.tree).left = (*(*huff).decompressor.tree).right;
    (*(*huff).decompressor.tree).parent = (*(*huff).decompressor.tree).left;
    // Add the NYT (not yet transmitted) node into the tree/list */
    let fresh8 = (*huff).compressor.blocNode;
    (*huff).compressor.blocNode = (*huff).compressor.blocNode + 1;
    (*huff).compressor.loc[256 as i32 as usize] = &mut *(*huff)
        .compressor
        .nodeList
        .as_mut_ptr()
        .offset(fresh8 as isize)
        as *mut crate::qcommon_h::node_t;
    (*huff).compressor.lhead = (*huff).compressor.loc[256 as i32 as usize];
    (*huff).compressor.tree = (*huff).compressor.lhead;
    (*(*huff).compressor.tree).symbol = 256 as i32;
    (*(*huff).compressor.tree).weight = 0 as i32;
    (*(*huff).compressor.lhead).prev = 0 as *mut crate::qcommon_h::nodetype;
    (*(*huff).compressor.lhead).next = (*(*huff).compressor.lhead).prev;
    (*(*huff).compressor.tree).right = 0 as *mut crate::qcommon_h::nodetype;
    (*(*huff).compressor.tree).left = (*(*huff).compressor.tree).right;
    (*(*huff).compressor.tree).parent = (*(*huff).compressor.tree).left;
}
