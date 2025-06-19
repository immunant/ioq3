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
    pub type_0: crate::qcommon_h::netadrtype_t,
    pub family: crate::stdlib::sa_family_t,
    pub addr: crate::stdlib::sockaddr_storage,
    pub netmask: crate::stdlib::sockaddr_storage,
}

static mut usingSocks: crate::src::qcommon::q_shared::qboolean =
    crate::src::qcommon::q_shared::qfalse;

static mut networkingEnabled: i32 = 0 as i32;

static mut net_enabled: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;

static mut net_socksEnabled: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;

static mut net_socksServer: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;

static mut net_socksPort: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;

static mut net_socksUsername: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;

static mut net_socksPassword: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;

static mut net_ip: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;

static mut net_ip6: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;

static mut net_port: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;

static mut net_port6: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;

static mut net_mcast6addr: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;

static mut net_mcast6iface: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;

static mut net_dropsim: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;

static mut socksRelayAddr: ::libc::sockaddr = ::libc::sockaddr {
    sa_family: 0,
    sa_data: [0; 14],
};

static mut ip_socket: SOCKET = -(1 as i32);

static mut ip6_socket: SOCKET = -(1 as i32);

static mut socks_socket: SOCKET = -(1 as i32);

static mut multicast6_socket: SOCKET = -(1 as i32);
// Keep track of currently joined multicast group.

static mut curgroup: crate::stdlib::ipv6_mreq = crate::stdlib::ipv6_mreq {
    ipv6mr_multiaddr: crate::stdlib::in6_addr {
        __in6_u: crate::stdlib::C2RustUnnamed_129 {
            __u6_addr8: [0; 16],
        },
    },
    ipv6mr_interface: 0,
};
// And the currently bound address.

static mut boundto: crate::stdlib::sockaddr_in6 = crate::stdlib::sockaddr_in6 {
    sin6_family: 0,
    sin6_port: 0,
    sin6_flowinfo: 0,
    sin6_addr: crate::stdlib::in6_addr {
        __in6_u: crate::stdlib::C2RustUnnamed_129 {
            __u6_addr8: [0; 16],
        },
    },
    sin6_scope_id: 0,
};

static mut localIP: [nip_localaddr_t; 32] = [nip_localaddr_t {
    ifname: [0; 16],
    type_0: crate::qcommon_h::NA_BAD,
    family: 0,
    addr: crate::stdlib::sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    },
    netmask: crate::stdlib::sockaddr_storage {
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
    return ::libc::strerror(*::libc::__errno_location());
}

unsafe extern "C" fn NetadrToSockadr(
    mut a: *mut crate::qcommon_h::netadr_t,
    mut s: *mut ::libc::sockaddr,
) {
    if (*a).type_0 as u32 == crate::qcommon_h::NA_BROADCAST as i32 as u32 {
        (*(s as *mut ::libc::sockaddr_in)).sin_family = 2 as i32 as crate::stdlib::sa_family_t;
        (*(s as *mut ::libc::sockaddr_in)).sin_port = (*a).port;
        (*(s as *mut ::libc::sockaddr_in)).sin_addr.s_addr = 0xffffffff as u32
    } else if (*a).type_0 as u32 == crate::qcommon_h::NA_IP as i32 as u32 {
        (*(s as *mut ::libc::sockaddr_in)).sin_family = 2 as i32 as crate::stdlib::sa_family_t;
        (*(s as *mut ::libc::sockaddr_in)).sin_addr.s_addr =
            *(&mut (*a).ip as *mut [crate::src::qcommon::q_shared::byte; 4] as *mut i32)
                as crate::stdlib::in_addr_t;
        (*(s as *mut ::libc::sockaddr_in)).sin_port = (*a).port
    } else if (*a).type_0 as u32 == crate::qcommon_h::NA_IP6 as i32 as u32 {
        (*(s as *mut crate::stdlib::sockaddr_in6)).sin6_family =
            10 as i32 as crate::stdlib::sa_family_t;
        (*(s as *mut crate::stdlib::sockaddr_in6)).sin6_addr = *(&mut (*a).ip6
            as *mut [crate::src::qcommon::q_shared::byte; 16]
            as *mut crate::stdlib::in6_addr);
        (*(s as *mut crate::stdlib::sockaddr_in6)).sin6_port = (*a).port;
        (*(s as *mut crate::stdlib::sockaddr_in6)).sin6_scope_id =
            (*a).scope_id as crate::stdlib::uint32_t
    } else if (*a).type_0 as u32 == crate::qcommon_h::NA_MULTICAST6 as i32 as u32 {
        (*(s as *mut crate::stdlib::sockaddr_in6)).sin6_family =
            10 as i32 as crate::stdlib::sa_family_t;
        (*(s as *mut crate::stdlib::sockaddr_in6)).sin6_addr = curgroup.ipv6mr_multiaddr;
        (*(s as *mut crate::stdlib::sockaddr_in6)).sin6_port = (*a).port
    };
}

unsafe extern "C" fn SockadrToNetadr(
    mut s: *mut ::libc::sockaddr,
    mut a: *mut crate::qcommon_h::netadr_t,
) {
    if (*s).sa_family as i32 == 2 as i32 {
        (*a).type_0 = crate::qcommon_h::NA_IP;
        *(&mut (*a).ip as *mut [crate::src::qcommon::q_shared::byte; 4] as *mut i32) =
            (*(s as *mut ::libc::sockaddr_in)).sin_addr.s_addr as i32;
        (*a).port = (*(s as *mut ::libc::sockaddr_in)).sin_port
    } else if (*s).sa_family as i32 == 10 as i32 {
        (*a).type_0 = crate::qcommon_h::NA_IP6;
        crate::stdlib::memcpy(
            (*a).ip6.as_mut_ptr() as *mut libc::c_void,
            &mut (*(s as *mut crate::stdlib::sockaddr_in6)).sin6_addr
                as *mut crate::stdlib::in6_addr as *const libc::c_void,
            ::std::mem::size_of::<[crate::src::qcommon::q_shared::byte; 16]>() as libc::c_ulong,
        );
        (*a).port = (*(s as *mut crate::stdlib::sockaddr_in6)).sin6_port;
        (*a).scope_id = (*(s as *mut crate::stdlib::sockaddr_in6)).sin6_scope_id as libc::c_ulong
    };
}

unsafe extern "C" fn SearchAddrInfo(
    mut hints: *mut ::libc::addrinfo,
    mut family: crate::stdlib::sa_family_t,
) -> *mut ::libc::addrinfo {
    while !hints.is_null() {
        if (*hints).ai_family == family as i32 {
            return hints;
        }
        hints = (*hints).ai_next
    }
    return 0 as *mut ::libc::addrinfo;
}
/*
=============
Sys_StringToSockaddr
=============
*/

unsafe extern "C" fn Sys_StringToSockaddr(
    mut s: *const libc::c_char,
    mut sadr: *mut ::libc::sockaddr,
    mut sadr_len: i32,
    mut family: crate::stdlib::sa_family_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut hints: ::libc::addrinfo = ::libc::addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: 0 as *mut ::libc::sockaddr,
        ai_canonname: 0 as *mut libc::c_char,
        ai_next: 0 as *mut ::libc::addrinfo,
    };
    let mut res: *mut ::libc::addrinfo = 0 as *mut ::libc::addrinfo;
    let mut search: *mut ::libc::addrinfo = 0 as *mut ::libc::addrinfo;
    let mut hintsp: *mut ::libc::addrinfo = 0 as *mut ::libc::addrinfo;
    let mut retval: i32 = 0;
    crate::stdlib::memset(
        sadr as *mut libc::c_void,
        '\u{0}' as i32,
        ::std::mem::size_of::<::libc::sockaddr>() as libc::c_ulong,
    );
    crate::stdlib::memset(
        &mut hints as *mut ::libc::addrinfo as *mut libc::c_void,
        '\u{0}' as i32,
        ::std::mem::size_of::<::libc::addrinfo>() as libc::c_ulong,
    );
    hintsp = &mut hints;
    (*hintsp).ai_family = family as i32;
    (*hintsp).ai_socktype = crate::stdlib::SOCK_DGRAM as i32;
    retval = ::libc::getaddrinfo(
        s,
        0 as *const libc::c_char,
        hintsp as *const ::libc::addrinfo,
        &mut res as *mut _ as *mut *mut ::libc::addrinfo,
    );
    if retval == 0 {
        if family as i32 == 0 as i32 {
            // Decide here and now which protocol family to use
            if (*net_enabled).integer & 0x4 as i32 != 0 {
                if (*net_enabled).integer & 0x2 as i32 != 0 {
                    search = SearchAddrInfo(res, 10 as i32 as crate::stdlib::sa_family_t)
                }
                if search.is_null() && (*net_enabled).integer & 0x1 as i32 != 0 {
                    search = SearchAddrInfo(res, 2 as i32 as crate::stdlib::sa_family_t)
                }
            } else {
                if (*net_enabled).integer & 0x1 as i32 != 0 {
                    search = SearchAddrInfo(res, 2 as i32 as crate::stdlib::sa_family_t)
                }
                if search.is_null() && (*net_enabled).integer & 0x2 as i32 != 0 {
                    search = SearchAddrInfo(res, 10 as i32 as crate::stdlib::sa_family_t)
                }
            }
        } else {
            search = SearchAddrInfo(res, family)
        }
        if !search.is_null() {
            if (*search).ai_addrlen > sadr_len as u32 {
                (*search).ai_addrlen = sadr_len as crate::stdlib::socklen_t
            }
            crate::stdlib::memcpy(
                sadr as *mut libc::c_void,
                (*search).ai_addr as *const libc::c_void,
                (*search).ai_addrlen as libc::c_ulong,
            );
            ::libc::freeaddrinfo(res as *mut ::libc::addrinfo);
            return crate::src::qcommon::q_shared::qtrue;
        } else {
            crate::src::qcommon::common::Com_Printf(b"Sys_StringToSockaddr: Error resolving %s: No address of required type found.\n\x00"
                           as *const u8 as *const libc::c_char, s);
        }
    } else {
        crate::src::qcommon::common::Com_Printf(
            b"Sys_StringToSockaddr: Error resolving %s: %s\n\x00" as *const u8
                as *const libc::c_char,
            s,
            ::libc::gai_strerror(retval),
        );
    }
    if !res.is_null() {
        ::libc::freeaddrinfo(res as *mut ::libc::addrinfo);
    }
    return crate::src::qcommon::q_shared::qfalse;
}
/*
=============
Sys_SockaddrToString
=============
*/

unsafe extern "C" fn Sys_SockaddrToString(
    mut dest: *mut libc::c_char,
    mut destlen: i32,
    mut input: *mut ::libc::sockaddr,
) {
    let mut inputlen: crate::stdlib::socklen_t = 0;
    if (*input).sa_family as i32 == 10 as i32 {
        inputlen = ::std::mem::size_of::<crate::stdlib::sockaddr_in6>() as libc::c_ulong
            as crate::stdlib::socklen_t
    } else {
        inputlen = ::std::mem::size_of::<::libc::sockaddr_in>() as libc::c_ulong
            as crate::stdlib::socklen_t
    }
    if ::libc::getnameinfo(
        input as *const ::libc::sockaddr,
        inputlen,
        dest,
        destlen as crate::stdlib::socklen_t,
        0 as *mut libc::c_char,
        0 as i32 as crate::stdlib::socklen_t,
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
    mut a: *mut crate::qcommon_h::netadr_t,
    mut family: crate::qcommon_h::netadrtype_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut sadr: crate::stdlib::sockaddr_storage = crate::stdlib::sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut fam: crate::stdlib::sa_family_t = 0;
    match family as u32 {
        4 => fam = 2 as i32 as crate::stdlib::sa_family_t,
        5 => fam = 10 as i32 as crate::stdlib::sa_family_t,
        _ => fam = 0 as i32 as crate::stdlib::sa_family_t,
    }
    if Sys_StringToSockaddr(
        s,
        &mut sadr as *mut crate::stdlib::sockaddr_storage as *mut ::libc::sockaddr,
        ::std::mem::size_of::<crate::stdlib::sockaddr_storage>() as libc::c_ulong as i32,
        fam,
    ) as u64
        == 0
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    SockadrToNetadr(
        &mut sadr as *mut crate::stdlib::sockaddr_storage as *mut ::libc::sockaddr,
        a,
    );
    return crate::src::qcommon::q_shared::qtrue;
}
/*
===================
NET_CompareBaseAdrMask

Compare without port, and up to the bit number given in netmask.
===================
*/
#[no_mangle]

pub unsafe extern "C" fn NET_CompareBaseAdrMask(
    mut a: crate::qcommon_h::netadr_t,
    mut b: crate::qcommon_h::netadr_t,
    mut netmask: i32,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut cmpmask: crate::src::qcommon::q_shared::byte = 0;
    let mut addra: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut addrb: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut curbyte: i32 = 0;
    if a.type_0 as u32 != b.type_0 as u32 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if a.type_0 as u32 == crate::qcommon_h::NA_LOOPBACK as i32 as u32 {
        return crate::src::qcommon::q_shared::qtrue;
    }
    if a.type_0 as u32 == crate::qcommon_h::NA_IP as i32 as u32 {
        addra = &mut a.ip as *mut [crate::src::qcommon::q_shared::byte; 4]
            as *mut crate::src::qcommon::q_shared::byte;
        addrb = &mut b.ip as *mut [crate::src::qcommon::q_shared::byte; 4]
            as *mut crate::src::qcommon::q_shared::byte;
        if netmask < 0 as i32 || netmask > 32 as i32 {
            netmask = 32 as i32
        }
    } else if a.type_0 as u32 == crate::qcommon_h::NA_IP6 as i32 as u32 {
        addra = &mut a.ip6 as *mut [crate::src::qcommon::q_shared::byte; 16]
            as *mut crate::src::qcommon::q_shared::byte;
        addrb = &mut b.ip6 as *mut [crate::src::qcommon::q_shared::byte; 16]
            as *mut crate::src::qcommon::q_shared::byte;
        if netmask < 0 as i32 || netmask > 128 as i32 {
            netmask = 128 as i32
        }
    } else {
        crate::src::qcommon::common::Com_Printf(
            b"NET_CompareBaseAdr: bad address type\n\x00" as *const u8 as *const libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse;
    }
    curbyte = netmask >> 3 as i32;
    if curbyte != 0
        && crate::stdlib::memcmp(
            addra as *const libc::c_void,
            addrb as *const libc::c_void,
            curbyte as libc::c_ulong,
        ) != 0
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    netmask &= 0x7 as i32;
    if netmask != 0 {
        cmpmask = (((1 as i32) << netmask) - 1 as i32) as crate::src::qcommon::q_shared::byte;
        cmpmask = ((cmpmask as i32) << 8 as i32 - netmask) as crate::src::qcommon::q_shared::byte;
        if *addra.offset(curbyte as isize) as i32 & cmpmask as i32
            == *addrb.offset(curbyte as isize) as i32 & cmpmask as i32
        {
            return crate::src::qcommon::q_shared::qtrue;
        }
    } else {
        return crate::src::qcommon::q_shared::qtrue;
    }
    return crate::src::qcommon::q_shared::qfalse;
}
/*
===================
NET_CompareBaseAdr

Compares without the port
===================
*/
#[no_mangle]

pub unsafe extern "C" fn NET_CompareBaseAdr(
    mut a: crate::qcommon_h::netadr_t,
    mut b: crate::qcommon_h::netadr_t,
) -> crate::src::qcommon::q_shared::qboolean {
    return NET_CompareBaseAdrMask(a, b, -(1 as i32));
}
#[no_mangle]

pub unsafe extern "C" fn NET_AdrToString(mut a: crate::qcommon_h::netadr_t) -> *const libc::c_char {
    static mut s: [libc::c_char; 48] = [0; 48];
    if a.type_0 as u32 == crate::qcommon_h::NA_LOOPBACK as i32 as u32 {
        crate::src::qcommon::q_shared::Com_sprintf(
            s.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 48]>() as libc::c_ulong as i32,
            b"loopback\x00" as *const u8 as *const libc::c_char,
        );
    } else if a.type_0 as u32 == crate::qcommon_h::NA_BOT as i32 as u32 {
        crate::src::qcommon::q_shared::Com_sprintf(
            s.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 48]>() as libc::c_ulong as i32,
            b"bot\x00" as *const u8 as *const libc::c_char,
        );
    } else if a.type_0 as u32 == crate::qcommon_h::NA_IP as i32 as u32
        || a.type_0 as u32 == crate::qcommon_h::NA_IP6 as i32 as u32
    {
        let mut sadr: crate::stdlib::sockaddr_storage = crate::stdlib::sockaddr_storage {
            ss_family: 0,
            __ss_padding: [0; 118],
            __ss_align: 0,
        };
        crate::stdlib::memset(
            &mut sadr as *mut crate::stdlib::sockaddr_storage as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<crate::stdlib::sockaddr_storage>() as libc::c_ulong,
        );
        NetadrToSockadr(
            &mut a,
            &mut sadr as *mut crate::stdlib::sockaddr_storage as *mut ::libc::sockaddr,
        );
        Sys_SockaddrToString(
            s.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 48]>() as libc::c_ulong as i32,
            &mut sadr as *mut crate::stdlib::sockaddr_storage as *mut ::libc::sockaddr,
        );
    }
    return s.as_mut_ptr();
}
#[no_mangle]

pub unsafe extern "C" fn NET_AdrToStringwPort(
    mut a: crate::qcommon_h::netadr_t,
) -> *const libc::c_char {
    static mut s: [libc::c_char; 48] = [0; 48];
    if a.type_0 as u32 == crate::qcommon_h::NA_LOOPBACK as i32 as u32 {
        crate::src::qcommon::q_shared::Com_sprintf(
            s.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 48]>() as libc::c_ulong as i32,
            b"loopback\x00" as *const u8 as *const libc::c_char,
        );
    } else if a.type_0 as u32 == crate::qcommon_h::NA_BOT as i32 as u32 {
        crate::src::qcommon::q_shared::Com_sprintf(
            s.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 48]>() as libc::c_ulong as i32,
            b"bot\x00" as *const u8 as *const libc::c_char,
        );
    } else if a.type_0 as u32 == crate::qcommon_h::NA_IP as i32 as u32 {
        crate::src::qcommon::q_shared::Com_sprintf(
            s.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 48]>() as libc::c_ulong as i32,
            b"%s:%hu\x00" as *const u8 as *const libc::c_char,
            NET_AdrToString(a),
            __bswap_16(a.port) as i32,
        );
    } else if a.type_0 as u32 == crate::qcommon_h::NA_IP6 as i32 as u32 {
        crate::src::qcommon::q_shared::Com_sprintf(
            s.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 48]>() as libc::c_ulong as i32,
            b"[%s]:%hu\x00" as *const u8 as *const libc::c_char,
            NET_AdrToString(a),
            __bswap_16(a.port) as i32,
        );
    }
    return s.as_mut_ptr();
}
#[no_mangle]

pub unsafe extern "C" fn NET_CompareAdr(
    mut a: crate::qcommon_h::netadr_t,
    mut b: crate::qcommon_h::netadr_t,
) -> crate::src::qcommon::q_shared::qboolean {
    if NET_CompareBaseAdr(a, b) as u64 == 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if a.type_0 as u32 == crate::qcommon_h::NA_IP as i32 as u32
        || a.type_0 as u32 == crate::qcommon_h::NA_IP6 as i32 as u32
    {
        if a.port as i32 == b.port as i32 {
            return crate::src::qcommon::q_shared::qtrue;
        }
    } else {
        return crate::src::qcommon::q_shared::qtrue;
    }
    return crate::src::qcommon::q_shared::qfalse;
}
#[no_mangle]

pub unsafe extern "C" fn NET_IsLocalAddress(
    mut adr: crate::qcommon_h::netadr_t,
) -> crate::src::qcommon::q_shared::qboolean {
    return (adr.type_0 as u32 == crate::qcommon_h::NA_LOOPBACK as i32 as u32) as i32
        as crate::src::qcommon::q_shared::qboolean;
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
    mut net_from: *mut crate::qcommon_h::netadr_t,
    mut net_message: *mut crate::qcommon_h::msg_t,
    mut fdr: *mut crate::stdlib::fd_set,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut ret: i32 = 0;
    let mut from: crate::stdlib::sockaddr_storage = crate::stdlib::sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut fromlen: crate::stdlib::socklen_t = 0;
    let mut err: i32 = 0;
    if ip_socket != -(1 as i32)
        && (*fdr).__fds_bits[(ip_socket
            / (8 as i32
                * ::std::mem::size_of::<crate::stdlib::__fd_mask>() as libc::c_ulong as i32))
            as usize]
            & ((1 as libc::c_ulong)
                << ip_socket
                    % (8 as i32
                        * ::std::mem::size_of::<crate::stdlib::__fd_mask>() as libc::c_ulong
                            as i32)) as crate::stdlib::__fd_mask
            != 0 as i32 as libc::c_long
    {
        fromlen = ::std::mem::size_of::<crate::stdlib::sockaddr_storage>() as libc::c_ulong
            as crate::stdlib::socklen_t;
        ret = crate::stdlib::recvfrom(
            ip_socket,
            (*net_message).data as *mut libc::c_void,
            (*net_message).maxsize as crate::stddef_h::size_t,
            0 as i32,
            &mut from as *mut crate::stdlib::sockaddr_storage as *mut ::libc::sockaddr,
            &mut fromlen,
        ) as i32;
        if ret == -(1 as i32) {
            err = *::libc::__errno_location();
            if err != 11 as i32 && err != 104 as i32 {
                crate::src::qcommon::common::Com_Printf(
                    b"NET_GetPacket: %s\n\x00" as *const u8 as *const libc::c_char,
                    NET_ErrorString(),
                );
            }
        } else {
            crate::stdlib::memset(
                (*(&mut from as *mut crate::stdlib::sockaddr_storage as *mut ::libc::sockaddr_in))
                    .sin_zero
                    .as_mut_ptr() as *mut libc::c_void,
                0 as i32,
                8 as i32 as libc::c_ulong,
            );
            if usingSocks as u32 != 0
                && crate::stdlib::memcmp(
                    &mut from as *mut crate::stdlib::sockaddr_storage as *const libc::c_void,
                    &mut socksRelayAddr as *mut ::libc::sockaddr as *const libc::c_void,
                    fromlen as libc::c_ulong,
                ) == 0 as i32
            {
                if ret < 10 as i32
                    || *(*net_message).data.offset(0 as i32 as isize) as i32 != 0 as i32
                    || *(*net_message).data.offset(1 as i32 as isize) as i32 != 0 as i32
                    || *(*net_message).data.offset(2 as i32 as isize) as i32 != 0 as i32
                    || *(*net_message).data.offset(3 as i32 as isize) as i32 != 1 as i32
                {
                    return crate::src::qcommon::q_shared::qfalse;
                }
                (*net_from).type_0 = crate::qcommon_h::NA_IP;
                (*net_from).ip[0 as i32 as usize] = *(*net_message).data.offset(4 as i32 as isize);
                (*net_from).ip[1 as i32 as usize] = *(*net_message).data.offset(5 as i32 as isize);
                (*net_from).ip[2 as i32 as usize] = *(*net_message).data.offset(6 as i32 as isize);
                (*net_from).ip[3 as i32 as usize] = *(*net_message).data.offset(7 as i32 as isize);
                (*net_from).port = *(&mut *(*net_message).data.offset(8 as i32 as isize)
                    as *mut crate::src::qcommon::q_shared::byte
                    as *mut i16) as u16;
                (*net_message).readcount = 10 as i32
            } else {
                SockadrToNetadr(
                    &mut from as *mut crate::stdlib::sockaddr_storage as *mut ::libc::sockaddr,
                    net_from,
                );
                (*net_message).readcount = 0 as i32
            }
            if ret >= (*net_message).maxsize {
                crate::src::qcommon::common::Com_Printf(
                    b"Oversize packet from %s\n\x00" as *const u8 as *const libc::c_char,
                    NET_AdrToString(*net_from),
                );
                return crate::src::qcommon::q_shared::qfalse;
            }
            (*net_message).cursize = ret;
            return crate::src::qcommon::q_shared::qtrue;
        }
    }
    if ip6_socket != -(1 as i32)
        && (*fdr).__fds_bits[(ip6_socket
            / (8 as i32
                * ::std::mem::size_of::<crate::stdlib::__fd_mask>() as libc::c_ulong as i32))
            as usize]
            & ((1 as libc::c_ulong)
                << ip6_socket
                    % (8 as i32
                        * ::std::mem::size_of::<crate::stdlib::__fd_mask>() as libc::c_ulong
                            as i32)) as crate::stdlib::__fd_mask
            != 0 as i32 as libc::c_long
    {
        fromlen = ::std::mem::size_of::<crate::stdlib::sockaddr_storage>() as libc::c_ulong
            as crate::stdlib::socklen_t;
        ret = crate::stdlib::recvfrom(
            ip6_socket,
            (*net_message).data as *mut libc::c_void,
            (*net_message).maxsize as crate::stddef_h::size_t,
            0 as i32,
            &mut from as *mut crate::stdlib::sockaddr_storage as *mut ::libc::sockaddr,
            &mut fromlen,
        ) as i32;
        if ret == -(1 as i32) {
            err = *::libc::__errno_location();
            if err != 11 as i32 && err != 104 as i32 {
                crate::src::qcommon::common::Com_Printf(
                    b"NET_GetPacket: %s\n\x00" as *const u8 as *const libc::c_char,
                    NET_ErrorString(),
                );
            }
        } else {
            SockadrToNetadr(
                &mut from as *mut crate::stdlib::sockaddr_storage as *mut ::libc::sockaddr,
                net_from,
            );
            (*net_message).readcount = 0 as i32;
            if ret >= (*net_message).maxsize {
                crate::src::qcommon::common::Com_Printf(
                    b"Oversize packet from %s\n\x00" as *const u8 as *const libc::c_char,
                    NET_AdrToString(*net_from),
                );
                return crate::src::qcommon::q_shared::qfalse;
            }
            (*net_message).cursize = ret;
            return crate::src::qcommon::q_shared::qtrue;
        }
    }
    if multicast6_socket != -(1 as i32)
        && multicast6_socket != ip6_socket
        && (*fdr).__fds_bits[(multicast6_socket
            / (8 as i32
                * ::std::mem::size_of::<crate::stdlib::__fd_mask>() as libc::c_ulong as i32))
            as usize]
            & ((1 as libc::c_ulong)
                << multicast6_socket
                    % (8 as i32
                        * ::std::mem::size_of::<crate::stdlib::__fd_mask>() as libc::c_ulong
                            as i32)) as crate::stdlib::__fd_mask
            != 0 as i32 as libc::c_long
    {
        fromlen = ::std::mem::size_of::<crate::stdlib::sockaddr_storage>() as libc::c_ulong
            as crate::stdlib::socklen_t;
        ret = crate::stdlib::recvfrom(
            multicast6_socket,
            (*net_message).data as *mut libc::c_void,
            (*net_message).maxsize as crate::stddef_h::size_t,
            0 as i32,
            &mut from as *mut crate::stdlib::sockaddr_storage as *mut ::libc::sockaddr,
            &mut fromlen,
        ) as i32;
        if ret == -(1 as i32) {
            err = *::libc::__errno_location();
            if err != 11 as i32 && err != 104 as i32 {
                crate::src::qcommon::common::Com_Printf(
                    b"NET_GetPacket: %s\n\x00" as *const u8 as *const libc::c_char,
                    NET_ErrorString(),
                );
            }
        } else {
            SockadrToNetadr(
                &mut from as *mut crate::stdlib::sockaddr_storage as *mut ::libc::sockaddr,
                net_from,
            );
            (*net_message).readcount = 0 as i32;
            if ret >= (*net_message).maxsize {
                crate::src::qcommon::common::Com_Printf(
                    b"Oversize packet from %s\n\x00" as *const u8 as *const libc::c_char,
                    NET_AdrToString(*net_from),
                );
                return crate::src::qcommon::q_shared::qfalse;
            }
            (*net_message).cursize = ret;
            return crate::src::qcommon::q_shared::qtrue;
        }
    }
    return crate::src::qcommon::q_shared::qfalse;
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
    mut to: crate::qcommon_h::netadr_t,
) {
    let mut ret: i32 = -(1 as i32); // reserved
    let mut addr: crate::stdlib::sockaddr_storage = crate::stdlib::sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    }; // fragment (not fragmented)
    if to.type_0 as u32 != crate::qcommon_h::NA_BROADCAST as i32 as u32
        && to.type_0 as u32 != crate::qcommon_h::NA_IP as i32 as u32
        && to.type_0 as u32 != crate::qcommon_h::NA_IP6 as i32 as u32
        && to.type_0 as u32 != crate::qcommon_h::NA_MULTICAST6 as i32 as u32
    {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"Sys_SendPacket: bad address type\x00" as *const u8 as *const libc::c_char,
        ); // address type: IPV4
    }
    if ip_socket == -(1 as i32) && to.type_0 as u32 == crate::qcommon_h::NA_IP as i32 as u32
        || ip_socket == -(1 as i32)
            && to.type_0 as u32 == crate::qcommon_h::NA_BROADCAST as i32 as u32
        || ip6_socket == -(1 as i32) && to.type_0 as u32 == crate::qcommon_h::NA_IP6 as i32 as u32
        || ip6_socket == -(1 as i32)
            && to.type_0 as u32 == crate::qcommon_h::NA_MULTICAST6 as i32 as u32
    {
        return;
    }
    if to.type_0 as u32 == crate::qcommon_h::NA_MULTICAST6 as i32 as u32
        && (*net_enabled).integer & 0x8 as i32 != 0
    {
        return;
    }
    crate::stdlib::memset(
        &mut addr as *mut crate::stdlib::sockaddr_storage as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::stdlib::sockaddr_storage>() as libc::c_ulong,
    );
    NetadrToSockadr(
        &mut to,
        &mut addr as *mut crate::stdlib::sockaddr_storage as *mut ::libc::sockaddr,
    );
    if usingSocks as u32 != 0 && to.type_0 as u32 == crate::qcommon_h::NA_IP as i32 as u32 {
        socksBuf[0 as i32 as usize] = 0 as i32 as libc::c_char;
        socksBuf[1 as i32 as usize] = 0 as i32 as libc::c_char;
        socksBuf[2 as i32 as usize] = 0 as i32 as libc::c_char;
        socksBuf[3 as i32 as usize] = 1 as i32 as libc::c_char;
        *(&mut *socksBuf.as_mut_ptr().offset(4 as i32 as isize) as *mut libc::c_char as *mut i32) =
            (*(&mut addr as *mut crate::stdlib::sockaddr_storage as *mut ::libc::sockaddr_in))
                .sin_addr
                .s_addr as i32;
        *(&mut *socksBuf.as_mut_ptr().offset(8 as i32 as isize) as *mut libc::c_char as *mut i16) =
            (*(&mut addr as *mut crate::stdlib::sockaddr_storage as *mut ::libc::sockaddr_in))
                .sin_port as i16;
        crate::stdlib::memcpy(
            &mut *socksBuf.as_mut_ptr().offset(10 as i32 as isize) as *mut libc::c_char
                as *mut libc::c_void,
            data,
            length as libc::c_ulong,
        );
        ret = crate::stdlib::sendto(
            ip_socket,
            socksBuf.as_mut_ptr() as *const libc::c_void,
            (length + 10 as i32) as crate::stddef_h::size_t,
            0 as i32,
            &mut socksRelayAddr,
            ::std::mem::size_of::<::libc::sockaddr>() as libc::c_ulong as crate::stdlib::socklen_t,
        ) as i32
    } else if addr.ss_family as i32 == 2 as i32 {
        ret = crate::stdlib::sendto(
            ip_socket,
            data,
            length as crate::stddef_h::size_t,
            0 as i32,
            &mut addr as *mut crate::stdlib::sockaddr_storage as *mut ::libc::sockaddr,
            ::std::mem::size_of::<::libc::sockaddr_in>() as libc::c_ulong
                as crate::stdlib::socklen_t,
        ) as i32
    } else if addr.ss_family as i32 == 10 as i32 {
        ret = crate::stdlib::sendto(
            ip6_socket,
            data,
            length as crate::stddef_h::size_t,
            0 as i32,
            &mut addr as *mut crate::stdlib::sockaddr_storage as *mut ::libc::sockaddr,
            ::std::mem::size_of::<crate::stdlib::sockaddr_in6>() as libc::c_ulong
                as crate::stdlib::socklen_t,
        ) as i32
    }
    if ret == -(1 as i32) {
        let mut err: i32 = *::libc::__errno_location();
        // wouldblock is silent
        if err == 11 as i32 {
            return;
        }
        // some PPP links do not allow broadcasts and return an error
        if err == 99 as i32 && to.type_0 as u32 == crate::qcommon_h::NA_BROADCAST as i32 as u32 {
            return;
        }
        crate::src::qcommon::common::Com_Printf(
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

pub unsafe extern "C" fn Sys_IsLANAddress(
    mut adr: crate::qcommon_h::netadr_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut index: i32 = 0;
    let mut run: i32 = 0;
    let mut addrsize: i32 = 0;
    let mut differed: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut compareadr: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut comparemask: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut compareip: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    if adr.type_0 as u32 == crate::qcommon_h::NA_LOOPBACK as i32 as u32 {
        return crate::src::qcommon::q_shared::qtrue;
    }
    if adr.type_0 as u32 == crate::qcommon_h::NA_IP as i32 as u32 {
        // RFC1918:
        // 10.0.0.0        -   10.255.255.255  (10/8 prefix)
        // 172.16.0.0      -   172.31.255.255  (172.16/12 prefix)
        // 192.168.0.0     -   192.168.255.255 (192.168/16 prefix)
        if adr.ip[0 as i32 as usize] as i32 == 10 as i32 {
            return crate::src::qcommon::q_shared::qtrue;
        }
        if adr.ip[0 as i32 as usize] as i32 == 172 as i32
            && adr.ip[1 as i32 as usize] as i32 & 0xf0 as i32 == 16 as i32
        {
            return crate::src::qcommon::q_shared::qtrue;
        }
        if adr.ip[0 as i32 as usize] as i32 == 192 as i32
            && adr.ip[1 as i32 as usize] as i32 == 168 as i32
        {
            return crate::src::qcommon::q_shared::qtrue;
        }
        if adr.ip[0 as i32 as usize] as i32 == 127 as i32 {
            return crate::src::qcommon::q_shared::qtrue;
        }
    } else if adr.type_0 as u32 == crate::qcommon_h::NA_IP6 as i32 as u32 {
        if adr.ip6[0 as i32 as usize] as i32 == 0xfe as i32
            && adr.ip6[1 as i32 as usize] as i32 & 0xc0 as i32 == 0x80 as i32
        {
            return crate::src::qcommon::q_shared::qtrue;
        }
        if adr.ip6[0 as i32 as usize] as i32 & 0xfe as i32 == 0xfc as i32 {
            return crate::src::qcommon::q_shared::qtrue;
        }
    }
    // Now compare against the networks this computer is member of.
    index = 0 as i32;
    while index < numIP {
        if localIP[index as usize].type_0 as u32 == adr.type_0 as u32 {
            if adr.type_0 as u32 == crate::qcommon_h::NA_IP as i32 as u32 {
                compareip = &mut (*(&mut (*localIP.as_mut_ptr().offset(index as isize)).addr
                    as *mut crate::stdlib::sockaddr_storage
                    as *mut ::libc::sockaddr_in))
                    .sin_addr
                    .s_addr as *mut crate::stdlib::in_addr_t
                    as *mut crate::src::qcommon::q_shared::byte;
                comparemask = &mut (*(&mut (*localIP.as_mut_ptr().offset(index as isize)).netmask
                    as *mut crate::stdlib::sockaddr_storage
                    as *mut ::libc::sockaddr_in))
                    .sin_addr
                    .s_addr as *mut crate::stdlib::in_addr_t
                    as *mut crate::src::qcommon::q_shared::byte;
                compareadr = adr.ip.as_mut_ptr();
                addrsize = ::std::mem::size_of::<[crate::src::qcommon::q_shared::byte; 4]>()
                    as libc::c_ulong as i32
            } else {
                // TODO? should we check the scope_id here?
                compareip = &mut (*(&mut (*localIP.as_mut_ptr().offset(index as isize)).addr
                    as *mut crate::stdlib::sockaddr_storage
                    as *mut crate::stdlib::sockaddr_in6))
                    .sin6_addr as *mut crate::stdlib::in6_addr
                    as *mut crate::src::qcommon::q_shared::byte;
                comparemask = &mut (*(&mut (*localIP.as_mut_ptr().offset(index as isize)).netmask
                    as *mut crate::stdlib::sockaddr_storage
                    as *mut crate::stdlib::sockaddr_in6))
                    .sin6_addr as *mut crate::stdlib::in6_addr
                    as *mut crate::src::qcommon::q_shared::byte;
                compareadr = adr.ip6.as_mut_ptr();
                addrsize = ::std::mem::size_of::<[crate::src::qcommon::q_shared::byte; 16]>()
                    as libc::c_ulong as i32
            }
            differed = crate::src::qcommon::q_shared::qfalse;
            run = 0 as i32;
            while run < addrsize {
                if *compareip.offset(run as isize) as i32 & *comparemask.offset(run as isize) as i32
                    != *compareadr.offset(run as isize) as i32
                        & *comparemask.offset(run as isize) as i32
                {
                    differed = crate::src::qcommon::q_shared::qtrue;
                    break;
                } else {
                    run += 1
                }
            }
            if differed as u64 == 0 {
                return crate::src::qcommon::q_shared::qtrue;
            }
        }
        index += 1
    }
    return crate::src::qcommon::q_shared::qfalse;
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
            ::std::mem::size_of::<[libc::c_char; 48]>() as libc::c_ulong as i32,
            &mut (*localIP.as_mut_ptr().offset(i as isize)).addr
                as *mut crate::stdlib::sockaddr_storage as *mut ::libc::sockaddr,
        );
        if localIP[i as usize].type_0 as u32 == crate::qcommon_h::NA_IP as i32 as u32 {
            crate::src::qcommon::common::Com_Printf(
                b"IP: %s\n\x00" as *const u8 as *const libc::c_char,
                addrbuf.as_mut_ptr(),
            );
        } else if localIP[i as usize].type_0 as u32 == crate::qcommon_h::NA_IP6 as i32 as u32 {
            crate::src::qcommon::common::Com_Printf(
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
    let mut address: ::libc::sockaddr_in = ::libc::sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: ::libc::in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut _true: ioctlarg_t = 1 as i32;
    let mut i: i32 = 1 as i32;
    *err = 0 as i32;
    if !net_interface.is_null() {
        crate::src::qcommon::common::Com_Printf(
            b"Opening IP socket: %s:%i\n\x00" as *const u8 as *const libc::c_char,
            net_interface,
            port,
        );
    } else {
        crate::src::qcommon::common::Com_Printf(
            b"Opening IP socket: 0.0.0.0:%i\n\x00" as *const u8 as *const libc::c_char,
            port,
        );
    }
    newsocket = ::libc::socket(
        2 as i32,
        crate::stdlib::SOCK_DGRAM as i32,
        crate::stdlib::IPPROTO_UDP as i32,
    );
    if newsocket == -(1 as i32) {
        *err = *::libc::__errno_location();
        crate::src::qcommon::common::Com_Printf(
            b"WARNING: NET_IPSocket: socket: %s\n\x00" as *const u8 as *const libc::c_char,
            NET_ErrorString(),
        );
        return newsocket;
    }
    // make it non-blocking
    if ::libc::ioctl(
        newsocket,
        0x5421 as i32 as libc::c_ulong,
        &mut _true as *mut ioctlarg_t,
    ) == -(1 as i32)
    {
        crate::src::qcommon::common::Com_Printf(
            b"WARNING: NET_IPSocket: ioctl FIONBIO: %s\n\x00" as *const u8 as *const libc::c_char,
            NET_ErrorString(),
        );
        *err = *::libc::__errno_location();
        ::libc::close(newsocket);
        return -(1 as i32);
    }
    // make it broadcast capable
    if ::libc::setsockopt(
        newsocket,
        1 as i32,
        6 as i32,
        &mut i as *mut i32 as *mut libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<i32>() as libc::c_ulong as crate::stdlib::socklen_t,
    ) == -(1 as i32)
    {
        crate::src::qcommon::common::Com_Printf(
            b"WARNING: NET_IPSocket: setsockopt SO_BROADCAST: %s\n\x00" as *const u8
                as *const libc::c_char,
            NET_ErrorString(),
        );
    }
    if net_interface.is_null() || *net_interface.offset(0 as i32 as isize) == 0 {
        address.sin_family = 2 as i32 as crate::stdlib::sa_family_t;
        address.sin_addr.s_addr = 0 as i32 as crate::stdlib::in_addr_t
    } else if Sys_StringToSockaddr(
        net_interface,
        &mut address as *mut ::libc::sockaddr_in as *mut ::libc::sockaddr,
        ::std::mem::size_of::<::libc::sockaddr_in>() as libc::c_ulong as i32,
        2 as i32 as crate::stdlib::sa_family_t,
    ) as u64
        == 0
    {
        ::libc::close(newsocket);
        return -(1 as i32);
    }
    if port == -(1 as i32) {
        address.sin_port = 0 as i32 as crate::stdlib::in_port_t
    } else {
        address.sin_port = __bswap_16(port as i16 as crate::stdlib::__uint16_t)
    }
    if ::libc::bind(
        newsocket,
        &mut address as *mut ::libc::sockaddr_in as *mut libc::c_void as *const ::libc::sockaddr
            as *const ::libc::sockaddr,
        ::std::mem::size_of::<::libc::sockaddr_in>() as libc::c_ulong as crate::stdlib::socklen_t,
    ) == -(1 as i32)
    {
        crate::src::qcommon::common::Com_Printf(
            b"WARNING: NET_IPSocket: bind: %s\n\x00" as *const u8 as *const libc::c_char,
            NET_ErrorString(),
        );
        *err = *::libc::__errno_location();
        ::libc::close(newsocket);
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
    mut bindto: *mut crate::stdlib::sockaddr_in6,
    mut err: *mut i32,
) -> SOCKET {
    let mut newsocket: SOCKET = 0;
    let mut address: crate::stdlib::sockaddr_in6 = crate::stdlib::sockaddr_in6 {
        sin6_family: 0,
        sin6_port: 0,
        sin6_flowinfo: 0,
        sin6_addr: crate::stdlib::in6_addr {
            __in6_u: crate::stdlib::C2RustUnnamed_129 {
                __u6_addr8: [0; 16],
            },
        },
        sin6_scope_id: 0,
    };
    let mut _true: ioctlarg_t = 1 as i32;
    *err = 0 as i32;
    if !net_interface.is_null() {
        // Print the name in brackets if there is a colon:
        if crate::src::qcommon::q_shared::Q_CountChar(net_interface, ':' as i32 as libc::c_char)
            != 0
        {
            crate::src::qcommon::common::Com_Printf(
                b"Opening IP6 socket: [%s]:%i\n\x00" as *const u8 as *const libc::c_char,
                net_interface,
                port,
            );
        } else {
            crate::src::qcommon::common::Com_Printf(
                b"Opening IP6 socket: %s:%i\n\x00" as *const u8 as *const libc::c_char,
                net_interface,
                port,
            );
        }
    } else {
        crate::src::qcommon::common::Com_Printf(
            b"Opening IP6 socket: [::]:%i\n\x00" as *const u8 as *const libc::c_char,
            port,
        );
    }
    newsocket = ::libc::socket(
        10 as i32,
        crate::stdlib::SOCK_DGRAM as i32,
        crate::stdlib::IPPROTO_UDP as i32,
    );
    if newsocket == -(1 as i32) {
        *err = *::libc::__errno_location();
        crate::src::qcommon::common::Com_Printf(
            b"WARNING: NET_IP6Socket: socket: %s\n\x00" as *const u8 as *const libc::c_char,
            NET_ErrorString(),
        );
        return newsocket;
    }
    // make it non-blocking
    if ::libc::ioctl(
        newsocket,
        0x5421 as i32 as libc::c_ulong,
        &mut _true as *mut ioctlarg_t,
    ) == -(1 as i32)
    {
        crate::src::qcommon::common::Com_Printf(
            b"WARNING: NET_IP6Socket: ioctl FIONBIO: %s\n\x00" as *const u8 as *const libc::c_char,
            NET_ErrorString(),
        );
        *err = *::libc::__errno_location();
        ::libc::close(newsocket);
        return -(1 as i32);
    }
    let mut i: i32 = 1 as i32;
    // ipv4 addresses should not be allowed to connect via this socket.
    if ::libc::setsockopt(
        newsocket,
        crate::stdlib::IPPROTO_IPV6 as i32,
        26 as i32,
        &mut i as *mut i32 as *mut libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<i32>() as libc::c_ulong as crate::stdlib::socklen_t,
    ) == -(1 as i32)
    {
        // win32 systems don't seem to support this anyways.
        crate::src::qcommon::common::Com_DPrintf(
            b"WARNING: NET_IP6Socket: setsockopt IPV6_V6ONLY: %s\n\x00" as *const u8
                as *const libc::c_char,
            NET_ErrorString(),
        );
    }
    if net_interface.is_null() || *net_interface.offset(0 as i32 as isize) == 0 {
        address.sin6_family = 10 as i32 as crate::stdlib::sa_family_t;
        address.sin6_addr = crate::stdlib::in6addr_any
    } else if Sys_StringToSockaddr(
        net_interface,
        &mut address as *mut crate::stdlib::sockaddr_in6 as *mut ::libc::sockaddr,
        ::std::mem::size_of::<crate::stdlib::sockaddr_in6>() as libc::c_ulong as i32,
        10 as i32 as crate::stdlib::sa_family_t,
    ) as u64
        == 0
    {
        ::libc::close(newsocket);
        return -(1 as i32);
    }
    if port == -(1 as i32) {
        address.sin6_port = 0 as i32 as crate::stdlib::in_port_t
    } else {
        address.sin6_port = __bswap_16(port as i16 as crate::stdlib::__uint16_t)
    }
    if ::libc::bind(
        newsocket,
        &mut address as *mut crate::stdlib::sockaddr_in6 as *mut libc::c_void
            as *const ::libc::sockaddr as *const ::libc::sockaddr,
        ::std::mem::size_of::<crate::stdlib::sockaddr_in6>() as libc::c_ulong
            as crate::stdlib::socklen_t,
    ) == -(1 as i32)
    {
        crate::src::qcommon::common::Com_Printf(
            b"WARNING: NET_IP6Socket: bind: %s\n\x00" as *const u8 as *const libc::c_char,
            NET_ErrorString(),
        );
        *err = *::libc::__errno_location();
        ::libc::close(newsocket);
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
    let mut addr: crate::stdlib::sockaddr_in6 = crate::stdlib::sockaddr_in6 {
        sin6_family: 0,
        sin6_port: 0,
        sin6_flowinfo: 0,
        sin6_addr: crate::stdlib::in6_addr {
            __in6_u: crate::stdlib::C2RustUnnamed_129 {
                __u6_addr8: [0; 16],
            },
        },
        sin6_scope_id: 0,
    };
    if *(*net_mcast6addr).string == 0
        || Sys_StringToSockaddr(
            (*net_mcast6addr).string,
            &mut addr as *mut crate::stdlib::sockaddr_in6 as *mut ::libc::sockaddr,
            ::std::mem::size_of::<crate::stdlib::sockaddr_in6>() as libc::c_ulong as i32,
            10 as i32 as crate::stdlib::sa_family_t,
        ) as u64
            == 0
    {
        crate::src::qcommon::common::Com_Printf(b"WARNING: NET_JoinMulticast6: Incorrect multicast address given, please set cvar %s to a sane value.\n\x00"
                       as *const u8 as *const libc::c_char,
                   (*net_mcast6addr).name);
        crate::src::qcommon::cvar::Cvar_SetValue(
            (*net_enabled).name,
            ((*net_enabled).integer | 0x8 as i32) as f32,
        );
        return;
    }
    crate::stdlib::memcpy(
        &mut curgroup.ipv6mr_multiaddr as *mut crate::stdlib::in6_addr as *mut libc::c_void,
        &mut addr.sin6_addr as *mut crate::stdlib::in6_addr as *const libc::c_void,
        ::std::mem::size_of::<crate::stdlib::in6_addr>() as libc::c_ulong,
    );
    if *(*net_mcast6iface).string != 0 {
        curgroup.ipv6mr_interface = ::libc::if_nametoindex((*net_mcast6iface).string)
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
    if *(&mut boundto.sin6_addr as *mut crate::stdlib::in6_addr as *const crate::stdlib::uint8_t)
        .offset(0 as i32 as isize) as i32
        == 0xff as i32
        || ({
            let mut __a: *const crate::stdlib::in6_addr = &mut boundto.sin6_addr
                as *mut crate::stdlib::in6_addr
                as *const crate::stdlib::in6_addr;
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
            0 as *mut crate::stdlib::sockaddr_in6,
            &mut err,
        );
        if multicast6_socket == -(1 as i32) {
            // If the OS does not support binding to multicast addresses, like WinXP, at least try with the normal file descriptor.
            multicast6_socket = ip6_socket
        }
    }
    if curgroup.ipv6mr_interface != 0 {
        if ::libc::setsockopt(
            multicast6_socket,
            crate::stdlib::IPPROTO_IPV6 as i32,
            17 as i32,
            &mut curgroup.ipv6mr_interface as *mut u32 as *mut libc::c_char as *const libc::c_void,
            ::std::mem::size_of::<u32>() as libc::c_ulong as crate::stdlib::socklen_t,
        ) < 0 as i32
        {
            crate::src::qcommon::common::Com_Printf(
                b"NET_JoinMulticast6: Couldn\'t set scope on multicast socket: %s\n\x00"
                    as *const u8 as *const libc::c_char,
                NET_ErrorString(),
            );
            if multicast6_socket != ip6_socket {
                ::libc::close(multicast6_socket);
                multicast6_socket = -(1 as i32);
                return;
            }
        }
    }
    if ::libc::setsockopt(
        multicast6_socket,
        crate::stdlib::IPPROTO_IPV6 as i32,
        20 as i32,
        &mut curgroup as *mut crate::stdlib::ipv6_mreq as *mut libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<crate::stdlib::ipv6_mreq>() as libc::c_ulong
            as crate::stdlib::socklen_t,
    ) != 0
    {
        crate::src::qcommon::common::Com_Printf(
            b"NET_JoinMulticast6: Couldn\'t join multicast group: %s\n\x00" as *const u8
                as *const libc::c_char,
            NET_ErrorString(),
        );
        if multicast6_socket != ip6_socket {
            ::libc::close(multicast6_socket);
            multicast6_socket = -(1 as i32);
            return;
        }
    };
}
#[no_mangle]

pub unsafe extern "C" fn NET_LeaveMulticast6() {
    if multicast6_socket != -(1 as i32) {
        if multicast6_socket != ip6_socket {
            ::libc::close(multicast6_socket);
        } else {
            ::libc::setsockopt(
                multicast6_socket,
                crate::stdlib::IPPROTO_IPV6 as i32,
                21 as i32,
                &mut curgroup as *mut crate::stdlib::ipv6_mreq as *mut libc::c_char
                    as *const libc::c_void,
                ::std::mem::size_of::<crate::stdlib::ipv6_mreq>() as libc::c_ulong
                    as crate::stdlib::socklen_t,
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
    let mut address: ::libc::sockaddr_in = ::libc::sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: ::libc::in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut h: *mut ::libc::hostent = 0 as *mut ::libc::hostent;
    let mut len: i32 = 0;
    let mut rfc1929: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut buf: [u8; 64] = [0; 64];
    usingSocks = crate::src::qcommon::q_shared::qfalse;
    crate::src::qcommon::common::Com_Printf(
        b"Opening connection to SOCKS server.\n\x00" as *const u8 as *const libc::c_char,
    );
    socks_socket = ::libc::socket(
        2 as i32,
        crate::stdlib::SOCK_STREAM as i32,
        crate::stdlib::IPPROTO_TCP as i32,
    );
    if socks_socket == -(1 as i32) {
        crate::src::qcommon::common::Com_Printf(
            b"WARNING: NET_OpenSocks: socket: %s\n\x00" as *const u8 as *const libc::c_char,
            NET_ErrorString(),
        );
        return;
    }
    h = crate::stdlib::gethostbyname((*net_socksServer).string);
    if h.is_null() {
        crate::src::qcommon::common::Com_Printf(
            b"WARNING: NET_OpenSocks: gethostbyname: %s\n\x00" as *const u8 as *const libc::c_char,
            NET_ErrorString(),
        );
        return;
    }
    if (*h).h_addrtype != 2 as i32 {
        crate::src::qcommon::common::Com_Printf(
            b"WARNING: NET_OpenSocks: gethostbyname: address type was not AF_INET\n\x00"
                as *const u8 as *const libc::c_char,
        );
        return;
    }
    address.sin_family = 2 as i32 as crate::stdlib::sa_family_t;
    address.sin_addr.s_addr =
        *(*(*h).h_addr_list.offset(0 as i32 as isize) as *mut i32) as crate::stdlib::in_addr_t;
    address.sin_port = __bswap_16((*net_socksPort).integer as i16 as crate::stdlib::__uint16_t);
    if ::libc::connect(
        socks_socket,
        &mut address as *mut ::libc::sockaddr_in as *mut ::libc::sockaddr
            as *const ::libc::sockaddr,
        ::std::mem::size_of::<::libc::sockaddr_in>() as libc::c_ulong as crate::stdlib::socklen_t,
    ) == -(1 as i32)
    {
        crate::src::qcommon::common::Com_Printf(
            b"NET_OpenSocks: connect: %s\n\x00" as *const u8 as *const libc::c_char,
            NET_ErrorString(),
        );
        return;
    }
    // send socks authentication handshake
    if *(*net_socksUsername).string as i32 != 0 || *(*net_socksPassword).string as i32 != 0 {
        rfc1929 = crate::src::qcommon::q_shared::qtrue
    } else {
        rfc1929 = crate::src::qcommon::q_shared::qfalse
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
        len as crate::stddef_h::size_t,
        0 as i32,
    ) == -(1 as i32) as libc::c_long
    {
        crate::src::qcommon::common::Com_Printf(
            b"NET_OpenSocks: send: %s\n\x00" as *const u8 as *const libc::c_char,
            NET_ErrorString(),
        );
        return;
    }
    // get the response
    len = crate::stdlib::recv(
        socks_socket,
        buf.as_mut_ptr() as *mut libc::c_void,
        64 as i32 as crate::stddef_h::size_t,
        0 as i32,
    ) as i32;
    if len == -(1 as i32) {
        crate::src::qcommon::common::Com_Printf(
            b"NET_OpenSocks: recv: %s\n\x00" as *const u8 as *const libc::c_char,
            NET_ErrorString(),
        );
        return;
    }
    if len != 2 as i32 || buf[0 as i32 as usize] as i32 != 5 as i32 {
        crate::src::qcommon::common::Com_Printf(
            b"NET_OpenSocks: bad response\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    match buf[1 as i32 as usize] as i32 {
        0 => {}
        2 => {}
        _ => {
            crate::src::qcommon::common::Com_Printf(
                b"NET_OpenSocks: request denied\n\x00" as *const u8 as *const libc::c_char,
            );
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
                ulen as libc::c_ulong,
            );
        }
        buf[(2 as i32 + ulen) as usize] = plen as u8;
        if plen != 0 {
            crate::stdlib::memcpy(
                &mut *buf.as_mut_ptr().offset((3 as i32 + ulen) as isize) as *mut u8
                    as *mut libc::c_void,
                (*net_socksPassword).string as *const libc::c_void,
                plen as libc::c_ulong,
            );
        }
        // send it
        if crate::stdlib::send(
            socks_socket,
            buf.as_mut_ptr() as *mut libc::c_void,
            (3 as i32 + ulen + plen) as crate::stddef_h::size_t,
            0 as i32,
        ) == -(1 as i32) as libc::c_long
        {
            crate::src::qcommon::common::Com_Printf(
                b"NET_OpenSocks: send: %s\n\x00" as *const u8 as *const libc::c_char,
                NET_ErrorString(),
            );
            return;
        }
        // get the response
        len = crate::stdlib::recv(
            socks_socket,
            buf.as_mut_ptr() as *mut libc::c_void,
            64 as i32 as crate::stddef_h::size_t,
            0 as i32,
        ) as i32;
        if len == -(1 as i32) {
            crate::src::qcommon::common::Com_Printf(
                b"NET_OpenSocks: recv: %s\n\x00" as *const u8 as *const libc::c_char,
                NET_ErrorString(),
            );
            return;
        }
        if len != 2 as i32 || buf[0 as i32 as usize] as i32 != 1 as i32 {
            crate::src::qcommon::common::Com_Printf(
                b"NET_OpenSocks: bad response\n\x00" as *const u8 as *const libc::c_char,
            );
            return;
        }
        if buf[1 as i32 as usize] as i32 != 0 as i32 {
            crate::src::qcommon::common::Com_Printf(
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
        0 as i32 as crate::stdlib::in_addr_t as i32; // port
    *(&mut *buf.as_mut_ptr().offset(8 as i32 as isize) as *mut u8 as *mut i16) =
        __bswap_16(port as i16 as crate::stdlib::__uint16_t) as i16;
    if crate::stdlib::send(
        socks_socket,
        buf.as_mut_ptr() as *mut libc::c_void,
        10 as i32 as crate::stddef_h::size_t,
        0 as i32,
    ) == -(1 as i32) as libc::c_long
    {
        crate::src::qcommon::common::Com_Printf(
            b"NET_OpenSocks: send: %s\n\x00" as *const u8 as *const libc::c_char,
            NET_ErrorString(),
        );
        return;
    }
    // get the response
    len = crate::stdlib::recv(
        socks_socket,
        buf.as_mut_ptr() as *mut libc::c_void,
        64 as i32 as crate::stddef_h::size_t,
        0 as i32,
    ) as i32;
    if len == -(1 as i32) {
        crate::src::qcommon::common::Com_Printf(
            b"NET_OpenSocks: recv: %s\n\x00" as *const u8 as *const libc::c_char,
            NET_ErrorString(),
        );
        return;
    }
    if len < 2 as i32 || buf[0 as i32 as usize] as i32 != 5 as i32 {
        crate::src::qcommon::common::Com_Printf(
            b"NET_OpenSocks: bad response\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    // check completion code
    if buf[1 as i32 as usize] as i32 != 0 as i32 {
        crate::src::qcommon::common::Com_Printf(
            b"NET_OpenSocks: request denied: %i\n\x00" as *const u8 as *const libc::c_char,
            buf[1 as i32 as usize] as i32,
        );
        return;
    }
    if buf[3 as i32 as usize] as i32 != 1 as i32 {
        crate::src::qcommon::common::Com_Printf(
            b"NET_OpenSocks: relay address is not IPV4: %i\n\x00" as *const u8
                as *const libc::c_char,
            buf[3 as i32 as usize] as i32,
        );
        return;
    }
    (*(&mut socksRelayAddr as *mut ::libc::sockaddr as *mut ::libc::sockaddr_in)).sin_family =
        2 as i32 as crate::stdlib::sa_family_t;
    (*(&mut socksRelayAddr as *mut ::libc::sockaddr as *mut ::libc::sockaddr_in))
        .sin_addr
        .s_addr = *(&mut *buf.as_mut_ptr().offset(4 as i32 as isize) as *mut u8 as *mut i32)
        as crate::stdlib::in_addr_t;
    (*(&mut socksRelayAddr as *mut ::libc::sockaddr as *mut ::libc::sockaddr_in)).sin_port =
        *(&mut *buf.as_mut_ptr().offset(8 as i32 as isize) as *mut u8 as *mut i16)
            as crate::stdlib::in_port_t;
    crate::stdlib::memset(
        (*(&mut socksRelayAddr as *mut ::libc::sockaddr as *mut ::libc::sockaddr_in))
            .sin_zero
            .as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        8 as i32 as libc::c_ulong,
    );
    usingSocks = crate::src::qcommon::q_shared::qtrue;
}
/*
=====================
NET_AddLocalAddress
=====================
*/

unsafe extern "C" fn NET_AddLocalAddress(
    mut ifname: *mut libc::c_char,
    mut addr: *mut ::libc::sockaddr,
    mut netmask: *mut ::libc::sockaddr,
) {
    let mut addrlen: i32 = 0;
    let mut family: crate::stdlib::sa_family_t = 0;
    // only add addresses that have all required info.
    if addr.is_null() || netmask.is_null() || ifname.is_null() {
        return;
    }
    family = (*addr).sa_family;
    if numIP < 32 as i32 {
        if family as i32 == 2 as i32 {
            addrlen = ::std::mem::size_of::<::libc::sockaddr_in>() as libc::c_ulong as i32;
            localIP[numIP as usize].type_0 = crate::qcommon_h::NA_IP
        } else if family as i32 == 10 as i32 {
            addrlen = ::std::mem::size_of::<crate::stdlib::sockaddr_in6>() as libc::c_ulong as i32;
            localIP[numIP as usize].type_0 = crate::qcommon_h::NA_IP6
        } else {
            return;
        }
        crate::src::qcommon::q_shared::Q_strncpyz(
            localIP[numIP as usize].ifname.as_mut_ptr(),
            ifname,
            ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as i32,
        );
        localIP[numIP as usize].family = family;
        crate::stdlib::memcpy(
            &mut (*localIP.as_mut_ptr().offset(numIP as isize)).addr
                as *mut crate::stdlib::sockaddr_storage as *mut libc::c_void,
            addr as *const libc::c_void,
            addrlen as libc::c_ulong,
        );
        crate::stdlib::memcpy(
            &mut (*localIP.as_mut_ptr().offset(numIP as isize)).netmask
                as *mut crate::stdlib::sockaddr_storage as *mut libc::c_void,
            netmask as *const libc::c_void,
            addrlen as libc::c_ulong,
        );
        numIP += 1
    };
}

unsafe extern "C" fn NET_GetLocalAddress() {
    let mut ifap: *mut crate::stdlib::ifaddrs = 0 as *mut crate::stdlib::ifaddrs;
    let mut search: *mut crate::stdlib::ifaddrs = 0 as *mut crate::stdlib::ifaddrs;
    numIP = 0 as i32;
    if crate::stdlib::getifaddrs(&mut ifap) != 0 {
        crate::src::qcommon::common::Com_Printf(
            b"NET_GetLocalAddress: Unable to get list of network interfaces: %s\n\x00" as *const u8
                as *const libc::c_char,
            NET_ErrorString(),
        );
    } else {
        search = ifap;
        while !search.is_null() {
            // Only add interfaces that are up.
            if (*ifap).ifa_flags & crate::stdlib::IFF_UP as i32 as u32 != 0 {
                NET_AddLocalAddress(
                    (*search).ifa_name,
                    (*search).ifa_addr,
                    (*search).ifa_netmask,
                );
            }
            search = (*search).ifa_next
        }
        crate::stdlib::freeifaddrs(ifap);
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
                crate::src::qcommon::cvar::Cvar_SetValue(
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
            crate::src::qcommon::common::Com_Printf(
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
                crate::src::qcommon::cvar::Cvar_SetValue(
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
            crate::src::qcommon::common::Com_Printf(
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

unsafe extern "C" fn NET_GetCvars() -> crate::src::qcommon::q_shared::qboolean {
    let mut modified: i32 = 0;
    /* End users have it enabled so they can connect to ipv6-only hosts, but ipv4 will be
     * used if available due to ping */
    net_enabled = crate::src::qcommon::cvar::Cvar_Get(
        b"net_enabled\x00" as *const u8 as *const libc::c_char,
        b"3\x00" as *const u8 as *const libc::c_char,
        0x20 as i32 | 0x1 as i32,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    modified = (*net_enabled).modified as i32;
    (*net_enabled).modified = crate::src::qcommon::q_shared::qfalse;
    net_ip = crate::src::qcommon::cvar::Cvar_Get(
        b"net_ip\x00" as *const u8 as *const libc::c_char,
        b"0.0.0.0\x00" as *const u8 as *const libc::c_char,
        0x20 as i32,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    modified = (modified as u32).wrapping_add((*net_ip).modified as u32) as i32 as i32;
    (*net_ip).modified = crate::src::qcommon::q_shared::qfalse;
    net_ip6 = crate::src::qcommon::cvar::Cvar_Get(
        b"net_ip6\x00" as *const u8 as *const libc::c_char,
        b"::\x00" as *const u8 as *const libc::c_char,
        0x20 as i32,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    modified = (modified as u32).wrapping_add((*net_ip6).modified as u32) as i32 as i32;
    (*net_ip6).modified = crate::src::qcommon::q_shared::qfalse;
    net_port = crate::src::qcommon::cvar::Cvar_Get(
        b"net_port\x00" as *const u8 as *const libc::c_char,
        crate::src::qcommon::q_shared::va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            27960 as i32,
        ),
        0x20 as i32,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    modified = (modified as u32).wrapping_add((*net_port).modified as u32) as i32 as i32;
    (*net_port).modified = crate::src::qcommon::q_shared::qfalse;
    net_port6 = crate::src::qcommon::cvar::Cvar_Get(
        b"net_port6\x00" as *const u8 as *const libc::c_char,
        crate::src::qcommon::q_shared::va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            27960 as i32,
        ),
        0x20 as i32,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    modified = (modified as u32).wrapping_add((*net_port6).modified as u32) as i32 as i32;
    (*net_port6).modified = crate::src::qcommon::q_shared::qfalse;
    // Some cvars for configuring multicast options which facilitates scanning for servers on local subnets.
    net_mcast6addr = crate::src::qcommon::cvar::Cvar_Get(
        b"net_mcast6addr\x00" as *const u8 as *const libc::c_char,
        b"ff04::696f:7175:616b:6533\x00" as *const u8 as *const libc::c_char,
        0x20 as i32 | 0x1 as i32,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    modified = (modified as u32).wrapping_add((*net_mcast6addr).modified as u32) as i32 as i32;
    (*net_mcast6addr).modified = crate::src::qcommon::q_shared::qfalse;
    net_mcast6iface = crate::src::qcommon::cvar::Cvar_Get(
        b"net_mcast6iface\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x20 as i32 | 0x1 as i32,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    modified = (modified as u32).wrapping_add((*net_mcast6iface).modified as u32) as i32 as i32;
    (*net_mcast6iface).modified = crate::src::qcommon::q_shared::qfalse;
    net_socksEnabled = crate::src::qcommon::cvar::Cvar_Get(
        b"net_socksEnabled\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x20 as i32 | 0x1 as i32,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    modified = (modified as u32).wrapping_add((*net_socksEnabled).modified as u32) as i32 as i32;
    (*net_socksEnabled).modified = crate::src::qcommon::q_shared::qfalse;
    net_socksServer = crate::src::qcommon::cvar::Cvar_Get(
        b"net_socksServer\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x20 as i32 | 0x1 as i32,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    modified = (modified as u32).wrapping_add((*net_socksServer).modified as u32) as i32 as i32;
    (*net_socksServer).modified = crate::src::qcommon::q_shared::qfalse;
    net_socksPort = crate::src::qcommon::cvar::Cvar_Get(
        b"net_socksPort\x00" as *const u8 as *const libc::c_char,
        b"1080\x00" as *const u8 as *const libc::c_char,
        0x20 as i32 | 0x1 as i32,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    modified = (modified as u32).wrapping_add((*net_socksPort).modified as u32) as i32 as i32;
    (*net_socksPort).modified = crate::src::qcommon::q_shared::qfalse;
    net_socksUsername = crate::src::qcommon::cvar::Cvar_Get(
        b"net_socksUsername\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x20 as i32 | 0x1 as i32,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    modified = (modified as u32).wrapping_add((*net_socksUsername).modified as u32) as i32 as i32;
    (*net_socksUsername).modified = crate::src::qcommon::q_shared::qfalse;
    net_socksPassword = crate::src::qcommon::cvar::Cvar_Get(
        b"net_socksPassword\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x20 as i32 | 0x1 as i32,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    modified = (modified as u32).wrapping_add((*net_socksPassword).modified as u32) as i32 as i32;
    (*net_socksPassword).modified = crate::src::qcommon::q_shared::qfalse;
    net_dropsim = crate::src::qcommon::cvar::Cvar_Get(
        b"net_dropsim\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x100 as i32,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    return if modified != 0 {
        crate::src::qcommon::q_shared::qtrue as i32
    } else {
        crate::src::qcommon::q_shared::qfalse as i32
    } as crate::src::qcommon::q_shared::qboolean;
}
/*
====================
NET_Config
====================
*/
#[no_mangle]

pub unsafe extern "C" fn NET_Config(mut enableNetworking: crate::src::qcommon::q_shared::qboolean) {
    let mut modified: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut stop: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    let mut start: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    // get any latched changes to cvars
    modified = NET_GetCvars();
    if (*net_enabled).integer == 0 {
        enableNetworking = crate::src::qcommon::q_shared::qfalse
    }
    // if enable state is the same and no cvars were modified, we have nothing to do
    if enableNetworking as u32 == networkingEnabled as u32 && modified as u64 == 0 {
        return;
    }
    if enableNetworking as u32 == networkingEnabled as u32 {
        if enableNetworking as u64 != 0 {
            stop = crate::src::qcommon::q_shared::qtrue;
            start = crate::src::qcommon::q_shared::qtrue
        } else {
            stop = crate::src::qcommon::q_shared::qfalse;
            start = crate::src::qcommon::q_shared::qfalse
        }
    } else {
        if enableNetworking as u64 != 0 {
            stop = crate::src::qcommon::q_shared::qfalse;
            start = crate::src::qcommon::q_shared::qtrue
        } else {
            stop = crate::src::qcommon::q_shared::qtrue;
            start = crate::src::qcommon::q_shared::qfalse
        }
        networkingEnabled = enableNetworking as i32
    }
    if stop as u64 != 0 {
        if ip_socket != -(1 as i32) {
            ::libc::close(ip_socket);
            ip_socket = -(1 as i32)
        }
        if multicast6_socket != -(1 as i32) {
            if multicast6_socket != ip6_socket {
                ::libc::close(multicast6_socket);
            }
            multicast6_socket = -(1 as i32)
        }
        if ip6_socket != -(1 as i32) {
            ::libc::close(ip6_socket);
            ip6_socket = -(1 as i32)
        }
        if socks_socket != -(1 as i32) {
            ::libc::close(socks_socket);
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
    NET_Config(crate::src::qcommon::q_shared::qtrue);
    crate::src::qcommon::cmd::Cmd_AddCommand(
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
    NET_Config(crate::src::qcommon::q_shared::qfalse);
}
/*
====================
NET_Event

Called from NET_Sleep which uses select() to determine which sockets have seen action.
====================
*/
#[no_mangle]

pub unsafe extern "C" fn NET_Event(mut fdr: *mut crate::stdlib::fd_set) {
    let mut bufData: [crate::src::qcommon::q_shared::byte; 16385] = [0; 16385];
    let mut from: crate::qcommon_h::netadr_t = {
        let mut init = crate::qcommon_h::netadr_t {
            type_0: crate::qcommon_h::NA_BAD,
            ip: [0; 4],
            ip6: [0; 16],
            port: 0,
            scope_id: 0,
        };
        init
    };
    let mut netmsg: crate::qcommon_h::msg_t = crate::qcommon_h::msg_t {
        allowoverflow: crate::src::qcommon::q_shared::qfalse,
        overflowed: crate::src::qcommon::q_shared::qfalse,
        oob: crate::src::qcommon::q_shared::qfalse,
        data: 0 as *mut crate::src::qcommon::q_shared::byte,
        maxsize: 0,
        cursize: 0,
        readcount: 0,
        bit: 0,
    };
    loop {
        crate::src::qcommon::msg::MSG_Init(
            &mut netmsg as *mut _ as *mut crate::qcommon_h::msg_t,
            bufData.as_mut_ptr(),
            ::std::mem::size_of::<[crate::src::qcommon::q_shared::byte; 16385]>() as libc::c_ulong
                as i32,
        );
        if !(NET_GetPacket(&mut from, &mut netmsg, fdr) as u64 != 0) {
            break;
        }
        if (*net_dropsim).value > 0.0f32 && (*net_dropsim).value <= 100.0f32 {
            // com_dropsim->value percent of incoming packets get dropped.
            if ::libc::rand()
                < (2147483647 as i32 as f64 / 100.0f64 * (*net_dropsim).value as f64) as i32
            {
                continue;
            }
            // drop this packet
        }
        if (*crate::src::qcommon::common::com_sv_running).integer != 0 {
            crate::src::qcommon::common::Com_RunAndTimeServerPacket(
                &mut from as *mut _ as *mut crate::qcommon_h::netadr_t,
                &mut netmsg as *mut _ as *mut crate::qcommon_h::msg_t,
            );
        } else {
            crate::src::client::cl_main::CL_PacketEvent(
                from as crate::qcommon_h::netadr_t,
                &mut netmsg as *mut _ as *mut crate::qcommon_h::msg_t,
            );
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
    let mut timeout: ::libc::timeval = ::libc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut fdr: crate::stdlib::fd_set = crate::stdlib::fd_set {
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
    let fresh4 = (::std::mem::size_of::<crate::stdlib::fd_set>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<crate::stdlib::__fd_mask>() as libc::c_ulong);
    let fresh5 = &mut *fdr.__fds_bits.as_mut_ptr().offset(0 as i32 as isize)
        as *mut crate::stdlib::__fd_mask;
    asm!("cld; rep; stosq" : "={cx}" (fresh1), "={di}" (fresh3) : "{ax}"
     (0 as i32), "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh4)), "1"
     (c2rust_asm_casts::AsmCast::cast_in(fresh2, fresh5)) : "memory" :
     "volatile");
    c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh4, fresh1);
    c2rust_asm_casts::AsmCast::cast_out(fresh2, fresh5, fresh3);
    if ip_socket != -(1 as i32) {
        fdr.__fds_bits[(ip_socket
            / (8 as i32
                * ::std::mem::size_of::<crate::stdlib::__fd_mask>() as libc::c_ulong as i32))
            as usize] |= ((1 as libc::c_ulong)
            << ip_socket
                % (8 as i32
                    * ::std::mem::size_of::<crate::stdlib::__fd_mask>() as libc::c_ulong as i32))
            as crate::stdlib::__fd_mask;
        highestfd = ip_socket
    }
    if ip6_socket != -(1 as i32) {
        fdr.__fds_bits[(ip6_socket
            / (8 as i32
                * ::std::mem::size_of::<crate::stdlib::__fd_mask>() as libc::c_ulong as i32))
            as usize] |= ((1 as libc::c_ulong)
            << ip6_socket
                % (8 as i32
                    * ::std::mem::size_of::<crate::stdlib::__fd_mask>() as libc::c_ulong as i32))
            as crate::stdlib::__fd_mask;
        if highestfd == -(1 as i32) || ip6_socket > highestfd {
            highestfd = ip6_socket
        }
    }
    timeout.tv_sec = (msec / 1000 as i32) as crate::stdlib::__time_t;
    timeout.tv_usec = (msec % 1000 as i32 * 1000 as i32) as crate::stdlib::__suseconds_t;
    retval = crate::stdlib::select(
        highestfd + 1 as i32,
        &mut fdr,
        0 as *mut crate::stdlib::fd_set,
        0 as *mut crate::stdlib::fd_set,
        &mut timeout,
    );
    if retval == -(1 as i32) {
        crate::src::qcommon::common::Com_Printf(
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
    NET_Config(crate::src::qcommon::q_shared::qtrue);
}
