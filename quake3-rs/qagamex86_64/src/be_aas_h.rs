pub const SOLID_NOT: crate::bg_public_h::C2RustUnnamed_0 = 0;
pub const SOLID_TRIGGER: crate::bg_public_h::C2RustUnnamed_0 = 1;
pub const SOLID_BBOX: crate::bg_public_h::C2RustUnnamed_0 = 2;
pub const SOLID_BSP: crate::bg_public_h::C2RustUnnamed_0 = 3;
//a trace is returned when a box is swept through the AAS world
pub type aas_trace_t = aas_trace_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aas_trace_s {
    pub startsolid: crate::src::qcommon::q_shared::qboolean,
    pub fraction: libc::c_float,
    pub endpos: crate::src::qcommon::q_shared::vec3_t,
    pub ent: libc::c_int,
    pub lastarea: libc::c_int,
    pub area: libc::c_int,
    pub planenum: libc::c_int,
}
/* Defined in botlib.h

//bsp_trace_t hit surface
typedef struct bsp_surface_s
{
    char name[16];
    int flags;
    int value;
} bsp_surface_t;

//a trace is returned when a box is swept through the BSP world
typedef struct bsp_trace_s
{
    qboolean		allsolid;	// if true, plane is not valid
    qboolean		startsolid;	// if true, the initial point was in a solid area
    float			fraction;	// time completed, 1.0 = didn't hit anything
    vec3_t			endpos;		// final position
    cplane_t		plane;		// surface normal at impact
    float			exp_dist;	// expanded plane distance
    int				sidenum;	// number of the brush side hit
    bsp_surface_t	surface;	// hit surface
    int				contents;	// contents on other side of surface hit
    int				ent;		// number of entity hit
} bsp_trace_t;
//
*/

//entity info
pub type aas_entityinfo_t = aas_entityinfo_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aas_entityinfo_s {
    pub valid: libc::c_int,
    pub type_0: libc::c_int,
    pub flags: libc::c_int,
    pub ltime: libc::c_float,
    pub update_time: libc::c_float,
    pub number: libc::c_int,
    pub origin: crate::src::qcommon::q_shared::vec3_t,
    pub angles: crate::src::qcommon::q_shared::vec3_t,
    pub old_origin: crate::src::qcommon::q_shared::vec3_t,
    pub lastvisorigin: crate::src::qcommon::q_shared::vec3_t,
    pub mins: crate::src::qcommon::q_shared::vec3_t,
    pub maxs: crate::src::qcommon::q_shared::vec3_t,
    pub groundent: libc::c_int,
    pub solid: libc::c_int,
    pub modelindex: libc::c_int,
    pub modelindex2: libc::c_int,
    pub frame: libc::c_int,
    pub event: libc::c_int,
    pub eventParm: libc::c_int,
    pub powerups: libc::c_int,
    pub weapon: libc::c_int,
    pub legsAnim: libc::c_int,
    pub torsoAnim: libc::c_int,
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

// area info
pub type aas_areainfo_t = aas_areainfo_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aas_areainfo_s {
    pub contents: libc::c_int,
    pub flags: libc::c_int,
    pub presencetype: libc::c_int,
    pub cluster: libc::c_int,
    pub mins: crate::src::qcommon::q_shared::vec3_t,
    pub maxs: crate::src::qcommon::q_shared::vec3_t,
    pub center: crate::src::qcommon::q_shared::vec3_t,
}
// client movement prediction stop events, stop as soon as:

// the ground is hit

// there's no ground

// water is entered

// slime is entered

// lava is entered

// the ground is hit with damage

// there's a gap

// touching a jump pad area

// touching teleporter

// the given stoparea is entered

// a ground face in the area is hit

// hit the specified bounding box

// touching a cluster portal
pub type aas_clientmove_t = aas_clientmove_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aas_clientmove_s {
    pub endpos: crate::src::qcommon::q_shared::vec3_t,
    pub endarea: libc::c_int,
    pub velocity: crate::src::qcommon::q_shared::vec3_t,
    pub trace: aas_trace_t,
    pub presencetype: libc::c_int,
    pub stopevent: libc::c_int,
    pub endcontents: libc::c_int,
    pub time: libc::c_float,
    pub frames: libc::c_int,
}
//position at the end of movement prediction

//area at end of movement prediction

//velocity at the end of movement prediction

//last trace

//presence type at end of movement prediction

//event that made the prediction stop

//contents at the end of movement prediction

//time predicted ahead

//number of frames predicted ahead

// alternate route goals
pub type aas_altroutegoal_t = aas_altroutegoal_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aas_altroutegoal_s {
    pub origin: crate::src::qcommon::q_shared::vec3_t,
    pub areanum: libc::c_int,
    pub starttraveltime: libc::c_ushort,
    pub goaltraveltime: libc::c_ushort,
    pub extratraveltime: libc::c_ushort,
}
//stop when entering the given contents

//stop when entering the given area
pub type aas_predictroute_t = aas_predictroute_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aas_predictroute_s {
    pub endpos: crate::src::qcommon::q_shared::vec3_t,
    pub endarea: libc::c_int,
    pub stopevent: libc::c_int,
    pub endcontents: libc::c_int,
    pub endtravelflags: libc::c_int,
    pub numareas: libc::c_int,
    pub time: libc::c_int,
}
