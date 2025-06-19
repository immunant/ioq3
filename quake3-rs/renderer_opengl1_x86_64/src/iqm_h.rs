pub type iqmHeader_t = iqmheader;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iqmheader {
    pub magic: [libc::c_char; 16],
    pub version: libc::c_uint,
    pub filesize: libc::c_uint,
    pub flags: libc::c_uint,
    pub num_text: libc::c_uint,
    pub ofs_text: libc::c_uint,
    pub num_meshes: libc::c_uint,
    pub ofs_meshes: libc::c_uint,
    pub num_vertexarrays: libc::c_uint,
    pub num_vertexes: libc::c_uint,
    pub ofs_vertexarrays: libc::c_uint,
    pub num_triangles: libc::c_uint,
    pub ofs_triangles: libc::c_uint,
    pub ofs_adjacency: libc::c_uint,
    pub num_joints: libc::c_uint,
    pub ofs_joints: libc::c_uint,
    pub num_poses: libc::c_uint,
    pub ofs_poses: libc::c_uint,
    pub num_anims: libc::c_uint,
    pub ofs_anims: libc::c_uint,
    pub num_frames: libc::c_uint,
    pub num_framechannels: libc::c_uint,
    pub ofs_frames: libc::c_uint,
    pub ofs_bounds: libc::c_uint,
    pub num_comment: libc::c_uint,
    pub ofs_comment: libc::c_uint,
    pub num_extensions: libc::c_uint,
    pub ofs_extensions: libc::c_uint,
}
pub type iqmMesh_t = iqmmesh;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iqmmesh {
    pub name: libc::c_uint,
    pub material: libc::c_uint,
    pub first_vertex: libc::c_uint,
    pub num_vertexes: libc::c_uint,
    pub first_triangle: libc::c_uint,
    pub num_triangles: libc::c_uint,
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
    pub vertex: [libc::c_uint; 3],
}
pub type iqmJoint_t = iqmjoint;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iqmjoint {
    pub name: libc::c_uint,
    pub parent: libc::c_int,
    pub translate: [libc::c_float; 3],
    pub rotate: [libc::c_float; 4],
    pub scale: [libc::c_float; 3],
}
pub type iqmPose_t = iqmpose;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iqmpose {
    pub parent: libc::c_int,
    pub mask: libc::c_uint,
    pub channeloffset: [libc::c_float; 10],
    pub channelscale: [libc::c_float; 10],
}
pub type iqmVertexArray_t = iqmvertexarray;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iqmvertexarray {
    pub type_0: libc::c_uint,
    pub flags: libc::c_uint,
    pub format: libc::c_uint,
    pub size: libc::c_uint,
    pub offset: libc::c_uint,
}
pub type iqmBounds_t = iqmbounds;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iqmbounds {
    pub bbmin: [libc::c_float; 3],
    pub bbmax: [libc::c_float; 3],
    pub xyradius: libc::c_float,
    pub radius: libc::c_float,
}
