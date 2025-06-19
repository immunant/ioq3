use ::libc;

pub use crate::src::qcommon::q_shared::byte;
#[no_mangle]

pub static mut crctable: [u16; 257] = [
    0 as i32 as u16,
    0x1021 as i32 as u16,
    0x2042 as i32 as u16,
    0x3063 as i32 as u16,
    0x4084 as i32 as u16,
    0x50a5 as i32 as u16,
    0x60c6 as i32 as u16,
    0x70e7 as i32 as u16,
    0x8108 as i32 as u16,
    0x9129 as i32 as u16,
    0xa14a as i32 as u16,
    0xb16b as i32 as u16,
    0xc18c as i32 as u16,
    0xd1ad as i32 as u16,
    0xe1ce as i32 as u16,
    0xf1ef as i32 as u16,
    0x1231 as i32 as u16,
    0x210 as i32 as u16,
    0x3273 as i32 as u16,
    0x2252 as i32 as u16,
    0x52b5 as i32 as u16,
    0x4294 as i32 as u16,
    0x72f7 as i32 as u16,
    0x62d6 as i32 as u16,
    0x9339 as i32 as u16,
    0x8318 as i32 as u16,
    0xb37b as i32 as u16,
    0xa35a as i32 as u16,
    0xd3bd as i32 as u16,
    0xc39c as i32 as u16,
    0xf3ff as i32 as u16,
    0xe3de as i32 as u16,
    0x2462 as i32 as u16,
    0x3443 as i32 as u16,
    0x420 as i32 as u16,
    0x1401 as i32 as u16,
    0x64e6 as i32 as u16,
    0x74c7 as i32 as u16,
    0x44a4 as i32 as u16,
    0x5485 as i32 as u16,
    0xa56a as i32 as u16,
    0xb54b as i32 as u16,
    0x8528 as i32 as u16,
    0x9509 as i32 as u16,
    0xe5ee as i32 as u16,
    0xf5cf as i32 as u16,
    0xc5ac as i32 as u16,
    0xd58d as i32 as u16,
    0x3653 as i32 as u16,
    0x2672 as i32 as u16,
    0x1611 as i32 as u16,
    0x630 as i32 as u16,
    0x76d7 as i32 as u16,
    0x66f6 as i32 as u16,
    0x5695 as i32 as u16,
    0x46b4 as i32 as u16,
    0xb75b as i32 as u16,
    0xa77a as i32 as u16,
    0x9719 as i32 as u16,
    0x8738 as i32 as u16,
    0xf7df as i32 as u16,
    0xe7fe as i32 as u16,
    0xd79d as i32 as u16,
    0xc7bc as i32 as u16,
    0x48c4 as i32 as u16,
    0x58e5 as i32 as u16,
    0x6886 as i32 as u16,
    0x78a7 as i32 as u16,
    0x840 as i32 as u16,
    0x1861 as i32 as u16,
    0x2802 as i32 as u16,
    0x3823 as i32 as u16,
    0xc9cc as i32 as u16,
    0xd9ed as i32 as u16,
    0xe98e as i32 as u16,
    0xf9af as i32 as u16,
    0x8948 as i32 as u16,
    0x9969 as i32 as u16,
    0xa90a as i32 as u16,
    0xb92b as i32 as u16,
    0x5af5 as i32 as u16,
    0x4ad4 as i32 as u16,
    0x7ab7 as i32 as u16,
    0x6a96 as i32 as u16,
    0x1a71 as i32 as u16,
    0xa50 as i32 as u16,
    0x3a33 as i32 as u16,
    0x2a12 as i32 as u16,
    0xdbfd as i32 as u16,
    0xcbdc as i32 as u16,
    0xfbbf as i32 as u16,
    0xeb9e as i32 as u16,
    0x9b79 as i32 as u16,
    0x8b58 as i32 as u16,
    0xbb3b as i32 as u16,
    0xab1a as i32 as u16,
    0x6ca6 as i32 as u16,
    0x7c87 as i32 as u16,
    0x4ce4 as i32 as u16,
    0x5cc5 as i32 as u16,
    0x2c22 as i32 as u16,
    0x3c03 as i32 as u16,
    0xc60 as i32 as u16,
    0x1c41 as i32 as u16,
    0xedae as i32 as u16,
    0xfd8f as i32 as u16,
    0xcdec as i32 as u16,
    0xddcd as i32 as u16,
    0xad2a as i32 as u16,
    0xbd0b as i32 as u16,
    0x8d68 as i32 as u16,
    0x9d49 as i32 as u16,
    0x7e97 as i32 as u16,
    0x6eb6 as i32 as u16,
    0x5ed5 as i32 as u16,
    0x4ef4 as i32 as u16,
    0x3e13 as i32 as u16,
    0x2e32 as i32 as u16,
    0x1e51 as i32 as u16,
    0xe70 as i32 as u16,
    0xff9f as i32 as u16,
    0xefbe as i32 as u16,
    0xdfdd as i32 as u16,
    0xcffc as i32 as u16,
    0xbf1b as i32 as u16,
    0xaf3a as i32 as u16,
    0x9f59 as i32 as u16,
    0x8f78 as i32 as u16,
    0x9188 as i32 as u16,
    0x81a9 as i32 as u16,
    0xb1ca as i32 as u16,
    0xa1eb as i32 as u16,
    0xd10c as i32 as u16,
    0xc12d as i32 as u16,
    0xf14e as i32 as u16,
    0xe16f as i32 as u16,
    0x1080 as i32 as u16,
    0xa1 as i32 as u16,
    0x30c2 as i32 as u16,
    0x20e3 as i32 as u16,
    0x5004 as i32 as u16,
    0x4025 as i32 as u16,
    0x7046 as i32 as u16,
    0x6067 as i32 as u16,
    0x83b9 as i32 as u16,
    0x9398 as i32 as u16,
    0xa3fb as i32 as u16,
    0xb3da as i32 as u16,
    0xc33d as i32 as u16,
    0xd31c as i32 as u16,
    0xe37f as i32 as u16,
    0xf35e as i32 as u16,
    0x2b1 as i32 as u16,
    0x1290 as i32 as u16,
    0x22f3 as i32 as u16,
    0x32d2 as i32 as u16,
    0x4235 as i32 as u16,
    0x5214 as i32 as u16,
    0x6277 as i32 as u16,
    0x7256 as i32 as u16,
    0xb5ea as i32 as u16,
    0xa5cb as i32 as u16,
    0x95a8 as i32 as u16,
    0x8589 as i32 as u16,
    0xf56e as i32 as u16,
    0xe54f as i32 as u16,
    0xd52c as i32 as u16,
    0xc50d as i32 as u16,
    0x34e2 as i32 as u16,
    0x24c3 as i32 as u16,
    0x14a0 as i32 as u16,
    0x481 as i32 as u16,
    0x7466 as i32 as u16,
    0x6447 as i32 as u16,
    0x5424 as i32 as u16,
    0x4405 as i32 as u16,
    0xa7db as i32 as u16,
    0xb7fa as i32 as u16,
    0x8799 as i32 as u16,
    0x97b8 as i32 as u16,
    0xe75f as i32 as u16,
    0xf77e as i32 as u16,
    0xc71d as i32 as u16,
    0xd73c as i32 as u16,
    0x26d3 as i32 as u16,
    0x36f2 as i32 as u16,
    0x691 as i32 as u16,
    0x16b0 as i32 as u16,
    0x6657 as i32 as u16,
    0x7676 as i32 as u16,
    0x4615 as i32 as u16,
    0x5634 as i32 as u16,
    0xd94c as i32 as u16,
    0xc96d as i32 as u16,
    0xf90e as i32 as u16,
    0xe92f as i32 as u16,
    0x99c8 as i32 as u16,
    0x89e9 as i32 as u16,
    0xb98a as i32 as u16,
    0xa9ab as i32 as u16,
    0x5844 as i32 as u16,
    0x4865 as i32 as u16,
    0x7806 as i32 as u16,
    0x6827 as i32 as u16,
    0x18c0 as i32 as u16,
    0x8e1 as i32 as u16,
    0x3882 as i32 as u16,
    0x28a3 as i32 as u16,
    0xcb7d as i32 as u16,
    0xdb5c as i32 as u16,
    0xeb3f as i32 as u16,
    0xfb1e as i32 as u16,
    0x8bf9 as i32 as u16,
    0x9bd8 as i32 as u16,
    0xabbb as i32 as u16,
    0xbb9a as i32 as u16,
    0x4a75 as i32 as u16,
    0x5a54 as i32 as u16,
    0x6a37 as i32 as u16,
    0x7a16 as i32 as u16,
    0xaf1 as i32 as u16,
    0x1ad0 as i32 as u16,
    0x2ab3 as i32 as u16,
    0x3a92 as i32 as u16,
    0xfd2e as i32 as u16,
    0xed0f as i32 as u16,
    0xdd6c as i32 as u16,
    0xcd4d as i32 as u16,
    0xbdaa as i32 as u16,
    0xad8b as i32 as u16,
    0x9de8 as i32 as u16,
    0x8dc9 as i32 as u16,
    0x7c26 as i32 as u16,
    0x6c07 as i32 as u16,
    0x5c64 as i32 as u16,
    0x4c45 as i32 as u16,
    0x3ca2 as i32 as u16,
    0x2c83 as i32 as u16,
    0x1ce0 as i32 as u16,
    0xcc1 as i32 as u16,
    0xef1f as i32 as u16,
    0xff3e as i32 as u16,
    0xcf5d as i32 as u16,
    0xdf7c as i32 as u16,
    0xaf9b as i32 as u16,
    0xbfba as i32 as u16,
    0x8fd9 as i32 as u16,
    0x9ff8 as i32 as u16,
    0x6e17 as i32 as u16,
    0x7e36 as i32 as u16,
    0x4e55 as i32 as u16,
    0x5e74 as i32 as u16,
    0x2e93 as i32 as u16,
    0x3eb2 as i32 as u16,
    0xed1 as i32 as u16,
    0x1ef0 as i32 as u16,
    0,
];
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn CRC_Init(mut crcvalue: *mut u16) {
    *crcvalue = 0xffff as i32 as u16;
}
//end of the function CRC_Init
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn CRC_ProcessByte(
    mut crcvalue: *mut u16,
    mut data: crate::src::qcommon::q_shared::byte,
) {
    *crcvalue = ((*crcvalue as i32) << 8 as i32
        ^ crctable[(*crcvalue as i32 >> 8 as i32 ^ data as i32) as usize]
            as i32) as u16;
}
//end of the function CRC_ProcessByte
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn CRC_Value(mut crcvalue: u16) -> u16 {
    return (crcvalue as i32 ^ 0 as i32) as u16;
}
//end of the function CRC_Value
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn CRC_ProcessString(
    mut data: *mut u8,
    mut length: i32,
) -> u16 {
    let mut crcvalue: u16 = 0; //end for
    let mut i: i32 = 0;
    let mut ind: i32 = 0;
    CRC_Init(&mut crcvalue);
    i = 0 as i32;
    while i < length {
        ind = crcvalue as i32 >> 8 as i32 ^ *data.offset(i as isize) as i32;
        if ind < 0 as i32 || ind > 256 as i32 {
            ind = 0 as i32
        }
        crcvalue = ((crcvalue as i32) << 8 as i32
            ^ crctable[ind as usize] as i32) as u16;
        i += 1
    }
    return CRC_Value(crcvalue);
}
//end of the function CRC_ProcessString
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn CRC_ContinueProcessString(
    mut crc: *mut u16,
    mut data: *mut libc::c_char,
    mut length: i32,
) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < length {
        *crc = ((*crc as i32) << 8 as i32
            ^ crctable[(*crc as i32 >> 8 as i32
                ^ *data.offset(i as isize) as i32) as usize] as i32)
            as u16;
        i += 1
    }
    //end for
}
//end of the function CRC_ProcessString
