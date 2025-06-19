pub type md3Frame_t = md3Frame_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct md3Frame_s {
    pub bounds: [crate::src::qcommon::q_shared::vec3_t; 2],
    pub localOrigin: crate::src::qcommon::q_shared::vec3_t,
    pub radius: libc::c_float,
    pub name: [libc::c_char; 16],
}
pub type md3Tag_t = md3Tag_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct md3Tag_s {
    pub name: [libc::c_char; 64],
    pub origin: crate::src::qcommon::q_shared::vec3_t,
    pub axis: [crate::src::qcommon::q_shared::vec3_t; 3],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct md3Surface_t {
    pub ident: libc::c_int,
    pub name: [libc::c_char; 64],
    pub flags: libc::c_int,
    pub numFrames: libc::c_int,
    pub numShaders: libc::c_int,
    pub numVerts: libc::c_int,
    pub numTriangles: libc::c_int,
    pub ofsTriangles: libc::c_int,
    pub ofsShaders: libc::c_int,
    pub ofsSt: libc::c_int,
    pub ofsXyzNormals: libc::c_int,
    pub ofsEnd: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct md3Shader_t {
    pub name: [libc::c_char; 64],
    pub shaderIndex: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct md3Triangle_t {
    pub indexes: [libc::c_int; 3],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct md3St_t {
    pub st: [libc::c_float; 2],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct md3XyzNormal_t {
    pub xyz: [libc::c_short; 3],
    pub normal: libc::c_short,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct md3Header_t {
    pub ident: libc::c_int,
    pub version: libc::c_int,
    pub name: [libc::c_char; 64],
    pub flags: libc::c_int,
    pub numFrames: libc::c_int,
    pub numTags: libc::c_int,
    pub numSurfaces: libc::c_int,
    pub numSkins: libc::c_int,
    pub ofsFrames: libc::c_int,
    pub ofsTags: libc::c_int,
    pub ofsSurfaces: libc::c_int,
    pub ofsEnd: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdrWeight_t {
    pub boneIndex: libc::c_int,
    pub boneWeight: libc::c_float,
    pub offset: crate::src::qcommon::q_shared::vec3_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdrVertex_t {
    pub normal: crate::src::qcommon::q_shared::vec3_t,
    pub texCoords: crate::src::qcommon::q_shared::vec2_t,
    pub numWeights: libc::c_int,
    pub weights: [mdrWeight_t; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdrTriangle_t {
    pub indexes: [libc::c_int; 3],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdrSurface_t {
    pub ident: libc::c_int,
    pub name: [libc::c_char; 64],
    pub shader: [libc::c_char; 64],
    pub shaderIndex: libc::c_int,
    pub ofsHeader: libc::c_int,
    pub numVerts: libc::c_int,
    pub ofsVerts: libc::c_int,
    pub numTriangles: libc::c_int,
    pub ofsTriangles: libc::c_int,
    pub numBoneReferences: libc::c_int,
    pub ofsBoneReferences: libc::c_int,
    pub ofsEnd: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdrBone_t {
    pub matrix: [[libc::c_float; 4]; 3],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdrFrame_t {
    pub bounds: [crate::src::qcommon::q_shared::vec3_t; 2],
    pub localOrigin: crate::src::qcommon::q_shared::vec3_t,
    pub radius: libc::c_float,
    pub name: [libc::c_char; 16],
    pub bones: [mdrBone_t; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdrCompBone_t {
    pub Comp: [libc::c_uchar; 24],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdrCompFrame_t {
    pub bounds: [crate::src::qcommon::q_shared::vec3_t; 2],
    pub localOrigin: crate::src::qcommon::q_shared::vec3_t,
    pub radius: libc::c_float,
    pub bones: [mdrCompBone_t; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdrLOD_t {
    pub numSurfaces: libc::c_int,
    pub ofsSurfaces: libc::c_int,
    pub ofsEnd: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdrTag_t {
    pub boneIndex: libc::c_int,
    pub name: [libc::c_char; 32],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdrHeader_t {
    pub ident: libc::c_int,
    pub version: libc::c_int,
    pub name: [libc::c_char; 64],
    pub numFrames: libc::c_int,
    pub numBones: libc::c_int,
    pub ofsFrames: libc::c_int,
    pub numLODs: libc::c_int,
    pub ofsLODs: libc::c_int,
    pub numTags: libc::c_int,
    pub ofsTags: libc::c_int,
    pub ofsEnd: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lump_t {
    pub fileofs: libc::c_int,
    pub filelen: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dheader_t {
    pub ident: libc::c_int,
    pub version: libc::c_int,
    pub lumps: [lump_t; 17],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmodel_t {
    pub mins: [libc::c_float; 3],
    pub maxs: [libc::c_float; 3],
    pub firstSurface: libc::c_int,
    pub numSurfaces: libc::c_int,
    pub firstBrush: libc::c_int,
    pub numBrushes: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dshader_t {
    pub shader: [libc::c_char; 64],
    pub surfaceFlags: libc::c_int,
    pub contentFlags: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dplane_t {
    pub normal: [libc::c_float; 3],
    pub dist: libc::c_float,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dnode_t {
    pub planeNum: libc::c_int,
    pub children: [libc::c_int; 2],
    pub mins: [libc::c_int; 3],
    pub maxs: [libc::c_int; 3],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dleaf_t {
    pub cluster: libc::c_int,
    pub area: libc::c_int,
    pub mins: [libc::c_int; 3],
    pub maxs: [libc::c_int; 3],
    pub firstLeafSurface: libc::c_int,
    pub numLeafSurfaces: libc::c_int,
    pub firstLeafBrush: libc::c_int,
    pub numLeafBrushes: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbrushside_t {
    pub planeNum: libc::c_int,
    pub shaderNum: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbrush_t {
    pub firstSide: libc::c_int,
    pub numSides: libc::c_int,
    pub shaderNum: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfog_t {
    pub shader: [libc::c_char; 64],
    pub brushNum: libc::c_int,
    pub visibleSide: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drawVert_t {
    pub xyz: crate::src::qcommon::q_shared::vec3_t,
    pub st: [libc::c_float; 2],
    pub lightmap: [libc::c_float; 2],
    pub normal: crate::src::qcommon::q_shared::vec3_t,
    pub color: [crate::src::qcommon::q_shared::byte; 4],
}
pub const MST_BAD: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 0;
pub const MST_PLANAR: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 1;
pub const MST_PATCH: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 2;
pub const MST_TRIANGLE_SOUP: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 3;
pub const MST_FLARE: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsurface_t {
    pub shaderNum: libc::c_int,
    pub fogNum: libc::c_int,
    pub surfaceType: libc::c_int,
    pub firstVert: libc::c_int,
    pub numVerts: libc::c_int,
    pub firstIndex: libc::c_int,
    pub numIndexes: libc::c_int,
    pub lightmapNum: libc::c_int,
    pub lightmapX: libc::c_int,
    pub lightmapY: libc::c_int,
    pub lightmapWidth: libc::c_int,
    pub lightmapHeight: libc::c_int,
    pub lightmapOrigin: crate::src::qcommon::q_shared::vec3_t,
    pub lightmapVecs: [crate::src::qcommon::q_shared::vec3_t; 3],
    pub patchWidth: libc::c_int,
    pub patchHeight: libc::c_int,
}
