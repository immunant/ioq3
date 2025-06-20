use ::c2rust_asm_casts;
use ::libc;

pub mod byteswap_h {
    #[inline]

    pub unsafe extern "C" fn __bswap_16(
        mut __bsx: crate::stdlib::__uint16_t,
    ) -> crate::stdlib::__uint16_t {
        return (__bsx as i32 >> 8 as i32 & 0xff as i32 | (__bsx as i32 & 0xff as i32) << 8 as i32)
            as crate::stdlib::__uint16_t;
    }
}

pub use crate::stddef_h::size_t;
pub use crate::stdlib::__socklen_t;
pub use crate::stdlib::__ssize_t;
pub use crate::stdlib::__suseconds_t;
pub use crate::stdlib::__time_t;
pub use crate::stdlib::__uint16_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::__uint8_t;
pub use crate::stdlib::ssize_t;
pub use ::libc::timeval;
use c2rust_asm_casts::AsmCastTrait;

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::qcommon_h::msg_t;
pub use crate::qcommon_h::netadr_t;
pub use crate::qcommon_h::netadrtype_t;
pub use crate::qcommon_h::xcommand_t;
pub use crate::qcommon_h::NA_BAD;
pub use crate::qcommon_h::NA_BOT;
pub use crate::qcommon_h::NA_BROADCAST;
pub use crate::qcommon_h::NA_IP;
pub use crate::qcommon_h::NA_IP6;
pub use crate::qcommon_h::NA_LOOPBACK;
pub use crate::qcommon_h::NA_MULTICAST6;
pub use crate::qcommon_h::NA_UNSPEC;
pub use crate::src::client::cl_main::CL_PacketEvent;
pub use crate::src::qcommon::cmd::Cmd_AddCommand;
pub use crate::src::qcommon::common::com_sv_running;
pub use crate::src::qcommon::common::Com_DPrintf;
pub use crate::src::qcommon::common::Com_Error;
pub use crate::src::qcommon::common::Com_Printf;
pub use crate::src::qcommon::common::Com_RunAndTimeServerPacket;
pub use crate::src::qcommon::cvar::Cvar_Get;
pub use crate::src::qcommon::cvar::Cvar_SetValue;
pub use crate::src::qcommon::msg::MSG_Init;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Q_CountChar;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;
pub use crate::stdlib::__fd_mask;
pub use crate::stdlib::fd_set;
pub use crate::stdlib::in6_addr;
pub use crate::stdlib::in6addr_any;
pub use crate::stdlib::in_addr_t;
pub use crate::stdlib::in_port_t;
pub use crate::stdlib::ipv6_mreq;
pub use crate::stdlib::sa_family_t;
pub use crate::stdlib::select;
pub use crate::stdlib::sockaddr_in6;
pub use crate::stdlib::uint16_t;
pub use crate::stdlib::uint32_t;
pub use crate::stdlib::uint8_t;
pub use crate::stdlib::C2RustUnnamed_129;
pub use crate::stdlib::IPPROTO_AH;
pub use crate::stdlib::IPPROTO_BEETPH;
pub use crate::stdlib::IPPROTO_COMP;
pub use crate::stdlib::IPPROTO_DCCP;
pub use crate::stdlib::IPPROTO_EGP;
pub use crate::stdlib::IPPROTO_ENCAP;
pub use crate::stdlib::IPPROTO_ESP;
pub use crate::stdlib::IPPROTO_GRE;
pub use crate::stdlib::IPPROTO_ICMP;
pub use crate::stdlib::IPPROTO_IDP;
pub use crate::stdlib::IPPROTO_IGMP;
pub use crate::stdlib::IPPROTO_IP;
pub use crate::stdlib::IPPROTO_IPIP;
pub use crate::stdlib::IPPROTO_IPV6;
pub use crate::stdlib::IPPROTO_MAX;
pub use crate::stdlib::IPPROTO_MPLS;
pub use crate::stdlib::IPPROTO_MTP;
pub use crate::stdlib::IPPROTO_PIM;
pub use crate::stdlib::IPPROTO_PUP;
pub use crate::stdlib::IPPROTO_RAW;
pub use crate::stdlib::IPPROTO_RSVP;
pub use crate::stdlib::IPPROTO_SCTP;
pub use crate::stdlib::IPPROTO_TCP;
pub use crate::stdlib::IPPROTO_TP;
pub use crate::stdlib::IPPROTO_UDP;
pub use crate::stdlib::IPPROTO_UDPLITE;
pub use ::libc::in_addr;
pub use ::libc::sockaddr_in;

pub use crate::src::qcommon::net_ip::byteswap_h::__bswap_16;
pub use crate::stdlib::__socket_type;
pub use crate::stdlib::freeifaddrs;
pub use crate::stdlib::gethostbyname;
pub use crate::stdlib::getifaddrs;
pub use crate::stdlib::ifaddrs;

pub use crate::stdlib::sockaddr_storage;
pub use crate::stdlib::socklen_t;

pub use crate::stdlib::C2RustUnnamed_131;
pub use crate::stdlib::IFF_ALLMULTI;
pub use crate::stdlib::IFF_AUTOMEDIA;
pub use crate::stdlib::IFF_BROADCAST;
pub use crate::stdlib::IFF_DEBUG;
pub use crate::stdlib::IFF_DYNAMIC;
pub use crate::stdlib::IFF_LOOPBACK;
pub use crate::stdlib::IFF_MASTER;
pub use crate::stdlib::IFF_MULTICAST;
pub use crate::stdlib::IFF_NOARP;
pub use crate::stdlib::IFF_NOTRAILERS;
pub use crate::stdlib::IFF_POINTOPOINT;
pub use crate::stdlib::IFF_PORTSEL;
pub use crate::stdlib::IFF_PROMISC;
pub use crate::stdlib::IFF_RUNNING;
pub use crate::stdlib::IFF_SLAVE;
pub use crate::stdlib::IFF_UP;
pub use crate::stdlib::SOCK_CLOEXEC;
pub use crate::stdlib::SOCK_DCCP;
pub use crate::stdlib::SOCK_DGRAM;
pub use crate::stdlib::SOCK_NONBLOCK;
pub use crate::stdlib::SOCK_PACKET;
pub use crate::stdlib::SOCK_RAW;
pub use crate::stdlib::SOCK_RDM;
pub use crate::stdlib::SOCK_SEQPACKET;
pub use crate::stdlib::SOCK_STREAM;

pub use ::libc::addrinfo;

pub use ::libc::freeaddrinfo;
pub use ::libc::gai_strerror;
pub use ::libc::getaddrinfo;
pub use ::libc::getnameinfo;
pub use ::libc::hostent;
pub use ::libc::if_nametoindex;

pub use ::libc::sockaddr;

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

pub type SOCKET = i32;

pub type ioctlarg_t = i32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nip_localaddr_t {
    pub ifname: [libc::c_char; 16],
    pub type_0: netadrtype_t,
    pub family: sa_family_t,
    pub addr: sockaddr_storage,
    pub netmask: sockaddr_storage,
}

static mut usingSocks: qboolean = qfalse;

static mut networkingEnabled: i32 = 0 as i32;

static mut net_enabled: *mut cvar_t = std::ptr::null_mut();

static mut net_socksEnabled: *mut cvar_t = std::ptr::null_mut();

static mut net_socksServer: *mut cvar_t = std::ptr::null_mut();

static mut net_socksPort: *mut cvar_t = std::ptr::null_mut();

static mut net_socksUsername: *mut cvar_t = std::ptr::null_mut();

static mut net_socksPassword: *mut cvar_t = std::ptr::null_mut();

static mut net_ip: *mut cvar_t = std::ptr::null_mut();

static mut net_ip6: *mut cvar_t = std::ptr::null_mut();

static mut net_port: *mut cvar_t = std::ptr::null_mut();

static mut net_port6: *mut cvar_t = std::ptr::null_mut();

static mut net_mcast6addr: *mut cvar_t = std::ptr::null_mut();

static mut net_mcast6iface: *mut cvar_t = std::ptr::null_mut();

static mut net_dropsim: *mut cvar_t = std::ptr::null_mut();

static mut socksRelayAddr: sockaddr = sockaddr {
    sa_family: 0,
    sa_data: [0; 14],
};

static mut ip_socket: SOCKET = -(1 as i32);

static mut ip6_socket: SOCKET = -(1 as i32);

static mut socks_socket: SOCKET = -(1 as i32);

static mut multicast6_socket: SOCKET = -(1 as i32);
// Keep track of currently joined multicast group.

static mut curgroup: ipv6_mreq = ipv6_mreq {
    ipv6mr_multiaddr: in6_addr {
        __in6_u: C2RustUnnamed_129 {
            __u6_addr8: [0; 16],
        },
    },
    ipv6mr_interface: 0,
};
// And the currently bound address.

static mut boundto: sockaddr_in6 = sockaddr_in6 {
    sin6_family: 0,
    sin6_port: 0,
    sin6_flowinfo: 0,
    sin6_addr: in6_addr {
        __in6_u: C2RustUnnamed_129 {
            __u6_addr8: [0; 16],
        },
    },
    sin6_scope_id: 0,
};

static mut localIP: [nip_localaddr_t; 32] = [nip_localaddr_t {
    ifname: [0; 16],
    type_0: NA_BAD,
    family: 0,
    addr: sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    },
    netmask: sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    },
}; 32];

static mut numIP: i32 = 0;
//=============================================================================
/*
====================
NET_ErrorString
====================
*/
#[no_mangle]

pub unsafe extern "C" fn NET_ErrorString() -> *mut libc::c_char {
    return libc::strerror(*libc::__errno_location());
}

unsafe extern "C" fn NetadrToSockadr(mut a: *mut netadr_t, mut s: *mut sockaddr) {
    if (*a).type_0 as u32 == NA_BROADCAST as i32 as u32 {
        (*(s as *mut sockaddr_in)).sin_family = 2 as i32 as sa_family_t;
        (*(s as *mut sockaddr_in)).sin_port = (*a).port;
        (*(s as *mut sockaddr_in)).sin_addr.s_addr = 0xffffffff as u32
    } else if (*a).type_0 as u32 == NA_IP as i32 as u32 {
        (*(s as *mut sockaddr_in)).sin_family = 2 as i32 as sa_family_t;
        (*(s as *mut sockaddr_in)).sin_addr.s_addr =
            *(&mut (*a).ip as *mut [byte; 4] as *mut i32) as in_addr_t;
        (*(s as *mut sockaddr_in)).sin_port = (*a).port
    } else if (*a).type_0 as u32 == NA_IP6 as i32 as u32 {
        (*(s as *mut sockaddr_in6)).sin6_family = 10 as i32 as sa_family_t;
        (*(s as *mut sockaddr_in6)).sin6_addr =
            *(&mut (*a).ip6 as *mut [byte; 16] as *mut in6_addr);
        (*(s as *mut sockaddr_in6)).sin6_port = (*a).port;
        (*(s as *mut sockaddr_in6)).sin6_scope_id = (*a).scope_id as uint32_t
    } else if (*a).type_0 as u32 == NA_MULTICAST6 as i32 as u32 {
        (*(s as *mut sockaddr_in6)).sin6_family = 10 as i32 as sa_family_t;
        (*(s as *mut sockaddr_in6)).sin6_addr = curgroup.ipv6mr_multiaddr;
        (*(s as *mut sockaddr_in6)).sin6_port = (*a).port
    };
}

unsafe extern "C" fn SockadrToNetadr(mut s: *mut sockaddr, mut a: *mut netadr_t) {
    if (*s).sa_family as i32 == 2 as i32 {
        (*a).type_0 = NA_IP;
        *(&mut (*a).ip as *mut [byte; 4] as *mut i32) =
            (*(s as *mut sockaddr_in)).sin_addr.s_addr as i32;
        (*a).port = (*(s as *mut sockaddr_in)).sin_port
    } else if (*s).sa_family as i32 == 10 as i32 {
        (*a).type_0 = NA_IP6;
        crate::stdlib::memcpy(
            (*a).ip6.as_mut_ptr() as *mut libc::c_void,
            &mut (*(s as *mut sockaddr_in6)).sin6_addr as *mut in6_addr as *const libc::c_void,
            ::std::mem::size_of::<[byte; 16]>() as usize,
        );
        (*a).port = (*(s as *mut sockaddr_in6)).sin6_port;
        (*a).scope_id = (*(s as *mut sockaddr_in6)).sin6_scope_id as usize
    };
}

unsafe extern "C" fn SearchAddrInfo(
    mut hints: *mut addrinfo,
    mut family: sa_family_t,
) -> *mut addrinfo {
    while !hints.is_null() {
        if (*hints).ai_family == family as i32 {
            return hints;
        }
        hints = (*hints).ai_next
    }
    return std::ptr::null_mut();
}
/*
=============
Sys_StringToSockaddr
=============
*/

unsafe extern "C" fn Sys_StringToSockaddr(
    mut s: *const libc::c_char,
    mut sadr: *mut sockaddr,
    mut sadr_len: i32,
    mut family: sa_family_t,
) -> qboolean {
    let mut hints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: std::ptr::null_mut(),
        ai_canonname: std::ptr::null_mut(),
        ai_next: std::ptr::null_mut(),
    };
    let mut res: *mut addrinfo = std::ptr::null_mut();
    let mut search: *mut addrinfo = std::ptr::null_mut();
    let mut hintsp: *mut addrinfo = std::ptr::null_mut();
    let mut retval: i32 = 0;
    crate::stdlib::memset(
        sadr as *mut libc::c_void,
        '\u{0}' as i32,
        ::std::mem::size_of::<sockaddr>() as usize,
    );
    crate::stdlib::memset(
        &mut hints as *mut addrinfo as *mut libc::c_void,
        '\u{0}' as i32,
        ::std::mem::size_of::<addrinfo>() as usize,
    );
    hintsp = &mut hints;
    (*hintsp).ai_family = family as i32;
    (*hintsp).ai_socktype = SOCK_DGRAM as i32;
    retval = getaddrinfo(
        s,
        std::ptr::null(),
        hintsp as *const addrinfo,
        &mut res as *mut _ as *mut *mut addrinfo,
    );
    if retval == 0 {
        if family as i32 == 0 as i32 {
            // Decide here and now which protocol family to use
            if (*net_enabled).integer & 0x4 as i32 != 0 {
                if (*net_enabled).integer & 0x2 as i32 != 0 {
                    search = SearchAddrInfo(res, 10 as i32 as sa_family_t)
                }
                if search.is_null() && (*net_enabled).integer & 0x1 as i32 != 0 {
                    search = SearchAddrInfo(res, 2 as i32 as sa_family_t)
                }
            } else {
                if (*net_enabled).integer & 0x1 as i32 != 0 {
                    search = SearchAddrInfo(res, 2 as i32 as sa_family_t)
                }
                if search.is_null() && (*net_enabled).integer & 0x2 as i32 != 0 {
                    search = SearchAddrInfo(res, 10 as i32 as sa_family_t)
                }
            }
        } else {
            search = SearchAddrInfo(res, family)
        }
        if !search.is_null() {
            if (*search).ai_addrlen > sadr_len as u32 {
                (*search).ai_addrlen = sadr_len as socklen_t
            }
            crate::stdlib::memcpy(
                sadr as *mut libc::c_void,
                (*search).ai_addr as *const libc::c_void,
                (*search).ai_addrlen as usize,
            );
            freeaddrinfo(res as *mut addrinfo);
            return qtrue;
        } else {
            Com_Printf(b"Sys_StringToSockaddr: Error resolving %s: No address of required type found.\n\x00"
                           as *const u8 as *const libc::c_char, s);
        }
    } else {
        Com_Printf(
            b"Sys_StringToSockaddr: Error resolving %s: %s\n\x00" as *const u8
                as *const libc::c_char,
            s,
            gai_strerror(retval),
        );
    }
    if !res.is_null() {
        freeaddrinfo(res as *mut addrinfo);
    }
    return qfalse;
}
/*
=============
Sys_SockaddrToString
=============
*/

unsafe extern "C" fn Sys_SockaddrToString(
    mut dest: *mut libc::c_char,
    mut destlen: i32,
    mut input: *mut sockaddr,
) {
    let mut inputlen: socklen_t = 0;
    if (*input).sa_family as i32 == 10 as i32 {
        inputlen = ::std::mem::size_of::<sockaddr_in6>() as usize as socklen_t
    } else {
        inputlen = ::std::mem::size_of::<sockaddr_in>() as usize as socklen_t
    }
    if getnameinfo(
        input as *const sockaddr,
        inputlen,
        dest,
        destlen as socklen_t,
        std::ptr::null_mut(),
        0 as i32 as socklen_t,
        1 as i32,
    ) != 0
        && destlen > 0 as i32
    {
        *dest = '\u{0}' as i32 as libc::c_char
    };
}
/*
=============
Sys_StringToAdr
=============
*/
#[no_mangle]

pub unsafe extern "C" fn Sys_StringToAdr(
    mut s: *const libc::c_char,
    mut a: *mut netadr_t,
    mut family: netadrtype_t,
) -> qboolean {
    let mut sadr: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut fam: sa_family_t = 0;
    match family as u32 {
        4 => fam = 2 as i32 as sa_family_t,
        5 => fam = 10 as i32 as sa_family_t,
        _ => fam = 0 as i32 as sa_family_t,
    }
    if Sys_StringToSockaddr(
        s,
        &mut sadr as *mut sockaddr_storage as *mut sockaddr,
        ::std::mem::size_of::<sockaddr_storage>() as usize as i32,
        fam,
    ) as u64
        == 0
    {
        return qfalse;
    }
    SockadrToNetadr(&mut sadr as *mut sockaddr_storage as *mut sockaddr, a);
    return qtrue;
}
/*
===================
NET_CompareBaseAdrMask

Compare without port, and up to the bit number given in netmask.
===================
*/
#[no_mangle]

pub unsafe extern "C" fn NET_CompareBaseAdrMask(
    mut a: netadr_t,
    mut b: netadr_t,
    mut netmask: i32,
) -> qboolean {
    let mut cmpmask: byte = 0;
    let mut addra: *mut byte = std::ptr::null_mut();
    let mut addrb: *mut byte = std::ptr::null_mut();
    let mut curbyte: i32 = 0;
    if a.type_0 as u32 != b.type_0 as u32 {
        return qfalse;
    }
    if a.type_0 as u32 == NA_LOOPBACK as i32 as u32 {
        return qtrue;
    }
    if a.type_0 as u32 == NA_IP as i32 as u32 {
        addra = &mut a.ip as *mut [byte; 4] as *mut byte;
        addrb = &mut b.ip as *mut [byte; 4] as *mut byte;
        if netmask < 0 as i32 || netmask > 32 as i32 {
            netmask = 32 as i32
        }
    } else if a.type_0 as u32 == NA_IP6 as i32 as u32 {
        addra = &mut a.ip6 as *mut [byte; 16] as *mut byte;
        addrb = &mut b.ip6 as *mut [byte; 16] as *mut byte;
        if netmask < 0 as i32 || netmask > 128 as i32 {
            netmask = 128 as i32
        }
    } else {
        Com_Printf(
            b"NET_CompareBaseAdr: bad address type\n\x00" as *const u8 as *const libc::c_char,
        );
        return qfalse;
    }
    curbyte = netmask >> 3 as i32;
    if curbyte != 0
        && crate::stdlib::memcmp(
            addra as *const libc::c_void,
            addrb as *const libc::c_void,
            curbyte as usize,
        ) != 0
    {
        return qfalse;
    }
    netmask &= 0x7 as i32;
    if netmask != 0 {
        cmpmask = (((1 as i32) << netmask) - 1 as i32) as byte;
        cmpmask = ((cmpmask as i32) << 8 as i32 - netmask) as byte;
        if *addra.offset(curbyte as isize) as i32 & cmpmask as i32
            == *addrb.offset(curbyte as isize) as i32 & cmpmask as i32
        {
            return qtrue;
        }
    } else {
        return qtrue;
    }
    return qfalse;
}
/*
===================
NET_CompareBaseAdr

Compares without the port
===================
*/
#[no_mangle]

pub unsafe extern "C" fn NET_CompareBaseAdr(mut a: netadr_t, mut b: netadr_t) -> qboolean {
    return NET_CompareBaseAdrMask(a, b, -(1 as i32));
}
#[no_mangle]

pub unsafe extern "C" fn NET_AdrToString(mut a: netadr_t) -> *const libc::c_char {
    static mut s: [libc::c_char; 48] = [0; 48];
    if a.type_0 as u32 == NA_LOOPBACK as i32 as u32 {
        Com_sprintf(
            s.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 48]>() as usize as i32,
            b"loopback\x00" as *const u8 as *const libc::c_char,
        );
    } else if a.type_0 as u32 == NA_BOT as i32 as u32 {
        Com_sprintf(
            s.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 48]>() as usize as i32,
            b"bot\x00" as *const u8 as *const libc::c_char,
        );
    } else if a.type_0 as u32 == NA_IP as i32 as u32 || a.type_0 as u32 == NA_IP6 as i32 as u32 {
        let mut sadr: sockaddr_storage = sockaddr_storage {
            ss_family: 0,
            __ss_padding: [0; 118],
            __ss_align: 0,
        };
        crate::stdlib::memset(
            &mut sadr as *mut sockaddr_storage as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<sockaddr_storage>() as usize,
        );
        NetadrToSockadr(&mut a, &mut sadr as *mut sockaddr_storage as *mut sockaddr);
        Sys_SockaddrToString(
            s.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 48]>() as usize as i32,
            &mut sadr as *mut sockaddr_storage as *mut sockaddr,
        );
    }
    return s.as_mut_ptr();
}
#[no_mangle]

pub unsafe extern "C" fn NET_AdrToStringwPort(mut a: netadr_t) -> *const libc::c_char {
    static mut s: [libc::c_char; 48] = [0; 48];
    if a.type_0 as u32 == NA_LOOPBACK as i32 as u32 {
        Com_sprintf(
            s.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 48]>() as usize as i32,
            b"loopback\x00" as *const u8 as *const libc::c_char,
        );
    } else if a.type_0 as u32 == NA_BOT as i32 as u32 {
        Com_sprintf(
            s.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 48]>() as usize as i32,
            b"bot\x00" as *const u8 as *const libc::c_char,
        );
    } else if a.type_0 as u32 == NA_IP as i32 as u32 {
        Com_sprintf(
            s.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 48]>() as usize as i32,
            b"%s:%hu\x00" as *const u8 as *const libc::c_char,
            NET_AdrToString(a),
            __bswap_16(a.port) as i32,
        );
    } else if a.type_0 as u32 == NA_IP6 as i32 as u32 {
        Com_sprintf(
            s.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 48]>() as usize as i32,
            b"[%s]:%hu\x00" as *const u8 as *const libc::c_char,
            NET_AdrToString(a),
            __bswap_16(a.port) as i32,
        );
    }
    return s.as_mut_ptr();
}
#[no_mangle]

pub unsafe extern "C" fn NET_CompareAdr(mut a: netadr_t, mut b: netadr_t) -> qboolean {
    if NET_CompareBaseAdr(a, b) as u64 == 0 {
        return qfalse;
    }
    if a.type_0 as u32 == NA_IP as i32 as u32 || a.type_0 as u32 == NA_IP6 as i32 as u32 {
        if a.port as i32 == b.port as i32 {
            return qtrue;
        }
    } else {
        return qtrue;
    }
    return qfalse;
}
#[no_mangle]

pub unsafe extern "C" fn NET_IsLocalAddress(mut adr: netadr_t) -> qboolean {
    return (adr.type_0 as u32 == NA_LOOPBACK as i32 as u32) as i32 as qboolean;
}
//=============================================================================
/*
==================
NET_GetPacket

Receive one packet
==================
*/
#[no_mangle]

pub unsafe extern "C" fn NET_GetPacket(
    mut net_from: *mut netadr_t,
    mut net_message: *mut msg_t,
    mut fdr: *mut fd_set,
) -> qboolean {
    let mut ret: i32 = 0;
    let mut from: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut fromlen: socklen_t = 0;
    let mut err: i32 = 0;
    if ip_socket != -(1 as i32)
        && (*fdr).__fds_bits
            [(ip_socket / (8 as i32 * ::std::mem::size_of::<__fd_mask>() as usize as i32)) as usize]
            & ((1 as usize)
                << ip_socket % (8 as i32 * ::std::mem::size_of::<__fd_mask>() as usize as i32))
                as __fd_mask
            != 0 as i32 as isize
    {
        fromlen = ::std::mem::size_of::<sockaddr_storage>() as usize as socklen_t;
        ret = crate::stdlib::recvfrom(
            ip_socket,
            (*net_message).data as *mut libc::c_void,
            (*net_message).maxsize as size_t,
            0 as i32,
            &mut from as *mut sockaddr_storage as *mut sockaddr,
            &mut fromlen,
        ) as i32;
        if ret == -(1 as i32) {
            err = *libc::__errno_location();
            if err != 11 as i32 && err != 104 as i32 {
                Com_Printf(
                    b"NET_GetPacket: %s\n\x00" as *const u8 as *const libc::c_char,
                    NET_ErrorString(),
                );
            }
        } else {
            crate::stdlib::memset(
                (*(&mut from as *mut sockaddr_storage as *mut sockaddr_in))
                    .sin_zero
                    .as_mut_ptr() as *mut libc::c_void,
                0 as i32,
                8 as i32 as usize,
            );
            if usingSocks as u32 != 0
                && crate::stdlib::memcmp(
                    &mut from as *mut sockaddr_storage as *const libc::c_void,
                    &mut socksRelayAddr as *mut sockaddr as *const libc::c_void,
                    fromlen as usize,
                ) == 0 as i32
            {
                if ret < 10 as i32
                    || *(*net_message).data.offset(0 as i32 as isize) as i32 != 0 as i32
                    || *(*net_message).data.offset(1 as i32 as isize) as i32 != 0 as i32
                    || *(*net_message).data.offset(2 as i32 as isize) as i32 != 0 as i32
                    || *(*net_message).data.offset(3 as i32 as isize) as i32 != 1 as i32
                {
                    return qfalse;
                }
                (*net_from).type_0 = NA_IP;
                (*net_from).ip[0 as i32 as usize] = *(*net_message).data.offset(4 as i32 as isize);
                (*net_from).ip[1 as i32 as usize] = *(*net_message).data.offset(5 as i32 as isize);
                (*net_from).ip[2 as i32 as usize] = *(*net_message).data.offset(6 as i32 as isize);
                (*net_from).ip[3 as i32 as usize] = *(*net_message).data.offset(7 as i32 as isize);
                (*net_from).port = *(&mut *(*net_message).data.offset(8 as i32 as isize)
                    as *mut byte as *mut i16) as u16;
                (*net_message).readcount = 10 as i32
            } else {
                SockadrToNetadr(
                    &mut from as *mut sockaddr_storage as *mut sockaddr,
                    net_from,
                );
                (*net_message).readcount = 0 as i32
            }
            if ret >= (*net_message).maxsize {
                Com_Printf(
                    b"Oversize packet from %s\n\x00" as *const u8 as *const libc::c_char,
                    NET_AdrToString(*net_from),
                );
                return qfalse;
            }
            (*net_message).cursize = ret;
            return qtrue;
        }
    }
    if ip6_socket != -(1 as i32)
        && (*fdr).__fds_bits[(ip6_socket
            / (8 as i32 * ::std::mem::size_of::<__fd_mask>() as usize as i32))
            as usize]
            & ((1 as usize)
                << ip6_socket % (8 as i32 * ::std::mem::size_of::<__fd_mask>() as usize as i32))
                as __fd_mask
            != 0 as i32 as isize
    {
        fromlen = ::std::mem::size_of::<sockaddr_storage>() as usize as socklen_t;
        ret = crate::stdlib::recvfrom(
            ip6_socket,
            (*net_message).data as *mut libc::c_void,
            (*net_message).maxsize as size_t,
            0 as i32,
            &mut from as *mut sockaddr_storage as *mut sockaddr,
            &mut fromlen,
        ) as i32;
        if ret == -(1 as i32) {
            err = *libc::__errno_location();
            if err != 11 as i32 && err != 104 as i32 {
                Com_Printf(
                    b"NET_GetPacket: %s\n\x00" as *const u8 as *const libc::c_char,
                    NET_ErrorString(),
                );
            }
        } else {
            SockadrToNetadr(
                &mut from as *mut sockaddr_storage as *mut sockaddr,
                net_from,
            );
            (*net_message).readcount = 0 as i32;
            if ret >= (*net_message).maxsize {
                Com_Printf(
                    b"Oversize packet from %s\n\x00" as *const u8 as *const libc::c_char,
                    NET_AdrToString(*net_from),
                );
                return qfalse;
            }
            (*net_message).cursize = ret;
            return qtrue;
        }
    }
    if multicast6_socket != -(1 as i32)
        && multicast6_socket != ip6_socket
        && (*fdr).__fds_bits[(multicast6_socket
            / (8 as i32 * ::std::mem::size_of::<__fd_mask>() as usize as i32))
            as usize]
            & ((1 as usize)
                << multicast6_socket
                    % (8 as i32 * ::std::mem::size_of::<__fd_mask>() as usize as i32))
                as __fd_mask
            != 0 as i32 as isize
    {
        fromlen = ::std::mem::size_of::<sockaddr_storage>() as usize as socklen_t;
        ret = crate::stdlib::recvfrom(
            multicast6_socket,
            (*net_message).data as *mut libc::c_void,
            (*net_message).maxsize as size_t,
            0 as i32,
            &mut from as *mut sockaddr_storage as *mut sockaddr,
            &mut fromlen,
        ) as i32;
        if ret == -(1 as i32) {
            err = *libc::__errno_location();
            if err != 11 as i32 && err != 104 as i32 {
                Com_Printf(
                    b"NET_GetPacket: %s\n\x00" as *const u8 as *const libc::c_char,
                    NET_ErrorString(),
                );
            }
        } else {
            SockadrToNetadr(
                &mut from as *mut sockaddr_storage as *mut sockaddr,
                net_from,
            );
            (*net_message).readcount = 0 as i32;
            if ret >= (*net_message).maxsize {
                Com_Printf(
                    b"Oversize packet from %s\n\x00" as *const u8 as *const libc::c_char,
                    NET_AdrToString(*net_from),
                );
                return qfalse;
            }
            (*net_message).cursize = ret;
            return qtrue;
        }
    }
    return qfalse;
}
//=============================================================================

static mut socksBuf: [libc::c_char; 4096] = [0; 4096];
/*
==================
Sys_SendPacket
==================
*/
#[no_mangle]

pub unsafe extern "C" fn Sys_SendPacket(
    mut length: i32,
    mut data: *const libc::c_void,
    mut to: netadr_t,
) {
    let mut ret: i32 = -(1 as i32); // reserved
    let mut addr: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    }; // fragment (not fragmented)
    if to.type_0 as u32 != NA_BROADCAST as i32 as u32
        && to.type_0 as u32 != NA_IP as i32 as u32
        && to.type_0 as u32 != NA_IP6 as i32 as u32
        && to.type_0 as u32 != NA_MULTICAST6 as i32 as u32
    {
        Com_Error(
            ERR_FATAL as i32,
            b"Sys_SendPacket: bad address type\x00" as *const u8 as *const libc::c_char,
        ); // address type: IPV4
    }
    if ip_socket == -(1 as i32) && to.type_0 as u32 == NA_IP as i32 as u32
        || ip_socket == -(1 as i32) && to.type_0 as u32 == NA_BROADCAST as i32 as u32
        || ip6_socket == -(1 as i32) && to.type_0 as u32 == NA_IP6 as i32 as u32
        || ip6_socket == -(1 as i32) && to.type_0 as u32 == NA_MULTICAST6 as i32 as u32
    {
        return;
    }
    if to.type_0 as u32 == NA_MULTICAST6 as i32 as u32 && (*net_enabled).integer & 0x8 as i32 != 0 {
        return;
    }
    crate::stdlib::memset(
        &mut addr as *mut sockaddr_storage as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<sockaddr_storage>() as usize,
    );
    NetadrToSockadr(&mut to, &mut addr as *mut sockaddr_storage as *mut sockaddr);
    if usingSocks as u32 != 0 && to.type_0 as u32 == NA_IP as i32 as u32 {
        socksBuf[0 as i32 as usize] = 0 as i32 as libc::c_char;
        socksBuf[1 as i32 as usize] = 0 as i32 as libc::c_char;
        socksBuf[2 as i32 as usize] = 0 as i32 as libc::c_char;
        socksBuf[3 as i32 as usize] = 1 as i32 as libc::c_char;
        *(&mut *socksBuf.as_mut_ptr().offset(4 as i32 as isize) as *mut libc::c_char as *mut i32) =
            (*(&mut addr as *mut sockaddr_storage as *mut sockaddr_in))
                .sin_addr
                .s_addr as i32;
        *(&mut *socksBuf.as_mut_ptr().offset(8 as i32 as isize) as *mut libc::c_char as *mut i16) =
            (*(&mut addr as *mut sockaddr_storage as *mut sockaddr_in)).sin_port as i16;
        crate::stdlib::memcpy(
            &mut *socksBuf.as_mut_ptr().offset(10 as i32 as isize) as *mut libc::c_char
                as *mut libc::c_void,
            data,
            length as usize,
        );
        ret = crate::stdlib::sendto(
            ip_socket,
            socksBuf.as_mut_ptr() as *const libc::c_void,
            (length + 10 as i32) as size_t,
            0 as i32,
            &mut socksRelayAddr,
            ::std::mem::size_of::<sockaddr>() as usize as socklen_t,
        ) as i32
    } else if addr.ss_family as i32 == 2 as i32 {
        ret = crate::stdlib::sendto(
            ip_socket,
            data,
            length as size_t,
            0 as i32,
            &mut addr as *mut sockaddr_storage as *mut sockaddr,
            ::std::mem::size_of::<sockaddr_in>() as usize as socklen_t,
        ) as i32
    } else if addr.ss_family as i32 == 10 as i32 {
        ret = crate::stdlib::sendto(
            ip6_socket,
            data,
            length as size_t,
            0 as i32,
            &mut addr as *mut sockaddr_storage as *mut sockaddr,
            ::std::mem::size_of::<sockaddr_in6>() as usize as socklen_t,
        ) as i32
    }
    if ret == -(1 as i32) {
        let mut err: i32 = *libc::__errno_location();
        // wouldblock is silent
        if err == 11 as i32 {
            return;
        }
        // some PPP links do not allow broadcasts and return an error
        if err == 99 as i32 && to.type_0 as u32 == NA_BROADCAST as i32 as u32 {
            return;
        }
        Com_Printf(
            b"Sys_SendPacket: %s\n\x00" as *const u8 as *const libc::c_char,
            NET_ErrorString(),
        );
    };
}
//=============================================================================
/*
==================
Sys_IsLANAddress

LAN clients will have their rate var ignored
==================
*/
#[no_mangle]

pub unsafe extern "C" fn Sys_IsLANAddress(mut adr: netadr_t) -> qboolean {
    let mut index: i32 = 0;
    let mut run: i32 = 0;
    let mut addrsize: i32 = 0;
    let mut differed: qboolean = qfalse;
    let mut compareadr: *mut byte = std::ptr::null_mut();
    let mut comparemask: *mut byte = std::ptr::null_mut();
    let mut compareip: *mut byte = std::ptr::null_mut();
    if adr.type_0 as u32 == NA_LOOPBACK as i32 as u32 {
        return qtrue;
    }
    if adr.type_0 as u32 == NA_IP as i32 as u32 {
        // RFC1918:
        // 10.0.0.0        -   10.255.255.255  (10/8 prefix)
        // 172.16.0.0      -   172.31.255.255  (172.16/12 prefix)
        // 192.168.0.0     -   192.168.255.255 (192.168/16 prefix)
        if adr.ip[0 as i32 as usize] as i32 == 10 as i32 {
            return qtrue;
        }
        if adr.ip[0 as i32 as usize] as i32 == 172 as i32
            && adr.ip[1 as i32 as usize] as i32 & 0xf0 as i32 == 16 as i32
        {
            return qtrue;
        }
        if adr.ip[0 as i32 as usize] as i32 == 192 as i32
            && adr.ip[1 as i32 as usize] as i32 == 168 as i32
        {
            return qtrue;
        }
        if adr.ip[0 as i32 as usize] as i32 == 127 as i32 {
            return qtrue;
        }
    } else if adr.type_0 as u32 == NA_IP6 as i32 as u32 {
        if adr.ip6[0 as i32 as usize] as i32 == 0xfe as i32
            && adr.ip6[1 as i32 as usize] as i32 & 0xc0 as i32 == 0x80 as i32
        {
            return qtrue;
        }
        if adr.ip6[0 as i32 as usize] as i32 & 0xfe as i32 == 0xfc as i32 {
            return qtrue;
        }
    }
    // Now compare against the networks this computer is member of.
    index = 0 as i32;
    while index < numIP {
        if localIP[index as usize].type_0 as u32 == adr.type_0 as u32 {
            if adr.type_0 as u32 == NA_IP as i32 as u32 {
                compareip = &mut (*(&mut (*localIP.as_mut_ptr().offset(index as isize)).addr
                    as *mut sockaddr_storage
                    as *mut sockaddr_in))
                    .sin_addr
                    .s_addr as *mut in_addr_t as *mut byte;
                comparemask = &mut (*(&mut (*localIP.as_mut_ptr().offset(index as isize)).netmask
                    as *mut sockaddr_storage
                    as *mut sockaddr_in))
                    .sin_addr
                    .s_addr as *mut in_addr_t as *mut byte;
                compareadr = adr.ip.as_mut_ptr();
                addrsize = ::std::mem::size_of::<[byte; 4]>() as usize as i32
            } else {
                // TODO? should we check the scope_id here?
                compareip = &mut (*(&mut (*localIP.as_mut_ptr().offset(index as isize)).addr
                    as *mut sockaddr_storage
                    as *mut sockaddr_in6))
                    .sin6_addr as *mut in6_addr as *mut byte;
                comparemask = &mut (*(&mut (*localIP.as_mut_ptr().offset(index as isize)).netmask
                    as *mut sockaddr_storage
                    as *mut sockaddr_in6))
                    .sin6_addr as *mut in6_addr as *mut byte;
                compareadr = adr.ip6.as_mut_ptr();
                addrsize = ::std::mem::size_of::<[byte; 16]>() as usize as i32
            }
            differed = qfalse;
            run = 0 as i32;
            while run < addrsize {
                if *compareip.offset(run as isize) as i32 & *comparemask.offset(run as isize) as i32
                    != *compareadr.offset(run as isize) as i32
                        & *comparemask.offset(run as isize) as i32
                {
                    differed = qtrue;
                    break;
                } else {
                    run += 1
                }
            }
            if differed as u64 == 0 {
                return qtrue;
            }
        }
        index += 1
    }
    return qfalse;
}
/*
==============================================================

CLIENT / SERVER SYSTEMS

==============================================================
*/
//
// client interface
//
// the keyboard binding interface must be setup before execing
// config files, but the rest of client startup will happen later
// char events are for field typing, not game control
// do a screen update before starting to load a map
// when the server is going to load a new map, the entire hunk
// will be cleared, so the client must shutdown cgame, ui, and
// the renderer
// adds the current command line as a clc_clientCommand to the client message.
// things like godmode, noclip, etc, are commands directed to the server,
// so when they are typed in at the console, they will need to be forwarded.
// bring up the "need a cd to play" dialog
// dump all memory on an error
// shutdown client
// initialize renderer interface
// start all the client stuff using the hunk
// Restart sound subsystem
// for keyname autocompletion
// for writing the config files
// call before filesystem access
// FIXME: move logging to common?
// AVI files have the start of pixel lines 4 byte-aligned
//
// server interface
//
//
// UI interface
//
//
// input interface
//
/*
==============================================================

NON-PORTABLE SYSTEM SERVICES

==============================================================
*/
// general development dll loading for virtual machine testing
// note that this isn't journaled...
// Sys_Milliseconds should only be used for profiling purposes,
// any game related timing information should come from event timestamps
// the system console is shown when a dedicated server is running
//Does NOT parse port numbers, only base addresses.
/*
==================
Sys_ShowIP
==================
*/
#[no_mangle]

pub unsafe extern "C" fn Sys_ShowIP() {
    let mut i: i32 = 0;
    let mut addrbuf: [libc::c_char; 48] = [0; 48];
    i = 0 as i32;
    while i < numIP {
        Sys_SockaddrToString(
            addrbuf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 48]>() as usize as i32,
            &mut (*localIP.as_mut_ptr().offset(i as isize)).addr as *mut sockaddr_storage
                as *mut sockaddr,
        );
        if localIP[i as usize].type_0 as u32 == NA_IP as i32 as u32 {
            Com_Printf(
                b"IP: %s\n\x00" as *const u8 as *const libc::c_char,
                addrbuf.as_mut_ptr(),
            );
        } else if localIP[i as usize].type_0 as u32 == NA_IP6 as i32 as u32 {
            Com_Printf(
                b"IP6: %s\n\x00" as *const u8 as *const libc::c_char,
                addrbuf.as_mut_ptr(),
            );
        }
        i += 1
    }
}
//=============================================================================
/*
====================
NET_IPSocket
====================
*/
#[no_mangle]

pub unsafe extern "C" fn NET_IPSocket(
    mut net_interface: *mut libc::c_char,
    mut port: i32,
    mut err: *mut i32,
) -> SOCKET {
    let mut newsocket: SOCKET = 0;
    let mut address: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut _true: ioctlarg_t = 1 as i32;
    let mut i: i32 = 1 as i32;
    *err = 0 as i32;
    if !net_interface.is_null() {
        Com_Printf(
            b"Opening IP socket: %s:%i\n\x00" as *const u8 as *const libc::c_char,
            net_interface,
            port,
        );
    } else {
        Com_Printf(
            b"Opening IP socket: 0.0.0.0:%i\n\x00" as *const u8 as *const libc::c_char,
            port,
        );
    }
    newsocket = libc::socket(2 as i32, SOCK_DGRAM as i32, IPPROTO_UDP as i32);
    if newsocket == -(1 as i32) {
        *err = *libc::__errno_location();
        Com_Printf(
            b"WARNING: NET_IPSocket: socket: %s\n\x00" as *const u8 as *const libc::c_char,
            NET_ErrorString(),
        );
        return newsocket;
    }
    // make it non-blocking
    if libc::ioctl(newsocket, 0x5421u64, &mut _true as *mut ioctlarg_t) == -(1 as i32) {
        Com_Printf(
            b"WARNING: NET_IPSocket: ioctl FIONBIO: %s\n\x00" as *const u8 as *const libc::c_char,
            NET_ErrorString(),
        );
        *err = *libc::__errno_location();
        libc::close(newsocket);
        return -(1 as i32);
    }
    // make it broadcast capable
    if libc::setsockopt(
        newsocket,
        1 as i32,
        6 as i32,
        &mut i as *mut i32 as *mut libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<i32>() as usize as socklen_t,
    ) == -(1 as i32)
    {
        Com_Printf(
            b"WARNING: NET_IPSocket: setsockopt SO_BROADCAST: %s\n\x00" as *const u8
                as *const libc::c_char,
            NET_ErrorString(),
        );
    }
    if net_interface.is_null() || *net_interface.offset(0 as i32 as isize) == 0 {
        address.sin_family = 2 as i32 as sa_family_t;
        address.sin_addr.s_addr = 0 as i32 as in_addr_t
    } else if Sys_StringToSockaddr(
        net_interface,
        &mut address as *mut sockaddr_in as *mut sockaddr,
        ::std::mem::size_of::<sockaddr_in>() as usize as i32,
        2 as i32 as sa_family_t,
    ) as u64
        == 0
    {
        libc::close(newsocket);
        return -(1 as i32);
    }
    if port == -(1 as i32) {
        address.sin_port = 0 as i32 as in_port_t
    } else {
        address.sin_port = __bswap_16(port as i16 as __uint16_t)
    }
    if libc::bind(
        newsocket,
        &mut address as *mut sockaddr_in as *mut libc::c_void as *const sockaddr as *const sockaddr,
        ::std::mem::size_of::<sockaddr_in>() as usize as socklen_t,
    ) == -(1 as i32)
    {
        Com_Printf(
            b"WARNING: NET_IPSocket: bind: %s\n\x00" as *const u8 as *const libc::c_char,
            NET_ErrorString(),
        );
        *err = *libc::__errno_location();
        libc::close(newsocket);
        return -(1 as i32);
    }
    return newsocket;
}
/*
====================
NET_IP6Socket
====================
*/
#[no_mangle]

pub unsafe extern "C" fn NET_IP6Socket(
    mut net_interface: *mut libc::c_char,
    mut port: i32,
    mut bindto: *mut sockaddr_in6,
    mut err: *mut i32,
) -> SOCKET {
    let mut newsocket: SOCKET = 0;
    let mut address: sockaddr_in6 = sockaddr_in6 {
        sin6_family: 0,
        sin6_port: 0,
        sin6_flowinfo: 0,
        sin6_addr: in6_addr {
            __in6_u: C2RustUnnamed_129 {
                __u6_addr8: [0; 16],
            },
        },
        sin6_scope_id: 0,
    };
    let mut _true: ioctlarg_t = 1 as i32;
    *err = 0 as i32;
    if !net_interface.is_null() {
        // Print the name in brackets if there is a colon:
        if Q_CountChar(net_interface, ':' as i32 as libc::c_char) != 0 {
            Com_Printf(
                b"Opening IP6 socket: [%s]:%i\n\x00" as *const u8 as *const libc::c_char,
                net_interface,
                port,
            );
        } else {
            Com_Printf(
                b"Opening IP6 socket: %s:%i\n\x00" as *const u8 as *const libc::c_char,
                net_interface,
                port,
            );
        }
    } else {
        Com_Printf(
            b"Opening IP6 socket: [::]:%i\n\x00" as *const u8 as *const libc::c_char,
            port,
        );
    }
    newsocket = libc::socket(10 as i32, SOCK_DGRAM as i32, IPPROTO_UDP as i32);
    if newsocket == -(1 as i32) {
        *err = *libc::__errno_location();
        Com_Printf(
            b"WARNING: NET_IP6Socket: socket: %s\n\x00" as *const u8 as *const libc::c_char,
            NET_ErrorString(),
        );
        return newsocket;
    }
    // make it non-blocking
    if libc::ioctl(newsocket, 0x5421u64, &mut _true as *mut ioctlarg_t) == -(1 as i32) {
        Com_Printf(
            b"WARNING: NET_IP6Socket: ioctl FIONBIO: %s\n\x00" as *const u8 as *const libc::c_char,
            NET_ErrorString(),
        );
        *err = *libc::__errno_location();
        libc::close(newsocket);
        return -(1 as i32);
    }
    let mut i: i32 = 1 as i32;
    // ipv4 addresses should not be allowed to connect via this socket.
    if libc::setsockopt(
        newsocket,
        IPPROTO_IPV6 as i32,
        26 as i32,
        &mut i as *mut i32 as *mut libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<i32>() as usize as socklen_t,
    ) == -(1 as i32)
    {
        // win32 systems don't seem to support this anyways.
        Com_DPrintf(
            b"WARNING: NET_IP6Socket: setsockopt IPV6_V6ONLY: %s\n\x00" as *const u8
                as *const libc::c_char,
            NET_ErrorString(),
        );
    }
    if net_interface.is_null() || *net_interface.offset(0 as i32 as isize) == 0 {
        address.sin6_family = 10 as i32 as sa_family_t;
        address.sin6_addr = in6addr_any
    } else if Sys_StringToSockaddr(
        net_interface,
        &mut address as *mut sockaddr_in6 as *mut sockaddr,
        ::std::mem::size_of::<sockaddr_in6>() as usize as i32,
        10 as i32 as sa_family_t,
    ) as u64
        == 0
    {
        libc::close(newsocket);
        return -(1 as i32);
    }
    if port == -(1 as i32) {
        address.sin6_port = 0 as i32 as in_port_t
    } else {
        address.sin6_port = __bswap_16(port as i16 as __uint16_t)
    }
    if libc::bind(
        newsocket,
        &mut address as *mut sockaddr_in6 as *mut libc::c_void as *const sockaddr
            as *const sockaddr,
        ::std::mem::size_of::<sockaddr_in6>() as usize as socklen_t,
    ) == -(1 as i32)
    {
        Com_Printf(
            b"WARNING: NET_IP6Socket: bind: %s\n\x00" as *const u8 as *const libc::c_char,
            NET_ErrorString(),
        );
        *err = *libc::__errno_location();
        libc::close(newsocket);
        return -(1 as i32);
    }
    if !bindto.is_null() {
        *bindto = address
    }
    return newsocket;
}
/*
====================
NET_SetMulticast
Set the current multicast group
====================
*/
#[no_mangle]

pub unsafe extern "C" fn NET_SetMulticast6() {
    let mut addr: sockaddr_in6 = sockaddr_in6 {
        sin6_family: 0,
        sin6_port: 0,
        sin6_flowinfo: 0,
        sin6_addr: in6_addr {
            __in6_u: C2RustUnnamed_129 {
                __u6_addr8: [0; 16],
            },
        },
        sin6_scope_id: 0,
    };
    if *(*net_mcast6addr).string == 0
        || Sys_StringToSockaddr(
            (*net_mcast6addr).string,
            &mut addr as *mut sockaddr_in6 as *mut sockaddr,
            ::std::mem::size_of::<sockaddr_in6>() as usize as i32,
            10 as i32 as sa_family_t,
        ) as u64
            == 0
    {
        Com_Printf(b"WARNING: NET_JoinMulticast6: Incorrect multicast address given, please set cvar %s to a sane value.\n\x00"
                       as *const u8 as *const libc::c_char,
                   (*net_mcast6addr).name);
        Cvar_SetValue(
            (*net_enabled).name,
            ((*net_enabled).integer | 0x8 as i32) as f32,
        );
        return;
    }
    crate::stdlib::memcpy(
        &mut curgroup.ipv6mr_multiaddr as *mut in6_addr as *mut libc::c_void,
        &mut addr.sin6_addr as *mut in6_addr as *const libc::c_void,
        ::std::mem::size_of::<in6_addr>() as usize,
    );
    if *(*net_mcast6iface).string != 0 {
        curgroup.ipv6mr_interface = if_nametoindex((*net_mcast6iface).string)
    } else {
        curgroup.ipv6mr_interface = 0 as i32 as u32
    };
}
/*
====================
NET_JoinMulticast
Join an ipv6 multicast group
====================
*/
#[no_mangle]

pub unsafe extern "C" fn NET_JoinMulticast6() {
    let mut err: i32 = 0;
    if ip6_socket == -(1 as i32)
        || multicast6_socket != -(1 as i32)
        || (*net_enabled).integer & 0x8 as i32 != 0
    {
        return;
    }
    if *(&mut boundto.sin6_addr as *mut in6_addr as *const uint8_t).offset(0 as i32 as isize) as i32
        == 0xff as i32
        || ({
            let mut __a: *const in6_addr =
                &mut boundto.sin6_addr as *mut in6_addr as *const in6_addr;
            ((*__a).__in6_u.__u6_addr32[0 as i32 as usize] == 0 as i32 as u32
                && (*__a).__in6_u.__u6_addr32[1 as i32 as usize] == 0 as i32 as u32
                && (*__a).__in6_u.__u6_addr32[2 as i32 as usize] == 0 as i32 as u32
                && (*__a).__in6_u.__u6_addr32[3 as i32 as usize] == 0 as i32 as u32)
                as i32
        }) != 0
    {
        // The way the socket was bound does not prohibit receiving multi-cast packets. So we don't need to open a new one.
        multicast6_socket = ip6_socket
    } else {
        multicast6_socket = NET_IP6Socket(
            (*net_mcast6addr).string,
            __bswap_16(boundto.sin6_port) as i32,
            std::ptr::null_mut(),
            &mut err,
        );
        if multicast6_socket == -(1 as i32) {
            // If the OS does not support binding to multicast addresses, like WinXP, at least try with the normal file descriptor.
            multicast6_socket = ip6_socket
        }
    }
    if curgroup.ipv6mr_interface != 0 {
        if libc::setsockopt(
            multicast6_socket,
            IPPROTO_IPV6 as i32,
            17 as i32,
            &mut curgroup.ipv6mr_interface as *mut u32 as *mut libc::c_char as *const libc::c_void,
            ::std::mem::size_of::<u32>() as usize as socklen_t,
        ) < 0 as i32
        {
            Com_Printf(
                b"NET_JoinMulticast6: Couldn\'t set scope on multicast socket: %s\n\x00"
                    as *const u8 as *const libc::c_char,
                NET_ErrorString(),
            );
            if multicast6_socket != ip6_socket {
                libc::close(multicast6_socket);
                multicast6_socket = -(1 as i32);
                return;
            }
        }
    }
    if libc::setsockopt(
        multicast6_socket,
        IPPROTO_IPV6 as i32,
        20 as i32,
        &mut curgroup as *mut ipv6_mreq as *mut libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<ipv6_mreq>() as usize as socklen_t,
    ) != 0
    {
        Com_Printf(
            b"NET_JoinMulticast6: Couldn\'t join multicast group: %s\n\x00" as *const u8
                as *const libc::c_char,
            NET_ErrorString(),
        );
        if multicast6_socket != ip6_socket {
            libc::close(multicast6_socket);
            multicast6_socket = -(1 as i32);
            return;
        }
    };
}
#[no_mangle]

pub unsafe extern "C" fn NET_LeaveMulticast6() {
    if multicast6_socket != -(1 as i32) {
        if multicast6_socket != ip6_socket {
            libc::close(multicast6_socket);
        } else {
            libc::setsockopt(
                multicast6_socket,
                IPPROTO_IPV6 as i32,
                21 as i32,
                &mut curgroup as *mut ipv6_mreq as *mut libc::c_char as *const libc::c_void,
                ::std::mem::size_of::<ipv6_mreq>() as usize as socklen_t,
            );
        }
        multicast6_socket = -(1 as i32)
    };
}
/*
====================
NET_OpenSocks
====================
*/
#[no_mangle]

pub unsafe extern "C" fn NET_OpenSocks(mut port: i32) {
    let mut address: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut h: *mut hostent = std::ptr::null_mut();
    let mut len: i32 = 0;
    let mut rfc1929: qboolean = qfalse;
    let mut buf: [u8; 64] = [0; 64];
    usingSocks = qfalse;
    Com_Printf(b"Opening connection to SOCKS server.\n\x00" as *const u8 as *const libc::c_char);
    socks_socket = libc::socket(2 as i32, SOCK_STREAM as i32, IPPROTO_TCP as i32);
    if socks_socket == -(1 as i32) {
        Com_Printf(
            b"WARNING: NET_OpenSocks: socket: %s\n\x00" as *const u8 as *const libc::c_char,
            NET_ErrorString(),
        );
        return;
    }
    h = gethostbyname((*net_socksServer).string);
    if h.is_null() {
        Com_Printf(
            b"WARNING: NET_OpenSocks: gethostbyname: %s\n\x00" as *const u8 as *const libc::c_char,
            NET_ErrorString(),
        );
        return;
    }
    if (*h).h_addrtype != 2 as i32 {
        Com_Printf(
            b"WARNING: NET_OpenSocks: gethostbyname: address type was not AF_INET\n\x00"
                as *const u8 as *const libc::c_char,
        );
        return;
    }
    address.sin_family = 2 as i32 as sa_family_t;
    address.sin_addr.s_addr =
        *(*(*h).h_addr_list.offset(0 as i32 as isize) as *mut i32) as in_addr_t;
    address.sin_port = __bswap_16((*net_socksPort).integer as i16 as __uint16_t);
    if libc::connect(
        socks_socket,
        &mut address as *mut sockaddr_in as *mut sockaddr as *const sockaddr,
        ::std::mem::size_of::<sockaddr_in>() as usize as socklen_t,
    ) == -(1 as i32)
    {
        Com_Printf(
            b"NET_OpenSocks: connect: %s\n\x00" as *const u8 as *const libc::c_char,
            NET_ErrorString(),
        );
        return;
    }
    // send socks authentication handshake
    if *(*net_socksUsername).string as i32 != 0 || *(*net_socksPassword).string as i32 != 0 {
        rfc1929 = qtrue
    } else {
        rfc1929 = qfalse
    } // SOCKS version
    buf[0 as i32 as usize] = 5 as i32 as u8;
    // method count
    if rfc1929 as u64 != 0 {
        buf[1 as i32 as usize] = 2 as i32 as u8; // method #1 - method id #00: no authentication
        len = 4 as i32
    } else {
        buf[1 as i32 as usize] = 1 as i32 as u8;
        len = 3 as i32
    }
    buf[2 as i32 as usize] = 0 as i32 as u8;
    if rfc1929 as u64 != 0 {
        buf[2 as i32 as usize] = 2 as i32 as u8
        // method #2 - method id #02: username/password
    }
    if crate::stdlib::send(
        socks_socket,
        buf.as_mut_ptr() as *mut libc::c_void,
        len as size_t,
        0 as i32,
    ) == -(1 as i32) as isize
    {
        Com_Printf(
            b"NET_OpenSocks: send: %s\n\x00" as *const u8 as *const libc::c_char,
            NET_ErrorString(),
        );
        return;
    }
    // get the response
    len = crate::stdlib::recv(
        socks_socket,
        buf.as_mut_ptr() as *mut libc::c_void,
        64 as i32 as size_t,
        0 as i32,
    ) as i32;
    if len == -(1 as i32) {
        Com_Printf(
            b"NET_OpenSocks: recv: %s\n\x00" as *const u8 as *const libc::c_char,
            NET_ErrorString(),
        );
        return;
    }
    if len != 2 as i32 || buf[0 as i32 as usize] as i32 != 5 as i32 {
        Com_Printf(b"NET_OpenSocks: bad response\n\x00" as *const u8 as *const libc::c_char);
        return;
    }
    match buf[1 as i32 as usize] as i32 {
        0 => {}
        2 => {}
        _ => {
            Com_Printf(b"NET_OpenSocks: request denied\n\x00" as *const u8 as *const libc::c_char);
            return;
        }
    }
    // do username/password authentication if needed
    if buf[1 as i32 as usize] as i32 == 2 as i32 {
        let mut ulen: i32 = 0;
        let mut plen: i32 = 0;
        // build the request
        ulen = crate::stdlib::strlen((*net_socksUsername).string) as i32; // username/password authentication version
        plen = crate::stdlib::strlen((*net_socksPassword).string) as i32;
        buf[0 as i32 as usize] = 1 as i32 as u8;
        buf[1 as i32 as usize] = ulen as u8;
        if ulen != 0 {
            crate::stdlib::memcpy(
                &mut *buf.as_mut_ptr().offset(2 as i32 as isize) as *mut u8 as *mut libc::c_void,
                (*net_socksUsername).string as *const libc::c_void,
                ulen as usize,
            );
        }
        buf[(2 as i32 + ulen) as usize] = plen as u8;
        if plen != 0 {
            crate::stdlib::memcpy(
                &mut *buf.as_mut_ptr().offset((3 as i32 + ulen) as isize) as *mut u8
                    as *mut libc::c_void,
                (*net_socksPassword).string as *const libc::c_void,
                plen as usize,
            );
        }
        // send it
        if crate::stdlib::send(
            socks_socket,
            buf.as_mut_ptr() as *mut libc::c_void,
            (3 as i32 + ulen + plen) as size_t,
            0 as i32,
        ) == -(1 as i32) as isize
        {
            Com_Printf(
                b"NET_OpenSocks: send: %s\n\x00" as *const u8 as *const libc::c_char,
                NET_ErrorString(),
            );
            return;
        }
        // get the response
        len = crate::stdlib::recv(
            socks_socket,
            buf.as_mut_ptr() as *mut libc::c_void,
            64 as i32 as size_t,
            0 as i32,
        ) as i32;
        if len == -(1 as i32) {
            Com_Printf(
                b"NET_OpenSocks: recv: %s\n\x00" as *const u8 as *const libc::c_char,
                NET_ErrorString(),
            );
            return;
        }
        if len != 2 as i32 || buf[0 as i32 as usize] as i32 != 1 as i32 {
            Com_Printf(b"NET_OpenSocks: bad response\n\x00" as *const u8 as *const libc::c_char);
            return;
        }
        if buf[1 as i32 as usize] as i32 != 0 as i32 {
            Com_Printf(
                b"NET_OpenSocks: authentication failed\n\x00" as *const u8 as *const libc::c_char,
            );
            return;
        }
    }
    // send the UDP associate request
    buf[0 as i32 as usize] = 5 as i32 as u8; // SOCKS version
    buf[1 as i32 as usize] = 3 as i32 as u8; // command: UDP associate
    buf[2 as i32 as usize] = 0 as i32 as u8; // reserved
    buf[3 as i32 as usize] = 1 as i32 as u8; // address type: IPV4
    *(&mut *buf.as_mut_ptr().offset(4 as i32 as isize) as *mut u8 as *mut i32) =
        0 as i32 as in_addr_t as i32; // port
    *(&mut *buf.as_mut_ptr().offset(8 as i32 as isize) as *mut u8 as *mut i16) =
        __bswap_16(port as i16 as __uint16_t) as i16;
    if crate::stdlib::send(
        socks_socket,
        buf.as_mut_ptr() as *mut libc::c_void,
        10 as i32 as size_t,
        0 as i32,
    ) == -(1 as i32) as isize
    {
        Com_Printf(
            b"NET_OpenSocks: send: %s\n\x00" as *const u8 as *const libc::c_char,
            NET_ErrorString(),
        );
        return;
    }
    // get the response
    len = crate::stdlib::recv(
        socks_socket,
        buf.as_mut_ptr() as *mut libc::c_void,
        64 as i32 as size_t,
        0 as i32,
    ) as i32;
    if len == -(1 as i32) {
        Com_Printf(
            b"NET_OpenSocks: recv: %s\n\x00" as *const u8 as *const libc::c_char,
            NET_ErrorString(),
        );
        return;
    }
    if len < 2 as i32 || buf[0 as i32 as usize] as i32 != 5 as i32 {
        Com_Printf(b"NET_OpenSocks: bad response\n\x00" as *const u8 as *const libc::c_char);
        return;
    }
    // check completion code
    if buf[1 as i32 as usize] as i32 != 0 as i32 {
        Com_Printf(
            b"NET_OpenSocks: request denied: %i\n\x00" as *const u8 as *const libc::c_char,
            buf[1 as i32 as usize] as i32,
        );
        return;
    }
    if buf[3 as i32 as usize] as i32 != 1 as i32 {
        Com_Printf(
            b"NET_OpenSocks: relay address is not IPV4: %i\n\x00" as *const u8
                as *const libc::c_char,
            buf[3 as i32 as usize] as i32,
        );
        return;
    }
    (*(&mut socksRelayAddr as *mut sockaddr as *mut sockaddr_in)).sin_family =
        2 as i32 as sa_family_t;
    (*(&mut socksRelayAddr as *mut sockaddr as *mut sockaddr_in))
        .sin_addr
        .s_addr =
        *(&mut *buf.as_mut_ptr().offset(4 as i32 as isize) as *mut u8 as *mut i32) as in_addr_t;
    (*(&mut socksRelayAddr as *mut sockaddr as *mut sockaddr_in)).sin_port =
        *(&mut *buf.as_mut_ptr().offset(8 as i32 as isize) as *mut u8 as *mut i16) as in_port_t;
    crate::stdlib::memset(
        (*(&mut socksRelayAddr as *mut sockaddr as *mut sockaddr_in))
            .sin_zero
            .as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        8 as i32 as usize,
    );
    usingSocks = qtrue;
}
/*
=====================
NET_AddLocalAddress
=====================
*/

unsafe extern "C" fn NET_AddLocalAddress(
    mut ifname: *mut libc::c_char,
    mut addr: *mut sockaddr,
    mut netmask: *mut sockaddr,
) {
    let mut addrlen: i32 = 0;
    let mut family: sa_family_t = 0;
    // only add addresses that have all required info.
    if addr.is_null() || netmask.is_null() || ifname.is_null() {
        return;
    }
    family = (*addr).sa_family;
    if numIP < 32 as i32 {
        if family as i32 == 2 as i32 {
            addrlen = ::std::mem::size_of::<sockaddr_in>() as usize as i32;
            localIP[numIP as usize].type_0 = NA_IP
        } else if family as i32 == 10 as i32 {
            addrlen = ::std::mem::size_of::<sockaddr_in6>() as usize as i32;
            localIP[numIP as usize].type_0 = NA_IP6
        } else {
            return;
        }
        Q_strncpyz(
            localIP[numIP as usize].ifname.as_mut_ptr(),
            ifname,
            ::std::mem::size_of::<[libc::c_char; 16]>() as usize as i32,
        );
        localIP[numIP as usize].family = family;
        crate::stdlib::memcpy(
            &mut (*localIP.as_mut_ptr().offset(numIP as isize)).addr as *mut sockaddr_storage
                as *mut libc::c_void,
            addr as *const libc::c_void,
            addrlen as usize,
        );
        crate::stdlib::memcpy(
            &mut (*localIP.as_mut_ptr().offset(numIP as isize)).netmask as *mut sockaddr_storage
                as *mut libc::c_void,
            netmask as *const libc::c_void,
            addrlen as usize,
        );
        numIP += 1
    };
}

unsafe extern "C" fn NET_GetLocalAddress() {
    let mut ifap: *mut ifaddrs = std::ptr::null_mut();
    let mut search: *mut ifaddrs = std::ptr::null_mut();
    numIP = 0 as i32;
    if getifaddrs(&mut ifap) != 0 {
        Com_Printf(
            b"NET_GetLocalAddress: Unable to get list of network interfaces: %s\n\x00" as *const u8
                as *const libc::c_char,
            NET_ErrorString(),
        );
    } else {
        search = ifap;
        while !search.is_null() {
            // Only add interfaces that are up.
            if (*ifap).ifa_flags & IFF_UP as i32 as u32 != 0 {
                NET_AddLocalAddress(
                    (*search).ifa_name,
                    (*search).ifa_addr,
                    (*search).ifa_netmask,
                );
            }
            search = (*search).ifa_next
        }
        freeifaddrs(ifap);
        Sys_ShowIP();
    };
}
/*
====================
NET_OpenIP
====================
*/
#[no_mangle]

pub unsafe extern "C" fn NET_OpenIP() {
    let mut i: i32 = 0;
    let mut err: i32 = 0;
    let mut port: i32 = 0;
    let mut port6: i32 = 0;
    port = (*net_port).integer;
    port6 = (*net_port6).integer;
    NET_GetLocalAddress();
    // automatically scan for a valid port, so multiple
    // dedicated servers can be started without requiring
    // a different net_port for each one
    if (*net_enabled).integer & 0x2 as i32 != 0 {
        i = 0 as i32;
        while i < 10 as i32 {
            ip6_socket = NET_IP6Socket((*net_ip6).string, port6 + i, &mut boundto, &mut err);
            if ip6_socket != -(1 as i32) {
                Cvar_SetValue(
                    b"net_port6\x00" as *const u8 as *const libc::c_char,
                    (port6 + i) as f32,
                );
                break;
            } else {
                if err == 97 as i32 {
                    break;
                }
                i += 1
            }
        }
        if ip6_socket == -(1 as i32) {
            Com_Printf(
                b"WARNING: Couldn\'t bind to a v6 ip address.\n\x00" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    if (*net_enabled).integer & 0x1 as i32 != 0 {
        i = 0 as i32;
        while i < 10 as i32 {
            ip_socket = NET_IPSocket((*net_ip).string, port + i, &mut err);
            if ip_socket != -(1 as i32) {
                Cvar_SetValue(
                    b"net_port\x00" as *const u8 as *const libc::c_char,
                    (port + i) as f32,
                );
                if (*net_socksEnabled).integer != 0 {
                    NET_OpenSocks(port + i);
                }
                break;
            } else {
                if err == 97 as i32 {
                    break;
                }
                i += 1
            }
        }
        if ip_socket == -(1 as i32) {
            Com_Printf(
                b"WARNING: Couldn\'t bind to a v4 ip address.\n\x00" as *const u8
                    as *const libc::c_char,
            );
        }
    };
}
//===================================================================
/*
====================
NET_GetCvars
====================
*/

unsafe extern "C" fn NET_GetCvars() -> qboolean {
    let mut modified: i32 = 0;
    /* End users have it enabled so they can connect to ipv6-only hosts, but ipv4 will be
     * used if available due to ping */
    net_enabled = Cvar_Get(
        b"net_enabled\x00" as *const u8 as *const libc::c_char,
        b"3\x00" as *const u8 as *const libc::c_char,
        0x20 as i32 | 0x1 as i32,
    ) as *mut cvar_s;
    modified = (*net_enabled).modified as i32;
    (*net_enabled).modified = qfalse;
    net_ip = Cvar_Get(
        b"net_ip\x00" as *const u8 as *const libc::c_char,
        b"0.0.0.0\x00" as *const u8 as *const libc::c_char,
        0x20 as i32,
    ) as *mut cvar_s;
    modified = (modified as u32).wrapping_add((*net_ip).modified as u32) as i32;
    (*net_ip).modified = qfalse;
    net_ip6 = Cvar_Get(
        b"net_ip6\x00" as *const u8 as *const libc::c_char,
        b"::\x00" as *const u8 as *const libc::c_char,
        0x20 as i32,
    ) as *mut cvar_s;
    modified = (modified as u32).wrapping_add((*net_ip6).modified as u32) as i32;
    (*net_ip6).modified = qfalse;
    net_port = Cvar_Get(
        b"net_port\x00" as *const u8 as *const libc::c_char,
        va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            27960 as i32,
        ),
        0x20 as i32,
    ) as *mut cvar_s;
    modified = (modified as u32).wrapping_add((*net_port).modified as u32) as i32;
    (*net_port).modified = qfalse;
    net_port6 = Cvar_Get(
        b"net_port6\x00" as *const u8 as *const libc::c_char,
        va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            27960 as i32,
        ),
        0x20 as i32,
    ) as *mut cvar_s;
    modified = (modified as u32).wrapping_add((*net_port6).modified as u32) as i32;
    (*net_port6).modified = qfalse;
    // Some cvars for configuring multicast options which facilitates scanning for servers on local subnets.
    net_mcast6addr = Cvar_Get(
        b"net_mcast6addr\x00" as *const u8 as *const libc::c_char,
        b"ff04::696f:7175:616b:6533\x00" as *const u8 as *const libc::c_char,
        0x20 as i32 | 0x1 as i32,
    ) as *mut cvar_s;
    modified = (modified as u32).wrapping_add((*net_mcast6addr).modified as u32) as i32;
    (*net_mcast6addr).modified = qfalse;
    net_mcast6iface = Cvar_Get(
        b"net_mcast6iface\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x20 as i32 | 0x1 as i32,
    ) as *mut cvar_s;
    modified = (modified as u32).wrapping_add((*net_mcast6iface).modified as u32) as i32;
    (*net_mcast6iface).modified = qfalse;
    net_socksEnabled = Cvar_Get(
        b"net_socksEnabled\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x20 as i32 | 0x1 as i32,
    ) as *mut cvar_s;
    modified = (modified as u32).wrapping_add((*net_socksEnabled).modified as u32) as i32;
    (*net_socksEnabled).modified = qfalse;
    net_socksServer = Cvar_Get(
        b"net_socksServer\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x20 as i32 | 0x1 as i32,
    ) as *mut cvar_s;
    modified = (modified as u32).wrapping_add((*net_socksServer).modified as u32) as i32;
    (*net_socksServer).modified = qfalse;
    net_socksPort = Cvar_Get(
        b"net_socksPort\x00" as *const u8 as *const libc::c_char,
        b"1080\x00" as *const u8 as *const libc::c_char,
        0x20 as i32 | 0x1 as i32,
    ) as *mut cvar_s;
    modified = (modified as u32).wrapping_add((*net_socksPort).modified as u32) as i32;
    (*net_socksPort).modified = qfalse;
    net_socksUsername = Cvar_Get(
        b"net_socksUsername\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x20 as i32 | 0x1 as i32,
    ) as *mut cvar_s;
    modified = (modified as u32).wrapping_add((*net_socksUsername).modified as u32) as i32;
    (*net_socksUsername).modified = qfalse;
    net_socksPassword = Cvar_Get(
        b"net_socksPassword\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x20 as i32 | 0x1 as i32,
    ) as *mut cvar_s;
    modified = (modified as u32).wrapping_add((*net_socksPassword).modified as u32) as i32;
    (*net_socksPassword).modified = qfalse;
    net_dropsim = Cvar_Get(
        b"net_dropsim\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x100 as i32,
    ) as *mut cvar_s;
    return if modified != 0 {
        qtrue as i32
    } else {
        qfalse as i32
    } as qboolean;
}
/*
====================
NET_Config
====================
*/
#[no_mangle]

pub unsafe extern "C" fn NET_Config(mut enableNetworking: qboolean) {
    let mut modified: qboolean = qfalse;
    let mut stop: qboolean = qfalse;
    let mut start: qboolean = qfalse;
    // get any latched changes to cvars
    modified = NET_GetCvars();
    if (*net_enabled).integer == 0 {
        enableNetworking = qfalse
    }
    // if enable state is the same and no cvars were modified, we have nothing to do
    if enableNetworking as u32 == networkingEnabled as u32 && modified as u64 == 0 {
        return;
    }
    if enableNetworking as u32 == networkingEnabled as u32 {
        if enableNetworking as u64 != 0 {
            stop = qtrue;
            start = qtrue
        } else {
            stop = qfalse;
            start = qfalse
        }
    } else {
        if enableNetworking as u64 != 0 {
            stop = qfalse;
            start = qtrue
        } else {
            stop = qtrue;
            start = qfalse
        }
        networkingEnabled = enableNetworking as i32
    }
    if stop as u64 != 0 {
        if ip_socket != -(1 as i32) {
            libc::close(ip_socket);
            ip_socket = -(1 as i32)
        }
        if multicast6_socket != -(1 as i32) {
            if multicast6_socket != ip6_socket {
                libc::close(multicast6_socket);
            }
            multicast6_socket = -(1 as i32)
        }
        if ip6_socket != -(1 as i32) {
            libc::close(ip6_socket);
            ip6_socket = -(1 as i32)
        }
        if socks_socket != -(1 as i32) {
            libc::close(socks_socket);
            socks_socket = -(1 as i32)
        }
    }
    if start as u64 != 0 {
        if (*net_enabled).integer != 0 {
            NET_OpenIP();
            NET_SetMulticast6();
        }
    };
}
/*
====================
NET_Init
====================
*/
#[no_mangle]

pub unsafe extern "C" fn NET_Init() {
    NET_Config(qtrue);
    Cmd_AddCommand(
        b"net_restart\x00" as *const u8 as *const libc::c_char,
        Some(NET_Restart_f as unsafe extern "C" fn() -> ()),
    );
}
/*
====================
NET_Shutdown
====================
*/
#[no_mangle]

pub unsafe extern "C" fn NET_Shutdown() {
    if networkingEnabled == 0 {
        return;
    }
    NET_Config(qfalse);
}
/*
====================
NET_Event

Called from NET_Sleep which uses select() to determine which sockets have seen action.
====================
*/
#[no_mangle]

pub unsafe extern "C" fn NET_Event(mut fdr: *mut fd_set) {
    let mut bufData: [byte; 16385] = [0; 16385];
    let mut from: netadr_t = {
        let mut init = netadr_t {
            type_0: NA_BAD,
            ip: [0; 4],
            ip6: [0; 16],
            port: 0,
            scope_id: 0,
        };
        init
    };
    let mut netmsg: msg_t = msg_t {
        allowoverflow: qfalse,
        overflowed: qfalse,
        oob: qfalse,
        data: std::ptr::null_mut(),
        maxsize: 0,
        cursize: 0,
        readcount: 0,
        bit: 0,
    };
    loop {
        MSG_Init(
            &mut netmsg as *mut _ as *mut msg_t,
            bufData.as_mut_ptr(),
            ::std::mem::size_of::<[byte; 16385]>() as usize as i32,
        );
        if !(NET_GetPacket(&mut from, &mut netmsg, fdr) as u64 != 0) {
            break;
        }
        if (*net_dropsim).value > 0.0f32 && (*net_dropsim).value <= 100.0f32 {
            // com_dropsim->value percent of incoming packets get dropped.
            if libc::rand()
                < (2147483647 as i32 as f64 / 100.0f64 * (*net_dropsim).value as f64) as i32
            {
                continue;
            }
            // drop this packet
        }
        if (*com_sv_running).integer != 0 {
            Com_RunAndTimeServerPacket(
                &mut from as *mut _ as *mut netadr_t,
                &mut netmsg as *mut _ as *mut msg_t,
            );
        } else {
            CL_PacketEvent(from as netadr_t, &mut netmsg as *mut _ as *mut msg_t);
        }
    }
}
/*
====================
NET_Sleep

Sleeps msec or until something happens on the network
====================
*/
#[no_mangle]

pub unsafe extern "C" fn NET_Sleep(mut msec: i32) {
    let mut timeout: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut fdr: fd_set = fd_set {
        __fds_bits: [0; 16],
    };
    let mut retval: i32 = 0;
    let mut highestfd: SOCKET = -(1 as i32);
    if msec < 0 as i32 {
        msec = 0 as i32
    }
    let mut __d0: i32 = 0;
    let mut __d1: i32 = 0;
    let fresh0 = &mut __d0;
    let fresh1;
    let fresh2 = &mut __d1;
    let fresh3;
    let fresh4 = (::std::mem::size_of::<fd_set>() as usize)
        .wrapping_div(::std::mem::size_of::<__fd_mask>() as usize);
    let fresh5 = &mut *fdr.__fds_bits.as_mut_ptr().offset(0 as i32 as isize) as *mut __fd_mask;
    asm!("cld; rep; stosq" : "={cx}" (fresh1), "={di}" (fresh3) : "{ax}"
     (0 as i32), "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh4)), "1"
     (c2rust_asm_casts::AsmCast::cast_in(fresh2, fresh5)) : "memory" :
     "volatile");
    c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh4, fresh1);
    c2rust_asm_casts::AsmCast::cast_out(fresh2, fresh5, fresh3);
    if ip_socket != -(1 as i32) {
        fdr.__fds_bits[(ip_socket / (8 as i32 * ::std::mem::size_of::<__fd_mask>() as usize as i32))
            as usize] |= ((1 as usize)
            << ip_socket % (8 as i32 * ::std::mem::size_of::<__fd_mask>() as usize as i32))
            as __fd_mask;
        highestfd = ip_socket
    }
    if ip6_socket != -(1 as i32) {
        fdr.__fds_bits[(ip6_socket
            / (8 as i32 * ::std::mem::size_of::<__fd_mask>() as usize as i32))
            as usize] |= ((1 as usize)
            << ip6_socket % (8 as i32 * ::std::mem::size_of::<__fd_mask>() as usize as i32))
            as __fd_mask;
        if highestfd == -(1 as i32) || ip6_socket > highestfd {
            highestfd = ip6_socket
        }
    }
    timeout.tv_sec = ((msec / 1000 as i32) as __time_t) as libc::time_t;
    timeout.tv_usec = ((msec % 1000 as i32 * 1000 as i32) as __suseconds_t) as libc::suseconds_t;
    retval = select(
        highestfd + 1 as i32,
        &mut fdr,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        &mut timeout,
    );
    if retval == -(1 as i32) {
        Com_Printf(
            b"Warning: select() syscall failed: %s\n\x00" as *const u8 as *const libc::c_char,
            NET_ErrorString(),
        );
    } else if retval > 0 as i32 {
        NET_Event(&mut fdr);
    };
}
/*
====================
NET_Restart_f
====================
*/
#[no_mangle]

pub unsafe extern "C" fn NET_Restart_f() {
    NET_Config(qtrue);
}
