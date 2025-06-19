use ::libc;

pub use crate::stdlib::__uint8_t;
pub use crate::stdlib::uint8_t;

pub use crate::curl_h::CURL;
pub use crate::multi_h::CURLM;
pub use crate::qcommon_h::clc_EOF;
pub use crate::qcommon_h::clc_bad;
pub use crate::qcommon_h::clc_clientCommand;
pub use crate::qcommon_h::clc_move;
pub use crate::qcommon_h::clc_moveNoDelta;
pub use crate::qcommon_h::clc_nop;
pub use crate::qcommon_h::clc_ops_e;
pub use crate::qcommon_h::clc_voipOpus;
pub use crate::qcommon_h::clc_voipSpeex;
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
pub use crate::src::qcommon::common::Com_DPrintf;
pub use crate::src::qcommon::msg::MSG_ReadLong;
pub use crate::src::qcommon::msg::MSG_WriteByte;
pub use crate::src::qcommon::net_chan::Netchan_Process;
pub use crate::src::qcommon::net_chan::Netchan_Transmit;
pub use crate::src::qcommon::net_chan::Netchan_TransmitNextFragment;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::connstate_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::CA_ACTIVE;
pub use crate::src::qcommon::q_shared::CA_AUTHORIZING;
pub use crate::src::qcommon::q_shared::CA_CHALLENGING;
pub use crate::src::qcommon::q_shared::CA_CINEMATIC;
pub use crate::src::qcommon::q_shared::CA_CONNECTED;
pub use crate::src::qcommon::q_shared::CA_CONNECTING;
pub use crate::src::qcommon::q_shared::CA_DISCONNECTED;
pub use crate::src::qcommon::q_shared::CA_LOADING;
pub use crate::src::qcommon::q_shared::CA_PRIMED;
pub use crate::src::qcommon::q_shared::CA_UNINITIALIZED;

pub use crate::client_h::clientConnection_t;
pub use crate::src::client::cl_main::clc;

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
CL_Netchan_Encode

    // first 12 bytes of the data are always:
    long serverId;
    long messageAcknowledge;
    long reliableAcknowledge;

==============
*/

unsafe extern "C" fn CL_Netchan_Encode(mut msg: *mut crate::qcommon_h::msg_t) {
    let mut serverId: i32 = 0;
    let mut messageAcknowledge: i32 = 0;
    let mut reliableAcknowledge: i32 = 0;
    let mut i: i32 = 0;
    let mut index: i32 = 0;
    let mut srdc: i32 = 0;
    let mut sbit: i32 = 0;
    let mut soob: i32 = 0;
    let mut key: crate::src::qcommon::q_shared::byte = 0;
    let mut string: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    if (*msg).cursize <= 12 as i32 {
        return;
    }
    srdc = (*msg).readcount;
    sbit = (*msg).bit;
    soob = (*msg).oob as i32;
    (*msg).bit = 0 as i32;
    (*msg).readcount = 0 as i32;
    (*msg).oob = crate::src::qcommon::q_shared::qfalse;
    serverId = crate::src::qcommon::msg::MSG_ReadLong(msg as *mut crate::qcommon_h::msg_t);
    messageAcknowledge =
        crate::src::qcommon::msg::MSG_ReadLong(msg as *mut crate::qcommon_h::msg_t);
    reliableAcknowledge =
        crate::src::qcommon::msg::MSG_ReadLong(msg as *mut crate::qcommon_h::msg_t);
    (*msg).oob = soob as crate::src::qcommon::q_shared::qboolean;
    (*msg).bit = sbit;
    (*msg).readcount = srdc;
    string = crate::src::client::cl_main::clc.serverCommands
        [(reliableAcknowledge & 64 as i32 - 1 as i32) as usize]
        .as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte;
    index = 0 as i32;
    //
    key = (crate::src::client::cl_main::clc.challenge ^ serverId ^ messageAcknowledge)
        as crate::src::qcommon::q_shared::byte;
    i = 12 as i32;
    while i < (*msg).cursize {
        // modify the key with the last received now acknowledged server command
        if *string.offset(index as isize) == 0 {
            index = 0 as i32
        }
        if *string.offset(index as isize) as i32 > 127 as i32
            || *string.offset(index as isize) as i32 == '%' as i32
        {
            key =
                (key as i32 ^ ('.' as i32) << (i & 1 as i32)) as crate::src::qcommon::q_shared::byte
        } else {
            key = (key as i32 ^ (*string.offset(index as isize) as i32) << (i & 1 as i32))
                as crate::src::qcommon::q_shared::byte
        }
        index += 1;
        // encode the data with this key
        *(*msg).data.offset(i as isize) = (*(*msg).data.offset(i as isize) as i32 ^ key as i32)
            as crate::src::qcommon::q_shared::byte;
        i += 1
    }
}
/*
==============
CL_Netchan_Decode

    // first four bytes of the data are always:
    long reliableAcknowledge;

==============
*/

unsafe extern "C" fn CL_Netchan_Decode(mut msg: *mut crate::qcommon_h::msg_t) {
    let mut reliableAcknowledge: isize = 0;
    let mut i: isize = 0;
    let mut index: isize = 0;
    let mut key: crate::src::qcommon::q_shared::byte = 0;
    let mut string: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut srdc: i32 = 0;
    let mut sbit: i32 = 0;
    let mut soob: i32 = 0;
    srdc = (*msg).readcount;
    sbit = (*msg).bit;
    soob = (*msg).oob as i32;
    (*msg).oob = crate::src::qcommon::q_shared::qfalse;
    reliableAcknowledge =
        crate::src::qcommon::msg::MSG_ReadLong(msg as *mut crate::qcommon_h::msg_t) as isize;
    (*msg).oob = soob as crate::src::qcommon::q_shared::qboolean;
    (*msg).bit = sbit;
    (*msg).readcount = srdc;
    string = crate::src::client::cl_main::clc.reliableCommands
        [(reliableAcknowledge & (64 as i32 - 1 as i32) as isize) as usize]
        .as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte;
    index = 0 as i32 as isize;
    // xor the client challenge with the netchan sequence number (need something that changes every message)
    key = (crate::src::client::cl_main::clc.challenge as u32 ^ *((*msg).data as *mut u32))
        as crate::src::qcommon::q_shared::byte;
    i = ((*msg).readcount + 4 as i32) as isize;
    while i < (*msg).cursize as isize {
        // modify the key with the last sent and with this message acknowledged client command
        if *string.offset(index as isize) == 0 {
            index = 0 as i32 as isize
        }
        if *string.offset(index as isize) as i32 > 127 as i32
            || *string.offset(index as isize) as i32 == '%' as i32
        {
            key = (key as i32 ^ ('.' as i32) << (i & 1 as i32 as isize))
                as crate::src::qcommon::q_shared::byte
        } else {
            key = (key as i32 ^ (*string.offset(index as isize) as i32) << (i & 1 as i32 as isize))
                as crate::src::qcommon::q_shared::byte
        }
        index += 1;
        // decode the data with this key
        *(*msg).data.offset(i as isize) = (*(*msg).data.offset(i as isize) as i32 ^ key as i32)
            as crate::src::qcommon::q_shared::byte;
        i += 1
    }
}
/*
=================
CL_Netchan_TransmitNextFragment
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_Netchan_TransmitNextFragment(
    mut chan: *mut crate::qcommon_h::netchan_t,
) -> crate::src::qcommon::q_shared::qboolean {
    if (*chan).unsentFragments as u64 != 0 {
        crate::src::qcommon::net_chan::Netchan_TransmitNextFragment(
            chan as *mut crate::qcommon_h::netchan_t,
        );
        return crate::src::qcommon::q_shared::qtrue;
    }
    return crate::src::qcommon::q_shared::qfalse;
}
/*
===============
CL_Netchan_Transmit
================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_Netchan_Transmit(
    mut chan: *mut crate::qcommon_h::netchan_t,
    mut msg: *mut crate::qcommon_h::msg_t,
) {
    crate::src::qcommon::msg::MSG_WriteByte(
        msg as *mut crate::qcommon_h::msg_t,
        crate::qcommon_h::clc_EOF as i32,
    );
    if (*chan).compat as u64 != 0 {
        CL_Netchan_Encode(msg);
    }
    crate::src::qcommon::net_chan::Netchan_Transmit(
        chan as *mut crate::qcommon_h::netchan_t,
        (*msg).cursize,
        (*msg).data,
    );
    // Transmit all fragments without delay
    while CL_Netchan_TransmitNextFragment(chan) as u64 != 0 {
        crate::src::qcommon::common::Com_DPrintf(
            b"WARNING: #462 unsent fragments (not supposed to happen!)\n\x00" as *const u8
                as *const libc::c_char,
        );
    }
}
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
// client.h -- primary header for client
/* USE_CURL */
// file full of random crap that gets used to create cl_guid
// time between connection packet retransmits
// snapshots are a view of the server at a given time
// cleared if delta parsing was invalid
// rate delayed and dropped commands
// server time the message is valid for (in msec)
// copied from netchan->incoming_sequence
// messageNum the delta is from
// time from when cmdNum-1 was sent to time packet was reeceived
// portalarea visibility bits
// the next cmdNum the server is expecting
// complete information about the current player at this time
// all of the entities that need to be presented
// at the time of this snapshot
// execute all commands up to this before
// making the snapshot current
/*
=============================================================================

the clientActive_t structure is wiped completely at every
new gamestate_t, potentially several times during an established connection

=============================================================================
*/
// cl.cmdNumber when packet was sent
// usercmd->serverTime when packet was sent
// cls.realtime when packet was sent
// the parseEntities array must be large enough to hold PACKET_BACKUP frames of
// entities, so that when a delta compressed message arives from the server
// it can be un-deltad from the original
// it requres several frames in a timeout condition
// to disconnect, preventing debugging breaks from
// causing immediate disconnects on continue
// latest received from server
// may be paused during play
// to prevent time from flowing bakcwards
// to check tournament restarts
// cl.serverTime = cls.realtime + cl.serverTimeDelta
// this value changes as net lag varies
// set if any cgame frame has been forced to extrapolate
// cleared when CL_AdjustTimeDelta looks at it
// set on parse of any valid packet
// configstrings
// extracted from CS_SERVERINFO
// index (not anded off) into cl_parse_entities[]
// added to by mouse events
// set by joystick events
// cgame communicates a few values to the client system
// current weapon to add to usercmd_t
// cmds[cmdNumber] is the predicted command, [cmdNumber-1] is the last
// properly generated command
// each mesage will send several old cmds
// incremented each frame, because multiple
// frames may need to be packed into a single packet
// information about each packet we have sent out
// the client maintains its own idea of view angles, which are
// sent to the server each frame.  It is cleared to 0 upon entering each level.
// the server sends a delta each frame which is added to the locally
// tracked view angles to account for standing on rotating objects,
// and teleport direction changes
// included in each client message so the server
// can tell if it is for a prior map_restart
// big stuff at end of structure so most offsets are 15 bits or less
// for delta compression when not in previous frame
/*
=============================================================================

the clientConnection_t structure is wiped when disconnecting from a server,
either to go to a full screen console, play a demo, or connect to a different server

A connection can be to either a server through the network layer or a
demo through a file.

=============================================================================
*/
// connection status
// for retransmits during connection
// for timeouts
// name of server from original connect (used by reconnect)
// for connection retransmits
// for display on connection dialog
// for display on connection dialog
// from the server to use for connecting
// from the server for checksum calculations
// these are our reliable messages that go to the server
// the last one the server has executed
// server message (unreliable) and command (reliable) sequence
// numbers are NOT cleared at level changes, but continue to
// increase as long as the connection is valid
// message sequence is used by both the network layer and the
// delta compression layer
// reliable messages received from server
// last server command grabbed or executed with CL_GetServerCommand
// file transfer from server
/* USE_CURL */
// block we are waiting for
// how many bytes we got
// how many bytes we got
// list of paks we need to download
// if true, we need to do another FS_Restart because we downloaded a pak
// demo information
// don't record until a non-delta message is received
// counter of rendered frames
// cls.realtime before first frame
// each frame will be at this time + frameNum * 50
// time the last frame was rendered
// minimum frame duration
// maximum frame duration
// log of frame durations
// incoming data...
// !!! FIXME: convert from parallel arrays to array of a struct.
// outgoing data...
// if voipTargets[i / 8] & (1 << (i % 8)),
// then we are sending to clientnum i.
// big stuff at end of structure so most offsets are 15 bits or less
/*
==================================================================

the clientStatic_t structure is never wiped, and is used even when
no client connection is active at all
(except when CL_Shutdown is called)

==================================================================
*/
// bring up the cd needed dialog next frame
// when the server clears the hunk, all of these must be restarted
// msec since last frame
// ignores pause
// ignoring pause, so console always works
// additional global servers
// source currently pinging or updating
// update server info
// rendering info
//=============================================================================
// interface to cgame dll or vm
// interface to ui dll or vm
// interface to refresh .dll
//
// cvars
//
// cl_voipSendTarget is a string: "all" to broadcast to everyone, "none" to
//  send to no one, or a comma-separated list of client numbers:
//  "0,7,2,23" ... an empty string is treated like "all".
// 20ms at 48k
// 3 frame is 60ms of audio, the max opus will encode at once
//=================================================
//
// cl_main
//
//
// cl_input
//
// key nums holding it down
// msec timestamp
// msec down this frame if both a down and up happened
// current state
// set when down, not cleared when up
//
// cl_parse.c
//
//====================================================================
//
// console
//
//
// cl_scrn.c
//
// returns in virtual 640x480 coordinates
// draws a string with embedded color control characters with fade
// ignores embedded color control characters
//
// cl_cin.c
//
//
// cl_cgame.c
//
//
// cl_ui.c
//
//
// cl_net_chan.c
//
//int length, const byte *data );
/*
=================
CL_Netchan_Process
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_Netchan_Process(
    mut chan: *mut crate::qcommon_h::netchan_t,
    mut msg: *mut crate::qcommon_h::msg_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut ret: i32 = 0;
    ret = crate::src::qcommon::net_chan::Netchan_Process(
        chan as *mut crate::qcommon_h::netchan_t,
        msg as *mut crate::qcommon_h::msg_t,
    ) as i32;
    if ret == 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if (*chan).compat as u64 != 0 {
        CL_Netchan_Decode(msg);
    }
    return crate::src::qcommon::q_shared::qtrue;
}
