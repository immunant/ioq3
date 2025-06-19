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
    pub fraction: f32,
    pub endpos: crate::src::qcommon::q_shared::vec3_t,
    pub ent: i32,
    pub lastarea: i32,
    pub area: i32,
    pub planenum: i32,
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
    pub valid: i32,
    pub type_0: i32,
    pub flags: i32,
    pub ltime: f32,
    pub update_time: f32,
    pub number: i32,
    pub origin: crate::src::qcommon::q_shared::vec3_t,
    pub angles: crate::src::qcommon::q_shared::vec3_t,
    pub old_origin: crate::src::qcommon::q_shared::vec3_t,
    pub lastvisorigin: crate::src::qcommon::q_shared::vec3_t,
    pub mins: crate::src::qcommon::q_shared::vec3_t,
    pub maxs: crate::src::qcommon::q_shared::vec3_t,
    pub groundent: i32,
    pub solid: i32,
    pub modelindex: i32,
    pub modelindex2: i32,
    pub frame: i32,
    pub event: i32,
    pub eventParm: i32,
    pub powerups: i32,
    pub weapon: i32,
    pub legsAnim: i32,
    pub torsoAnim: i32,
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
    pub contents: i32,
    pub flags: i32,
    pub presencetype: i32,
    pub cluster: i32,
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
    pub endarea: i32,
    pub velocity: crate::src::qcommon::q_shared::vec3_t,
    pub trace: aas_trace_t,
    pub presencetype: i32,
    pub stopevent: i32,
    pub endcontents: i32,
    pub time: f32,
    pub frames: i32,
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
    pub areanum: i32,
    pub starttraveltime: u16,
    pub goaltraveltime: u16,
    pub extratraveltime: u16,
}
//stop when entering the given contents

//stop when entering the given area
pub type aas_predictroute_t = aas_predictroute_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aas_predictroute_s {
    pub endpos: crate::src::qcommon::q_shared::vec3_t,
    pub endarea: i32,
    pub stopevent: i32,
    pub endcontents: i32,
    pub endtravelflags: i32,
    pub numareas: i32,
    pub time: i32,
}
