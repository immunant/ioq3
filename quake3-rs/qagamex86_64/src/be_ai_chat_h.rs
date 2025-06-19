pub type bot_consolemessage_t = bot_consolemessage_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_consolemessage_s {
    pub handle: libc::c_int,
    pub time: libc::c_float,
    pub type_0: libc::c_int,
    pub message: [libc::c_char; 256],
    pub prev: *mut bot_consolemessage_s,
    pub next: *mut bot_consolemessage_s,
}
// true if updated this frame

// entity type

// entity flags

// local time

// time between last and current update

// number of the entity

// origin of the entity

// angles of the model

// for lerping

// last visible origin

// bounding box minimums

// bounding box maximums

// ground entity

// solid type

// model used

// weapons, CTF flags, etc

// model frame number

// impulse events -- muzzle flashes, footsteps, etc

// even parameter

// bit flags

// determines weapon and flash model, etc

// mask off ANIM_TOGGLEBIT

// mask off ANIM_TOGGLEBIT

//match variable
pub type bot_matchvariable_t = bot_matchvariable_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_matchvariable_s {
    pub offset: libc::c_char,
    pub length: libc::c_int,
}
//returned to AI when a match is found
pub type bot_match_t = bot_match_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_match_s {
    pub string: [libc::c_char; 256],
    pub type_0: libc::c_int,
    pub subtype: libc::c_int,
    pub variables: [bot_matchvariable_t; 8],
}
