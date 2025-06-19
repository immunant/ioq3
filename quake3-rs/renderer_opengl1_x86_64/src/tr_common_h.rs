pub type imgType_t = libc::c_uint;
pub const IMGTYPE_COLORALPHA: imgType_t = 0;
pub const IMGTYPE_NORMAL: imgType_t = 1;
pub const IMGTYPE_NORMALHEIGHT: imgType_t = 2;
pub const IMGTYPE_DELUXE: imgType_t = 3;
pub type imgFlags_t = libc::c_uint;
pub const IMGFLAG_NONE: imgFlags_t = 0;
pub const IMGFLAG_MIPMAP: imgFlags_t = 1;
pub const IMGFLAG_PICMIP: imgFlags_t = 2;
pub const IMGFLAG_CUBEMAP: imgFlags_t = 4;
pub const IMGFLAG_NO_COMPRESSION: imgFlags_t = 16;
pub const IMGFLAG_NOLIGHTSCALE: imgFlags_t = 32;
pub const IMGFLAG_CLAMPTOEDGE: imgFlags_t = 64;
pub const IMGFLAG_GENNORMALMAP: imgFlags_t = 128;
pub type image_t = image_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct image_s {
    pub imgName: [libc::c_char; 64],
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub uploadWidth: libc::c_int,
    pub uploadHeight: libc::c_int,
    pub texnum: crate::stdlib::GLuint,
    pub frameUsed: libc::c_int,
    pub internalFormat: libc::c_int,
    pub TMU: libc::c_int,
    pub type_0: imgType_t,
    pub flags: imgFlags_t,
    pub next: *mut image_s,
}
