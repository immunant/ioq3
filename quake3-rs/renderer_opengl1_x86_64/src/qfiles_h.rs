pub type md3Frame_t = md3Frame_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct md3Frame_s {
    pub bounds: [crate::src::qcommon::q_shared::vec3_t; 2],
    pub localOrigin: crate::src::qcommon::q_shared::vec3_t,
    pub radius: f32,
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
    pub ident: i32,
    pub name: [libc::c_char; 64],
    pub flags: i32,
    pub numFrames: i32,
    pub numShaders: i32,
    pub numVerts: i32,
    pub numTriangles: i32,
    pub ofsTriangles: i32,
    pub ofsShaders: i32,
    pub ofsSt: i32,
    pub ofsXyzNormals: i32,
    pub ofsEnd: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct md3Shader_t {
    pub name: [libc::c_char; 64],
    pub shaderIndex: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct md3Triangle_t {
    pub indexes: [i32; 3],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct md3St_t {
    pub st: [f32; 2],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct md3XyzNormal_t {
    pub xyz: [i16; 3],
    pub normal: i16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct md3Header_t {
    pub ident: i32,
    pub version: i32,
    pub name: [libc::c_char; 64],
    pub flags: i32,
    pub numFrames: i32,
    pub numTags: i32,
    pub numSurfaces: i32,
    pub numSkins: i32,
    pub ofsFrames: i32,
    pub ofsTags: i32,
    pub ofsSurfaces: i32,
    pub ofsEnd: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdrWeight_t {
    pub boneIndex: i32,
    pub boneWeight: f32,
    pub offset: crate::src::qcommon::q_shared::vec3_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdrVertex_t {
    pub normal: crate::src::qcommon::q_shared::vec3_t,
    pub texCoords: crate::src::qcommon::q_shared::vec2_t,
    pub numWeights: i32,
    pub weights: [mdrWeight_t; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdrTriangle_t {
    pub indexes: [i32; 3],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdrSurface_t {
    pub ident: i32,
    pub name: [libc::c_char; 64],
    pub shader: [libc::c_char; 64],
    pub shaderIndex: i32,
    pub ofsHeader: i32,
    pub numVerts: i32,
    pub ofsVerts: i32,
    pub numTriangles: i32,
    pub ofsTriangles: i32,
    pub numBoneReferences: i32,
    pub ofsBoneReferences: i32,
    pub ofsEnd: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdrBone_t {
    pub matrix: [[f32; 4]; 3],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdrFrame_t {
    pub bounds: [crate::src::qcommon::q_shared::vec3_t; 2],
    pub localOrigin: crate::src::qcommon::q_shared::vec3_t,
    pub radius: f32,
    pub name: [libc::c_char; 16],
    pub bones: [mdrBone_t; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdrCompBone_t {
    pub Comp: [u8; 24],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdrCompFrame_t {
    pub bounds: [crate::src::qcommon::q_shared::vec3_t; 2],
    pub localOrigin: crate::src::qcommon::q_shared::vec3_t,
    pub radius: f32,
    pub bones: [mdrCompBone_t; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdrLOD_t {
    pub numSurfaces: i32,
    pub ofsSurfaces: i32,
    pub ofsEnd: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdrTag_t {
    pub boneIndex: i32,
    pub name: [libc::c_char; 32],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdrHeader_t {
    pub ident: i32,
    pub version: i32,
    pub name: [libc::c_char; 64],
    pub numFrames: i32,
    pub numBones: i32,
    pub ofsFrames: i32,
    pub numLODs: i32,
    pub ofsLODs: i32,
    pub numTags: i32,
    pub ofsTags: i32,
    pub ofsEnd: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lump_t {
    pub fileofs: i32,
    pub filelen: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dheader_t {
    pub ident: i32,
    pub version: i32,
    pub lumps: [lump_t; 17],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmodel_t {
    pub mins: [f32; 3],
    pub maxs: [f32; 3],
    pub firstSurface: i32,
    pub numSurfaces: i32,
    pub firstBrush: i32,
    pub numBrushes: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dshader_t {
    pub shader: [libc::c_char; 64],
    pub surfaceFlags: i32,
    pub contentFlags: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dplane_t {
    pub normal: [f32; 3],
    pub dist: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dnode_t {
    pub planeNum: i32,
    pub children: [i32; 2],
    pub mins: [i32; 3],
    pub maxs: [i32; 3],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dleaf_t {
    pub cluster: i32,
    pub area: i32,
    pub mins: [i32; 3],
    pub maxs: [i32; 3],
    pub firstLeafSurface: i32,
    pub numLeafSurfaces: i32,
    pub firstLeafBrush: i32,
    pub numLeafBrushes: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbrushside_t {
    pub planeNum: i32,
    pub shaderNum: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbrush_t {
    pub firstSide: i32,
    pub numSides: i32,
    pub shaderNum: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfog_t {
    pub shader: [libc::c_char; 64],
    pub brushNum: i32,
    pub visibleSide: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drawVert_t {
    pub xyz: crate::src::qcommon::q_shared::vec3_t,
    pub st: [f32; 2],
    pub lightmap: [f32; 2],
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
    pub shaderNum: i32,
    pub fogNum: i32,
    pub surfaceType: i32,
    pub firstVert: i32,
    pub numVerts: i32,
    pub firstIndex: i32,
    pub numIndexes: i32,
    pub lightmapNum: i32,
    pub lightmapX: i32,
    pub lightmapY: i32,
    pub lightmapWidth: i32,
    pub lightmapHeight: i32,
    pub lightmapOrigin: crate::src::qcommon::q_shared::vec3_t,
    pub lightmapVecs: [crate::src::qcommon::q_shared::vec3_t; 3],
    pub patchWidth: i32,
    pub patchHeight: i32,
}
