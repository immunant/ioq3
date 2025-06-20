use ::libc;

pub mod stdlib_float_h {
    #[inline]

    pub unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> f64 {
        return libc::strtod(__nptr, std::ptr::null_mut() as *mut *mut libc::c_char);
    }
}

pub mod stdlib_h {
    #[inline]

    pub unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> i32 {
        return libc::strtol(
            __nptr,
            std::ptr::null_mut() as *mut *mut libc::c_char,
            10 as i32,
        ) as i32;
    }
}

pub use crate::bg_public_h::animation_s;
pub use crate::bg_public_h::animation_t;
pub use crate::bg_public_h::gitem_s;
pub use crate::bg_public_h::gitem_t;
pub use crate::bg_public_h::itemType_t;
pub use crate::bg_public_h::weapon_t;
pub use crate::bg_public_h::BOTH_DEAD1;
pub use crate::bg_public_h::BOTH_DEAD2;
pub use crate::bg_public_h::BOTH_DEAD3;
pub use crate::bg_public_h::BOTH_DEATH1;
pub use crate::bg_public_h::BOTH_DEATH2;
pub use crate::bg_public_h::BOTH_DEATH3;
pub use crate::bg_public_h::FLAG_RUN;
pub use crate::bg_public_h::FLAG_STAND;
pub use crate::bg_public_h::FLAG_STAND2RUN;
pub use crate::bg_public_h::IT_AMMO;
pub use crate::bg_public_h::IT_ARMOR;
pub use crate::bg_public_h::IT_BAD;
pub use crate::bg_public_h::IT_HEALTH;
pub use crate::bg_public_h::IT_HOLDABLE;
pub use crate::bg_public_h::IT_PERSISTANT_POWERUP;
pub use crate::bg_public_h::IT_POWERUP;
pub use crate::bg_public_h::IT_TEAM;
pub use crate::bg_public_h::IT_WEAPON;
pub use crate::bg_public_h::LEGS_BACK;
pub use crate::bg_public_h::LEGS_BACKCR;
pub use crate::bg_public_h::LEGS_BACKWALK;
pub use crate::bg_public_h::LEGS_IDLE;
pub use crate::bg_public_h::LEGS_IDLECR;
pub use crate::bg_public_h::LEGS_JUMP;
pub use crate::bg_public_h::LEGS_JUMPB;
pub use crate::bg_public_h::LEGS_LAND;
pub use crate::bg_public_h::LEGS_LANDB;
pub use crate::bg_public_h::LEGS_RUN;
pub use crate::bg_public_h::LEGS_SWIM;
pub use crate::bg_public_h::LEGS_TURN;
pub use crate::bg_public_h::LEGS_WALK;
pub use crate::bg_public_h::LEGS_WALKCR;
pub use crate::bg_public_h::MAX_ANIMATIONS;
pub use crate::bg_public_h::MAX_TOTALANIMATIONS;
pub use crate::bg_public_h::TORSO_AFFIRMATIVE;
pub use crate::bg_public_h::TORSO_ATTACK;
pub use crate::bg_public_h::TORSO_ATTACK2;
pub use crate::bg_public_h::TORSO_DROP;
pub use crate::bg_public_h::TORSO_FOLLOWME;
pub use crate::bg_public_h::TORSO_GESTURE;
pub use crate::bg_public_h::TORSO_GETFLAG;
pub use crate::bg_public_h::TORSO_GUARDBASE;
pub use crate::bg_public_h::TORSO_NEGATIVE;
pub use crate::bg_public_h::TORSO_PATROL;
pub use crate::bg_public_h::TORSO_RAISE;
pub use crate::bg_public_h::TORSO_STAND;
pub use crate::bg_public_h::TORSO_STAND2;
pub use crate::bg_public_h::WP_BFG;
pub use crate::bg_public_h::WP_GAUNTLET;
pub use crate::bg_public_h::WP_GRAPPLING_HOOK;
pub use crate::bg_public_h::WP_GRENADE_LAUNCHER;
pub use crate::bg_public_h::WP_LIGHTNING;
pub use crate::bg_public_h::WP_MACHINEGUN;
pub use crate::bg_public_h::WP_NONE;
pub use crate::bg_public_h::WP_NUM_WEAPONS;
pub use crate::bg_public_h::WP_PLASMAGUN;
pub use crate::bg_public_h::WP_RAILGUN;
pub use crate::bg_public_h::WP_ROCKET_LAUNCHER;
pub use crate::bg_public_h::WP_SHOTGUN;
pub use crate::src::game::bg_misc::bg_itemlist;
pub use crate::src::q3_ui::ui_atoms::uis;
pub use crate::src::q3_ui::ui_atoms::Com_Printf;
pub use crate::src::q3_ui::ui_atoms::UI_AdjustFrom640;
pub use crate::src::q3_ui::ui_players::stdlib_float_h::atof;
pub use crate::src::q3_ui::ui_players::stdlib_h::atoi;
pub use crate::src::q3_ui::ui_qmenu::weaponChangeSound;
pub use crate::src::qcommon::q_math::colorWhite;
pub use crate::src::qcommon::q_math::AngleMod;
pub use crate::src::qcommon::q_math::AngleSubtract;
pub use crate::src::qcommon::q_math::AngleVectors;
pub use crate::src::qcommon::q_math::AnglesSubtract;
pub use crate::src::qcommon::q_math::AnglesToAxis;
pub use crate::src::qcommon::q_math::AxisClear;
pub use crate::src::qcommon::q_math::MatrixMultiply;
pub use crate::src::qcommon::q_math::Q_fabs;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::clipHandle_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::fsMode_t;
pub use crate::src::qcommon::q_shared::orientation_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::COM_Parse;
pub use crate::src::qcommon::q_shared::COM_StripExtension;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Q_strcat;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::CHAN_ANNOUNCER;
pub use crate::src::qcommon::q_shared::CHAN_AUTO;
pub use crate::src::qcommon::q_shared::CHAN_BODY;
pub use crate::src::qcommon::q_shared::CHAN_ITEM;
pub use crate::src::qcommon::q_shared::CHAN_LOCAL;
pub use crate::src::qcommon::q_shared::CHAN_LOCAL_SOUND;
pub use crate::src::qcommon::q_shared::CHAN_VOICE;
pub use crate::src::qcommon::q_shared::CHAN_WEAPON;
pub use crate::src::qcommon::q_shared::FS_APPEND;
pub use crate::src::qcommon::q_shared::FS_APPEND_SYNC;
pub use crate::src::qcommon::q_shared::FS_READ;
pub use crate::src::qcommon::q_shared::FS_WRITE;
pub use crate::src::ui::ui_syscalls::trap_CM_LerpTag;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableValue;
pub use crate::src::ui::ui_syscalls::trap_Error;
pub use crate::src::ui::ui_syscalls::trap_FS_FCloseFile;
pub use crate::src::ui::ui_syscalls::trap_FS_FOpenFile;
pub use crate::src::ui::ui_syscalls::trap_FS_Read;
pub use crate::src::ui::ui_syscalls::trap_R_AddLightToScene;
pub use crate::src::ui::ui_syscalls::trap_R_AddRefEntityToScene;
pub use crate::src::ui::ui_syscalls::trap_R_ClearScene;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterModel;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterSkin;
pub use crate::src::ui::ui_syscalls::trap_R_RenderScene;
pub use crate::src::ui::ui_syscalls::trap_S_StartLocalSound;

pub use crate::tr_types_h::glDriverType_t;
pub use crate::tr_types_h::glHardwareType_t;
pub use crate::tr_types_h::glconfig_t;
pub use crate::tr_types_h::refEntityType_t;
pub use crate::tr_types_h::refEntity_t;
pub use crate::tr_types_h::refdef_t;
pub use crate::tr_types_h::textureCompression_t;
pub use crate::tr_types_h::GLDRV_ICD;
pub use crate::tr_types_h::GLDRV_STANDALONE;
pub use crate::tr_types_h::GLDRV_VOODOO;
pub use crate::tr_types_h::GLHW_3DFX_2D3D;
pub use crate::tr_types_h::GLHW_GENERIC;
pub use crate::tr_types_h::GLHW_PERMEDIA2;
pub use crate::tr_types_h::GLHW_RAGEPRO;
pub use crate::tr_types_h::GLHW_RIVA128;
pub use crate::tr_types_h::RT_BEAM;
pub use crate::tr_types_h::RT_LIGHTNING;
pub use crate::tr_types_h::RT_MAX_REF_ENTITY_TYPE;
pub use crate::tr_types_h::RT_MODEL;
pub use crate::tr_types_h::RT_POLY;
pub use crate::tr_types_h::RT_PORTALSURFACE;
pub use crate::tr_types_h::RT_RAIL_CORE;
pub use crate::tr_types_h::RT_RAIL_RINGS;
pub use crate::tr_types_h::RT_SPRITE;
pub use crate::tr_types_h::TC_NONE;
pub use crate::tr_types_h::TC_S3TC;
pub use crate::tr_types_h::TC_S3TC_ARB;
pub use crate::ui_local_h::_tag_menuframework;
pub use crate::ui_local_h::lerpFrame_t;
pub use crate::ui_local_h::menuframework_s;
pub use crate::ui_local_h::playerInfo_t;
pub use crate::ui_local_h::uiStatic_t;
pub use ::libc::rand;

pub use ::libc::strtod;
pub use ::libc::strtol;

static mut dp_realtime: i32 = 0;

static mut jumpHeight: f32 = 0.;
/*
===============
UI_PlayerInfo_SetWeapon
===============
*/

unsafe extern "C" fn UI_PlayerInfo_SetWeapon(mut pi: *mut playerInfo_t, mut weaponNum: weapon_t) {
    let mut item: *mut gitem_t = std::ptr::null_mut();
    let mut path: [libc::c_char; 64] = [0; 64];
    (*pi).currentWeapon = weaponNum;
    loop {
        (*pi).realWeapon = weaponNum as i32;
        (*pi).weaponModel = 0 as i32;
        (*pi).barrelModel = 0 as i32;
        (*pi).flashModel = 0 as i32;
        if weaponNum as u32 == WP_NONE as i32 as u32 {
            return;
        }
        item = bg_itemlist.as_mut_ptr().offset(1 as i32 as isize);
        while !(*item).classname.is_null() {
            if !((*item).giType as u32 != IT_WEAPON as i32 as u32) {
                if (*item).giTag as u32 == weaponNum as u32 {
                    break;
                }
            }
            item = item.offset(1)
        }
        if !(*item).classname.is_null() {
            (*pi).weaponModel = trap_R_RegisterModel((*item).world_model[0 as i32 as usize])
        }
        if !((*pi).weaponModel == 0 as i32) {
            break;
        }
        if weaponNum as u32 == WP_MACHINEGUN as i32 as u32 {
            weaponNum = WP_NONE
        } else {
            weaponNum = WP_MACHINEGUN
        }
    }
    if weaponNum as u32 == WP_MACHINEGUN as i32 as u32
        || weaponNum as u32 == WP_GAUNTLET as i32 as u32
        || weaponNum as u32 == WP_BFG as i32 as u32
    {
        COM_StripExtension(
            (*item).world_model[0 as i32 as usize],
            path.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
        );
        Q_strcat(
            path.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
            b"_barrel.md3\x00" as *const u8 as *const libc::c_char,
        );
        (*pi).barrelModel = trap_R_RegisterModel(path.as_mut_ptr())
    }
    COM_StripExtension(
        (*item).world_model[0 as i32 as usize],
        path.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
    );
    Q_strcat(
        path.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
        b"_flash.md3\x00" as *const u8 as *const libc::c_char,
    );
    (*pi).flashModel = trap_R_RegisterModel(path.as_mut_ptr());
    match weaponNum as u32 {
        1 => {
            (*pi).flashDlightColor[0 as i32 as usize] = 0.6f32;
            (*pi).flashDlightColor[1 as i32 as usize] = 0.6f32;
            (*pi).flashDlightColor[2 as i32 as usize] = 1 as i32 as vec_t
        }
        2 => {
            (*pi).flashDlightColor[0 as i32 as usize] = 1 as i32 as vec_t;
            (*pi).flashDlightColor[1 as i32 as usize] = 1 as i32 as vec_t;
            (*pi).flashDlightColor[2 as i32 as usize] = 0 as i32 as vec_t
        }
        3 => {
            (*pi).flashDlightColor[0 as i32 as usize] = 1 as i32 as vec_t;
            (*pi).flashDlightColor[1 as i32 as usize] = 1 as i32 as vec_t;
            (*pi).flashDlightColor[2 as i32 as usize] = 0 as i32 as vec_t
        }
        4 => {
            (*pi).flashDlightColor[0 as i32 as usize] = 1 as i32 as vec_t;
            (*pi).flashDlightColor[1 as i32 as usize] = 0.7f32;
            (*pi).flashDlightColor[2 as i32 as usize] = 0.5f32
        }
        5 => {
            (*pi).flashDlightColor[0 as i32 as usize] = 1 as i32 as vec_t;
            (*pi).flashDlightColor[1 as i32 as usize] = 0.75f32;
            (*pi).flashDlightColor[2 as i32 as usize] = 0 as i32 as vec_t
        }
        6 => {
            (*pi).flashDlightColor[0 as i32 as usize] = 0.6f32;
            (*pi).flashDlightColor[1 as i32 as usize] = 0.6f32;
            (*pi).flashDlightColor[2 as i32 as usize] = 1 as i32 as vec_t
        }
        7 => {
            (*pi).flashDlightColor[0 as i32 as usize] = 1 as i32 as vec_t;
            (*pi).flashDlightColor[1 as i32 as usize] = 0.5f32;
            (*pi).flashDlightColor[2 as i32 as usize] = 0 as i32 as vec_t
        }
        8 => {
            (*pi).flashDlightColor[0 as i32 as usize] = 0.6f32;
            (*pi).flashDlightColor[1 as i32 as usize] = 0.6f32;
            (*pi).flashDlightColor[2 as i32 as usize] = 1 as i32 as vec_t
        }
        9 => {
            (*pi).flashDlightColor[0 as i32 as usize] = 1 as i32 as vec_t;
            (*pi).flashDlightColor[1 as i32 as usize] = 0.7f32;
            (*pi).flashDlightColor[2 as i32 as usize] = 1 as i32 as vec_t
        }
        10 => {
            (*pi).flashDlightColor[0 as i32 as usize] = 0.6f32;
            (*pi).flashDlightColor[1 as i32 as usize] = 0.6f32;
            (*pi).flashDlightColor[2 as i32 as usize] = 1 as i32 as vec_t
        }
        _ => {
            (*pi).flashDlightColor[0 as i32 as usize] = 1 as i32 as vec_t;
            (*pi).flashDlightColor[1 as i32 as usize] = 1 as i32 as vec_t;
            (*pi).flashDlightColor[2 as i32 as usize] = 1 as i32 as vec_t
        }
    };
}
/*
===============
UI_ForceLegsAnim
===============
*/

unsafe extern "C" fn UI_ForceLegsAnim(mut pi: *mut playerInfo_t, mut anim: i32) {
    (*pi).legsAnim = (*pi).legsAnim & 128 as i32 ^ 128 as i32 | anim;
    if anim == LEGS_JUMP as i32 {
        (*pi).legsAnimationTimer = 1000 as i32
    };
}
/*
===============
UI_SetLegsAnim
===============
*/

unsafe extern "C" fn UI_SetLegsAnim(mut pi: *mut playerInfo_t, mut anim: i32) {
    if (*pi).pendingLegsAnim != 0 {
        anim = (*pi).pendingLegsAnim;
        (*pi).pendingLegsAnim = 0 as i32
    }
    UI_ForceLegsAnim(pi, anim);
}
/*
===============
UI_ForceTorsoAnim
===============
*/

unsafe extern "C" fn UI_ForceTorsoAnim(mut pi: *mut playerInfo_t, mut anim: i32) {
    (*pi).torsoAnim = (*pi).torsoAnim & 128 as i32 ^ 128 as i32 | anim;
    if anim == TORSO_GESTURE as i32 {
        (*pi).torsoAnimationTimer = 2300 as i32
    }
    if anim == TORSO_ATTACK as i32 || anim == TORSO_ATTACK2 as i32 {
        (*pi).torsoAnimationTimer = 500 as i32
    };
}
/*
===============
UI_SetTorsoAnim
===============
*/

unsafe extern "C" fn UI_SetTorsoAnim(mut pi: *mut playerInfo_t, mut anim: i32) {
    if (*pi).pendingTorsoAnim != 0 {
        anim = (*pi).pendingTorsoAnim;
        (*pi).pendingTorsoAnim = 0 as i32
    }
    UI_ForceTorsoAnim(pi, anim);
}
/*
===============
UI_TorsoSequencing
===============
*/

unsafe extern "C" fn UI_TorsoSequencing(mut pi: *mut playerInfo_t) {
    let mut currentAnim: i32 = 0;
    currentAnim = (*pi).torsoAnim & !(128 as i32);
    if (*pi).weapon as u32 != (*pi).currentWeapon as u32 {
        if currentAnim != TORSO_DROP as i32 {
            (*pi).torsoAnimationTimer = 300 as i32;
            UI_ForceTorsoAnim(pi, TORSO_DROP as i32);
        }
    }
    if (*pi).torsoAnimationTimer > 0 as i32 {
        return;
    }
    if currentAnim == TORSO_GESTURE as i32 {
        UI_SetTorsoAnim(pi, TORSO_STAND as i32);
        return;
    }
    if currentAnim == TORSO_ATTACK as i32 || currentAnim == TORSO_ATTACK2 as i32 {
        UI_SetTorsoAnim(pi, TORSO_STAND as i32);
        return;
    }
    if currentAnim == TORSO_DROP as i32 {
        UI_PlayerInfo_SetWeapon(pi, (*pi).weapon);
        (*pi).torsoAnimationTimer = 300 as i32;
        UI_ForceTorsoAnim(pi, TORSO_RAISE as i32);
        return;
    }
    if currentAnim == TORSO_RAISE as i32 {
        UI_SetTorsoAnim(pi, TORSO_STAND as i32);
        return;
    };
}
/*
===============
UI_LegsSequencing
===============
*/

unsafe extern "C" fn UI_LegsSequencing(mut pi: *mut playerInfo_t) {
    let mut currentAnim: i32 = 0;
    currentAnim = (*pi).legsAnim & !(128 as i32);
    if (*pi).legsAnimationTimer > 0 as i32 {
        if currentAnim == LEGS_JUMP as i32 {
            jumpHeight = (56 as i32 as f64
                * crate::stdlib::sin(
                    3.14159265358979323846f64 * (1000 as i32 - (*pi).legsAnimationTimer) as f64
                        / 1000 as i32 as f64,
                )) as f32
        }
        return;
    }
    if currentAnim == LEGS_JUMP as i32 {
        UI_ForceLegsAnim(pi, LEGS_LAND as i32);
        (*pi).legsAnimationTimer = 130 as i32;
        jumpHeight = 0 as i32 as f32;
        return;
    }
    if currentAnim == LEGS_LAND as i32 {
        UI_SetLegsAnim(pi, LEGS_IDLE as i32);
        return;
    };
}
/*
======================
UI_PositionEntityOnTag
======================
*/

unsafe extern "C" fn UI_PositionEntityOnTag(
    mut entity: *mut refEntity_t,
    mut parent: *const refEntity_t,
    mut parentModel: clipHandle_t,
    mut tagName: *mut libc::c_char,
) {
    let mut i: i32 = 0;
    let mut lerped: orientation_t = orientation_t {
        origin: [0.; 3],
        axis: [[0.; 3]; 3],
    };
    // lerp the tag
    trap_CM_LerpTag(
        &mut lerped as *mut _ as *mut orientation_t,
        parentModel,
        (*parent).oldframe,
        (*parent).frame,
        (1.0f64 - (*parent).backlerp as f64) as f32,
        tagName,
    );
    // FIXME: allow origin offsets along tag?
    (*entity).origin[0 as i32 as usize] = (*parent).origin[0 as i32 as usize];
    (*entity).origin[1 as i32 as usize] = (*parent).origin[1 as i32 as usize];
    (*entity).origin[2 as i32 as usize] = (*parent).origin[2 as i32 as usize];
    i = 0 as i32;
    while i < 3 as i32 {
        (*entity).origin[0 as i32 as usize] = (*entity).origin[0 as i32 as usize]
            + (*parent).axis[i as usize][0 as i32 as usize] * lerped.origin[i as usize];
        (*entity).origin[1 as i32 as usize] = (*entity).origin[1 as i32 as usize]
            + (*parent).axis[i as usize][1 as i32 as usize] * lerped.origin[i as usize];
        (*entity).origin[2 as i32 as usize] = (*entity).origin[2 as i32 as usize]
            + (*parent).axis[i as usize][2 as i32 as usize] * lerped.origin[i as usize];
        i += 1
    }
    // cast away const because of compiler problems
    MatrixMultiply(
        lerped.axis.as_mut_ptr(),
        (*(parent as *mut refEntity_t)).axis.as_mut_ptr(),
        (*entity).axis.as_mut_ptr(),
    );
    (*entity).backlerp = (*parent).backlerp;
}
/*
======================
UI_PositionRotatedEntityOnTag
======================
*/

unsafe extern "C" fn UI_PositionRotatedEntityOnTag(
    mut entity: *mut refEntity_t,
    mut parent: *const refEntity_t,
    mut parentModel: clipHandle_t,
    mut tagName: *mut libc::c_char,
) {
    let mut i: i32 = 0;
    let mut lerped: orientation_t = orientation_t {
        origin: [0.; 3],
        axis: [[0.; 3]; 3],
    };
    let mut tempAxis: [vec3_t; 3] = [[0.; 3]; 3];
    // lerp the tag
    trap_CM_LerpTag(
        &mut lerped as *mut _ as *mut orientation_t,
        parentModel,
        (*parent).oldframe,
        (*parent).frame,
        (1.0f64 - (*parent).backlerp as f64) as f32,
        tagName,
    );
    // FIXME: allow origin offsets along tag?
    (*entity).origin[0 as i32 as usize] = (*parent).origin[0 as i32 as usize];
    (*entity).origin[1 as i32 as usize] = (*parent).origin[1 as i32 as usize];
    (*entity).origin[2 as i32 as usize] = (*parent).origin[2 as i32 as usize];
    i = 0 as i32;
    while i < 3 as i32 {
        (*entity).origin[0 as i32 as usize] = (*entity).origin[0 as i32 as usize]
            + (*parent).axis[i as usize][0 as i32 as usize] * lerped.origin[i as usize];
        (*entity).origin[1 as i32 as usize] = (*entity).origin[1 as i32 as usize]
            + (*parent).axis[i as usize][1 as i32 as usize] * lerped.origin[i as usize];
        (*entity).origin[2 as i32 as usize] = (*entity).origin[2 as i32 as usize]
            + (*parent).axis[i as usize][2 as i32 as usize] * lerped.origin[i as usize];
        i += 1
    }
    // cast away const because of compiler problems
    MatrixMultiply(
        (*entity).axis.as_mut_ptr(),
        lerped.axis.as_mut_ptr(),
        tempAxis.as_mut_ptr(),
    );
    MatrixMultiply(
        tempAxis.as_mut_ptr(),
        (*(parent as *mut refEntity_t)).axis.as_mut_ptr(),
        (*entity).axis.as_mut_ptr(),
    );
}
/*
===============
UI_SetLerpFrameAnimation
===============
*/

unsafe extern "C" fn UI_SetLerpFrameAnimation(
    mut ci: *mut playerInfo_t,
    mut lf: *mut lerpFrame_t,
    mut newAnimation: i32,
) {
    let mut anim: *mut animation_t = std::ptr::null_mut();
    (*lf).animationNumber = newAnimation;
    newAnimation &= !(128 as i32);
    if newAnimation < 0 as i32 || newAnimation >= MAX_ANIMATIONS as i32 {
        trap_Error(va(
            b"Bad animation number: %i\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            newAnimation,
        ));
    }
    anim = &mut *(*ci).animations.as_mut_ptr().offset(newAnimation as isize) as *mut animation_t;
    (*lf).animation = anim;
    (*lf).animationTime = (*lf).frameTime + (*anim).initialLerp;
}
/*
===============
UI_RunLerpFrame
===============
*/

unsafe extern "C" fn UI_RunLerpFrame(
    mut ci: *mut playerInfo_t,
    mut lf: *mut lerpFrame_t,
    mut newAnimation: i32,
) {
    let mut f: i32 = 0;
    let mut numFrames: i32 = 0;
    let mut anim: *mut animation_t = std::ptr::null_mut();
    // see if the animation sequence is switching
    if newAnimation != (*lf).animationNumber || (*lf).animation.is_null() {
        UI_SetLerpFrameAnimation(ci, lf, newAnimation);
    }
    // if we have passed the current frame, move it to
    // oldFrame and calculate a new frame
    if dp_realtime >= (*lf).frameTime {
        (*lf).oldFrame = (*lf).frame;
        (*lf).oldFrameTime = (*lf).frameTime;
        // get the next frame based on the animation
        anim = (*lf).animation;
        if (*anim).frameLerp == 0 {
            return;
            // shouldn't happen
        }
        if dp_realtime < (*lf).animationTime {
            (*lf).frameTime = (*lf).animationTime
        // initial lerp
        } else {
            (*lf).frameTime = (*lf).oldFrameTime + (*anim).frameLerp
        }
        f = ((*lf).frameTime - (*lf).animationTime) / (*anim).frameLerp;
        numFrames = (*anim).numFrames;
        if (*anim).flipflop != 0 {
            numFrames *= 2 as i32
        }
        if f >= numFrames {
            f -= numFrames;
            if (*anim).loopFrames != 0 {
                f %= (*anim).loopFrames;
                f += (*anim).numFrames - (*anim).loopFrames
            } else {
                f = numFrames - 1 as i32;
                // the animation is stuck at the end, so it
                // can immediately transition to another sequence
                (*lf).frameTime = dp_realtime
            }
        }
        if (*anim).reversed != 0 {
            (*lf).frame = (*anim).firstFrame + (*anim).numFrames - 1 as i32 - f
        } else if (*anim).flipflop != 0 && f >= (*anim).numFrames {
            (*lf).frame = (*anim).firstFrame + (*anim).numFrames - 1 as i32 - f % (*anim).numFrames
        } else {
            (*lf).frame = (*anim).firstFrame + f
        }
        if dp_realtime > (*lf).frameTime {
            (*lf).frameTime = dp_realtime
        }
    }
    if (*lf).frameTime > dp_realtime + 200 as i32 {
        (*lf).frameTime = dp_realtime
    }
    if (*lf).oldFrameTime > dp_realtime {
        (*lf).oldFrameTime = dp_realtime
    }
    // calculate current lerp value
    if (*lf).frameTime == (*lf).oldFrameTime {
        (*lf).backlerp = 0 as i32 as f32
    } else {
        (*lf).backlerp = (1.0f64
            - ((dp_realtime - (*lf).oldFrameTime) as f32
                / ((*lf).frameTime - (*lf).oldFrameTime) as f32) as f64)
            as f32
    };
}
/*
===============
UI_PlayerAnimation
===============
*/

unsafe extern "C" fn UI_PlayerAnimation(
    mut pi: *mut playerInfo_t,
    mut legsOld: *mut i32,
    mut legs: *mut i32,
    mut legsBackLerp: *mut f32,
    mut torsoOld: *mut i32,
    mut torso: *mut i32,
    mut torsoBackLerp: *mut f32,
) {
    // legs animation
    (*pi).legsAnimationTimer -= uis.frametime;
    if (*pi).legsAnimationTimer < 0 as i32 {
        (*pi).legsAnimationTimer = 0 as i32
    }
    UI_LegsSequencing(pi);
    if (*pi).legs.yawing as u32 != 0 && (*pi).legsAnim & !(128 as i32) == LEGS_IDLE as i32 {
        UI_RunLerpFrame(pi, &mut (*pi).legs, LEGS_TURN as i32);
    } else {
        UI_RunLerpFrame(pi, &mut (*pi).legs, (*pi).legsAnim);
    }
    *legsOld = (*pi).legs.oldFrame;
    *legs = (*pi).legs.frame;
    *legsBackLerp = (*pi).legs.backlerp;
    // torso animation
    (*pi).torsoAnimationTimer -= uis.frametime;
    if (*pi).torsoAnimationTimer < 0 as i32 {
        (*pi).torsoAnimationTimer = 0 as i32
    }
    UI_TorsoSequencing(pi);
    UI_RunLerpFrame(pi, &mut (*pi).torso, (*pi).torsoAnim);
    *torsoOld = (*pi).torso.oldFrame;
    *torso = (*pi).torso.frame;
    *torsoBackLerp = (*pi).torso.backlerp;
}
/*
==================
UI_SwingAngles
==================
*/

unsafe extern "C" fn UI_SwingAngles(
    mut destination: f32,
    mut swingTolerance: f32,
    mut clampTolerance: f32,
    mut speed: f32,
    mut angle: *mut f32,
    mut swinging: *mut qboolean,
) {
    let mut swing: f32 = 0.;
    let mut move_0: f32 = 0.;
    let mut scale: f32 = 0.;
    if *swinging as u64 == 0 {
        // see if a swing should be started
        swing = AngleSubtract(*angle, destination);
        if swing > swingTolerance || swing < -swingTolerance {
            *swinging = qtrue
        }
    }
    if *swinging as u64 == 0 {
        return;
    }
    // modify the speed depending on the delta
    // so it doesn't seem so linear
    swing = AngleSubtract(destination, *angle);
    scale = crate::stdlib::fabs(swing as f64) as f32;
    if (scale as f64) < swingTolerance as f64 * 0.5f64 {
        scale = 0.5f64 as f32
    } else if scale < swingTolerance {
        scale = 1.0f64 as f32
    } else {
        scale = 2.0f64 as f32
    }
    // swing towards the destination angle
    if swing >= 0 as i32 as f32 {
        move_0 = uis.frametime as f32 * scale * speed;
        if move_0 >= swing {
            move_0 = swing;
            *swinging = qfalse
        }
        *angle = AngleMod(*angle + move_0)
    } else if swing < 0 as i32 as f32 {
        move_0 = uis.frametime as f32 * scale * -speed;
        if move_0 <= swing {
            move_0 = swing;
            *swinging = qfalse
        }
        *angle = AngleMod(*angle + move_0)
    }
    // clamp to no more than tolerance
    swing = AngleSubtract(destination, *angle);
    if swing > clampTolerance {
        *angle = AngleMod(destination - (clampTolerance - 1 as i32 as f32))
    } else if swing < -clampTolerance {
        *angle = AngleMod(destination + (clampTolerance - 1 as i32 as f32))
    };
}
/*
======================
UI_MovedirAdjustment
======================
*/

unsafe extern "C" fn UI_MovedirAdjustment(mut pi: *mut playerInfo_t) -> f32 {
    let mut relativeAngles: vec3_t = [0.; 3];
    let mut moveVector: vec3_t = [0.; 3];
    relativeAngles[0 as i32 as usize] =
        (*pi).viewAngles[0 as i32 as usize] - (*pi).moveAngles[0 as i32 as usize];
    relativeAngles[1 as i32 as usize] =
        (*pi).viewAngles[1 as i32 as usize] - (*pi).moveAngles[1 as i32 as usize];
    relativeAngles[2 as i32 as usize] =
        (*pi).viewAngles[2 as i32 as usize] - (*pi).moveAngles[2 as i32 as usize];
    AngleVectors(
        relativeAngles.as_mut_ptr() as *const vec_t,
        moveVector.as_mut_ptr(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );
    if (Q_fabs(moveVector[0 as i32 as usize]) as f64) < 0.01f64 {
        moveVector[0 as i32 as usize] = 0.0f64 as vec_t
    }
    if (Q_fabs(moveVector[1 as i32 as usize]) as f64) < 0.01f64 {
        moveVector[1 as i32 as usize] = 0.0f64 as vec_t
    }
    if moveVector[1 as i32 as usize] == 0 as i32 as f32
        && moveVector[0 as i32 as usize] > 0 as i32 as f32
    {
        return 0 as i32 as f32;
    }
    if moveVector[1 as i32 as usize] < 0 as i32 as f32
        && moveVector[0 as i32 as usize] > 0 as i32 as f32
    {
        return 22 as i32 as f32;
    }
    if moveVector[1 as i32 as usize] < 0 as i32 as f32
        && moveVector[0 as i32 as usize] == 0 as i32 as f32
    {
        return 45 as i32 as f32;
    }
    if moveVector[1 as i32 as usize] < 0 as i32 as f32
        && moveVector[0 as i32 as usize] < 0 as i32 as f32
    {
        return -(22 as i32) as f32;
    }
    if moveVector[1 as i32 as usize] == 0 as i32 as f32
        && moveVector[0 as i32 as usize] < 0 as i32 as f32
    {
        return 0 as i32 as f32;
    }
    if moveVector[1 as i32 as usize] > 0 as i32 as f32
        && moveVector[0 as i32 as usize] < 0 as i32 as f32
    {
        return 22 as i32 as f32;
    }
    if moveVector[1 as i32 as usize] > 0 as i32 as f32
        && moveVector[0 as i32 as usize] == 0 as i32 as f32
    {
        return -(45 as i32) as f32;
    }
    return -(22 as i32) as f32;
}
/*
===============
UI_PlayerAngles
===============
*/

unsafe extern "C" fn UI_PlayerAngles(
    mut pi: *mut playerInfo_t,
    mut legs: *mut vec3_t,
    mut torso: *mut vec3_t,
    mut head: *mut vec3_t,
) {
    let mut legsAngles: vec3_t = [0.; 3];
    let mut torsoAngles: vec3_t = [0.; 3];
    let mut headAngles: vec3_t = [0.; 3];
    let mut dest: f32 = 0.;
    let mut adjust: f32 = 0.;
    headAngles[0 as i32 as usize] = (*pi).viewAngles[0 as i32 as usize];
    headAngles[1 as i32 as usize] = (*pi).viewAngles[1 as i32 as usize];
    headAngles[2 as i32 as usize] = (*pi).viewAngles[2 as i32 as usize];
    headAngles[1 as i32 as usize] = AngleMod(headAngles[1 as i32 as usize]);
    legsAngles[2 as i32 as usize] = 0 as i32 as vec_t;
    legsAngles[1 as i32 as usize] = legsAngles[2 as i32 as usize];
    legsAngles[0 as i32 as usize] = legsAngles[1 as i32 as usize];
    torsoAngles[2 as i32 as usize] = 0 as i32 as vec_t;
    torsoAngles[1 as i32 as usize] = torsoAngles[2 as i32 as usize];
    torsoAngles[0 as i32 as usize] = torsoAngles[1 as i32 as usize];
    // --------- yaw -------------
    // allow yaw to drift a bit
    if (*pi).legsAnim & !(128 as i32) != LEGS_IDLE as i32
        || (*pi).torsoAnim & !(128 as i32) != TORSO_STAND as i32
    {
        // if not standing still, always point all in the same direction
        (*pi).torso.yawing = qtrue; // always center
                                    // always center
        (*pi).torso.pitching = qtrue; // always center
        (*pi).legs.yawing = qtrue
    }
    // adjust legs for movement dir
    adjust = UI_MovedirAdjustment(pi);
    legsAngles[1 as i32 as usize] = headAngles[1 as i32 as usize] + adjust;
    torsoAngles[1 as i32 as usize] =
        (headAngles[1 as i32 as usize] as f64 + 0.25f64 * adjust as f64) as vec_t;
    // torso
    UI_SwingAngles(
        torsoAngles[1 as i32 as usize],
        25 as i32 as f32,
        90 as i32 as f32,
        0.3f32,
        &mut (*pi).torso.yawAngle,
        &mut (*pi).torso.yawing,
    );
    UI_SwingAngles(
        legsAngles[1 as i32 as usize],
        40 as i32 as f32,
        90 as i32 as f32,
        0.3f32,
        &mut (*pi).legs.yawAngle,
        &mut (*pi).legs.yawing,
    );
    torsoAngles[1 as i32 as usize] = (*pi).torso.yawAngle;
    legsAngles[1 as i32 as usize] = (*pi).legs.yawAngle;
    // --------- pitch -------------
    // only show a fraction of the pitch angle in the torso
    if headAngles[0 as i32 as usize] > 180 as i32 as f32 {
        dest = ((-(360 as i32) as f32 + headAngles[0 as i32 as usize]) as f64 * 0.75f64) as f32
    } else {
        dest = (headAngles[0 as i32 as usize] as f64 * 0.75f64) as f32
    }
    UI_SwingAngles(
        dest,
        15 as i32 as f32,
        30 as i32 as f32,
        0.1f32,
        &mut (*pi).torso.pitchAngle,
        &mut (*pi).torso.pitching,
    );
    torsoAngles[0 as i32 as usize] = (*pi).torso.pitchAngle;
    if (*pi).fixedtorso as u64 != 0 {
        torsoAngles[0 as i32 as usize] = 0.0f32
    }
    if (*pi).fixedlegs as u64 != 0 {
        legsAngles[1 as i32 as usize] = torsoAngles[1 as i32 as usize];
        legsAngles[0 as i32 as usize] = 0.0f32;
        legsAngles[2 as i32 as usize] = 0.0f32
    }
    // pull the angles back out of the hierarchial chain
    AnglesSubtract(
        headAngles.as_mut_ptr(),
        torsoAngles.as_mut_ptr(),
        headAngles.as_mut_ptr(),
    );
    AnglesSubtract(
        torsoAngles.as_mut_ptr(),
        legsAngles.as_mut_ptr(),
        torsoAngles.as_mut_ptr(),
    );
    AnglesToAxis(legsAngles.as_mut_ptr() as *const vec_t, legs);
    AnglesToAxis(torsoAngles.as_mut_ptr() as *const vec_t, torso);
    AnglesToAxis(headAngles.as_mut_ptr() as *const vec_t, head);
}
/*
===============
UI_PlayerFloatSprite
===============
*/

unsafe extern "C" fn UI_PlayerFloatSprite(
    mut _pi: *mut playerInfo_t,
    mut origin: *mut vec_t,
    mut shader: qhandle_t,
) {
    let mut ent: refEntity_t = refEntity_t {
        reType: RT_MODEL,
        renderfx: 0,
        hModel: 0,
        lightingOrigin: [0.; 3],
        shadowPlane: 0.,
        axis: [[0.; 3]; 3],
        nonNormalizedAxes: qfalse,
        origin: [0.; 3],
        frame: 0,
        oldorigin: [0.; 3],
        oldframe: 0,
        backlerp: 0.,
        skinNum: 0,
        customSkin: 0,
        customShader: 0,
        shaderRGBA: [0; 4],
        shaderTexCoord: [0.; 2],
        shaderTime: 0.,
        radius: 0.,
        rotation: 0.,
    };
    crate::stdlib::memset(
        &mut ent as *mut refEntity_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<refEntity_t>() as usize,
    );
    ent.origin[0 as i32 as usize] = *origin.offset(0 as i32 as isize);
    ent.origin[1 as i32 as usize] = *origin.offset(1 as i32 as isize);
    ent.origin[2 as i32 as usize] = *origin.offset(2 as i32 as isize);
    ent.origin[2 as i32 as usize] += 48 as i32 as f32;
    ent.reType = RT_SPRITE;
    ent.customShader = shader;
    ent.radius = 10 as i32 as f32;
    ent.renderfx = 0 as i32;
    trap_R_AddRefEntityToScene(&mut ent as *mut _ as *const refEntity_t);
}
/*
======================
UI_MachinegunSpinAngle
======================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_MachinegunSpinAngle(mut pi: *mut playerInfo_t) -> f32 {
    let mut delta: i32 = 0;
    let mut angle: f32 = 0.;
    let mut speed: f32 = 0.;
    let mut torsoAnim: i32 = 0;
    delta = dp_realtime - (*pi).barrelTime;
    if (*pi).barrelSpinning as u64 != 0 {
        angle = (*pi).barrelAngle + delta as f32 * 0.9f32
    } else {
        if delta > 1000 as i32 {
            delta = 1000 as i32
        }
        speed =
            (0.5f64 * (0.9f32 + (1000 as i32 - delta) as f32 / 1000 as i32 as f32) as f64) as f32;
        angle = (*pi).barrelAngle + delta as f32 * speed
    }
    torsoAnim = (*pi).torsoAnim & !(128 as i32);
    if torsoAnim == TORSO_ATTACK2 as i32 {
        torsoAnim = TORSO_ATTACK as i32
    }
    if (*pi).barrelSpinning as u32 == !(torsoAnim == TORSO_ATTACK as i32) as i32 as u32 {
        (*pi).barrelTime = dp_realtime;
        (*pi).barrelAngle = AngleMod(angle);
        (*pi).barrelSpinning = (torsoAnim == TORSO_ATTACK as i32) as i32 as qboolean
    }
    return angle;
}
/*
===============
UI_DrawPlayer
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_DrawPlayer(
    mut x: f32,
    mut y: f32,
    mut w: f32,
    mut h: f32,
    mut pi: *mut playerInfo_t,
    mut time: i32,
) {
    let mut refdef: refdef_t = refdef_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        fov_x: 0.,
        fov_y: 0.,
        vieworg: [0.; 3],
        viewaxis: [[0.; 3]; 3],
        time: 0,
        rdflags: 0,
        areamask: [0; 32],
        text: [[0; 32]; 8],
    };
    let mut legs: refEntity_t = {
        let mut init = refEntity_t {
            reType: RT_MODEL,
            renderfx: 0,
            hModel: 0,
            lightingOrigin: [0.; 3],
            shadowPlane: 0.,
            axis: [[0.; 3]; 3],
            nonNormalizedAxes: qfalse,
            origin: [0.; 3],
            frame: 0,
            oldorigin: [0.; 3],
            oldframe: 0,
            backlerp: 0.,
            skinNum: 0,
            customSkin: 0,
            customShader: 0,
            shaderRGBA: [0; 4],
            shaderTexCoord: [0.; 2],
            shaderTime: 0.,
            radius: 0.,
            rotation: 0.,
        };
        init
    };
    let mut torso: refEntity_t = {
        let mut init = refEntity_t {
            reType: RT_MODEL,
            renderfx: 0,
            hModel: 0,
            lightingOrigin: [0.; 3],
            shadowPlane: 0.,
            axis: [[0.; 3]; 3],
            nonNormalizedAxes: qfalse,
            origin: [0.; 3],
            frame: 0,
            oldorigin: [0.; 3],
            oldframe: 0,
            backlerp: 0.,
            skinNum: 0,
            customSkin: 0,
            customShader: 0,
            shaderRGBA: [0; 4],
            shaderTexCoord: [0.; 2],
            shaderTime: 0.,
            radius: 0.,
            rotation: 0.,
        };
        init
    };
    let mut head: refEntity_t = {
        let mut init = refEntity_t {
            reType: RT_MODEL,
            renderfx: 0,
            hModel: 0,
            lightingOrigin: [0.; 3],
            shadowPlane: 0.,
            axis: [[0.; 3]; 3],
            nonNormalizedAxes: qfalse,
            origin: [0.; 3],
            frame: 0,
            oldorigin: [0.; 3],
            oldframe: 0,
            backlerp: 0.,
            skinNum: 0,
            customSkin: 0,
            customShader: 0,
            shaderRGBA: [0; 4],
            shaderTexCoord: [0.; 2],
            shaderTime: 0.,
            radius: 0.,
            rotation: 0.,
        };
        init
    };
    let mut gun: refEntity_t = {
        let mut init = refEntity_t {
            reType: RT_MODEL,
            renderfx: 0,
            hModel: 0,
            lightingOrigin: [0.; 3],
            shadowPlane: 0.,
            axis: [[0.; 3]; 3],
            nonNormalizedAxes: qfalse,
            origin: [0.; 3],
            frame: 0,
            oldorigin: [0.; 3],
            oldframe: 0,
            backlerp: 0.,
            skinNum: 0,
            customSkin: 0,
            customShader: 0,
            shaderRGBA: [0; 4],
            shaderTexCoord: [0.; 2],
            shaderTime: 0.,
            radius: 0.,
            rotation: 0.,
        };
        init
    };
    let mut barrel: refEntity_t = {
        let mut init = refEntity_t {
            reType: RT_MODEL,
            renderfx: 0,
            hModel: 0,
            lightingOrigin: [0.; 3],
            shadowPlane: 0.,
            axis: [[0.; 3]; 3],
            nonNormalizedAxes: qfalse,
            origin: [0.; 3],
            frame: 0,
            oldorigin: [0.; 3],
            oldframe: 0,
            backlerp: 0.,
            skinNum: 0,
            customSkin: 0,
            customShader: 0,
            shaderRGBA: [0; 4],
            shaderTexCoord: [0.; 2],
            shaderTime: 0.,
            radius: 0.,
            rotation: 0.,
        };
        init
    };
    let mut flash: refEntity_t = {
        let mut init = refEntity_t {
            reType: RT_MODEL,
            renderfx: 0,
            hModel: 0,
            lightingOrigin: [0.; 3],
            shadowPlane: 0.,
            axis: [[0.; 3]; 3],
            nonNormalizedAxes: qfalse,
            origin: [0.; 3],
            frame: 0,
            oldorigin: [0.; 3],
            oldframe: 0,
            backlerp: 0.,
            skinNum: 0,
            customSkin: 0,
            customShader: 0,
            shaderRGBA: [0; 4],
            shaderTexCoord: [0.; 2],
            shaderTime: 0.,
            radius: 0.,
            rotation: 0.,
        };
        init
    };
    let mut origin: vec3_t = [0.; 3];
    let mut renderfx: i32 = 0;
    let mut mins: vec3_t = [
        -(16 as i32) as vec_t,
        -(16 as i32) as vec_t,
        -(24 as i32) as vec_t,
    ];
    let mut maxs: vec3_t = [16 as i32 as vec_t, 16 as i32 as vec_t, 32 as i32 as vec_t];
    let mut len: f32 = 0.;
    let mut xx: f32 = 0.;
    if (*pi).legsModel == 0
        || (*pi).torsoModel == 0
        || (*pi).headModel == 0
        || (*pi).animations[0 as i32 as usize].numFrames == 0
    {
        return;
    }
    dp_realtime = time;
    if (*pi).pendingWeapon as u32 != WP_NUM_WEAPONS as i32 as u32 && dp_realtime > (*pi).weaponTimer
    {
        (*pi).weapon = (*pi).pendingWeapon;
        (*pi).lastWeapon = (*pi).pendingWeapon;
        (*pi).pendingWeapon = WP_NUM_WEAPONS;
        (*pi).weaponTimer = 0 as i32;
        if (*pi).currentWeapon as u32 != (*pi).weapon as u32 {
            trap_S_StartLocalSound(weaponChangeSound, CHAN_LOCAL as i32);
        }
    }
    UI_AdjustFrom640(&mut x, &mut y, &mut w, &mut h);
    y -= jumpHeight;
    crate::stdlib::memset(
        &mut refdef as *mut refdef_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<refdef_t>() as usize,
    );
    crate::stdlib::memset(
        &mut legs as *mut refEntity_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<refEntity_t>() as usize,
    );
    crate::stdlib::memset(
        &mut torso as *mut refEntity_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<refEntity_t>() as usize,
    );
    crate::stdlib::memset(
        &mut head as *mut refEntity_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<refEntity_t>() as usize,
    );
    refdef.rdflags = 0x1 as i32;
    AxisClear(refdef.viewaxis.as_mut_ptr());
    refdef.x = x as i32;
    refdef.y = y as i32;
    refdef.width = w as i32;
    refdef.height = h as i32;
    refdef.fov_x = (refdef.width as f32 / uis.xscale / 640.0f32 * 90.0f32) as i32 as f32;
    xx = ((refdef.width as f32 / uis.xscale) as f64
        / crate::stdlib::tan((refdef.fov_x / 360 as i32 as f32) as f64 * 3.14159265358979323846f64))
        as f32;
    refdef.fov_y =
        crate::stdlib::atan2((refdef.height as f32 / uis.yscale) as f64, xx as f64) as f32;
    refdef.fov_y = (refdef.fov_y as f64 * (360 as i32 as f64 / 3.14159265358979323846f64)) as f32;
    // calculate distance so the player nearly fills the box
    len = (0.7f64 * (maxs[2 as i32 as usize] - mins[2 as i32 as usize]) as f64) as f32;
    origin[0 as i32 as usize] = (len as f64
        / crate::stdlib::tan(
            refdef.fov_x as f64 * 3.14159265358979323846f64 / 180.0f32 as f64 * 0.5f64,
        )) as vec_t;
    origin[1 as i32 as usize] =
        (0.5f64 * (mins[1 as i32 as usize] + maxs[1 as i32 as usize]) as f64) as vec_t;
    origin[2 as i32 as usize] =
        (-0.5f64 * (mins[2 as i32 as usize] + maxs[2 as i32 as usize]) as f64) as vec_t;
    refdef.time = dp_realtime;
    trap_R_ClearScene();
    // get the rotation information
    UI_PlayerAngles(
        pi,
        legs.axis.as_mut_ptr(),
        torso.axis.as_mut_ptr(),
        head.axis.as_mut_ptr(),
    );
    // get the animation state (after rotation, to allow feet shuffle)
    UI_PlayerAnimation(
        pi,
        &mut legs.oldframe,
        &mut legs.frame,
        &mut legs.backlerp,
        &mut torso.oldframe,
        &mut torso.frame,
        &mut torso.backlerp,
    );
    renderfx = 0x80 as i32 | 0x40 as i32;
    //
    // add the legs
    //
    legs.hModel = (*pi).legsModel;
    legs.customSkin = (*pi).legsSkin;
    legs.origin[0 as i32 as usize] = origin[0 as i32 as usize];
    legs.origin[1 as i32 as usize] = origin[1 as i32 as usize];
    legs.origin[2 as i32 as usize] = origin[2 as i32 as usize];
    legs.lightingOrigin[0 as i32 as usize] = origin[0 as i32 as usize];
    legs.lightingOrigin[1 as i32 as usize] = origin[1 as i32 as usize];
    legs.lightingOrigin[2 as i32 as usize] = origin[2 as i32 as usize];
    legs.renderfx = renderfx;
    legs.oldorigin[0 as i32 as usize] = legs.origin[0 as i32 as usize];
    legs.oldorigin[1 as i32 as usize] = legs.origin[1 as i32 as usize];
    legs.oldorigin[2 as i32 as usize] = legs.origin[2 as i32 as usize];
    trap_R_AddRefEntityToScene(&mut legs as *mut _ as *const refEntity_t);
    if legs.hModel == 0 {
        return;
    }
    //
    // add the torso
    //
    torso.hModel = (*pi).torsoModel;
    if torso.hModel == 0 {
        return;
    }
    torso.customSkin = (*pi).torsoSkin;
    torso.lightingOrigin[0 as i32 as usize] = origin[0 as i32 as usize];
    torso.lightingOrigin[1 as i32 as usize] = origin[1 as i32 as usize];
    torso.lightingOrigin[2 as i32 as usize] = origin[2 as i32 as usize];
    UI_PositionRotatedEntityOnTag(
        &mut torso,
        &mut legs,
        (*pi).legsModel,
        b"tag_torso\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    torso.renderfx = renderfx;
    trap_R_AddRefEntityToScene(&mut torso as *mut _ as *const refEntity_t);
    //
    // add the head
    //
    head.hModel = (*pi).headModel;
    if head.hModel == 0 {
        return;
    }
    head.customSkin = (*pi).headSkin;
    head.lightingOrigin[0 as i32 as usize] = origin[0 as i32 as usize];
    head.lightingOrigin[1 as i32 as usize] = origin[1 as i32 as usize];
    head.lightingOrigin[2 as i32 as usize] = origin[2 as i32 as usize];
    UI_PositionRotatedEntityOnTag(
        &mut head,
        &mut torso,
        (*pi).torsoModel,
        b"tag_head\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    head.renderfx = renderfx;
    trap_R_AddRefEntityToScene(&mut head as *mut _ as *const refEntity_t);
    //
    // add the gun
    //
    if (*pi).currentWeapon as u32 != WP_NONE as i32 as u32 {
        crate::stdlib::memset(
            &mut gun as *mut refEntity_t as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<refEntity_t>() as usize,
        );
        gun.hModel = (*pi).weaponModel;
        if (*pi).currentWeapon as u32 == WP_RAILGUN as i32 as u32 {
            gun.shaderRGBA[0 as i32 as usize] = (*pi).c1RGBA[0 as i32 as usize];
            gun.shaderRGBA[1 as i32 as usize] = (*pi).c1RGBA[1 as i32 as usize];
            gun.shaderRGBA[2 as i32 as usize] = (*pi).c1RGBA[2 as i32 as usize];
            gun.shaderRGBA[3 as i32 as usize] = (*pi).c1RGBA[3 as i32 as usize]
        } else {
            gun.shaderRGBA[0 as i32 as usize] = colorWhite[0 as i32 as usize] as byte;
            gun.shaderRGBA[1 as i32 as usize] = colorWhite[1 as i32 as usize] as byte;
            gun.shaderRGBA[2 as i32 as usize] = colorWhite[2 as i32 as usize] as byte;
            gun.shaderRGBA[3 as i32 as usize] = colorWhite[3 as i32 as usize] as byte
        }
        gun.lightingOrigin[0 as i32 as usize] = origin[0 as i32 as usize];
        gun.lightingOrigin[1 as i32 as usize] = origin[1 as i32 as usize];
        gun.lightingOrigin[2 as i32 as usize] = origin[2 as i32 as usize];
        UI_PositionEntityOnTag(
            &mut gun,
            &mut torso,
            (*pi).torsoModel,
            b"tag_weapon\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        gun.renderfx = renderfx;
        trap_R_AddRefEntityToScene(&mut gun as *mut _ as *const refEntity_t);
    }
    //
    // add the spinning barrel
    //
    if (*pi).realWeapon == WP_MACHINEGUN as i32
        || (*pi).realWeapon == WP_GAUNTLET as i32
        || (*pi).realWeapon == WP_BFG as i32
    {
        let mut angles: vec3_t = [0.; 3];
        crate::stdlib::memset(
            &mut barrel as *mut refEntity_t as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<refEntity_t>() as usize,
        );
        barrel.lightingOrigin[0 as i32 as usize] = origin[0 as i32 as usize];
        barrel.lightingOrigin[1 as i32 as usize] = origin[1 as i32 as usize];
        barrel.lightingOrigin[2 as i32 as usize] = origin[2 as i32 as usize];
        barrel.renderfx = renderfx;
        barrel.hModel = (*pi).barrelModel;
        angles[1 as i32 as usize] = 0 as i32 as vec_t;
        angles[0 as i32 as usize] = 0 as i32 as vec_t;
        angles[2 as i32 as usize] = UI_MachinegunSpinAngle(pi);
        AnglesToAxis(
            angles.as_mut_ptr() as *const vec_t,
            barrel.axis.as_mut_ptr(),
        );
        UI_PositionRotatedEntityOnTag(
            &mut barrel,
            &mut gun,
            (*pi).weaponModel,
            b"tag_barrel\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        trap_R_AddRefEntityToScene(&mut barrel as *mut _ as *const refEntity_t);
    }
    //
    // add muzzle flash
    //
    if dp_realtime <= (*pi).muzzleFlashTime {
        if (*pi).flashModel != 0 {
            crate::stdlib::memset(
                &mut flash as *mut refEntity_t as *mut libc::c_void,
                0 as i32,
                ::std::mem::size_of::<refEntity_t>() as usize,
            );
            flash.hModel = (*pi).flashModel;
            if (*pi).currentWeapon as u32 == WP_RAILGUN as i32 as u32 {
                flash.shaderRGBA[0 as i32 as usize] = (*pi).c1RGBA[0 as i32 as usize];
                flash.shaderRGBA[1 as i32 as usize] = (*pi).c1RGBA[1 as i32 as usize];
                flash.shaderRGBA[2 as i32 as usize] = (*pi).c1RGBA[2 as i32 as usize];
                flash.shaderRGBA[3 as i32 as usize] = (*pi).c1RGBA[3 as i32 as usize]
            } else {
                flash.shaderRGBA[0 as i32 as usize] = colorWhite[0 as i32 as usize] as byte;
                flash.shaderRGBA[1 as i32 as usize] = colorWhite[1 as i32 as usize] as byte;
                flash.shaderRGBA[2 as i32 as usize] = colorWhite[2 as i32 as usize] as byte;
                flash.shaderRGBA[3 as i32 as usize] = colorWhite[3 as i32 as usize] as byte
            }
            flash.lightingOrigin[0 as i32 as usize] = origin[0 as i32 as usize];
            flash.lightingOrigin[1 as i32 as usize] = origin[1 as i32 as usize];
            flash.lightingOrigin[2 as i32 as usize] = origin[2 as i32 as usize];
            UI_PositionEntityOnTag(
                &mut flash,
                &mut gun,
                (*pi).weaponModel,
                b"tag_flash\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            flash.renderfx = renderfx;
            trap_R_AddRefEntityToScene(&mut flash as *mut _ as *const refEntity_t);
        }
        // make a dlight for the flash
        if (*pi).flashDlightColor[0 as i32 as usize] != 0.
            || (*pi).flashDlightColor[1 as i32 as usize] != 0.
            || (*pi).flashDlightColor[2 as i32 as usize] != 0.
        {
            trap_R_AddLightToScene(
                flash.origin.as_mut_ptr() as *const vec_t,
                (200 as i32 + (rand() & 31 as i32)) as f32,
                (*pi).flashDlightColor[0 as i32 as usize],
                (*pi).flashDlightColor[1 as i32 as usize],
                (*pi).flashDlightColor[2 as i32 as usize],
            );
        }
    }
    //
    // add the chat icon
    //
    if (*pi).chat as u64 != 0 {
        UI_PlayerFloatSprite(
            pi,
            origin.as_mut_ptr(),
            trap_R_RegisterShaderNoMip(b"sprites/balloon3\x00" as *const u8 as *const libc::c_char),
        );
    }
    //
    // add an accent light
    //
    origin[0 as i32 as usize] -= 100 as i32 as f32; // + = behind, - = in front
    origin[1 as i32 as usize] += 100 as i32 as f32; // + = left, - = right
    origin[2 as i32 as usize] += 100 as i32 as f32; // + = above, - = below
    trap_R_AddLightToScene(
        origin.as_mut_ptr() as *const vec_t,
        500 as i32 as f32,
        1.0f64 as f32,
        1.0f64 as f32,
        1.0f64 as f32,
    );
    origin[0 as i32 as usize] -= 100 as i32 as f32;
    origin[1 as i32 as usize] -= 100 as i32 as f32;
    origin[2 as i32 as usize] -= 100 as i32 as f32;
    trap_R_AddLightToScene(
        origin.as_mut_ptr() as *const vec_t,
        500 as i32 as f32,
        1.0f64 as f32,
        0.0f64 as f32,
        0.0f64 as f32,
    );
    trap_R_RenderScene(&mut refdef as *mut _ as *const refdef_t);
}
/*
==========================
UI_RegisterClientSkin
==========================
*/

unsafe extern "C" fn UI_RegisterClientSkin(
    mut pi: *mut playerInfo_t,
    mut modelName: *const libc::c_char,
    mut skinName: *const libc::c_char,
) -> qboolean {
    let mut filename: [libc::c_char; 64] = [0; 64];
    Com_sprintf(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
        b"models/players/%s/lower_%s.skin\x00" as *const u8 as *const libc::c_char,
        modelName,
        skinName,
    );
    (*pi).legsSkin = trap_R_RegisterSkin(filename.as_mut_ptr());
    Com_sprintf(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
        b"models/players/%s/upper_%s.skin\x00" as *const u8 as *const libc::c_char,
        modelName,
        skinName,
    );
    (*pi).torsoSkin = trap_R_RegisterSkin(filename.as_mut_ptr());
    Com_sprintf(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
        b"models/players/%s/head_%s.skin\x00" as *const u8 as *const libc::c_char,
        modelName,
        skinName,
    );
    (*pi).headSkin = trap_R_RegisterSkin(filename.as_mut_ptr());
    if (*pi).legsSkin == 0 || (*pi).torsoSkin == 0 || (*pi).headSkin == 0 {
        return qfalse;
    }
    return qtrue;
}
/*
======================
UI_ParseAnimationFile
======================
*/

unsafe extern "C" fn UI_ParseAnimationFile(
    mut filename: *const libc::c_char,
    mut pi: *mut playerInfo_t,
) -> qboolean {
    let mut text_p: *mut libc::c_char = std::ptr::null_mut();
    let mut prev: *mut libc::c_char = std::ptr::null_mut();
    let mut len: i32 = 0;
    let mut i: i32 = 0;
    let mut token: *mut libc::c_char = std::ptr::null_mut();
    let mut fps: f32 = 0.;
    let mut skip: i32 = 0;
    let mut text: [libc::c_char; 20000] = [0; 20000];
    let mut f: fileHandle_t = 0;
    let mut animations: *mut animation_t = std::ptr::null_mut();
    animations = (*pi).animations.as_mut_ptr();
    crate::stdlib::memset(
        animations as *mut libc::c_void,
        0 as i32,
        (::std::mem::size_of::<animation_t>() as usize)
            .wrapping_mul(MAX_ANIMATIONS as i32 as usize),
    );
    (*pi).fixedlegs = qfalse;
    (*pi).fixedtorso = qfalse;
    // load the file
    len = trap_FS_FOpenFile(filename, &mut f, FS_READ);
    if len <= 0 as i32 {
        return qfalse;
    }
    if len as usize
        >= (::std::mem::size_of::<[libc::c_char; 20000]>() as usize).wrapping_sub(1 as i32 as usize)
    {
        Com_Printf(
            b"File %s too long\n\x00" as *const u8 as *const libc::c_char,
            filename,
        );
        trap_FS_FCloseFile(f);
        return qfalse;
    }
    trap_FS_Read(text.as_mut_ptr() as *mut libc::c_void, len, f);
    text[len as usize] = 0 as i32 as libc::c_char;
    trap_FS_FCloseFile(f);
    // parse the text
    text_p = text.as_mut_ptr(); // quite the compiler warning
    skip = 0 as i32;
    loop
    // read optional parameters
    {
        prev = text_p; // so we can unget
        token = COM_Parse(&mut text_p);
        if *token.offset(0 as i32 as isize) == 0 {
            break;
        }
        if Q_stricmp(token, b"footsteps\x00" as *const u8 as *const libc::c_char) == 0 {
            token = COM_Parse(&mut text_p);
            if *token.offset(0 as i32 as isize) == 0 {
                break;
            }
        } else if Q_stricmp(token, b"headoffset\x00" as *const u8 as *const libc::c_char) == 0 {
            i = 0 as i32;
            while i < 3 as i32 {
                token = COM_Parse(&mut text_p);
                if *token.offset(0 as i32 as isize) == 0 {
                    break;
                }
                i += 1
            }
        } else if Q_stricmp(token, b"sex\x00" as *const u8 as *const libc::c_char) == 0 {
            token = COM_Parse(&mut text_p);
            if *token.offset(0 as i32 as isize) == 0 {
                break;
            }
        } else if Q_stricmp(token, b"fixedlegs\x00" as *const u8 as *const libc::c_char) == 0 {
            (*pi).fixedlegs = qtrue
        } else if Q_stricmp(token, b"fixedtorso\x00" as *const u8 as *const libc::c_char) == 0 {
            (*pi).fixedtorso = qtrue
        } else if *token.offset(0 as i32 as isize) as i32 >= '0' as i32
            && *token.offset(0 as i32 as isize) as i32 <= '9' as i32
        {
            // if it is a number, start parsing animations
            text_p = prev; // unget the token
            break;
        } else {
            Com_Printf(
                b"unknown token \'%s\' in %s\n\x00" as *const u8 as *const libc::c_char,
                token,
                filename,
            );
        }
    }
    // read information for each frame
    i = 0 as i32;
    while i < MAX_ANIMATIONS as i32 {
        token = COM_Parse(&mut text_p);
        if *token.offset(0 as i32 as isize) == 0 {
            if !(i >= TORSO_GETFLAG as i32 && i <= TORSO_NEGATIVE as i32) {
                break;
            }
            (*animations.offset(i as isize)).firstFrame =
                (*animations.offset(TORSO_GESTURE as i32 as isize)).firstFrame;
            (*animations.offset(i as isize)).frameLerp =
                (*animations.offset(TORSO_GESTURE as i32 as isize)).frameLerp;
            (*animations.offset(i as isize)).initialLerp =
                (*animations.offset(TORSO_GESTURE as i32 as isize)).initialLerp;
            (*animations.offset(i as isize)).loopFrames =
                (*animations.offset(TORSO_GESTURE as i32 as isize)).loopFrames;
            (*animations.offset(i as isize)).numFrames =
                (*animations.offset(TORSO_GESTURE as i32 as isize)).numFrames;
            (*animations.offset(i as isize)).reversed = qfalse as i32;
            (*animations.offset(i as isize)).flipflop = qfalse as i32
        } else {
            (*animations.offset(i as isize)).firstFrame = atoi(token);
            // leg only frames are adjusted to not count the upper body only frames
            if i == LEGS_WALKCR as i32 {
                skip = (*animations.offset(LEGS_WALKCR as i32 as isize)).firstFrame
                    - (*animations.offset(TORSO_GESTURE as i32 as isize)).firstFrame
            }
            if i >= LEGS_WALKCR as i32 && i < TORSO_GETFLAG as i32 {
                (*animations.offset(i as isize)).firstFrame -= skip
            }
            token = COM_Parse(&mut text_p);
            if *token.offset(0 as i32 as isize) == 0 {
                break;
            }
            (*animations.offset(i as isize)).numFrames = atoi(token);
            (*animations.offset(i as isize)).reversed = qfalse as i32;
            (*animations.offset(i as isize)).flipflop = qfalse as i32;
            // if numFrames is negative the animation is reversed
            if (*animations.offset(i as isize)).numFrames < 0 as i32 {
                (*animations.offset(i as isize)).numFrames =
                    -(*animations.offset(i as isize)).numFrames;
                (*animations.offset(i as isize)).reversed = qtrue as i32
            }
            token = COM_Parse(&mut text_p);
            if *token.offset(0 as i32 as isize) == 0 {
                break;
            }
            (*animations.offset(i as isize)).loopFrames = atoi(token);
            token = COM_Parse(&mut text_p);
            if *token.offset(0 as i32 as isize) == 0 {
                break;
            }
            fps = atof(token) as f32;
            if fps == 0 as i32 as f32 {
                fps = 1 as i32 as f32
            }
            (*animations.offset(i as isize)).frameLerp = (1000 as i32 as f32 / fps) as i32;
            (*animations.offset(i as isize)).initialLerp = (1000 as i32 as f32 / fps) as i32
        }
        i += 1
    }
    if i != MAX_ANIMATIONS as i32 {
        Com_Printf(
            b"Error parsing animation file: %s\n\x00" as *const u8 as *const libc::c_char,
            filename,
        );
        return qfalse;
    }
    return qtrue;
}
/*
==========================
UI_RegisterClientModelname
==========================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_RegisterClientModelname(
    mut pi: *mut playerInfo_t,
    mut modelSkinName: *const libc::c_char,
) -> qboolean {
    let mut modelName: [libc::c_char; 64] = [0; 64];
    let mut skinName: [libc::c_char; 64] = [0; 64];
    let mut filename: [libc::c_char; 64] = [0; 64];
    let mut slash: *mut libc::c_char = std::ptr::null_mut();
    (*pi).torsoModel = 0 as i32;
    (*pi).headModel = 0 as i32;
    if *modelSkinName.offset(0 as i32 as isize) == 0 {
        return qfalse;
    }
    Q_strncpyz(
        modelName.as_mut_ptr(),
        modelSkinName,
        ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
    );
    slash = libc::strchr(modelName.as_mut_ptr(), '/' as i32);
    if slash.is_null() {
        // modelName did not include a skin name
        Q_strncpyz(
            skinName.as_mut_ptr(),
            b"default\x00" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
        );
    } else {
        Q_strncpyz(
            skinName.as_mut_ptr(),
            slash.offset(1 as i32 as isize),
            ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
        );
        // truncate modelName
        *slash = 0 as i32 as libc::c_char
    }
    // load cmodels before models so filecache works
    Com_sprintf(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
        b"models/players/%s/lower.md3\x00" as *const u8 as *const libc::c_char,
        modelName.as_mut_ptr(),
    );
    (*pi).legsModel = trap_R_RegisterModel(filename.as_mut_ptr());
    if (*pi).legsModel == 0 {
        Com_Printf(
            b"Failed to load model file %s\n\x00" as *const u8 as *const libc::c_char,
            filename.as_mut_ptr(),
        );
        return qfalse;
    }
    Com_sprintf(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
        b"models/players/%s/upper.md3\x00" as *const u8 as *const libc::c_char,
        modelName.as_mut_ptr(),
    );
    (*pi).torsoModel = trap_R_RegisterModel(filename.as_mut_ptr());
    if (*pi).torsoModel == 0 {
        Com_Printf(
            b"Failed to load model file %s\n\x00" as *const u8 as *const libc::c_char,
            filename.as_mut_ptr(),
        );
        return qfalse;
    }
    Com_sprintf(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
        b"models/players/%s/head.md3\x00" as *const u8 as *const libc::c_char,
        modelName.as_mut_ptr(),
    );
    (*pi).headModel = trap_R_RegisterModel(filename.as_mut_ptr());
    if (*pi).headModel == 0 {
        Com_Printf(
            b"Failed to load model file %s\n\x00" as *const u8 as *const libc::c_char,
            filename.as_mut_ptr(),
        );
        return qfalse;
    }
    // if any skins failed to load, fall back to default
    if UI_RegisterClientSkin(pi, modelName.as_mut_ptr(), skinName.as_mut_ptr()) as u64 == 0 {
        if UI_RegisterClientSkin(
            pi,
            modelName.as_mut_ptr(),
            b"default\x00" as *const u8 as *const libc::c_char,
        ) as u64
            == 0
        {
            Com_Printf(
                b"Failed to load skin file: %s : %s\n\x00" as *const u8 as *const libc::c_char,
                modelName.as_mut_ptr(),
                skinName.as_mut_ptr(),
            );
            return qfalse;
        }
    }
    // load the animations
    Com_sprintf(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
        b"models/players/%s/animation.cfg\x00" as *const u8 as *const libc::c_char,
        modelName.as_mut_ptr(),
    );
    if UI_ParseAnimationFile(filename.as_mut_ptr(), pi) as u64 == 0 {
        Com_Printf(
            b"Failed to load animation file %s\n\x00" as *const u8 as *const libc::c_char,
            filename.as_mut_ptr(),
        );
        return qfalse;
    }
    return qtrue;
}
/*
===============
UI_PlayerInfo_SetModel
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_PlayerInfo_SetModel(
    mut pi: *mut playerInfo_t,
    mut model: *const libc::c_char,
) {
    crate::stdlib::memset(
        pi as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<playerInfo_t>() as usize,
    );
    UI_RegisterClientModelname(pi, model);
    (*pi).weapon = WP_MACHINEGUN;
    (*pi).currentWeapon = (*pi).weapon;
    (*pi).lastWeapon = (*pi).weapon;
    (*pi).pendingWeapon = WP_NUM_WEAPONS;
    (*pi).weaponTimer = 0 as i32;
    (*pi).chat = qfalse;
    (*pi).newModel = qtrue;
    UI_PlayerInfo_SetWeapon(pi, (*pi).weapon);
}
/*
===========================================================================
Copyright (C) 1999-2005 Id Software, Inc.

This file is part of Quake III Arena source code.

Quake III Arena source code is free software; you can redistribute it
and/or modify it under the terms of the GNU General Public License as
published by the Free Software Foundation; either version 2 of the License,
or (at your option) any later version.

Quake III Arena source code is distributed in the hope that it will be
useful, but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Quake III Arena source code; if not, write to the Free Software
Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
===========================================================================
*/
//
//NOTE: include the ui_public.h from the new UI
//redefine to old API version
//
// ui_qmenu.c
//
// edit field is only numbers
// steady focus
// pulse if focus
// only mouse input allowed
// skips drawing
// grays and disables
// disables any input
// skip default initialization
// edit field is all lower case
// edit field is all upper case
// callback notifications
//
// ui_mfield.c
//
//
// ui_menu.c
//
//
// ui_credits.c
//
//
// ui_ingame.c
//
//
// ui_confirm.c
//
//
// ui_setup.c
//
//
// ui_team.c
//
//
// ui_connect.c
//
//
// ui_controls2.c
//
//
// ui_demo2.c
//
//
// ui_cinematics.c
//
//
// ui_mods.c
//
//
// ui_cdkey.c
//
//
// ui_playermodel.c
//
//
// ui_playersettings.c
//
//
// ui_preferences.c
//
//
// ui_specifyleague.c
//
//
// ui_specifyserver.c
//
//
// ui_servers2.c
//
//
// ui_startserver.c
//
//
// ui_serverinfo.c
//
//
// ui_video.c
//
//
// ui_players.c
//
//FIXME ripped from cg_local.h
// time when ->oldFrame was exactly on
// time when ->frame will be exactly on
// may include ANIM_TOGGLEBIT
// time when the first frame of the animation will be exact
// model info
// true if legs yaw is always the same as torso yaw
// true if torso never changes yaw
// currently in use drawing parms
// animation vars
/*
===============
UI_PlayerInfo_SetInfo
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_PlayerInfo_SetInfo(
    mut pi: *mut playerInfo_t,
    mut legsAnim: i32,
    mut torsoAnim: i32,
    mut viewAngles: *mut vec_t,
    mut moveAngles: *mut vec_t,
    mut weaponNumber: weapon_t,
    mut chat: qboolean,
) {
    let mut currentAnim: i32 = 0;
    let mut weaponNum: weapon_t = WP_NONE;
    let mut c: i32 = 0;
    (*pi).chat = chat;
    c = trap_Cvar_VariableValue(b"color1\x00" as *const u8 as *const libc::c_char) as i32;
    (*pi).color1[2 as i32 as usize] = 0 as i32 as vec_t;
    (*pi).color1[1 as i32 as usize] = (*pi).color1[2 as i32 as usize];
    (*pi).color1[0 as i32 as usize] = (*pi).color1[1 as i32 as usize];
    if c < 1 as i32 || c > 7 as i32 {
        (*pi).color1[0 as i32 as usize] = 1 as i32 as vec_t;
        (*pi).color1[1 as i32 as usize] = 1 as i32 as vec_t;
        (*pi).color1[2 as i32 as usize] = 1 as i32 as vec_t
    } else {
        if c & 1 as i32 != 0 {
            (*pi).color1[2 as i32 as usize] = 1.0f32
        }
        if c & 2 as i32 != 0 {
            (*pi).color1[1 as i32 as usize] = 1.0f32
        }
        if c & 4 as i32 != 0 {
            (*pi).color1[0 as i32 as usize] = 1.0f32
        }
    }
    (*pi).c1RGBA[0 as i32 as usize] = (255 as i32 as f32 * (*pi).color1[0 as i32 as usize]) as byte;
    (*pi).c1RGBA[1 as i32 as usize] = (255 as i32 as f32 * (*pi).color1[1 as i32 as usize]) as byte;
    (*pi).c1RGBA[2 as i32 as usize] = (255 as i32 as f32 * (*pi).color1[2 as i32 as usize]) as byte;
    (*pi).c1RGBA[3 as i32 as usize] = 255 as i32 as byte;
    // view angles
    (*pi).viewAngles[0 as i32 as usize] = *viewAngles.offset(0 as i32 as isize);
    (*pi).viewAngles[1 as i32 as usize] = *viewAngles.offset(1 as i32 as isize);
    (*pi).viewAngles[2 as i32 as usize] = *viewAngles.offset(2 as i32 as isize);
    // move angles
    (*pi).moveAngles[0 as i32 as usize] = *moveAngles.offset(0 as i32 as isize);
    (*pi).moveAngles[1 as i32 as usize] = *moveAngles.offset(1 as i32 as isize);
    (*pi).moveAngles[2 as i32 as usize] = *moveAngles.offset(2 as i32 as isize);
    if (*pi).newModel as u64 != 0 {
        (*pi).newModel = qfalse;
        jumpHeight = 0 as i32 as f32;
        (*pi).pendingLegsAnim = 0 as i32;
        UI_ForceLegsAnim(pi, legsAnim);
        (*pi).legs.yawAngle = *viewAngles.offset(1 as i32 as isize);
        (*pi).legs.yawing = qfalse;
        (*pi).pendingTorsoAnim = 0 as i32;
        UI_ForceTorsoAnim(pi, torsoAnim);
        (*pi).torso.yawAngle = *viewAngles.offset(1 as i32 as isize);
        (*pi).torso.yawing = qfalse;
        if weaponNumber as u32 != WP_NUM_WEAPONS as i32 as u32 {
            (*pi).weapon = weaponNumber;
            (*pi).currentWeapon = weaponNumber;
            (*pi).lastWeapon = weaponNumber;
            (*pi).pendingWeapon = WP_NUM_WEAPONS;
            (*pi).weaponTimer = 0 as i32;
            UI_PlayerInfo_SetWeapon(pi, (*pi).weapon);
        }
        return;
    }
    // weapon
    if weaponNumber as u32 == WP_NUM_WEAPONS as i32 as u32 {
        (*pi).pendingWeapon = WP_NUM_WEAPONS;
        (*pi).weaponTimer = 0 as i32
    } else if weaponNumber as u32 != WP_NONE as i32 as u32 {
        (*pi).pendingWeapon = weaponNumber;
        (*pi).weaponTimer = dp_realtime + 250 as i32
    }
    weaponNum = (*pi).lastWeapon;
    (*pi).weapon = weaponNum;
    if torsoAnim == BOTH_DEATH1 as i32 || legsAnim == BOTH_DEATH1 as i32 {
        legsAnim = BOTH_DEATH1 as i32;
        torsoAnim = legsAnim;
        (*pi).currentWeapon = WP_NONE;
        (*pi).weapon = (*pi).currentWeapon;
        UI_PlayerInfo_SetWeapon(pi, (*pi).weapon);
        jumpHeight = 0 as i32 as f32;
        (*pi).pendingLegsAnim = 0 as i32;
        UI_ForceLegsAnim(pi, legsAnim);
        (*pi).pendingTorsoAnim = 0 as i32;
        UI_ForceTorsoAnim(pi, torsoAnim);
        return;
    }
    // leg animation
    currentAnim = (*pi).legsAnim & !(128 as i32);
    if legsAnim != LEGS_JUMP as i32
        && (currentAnim == LEGS_JUMP as i32 || currentAnim == LEGS_LAND as i32)
    {
        (*pi).pendingLegsAnim = legsAnim
    } else if legsAnim != currentAnim {
        jumpHeight = 0 as i32 as f32;
        (*pi).pendingLegsAnim = 0 as i32;
        UI_ForceLegsAnim(pi, legsAnim);
    }
    // torso animation
    if torsoAnim == TORSO_STAND as i32 || torsoAnim == TORSO_STAND2 as i32 {
        if weaponNum as u32 == WP_NONE as i32 as u32
            || weaponNum as u32 == WP_GAUNTLET as i32 as u32
        {
            torsoAnim = TORSO_STAND2 as i32
        } else {
            torsoAnim = TORSO_STAND as i32
        }
    }
    if torsoAnim == TORSO_ATTACK as i32 || torsoAnim == TORSO_ATTACK2 as i32 {
        if weaponNum as u32 == WP_NONE as i32 as u32
            || weaponNum as u32 == WP_GAUNTLET as i32 as u32
        {
            torsoAnim = TORSO_ATTACK2 as i32
        } else {
            torsoAnim = TORSO_ATTACK as i32
        }
        (*pi).muzzleFlashTime = dp_realtime + 20 as i32
        //FIXME play firing sound here
    }
    currentAnim = (*pi).torsoAnim & !(128 as i32);
    if weaponNum as u32 != (*pi).currentWeapon as u32
        || currentAnim == TORSO_RAISE as i32
        || currentAnim == TORSO_DROP as i32
    {
        (*pi).pendingTorsoAnim = torsoAnim
    } else if (currentAnim == TORSO_GESTURE as i32 || currentAnim == TORSO_ATTACK as i32)
        && torsoAnim != currentAnim
    {
        (*pi).pendingTorsoAnim = torsoAnim
    } else if torsoAnim != currentAnim {
        (*pi).pendingTorsoAnim = 0 as i32;
        UI_ForceTorsoAnim(pi, torsoAnim);
    };
}
