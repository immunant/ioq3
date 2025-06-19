pub type iqmHeader_t = iqmheader;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iqmheader {
    pub magic: [libc::c_char; 16],
    pub version: u32,
    pub filesize: u32,
    pub flags: u32,
    pub num_text: u32,
    pub ofs_text: u32,
    pub num_meshes: u32,
    pub ofs_meshes: u32,
    pub num_vertexarrays: u32,
    pub num_vertexes: u32,
    pub ofs_vertexarrays: u32,
    pub num_triangles: u32,
    pub ofs_triangles: u32,
    pub ofs_adjacency: u32,
    pub num_joints: u32,
    pub ofs_joints: u32,
    pub num_poses: u32,
    pub ofs_poses: u32,
    pub num_anims: u32,
    pub ofs_anims: u32,
    pub num_frames: u32,
    pub num_framechannels: u32,
    pub ofs_frames: u32,
    pub ofs_bounds: u32,
    pub num_comment: u32,
    pub ofs_comment: u32,
    pub num_extensions: u32,
    pub ofs_extensions: u32,
}
pub type iqmMesh_t = iqmmesh;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iqmmesh {
    pub name: u32,
    pub material: u32,
    pub first_vertex: u32,
    pub num_vertexes: u32,
    pub first_triangle: u32,
    pub num_triangles: u32,
}
pub const IQM_POSITION: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 0;
pub const IQM_TEXCOORD: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 1;
pub const IQM_NORMAL: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 2;
pub const IQM_TANGENT: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 3;
pub const IQM_BLENDINDEXES: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 4;
pub const IQM_BLENDWEIGHTS: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 5;
pub const IQM_COLOR: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 6;
pub const IQM_CUSTOM: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 16;
pub const IQM_BYTE: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 0;
pub const IQM_UBYTE: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 1;
pub const IQM_SHORT: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 2;
pub const IQM_USHORT: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 3;
pub const IQM_INT: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 4;
pub const IQM_UINT: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 5;
pub const IQM_HALF: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 6;
pub const IQM_FLOAT: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 7;
pub const IQM_DOUBLE: crate::src::jpeg_8c::jerror::C2RustUnnamed_1 = 8;
pub type iqmTriangle_t = iqmtriangle;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iqmtriangle {
    pub vertex: [u32; 3],
}
pub type iqmJoint_t = iqmjoint;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iqmjoint {
    pub name: u32,
    pub parent: i32,
    pub translate: [f32; 3],
    pub rotate: [f32; 4],
    pub scale: [f32; 3],
}
pub type iqmPose_t = iqmpose;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iqmpose {
    pub parent: i32,
    pub mask: u32,
    pub channeloffset: [f32; 10],
    pub channelscale: [f32; 10],
}
pub type iqmVertexArray_t = iqmvertexarray;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iqmvertexarray {
    pub type_0: u32,
    pub flags: u32,
    pub format: u32,
    pub size: u32,
    pub offset: u32,
}
pub type iqmBounds_t = iqmbounds;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iqmbounds {
    pub bbmin: [f32; 3],
    pub bbmax: [f32; 3],
    pub xyradius: f32,
    pub radius: f32,
}
