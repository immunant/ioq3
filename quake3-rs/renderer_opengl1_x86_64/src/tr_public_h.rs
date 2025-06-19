#[repr(C)]
#[derive(Copy, Clone)]
pub struct refexport_t {
    pub Shutdown: Option<unsafe extern "C" fn(_: crate::src::qcommon::q_shared::qboolean) -> ()>,
    pub BeginRegistration:
        Option<unsafe extern "C" fn(_: *mut crate::tr_types_h::glconfig_t) -> ()>,
    pub RegisterModel: Option<
        unsafe extern "C" fn(_: *const libc::c_char) -> crate::src::qcommon::q_shared::qhandle_t,
    >,
    pub RegisterSkin: Option<
        unsafe extern "C" fn(_: *const libc::c_char) -> crate::src::qcommon::q_shared::qhandle_t,
    >,
    pub RegisterShader: Option<
        unsafe extern "C" fn(_: *const libc::c_char) -> crate::src::qcommon::q_shared::qhandle_t,
    >,
    pub RegisterShaderNoMip: Option<
        unsafe extern "C" fn(_: *const libc::c_char) -> crate::src::qcommon::q_shared::qhandle_t,
    >,
    pub LoadWorld: Option<unsafe extern "C" fn(_: *const libc::c_char) -> ()>,
    pub SetWorldVisData:
        Option<unsafe extern "C" fn(_: *const crate::src::qcommon::q_shared::byte) -> ()>,
    pub EndRegistration: Option<unsafe extern "C" fn() -> ()>,
    pub ClearScene: Option<unsafe extern "C" fn() -> ()>,
    pub AddRefEntityToScene:
        Option<unsafe extern "C" fn(_: *const crate::tr_types_h::refEntity_t) -> ()>,
    pub AddPolyToScene: Option<
        unsafe extern "C" fn(
            _: crate::src::qcommon::q_shared::qhandle_t,
            _: i32,
            _: *const crate::tr_types_h::polyVert_t,
            _: i32,
        ) -> (),
    >,
    pub LightForPoint: Option<
        unsafe extern "C" fn(
            _: *mut crate::src::qcommon::q_shared::vec_t,
            _: *mut crate::src::qcommon::q_shared::vec_t,
            _: *mut crate::src::qcommon::q_shared::vec_t,
            _: *mut crate::src::qcommon::q_shared::vec_t,
        ) -> i32,
    >,
    pub AddLightToScene: Option<
        unsafe extern "C" fn(
            _: *const crate::src::qcommon::q_shared::vec_t,
            _: f32,
            _: f32,
            _: f32,
            _: f32,
        ) -> (),
    >,
    pub AddAdditiveLightToScene: Option<
        unsafe extern "C" fn(
            _: *const crate::src::qcommon::q_shared::vec_t,
            _: f32,
            _: f32,
            _: f32,
            _: f32,
        ) -> (),
    >,
    pub RenderScene: Option<unsafe extern "C" fn(_: *const crate::tr_types_h::refdef_t) -> ()>,
    pub SetColor: Option<unsafe extern "C" fn(_: *const f32) -> ()>,
    pub DrawStretchPic: Option<
        unsafe extern "C" fn(
            _: f32,
            _: f32,
            _: f32,
            _: f32,
            _: f32,
            _: f32,
            _: f32,
            _: f32,
            _: crate::src::qcommon::q_shared::qhandle_t,
        ) -> (),
    >,
    pub DrawStretchRaw: Option<
        unsafe extern "C" fn(
            _: i32,
            _: i32,
            _: i32,
            _: i32,
            _: i32,
            _: i32,
            _: *const crate::src::qcommon::q_shared::byte,
            _: i32,
            _: crate::src::qcommon::q_shared::qboolean,
        ) -> (),
    >,
    pub UploadCinematic: Option<
        unsafe extern "C" fn(
            _: i32,
            _: i32,
            _: i32,
            _: i32,
            _: *const crate::src::qcommon::q_shared::byte,
            _: i32,
            _: crate::src::qcommon::q_shared::qboolean,
        ) -> (),
    >,
    pub BeginFrame: Option<unsafe extern "C" fn(_: crate::tr_types_h::stereoFrame_t) -> ()>,
    pub EndFrame: Option<unsafe extern "C" fn(_: *mut i32, _: *mut i32) -> ()>,
    pub MarkFragments: Option<
        unsafe extern "C" fn(
            _: i32,
            _: *const crate::src::qcommon::q_shared::vec3_t,
            _: *const crate::src::qcommon::q_shared::vec_t,
            _: i32,
            _: *mut crate::src::qcommon::q_shared::vec_t,
            _: i32,
            _: *mut crate::src::qcommon::q_shared::markFragment_t,
        ) -> i32,
    >,
    pub LerpTag: Option<
        unsafe extern "C" fn(
            _: *mut crate::src::qcommon::q_shared::orientation_t,
            _: crate::src::qcommon::q_shared::qhandle_t,
            _: i32,
            _: i32,
            _: f32,
            _: *const libc::c_char,
        ) -> i32,
    >,
    pub ModelBounds: Option<
        unsafe extern "C" fn(
            _: crate::src::qcommon::q_shared::qhandle_t,
            _: *mut crate::src::qcommon::q_shared::vec_t,
            _: *mut crate::src::qcommon::q_shared::vec_t,
        ) -> (),
    >,
    pub RegisterFont: Option<
        unsafe extern "C" fn(
            _: *const libc::c_char,
            _: i32,
            _: *mut crate::src::qcommon::q_shared::fontInfo_t,
        ) -> (),
    >,
    pub RemapShader: Option<
        unsafe extern "C" fn(
            _: *const libc::c_char,
            _: *const libc::c_char,
            _: *const libc::c_char,
        ) -> (),
    >,
    pub GetEntityToken: Option<
        unsafe extern "C" fn(
            _: *mut libc::c_char,
            _: i32,
        ) -> crate::src::qcommon::q_shared::qboolean,
    >,
    pub inPVS: Option<
        unsafe extern "C" fn(
            _: *const crate::src::qcommon::q_shared::vec_t,
            _: *const crate::src::qcommon::q_shared::vec_t,
        ) -> crate::src::qcommon::q_shared::qboolean,
    >,
    pub TakeVideoFrame: Option<
        unsafe extern "C" fn(
            _: i32,
            _: i32,
            _: *mut crate::src::qcommon::q_shared::byte,
            _: *mut crate::src::qcommon::q_shared::byte,
            _: crate::src::qcommon::q_shared::qboolean,
        ) -> (),
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct refimport_t {
    pub Printf: Option<unsafe extern "C" fn(_: i32, _: *const libc::c_char, _: ...) -> ()>,
    pub Error: Option<unsafe extern "C" fn(_: i32, _: *const libc::c_char, _: ...) -> !>,
    pub Milliseconds: Option<unsafe extern "C" fn() -> i32>,
    pub Hunk_Alloc: Option<
        unsafe extern "C" fn(
            _: i32,
            _: crate::src::qcommon::q_shared::ha_pref,
        ) -> *mut libc::c_void,
    >,
    pub Hunk_AllocateTempMemory: Option<unsafe extern "C" fn(_: i32) -> *mut libc::c_void>,
    pub Hunk_FreeTempMemory: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    pub Malloc: Option<unsafe extern "C" fn(_: i32) -> *mut libc::c_void>,
    pub Free: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    pub Cvar_Get: Option<
        unsafe extern "C" fn(
            _: *const libc::c_char,
            _: *const libc::c_char,
            _: i32,
        ) -> *mut crate::src::qcommon::q_shared::cvar_t,
    >,
    pub Cvar_Set:
        Option<unsafe extern "C" fn(_: *const libc::c_char, _: *const libc::c_char) -> ()>,
    pub Cvar_SetValue: Option<unsafe extern "C" fn(_: *const libc::c_char, _: f32) -> ()>,
    pub Cvar_CheckRange: Option<
        unsafe extern "C" fn(
            _: *mut crate::src::qcommon::q_shared::cvar_t,
            _: f32,
            _: f32,
            _: crate::src::qcommon::q_shared::qboolean,
        ) -> (),
    >,
    pub Cvar_SetDescription: Option<
        unsafe extern "C" fn(
            _: *mut crate::src::qcommon::q_shared::cvar_t,
            _: *const libc::c_char,
        ) -> (),
    >,
    pub Cvar_VariableIntegerValue: Option<unsafe extern "C" fn(_: *const libc::c_char) -> i32>,
    pub Cmd_AddCommand: Option<
        unsafe extern "C" fn(_: *const libc::c_char, _: Option<unsafe extern "C" fn() -> ()>) -> (),
    >,
    pub Cmd_RemoveCommand: Option<unsafe extern "C" fn(_: *const libc::c_char) -> ()>,
    pub Cmd_Argc: Option<unsafe extern "C" fn() -> i32>,
    pub Cmd_Argv: Option<unsafe extern "C" fn(_: i32) -> *mut libc::c_char>,
    pub Cmd_ExecuteText: Option<unsafe extern "C" fn(_: i32, _: *const libc::c_char) -> ()>,
    pub CM_ClusterPVS:
        Option<unsafe extern "C" fn(_: i32) -> *mut crate::src::qcommon::q_shared::byte>,
    pub CM_DrawDebugSurface: Option<
        unsafe extern "C" fn(
            _: Option<unsafe extern "C" fn(_: i32, _: i32, _: *mut f32) -> ()>,
        ) -> (),
    >,
    pub FS_FileIsInPAK: Option<unsafe extern "C" fn(_: *const libc::c_char, _: *mut i32) -> i32>,
    pub FS_ReadFile: Option<
        unsafe extern "C" fn(_: *const libc::c_char, _: *mut *mut libc::c_void) -> isize,
    >,
    pub FS_FreeFile: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    pub FS_ListFiles: Option<
        unsafe extern "C" fn(
            _: *const libc::c_char,
            _: *const libc::c_char,
            _: *mut i32,
        ) -> *mut *mut libc::c_char,
    >,
    pub FS_FreeFileList: Option<unsafe extern "C" fn(_: *mut *mut libc::c_char) -> ()>,
    pub FS_WriteFile:
        Option<unsafe extern "C" fn(_: *const libc::c_char, _: *const libc::c_void, _: i32) -> ()>,
    pub FS_FileExists: Option<
        unsafe extern "C" fn(_: *const libc::c_char) -> crate::src::qcommon::q_shared::qboolean,
    >,
    pub CIN_UploadCinematic: Option<unsafe extern "C" fn(_: i32) -> ()>,
    pub CIN_PlayCinematic: Option<
        unsafe extern "C" fn(_: *const libc::c_char, _: i32, _: i32, _: i32, _: i32, _: i32) -> i32,
    >,
    pub CIN_RunCinematic:
        Option<unsafe extern "C" fn(_: i32) -> crate::src::qcommon::q_shared::e_status>,
    pub CL_WriteAVIVideoFrame:
        Option<unsafe extern "C" fn(_: *const crate::src::qcommon::q_shared::byte, _: i32) -> ()>,
    pub IN_Init: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    pub IN_Shutdown: Option<unsafe extern "C" fn() -> ()>,
    pub IN_Restart: Option<unsafe extern "C" fn() -> ()>,
    pub ftol: Option<unsafe extern "C" fn(_: f32) -> isize>,
    pub Sys_SetEnv:
        Option<unsafe extern "C" fn(_: *const libc::c_char, _: *const libc::c_char) -> ()>,
    pub Sys_GLimpSafeInit: Option<unsafe extern "C" fn() -> ()>,
    pub Sys_GLimpInit: Option<unsafe extern "C" fn() -> ()>,
    pub Sys_LowPhysicalMemory:
        Option<unsafe extern "C" fn() -> crate::src::qcommon::q_shared::qboolean>,
}
