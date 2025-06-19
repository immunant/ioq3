#[repr(C)]
#[derive(Copy, Clone)]
pub struct polyVert_t {
    pub xyz: crate::src::qcommon::q_shared::vec3_t,
    pub st: [libc::c_float; 2],
    pub modulate: [crate::src::qcommon::q_shared::byte; 4],
}
pub type refEntityType_t = libc::c_uint;
pub const RT_MODEL: refEntityType_t = 0;
pub const RT_POLY: refEntityType_t = 1;
pub const RT_SPRITE: refEntityType_t = 2;
pub const RT_BEAM: refEntityType_t = 3;
pub const RT_RAIL_CORE: refEntityType_t = 4;
pub const RT_RAIL_RINGS: refEntityType_t = 5;
pub const RT_LIGHTNING: refEntityType_t = 6;
pub const RT_PORTALSURFACE: refEntityType_t = 7;
pub const RT_MAX_REF_ENTITY_TYPE: refEntityType_t = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct refEntity_t {
    pub reType: refEntityType_t,
    pub renderfx: libc::c_int,
    pub hModel: crate::src::qcommon::q_shared::qhandle_t,
    pub lightingOrigin: crate::src::qcommon::q_shared::vec3_t,
    pub shadowPlane: libc::c_float,
    pub axis: [crate::src::qcommon::q_shared::vec3_t; 3],
    pub nonNormalizedAxes: crate::src::qcommon::q_shared::qboolean,
    pub origin: [libc::c_float; 3],
    pub frame: libc::c_int,
    pub oldorigin: [libc::c_float; 3],
    pub oldframe: libc::c_int,
    pub backlerp: libc::c_float,
    pub skinNum: libc::c_int,
    pub customSkin: crate::src::qcommon::q_shared::qhandle_t,
    pub customShader: crate::src::qcommon::q_shared::qhandle_t,
    pub shaderRGBA: [crate::src::qcommon::q_shared::byte; 4],
    pub shaderTexCoord: [libc::c_float; 2],
    pub shaderTime: libc::c_float,
    pub radius: libc::c_float,
    pub rotation: libc::c_float,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct refdef_t {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub fov_x: libc::c_float,
    pub fov_y: libc::c_float,
    pub vieworg: crate::src::qcommon::q_shared::vec3_t,
    pub viewaxis: [crate::src::qcommon::q_shared::vec3_t; 3],
    pub time: libc::c_int,
    pub rdflags: libc::c_int,
    pub areamask: [crate::src::qcommon::q_shared::byte; 32],
    pub text: [[libc::c_char; 32]; 8],
}
pub type stereoFrame_t = libc::c_uint;
pub const STEREO_CENTER: stereoFrame_t = 0;
pub const STEREO_LEFT: stereoFrame_t = 1;
pub const STEREO_RIGHT: stereoFrame_t = 2;
pub type textureCompression_t = libc::c_uint;
pub const TC_NONE: textureCompression_t = 0;
pub const TC_S3TC: textureCompression_t = 1;
pub const TC_S3TC_ARB: textureCompression_t = 2;
pub type glDriverType_t = libc::c_uint;
pub const GLDRV_ICD: glDriverType_t = 0;
pub const GLDRV_STANDALONE: glDriverType_t = 1;
pub const GLDRV_VOODOO: glDriverType_t = 2;
pub type glHardwareType_t = libc::c_uint;
pub const GLHW_GENERIC: glHardwareType_t = 0;
pub const GLHW_3DFX_2D3D: glHardwareType_t = 1;
pub const GLHW_RIVA128: glHardwareType_t = 2;
pub const GLHW_RAGEPRO: glHardwareType_t = 3;
pub const GLHW_PERMEDIA2: glHardwareType_t = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct glconfig_t {
    pub renderer_string: [libc::c_char; 1024],
    pub vendor_string: [libc::c_char; 1024],
    pub version_string: [libc::c_char; 1024],
    pub extensions_string: [libc::c_char; 8192],
    pub maxTextureSize: libc::c_int,
    pub numTextureUnits: libc::c_int,
    pub colorBits: libc::c_int,
    pub depthBits: libc::c_int,
    pub stencilBits: libc::c_int,
    pub driverType: glDriverType_t,
    pub hardwareType: glHardwareType_t,
    pub deviceSupportsGamma: crate::src::qcommon::q_shared::qboolean,
    pub textureCompression: textureCompression_t,
    pub textureEnvAddAvailable: crate::src::qcommon::q_shared::qboolean,
    pub vidWidth: libc::c_int,
    pub vidHeight: libc::c_int,
    pub windowAspect: libc::c_float,
    pub displayFrequency: libc::c_int,
    pub isFullscreen: crate::src::qcommon::q_shared::qboolean,
    pub stereoEnabled: crate::src::qcommon::q_shared::qboolean,
    pub smpActive: crate::src::qcommon::q_shared::qboolean,
}
