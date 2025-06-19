pub type glIndex_t = libc::c_uint;
pub type dlight_t = dlight_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlight_s {
    pub origin: crate::src::qcommon::q_shared::vec3_t,
    pub color: crate::src::qcommon::q_shared::vec3_t,
    pub radius: libc::c_float,
    pub transformed: crate::src::qcommon::q_shared::vec3_t,
    pub additive: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trRefEntity_t {
    pub e: crate::tr_types_h::refEntity_t,
    pub axisLength: libc::c_float,
    pub needDlights: crate::src::qcommon::q_shared::qboolean,
    pub lightingCalculated: crate::src::qcommon::q_shared::qboolean,
    pub lightDir: crate::src::qcommon::q_shared::vec3_t,
    pub ambientLight: crate::src::qcommon::q_shared::vec3_t,
    pub ambientLightInt: libc::c_int,
    pub directedLight: crate::src::qcommon::q_shared::vec3_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct orientationr_t {
    pub origin: crate::src::qcommon::q_shared::vec3_t,
    pub axis: [crate::src::qcommon::q_shared::vec3_t; 3],
    pub viewOrigin: crate::src::qcommon::q_shared::vec3_t,
    pub modelMatrix: [libc::c_float; 16],
}
pub const SS_BAD: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 0;
pub const SS_PORTAL: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 1;
pub const SS_ENVIRONMENT: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 2;
pub const SS_OPAQUE: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 3;
pub const SS_DECAL: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 4;
pub const SS_SEE_THROUGH: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 5;
pub const SS_BANNER: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 6;
pub const SS_FOG: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 7;
pub const SS_UNDERWATER: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 8;
pub const SS_BLEND0: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 9;
pub const SS_BLEND1: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 10;
pub const SS_BLEND2: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 11;
pub const SS_BLEND3: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 12;
pub const SS_BLEND6: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 13;
pub const SS_STENCIL_SHADOW: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 14;
pub const SS_ALMOST_NEAREST: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 15;
pub const SS_NEAREST: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 16;
pub type genFunc_t = libc::c_uint;
pub const GF_NONE: genFunc_t = 0;
pub const GF_SIN: genFunc_t = 1;
pub const GF_SQUARE: genFunc_t = 2;
pub const GF_TRIANGLE: genFunc_t = 3;
pub const GF_SAWTOOTH: genFunc_t = 4;
pub const GF_INVERSE_SAWTOOTH: genFunc_t = 5;
pub const GF_NOISE: genFunc_t = 6;
pub type deform_t = libc::c_uint;
pub const DEFORM_NONE: deform_t = 0;
pub const DEFORM_WAVE: deform_t = 1;
pub const DEFORM_NORMALS: deform_t = 2;
pub const DEFORM_BULGE: deform_t = 3;
pub const DEFORM_MOVE: deform_t = 4;
pub const DEFORM_PROJECTION_SHADOW: deform_t = 5;
pub const DEFORM_AUTOSPRITE: deform_t = 6;
pub const DEFORM_AUTOSPRITE2: deform_t = 7;
pub const DEFORM_TEXT0: deform_t = 8;
pub const DEFORM_TEXT1: deform_t = 9;
pub const DEFORM_TEXT2: deform_t = 10;
pub const DEFORM_TEXT3: deform_t = 11;
pub const DEFORM_TEXT4: deform_t = 12;
pub const DEFORM_TEXT5: deform_t = 13;
pub const DEFORM_TEXT6: deform_t = 14;
pub const DEFORM_TEXT7: deform_t = 15;
pub type alphaGen_t = libc::c_uint;
pub const AGEN_IDENTITY: alphaGen_t = 0;
pub const AGEN_SKIP: alphaGen_t = 1;
pub const AGEN_ENTITY: alphaGen_t = 2;
pub const AGEN_ONE_MINUS_ENTITY: alphaGen_t = 3;
pub const AGEN_VERTEX: alphaGen_t = 4;
pub const AGEN_ONE_MINUS_VERTEX: alphaGen_t = 5;
pub const AGEN_LIGHTING_SPECULAR: alphaGen_t = 6;
pub const AGEN_WAVEFORM: alphaGen_t = 7;
pub const AGEN_PORTAL: alphaGen_t = 8;
pub const AGEN_CONST: alphaGen_t = 9;
pub type colorGen_t = libc::c_uint;
pub const CGEN_BAD: colorGen_t = 0;
pub const CGEN_IDENTITY_LIGHTING: colorGen_t = 1;
pub const CGEN_IDENTITY: colorGen_t = 2;
pub const CGEN_ENTITY: colorGen_t = 3;
pub const CGEN_ONE_MINUS_ENTITY: colorGen_t = 4;
pub const CGEN_EXACT_VERTEX: colorGen_t = 5;
pub const CGEN_VERTEX: colorGen_t = 6;
pub const CGEN_ONE_MINUS_VERTEX: colorGen_t = 7;
pub const CGEN_WAVEFORM: colorGen_t = 8;
pub const CGEN_LIGHTING_DIFFUSE: colorGen_t = 9;
pub const CGEN_FOG: colorGen_t = 10;
pub const CGEN_CONST: colorGen_t = 11;
pub type texCoordGen_t = libc::c_uint;
pub const TCGEN_BAD: texCoordGen_t = 0;
pub const TCGEN_IDENTITY: texCoordGen_t = 1;
pub const TCGEN_LIGHTMAP: texCoordGen_t = 2;
pub const TCGEN_TEXTURE: texCoordGen_t = 3;
pub const TCGEN_ENVIRONMENT_MAPPED: texCoordGen_t = 4;
pub const TCGEN_FOG: texCoordGen_t = 5;
pub const TCGEN_VECTOR: texCoordGen_t = 6;
pub type acff_t = libc::c_uint;
pub const ACFF_NONE: acff_t = 0;
pub const ACFF_MODULATE_RGB: acff_t = 1;
pub const ACFF_MODULATE_RGBA: acff_t = 2;
pub const ACFF_MODULATE_ALPHA: acff_t = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct waveForm_t {
    pub func: genFunc_t,
    pub base: libc::c_float,
    pub amplitude: libc::c_float,
    pub phase: libc::c_float,
    pub frequency: libc::c_float,
}
pub type texMod_t = libc::c_uint;
pub const TMOD_NONE: texMod_t = 0;
pub const TMOD_TRANSFORM: texMod_t = 1;
pub const TMOD_TURBULENT: texMod_t = 2;
pub const TMOD_SCROLL: texMod_t = 3;
pub const TMOD_SCALE: texMod_t = 4;
pub const TMOD_STRETCH: texMod_t = 5;
pub const TMOD_ROTATE: texMod_t = 6;
pub const TMOD_ENTITY_TRANSLATE: texMod_t = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct deformStage_t {
    pub deformation: deform_t,
    pub moveVector: crate::src::qcommon::q_shared::vec3_t,
    pub deformationWave: waveForm_t,
    pub deformationSpread: libc::c_float,
    pub bulgeWidth: libc::c_float,
    pub bulgeHeight: libc::c_float,
    pub bulgeSpeed: libc::c_float,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct texModInfo_t {
    pub type_0: texMod_t,
    pub wave: waveForm_t,
    pub matrix: [[libc::c_float; 2]; 2],
    pub translate: [libc::c_float; 2],
    pub scale: [libc::c_float; 2],
    pub scroll: [libc::c_float; 2],
    pub rotateSpeed: libc::c_float,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct textureBundle_t {
    pub image: [*mut crate::tr_common_h::image_t; 8],
    pub numImageAnimations: libc::c_int,
    pub imageAnimationSpeed: libc::c_float,
    pub tcGen: texCoordGen_t,
    pub tcGenVectors: [crate::src::qcommon::q_shared::vec3_t; 2],
    pub numTexMods: libc::c_int,
    pub texMods: *mut texModInfo_t,
    pub videoMapHandle: libc::c_int,
    pub isLightmap: crate::src::qcommon::q_shared::qboolean,
    pub isVideoMap: crate::src::qcommon::q_shared::qboolean,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shaderStage_t {
    pub active: crate::src::qcommon::q_shared::qboolean,
    pub bundle: [textureBundle_t; 2],
    pub rgbWave: waveForm_t,
    pub rgbGen: colorGen_t,
    pub alphaWave: waveForm_t,
    pub alphaGen: alphaGen_t,
    pub constantColor: [crate::src::qcommon::q_shared::byte; 4],
    pub stateBits: libc::c_uint,
    pub adjustColorsForFog: acff_t,
    pub isDetail: crate::src::qcommon::q_shared::qboolean,
}
pub type cullType_t = libc::c_uint;
pub const CT_FRONT_SIDED: cullType_t = 0;
pub const CT_BACK_SIDED: cullType_t = 1;
pub const CT_TWO_SIDED: cullType_t = 2;
pub type fogPass_t = libc::c_uint;
pub const FP_NONE: fogPass_t = 0;
pub const FP_EQUAL: fogPass_t = 1;
pub const FP_LE: fogPass_t = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct skyParms_t {
    pub cloudHeight: libc::c_float,
    pub outerbox: [*mut crate::tr_common_h::image_t; 6],
    pub innerbox: [*mut crate::tr_common_h::image_t; 6],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fogParms_t {
    pub color: crate::src::qcommon::q_shared::vec3_t,
    pub depthForOpaque: libc::c_float,
}
pub type shader_t = shader_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shader_s {
    pub name: [libc::c_char; 64],
    pub lightmapIndex: libc::c_int,
    pub index: libc::c_int,
    pub sortedIndex: libc::c_int,
    pub sort: libc::c_float,
    pub defaultShader: crate::src::qcommon::q_shared::qboolean,
    pub explicitlyDefined: crate::src::qcommon::q_shared::qboolean,
    pub surfaceFlags: libc::c_int,
    pub contentFlags: libc::c_int,
    pub entityMergable: crate::src::qcommon::q_shared::qboolean,
    pub isSky: crate::src::qcommon::q_shared::qboolean,
    pub sky: skyParms_t,
    pub fogParms: fogParms_t,
    pub portalRange: libc::c_float,
    pub multitextureEnv: libc::c_int,
    pub cullType: cullType_t,
    pub polygonOffset: crate::src::qcommon::q_shared::qboolean,
    pub noMipMaps: crate::src::qcommon::q_shared::qboolean,
    pub noPicMip: crate::src::qcommon::q_shared::qboolean,
    pub fogPass: fogPass_t,
    pub needsNormal: crate::src::qcommon::q_shared::qboolean,
    pub needsST1: crate::src::qcommon::q_shared::qboolean,
    pub needsST2: crate::src::qcommon::q_shared::qboolean,
    pub needsColor: crate::src::qcommon::q_shared::qboolean,
    pub numDeforms: libc::c_int,
    pub deforms: [deformStage_t; 3],
    pub numUnfoggedPasses: libc::c_int,
    pub stages: [*mut shaderStage_t; 8],
    pub optimalStageIteratorFunc: Option<unsafe extern "C" fn() -> ()>,
    pub clampTime: libc::c_double,
    pub timeOffset: libc::c_double,
    pub remappedShader: *mut shader_s,
    pub next: *mut shader_s,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trRefdef_t {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub fov_x: libc::c_float,
    pub fov_y: libc::c_float,
    pub vieworg: crate::src::qcommon::q_shared::vec3_t,
    pub viewaxis: [crate::src::qcommon::q_shared::vec3_t; 3],
    pub stereoFrame: crate::tr_types_h::stereoFrame_t,
    pub time: libc::c_int,
    pub rdflags: libc::c_int,
    pub areamask: [crate::src::qcommon::q_shared::byte; 32],
    pub areamaskModified: crate::src::qcommon::q_shared::qboolean,
    pub floatTime: libc::c_double,
    pub text: [[libc::c_char; 32]; 8],
    pub num_entities: libc::c_int,
    pub entities: *mut trRefEntity_t,
    pub num_dlights: libc::c_int,
    pub dlights: *mut dlight_s,
    pub numPolys: libc::c_int,
    pub polys: *mut srfPoly_s,
    pub numDrawSurfs: libc::c_int,
    pub drawSurfs: *mut drawSurf_s,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct skinSurface_t {
    pub name: [libc::c_char; 64],
    pub shader: *mut shader_t,
}
pub type skin_t = skin_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct skin_s {
    pub name: [libc::c_char; 64],
    pub numSurfaces: libc::c_int,
    pub surfaces: *mut skinSurface_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fog_t {
    pub originalBrushNumber: libc::c_int,
    pub bounds: [crate::src::qcommon::q_shared::vec3_t; 2],
    pub colorInt: libc::c_uint,
    pub tcScale: libc::c_float,
    pub parms: fogParms_t,
    pub hasSurface: crate::src::qcommon::q_shared::qboolean,
    pub surface: [libc::c_float; 4],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct viewParms_t {
    pub or: orientationr_t,
    pub world: orientationr_t,
    pub pvsOrigin: crate::src::qcommon::q_shared::vec3_t,
    pub isPortal: crate::src::qcommon::q_shared::qboolean,
    pub isMirror: crate::src::qcommon::q_shared::qboolean,
    pub frameSceneNum: libc::c_int,
    pub frameCount: libc::c_int,
    pub portalPlane: crate::src::qcommon::q_shared::cplane_t,
    pub viewportX: libc::c_int,
    pub viewportY: libc::c_int,
    pub viewportWidth: libc::c_int,
    pub viewportHeight: libc::c_int,
    pub fovX: libc::c_float,
    pub fovY: libc::c_float,
    pub projectionMatrix: [libc::c_float; 16],
    pub frustum: [crate::src::qcommon::q_shared::cplane_t; 4],
    pub visBounds: [crate::src::qcommon::q_shared::vec3_t; 2],
    pub zFar: libc::c_float,
    pub stereoFrame: crate::tr_types_h::stereoFrame_t,
}
pub type surfaceType_t = libc::c_uint;
pub const SF_BAD: surfaceType_t = 0;
pub const SF_SKIP: surfaceType_t = 1;
pub const SF_FACE: surfaceType_t = 2;
pub const SF_GRID: surfaceType_t = 3;
pub const SF_TRIANGLES: surfaceType_t = 4;
pub const SF_POLY: surfaceType_t = 5;
pub const SF_MD3: surfaceType_t = 6;
pub const SF_MDR: surfaceType_t = 7;
pub const SF_IQM: surfaceType_t = 8;
pub const SF_FLARE: surfaceType_t = 9;
pub const SF_ENTITY: surfaceType_t = 10;
pub const SF_NUM_SURFACE_TYPES: surfaceType_t = 11;
pub const SF_MAX: surfaceType_t = 2147483647;
pub type drawSurf_t = drawSurf_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drawSurf_s {
    pub sort: libc::c_uint,
    pub surface: *mut surfaceType_t,
}
pub type srfPoly_t = srfPoly_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srfPoly_s {
    pub surfaceType: surfaceType_t,
    pub hShader: crate::src::qcommon::q_shared::qhandle_t,
    pub fogIndex: libc::c_int,
    pub numVerts: libc::c_int,
    pub verts: *mut crate::tr_types_h::polyVert_t,
}
pub type srfFlare_t = srfFlare_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srfFlare_s {
    pub surfaceType: surfaceType_t,
    pub origin: crate::src::qcommon::q_shared::vec3_t,
    pub normal: crate::src::qcommon::q_shared::vec3_t,
    pub color: crate::src::qcommon::q_shared::vec3_t,
}
pub type srfGridMesh_t = srfGridMesh_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srfGridMesh_s {
    pub surfaceType: surfaceType_t,
    pub dlightBits: libc::c_int,
    pub meshBounds: [crate::src::qcommon::q_shared::vec3_t; 2],
    pub localOrigin: crate::src::qcommon::q_shared::vec3_t,
    pub meshRadius: libc::c_float,
    pub lodOrigin: crate::src::qcommon::q_shared::vec3_t,
    pub lodRadius: libc::c_float,
    pub lodFixed: libc::c_int,
    pub lodStitched: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub widthLodError: *mut libc::c_float,
    pub heightLodError: *mut libc::c_float,
    pub verts: [crate::qfiles_h::drawVert_t; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srfSurfaceFace_t {
    pub surfaceType: surfaceType_t,
    pub plane: crate::src::qcommon::q_shared::cplane_t,
    pub dlightBits: libc::c_int,
    pub numPoints: libc::c_int,
    pub numIndices: libc::c_int,
    pub ofsIndices: libc::c_int,
    pub points: [[libc::c_float; 8]; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srfTriangles_t {
    pub surfaceType: surfaceType_t,
    pub dlightBits: libc::c_int,
    pub bounds: [crate::src::qcommon::q_shared::vec3_t; 2],
    pub localOrigin: crate::src::qcommon::q_shared::vec3_t,
    pub radius: libc::c_float,
    pub numIndexes: libc::c_int,
    pub indexes: *mut libc::c_int,
    pub numVerts: libc::c_int,
    pub verts: *mut crate::qfiles_h::drawVert_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iqmData_t {
    pub num_vertexes: libc::c_int,
    pub num_triangles: libc::c_int,
    pub num_frames: libc::c_int,
    pub num_surfaces: libc::c_int,
    pub num_joints: libc::c_int,
    pub num_poses: libc::c_int,
    pub surfaces: *mut srfIQModel_s,
    pub triangles: *mut libc::c_int,
    pub positions: *mut libc::c_float,
    pub texcoords: *mut libc::c_float,
    pub normals: *mut libc::c_float,
    pub tangents: *mut libc::c_float,
    pub colors: *mut crate::src::qcommon::q_shared::byte,
    pub influences: *mut libc::c_int,
    pub influenceBlendIndexes: *mut crate::src::qcommon::q_shared::byte,
    pub influenceBlendWeights: C2RustUnnamed_119,
    pub blendWeightsType: libc::c_int,
    pub jointNames: *mut libc::c_char,
    pub jointParents: *mut libc::c_int,
    pub jointMats: *mut libc::c_float,
    pub poseMats: *mut libc::c_float,
    pub bounds: *mut libc::c_float,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_119 {
    pub f: *mut libc::c_float,
    pub b: *mut crate::src::qcommon::q_shared::byte,
}
pub type srfIQModel_t = srfIQModel_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srfIQModel_s {
    pub surfaceType: surfaceType_t,
    pub name: [libc::c_char; 64],
    pub shader: *mut shader_t,
    pub data: *mut iqmData_t,
    pub first_vertex: libc::c_int,
    pub num_vertexes: libc::c_int,
    pub first_triangle: libc::c_int,
    pub num_triangles: libc::c_int,
    pub first_influence: libc::c_int,
    pub num_influences: libc::c_int,
}
pub type msurface_t = msurface_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msurface_s {
    pub viewCount: libc::c_int,
    pub shader: *mut shader_s,
    pub fogIndex: libc::c_int,
    pub data: *mut surfaceType_t,
}
pub type mnode_t = mnode_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mnode_s {
    pub contents: libc::c_int,
    pub visframe: libc::c_int,
    pub mins: crate::src::qcommon::q_shared::vec3_t,
    pub maxs: crate::src::qcommon::q_shared::vec3_t,
    pub parent: *mut mnode_s,
    pub plane: *mut crate::src::qcommon::q_shared::cplane_t,
    pub children: [*mut mnode_s; 2],
    pub cluster: libc::c_int,
    pub area: libc::c_int,
    pub firstmarksurface: *mut *mut msurface_t,
    pub nummarksurfaces: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmodel_t {
    pub bounds: [crate::src::qcommon::q_shared::vec3_t; 2],
    pub firstSurface: *mut msurface_t,
    pub numSurfaces: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct world_t {
    pub name: [libc::c_char; 64],
    pub baseName: [libc::c_char; 64],
    pub dataSize: libc::c_int,
    pub numShaders: libc::c_int,
    pub shaders: *mut crate::qfiles_h::dshader_t,
    pub bmodels: *mut bmodel_t,
    pub numplanes: libc::c_int,
    pub planes: *mut crate::src::qcommon::q_shared::cplane_t,
    pub numnodes: libc::c_int,
    pub numDecisionNodes: libc::c_int,
    pub nodes: *mut mnode_t,
    pub numsurfaces: libc::c_int,
    pub surfaces: *mut msurface_t,
    pub nummarksurfaces: libc::c_int,
    pub marksurfaces: *mut *mut msurface_t,
    pub numfogs: libc::c_int,
    pub fogs: *mut fog_t,
    pub lightGridOrigin: crate::src::qcommon::q_shared::vec3_t,
    pub lightGridSize: crate::src::qcommon::q_shared::vec3_t,
    pub lightGridInverseSize: crate::src::qcommon::q_shared::vec3_t,
    pub lightGridBounds: [libc::c_int; 3],
    pub lightGridData: *mut crate::src::qcommon::q_shared::byte,
    pub numClusters: libc::c_int,
    pub clusterBytes: libc::c_int,
    pub vis: *const crate::src::qcommon::q_shared::byte,
    pub novis: *mut crate::src::qcommon::q_shared::byte,
    pub entityString: *mut libc::c_char,
    pub entityParsePoint: *mut libc::c_char,
}
pub type modtype_t = libc::c_uint;
pub const MOD_BAD: modtype_t = 0;
pub const MOD_BRUSH: modtype_t = 1;
pub const MOD_MESH: modtype_t = 2;
pub const MOD_MDR: modtype_t = 3;
pub const MOD_IQM: modtype_t = 4;
pub type model_t = model_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct model_s {
    pub name: [libc::c_char; 64],
    pub type_0: modtype_t,
    pub index: libc::c_int,
    pub dataSize: libc::c_int,
    pub bmodel: *mut bmodel_t,
    pub md3: [*mut crate::qfiles_h::md3Header_t; 3],
    pub modelData: *mut libc::c_void,
    pub numLods: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct frontEndCounters_t {
    pub c_sphere_cull_patch_in: libc::c_int,
    pub c_sphere_cull_patch_clip: libc::c_int,
    pub c_sphere_cull_patch_out: libc::c_int,
    pub c_box_cull_patch_in: libc::c_int,
    pub c_box_cull_patch_clip: libc::c_int,
    pub c_box_cull_patch_out: libc::c_int,
    pub c_sphere_cull_md3_in: libc::c_int,
    pub c_sphere_cull_md3_clip: libc::c_int,
    pub c_sphere_cull_md3_out: libc::c_int,
    pub c_box_cull_md3_in: libc::c_int,
    pub c_box_cull_md3_clip: libc::c_int,
    pub c_box_cull_md3_out: libc::c_int,
    pub c_leafs: libc::c_int,
    pub c_dlightSurfaces: libc::c_int,
    pub c_dlightSurfacesCulled: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct glstate_t {
    pub currenttextures: [libc::c_int; 2],
    pub currenttmu: libc::c_int,
    pub finishCalled: crate::src::qcommon::q_shared::qboolean,
    pub texEnv: [libc::c_int; 2],
    pub faceCulling: libc::c_int,
    pub glStateBits: libc::c_ulong,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct backEndCounters_t {
    pub c_surfaces: libc::c_int,
    pub c_shaders: libc::c_int,
    pub c_vertexes: libc::c_int,
    pub c_indexes: libc::c_int,
    pub c_totalIndexes: libc::c_int,
    pub c_overDraw: libc::c_float,
    pub c_dlightVertexes: libc::c_int,
    pub c_dlightIndexes: libc::c_int,
    pub c_flareAdds: libc::c_int,
    pub c_flareTests: libc::c_int,
    pub c_flareRenders: libc::c_int,
    pub msec: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct backEndState_t {
    pub refdef: trRefdef_t,
    pub viewParms: viewParms_t,
    pub or: orientationr_t,
    pub pc: backEndCounters_t,
    pub isHyperspace: crate::src::qcommon::q_shared::qboolean,
    pub currentEntity: *mut trRefEntity_t,
    pub skyRenderedThisView: crate::src::qcommon::q_shared::qboolean,
    pub projection2D: crate::src::qcommon::q_shared::qboolean,
    pub color2D: [crate::src::qcommon::q_shared::byte; 4],
    pub vertexes2D: crate::src::qcommon::q_shared::qboolean,
    pub entity2D: trRefEntity_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trGlobals_t {
    pub registered: crate::src::qcommon::q_shared::qboolean,
    pub visCount: libc::c_int,
    pub frameCount: libc::c_int,
    pub sceneCount: libc::c_int,
    pub viewCount: libc::c_int,
    pub frameSceneNum: libc::c_int,
    pub worldMapLoaded: crate::src::qcommon::q_shared::qboolean,
    pub world: *mut world_t,
    pub externalVisData: *const crate::src::qcommon::q_shared::byte,
    pub defaultImage: *mut crate::tr_common_h::image_t,
    pub scratchImage: [*mut crate::tr_common_h::image_t; 32],
    pub fogImage: *mut crate::tr_common_h::image_t,
    pub dlightImage: *mut crate::tr_common_h::image_t,
    pub flareImage: *mut crate::tr_common_h::image_t,
    pub whiteImage: *mut crate::tr_common_h::image_t,
    pub identityLightImage: *mut crate::tr_common_h::image_t,
    pub defaultShader: *mut shader_t,
    pub shadowShader: *mut shader_t,
    pub projectionShadowShader: *mut shader_t,
    pub flareShader: *mut shader_t,
    pub sunShader: *mut shader_t,
    pub numLightmaps: libc::c_int,
    pub lightmaps: *mut *mut crate::tr_common_h::image_t,
    pub currentEntity: *mut trRefEntity_t,
    pub worldEntity: trRefEntity_t,
    pub currentEntityNum: libc::c_int,
    pub shiftedEntityNum: libc::c_int,
    pub currentModel: *mut model_t,
    pub viewParms: viewParms_t,
    pub identityLight: libc::c_float,
    pub identityLightByte: libc::c_int,
    pub overbrightBits: libc::c_int,
    pub or: orientationr_t,
    pub refdef: trRefdef_t,
    pub viewCluster: libc::c_int,
    pub sunLight: crate::src::qcommon::q_shared::vec3_t,
    pub sunDirection: crate::src::qcommon::q_shared::vec3_t,
    pub pc: frontEndCounters_t,
    pub frontEndMsec: libc::c_int,
    pub models: [*mut model_t; 1024],
    pub numModels: libc::c_int,
    pub numImages: libc::c_int,
    pub images: [*mut crate::tr_common_h::image_t; 2048],
    pub numShaders: libc::c_int,
    pub shaders: [*mut shader_t; 16384],
    pub sortedShaders: [*mut shader_t; 16384],
    pub numSkins: libc::c_int,
    pub skins: [*mut skin_t; 1024],
    pub sinTable: [libc::c_float; 1024],
    pub squareTable: [libc::c_float; 1024],
    pub triangleTable: [libc::c_float; 1024],
    pub sawToothTable: [libc::c_float; 1024],
    pub inverseSawToothTable: [libc::c_float; 1024],
    pub fogTable: [libc::c_float; 256],
}
pub type color4ub_t = [crate::src::qcommon::q_shared::byte; 4];
pub type stageVars_t = stageVars;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stageVars {
    pub colors: [color4ub_t; 1000],
    pub texcoords: [[crate::src::qcommon::q_shared::vec2_t; 1000]; 2],
}
pub type shaderCommands_t = shaderCommands_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shaderCommands_s {
    pub indexes: [glIndex_t; 6000],
    pub xyz: [crate::src::qcommon::q_shared::vec4_t; 1000],
    pub normal: [crate::src::qcommon::q_shared::vec4_t; 1000],
    pub texCoords: [[crate::src::qcommon::q_shared::vec2_t; 2]; 1000],
    pub vertexColors: [color4ub_t; 1000],
    pub vertexDlightBits: [libc::c_int; 1000],
    pub svars: stageVars_t,
    pub constantColor255: [color4ub_t; 1000],
    pub shader: *mut shader_t,
    pub shaderTime: libc::c_double,
    pub fogNum: libc::c_int,
    pub dlightBits: libc::c_int,
    pub numIndexes: libc::c_int,
    pub numVertexes: libc::c_int,
    pub numPasses: libc::c_int,
    pub currentStageIteratorFunc: Option<unsafe extern "C" fn() -> ()>,
    pub xstages: *mut *mut shaderStage_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct renderCommandList_t {
    pub cmds: [crate::src::qcommon::q_shared::byte; 262144],
    pub used: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct setColorCommand_t {
    pub commandId: libc::c_int,
    pub color: [libc::c_float; 4],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drawBufferCommand_t {
    pub commandId: libc::c_int,
    pub buffer: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct swapBuffersCommand_t {
    pub commandId: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stretchPicCommand_t {
    pub commandId: libc::c_int,
    pub shader: *mut shader_t,
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub w: libc::c_float,
    pub h: libc::c_float,
    pub s1: libc::c_float,
    pub t1: libc::c_float,
    pub s2: libc::c_float,
    pub t2: libc::c_float,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drawSurfsCommand_t {
    pub commandId: libc::c_int,
    pub refdef: trRefdef_t,
    pub viewParms: viewParms_t,
    pub drawSurfs: *mut drawSurf_t,
    pub numDrawSurfs: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct screenshotCommand_t {
    pub commandId: libc::c_int,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub fileName: *mut libc::c_char,
    pub jpeg: crate::src::qcommon::q_shared::qboolean,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct videoFrameCommand_t {
    pub commandId: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub captureBuffer: *mut crate::src::qcommon::q_shared::byte,
    pub encodeBuffer: *mut crate::src::qcommon::q_shared::byte,
    pub motionJpeg: crate::src::qcommon::q_shared::qboolean,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct colorMaskCommand_t {
    pub commandId: libc::c_int,
    pub rgba: [crate::stdlib::GLboolean; 4],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clearDepthCommand_t {
    pub commandId: libc::c_int,
}
pub const RC_END_OF_LIST: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 0;
pub const RC_SET_COLOR: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 1;
pub const RC_STRETCH_PIC: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 2;
pub const RC_DRAW_SURFS: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 3;
pub const RC_DRAW_BUFFER: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 4;
pub const RC_SWAP_BUFFERS: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 5;
pub const RC_SCREENSHOT: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 6;
pub const RC_VIDEOFRAME: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 7;
pub const RC_COLORMASK: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 8;
pub const RC_CLEARDEPTH: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 9;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct backEndData_t {
    pub drawSurfs: [drawSurf_t; 65536],
    pub dlights: [dlight_t; 32],
    pub entities: [trRefEntity_t; 1023],
    pub polys: *mut srfPoly_t,
    pub polyVerts: *mut crate::tr_types_h::polyVert_t,
    pub commands: renderCommandList_t,
}
