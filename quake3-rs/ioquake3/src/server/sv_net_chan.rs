use ::libc;

pub use crate::g_public_h::entityShared_t;
pub use crate::g_public_h::sharedEntity_t;
pub use crate::qcommon_h::msg_t;
pub use crate::qcommon_h::netadr_t;
pub use crate::qcommon_h::netadrtype_t;
pub use crate::qcommon_h::netchan_t;
pub use crate::qcommon_h::netsrc_t;
pub use crate::qcommon_h::svc_EOF;
pub use crate::qcommon_h::svc_bad;
pub use crate::qcommon_h::svc_baseline;
pub use crate::qcommon_h::svc_configstring;
pub use crate::qcommon_h::svc_download;
pub use crate::qcommon_h::svc_gamestate;
pub use crate::qcommon_h::svc_nop;
pub use crate::qcommon_h::svc_ops_e;
pub use crate::qcommon_h::svc_serverCommand;
pub use crate::qcommon_h::svc_snapshot;
pub use crate::qcommon_h::svc_voipOpus;
pub use crate::qcommon_h::svc_voipSpeex;
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
pub use crate::server_h::clientSnapshot_t;
pub use crate::server_h::clientState_t;
pub use crate::server_h::client_s;
pub use crate::server_h::client_t;
pub use crate::server_h::netchan_buffer_s;
pub use crate::server_h::netchan_buffer_t;
pub use crate::server_h::voipServerPacket_s;
pub use crate::server_h::voipServerPacket_t;
pub use crate::server_h::CS_ACTIVE;
pub use crate::server_h::CS_CONNECTED;
pub use crate::server_h::CS_FREE;
pub use crate::server_h::CS_PRIMED;
pub use crate::server_h::CS_ZOMBIE;
pub use crate::src::qcommon::common::Com_DPrintf;
pub use crate::src::qcommon::common::Z_Free;
pub use crate::src::qcommon::common::Z_Malloc;
pub use crate::src::qcommon::msg::MSG_Copy;
pub use crate::src::qcommon::msg::MSG_ReadLong;
pub use crate::src::qcommon::msg::MSG_WriteByte;
pub use crate::src::qcommon::net_chan::Netchan_Process;
pub use crate::src::qcommon::net_chan::Netchan_Transmit;
pub use crate::src::qcommon::net_chan::Netchan_TransmitNextFragment;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::entityState_s;
pub use crate::src::qcommon::q_shared::entityState_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::playerState_s;
pub use crate::src::qcommon::q_shared::playerState_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::trType_t;
pub use crate::src::qcommon::q_shared::trajectory_t;
pub use crate::src::qcommon::q_shared::usercmd_s;
pub use crate::src::qcommon::q_shared::usercmd_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;
pub use crate::src::server::sv_main::SV_RateMsec;
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
/*
==============
SV_Netchan_Encode

    // first four bytes of the data are always:
    long reliableAcknowledge;

==============
*/

unsafe extern "C" fn SV_Netchan_Encode(
    mut client: *mut client_t,
    mut msg: *mut msg_t,
    mut clientCommandString: *const libc::c_char,
) {
    let mut i: isize = 0;
    let mut index: isize = 0;
    let mut key: byte = 0;
    let mut string: *mut byte =
        0 as *mut byte;
    let mut srdc: i32 = 0;
    let mut sbit: i32 = 0;
    let mut soob: qboolean = qfalse;
    if (*msg).cursize < 4 as i32 {
        return;
    }
    srdc = (*msg).readcount;
    sbit = (*msg).bit;
    soob = (*msg).oob;
    (*msg).bit = 0 as i32;
    (*msg).readcount = 0 as i32;
    (*msg).oob = qfalse;
    /* reliableAcknowledge = */
    MSG_ReadLong(msg as *mut msg_t);
    (*msg).oob = soob;
    (*msg).bit = sbit;
    (*msg).readcount = srdc;
    string = clientCommandString as *mut byte;
    index = 0 as i32 as isize;
    // xor the client challenge with the netchan sequence number
    key = ((*client).challenge ^ (*client).netchan.outgoingSequence)
        as byte;
    i = 4 as i32 as isize;
    while i < (*msg).cursize as isize {
        // modify the key with the last received and with this message acknowledged client command
        if *string.offset(index as isize) == 0 {
            index = 0 as i32 as isize
        }
        if *string.offset(index as isize) as i32 > 127 as i32
            || *string.offset(index as isize) as i32 == '%' as i32
        {
            key = (key as i32 ^ ('.' as i32) << (i & 1 as i32 as isize))
                as byte
        } else {
            key = (key as i32 ^ (*string.offset(index as isize) as i32) << (i & 1 as i32 as isize))
                as byte
        }
        index += 1;
        // encode the data with this key
        *(*msg).data.offset(i as isize) = (*(*msg).data.offset(i as isize) as i32 ^ key as i32)
            as byte;
        i += 1
    }
}
/*
==============
SV_Netchan_Decode

    // first 12 bytes of the data are always:
    long serverId;
    long messageAcknowledge;
    long reliableAcknowledge;

==============
*/

unsafe extern "C" fn SV_Netchan_Decode(
    mut client: *mut client_t,
    mut msg: *mut msg_t,
) {
    let mut serverId: i32 = 0;
    let mut messageAcknowledge: i32 = 0;
    let mut reliableAcknowledge: i32 = 0;
    let mut i: i32 = 0;
    let mut index: i32 = 0;
    let mut srdc: i32 = 0;
    let mut sbit: i32 = 0;
    let mut soob: qboolean = qfalse;
    let mut key: byte = 0;
    let mut string: *mut byte =
        0 as *mut byte;
    srdc = (*msg).readcount;
    sbit = (*msg).bit;
    soob = (*msg).oob;
    (*msg).oob = qfalse;
    serverId = MSG_ReadLong(msg as *mut msg_t);
    messageAcknowledge =
        MSG_ReadLong(msg as *mut msg_t);
    reliableAcknowledge =
        MSG_ReadLong(msg as *mut msg_t);
    (*msg).oob = soob;
    (*msg).bit = sbit;
    (*msg).readcount = srdc;
    string = (*client).reliableCommands[(reliableAcknowledge & 64 as i32 - 1 as i32) as usize]
        .as_mut_ptr() as *mut byte;
    index = 0 as i32;
    //
    key = ((*client).challenge ^ serverId ^ messageAcknowledge)
        as byte;
    i = (*msg).readcount + 12 as i32;
    while i < (*msg).cursize {
        // modify the key with the last sent and acknowledged server command
        if *string.offset(index as isize) == 0 {
            index = 0 as i32
        }
        if *string.offset(index as isize) as i32 > 127 as i32
            || *string.offset(index as isize) as i32 == '%' as i32
        {
            key =
                (key as i32 ^ ('.' as i32) << (i & 1 as i32)) as byte
        } else {
            key = (key as i32 ^ (*string.offset(index as isize) as i32) << (i & 1 as i32))
                as byte
        }
        index += 1;
        // decode the data with this key
        *(*msg).data.offset(i as isize) = (*(*msg).data.offset(i as isize) as i32 ^ key as i32)
            as byte;
        i += 1
    }
}
/*
=================
SV_Netchan_FreeQueue
=================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_Netchan_FreeQueue(mut client: *mut client_t) {
    let mut netbuf: *mut netchan_buffer_t =
        0 as *mut netchan_buffer_t;
    let mut next: *mut netchan_buffer_t =
        0 as *mut netchan_buffer_t;
    netbuf = (*client).netchan_start_queue;
    while !netbuf.is_null() {
        next = (*netbuf).next;
        Z_Free(netbuf as *mut libc::c_void);
        netbuf = next
    }
    (*client).netchan_start_queue = 0 as *mut netchan_buffer_t;
    (*client).netchan_end_queue = &mut (*client).netchan_start_queue;
}
/*
=================
SV_Netchan_TransmitNextInQueue
=================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_Netchan_TransmitNextInQueue(
    mut client: *mut client_t,
) {
    let mut netbuf: *mut netchan_buffer_t =
        0 as *mut netchan_buffer_t;
    Com_DPrintf(
        b"#462 Netchan_TransmitNextFragment: popping a queued message for transmit\n\x00"
            as *const u8 as *const libc::c_char,
    );
    netbuf = (*client).netchan_start_queue;
    if (*client).compat as u64 != 0 {
        SV_Netchan_Encode(
            client,
            &mut (*netbuf).msg,
            (*netbuf).clientCommandString.as_mut_ptr(),
        );
    }
    Netchan_Transmit(
        &mut (*client).netchan as *mut _ as *mut netchan_t,
        (*netbuf).msg.cursize,
        (*netbuf).msg.data,
    );
    // pop from queue
    (*client).netchan_start_queue = (*netbuf).next;
    if (*client).netchan_start_queue.is_null() {
        Com_DPrintf(
            b"#462 Netchan_TransmitNextFragment: emptied queue\n\x00" as *const u8
                as *const libc::c_char,
        );
        (*client).netchan_end_queue = &mut (*client).netchan_start_queue
    } else {
        Com_DPrintf(
            b"#462 Netchan_TransmitNextFragment: remaining queued message\n\x00" as *const u8
                as *const libc::c_char,
        );
    }
    Z_Free(netbuf as *mut libc::c_void);
}
/*
=================
SV_Netchan_TransmitNextFragment
Transmit the next fragment and the next queued packet
Return number of ms until next message can be sent based on throughput given by client rate,
-1 if no packet was sent.
=================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_Netchan_TransmitNextFragment(
    mut client: *mut client_t,
) -> i32 {
    if (*client).netchan.unsentFragments as u64 != 0 {
        Netchan_TransmitNextFragment(
            &mut (*client).netchan as *mut _ as *mut netchan_t,
        );
        return SV_RateMsec(client as *mut client_s);
    } else {
        if !(*client).netchan_start_queue.is_null() {
            SV_Netchan_TransmitNextInQueue(client);
            return SV_RateMsec(
                client as *mut client_s,
            );
        }
    }
    return -(1 as i32);
}
// clip to a specific entity
//
// sv_net_chan.c
//
/*
===============
SV_Netchan_Transmit
TTimo
https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=462
if there are some unsent fragments (which may happen if the snapshots
and the gamestate are fragmenting, and collide on send for instance)
then buffer them and make sure they get sent in correct order
================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_Netchan_Transmit(
    mut client: *mut client_t,
    mut msg: *mut msg_t,
) {
    MSG_WriteByte(
        msg as *mut msg_t,
        svc_EOF as i32,
    );
    if (*client).netchan.unsentFragments as u32 != 0 || !(*client).netchan_start_queue.is_null() {
        let mut netbuf: *mut netchan_buffer_t =
            0 as *mut netchan_buffer_t;
        Com_DPrintf(
            b"#462 SV_Netchan_Transmit: unsent fragments, stacked\n\x00" as *const u8
                as *const libc::c_char,
        );
        netbuf = Z_Malloc(::std::mem::size_of::<
            netchan_buffer_t,
        >() as libc::c_ulong as i32) as *mut netchan_buffer_t;
        // store the msg, we can't store it encoded, as the encoding depends on stuff we still have to finish sending
        MSG_Copy(
            &mut (*netbuf).msg as *mut _ as *mut msg_t,
            (*netbuf).msgBuffer.as_mut_ptr(),
            ::std::mem::size_of::<[byte; 16384]>() as libc::c_ulong
                as i32,
            msg as *mut msg_t,
        );
        if (*client).compat as u64 != 0 {
            Q_strncpyz(
                (*netbuf).clientCommandString.as_mut_ptr(),
                (*client).lastClientCommandString.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
            );
        }
        (*netbuf).next = 0 as *mut netchan_buffer_s;
        // insert it in the queue, the message will be encoded and sent later
        *(*client).netchan_end_queue = netbuf;
        (*client).netchan_end_queue = &mut (**(*client).netchan_end_queue).next
    } else {
        if (*client).compat as u64 != 0 {
            SV_Netchan_Encode(client, msg, (*client).lastClientCommandString.as_mut_ptr());
        }
        Netchan_Transmit(
            &mut (*client).netchan as *mut _ as *mut netchan_t,
            (*msg).cursize,
            (*msg).data,
        );
    };
}
/*
=================
Netchan_SV_Process
=================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_Netchan_Process(
    mut client: *mut client_t,
    mut msg: *mut msg_t,
) -> qboolean {
    let mut ret: i32 = 0;
    ret = Netchan_Process(
        &mut (*client).netchan as *mut _ as *mut netchan_t,
        msg as *mut msg_t,
    ) as i32;
    if ret == 0 {
        return qfalse;
    }
    if (*client).compat as u64 != 0 {
        SV_Netchan_Decode(client, msg);
    }
    return qtrue;
}
