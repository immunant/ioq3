pub type glIndex_t = u32;
pub type dlight_t = dlight_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlight_s {
    pub origin: crate::src::qcommon::q_shared::vec3_t,
    pub color: crate::src::qcommon::q_shared::vec3_t,
    pub radius: f32,
    pub transformed: crate::src::qcommon::q_shared::vec3_t,
    pub additive: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trRefEntity_t {
    pub e: crate::tr_types_h::refEntity_t,
    pub axisLength: f32,
    pub needDlights: crate::src::qcommon::q_shared::qboolean,
    pub lightingCalculated: crate::src::qcommon::q_shared::qboolean,
    pub lightDir: crate::src::qcommon::q_shared::vec3_t,
    pub ambientLight: crate::src::qcommon::q_shared::vec3_t,
    pub ambientLightInt: i32,
    pub directedLight: crate::src::qcommon::q_shared::vec3_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct orientationr_t {
    pub origin: crate::src::qcommon::q_shared::vec3_t,
    pub axis: [crate::src::qcommon::q_shared::vec3_t; 3],
    pub viewOrigin: crate::src::qcommon::q_shared::vec3_t,
    pub modelMatrix: [f32; 16],
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
pub type genFunc_t = u32;
pub const GF_NONE: genFunc_t = 0;
pub const GF_SIN: genFunc_t = 1;
pub const GF_SQUARE: genFunc_t = 2;
pub const GF_TRIANGLE: genFunc_t = 3;
pub const GF_SAWTOOTH: genFunc_t = 4;
pub const GF_INVERSE_SAWTOOTH: genFunc_t = 5;
pub const GF_NOISE: genFunc_t = 6;
pub type deform_t = u32;
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
pub type alphaGen_t = u32;
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
pub type colorGen_t = u32;
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
pub type texCoordGen_t = u32;
pub const TCGEN_BAD: texCoordGen_t = 0;
pub const TCGEN_IDENTITY: texCoordGen_t = 1;
pub const TCGEN_LIGHTMAP: texCoordGen_t = 2;
pub const TCGEN_TEXTURE: texCoordGen_t = 3;
pub const TCGEN_ENVIRONMENT_MAPPED: texCoordGen_t = 4;
pub const TCGEN_FOG: texCoordGen_t = 5;
pub const TCGEN_VECTOR: texCoordGen_t = 6;
pub type acff_t = u32;
pub const ACFF_NONE: acff_t = 0;
pub const ACFF_MODULATE_RGB: acff_t = 1;
pub const ACFF_MODULATE_RGBA: acff_t = 2;
pub const ACFF_MODULATE_ALPHA: acff_t = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct waveForm_t {
    pub func: genFunc_t,
    pub base: f32,
    pub amplitude: f32,
    pub phase: f32,
    pub frequency: f32,
}
pub type texMod_t = u32;
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
    pub deformationSpread: f32,
    pub bulgeWidth: f32,
    pub bulgeHeight: f32,
    pub bulgeSpeed: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct texModInfo_t {
    pub type_0: texMod_t,
    pub wave: waveForm_t,
    pub matrix: [[f32; 2]; 2],
    pub translate: [f32; 2],
    pub scale: [f32; 2],
    pub scroll: [f32; 2],
    pub rotateSpeed: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct textureBundle_t {
    pub image: [*mut crate::tr_common_h::image_t; 8],
    pub numImageAnimations: i32,
    pub imageAnimationSpeed: f32,
    pub tcGen: texCoordGen_t,
    pub tcGenVectors: [crate::src::qcommon::q_shared::vec3_t; 2],
    pub numTexMods: i32,
    pub texMods: *mut texModInfo_t,
    pub videoMapHandle: i32,
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
    pub stateBits: u32,
    pub adjustColorsForFog: acff_t,
    pub isDetail: crate::src::qcommon::q_shared::qboolean,
}
pub type cullType_t = u32;
pub const CT_FRONT_SIDED: cullType_t = 0;
pub const CT_BACK_SIDED: cullType_t = 1;
pub const CT_TWO_SIDED: cullType_t = 2;
pub type fogPass_t = u32;
pub const FP_NONE: fogPass_t = 0;
pub const FP_EQUAL: fogPass_t = 1;
pub const FP_LE: fogPass_t = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct skyParms_t {
    pub cloudHeight: f32,
    pub outerbox: [*mut crate::tr_common_h::image_t; 6],
    pub innerbox: [*mut crate::tr_common_h::image_t; 6],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fogParms_t {
    pub color: crate::src::qcommon::q_shared::vec3_t,
    pub depthForOpaque: f32,
}
pub type shader_t = shader_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shader_s {
    pub name: [libc::c_char; 64],
    pub lightmapIndex: i32,
    pub index: i32,
    pub sortedIndex: i32,
    pub sort: f32,
    pub defaultShader: crate::src::qcommon::q_shared::qboolean,
    pub explicitlyDefined: crate::src::qcommon::q_shared::qboolean,
    pub surfaceFlags: i32,
    pub contentFlags: i32,
    pub entityMergable: crate::src::qcommon::q_shared::qboolean,
    pub isSky: crate::src::qcommon::q_shared::qboolean,
    pub sky: skyParms_t,
    pub fogParms: fogParms_t,
    pub portalRange: f32,
    pub multitextureEnv: i32,
    pub cullType: cullType_t,
    pub polygonOffset: crate::src::qcommon::q_shared::qboolean,
    pub noMipMaps: crate::src::qcommon::q_shared::qboolean,
    pub noPicMip: crate::src::qcommon::q_shared::qboolean,
    pub fogPass: fogPass_t,
    pub needsNormal: crate::src::qcommon::q_shared::qboolean,
    pub needsST1: crate::src::qcommon::q_shared::qboolean,
    pub needsST2: crate::src::qcommon::q_shared::qboolean,
    pub needsColor: crate::src::qcommon::q_shared::qboolean,
    pub numDeforms: i32,
    pub deforms: [deformStage_t; 3],
    pub numUnfoggedPasses: i32,
    pub stages: [*mut shaderStage_t; 8],
    pub optimalStageIteratorFunc: Option<unsafe extern "C" fn() -> ()>,
    pub clampTime: f64,
    pub timeOffset: f64,
    pub remappedShader: *mut shader_s,
    pub next: *mut shader_s,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trRefdef_t {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub fov_x: f32,
    pub fov_y: f32,
    pub vieworg: crate::src::qcommon::q_shared::vec3_t,
    pub viewaxis: [crate::src::qcommon::q_shared::vec3_t; 3],
    pub stereoFrame: crate::tr_types_h::stereoFrame_t,
    pub time: i32,
    pub rdflags: i32,
    pub areamask: [crate::src::qcommon::q_shared::byte; 32],
    pub areamaskModified: crate::src::qcommon::q_shared::qboolean,
    pub floatTime: f64,
    pub text: [[libc::c_char; 32]; 8],
    pub num_entities: i32,
    pub entities: *mut trRefEntity_t,
    pub num_dlights: i32,
    pub dlights: *mut dlight_s,
    pub numPolys: i32,
    pub polys: *mut srfPoly_s,
    pub numDrawSurfs: i32,
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
    pub numSurfaces: i32,
    pub surfaces: *mut skinSurface_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fog_t {
    pub originalBrushNumber: i32,
    pub bounds: [crate::src::qcommon::q_shared::vec3_t; 2],
    pub colorInt: u32,
    pub tcScale: f32,
    pub parms: fogParms_t,
    pub hasSurface: crate::src::qcommon::q_shared::qboolean,
    pub surface: [f32; 4],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct viewParms_t {
    pub or: orientationr_t,
    pub world: orientationr_t,
    pub pvsOrigin: crate::src::qcommon::q_shared::vec3_t,
    pub isPortal: crate::src::qcommon::q_shared::qboolean,
    pub isMirror: crate::src::qcommon::q_shared::qboolean,
    pub frameSceneNum: i32,
    pub frameCount: i32,
    pub portalPlane: crate::src::qcommon::q_shared::cplane_t,
    pub viewportX: i32,
    pub viewportY: i32,
    pub viewportWidth: i32,
    pub viewportHeight: i32,
    pub fovX: f32,
    pub fovY: f32,
    pub projectionMatrix: [f32; 16],
    pub frustum: [crate::src::qcommon::q_shared::cplane_t; 4],
    pub visBounds: [crate::src::qcommon::q_shared::vec3_t; 2],
    pub zFar: f32,
    pub stereoFrame: crate::tr_types_h::stereoFrame_t,
}
pub type surfaceType_t = u32;
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
    pub sort: u32,
    pub surface: *mut surfaceType_t,
}
pub type srfPoly_t = srfPoly_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srfPoly_s {
    pub surfaceType: surfaceType_t,
    pub hShader: crate::src::qcommon::q_shared::qhandle_t,
    pub fogIndex: i32,
    pub numVerts: i32,
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
    pub dlightBits: i32,
    pub meshBounds: [crate::src::qcommon::q_shared::vec3_t; 2],
    pub localOrigin: crate::src::qcommon::q_shared::vec3_t,
    pub meshRadius: f32,
    pub lodOrigin: crate::src::qcommon::q_shared::vec3_t,
    pub lodRadius: f32,
    pub lodFixed: i32,
    pub lodStitched: i32,
    pub width: i32,
    pub height: i32,
    pub widthLodError: *mut f32,
    pub heightLodError: *mut f32,
    pub verts: [crate::qfiles_h::drawVert_t; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srfSurfaceFace_t {
    pub surfaceType: surfaceType_t,
    pub plane: crate::src::qcommon::q_shared::cplane_t,
    pub dlightBits: i32,
    pub numPoints: i32,
    pub numIndices: i32,
    pub ofsIndices: i32,
    pub points: [[f32; 8]; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srfTriangles_t {
    pub surfaceType: surfaceType_t,
    pub dlightBits: i32,
    pub bounds: [crate::src::qcommon::q_shared::vec3_t; 2],
    pub localOrigin: crate::src::qcommon::q_shared::vec3_t,
    pub radius: f32,
    pub numIndexes: i32,
    pub indexes: *mut i32,
    pub numVerts: i32,
    pub verts: *mut crate::qfiles_h::drawVert_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iqmData_t {
    pub num_vertexes: i32,
    pub num_triangles: i32,
    pub num_frames: i32,
    pub num_surfaces: i32,
    pub num_joints: i32,
    pub num_poses: i32,
    pub surfaces: *mut srfIQModel_s,
    pub triangles: *mut i32,
    pub positions: *mut f32,
    pub texcoords: *mut f32,
    pub normals: *mut f32,
    pub tangents: *mut f32,
    pub colors: *mut crate::src::qcommon::q_shared::byte,
    pub influences: *mut i32,
    pub influenceBlendIndexes: *mut crate::src::qcommon::q_shared::byte,
    pub influenceBlendWeights: C2RustUnnamed_119,
    pub blendWeightsType: i32,
    pub jointNames: *mut libc::c_char,
    pub jointParents: *mut i32,
    pub jointMats: *mut f32,
    pub poseMats: *mut f32,
    pub bounds: *mut f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_119 {
    pub f: *mut f32,
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
    pub first_vertex: i32,
    pub num_vertexes: i32,
    pub first_triangle: i32,
    pub num_triangles: i32,
    pub first_influence: i32,
    pub num_influences: i32,
}
pub type msurface_t = msurface_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msurface_s {
    pub viewCount: i32,
    pub shader: *mut shader_s,
    pub fogIndex: i32,
    pub data: *mut surfaceType_t,
}
pub type mnode_t = mnode_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mnode_s {
    pub contents: i32,
    pub visframe: i32,
    pub mins: crate::src::qcommon::q_shared::vec3_t,
    pub maxs: crate::src::qcommon::q_shared::vec3_t,
    pub parent: *mut mnode_s,
    pub plane: *mut crate::src::qcommon::q_shared::cplane_t,
    pub children: [*mut mnode_s; 2],
    pub cluster: i32,
    pub area: i32,
    pub firstmarksurface: *mut *mut msurface_t,
    pub nummarksurfaces: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmodel_t {
    pub bounds: [crate::src::qcommon::q_shared::vec3_t; 2],
    pub firstSurface: *mut msurface_t,
    pub numSurfaces: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct world_t {
    pub name: [libc::c_char; 64],
    pub baseName: [libc::c_char; 64],
    pub dataSize: i32,
    pub numShaders: i32,
    pub shaders: *mut crate::qfiles_h::dshader_t,
    pub bmodels: *mut bmodel_t,
    pub numplanes: i32,
    pub planes: *mut crate::src::qcommon::q_shared::cplane_t,
    pub numnodes: i32,
    pub numDecisionNodes: i32,
    pub nodes: *mut mnode_t,
    pub numsurfaces: i32,
    pub surfaces: *mut msurface_t,
    pub nummarksurfaces: i32,
    pub marksurfaces: *mut *mut msurface_t,
    pub numfogs: i32,
    pub fogs: *mut fog_t,
    pub lightGridOrigin: crate::src::qcommon::q_shared::vec3_t,
    pub lightGridSize: crate::src::qcommon::q_shared::vec3_t,
    pub lightGridInverseSize: crate::src::qcommon::q_shared::vec3_t,
    pub lightGridBounds: [i32; 3],
    pub lightGridData: *mut crate::src::qcommon::q_shared::byte,
    pub numClusters: i32,
    pub clusterBytes: i32,
    pub vis: *const crate::src::qcommon::q_shared::byte,
    pub novis: *mut crate::src::qcommon::q_shared::byte,
    pub entityString: *mut libc::c_char,
    pub entityParsePoint: *mut libc::c_char,
}
pub type modtype_t = u32;
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
    pub index: i32,
    pub dataSize: i32,
    pub bmodel: *mut bmodel_t,
    pub md3: [*mut crate::qfiles_h::md3Header_t; 3],
    pub modelData: *mut libc::c_void,
    pub numLods: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct frontEndCounters_t {
    pub c_sphere_cull_patch_in: i32,
    pub c_sphere_cull_patch_clip: i32,
    pub c_sphere_cull_patch_out: i32,
    pub c_box_cull_patch_in: i32,
    pub c_box_cull_patch_clip: i32,
    pub c_box_cull_patch_out: i32,
    pub c_sphere_cull_md3_in: i32,
    pub c_sphere_cull_md3_clip: i32,
    pub c_sphere_cull_md3_out: i32,
    pub c_box_cull_md3_in: i32,
    pub c_box_cull_md3_clip: i32,
    pub c_box_cull_md3_out: i32,
    pub c_leafs: i32,
    pub c_dlightSurfaces: i32,
    pub c_dlightSurfacesCulled: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct glstate_t {
    pub currenttextures: [i32; 2],
    pub currenttmu: i32,
    pub finishCalled: crate::src::qcommon::q_shared::qboolean,
    pub texEnv: [i32; 2],
    pub faceCulling: i32,
    pub glStateBits: libc::c_ulong,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct backEndCounters_t {
    pub c_surfaces: i32,
    pub c_shaders: i32,
    pub c_vertexes: i32,
    pub c_indexes: i32,
    pub c_totalIndexes: i32,
    pub c_overDraw: f32,
    pub c_dlightVertexes: i32,
    pub c_dlightIndexes: i32,
    pub c_flareAdds: i32,
    pub c_flareTests: i32,
    pub c_flareRenders: i32,
    pub msec: i32,
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
    pub visCount: i32,
    pub frameCount: i32,
    pub sceneCount: i32,
    pub viewCount: i32,
    pub frameSceneNum: i32,
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
    pub numLightmaps: i32,
    pub lightmaps: *mut *mut crate::tr_common_h::image_t,
    pub currentEntity: *mut trRefEntity_t,
    pub worldEntity: trRefEntity_t,
    pub currentEntityNum: i32,
    pub shiftedEntityNum: i32,
    pub currentModel: *mut model_t,
    pub viewParms: viewParms_t,
    pub identityLight: f32,
    pub identityLightByte: i32,
    pub overbrightBits: i32,
    pub or: orientationr_t,
    pub refdef: trRefdef_t,
    pub viewCluster: i32,
    pub sunLight: crate::src::qcommon::q_shared::vec3_t,
    pub sunDirection: crate::src::qcommon::q_shared::vec3_t,
    pub pc: frontEndCounters_t,
    pub frontEndMsec: i32,
    pub models: [*mut model_t; 1024],
    pub numModels: i32,
    pub numImages: i32,
    pub images: [*mut crate::tr_common_h::image_t; 2048],
    pub numShaders: i32,
    pub shaders: [*mut shader_t; 16384],
    pub sortedShaders: [*mut shader_t; 16384],
    pub numSkins: i32,
    pub skins: [*mut skin_t; 1024],
    pub sinTable: [f32; 1024],
    pub squareTable: [f32; 1024],
    pub triangleTable: [f32; 1024],
    pub sawToothTable: [f32; 1024],
    pub inverseSawToothTable: [f32; 1024],
    pub fogTable: [f32; 256],
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
    pub vertexDlightBits: [i32; 1000],
    pub svars: stageVars_t,
    pub constantColor255: [color4ub_t; 1000],
    pub shader: *mut shader_t,
    pub shaderTime: f64,
    pub fogNum: i32,
    pub dlightBits: i32,
    pub numIndexes: i32,
    pub numVertexes: i32,
    pub numPasses: i32,
    pub currentStageIteratorFunc: Option<unsafe extern "C" fn() -> ()>,
    pub xstages: *mut *mut shaderStage_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct renderCommandList_t {
    pub cmds: [crate::src::qcommon::q_shared::byte; 262144],
    pub used: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct setColorCommand_t {
    pub commandId: i32,
    pub color: [f32; 4],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drawBufferCommand_t {
    pub commandId: i32,
    pub buffer: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct swapBuffersCommand_t {
    pub commandId: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stretchPicCommand_t {
    pub commandId: i32,
    pub shader: *mut shader_t,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub s1: f32,
    pub t1: f32,
    pub s2: f32,
    pub t2: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drawSurfsCommand_t {
    pub commandId: i32,
    pub refdef: trRefdef_t,
    pub viewParms: viewParms_t,
    pub drawSurfs: *mut drawSurf_t,
    pub numDrawSurfs: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct screenshotCommand_t {
    pub commandId: i32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub fileName: *mut libc::c_char,
    pub jpeg: crate::src::qcommon::q_shared::qboolean,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct videoFrameCommand_t {
    pub commandId: i32,
    pub width: i32,
    pub height: i32,
    pub captureBuffer: *mut crate::src::qcommon::q_shared::byte,
    pub encodeBuffer: *mut crate::src::qcommon::q_shared::byte,
    pub motionJpeg: crate::src::qcommon::q_shared::qboolean,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct colorMaskCommand_t {
    pub commandId: i32,
    pub rgba: [crate::stdlib::GLboolean; 4],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clearDepthCommand_t {
    pub commandId: i32,
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
