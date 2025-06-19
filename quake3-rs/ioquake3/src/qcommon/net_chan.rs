use ::libc;

pub mod stdlib_h {
    #[inline]

    pub unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> i32 {
        return libc::strtol(
            __nptr,
            0 as *mut libc::c_void as *mut *mut libc::c_char,
            10 as i32,
        ) as i32;
    }
}

pub use crate::internal::__builtin_va_list;
pub use crate::internal::__va_list_tag;
pub use crate::stdarg_h::va_list;

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::qcommon_h::msg_t;
pub use crate::qcommon_h::netadr_t;
pub use crate::qcommon_h::netadrtype_t;
pub use crate::qcommon_h::netchan_t;
pub use crate::qcommon_h::netsrc_t;
pub use crate::qcommon_h::NA_BAD;
pub use crate::qcommon_h::NA_BOT;
pub use crate::qcommon_h::NA_BROADCAST;
pub use crate::qcommon_h::NA_IP;
pub use crate::qcommon_h::NA_IP6;
pub use crate::qcommon_h::NA_LOOPBACK;
pub use crate::qcommon_h::NA_MULTICAST6;
pub use crate::qcommon_h::NA_UNSPEC;
pub use crate::qcommon_h::NS_CLIENT;
pub use crate::qcommon_h::NS_SERVER;
pub use crate::src::qcommon::common::cl_packetdelay;
pub use crate::src::qcommon::common::com_timescale;
pub use crate::src::qcommon::common::sv_packetdelay;
pub use crate::src::qcommon::common::Com_Error;
pub use crate::src::qcommon::common::Com_Printf;
pub use crate::src::qcommon::common::S_Malloc;
pub use crate::src::qcommon::common::Z_Free;
pub use crate::src::qcommon::cvar::Cvar_Get;
pub use crate::src::qcommon::huffman::Huff_Compress;
pub use crate::src::qcommon::msg::MSG_BeginReadingOOB;
pub use crate::src::qcommon::msg::MSG_InitOOB;
pub use crate::src::qcommon::msg::MSG_ReadLong;
pub use crate::src::qcommon::msg::MSG_ReadShort;
pub use crate::src::qcommon::msg::MSG_WriteData;
pub use crate::src::qcommon::msg::MSG_WriteLong;
pub use crate::src::qcommon::msg::MSG_WriteShort;
pub use crate::src::qcommon::net_chan::stdlib_h::atoi;
pub use crate::src::qcommon::net_ip::NET_AdrToString;
pub use crate::src::qcommon::net_ip::Sys_SendPacket;
pub use crate::src::qcommon::net_ip::Sys_StringToAdr;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::Q_CountChar;
pub use crate::src::qcommon::q_shared::Q_strncpyz;

pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;
pub use crate::src::sys::sys_unix::Sys_Milliseconds;

pub use ::libc::strtol;
//=============================================================================

pub type packetQueue_t = packetQueue_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct packetQueue_s {
    pub next: *mut packetQueue_s,
    pub length: i32,
    pub data: *mut byte,
    pub to: netadr_t,
    pub release: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct loopmsg_t {
    pub data: [byte; 1400],
    pub datalen: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct loopback_t {
    pub msgs: [loopmsg_t; 16],
    pub get: i32,
    pub send: i32,
}
#[no_mangle]

pub static mut showpackets: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]

pub static mut showdrop: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]

pub static mut qport: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;

static mut netsrcString: [*mut libc::c_char; 2] = [
    b"client\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"server\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
/*
===============
Netchan_Init

===============
*/
#[no_mangle]

pub unsafe extern "C" fn Netchan_Init(mut port: i32) {
    port &= 0xffff as i32;
    showpackets = Cvar_Get(
        b"showpackets\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x100 as i32,
    ) as *mut cvar_s;
    showdrop = Cvar_Get(
        b"showdrop\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x100 as i32,
    ) as *mut cvar_s;
    qport = Cvar_Get(
        b"net_qport\x00" as *const u8 as *const libc::c_char,
        va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            port,
        ),
        0x10 as i32,
    ) as *mut cvar_s;
}
/*
==============
Netchan_Setup

called to open a channel to a remote system
==============
*/
#[no_mangle]

pub unsafe extern "C" fn Netchan_Setup(
    mut sock: netsrc_t,
    mut chan: *mut netchan_t,
    mut adr: netadr_t,
    mut qport_0: i32,
    mut challenge: i32,
    mut compat: qboolean,
) {
    crate::stdlib::memset(
        chan as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<netchan_t>() as usize,
    );
    (*chan).sock = sock;
    (*chan).remoteAddress = adr;
    (*chan).qport = qport_0;
    (*chan).incomingSequence = 0 as i32;
    (*chan).outgoingSequence = 1 as i32;
    (*chan).challenge = challenge;
    (*chan).compat = compat;
}
/*
=================
Netchan_TransmitNextFragment

Send one fragment of the current message
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Netchan_TransmitNextFragment(mut chan: *mut netchan_t) {
    let mut send: msg_t = msg_t {
        allowoverflow: qfalse,
        overflowed: qfalse,
        oob: qfalse,
        data: 0 as *mut byte,
        maxsize: 0,
        cursize: 0,
        readcount: 0,
        bit: 0,
    };
    let mut send_buf: [byte; 1400] = [0; 1400];
    let mut fragmentLength: i32 = 0;
    let mut outgoingSequence: i32 = 0;
    // write the packet header
    MSG_InitOOB(
        &mut send as *mut _ as *mut msg_t,
        send_buf.as_mut_ptr(),
        ::std::mem::size_of::<[byte; 1400]>() as usize as i32,
    ); // <-- only do the oob here
    outgoingSequence = ((*chan).outgoingSequence as u32 | (1 as u32) << 31 as i32) as i32;
    MSG_WriteLong(&mut send as *mut _ as *mut msg_t, outgoingSequence);
    // send the qport if we are a client
    if (*chan).sock as u32 == NS_CLIENT as i32 as u32 {
        MSG_WriteShort(&mut send as *mut _ as *mut msg_t, (*qport).integer);
    }
    if (*chan).compat as u64 == 0 {
        MSG_WriteLong(
            &mut send as *mut _ as *mut msg_t,
            (*chan).challenge ^ (*chan).outgoingSequence * (*chan).challenge,
        );
    }
    // copy the reliable message to the packet first
    fragmentLength = 1400 as i32 - 100 as i32;
    if (*chan).unsentFragmentStart + fragmentLength > (*chan).unsentLength {
        fragmentLength = (*chan).unsentLength - (*chan).unsentFragmentStart
    }
    MSG_WriteShort(
        &mut send as *mut _ as *mut msg_t,
        (*chan).unsentFragmentStart,
    );
    MSG_WriteShort(&mut send as *mut _ as *mut msg_t, fragmentLength);
    MSG_WriteData(
        &mut send as *mut _ as *mut msg_t,
        (*chan)
            .unsentBuffer
            .as_mut_ptr()
            .offset((*chan).unsentFragmentStart as isize) as *const libc::c_void,
        fragmentLength,
    );
    // send the datagram
    NET_SendPacket(
        (*chan).sock,
        send.cursize,
        send.data as *const libc::c_void,
        (*chan).remoteAddress,
    );
    // Store send time and size of this packet for rate control
    (*chan).lastSentTime = Sys_Milliseconds();
    (*chan).lastSentSize = send.cursize;
    if (*showpackets).integer != 0 {
        Com_Printf(
            b"%s send %4i : s=%i fragment=%i,%i\n\x00" as *const u8 as *const libc::c_char,
            netsrcString[(*chan).sock as usize],
            send.cursize,
            (*chan).outgoingSequence,
            (*chan).unsentFragmentStart,
            fragmentLength,
        );
    }
    (*chan).unsentFragmentStart += fragmentLength;
    // this exit condition is a little tricky, because a packet
    // that is exactly the fragment length still needs to send
    // a second packet of zero length so that the other side
    // can tell there aren't more to follow
    if (*chan).unsentFragmentStart == (*chan).unsentLength
        && fragmentLength != 1400 as i32 - 100 as i32
    {
        (*chan).outgoingSequence += 1;
        (*chan).unsentFragments = qfalse
    };
}
/*
===============
Netchan_Transmit

Sends a message to a connection, fragmenting if necessary
A 0 length will still generate a packet.
================
*/
#[no_mangle]

pub unsafe extern "C" fn Netchan_Transmit(
    mut chan: *mut netchan_t,
    mut length: i32,
    mut data: *const byte,
) {
    let mut send: msg_t = msg_t {
        allowoverflow: qfalse,
        overflowed: qfalse,
        oob: qfalse,
        data: 0 as *mut byte,
        maxsize: 0,
        cursize: 0,
        readcount: 0,
        bit: 0,
    };
    let mut send_buf: [byte; 1400] = [0; 1400];
    if length > 16384 as i32 {
        Com_Error(
            ERR_DROP as i32,
            b"Netchan_Transmit: length = %i\x00" as *const u8 as *const libc::c_char,
            length,
        );
    }
    (*chan).unsentFragmentStart = 0 as i32;
    // fragment large reliable messages
    if length >= 1400 as i32 - 100 as i32 {
        (*chan).unsentFragments = qtrue;
        (*chan).unsentLength = length;
        crate::stdlib::memcpy(
            (*chan).unsentBuffer.as_mut_ptr() as *mut libc::c_void,
            data as *const libc::c_void,
            length as usize,
        );
        // only send the first fragment now
        Netchan_TransmitNextFragment(chan);
        return;
    }
    // write the packet header
    MSG_InitOOB(
        &mut send as *mut _ as *mut msg_t,
        send_buf.as_mut_ptr(),
        ::std::mem::size_of::<[byte; 1400]>() as usize as i32,
    );
    MSG_WriteLong(&mut send as *mut _ as *mut msg_t, (*chan).outgoingSequence);
    // send the qport if we are a client
    if (*chan).sock as u32 == NS_CLIENT as i32 as u32 {
        MSG_WriteShort(&mut send as *mut _ as *mut msg_t, (*qport).integer);
    }
    if (*chan).compat as u64 == 0 {
        MSG_WriteLong(
            &mut send as *mut _ as *mut msg_t,
            (*chan).challenge ^ (*chan).outgoingSequence * (*chan).challenge,
        );
    }
    (*chan).outgoingSequence += 1;
    MSG_WriteData(
        &mut send as *mut _ as *mut msg_t,
        data as *const libc::c_void,
        length,
    );
    // send the datagram
    NET_SendPacket(
        (*chan).sock,
        send.cursize,
        send.data as *const libc::c_void,
        (*chan).remoteAddress,
    );
    // Store send time and size of this packet for rate control
    (*chan).lastSentTime = Sys_Milliseconds();
    (*chan).lastSentSize = send.cursize;
    if (*showpackets).integer != 0 {
        Com_Printf(
            b"%s send %4i : s=%i ack=%i\n\x00" as *const u8 as *const libc::c_char,
            netsrcString[(*chan).sock as usize],
            send.cursize,
            (*chan).outgoingSequence - 1 as i32,
            (*chan).incomingSequence,
        );
    };
}
/*
=================
Netchan_Process

Returns qfalse if the message should not be processed due to being
out of order or a fragment.

Msg must be large enough to hold MAX_MSGLEN, because if this is the
final fragment of a multi-part message, the entire thing will be
copied out.
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Netchan_Process(
    mut chan: *mut netchan_t,
    mut msg: *mut msg_t,
) -> qboolean {
    let mut sequence: i32 = 0;
    let mut fragmentStart: i32 = 0;
    let mut fragmentLength: i32 = 0;
    let mut fragmented: qboolean = qfalse;
    // XOR unscramble all data in the packet after the header
    //	Netchan_UnScramblePacket( msg );
    // get sequence numbers
    MSG_BeginReadingOOB(msg as *mut msg_t);
    sequence = MSG_ReadLong(msg as *mut msg_t);
    // check for fragment information
    if sequence as u32 & (1 as u32) << 31 as i32 != 0 {
        sequence = (sequence as u32 & !((1 as u32) << 31 as i32)) as i32;
        fragmented = qtrue
    } else {
        fragmented = qfalse
    }
    // read the qport if we are a server
    if (*chan).sock as u32 == NS_SERVER as i32 as u32 {
        MSG_ReadShort(msg as *mut msg_t);
    }
    if (*chan).compat as u64 == 0 {
        let mut checksum: i32 = MSG_ReadLong(msg as *mut msg_t);
        // UDP spoofing protection
        if (*chan).challenge ^ sequence * (*chan).challenge != checksum {
            return qfalse;
        }
    }
    // read the fragment information
    if fragmented as u64 != 0 {
        fragmentStart = MSG_ReadShort(msg as *mut msg_t); // stop warning message
        fragmentLength = MSG_ReadShort(msg as *mut msg_t)
    } else {
        fragmentStart = 0 as i32;
        fragmentLength = 0 as i32
    }
    if (*showpackets).integer != 0 {
        if fragmented as u64 != 0 {
            Com_Printf(
                b"%s recv %4i : s=%i fragment=%i,%i\n\x00" as *const u8 as *const libc::c_char,
                netsrcString[(*chan).sock as usize],
                (*msg).cursize,
                sequence,
                fragmentStart,
                fragmentLength,
            );
        } else {
            Com_Printf(
                b"%s recv %4i : s=%i\n\x00" as *const u8 as *const libc::c_char,
                netsrcString[(*chan).sock as usize],
                (*msg).cursize,
                sequence,
            );
        }
    }
    //
    // discard out of order or duplicated packets
    //
    if sequence <= (*chan).incomingSequence {
        if (*showdrop).integer != 0 || (*showpackets).integer != 0 {
            Com_Printf(
                b"%s:Out of order packet %i at %i\n\x00" as *const u8 as *const libc::c_char,
                NET_AdrToString((*chan).remoteAddress as netadr_t),
                sequence,
                (*chan).incomingSequence,
            );
        }
        return qfalse;
    }
    //
    // dropped packets don't keep the message from being used
    //
    (*chan).dropped = sequence - ((*chan).incomingSequence + 1 as i32);
    if (*chan).dropped > 0 as i32 {
        if (*showdrop).integer != 0 || (*showpackets).integer != 0 {
            Com_Printf(
                b"%s:Dropped %i packets at %i\n\x00" as *const u8 as *const libc::c_char,
                NET_AdrToString((*chan).remoteAddress as netadr_t),
                (*chan).dropped,
                sequence,
            );
        }
    }
    //
    // if this is the final framgent of a reliable message,
    // bump incoming_reliable_sequence
    //
    if fragmented as u64 != 0 {
        // TTimo
        // make sure we add the fragments in correct order
        // either a packet was dropped, or we received this one too soon
        // we don't reconstruct the fragments. we will wait till this fragment gets to us again
        // (NOTE: we could probably try to rebuild by out of order chunks if needed)
        if sequence != (*chan).fragmentSequence {
            (*chan).fragmentSequence = sequence;
            (*chan).fragmentLength = 0 as i32
        }
        // if we missed a fragment, dump the message
        if fragmentStart != (*chan).fragmentLength {
            if (*showdrop).integer != 0 || (*showpackets).integer != 0 {
                Com_Printf(
                    b"%s:Dropped a message fragment\n\x00" as *const u8 as *const libc::c_char,
                    NET_AdrToString((*chan).remoteAddress as netadr_t),
                );
            }
            // we can still keep the part that we have so far,
            // so we don't need to clear chan->fragmentLength
            return qfalse;
        }
        // copy the fragment to the fragment buffer
        if fragmentLength < 0 as i32
            || (*msg).readcount + fragmentLength > (*msg).cursize
            || ((*chan).fragmentLength + fragmentLength) as usize
                > ::std::mem::size_of::<[byte; 16384]>() as usize
        {
            if (*showdrop).integer != 0 || (*showpackets).integer != 0 {
                Com_Printf(
                    b"%s:illegal fragment length\n\x00" as *const u8 as *const libc::c_char,
                    NET_AdrToString((*chan).remoteAddress as netadr_t),
                );
            }
            return qfalse;
        }
        crate::stdlib::memcpy(
            (*chan)
                .fragmentBuffer
                .as_mut_ptr()
                .offset((*chan).fragmentLength as isize) as *mut libc::c_void,
            (*msg).data.offset((*msg).readcount as isize) as *const libc::c_void,
            fragmentLength as usize,
        );
        (*chan).fragmentLength += fragmentLength;
        // if this wasn't the last fragment, don't process anything
        if fragmentLength == 1400 as i32 - 100 as i32 {
            return qfalse;
        }
        if (*chan).fragmentLength > (*msg).maxsize {
            Com_Printf(
                b"%s:fragmentLength %i > msg->maxsize\n\x00" as *const u8 as *const libc::c_char,
                NET_AdrToString((*chan).remoteAddress as netadr_t),
                (*chan).fragmentLength,
            );
            return qfalse;
        }
        // copy the full message over the partial fragment
        // make sure the sequence number is still there
        *((*msg).data as *mut i32) = sequence; // past the sequence number
        crate::stdlib::memcpy(
            (*msg).data.offset(4 as i32 as isize) as *mut libc::c_void,
            (*chan).fragmentBuffer.as_mut_ptr() as *const libc::c_void,
            (*chan).fragmentLength as usize,
        ); // past the sequence number
        (*msg).cursize = (*chan).fragmentLength + 4 as i32;
        (*chan).fragmentLength = 0 as i32;
        (*msg).readcount = 4 as i32;
        (*msg).bit = 32 as i32;
        // TTimo
        // clients were not acking fragmented messages
        (*chan).incomingSequence = sequence;
        return qtrue;
    }
    //
    // the message can now be read from the current message pointer
    //
    (*chan).incomingSequence = sequence;
    return qtrue;
}
#[no_mangle]

pub static mut loopbacks: [loopback_t; 2] = [loopback_t {
    msgs: [loopmsg_t {
        data: [0; 1400],
        datalen: 0,
    }; 16],
    get: 0,
    send: 0,
}; 2];
#[no_mangle]

pub unsafe extern "C" fn NET_GetLoopPacket(
    mut sock: netsrc_t,
    mut net_from: *mut netadr_t,
    mut net_message: *mut msg_t,
) -> qboolean {
    let mut i: i32 = 0;
    let mut loop_0: *mut loopback_t = 0 as *mut loopback_t;
    loop_0 = &mut *loopbacks.as_mut_ptr().offset(sock as isize) as *mut loopback_t;
    if (*loop_0).send - (*loop_0).get > 16 as i32 {
        (*loop_0).get = (*loop_0).send - 16 as i32
    }
    if (*loop_0).get >= (*loop_0).send {
        return qfalse;
    }
    i = (*loop_0).get & 16 as i32 - 1 as i32;
    (*loop_0).get += 1;
    crate::stdlib::memcpy(
        (*net_message).data as *mut libc::c_void,
        (*loop_0).msgs[i as usize].data.as_mut_ptr() as *const libc::c_void,
        (*loop_0).msgs[i as usize].datalen as usize,
    );
    (*net_message).cursize = (*loop_0).msgs[i as usize].datalen;
    crate::stdlib::memset(
        net_from as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<netadr_t>() as usize,
    );
    (*net_from).type_0 = NA_LOOPBACK;
    return qtrue;
}
#[no_mangle]

pub unsafe extern "C" fn NET_SendLoopPacket(
    mut sock: netsrc_t,
    mut length: i32,
    mut data: *const libc::c_void,
    mut _to: netadr_t,
) {
    let mut i: i32 = 0;
    let mut loop_0: *mut loopback_t = 0 as *mut loopback_t;
    loop_0 = &mut *loopbacks
        .as_mut_ptr()
        .offset((sock as u32 ^ 1 as i32 as u32) as isize) as *mut loopback_t;
    i = (*loop_0).send & 16 as i32 - 1 as i32;
    (*loop_0).send += 1;
    crate::stdlib::memcpy(
        (*loop_0).msgs[i as usize].data.as_mut_ptr() as *mut libc::c_void,
        data,
        length as usize,
    );
    (*loop_0).msgs[i as usize].datalen = length;
}
#[no_mangle]

pub static mut packetQueue: *mut packetQueue_t = 0 as *const packetQueue_t as *mut packetQueue_t;

unsafe extern "C" fn NET_QueuePacket(
    mut length: i32,
    mut data: *const libc::c_void,
    mut to: netadr_t,
    mut offset: i32,
) {
    let mut new: *mut packetQueue_t = 0 as *mut packetQueue_t;
    let mut next: *mut packetQueue_t = packetQueue;
    if offset > 999 as i32 {
        offset = 999 as i32
    }
    new = S_Malloc(::std::mem::size_of::<packetQueue_t>() as usize as i32)
        as *mut packetQueue_t;
    (*new).data = S_Malloc(length) as *mut byte;
    crate::stdlib::memcpy(
        (*new).data as *mut libc::c_void,
        data,
        length as usize,
    );
    (*new).length = length;
    (*new).to = to;
    (*new).release = Sys_Milliseconds() + (offset as f32 / (*com_timescale).value) as i32;
    (*new).next = 0 as *mut packetQueue_s;
    if packetQueue.is_null() {
        packetQueue = new;
        return;
    }
    while !next.is_null() {
        if (*next).next.is_null() {
            (*next).next = new;
            return;
        }
        next = (*next).next
    }
}
#[no_mangle]

pub unsafe extern "C" fn NET_FlushPacketQueue() {
    let mut last: *mut packetQueue_t = 0 as *mut packetQueue_t;
    let mut now: i32 = 0;
    while !packetQueue.is_null() {
        now = Sys_Milliseconds();
        if (*packetQueue).release >= now {
            break;
        }
        Sys_SendPacket(
            (*packetQueue).length,
            (*packetQueue).data as *const libc::c_void,
            (*packetQueue).to as netadr_t,
        );
        last = packetQueue;
        packetQueue = (*packetQueue).next;
        Z_Free((*last).data as *mut libc::c_void);
        Z_Free(last as *mut libc::c_void);
    }
}
#[no_mangle]

pub unsafe extern "C" fn NET_SendPacket(
    mut sock: netsrc_t,
    mut length: i32,
    mut data: *const libc::c_void,
    mut to: netadr_t,
) {
    // sequenced packets are shown in netchan, so just show oob
    if (*showpackets).integer != 0 && *(data as *mut i32) == -(1 as i32) {
        Com_Printf(
            b"send packet %4i\n\x00" as *const u8 as *const libc::c_char,
            length,
        );
    }
    if to.type_0 as u32 == NA_LOOPBACK as i32 as u32 {
        NET_SendLoopPacket(sock, length, data, to);
        return;
    }
    if to.type_0 as u32 == NA_BOT as i32 as u32 {
        return;
    }
    if to.type_0 as u32 == NA_BAD as i32 as u32 {
        return;
    }
    if sock as u32 == NS_CLIENT as i32 as u32 && (*cl_packetdelay).integer > 0 as i32 {
        NET_QueuePacket(length, data, to, (*cl_packetdelay).integer);
    } else if sock as u32 == NS_SERVER as i32 as u32 && (*sv_packetdelay).integer > 0 as i32 {
        NET_QueuePacket(length, data, to, (*sv_packetdelay).integer);
    } else {
        Sys_SendPacket(length, data, to as netadr_t);
    };
}
/*
===============
NET_OutOfBandPrint

Sends a text message in an out-of-band datagram
================
*/
#[no_mangle]

pub unsafe extern "C" fn NET_OutOfBandPrint(
    mut sock: netsrc_t,
    mut adr: netadr_t,
    mut format: *const libc::c_char,
    mut args: ...
) {
    let mut argptr: ::std::ffi::VaListImpl;
    let mut string: [libc::c_char; 16384] = [0; 16384];
    // set the header
    string[0 as i32 as usize] = -(1 as i32) as libc::c_char;
    string[1 as i32 as usize] = -(1 as i32) as libc::c_char;
    string[2 as i32 as usize] = -(1 as i32) as libc::c_char;
    string[3 as i32 as usize] = -(1 as i32) as libc::c_char;
    argptr = args.clone();
    crate::stdlib::vsnprintf(
        string.as_mut_ptr().offset(4 as i32 as isize),
        (::std::mem::size_of::<[libc::c_char; 16384]>() as usize)
            .wrapping_sub(4 as i32 as usize),
        format,
        argptr.as_va_list(),
    );
    // send the datagram
    NET_SendPacket(
        sock,
        crate::stdlib::strlen(string.as_mut_ptr()) as i32,
        string.as_mut_ptr() as *const libc::c_void,
        adr,
    );
}
/*
===============
NET_OutOfBandPrint

Sends a data message in an out-of-band datagram (only used for "connect")
================
*/
#[no_mangle]

pub unsafe extern "C" fn NET_OutOfBandData(
    mut sock: netsrc_t,
    mut adr: netadr_t,
    mut format: *mut byte,
    mut len: i32,
) {
    let mut string: [byte; 32768] = [0; 32768];
    let mut i: i32 = 0;
    let mut mbuf: msg_t = msg_t {
        allowoverflow: qfalse,
        overflowed: qfalse,
        oob: qfalse,
        data: 0 as *mut byte,
        maxsize: 0,
        cursize: 0,
        readcount: 0,
        bit: 0,
    };
    // set the header
    string[0 as i32 as usize] = 0xff as i32 as byte;
    string[1 as i32 as usize] = 0xff as i32 as byte;
    string[2 as i32 as usize] = 0xff as i32 as byte;
    string[3 as i32 as usize] = 0xff as i32 as byte;
    i = 0 as i32;
    while i < len {
        string[(i + 4 as i32) as usize] = *format.offset(i as isize);
        i += 1
    }
    mbuf.data = string.as_mut_ptr();
    mbuf.cursize = len + 4 as i32;
    Huff_Compress(&mut mbuf as *mut _ as *mut msg_t, 12 as i32);
    // send the datagram
    NET_SendPacket(sock, mbuf.cursize, mbuf.data as *const libc::c_void, adr);
}
/*
=============
NET_StringToAdr

Traps "localhost" for loopback, passes everything else to system
return 0 on address not found, 1 on address found with port, 2 on address found without port.
=============
*/
#[no_mangle]

pub unsafe extern "C" fn NET_StringToAdr(
    mut s: *const libc::c_char,
    mut a: *mut netadr_t,
    mut family: netadrtype_t,
) -> i32 {
    let mut base: [libc::c_char; 1024] = [0; 1024];
    let mut search: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut port: *mut libc::c_char = 0 as *mut libc::c_char;
    if libc::strcmp(s, b"localhost\x00" as *const u8 as *const libc::c_char) == 0 {
        crate::stdlib::memset(
            a as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<netadr_t>() as usize,
        );
        (*a).type_0 = NA_LOOPBACK;
        // as NA_LOOPBACK doesn't require ports report port was given.
        return 1 as i32;
    }
    Q_strncpyz(
        base.as_mut_ptr(),
        s,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as usize as i32,
    );
    if *base.as_mut_ptr() as i32 == '[' as i32
        || Q_CountChar(base.as_mut_ptr(), ':' as i32 as libc::c_char) > 1 as i32
    {
        // This is an ipv6 address, handle it specially.
        search = libc::strchr(base.as_mut_ptr(), ']' as i32);
        if !search.is_null() {
            *search = '\u{0}' as i32 as libc::c_char;
            search = search.offset(1);
            if *search as i32 == ':' as i32 {
                port = search.offset(1 as i32 as isize)
            }
        }
        if *base.as_mut_ptr() as i32 == '[' as i32 {
            search = base.as_mut_ptr().offset(1 as i32 as isize)
        } else {
            search = base.as_mut_ptr()
        }
    } else {
        // look for a port number
        port = libc::strchr(base.as_mut_ptr(), ':' as i32);
        if !port.is_null() {
            *port = '\u{0}' as i32 as libc::c_char;
            port = port.offset(1)
        }
        search = base.as_mut_ptr()
    }
    if Sys_StringToAdr(search, a as *mut netadr_t, family) as u64 == 0 {
        (*a).type_0 = NA_BAD;
        return 0 as i32;
    }
    if !port.is_null() {
        (*a).port = crate::src::qcommon::q_shared::ShortSwap(atoi(port) as i16) as u16;
        return 1 as i32;
    } else {
        (*a).port = crate::src::qcommon::q_shared::ShortSwap(27960 as i32 as i16) as u16;
        return 2 as i32;
    };
}
