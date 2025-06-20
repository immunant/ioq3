use ::libc;

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
pub use crate::keycodes_h::K_ALT;
pub use crate::keycodes_h::K_AUX1;
pub use crate::keycodes_h::K_AUX10;
pub use crate::keycodes_h::K_AUX11;
pub use crate::keycodes_h::K_AUX12;
pub use crate::keycodes_h::K_AUX13;
pub use crate::keycodes_h::K_AUX14;
pub use crate::keycodes_h::K_AUX15;
pub use crate::keycodes_h::K_AUX16;
pub use crate::keycodes_h::K_AUX2;
pub use crate::keycodes_h::K_AUX3;
pub use crate::keycodes_h::K_AUX4;
pub use crate::keycodes_h::K_AUX5;
pub use crate::keycodes_h::K_AUX6;
pub use crate::keycodes_h::K_AUX7;
pub use crate::keycodes_h::K_AUX8;
pub use crate::keycodes_h::K_AUX9;
pub use crate::keycodes_h::K_BACKSPACE;
pub use crate::keycodes_h::K_BREAK;
pub use crate::keycodes_h::K_CAPSLOCK;
pub use crate::keycodes_h::K_COMMAND;
pub use crate::keycodes_h::K_COMPOSE;
pub use crate::keycodes_h::K_CONSOLE;
pub use crate::keycodes_h::K_CTRL;
pub use crate::keycodes_h::K_DEL;
pub use crate::keycodes_h::K_DOWNARROW;
pub use crate::keycodes_h::K_END;
pub use crate::keycodes_h::K_ENTER;
pub use crate::keycodes_h::K_ESCAPE;
pub use crate::keycodes_h::K_EURO;
pub use crate::keycodes_h::K_F1;
pub use crate::keycodes_h::K_F10;
pub use crate::keycodes_h::K_F11;
pub use crate::keycodes_h::K_F12;
pub use crate::keycodes_h::K_F13;
pub use crate::keycodes_h::K_F14;
pub use crate::keycodes_h::K_F15;
pub use crate::keycodes_h::K_F2;
pub use crate::keycodes_h::K_F3;
pub use crate::keycodes_h::K_F4;
pub use crate::keycodes_h::K_F5;
pub use crate::keycodes_h::K_F6;
pub use crate::keycodes_h::K_F7;
pub use crate::keycodes_h::K_F8;
pub use crate::keycodes_h::K_F9;
pub use crate::keycodes_h::K_HELP;
pub use crate::keycodes_h::K_HOME;
pub use crate::keycodes_h::K_INS;
pub use crate::keycodes_h::K_JOY1;
pub use crate::keycodes_h::K_JOY10;
pub use crate::keycodes_h::K_JOY11;
pub use crate::keycodes_h::K_JOY12;
pub use crate::keycodes_h::K_JOY13;
pub use crate::keycodes_h::K_JOY14;
pub use crate::keycodes_h::K_JOY15;
pub use crate::keycodes_h::K_JOY16;
pub use crate::keycodes_h::K_JOY17;
pub use crate::keycodes_h::K_JOY18;
pub use crate::keycodes_h::K_JOY19;
pub use crate::keycodes_h::K_JOY2;
pub use crate::keycodes_h::K_JOY20;
pub use crate::keycodes_h::K_JOY21;
pub use crate::keycodes_h::K_JOY22;
pub use crate::keycodes_h::K_JOY23;
pub use crate::keycodes_h::K_JOY24;
pub use crate::keycodes_h::K_JOY25;
pub use crate::keycodes_h::K_JOY26;
pub use crate::keycodes_h::K_JOY27;
pub use crate::keycodes_h::K_JOY28;
pub use crate::keycodes_h::K_JOY29;
pub use crate::keycodes_h::K_JOY3;
pub use crate::keycodes_h::K_JOY30;
pub use crate::keycodes_h::K_JOY31;
pub use crate::keycodes_h::K_JOY32;
pub use crate::keycodes_h::K_JOY4;
pub use crate::keycodes_h::K_JOY5;
pub use crate::keycodes_h::K_JOY6;
pub use crate::keycodes_h::K_JOY7;
pub use crate::keycodes_h::K_JOY8;
pub use crate::keycodes_h::K_JOY9;
pub use crate::keycodes_h::K_KP_5;
pub use crate::keycodes_h::K_KP_DEL;
pub use crate::keycodes_h::K_KP_DOWNARROW;
pub use crate::keycodes_h::K_KP_END;
pub use crate::keycodes_h::K_KP_ENTER;
pub use crate::keycodes_h::K_KP_EQUALS;
pub use crate::keycodes_h::K_KP_HOME;
pub use crate::keycodes_h::K_KP_INS;
pub use crate::keycodes_h::K_KP_LEFTARROW;
pub use crate::keycodes_h::K_KP_MINUS;
pub use crate::keycodes_h::K_KP_NUMLOCK;
pub use crate::keycodes_h::K_KP_PGDN;
pub use crate::keycodes_h::K_KP_PGUP;
pub use crate::keycodes_h::K_KP_PLUS;
pub use crate::keycodes_h::K_KP_RIGHTARROW;
pub use crate::keycodes_h::K_KP_SLASH;
pub use crate::keycodes_h::K_KP_STAR;
pub use crate::keycodes_h::K_KP_UPARROW;
pub use crate::keycodes_h::K_LEFTARROW;
pub use crate::keycodes_h::K_MENU;
pub use crate::keycodes_h::K_MODE;
pub use crate::keycodes_h::K_MOUSE1;
pub use crate::keycodes_h::K_MOUSE2;
pub use crate::keycodes_h::K_MOUSE3;
pub use crate::keycodes_h::K_MOUSE4;
pub use crate::keycodes_h::K_MOUSE5;
pub use crate::keycodes_h::K_MWHEELDOWN;
pub use crate::keycodes_h::K_MWHEELUP;
pub use crate::keycodes_h::K_PAD0_A;
pub use crate::keycodes_h::K_PAD0_B;
pub use crate::keycodes_h::K_PAD0_BACK;
pub use crate::keycodes_h::K_PAD0_DPAD_DOWN;
pub use crate::keycodes_h::K_PAD0_DPAD_LEFT;
pub use crate::keycodes_h::K_PAD0_DPAD_RIGHT;
pub use crate::keycodes_h::K_PAD0_DPAD_UP;
pub use crate::keycodes_h::K_PAD0_GUIDE;
pub use crate::keycodes_h::K_PAD0_LEFTSHOULDER;
pub use crate::keycodes_h::K_PAD0_LEFTSTICK_CLICK;
pub use crate::keycodes_h::K_PAD0_LEFTSTICK_DOWN;
pub use crate::keycodes_h::K_PAD0_LEFTSTICK_LEFT;
pub use crate::keycodes_h::K_PAD0_LEFTSTICK_RIGHT;
pub use crate::keycodes_h::K_PAD0_LEFTSTICK_UP;
pub use crate::keycodes_h::K_PAD0_LEFTTRIGGER;
pub use crate::keycodes_h::K_PAD0_RIGHTSHOULDER;
pub use crate::keycodes_h::K_PAD0_RIGHTSTICK_CLICK;
pub use crate::keycodes_h::K_PAD0_RIGHTSTICK_DOWN;
pub use crate::keycodes_h::K_PAD0_RIGHTSTICK_LEFT;
pub use crate::keycodes_h::K_PAD0_RIGHTSTICK_RIGHT;
pub use crate::keycodes_h::K_PAD0_RIGHTSTICK_UP;
pub use crate::keycodes_h::K_PAD0_RIGHTTRIGGER;
pub use crate::keycodes_h::K_PAD0_START;
pub use crate::keycodes_h::K_PAD0_X;
pub use crate::keycodes_h::K_PAD0_Y;
pub use crate::keycodes_h::K_PAUSE;
pub use crate::keycodes_h::K_PGDN;
pub use crate::keycodes_h::K_PGUP;
pub use crate::keycodes_h::K_POWER;
pub use crate::keycodes_h::K_PRINT;
pub use crate::keycodes_h::K_RIGHTARROW;
pub use crate::keycodes_h::K_SCROLLOCK;
pub use crate::keycodes_h::K_SHIFT;
pub use crate::keycodes_h::K_SPACE;
pub use crate::keycodes_h::K_SUPER;
pub use crate::keycodes_h::K_SYSREQ;
pub use crate::keycodes_h::K_TAB;
pub use crate::keycodes_h::K_UNDO;
pub use crate::keycodes_h::K_UPARROW;
pub use crate::keycodes_h::K_WORLD_0;
pub use crate::keycodes_h::K_WORLD_1;
pub use crate::keycodes_h::K_WORLD_10;
pub use crate::keycodes_h::K_WORLD_11;
pub use crate::keycodes_h::K_WORLD_12;
pub use crate::keycodes_h::K_WORLD_13;
pub use crate::keycodes_h::K_WORLD_14;
pub use crate::keycodes_h::K_WORLD_15;
pub use crate::keycodes_h::K_WORLD_16;
pub use crate::keycodes_h::K_WORLD_17;
pub use crate::keycodes_h::K_WORLD_18;
pub use crate::keycodes_h::K_WORLD_19;
pub use crate::keycodes_h::K_WORLD_2;
pub use crate::keycodes_h::K_WORLD_20;
pub use crate::keycodes_h::K_WORLD_21;
pub use crate::keycodes_h::K_WORLD_22;
pub use crate::keycodes_h::K_WORLD_23;
pub use crate::keycodes_h::K_WORLD_24;
pub use crate::keycodes_h::K_WORLD_25;
pub use crate::keycodes_h::K_WORLD_26;
pub use crate::keycodes_h::K_WORLD_27;
pub use crate::keycodes_h::K_WORLD_28;
pub use crate::keycodes_h::K_WORLD_29;
pub use crate::keycodes_h::K_WORLD_3;
pub use crate::keycodes_h::K_WORLD_30;
pub use crate::keycodes_h::K_WORLD_31;
pub use crate::keycodes_h::K_WORLD_32;
pub use crate::keycodes_h::K_WORLD_33;
pub use crate::keycodes_h::K_WORLD_34;
pub use crate::keycodes_h::K_WORLD_35;
pub use crate::keycodes_h::K_WORLD_36;
pub use crate::keycodes_h::K_WORLD_37;
pub use crate::keycodes_h::K_WORLD_38;
pub use crate::keycodes_h::K_WORLD_39;
pub use crate::keycodes_h::K_WORLD_4;
pub use crate::keycodes_h::K_WORLD_40;
pub use crate::keycodes_h::K_WORLD_41;
pub use crate::keycodes_h::K_WORLD_42;
pub use crate::keycodes_h::K_WORLD_43;
pub use crate::keycodes_h::K_WORLD_44;
pub use crate::keycodes_h::K_WORLD_45;
pub use crate::keycodes_h::K_WORLD_46;
pub use crate::keycodes_h::K_WORLD_47;
pub use crate::keycodes_h::K_WORLD_48;
pub use crate::keycodes_h::K_WORLD_49;
pub use crate::keycodes_h::K_WORLD_5;
pub use crate::keycodes_h::K_WORLD_50;
pub use crate::keycodes_h::K_WORLD_51;
pub use crate::keycodes_h::K_WORLD_52;
pub use crate::keycodes_h::K_WORLD_53;
pub use crate::keycodes_h::K_WORLD_54;
pub use crate::keycodes_h::K_WORLD_55;
pub use crate::keycodes_h::K_WORLD_56;
pub use crate::keycodes_h::K_WORLD_57;
pub use crate::keycodes_h::K_WORLD_58;
pub use crate::keycodes_h::K_WORLD_59;
pub use crate::keycodes_h::K_WORLD_6;
pub use crate::keycodes_h::K_WORLD_60;
pub use crate::keycodes_h::K_WORLD_61;
pub use crate::keycodes_h::K_WORLD_62;
pub use crate::keycodes_h::K_WORLD_63;
pub use crate::keycodes_h::K_WORLD_64;
pub use crate::keycodes_h::K_WORLD_65;
pub use crate::keycodes_h::K_WORLD_66;
pub use crate::keycodes_h::K_WORLD_67;
pub use crate::keycodes_h::K_WORLD_68;
pub use crate::keycodes_h::K_WORLD_69;
pub use crate::keycodes_h::K_WORLD_7;
pub use crate::keycodes_h::K_WORLD_70;
pub use crate::keycodes_h::K_WORLD_71;
pub use crate::keycodes_h::K_WORLD_72;
pub use crate::keycodes_h::K_WORLD_73;
pub use crate::keycodes_h::K_WORLD_74;
pub use crate::keycodes_h::K_WORLD_75;
pub use crate::keycodes_h::K_WORLD_76;
pub use crate::keycodes_h::K_WORLD_77;
pub use crate::keycodes_h::K_WORLD_78;
pub use crate::keycodes_h::K_WORLD_79;
pub use crate::keycodes_h::K_WORLD_8;
pub use crate::keycodes_h::K_WORLD_80;
pub use crate::keycodes_h::K_WORLD_81;
pub use crate::keycodes_h::K_WORLD_82;
pub use crate::keycodes_h::K_WORLD_83;
pub use crate::keycodes_h::K_WORLD_84;
pub use crate::keycodes_h::K_WORLD_85;
pub use crate::keycodes_h::K_WORLD_86;
pub use crate::keycodes_h::K_WORLD_87;
pub use crate::keycodes_h::K_WORLD_88;
pub use crate::keycodes_h::K_WORLD_89;
pub use crate::keycodes_h::K_WORLD_9;
pub use crate::keycodes_h::K_WORLD_90;
pub use crate::keycodes_h::K_WORLD_91;
pub use crate::keycodes_h::K_WORLD_92;
pub use crate::keycodes_h::K_WORLD_93;
pub use crate::keycodes_h::K_WORLD_94;
pub use crate::keycodes_h::K_WORLD_95;
pub use crate::keycodes_h::MAX_KEYS;
pub use crate::src::game::bg_misc::bg_itemlist;
pub use crate::src::q3_ui::ui_atoms::uis;
pub use crate::src::q3_ui::ui_atoms::UI_ClampCvar;
pub use crate::src::q3_ui::ui_atoms::UI_Cvar_VariableString;
pub use crate::src::q3_ui::ui_atoms::UI_DrawChar;
pub use crate::src::q3_ui::ui_atoms::UI_DrawProportionalString;
pub use crate::src::q3_ui::ui_atoms::UI_DrawString;
pub use crate::src::q3_ui::ui_atoms::UI_FillRect;
pub use crate::src::q3_ui::ui_atoms::UI_PopMenu;
pub use crate::src::q3_ui::ui_atoms::UI_PushMenu;
pub use crate::src::q3_ui::ui_confirm::UI_ConfirmMenu;
pub use crate::src::q3_ui::ui_players::UI_DrawPlayer;
pub use crate::src::q3_ui::ui_players::UI_PlayerInfo_SetInfo;
pub use crate::src::q3_ui::ui_players::UI_PlayerInfo_SetModel;
pub use crate::src::q3_ui::ui_qmenu::color_red;
pub use crate::src::q3_ui::ui_qmenu::color_white;
pub use crate::src::q3_ui::ui_qmenu::color_yellow;
pub use crate::src::q3_ui::ui_qmenu::listbar_color;
pub use crate::src::q3_ui::ui_qmenu::menu_out_sound;
pub use crate::src::q3_ui::ui_qmenu::text_color_disabled;
pub use crate::src::q3_ui::ui_qmenu::text_color_highlight;
pub use crate::src::q3_ui::ui_qmenu::text_color_normal;
pub use crate::src::q3_ui::ui_qmenu::Menu_AddItem;
pub use crate::src::q3_ui::ui_qmenu::Menu_DefaultKey;
pub use crate::src::q3_ui::ui_qmenu::Menu_ItemAtCursor;
pub use crate::src::qcommon::q_math::colorWhite;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::Q_CleanStr;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strupr;
pub use crate::src::qcommon::q_shared::EXEC_APPEND;
pub use crate::src::qcommon::q_shared::EXEC_INSERT;
pub use crate::src::qcommon::q_shared::EXEC_NOW;
pub use crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText;
pub use crate::src::ui::ui_syscalls::trap_Cvar_Reset;
pub use crate::src::ui::ui_syscalls::trap_Cvar_SetValue;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableValue;
pub use crate::src::ui::ui_syscalls::trap_Key_GetBindingBuf;
pub use crate::src::ui::ui_syscalls::trap_Key_KeynumToStringBuf;
pub use crate::src::ui::ui_syscalls::trap_Key_SetBinding;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterModel;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip;

pub use crate::tr_types_h::glDriverType_t;
pub use crate::tr_types_h::glHardwareType_t;
pub use crate::tr_types_h::glconfig_t;
pub use crate::tr_types_h::textureCompression_t;
pub use crate::tr_types_h::GLDRV_ICD;
pub use crate::tr_types_h::GLDRV_STANDALONE;
pub use crate::tr_types_h::GLDRV_VOODOO;
pub use crate::tr_types_h::GLHW_3DFX_2D3D;
pub use crate::tr_types_h::GLHW_GENERIC;
pub use crate::tr_types_h::GLHW_PERMEDIA2;
pub use crate::tr_types_h::GLHW_RAGEPRO;
pub use crate::tr_types_h::GLHW_RIVA128;
pub use crate::tr_types_h::TC_NONE;
pub use crate::tr_types_h::TC_S3TC;
pub use crate::tr_types_h::TC_S3TC_ARB;
pub use crate::ui_local_h::_tag_menuframework;
pub use crate::ui_local_h::lerpFrame_t;
pub use crate::ui_local_h::menuaction_s;
pub use crate::ui_local_h::menubitmap_s;
pub use crate::ui_local_h::menucommon_s;
pub use crate::ui_local_h::menuframework_s;
pub use crate::ui_local_h::menuradiobutton_s;
pub use crate::ui_local_h::menuslider_s;
pub use crate::ui_local_h::menutext_s;
pub use crate::ui_local_h::playerInfo_t;
pub use crate::ui_local_h::uiStatic_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct controls_t {
    pub menu: menuframework_s,
    pub banner: menutext_s,
    pub framel: menubitmap_s,
    pub framer: menubitmap_s,
    pub player: menubitmap_s,
    pub movement: menutext_s,
    pub looking: menutext_s,
    pub weapons: menutext_s,
    pub misc: menutext_s,
    pub walkforward: menuaction_s,
    pub backpedal: menuaction_s,
    pub stepleft: menuaction_s,
    pub stepright: menuaction_s,
    pub moveup: menuaction_s,
    pub movedown: menuaction_s,
    pub turnleft: menuaction_s,
    pub turnright: menuaction_s,
    pub sidestep: menuaction_s,
    pub run: menuaction_s,
    pub machinegun: menuaction_s,
    pub chainsaw: menuaction_s,
    pub shotgun: menuaction_s,
    pub grenadelauncher: menuaction_s,
    pub rocketlauncher: menuaction_s,
    pub lightning: menuaction_s,
    pub railgun: menuaction_s,
    pub plasma: menuaction_s,
    pub bfg: menuaction_s,
    pub attack: menuaction_s,
    pub prevweapon: menuaction_s,
    pub nextweapon: menuaction_s,
    pub lookup: menuaction_s,
    pub lookdown: menuaction_s,
    pub mouselook: menuaction_s,
    pub freelook: menuradiobutton_s,
    pub centerview: menuaction_s,
    pub zoomview: menuaction_s,
    pub gesture: menuaction_s,
    pub invertmouse: menuradiobutton_s,
    pub sensitivity: menuslider_s,
    pub smoothmouse: menuradiobutton_s,
    pub alwaysrun: menuradiobutton_s,
    pub showscores: menuaction_s,
    pub autoswitch: menuradiobutton_s,
    pub useitem: menuaction_s,
    pub playerinfo: playerInfo_t,
    pub changesmade: qboolean,
    pub chat: menuaction_s,
    pub chat2: menuaction_s,
    pub chat3: menuaction_s,
    pub chat4: menuaction_s,
    pub togglemenu: menuaction_s,
    pub joyenable: menuradiobutton_s,
    pub joythreshold: menuslider_s,
    pub section: i32,
    pub waitingforkey: qboolean,
    pub playerModel: [libc::c_char; 64],
    pub playerViewangles: vec3_t,
    pub playerMoveangles: vec3_t,
    pub playerLegs: i32,
    pub playerTorso: i32,
    pub playerWeapon: weapon_t,
    pub playerChat: qboolean,
    pub back: menubitmap_s,
    pub name: menutext_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct configcvar_t {
    pub name: *mut libc::c_char,
    pub defaultvalue: f32,
    pub value: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bind_t {
    pub command: *mut libc::c_char,
    pub label: *mut libc::c_char,
    pub id: i32,
    pub anim: i32,
    pub defaultbind1: i32,
    pub defaultbind2: i32,
    pub bind1: i32,
    pub bind2: i32,
}

static mut s_controls: controls_t = controls_t {
    menu: menuframework_s {
        cursor: 0,
        cursor_prev: 0,
        nitems: 0,
        items: [std::ptr::null_mut(); 64],
        draw: None,
        key: None,
        wrapAround: qfalse,
        fullscreen: qfalse,
        showlogo: qfalse,
    },
    banner: menutext_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        string: std::ptr::null_mut(),
        style: 0,
        color: std::ptr::null_mut(),
    },
    framel: menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        focuspic: std::ptr::null_mut(),
        errorpic: std::ptr::null_mut(),
        shader: 0,
        focusshader: 0,
        width: 0,
        height: 0,
        focuscolor: std::ptr::null_mut(),
    },
    framer: menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        focuspic: std::ptr::null_mut(),
        errorpic: std::ptr::null_mut(),
        shader: 0,
        focusshader: 0,
        width: 0,
        height: 0,
        focuscolor: std::ptr::null_mut(),
    },
    player: menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        focuspic: std::ptr::null_mut(),
        errorpic: std::ptr::null_mut(),
        shader: 0,
        focusshader: 0,
        width: 0,
        height: 0,
        focuscolor: std::ptr::null_mut(),
    },
    movement: menutext_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        string: std::ptr::null_mut(),
        style: 0,
        color: std::ptr::null_mut(),
    },
    looking: menutext_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        string: std::ptr::null_mut(),
        style: 0,
        color: std::ptr::null_mut(),
    },
    weapons: menutext_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        string: std::ptr::null_mut(),
        style: 0,
        color: std::ptr::null_mut(),
    },
    misc: menutext_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        string: std::ptr::null_mut(),
        style: 0,
        color: std::ptr::null_mut(),
    },
    walkforward: menuaction_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
    },
    backpedal: menuaction_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
    },
    stepleft: menuaction_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
    },
    stepright: menuaction_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
    },
    moveup: menuaction_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
    },
    movedown: menuaction_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
    },
    turnleft: menuaction_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
    },
    turnright: menuaction_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
    },
    sidestep: menuaction_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
    },
    run: menuaction_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
    },
    machinegun: menuaction_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
    },
    chainsaw: menuaction_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
    },
    shotgun: menuaction_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
    },
    grenadelauncher: menuaction_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
    },
    rocketlauncher: menuaction_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
    },
    lightning: menuaction_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
    },
    railgun: menuaction_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
    },
    plasma: menuaction_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
    },
    bfg: menuaction_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
    },
    attack: menuaction_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
    },
    prevweapon: menuaction_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
    },
    nextweapon: menuaction_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
    },
    lookup: menuaction_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
    },
    lookdown: menuaction_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
    },
    mouselook: menuaction_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
    },
    freelook: menuradiobutton_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        curvalue: 0,
    },
    centerview: menuaction_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
    },
    zoomview: menuaction_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
    },
    gesture: menuaction_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
    },
    invertmouse: menuradiobutton_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        curvalue: 0,
    },
    sensitivity: menuslider_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        minvalue: 0.,
        maxvalue: 0.,
        curvalue: 0.,
        range: 0.,
    },
    smoothmouse: menuradiobutton_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        curvalue: 0,
    },
    alwaysrun: menuradiobutton_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        curvalue: 0,
    },
    showscores: menuaction_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
    },
    autoswitch: menuradiobutton_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        curvalue: 0,
    },
    useitem: menuaction_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
    },
    playerinfo: playerInfo_t {
        legsModel: 0,
        legsSkin: 0,
        legs: lerpFrame_t {
            oldFrame: 0,
            oldFrameTime: 0,
            frame: 0,
            frameTime: 0,
            backlerp: 0.,
            yawAngle: 0.,
            yawing: qfalse,
            pitchAngle: 0.,
            pitching: qfalse,
            animationNumber: 0,
            animation: std::ptr::null_mut(),
            animationTime: 0,
        },
        torsoModel: 0,
        torsoSkin: 0,
        torso: lerpFrame_t {
            oldFrame: 0,
            oldFrameTime: 0,
            frame: 0,
            frameTime: 0,
            backlerp: 0.,
            yawAngle: 0.,
            yawing: qfalse,
            pitchAngle: 0.,
            pitching: qfalse,
            animationNumber: 0,
            animation: std::ptr::null_mut(),
            animationTime: 0,
        },
        headModel: 0,
        headSkin: 0,
        animations: [animation_t {
            firstFrame: 0,
            numFrames: 0,
            loopFrames: 0,
            frameLerp: 0,
            initialLerp: 0,
            reversed: 0,
            flipflop: 0,
        }; 31],
        fixedlegs: qfalse,
        fixedtorso: qfalse,
        weaponModel: 0,
        barrelModel: 0,
        flashModel: 0,
        flashDlightColor: [0.; 3],
        muzzleFlashTime: 0,
        color1: [0.; 3],
        c1RGBA: [0; 4],
        viewAngles: [0.; 3],
        moveAngles: [0.; 3],
        currentWeapon: WP_NONE,
        legsAnim: 0,
        torsoAnim: 0,
        weapon: WP_NONE,
        lastWeapon: WP_NONE,
        pendingWeapon: WP_NONE,
        weaponTimer: 0,
        pendingLegsAnim: 0,
        torsoAnimationTimer: 0,
        pendingTorsoAnim: 0,
        legsAnimationTimer: 0,
        chat: qfalse,
        newModel: qfalse,
        barrelSpinning: qfalse,
        barrelAngle: 0.,
        barrelTime: 0,
        realWeapon: 0,
    },
    changesmade: qfalse,
    chat: menuaction_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
    },
    chat2: menuaction_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
    },
    chat3: menuaction_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
    },
    chat4: menuaction_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
    },
    togglemenu: menuaction_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
    },
    joyenable: menuradiobutton_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        curvalue: 0,
    },
    joythreshold: menuslider_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        minvalue: 0.,
        maxvalue: 0.,
        curvalue: 0.,
        range: 0.,
    },
    section: 0,
    waitingforkey: qfalse,
    playerModel: [0; 64],
    playerViewangles: [0.; 3],
    playerMoveangles: [0.; 3],
    playerLegs: 0,
    playerTorso: 0,
    playerWeapon: WP_NONE,
    playerChat: qfalse,
    back: menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        focuspic: std::ptr::null_mut(),
        errorpic: std::ptr::null_mut(),
        shader: 0,
        focusshader: 0,
        width: 0,
        height: 0,
        focuscolor: std::ptr::null_mut(),
    },
    name: menutext_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        string: std::ptr::null_mut(),
        style: 0,
        color: std::ptr::null_mut(),
    },
};

static mut controls_binding_color: vec4_t = [1.00f32, 0.43f32, 0.00f32, 1.00f32];

static mut g_bindings: [bind_t; 36] = [
    {
        let mut init = bind_t {
            command: b"+scores\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            label: b"show scores\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: 0 as i32,
            anim: 0 as i32,
            defaultbind1: K_TAB as i32,
            defaultbind2: -(1 as i32),
            bind1: -(1 as i32),
            bind2: -(1 as i32),
        };
        init
    },
    {
        let mut init = bind_t {
            command: b"+button2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            label: b"use item\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: 1 as i32,
            anim: 0 as i32,
            defaultbind1: K_ENTER as i32,
            defaultbind2: -(1 as i32),
            bind1: -(1 as i32),
            bind2: -(1 as i32),
        };
        init
    },
    {
        let mut init = bind_t {
            command: b"+speed\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            label: b"run / walk\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: 2 as i32,
            anim: 1 as i32,
            defaultbind1: K_SHIFT as i32,
            defaultbind2: -(1 as i32),
            bind1: -(1 as i32),
            bind2: -(1 as i32),
        };
        init
    },
    {
        let mut init = bind_t {
            command: b"+forward\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            label: b"walk forward\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: 3 as i32,
            anim: 2 as i32,
            defaultbind1: K_UPARROW as i32,
            defaultbind2: -(1 as i32),
            bind1: -(1 as i32),
            bind2: -(1 as i32),
        };
        init
    },
    {
        let mut init = bind_t {
            command: b"+back\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            label: b"backpedal\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: 4 as i32,
            anim: 3 as i32,
            defaultbind1: K_DOWNARROW as i32,
            defaultbind2: -(1 as i32),
            bind1: -(1 as i32),
            bind2: -(1 as i32),
        };
        init
    },
    {
        let mut init = bind_t {
            command: b"+moveleft\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            label: b"step left\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: 5 as i32,
            anim: 6 as i32,
            defaultbind1: ',' as i32,
            defaultbind2: -(1 as i32),
            bind1: -(1 as i32),
            bind2: -(1 as i32),
        };
        init
    },
    {
        let mut init = bind_t {
            command: b"+moveright\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            label: b"step right\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: 6 as i32,
            anim: 7 as i32,
            defaultbind1: '.' as i32,
            defaultbind2: -(1 as i32),
            bind1: -(1 as i32),
            bind2: -(1 as i32),
        };
        init
    },
    {
        let mut init = bind_t {
            command: b"+moveup\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            label: b"up / jump\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: 7 as i32,
            anim: 4 as i32,
            defaultbind1: K_SPACE as i32,
            defaultbind2: -(1 as i32),
            bind1: -(1 as i32),
            bind2: -(1 as i32),
        };
        init
    },
    {
        let mut init = bind_t {
            command: b"+movedown\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            label: b"down / crouch\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: 8 as i32,
            anim: 5 as i32,
            defaultbind1: 'c' as i32,
            defaultbind2: -(1 as i32),
            bind1: -(1 as i32),
            bind2: -(1 as i32),
        };
        init
    },
    {
        let mut init = bind_t {
            command: b"+left\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            label: b"turn left\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: 9 as i32,
            anim: 8 as i32,
            defaultbind1: K_LEFTARROW as i32,
            defaultbind2: -(1 as i32),
            bind1: -(1 as i32),
            bind2: -(1 as i32),
        };
        init
    },
    {
        let mut init = bind_t {
            command: b"+right\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            label: b"turn right\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: 10 as i32,
            anim: 9 as i32,
            defaultbind1: K_RIGHTARROW as i32,
            defaultbind2: -(1 as i32),
            bind1: -(1 as i32),
            bind2: -(1 as i32),
        };
        init
    },
    {
        let mut init = bind_t {
            command: b"+strafe\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            label: b"sidestep / turn\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: 11 as i32,
            anim: 0 as i32,
            defaultbind1: K_ALT as i32,
            defaultbind2: -(1 as i32),
            bind1: -(1 as i32),
            bind2: -(1 as i32),
        };
        init
    },
    {
        let mut init = bind_t {
            command: b"+lookup\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            label: b"look up\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: 12 as i32,
            anim: 10 as i32,
            defaultbind1: K_PGDN as i32,
            defaultbind2: -(1 as i32),
            bind1: -(1 as i32),
            bind2: -(1 as i32),
        };
        init
    },
    {
        let mut init = bind_t {
            command: b"+lookdown\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            label: b"look down\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: 13 as i32,
            anim: 11 as i32,
            defaultbind1: K_DEL as i32,
            defaultbind2: -(1 as i32),
            bind1: -(1 as i32),
            bind2: -(1 as i32),
        };
        init
    },
    {
        let mut init = bind_t {
            command: b"+mlook\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            label: b"mouse look\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: 14 as i32,
            anim: 0 as i32,
            defaultbind1: '/' as i32,
            defaultbind2: -(1 as i32),
            bind1: -(1 as i32),
            bind2: -(1 as i32),
        };
        init
    },
    {
        let mut init = bind_t {
            command: b"centerview\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            label: b"center view\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: 15 as i32,
            anim: 0 as i32,
            defaultbind1: K_END as i32,
            defaultbind2: -(1 as i32),
            bind1: -(1 as i32),
            bind2: -(1 as i32),
        };
        init
    },
    {
        let mut init = bind_t {
            command: b"+zoom\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            label: b"zoom view\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: 16 as i32,
            anim: 0 as i32,
            defaultbind1: -(1 as i32),
            defaultbind2: -(1 as i32),
            bind1: -(1 as i32),
            bind2: -(1 as i32),
        };
        init
    },
    {
        let mut init = bind_t {
            command: b"weapon 1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            label: b"gauntlet\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: 17 as i32,
            anim: 12 as i32,
            defaultbind1: '1' as i32,
            defaultbind2: -(1 as i32),
            bind1: -(1 as i32),
            bind2: -(1 as i32),
        };
        init
    },
    {
        let mut init = bind_t {
            command: b"weapon 2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            label: b"machinegun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: 18 as i32,
            anim: 13 as i32,
            defaultbind1: '2' as i32,
            defaultbind2: -(1 as i32),
            bind1: -(1 as i32),
            bind2: -(1 as i32),
        };
        init
    },
    {
        let mut init = bind_t {
            command: b"weapon 3\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            label: b"shotgun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: 19 as i32,
            anim: 14 as i32,
            defaultbind1: '3' as i32,
            defaultbind2: -(1 as i32),
            bind1: -(1 as i32),
            bind2: -(1 as i32),
        };
        init
    },
    {
        let mut init = bind_t {
            command: b"weapon 4\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            label: b"grenade launcher\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: 20 as i32,
            anim: 15 as i32,
            defaultbind1: '4' as i32,
            defaultbind2: -(1 as i32),
            bind1: -(1 as i32),
            bind2: -(1 as i32),
        };
        init
    },
    {
        let mut init = bind_t {
            command: b"weapon 5\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            label: b"rocket launcher\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: 21 as i32,
            anim: 16 as i32,
            defaultbind1: '5' as i32,
            defaultbind2: -(1 as i32),
            bind1: -(1 as i32),
            bind2: -(1 as i32),
        };
        init
    },
    {
        let mut init = bind_t {
            command: b"weapon 6\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            label: b"lightning\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: 22 as i32,
            anim: 17 as i32,
            defaultbind1: '6' as i32,
            defaultbind2: -(1 as i32),
            bind1: -(1 as i32),
            bind2: -(1 as i32),
        };
        init
    },
    {
        let mut init = bind_t {
            command: b"weapon 7\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            label: b"railgun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: 23 as i32,
            anim: 18 as i32,
            defaultbind1: '7' as i32,
            defaultbind2: -(1 as i32),
            bind1: -(1 as i32),
            bind2: -(1 as i32),
        };
        init
    },
    {
        let mut init = bind_t {
            command: b"weapon 8\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            label: b"plasma gun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: 24 as i32,
            anim: 19 as i32,
            defaultbind1: '8' as i32,
            defaultbind2: -(1 as i32),
            bind1: -(1 as i32),
            bind2: -(1 as i32),
        };
        init
    },
    {
        let mut init = bind_t {
            command: b"weapon 9\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            label: b"BFG\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: 25 as i32,
            anim: 20 as i32,
            defaultbind1: '9' as i32,
            defaultbind2: -(1 as i32),
            bind1: -(1 as i32),
            bind2: -(1 as i32),
        };
        init
    },
    {
        let mut init = bind_t {
            command: b"+attack\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            label: b"attack\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: 26 as i32,
            anim: 22 as i32,
            defaultbind1: K_CTRL as i32,
            defaultbind2: -(1 as i32),
            bind1: -(1 as i32),
            bind2: -(1 as i32),
        };
        init
    },
    {
        let mut init = bind_t {
            command: b"weapprev\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            label: b"prev weapon\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: 27 as i32,
            anim: 0 as i32,
            defaultbind1: '[' as i32,
            defaultbind2: -(1 as i32),
            bind1: -(1 as i32),
            bind2: -(1 as i32),
        };
        init
    },
    {
        let mut init = bind_t {
            command: b"weapnext\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            label: b"next weapon\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: 28 as i32,
            anim: 0 as i32,
            defaultbind1: ']' as i32,
            defaultbind2: -(1 as i32),
            bind1: -(1 as i32),
            bind2: -(1 as i32),
        };
        init
    },
    {
        let mut init = bind_t {
            command: b"+button3\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            label: b"gesture\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: 29 as i32,
            anim: 23 as i32,
            defaultbind1: K_MOUSE3 as i32,
            defaultbind2: -(1 as i32),
            bind1: -(1 as i32),
            bind2: -(1 as i32),
        };
        init
    },
    {
        let mut init = bind_t {
            command: b"messagemode\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            label: b"chat\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: 30 as i32,
            anim: 25 as i32,
            defaultbind1: 't' as i32,
            defaultbind2: -(1 as i32),
            bind1: -(1 as i32),
            bind2: -(1 as i32),
        };
        init
    },
    {
        let mut init = bind_t {
            command: b"messagemode2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            label: b"chat - team\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: 31 as i32,
            anim: 25 as i32,
            defaultbind1: -(1 as i32),
            defaultbind2: -(1 as i32),
            bind1: -(1 as i32),
            bind2: -(1 as i32),
        };
        init
    },
    {
        let mut init = bind_t {
            command: b"messagemode3\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            label: b"chat - target\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: 32 as i32,
            anim: 25 as i32,
            defaultbind1: -(1 as i32),
            defaultbind2: -(1 as i32),
            bind1: -(1 as i32),
            bind2: -(1 as i32),
        };
        init
    },
    {
        let mut init = bind_t {
            command: b"messagemode4\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            label: b"chat - attacker\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: 33 as i32,
            anim: 25 as i32,
            defaultbind1: -(1 as i32),
            defaultbind2: -(1 as i32),
            bind1: -(1 as i32),
            bind2: -(1 as i32),
        };
        init
    },
    {
        let mut init = bind_t {
            command: b"togglemenu\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            label: b"toggle menu\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            id: 34 as i32,
            anim: 0 as i32,
            defaultbind1: K_ESCAPE as i32,
            defaultbind2: -(1 as i32),
            bind1: -(1 as i32),
            bind2: -(1 as i32),
        };
        init
    },
    {
        let mut init = bind_t {
            command: std::ptr::null_mut() as *mut libc::c_char,
            label: std::ptr::null_mut() as *mut libc::c_char,
            id: 0 as i32,
            anim: 0 as i32,
            defaultbind1: -(1 as i32),
            defaultbind2: -(1 as i32),
            bind1: -(1 as i32),
            bind2: -(1 as i32),
        };
        init
    },
];

static mut g_configcvars: [configcvar_t; 9] = [
    {
        let mut init = configcvar_t {
            name: b"cl_run\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultvalue: 0 as i32 as f32,
            value: 0 as i32 as f32,
        };
        init
    },
    {
        let mut init = configcvar_t {
            name: b"m_pitch\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultvalue: 0 as i32 as f32,
            value: 0 as i32 as f32,
        };
        init
    },
    {
        let mut init = configcvar_t {
            name: b"cg_autoswitch\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultvalue: 0 as i32 as f32,
            value: 0 as i32 as f32,
        };
        init
    },
    {
        let mut init = configcvar_t {
            name: b"sensitivity\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultvalue: 0 as i32 as f32,
            value: 0 as i32 as f32,
        };
        init
    },
    {
        let mut init = configcvar_t {
            name: b"in_joystick\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultvalue: 0 as i32 as f32,
            value: 0 as i32 as f32,
        };
        init
    },
    {
        let mut init = configcvar_t {
            name: b"joy_threshold\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultvalue: 0 as i32 as f32,
            value: 0 as i32 as f32,
        };
        init
    },
    {
        let mut init = configcvar_t {
            name: b"m_filter\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultvalue: 0 as i32 as f32,
            value: 0 as i32 as f32,
        };
        init
    },
    {
        let mut init = configcvar_t {
            name: b"cl_freelook\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultvalue: 0 as i32 as f32,
            value: 0 as i32 as f32,
        };
        init
    },
    {
        let mut init = configcvar_t {
            name: std::ptr::null_mut(),
            defaultvalue: 0 as i32 as f32,
            value: 0 as i32 as f32,
        };
        init
    },
];
// Initialized in run_static_initializers

static mut g_movement_controls: [*mut menucommon_s; 12] = [std::ptr::null_mut(); 12];
// Initialized in run_static_initializers

static mut g_weapons_controls: [*mut menucommon_s; 14] = [std::ptr::null_mut(); 14];
// Initialized in run_static_initializers

static mut g_looking_controls: [*mut menucommon_s; 12] = [std::ptr::null_mut(); 12];
// Initialized in run_static_initializers

static mut g_misc_controls: [*mut menucommon_s; 9] = [std::ptr::null_mut(); 9];

static mut g_controls: [*mut *mut menucommon_s; 4] = unsafe {
    [
        g_movement_controls.as_ptr() as *mut _,
        g_looking_controls.as_ptr() as *mut _,
        g_weapons_controls.as_ptr() as *mut _,
        g_misc_controls.as_ptr() as *mut _,
    ]
};
/*
=================
Controls_InitCvars
=================
*/

unsafe extern "C" fn Controls_InitCvars() {
    let mut cvarptr: *mut configcvar_t = std::ptr::null_mut();
    cvarptr = g_configcvars.as_mut_ptr();
    while !(*cvarptr).name.is_null() {
        // get current value
        (*cvarptr).value = trap_Cvar_VariableValue((*cvarptr).name);
        // get default value
        trap_Cvar_Reset((*cvarptr).name);
        (*cvarptr).defaultvalue = trap_Cvar_VariableValue((*cvarptr).name);
        // restore current value
        trap_Cvar_SetValue((*cvarptr).name, (*cvarptr).value);
        cvarptr = cvarptr.offset(1)
    }
}
/*
=================
Controls_GetCvarDefault
=================
*/

unsafe extern "C" fn Controls_GetCvarDefault(mut name: *mut libc::c_char) -> f32 {
    let mut cvarptr: *mut configcvar_t = std::ptr::null_mut();
    cvarptr = g_configcvars.as_mut_ptr();
    loop {
        if (*cvarptr).name.is_null() {
            return 0 as i32 as f32;
        }
        if libc::strcmp((*cvarptr).name, name) == 0 {
            break;
        }
        cvarptr = cvarptr.offset(1)
    }
    return (*cvarptr).defaultvalue;
}
/*
=================
Controls_GetCvarValue
=================
*/

unsafe extern "C" fn Controls_GetCvarValue(mut name: *mut libc::c_char) -> f32 {
    let mut cvarptr: *mut configcvar_t = std::ptr::null_mut();
    cvarptr = g_configcvars.as_mut_ptr();
    loop {
        if (*cvarptr).name.is_null() {
            return 0 as i32 as f32;
        }
        if libc::strcmp((*cvarptr).name, name) == 0 {
            break;
        }
        cvarptr = cvarptr.offset(1)
    }
    return (*cvarptr).value;
}
/*
=================
Controls_UpdateModel
=================
*/

unsafe extern "C" fn Controls_UpdateModel(mut anim: i32) {
    s_controls.playerViewangles[2 as i32 as usize] = 0 as i32 as vec_t;
    s_controls.playerViewangles[1 as i32 as usize] = s_controls.playerViewangles[2 as i32 as usize];
    s_controls.playerViewangles[0 as i32 as usize] = s_controls.playerViewangles[1 as i32 as usize];
    s_controls.playerMoveangles[2 as i32 as usize] = 0 as i32 as vec_t;
    s_controls.playerMoveangles[1 as i32 as usize] = s_controls.playerMoveangles[2 as i32 as usize];
    s_controls.playerMoveangles[0 as i32 as usize] = s_controls.playerMoveangles[1 as i32 as usize];
    s_controls.playerViewangles[1 as i32 as usize] = (180 as i32 - 30 as i32) as vec_t;
    s_controls.playerMoveangles[1 as i32 as usize] = s_controls.playerViewangles[1 as i32 as usize];
    s_controls.playerLegs = LEGS_IDLE as i32;
    s_controls.playerTorso = TORSO_STAND as i32;
    s_controls.playerWeapon = WP_NUM_WEAPONS;
    s_controls.playerChat = qfalse;
    match anim {
        1 => s_controls.playerLegs = LEGS_RUN as i32,
        2 => s_controls.playerLegs = LEGS_WALK as i32,
        3 => s_controls.playerLegs = LEGS_BACK as i32,
        4 => s_controls.playerLegs = LEGS_JUMP as i32,
        5 => s_controls.playerLegs = LEGS_IDLECR as i32,
        8 => s_controls.playerViewangles[1 as i32 as usize] += 90 as i32 as f32,
        9 => s_controls.playerViewangles[1 as i32 as usize] -= 90 as i32 as f32,
        6 => {
            s_controls.playerLegs = LEGS_WALK as i32;
            s_controls.playerMoveangles[1 as i32 as usize] =
                s_controls.playerViewangles[1 as i32 as usize] + 90 as i32 as f32
        }
        7 => {
            s_controls.playerLegs = LEGS_WALK as i32;
            s_controls.playerMoveangles[1 as i32 as usize] =
                s_controls.playerViewangles[1 as i32 as usize] - 90 as i32 as f32
        }
        10 => s_controls.playerViewangles[0 as i32 as usize] = -(45 as i32) as vec_t,
        11 => s_controls.playerViewangles[0 as i32 as usize] = 45 as i32 as vec_t,
        12 => s_controls.playerWeapon = WP_GAUNTLET,
        13 => s_controls.playerWeapon = WP_MACHINEGUN,
        14 => s_controls.playerWeapon = WP_SHOTGUN,
        15 => s_controls.playerWeapon = WP_GRENADE_LAUNCHER,
        16 => s_controls.playerWeapon = WP_ROCKET_LAUNCHER,
        17 => s_controls.playerWeapon = WP_LIGHTNING,
        18 => s_controls.playerWeapon = WP_RAILGUN,
        19 => s_controls.playerWeapon = WP_PLASMAGUN,
        20 => s_controls.playerWeapon = WP_BFG,
        21 => s_controls.playerWeapon = WP_GRAPPLING_HOOK,
        22 => s_controls.playerTorso = TORSO_ATTACK as i32,
        23 => s_controls.playerTorso = TORSO_GESTURE as i32,
        24 => {
            s_controls.playerLegs = BOTH_DEATH1 as i32;
            s_controls.playerTorso = BOTH_DEATH1 as i32;
            s_controls.playerWeapon = WP_NONE
        }
        25 => s_controls.playerChat = qtrue,
        _ => {}
    }
    UI_PlayerInfo_SetInfo(
        &mut s_controls.playerinfo as *mut _ as *mut playerInfo_t,
        s_controls.playerLegs,
        s_controls.playerTorso,
        s_controls.playerViewangles.as_mut_ptr(),
        s_controls.playerMoveangles.as_mut_ptr(),
        s_controls.playerWeapon,
        s_controls.playerChat,
    );
}
/*
=================
Controls_Update
=================
*/

unsafe extern "C" fn Controls_Update() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut y: i32 = 0;
    let mut controls: *mut *mut menucommon_s = std::ptr::null_mut();
    let mut control: *mut menucommon_s = std::ptr::null_mut();
    // disable all controls in all groups
    i = 0 as i32;
    while i < 4 as i32 {
        controls = g_controls[i as usize];
        j = 0 as i32;
        loop {
            control = *controls.offset(j as isize);
            if control.is_null() {
                break;
            }
            (*control).flags |= 0x1000 as i32 as u32 | 0x4000 as i32 as u32;
            j += 1
        }
        i += 1
    }
    controls = g_controls[s_controls.section as usize];
    // enable controls in active group (and count number of items for vertical centering)
    j = 0 as i32;
    loop {
        control = *controls.offset(j as isize);
        if control.is_null() {
            break;
        }
        (*control).flags &= !(0x2000 as i32 as u32 | 0x1000 as i32 as u32 | 0x4000 as i32 as u32);
        j += 1
    }
    // position controls
    y = (480 as i32 - j * 16 as i32) / 2 as i32;
    j = 0 as i32;
    loop {
        control = *controls.offset(j as isize);
        if control.is_null() {
            break;
        }
        (*control).x = 320 as i32;
        (*control).y = y;
        (*control).left = 320 as i32 - 19 as i32 * 8 as i32;
        (*control).right = 320 as i32 + 21 as i32 * 8 as i32;
        (*control).top = y;
        (*control).bottom = y + 16 as i32;
        j += 1;
        y += 16 as i32
    }
    if s_controls.waitingforkey as u64 != 0 {
        // disable everybody
        i = 0 as i32;
        while i < s_controls.menu.nitems {
            (*(s_controls.menu.items[i as usize] as *mut menucommon_s)).flags |=
                0x2000 as i32 as u32;
            i += 1
        }
        // enable action item
        (*(s_controls.menu.items[s_controls.menu.cursor as usize] as *mut menucommon_s)).flags &=
            !(0x2000 as i32 as u32);
        // don't gray out player's name
        s_controls.name.generic.flags &= !(0x2000 as i32 as u32);
        return;
    }
    // enable everybody
    i = 0 as i32;
    while i < s_controls.menu.nitems {
        (*(s_controls.menu.items[i as usize] as *mut menucommon_s)).flags &=
            !(0x2000 as i32 as u32);
        i += 1
    }
    // makes sure flags are right on the group selection controls
    s_controls.looking.generic.flags &=
        !(0x2000 as i32 as u32 | 0x40 as i32 as u32 | 0x80 as i32 as u32);
    s_controls.movement.generic.flags &=
        !(0x2000 as i32 as u32 | 0x40 as i32 as u32 | 0x80 as i32 as u32);
    s_controls.weapons.generic.flags &=
        !(0x2000 as i32 as u32 | 0x40 as i32 as u32 | 0x80 as i32 as u32);
    s_controls.misc.generic.flags &=
        !(0x2000 as i32 as u32 | 0x40 as i32 as u32 | 0x80 as i32 as u32);
    s_controls.looking.generic.flags |= 0x100 as i32 as u32;
    s_controls.movement.generic.flags |= 0x100 as i32 as u32;
    s_controls.weapons.generic.flags |= 0x100 as i32 as u32;
    s_controls.misc.generic.flags |= 0x100 as i32 as u32;
    // set buttons
    match s_controls.section {
        0 => {
            s_controls.movement.generic.flags &= !(0x100 as i32 as u32);
            s_controls.movement.generic.flags |= 0x40 as i32 as u32 | 0x80 as i32 as u32
        }
        1 => {
            s_controls.looking.generic.flags &= !(0x100 as i32 as u32);
            s_controls.looking.generic.flags |= 0x40 as i32 as u32 | 0x80 as i32 as u32
        }
        2 => {
            s_controls.weapons.generic.flags &= !(0x100 as i32 as u32);
            s_controls.weapons.generic.flags |= 0x40 as i32 as u32 | 0x80 as i32 as u32
        }
        3 => {
            s_controls.misc.generic.flags &= !(0x100 as i32 as u32);
            s_controls.misc.generic.flags |= 0x40 as i32 as u32 | 0x80 as i32 as u32
        }
        _ => {}
    };
}
/*
=================
Controls_DrawKeyBinding
=================
*/

unsafe extern "C" fn Controls_DrawKeyBinding(mut self_0: *mut libc::c_void) {
    let mut a: *mut menuaction_s = std::ptr::null_mut();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut b1: i32 = 0;
    let mut b2: i32 = 0;
    let mut c: qboolean = qfalse;
    let mut name: [libc::c_char; 32] = [0; 32];
    let mut name2: [libc::c_char; 32] = [0; 32];
    a = self_0 as *mut menuaction_s;
    x = (*a).generic.x;
    y = (*a).generic.y;
    c = (Menu_ItemAtCursor((*a).generic.parent as *mut _tag_menuframework)
        == a as *mut libc::c_void) as i32 as qboolean;
    b1 = g_bindings[(*a).generic.id as usize].bind1;
    if b1 == -(1 as i32) {
        libc::strcpy(
            name.as_mut_ptr(),
            b"???\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        trap_Key_KeynumToStringBuf(b1, name.as_mut_ptr(), 32 as i32);
        Q_strupr(name.as_mut_ptr());
        b2 = g_bindings[(*a).generic.id as usize].bind2;
        if b2 != -(1 as i32) {
            trap_Key_KeynumToStringBuf(b2, name2.as_mut_ptr(), 32 as i32);
            Q_strupr(name2.as_mut_ptr());
            libc::strcat(
                name.as_mut_ptr(),
                b" or \x00" as *const u8 as *const libc::c_char,
            );
            libc::strcat(name.as_mut_ptr(), name2.as_mut_ptr());
        }
    }
    if c as u64 != 0 {
        UI_FillRect(
            (*a).generic.left as f32,
            (*a).generic.top as f32,
            ((*a).generic.right - (*a).generic.left + 1 as i32) as f32,
            ((*a).generic.bottom - (*a).generic.top + 1 as i32) as f32,
            listbar_color.as_mut_ptr(),
        );
        UI_DrawString(
            x - 8 as i32,
            y,
            g_bindings[(*a).generic.id as usize].label,
            0x2 as i32 | 0x10 as i32,
            text_color_highlight.as_mut_ptr(),
        );
        UI_DrawString(
            x + 8 as i32,
            y,
            name.as_mut_ptr(),
            0 as i32 | 0x10 as i32 | 0x4000 as i32,
            text_color_highlight.as_mut_ptr(),
        );
        if s_controls.waitingforkey as u64 != 0 {
            UI_DrawChar(
                x,
                y,
                '=' as i32,
                0x1 as i32 | 0x1000 as i32 | 0x10 as i32,
                text_color_highlight.as_mut_ptr(),
            );
            UI_DrawString(
                (640 as i32 as f64 * 0.50f64) as i32,
                (480 as i32 as f64 * 0.80f64) as i32,
                b"Waiting for new key ... ESCAPE to cancel\x00" as *const u8 as *const libc::c_char,
                0x10 as i32 | 0x1 as i32 | 0x4000 as i32,
                colorWhite.as_mut_ptr(),
            );
        } else {
            UI_DrawChar(
                x,
                y,
                13 as i32,
                0x1 as i32 | 0x1000 as i32 | 0x10 as i32,
                text_color_highlight.as_mut_ptr(),
            );
            UI_DrawString(
                (640 as i32 as f64 * 0.50f64) as i32,
                (480 as i32 as f64 * 0.78f64) as i32,
                b"Press ENTER or CLICK to change\x00" as *const u8 as *const libc::c_char,
                0x10 as i32 | 0x1 as i32,
                colorWhite.as_mut_ptr(),
            );
            UI_DrawString(
                (640 as i32 as f64 * 0.50f64) as i32,
                (480 as i32 as f64 * 0.82f64) as i32,
                b"Press BACKSPACE to clear\x00" as *const u8 as *const libc::c_char,
                0x10 as i32 | 0x1 as i32,
                colorWhite.as_mut_ptr(),
            );
        }
    } else if (*a).generic.flags & 0x2000 as i32 as u32 != 0 {
        UI_DrawString(
            x - 8 as i32,
            y,
            g_bindings[(*a).generic.id as usize].label,
            0x2 as i32 | 0x10 as i32,
            text_color_disabled.as_mut_ptr(),
        );
        UI_DrawString(
            x + 8 as i32,
            y,
            name.as_mut_ptr(),
            0 as i32 | 0x10 as i32,
            text_color_disabled.as_mut_ptr(),
        );
    } else {
        UI_DrawString(
            x - 8 as i32,
            y,
            g_bindings[(*a).generic.id as usize].label,
            0x2 as i32 | 0x10 as i32,
            controls_binding_color.as_mut_ptr(),
        );
        UI_DrawString(
            x + 8 as i32,
            y,
            name.as_mut_ptr(),
            0 as i32 | 0x10 as i32,
            controls_binding_color.as_mut_ptr(),
        );
    };
}
/*
=================
Controls_StatusBar
=================
*/

unsafe extern "C" fn Controls_StatusBar(mut _self_0: *mut libc::c_void) {
    UI_DrawString(
        (640 as i32 as f64 * 0.50f64) as i32,
        (480 as i32 as f64 * 0.80f64) as i32,
        b"Use Arrow Keys or CLICK to change\x00" as *const u8 as *const libc::c_char,
        0x10 as i32 | 0x1 as i32,
        colorWhite.as_mut_ptr(),
    );
}
/*
=================
Controls_DrawPlayer
=================
*/

unsafe extern "C" fn Controls_DrawPlayer(mut self_0: *mut libc::c_void) {
    let mut b: *mut menubitmap_s = std::ptr::null_mut();
    let mut buf: [libc::c_char; 64] = [0; 64];
    trap_Cvar_VariableStringBuffer(
        b"model\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
    );
    if libc::strcmp(buf.as_mut_ptr(), s_controls.playerModel.as_mut_ptr()) != 0 as i32 {
        UI_PlayerInfo_SetModel(
            &mut s_controls.playerinfo as *mut _ as *mut playerInfo_t,
            buf.as_mut_ptr(),
        );
        libc::strcpy(s_controls.playerModel.as_mut_ptr(), buf.as_mut_ptr());
        Controls_UpdateModel(0 as i32);
    }
    b = self_0 as *mut menubitmap_s;
    UI_DrawPlayer(
        (*b).generic.x as f32,
        (*b).generic.y as f32,
        (*b).width as f32,
        (*b).height as f32,
        &mut s_controls.playerinfo as *mut _ as *mut playerInfo_t,
        uis.realtime / 2 as i32,
    );
}
/*
=================
Controls_GetKeyAssignment
=================
*/

unsafe extern "C" fn Controls_GetKeyAssignment(
    mut command: *mut libc::c_char,
    mut twokeys: *mut i32,
) {
    let mut count: i32 = 0;
    let mut j: i32 = 0;
    let mut b: [libc::c_char; 256] = [0; 256];
    let ref mut fresh0 = *twokeys.offset(1 as i32 as isize);
    *fresh0 = -(1 as i32);
    *twokeys.offset(0 as i32 as isize) = *fresh0;
    count = 0 as i32;
    j = 0 as i32;
    while j < 256 as i32 {
        trap_Key_GetBindingBuf(j, b.as_mut_ptr(), 256 as i32);
        if !(*b.as_mut_ptr() as i32 == 0 as i32) {
            if Q_stricmp(b.as_mut_ptr(), command) == 0 {
                *twokeys.offset(count as isize) = j;
                count += 1;
                if count == 2 as i32 {
                    break;
                }
            }
        }
        j += 1
    }
}
/*
=================
Controls_GetConfig
=================
*/

unsafe extern "C" fn Controls_GetConfig() {
    let mut twokeys: [i32; 2] = [0; 2];
    let mut bindptr: *mut bind_t = std::ptr::null_mut();
    // put the bindings into a local store
    bindptr = g_bindings.as_mut_ptr();
    // iterate each command, get its numeric binding
    while !(*bindptr).label.is_null() {
        Controls_GetKeyAssignment((*bindptr).command, twokeys.as_mut_ptr());
        (*bindptr).bind1 = twokeys[0 as i32 as usize];
        (*bindptr).bind2 = twokeys[1 as i32 as usize];
        bindptr = bindptr.offset(1)
    }
    s_controls.invertmouse.curvalue = (Controls_GetCvarValue(
        b"m_pitch\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) < 0 as i32 as f32) as i32;
    s_controls.smoothmouse.curvalue = UI_ClampCvar(
        0 as i32 as f32,
        1 as i32 as f32,
        Controls_GetCvarValue(
            b"m_filter\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
    ) as i32;
    s_controls.alwaysrun.curvalue = UI_ClampCvar(
        0 as i32 as f32,
        1 as i32 as f32,
        Controls_GetCvarValue(
            b"cl_run\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
    ) as i32;
    s_controls.autoswitch.curvalue = UI_ClampCvar(
        0 as i32 as f32,
        1 as i32 as f32,
        Controls_GetCvarValue(
            b"cg_autoswitch\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
    ) as i32;
    s_controls.sensitivity.curvalue = UI_ClampCvar(
        2 as i32 as f32,
        30 as i32 as f32,
        Controls_GetCvarValue(
            b"sensitivity\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
    );
    s_controls.joyenable.curvalue = UI_ClampCvar(
        0 as i32 as f32,
        1 as i32 as f32,
        Controls_GetCvarValue(
            b"in_joystick\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
    ) as i32;
    s_controls.joythreshold.curvalue = UI_ClampCvar(
        0.05f32,
        0.75f32,
        Controls_GetCvarValue(
            b"joy_threshold\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
    );
    s_controls.freelook.curvalue = UI_ClampCvar(
        0 as i32 as f32,
        1 as i32 as f32,
        Controls_GetCvarValue(
            b"cl_freelook\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
    ) as i32;
}
/*
=================
Controls_SetConfig
=================
*/

unsafe extern "C" fn Controls_SetConfig() {
    let mut bindptr: *mut bind_t = std::ptr::null_mut();
    // set the bindings from the local store
    bindptr = g_bindings.as_mut_ptr();
    // iterate each command, get its numeric binding
    while !(*bindptr).label.is_null() {
        if (*bindptr).bind1 != -(1 as i32) {
            trap_Key_SetBinding((*bindptr).bind1, (*bindptr).command);
            if (*bindptr).bind2 != -(1 as i32) {
                trap_Key_SetBinding((*bindptr).bind2, (*bindptr).command);
            }
        }
        bindptr = bindptr.offset(1)
    }
    if s_controls.invertmouse.curvalue != 0 {
        trap_Cvar_SetValue(
            b"m_pitch\x00" as *const u8 as *const libc::c_char,
            -crate::stdlib::fabs(trap_Cvar_VariableValue(
                b"m_pitch\x00" as *const u8 as *const libc::c_char,
            ) as f64) as f32,
        );
    } else {
        trap_Cvar_SetValue(
            b"m_pitch\x00" as *const u8 as *const libc::c_char,
            crate::stdlib::fabs(trap_Cvar_VariableValue(
                b"m_pitch\x00" as *const u8 as *const libc::c_char,
            ) as f64) as f32,
        );
    }
    trap_Cvar_SetValue(
        b"m_filter\x00" as *const u8 as *const libc::c_char,
        s_controls.smoothmouse.curvalue as f32,
    );
    trap_Cvar_SetValue(
        b"cl_run\x00" as *const u8 as *const libc::c_char,
        s_controls.alwaysrun.curvalue as f32,
    );
    trap_Cvar_SetValue(
        b"cg_autoswitch\x00" as *const u8 as *const libc::c_char,
        s_controls.autoswitch.curvalue as f32,
    );
    trap_Cvar_SetValue(
        b"sensitivity\x00" as *const u8 as *const libc::c_char,
        s_controls.sensitivity.curvalue,
    );
    trap_Cvar_SetValue(
        b"in_joystick\x00" as *const u8 as *const libc::c_char,
        s_controls.joyenable.curvalue as f32,
    );
    trap_Cvar_SetValue(
        b"joy_threshold\x00" as *const u8 as *const libc::c_char,
        s_controls.joythreshold.curvalue,
    );
    trap_Cvar_SetValue(
        b"cl_freelook\x00" as *const u8 as *const libc::c_char,
        s_controls.freelook.curvalue as f32,
    );
    trap_Cmd_ExecuteText(
        EXEC_APPEND as i32,
        b"in_restart\n\x00" as *const u8 as *const libc::c_char,
    );
}
/*
=================
Controls_SetDefaults
=================
*/

unsafe extern "C" fn Controls_SetDefaults() {
    let mut bindptr: *mut bind_t = std::ptr::null_mut();
    // set the bindings from the local store
    bindptr = g_bindings.as_mut_ptr();
    // iterate each command, set its default binding
    while !(*bindptr).label.is_null() {
        (*bindptr).bind1 = (*bindptr).defaultbind1;
        (*bindptr).bind2 = (*bindptr).defaultbind2;
        bindptr = bindptr.offset(1)
    }
    s_controls.invertmouse.curvalue = (Controls_GetCvarDefault(
        b"m_pitch\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) < 0 as i32 as f32) as i32;
    s_controls.smoothmouse.curvalue = Controls_GetCvarDefault(
        b"m_filter\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as i32;
    s_controls.alwaysrun.curvalue = Controls_GetCvarDefault(
        b"cl_run\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as i32;
    s_controls.autoswitch.curvalue = Controls_GetCvarDefault(
        b"cg_autoswitch\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as i32;
    s_controls.sensitivity.curvalue = Controls_GetCvarDefault(
        b"sensitivity\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    s_controls.joyenable.curvalue = Controls_GetCvarDefault(
        b"in_joystick\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as i32;
    s_controls.joythreshold.curvalue = Controls_GetCvarDefault(
        b"joy_threshold\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    s_controls.freelook.curvalue = Controls_GetCvarDefault(
        b"cl_freelook\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as i32;
}
/*
=================
Controls_MenuKey
=================
*/

unsafe extern "C" fn Controls_MenuKey(mut key: i32) -> sfxHandle_t {
    let mut current_block: u64;
    let mut id: i32 = 0;
    let mut _i: i32 = 0;
    let mut found: qboolean = qfalse;
    let mut bindptr: *mut bind_t = std::ptr::null_mut();
    found = qfalse;
    if s_controls.waitingforkey as u64 == 0 {
        match key {
            127 | 140 | 171 => {
                current_block = 15828856531406745863;
                match current_block {
                    11504342534947256683 => {
                        if s_controls.changesmade as u64 != 0 {
                            Controls_SetConfig();
                        }
                        current_block = 14059375994042912985;
                    }
                    _ => {
                        key = -(1 as i32);
                        current_block = 15904375183555213903;
                    }
                }
            }
            179 | 27 => {
                current_block = 11504342534947256683;
                match current_block {
                    11504342534947256683 => {
                        if s_controls.changesmade as u64 != 0 {
                            Controls_SetConfig();
                        }
                        current_block = 14059375994042912985;
                    }
                    _ => {
                        key = -(1 as i32);
                        current_block = 15904375183555213903;
                    }
                }
            }
            _ => {
                current_block = 14059375994042912985;
            }
        }
    } else if key & 1024 as i32 != 0 {
        current_block = 14059375994042912985;
    } else {
        match key {
            27 => {
                s_controls.waitingforkey = qfalse;
                Controls_Update();
                return menu_out_sound;
            }
            96 => {
                current_block = 14059375994042912985;
            }
            _ => {
                current_block = 15904375183555213903;
            }
        }
    }
    match current_block {
        15904375183555213903 => {
            s_controls.changesmade = qtrue;
            if key != -(1 as i32) {
                // remove from any other bind
                bindptr = g_bindings.as_mut_ptr();
                _i = 0 as i32;
                while !(*bindptr).label.is_null() {
                    if (*bindptr).bind2 == key {
                        (*bindptr).bind2 = -(1 as i32)
                    }
                    if (*bindptr).bind1 == key {
                        (*bindptr).bind1 = (*bindptr).bind2;
                        (*bindptr).bind2 = -(1 as i32)
                    }
                    _i += 1;
                    bindptr = bindptr.offset(1)
                }
            }
            // assign key to local store
            id =
                (*(s_controls.menu.items[s_controls.menu.cursor as usize] as *mut menucommon_s)).id;
            bindptr = g_bindings.as_mut_ptr();
            _i = 0 as i32;
            while !(*bindptr).label.is_null() {
                if (*bindptr).id == id {
                    found = qtrue;
                    if key == -(1 as i32) {
                        if (*bindptr).bind1 != -(1 as i32) {
                            trap_Key_SetBinding(
                                (*bindptr).bind1,
                                b"\x00" as *const u8 as *const libc::c_char,
                            );
                            (*bindptr).bind1 = -(1 as i32)
                        }
                        if (*bindptr).bind2 != -(1 as i32) {
                            trap_Key_SetBinding(
                                (*bindptr).bind2,
                                b"\x00" as *const u8 as *const libc::c_char,
                            );
                            (*bindptr).bind2 = -(1 as i32)
                        }
                    } else if (*bindptr).bind1 == -(1 as i32) {
                        (*bindptr).bind1 = key
                    } else if (*bindptr).bind1 != key && (*bindptr).bind2 == -(1 as i32) {
                        (*bindptr).bind2 = key
                    } else {
                        trap_Key_SetBinding(
                            (*bindptr).bind1,
                            b"\x00" as *const u8 as *const libc::c_char,
                        );
                        trap_Key_SetBinding(
                            (*bindptr).bind2,
                            b"\x00" as *const u8 as *const libc::c_char,
                        );
                        (*bindptr).bind1 = key;
                        (*bindptr).bind2 = -(1 as i32)
                    }
                    break;
                } else {
                    _i += 1;
                    bindptr = bindptr.offset(1)
                }
            }
            s_controls.waitingforkey = qfalse;
            if found as u64 != 0 {
                Controls_Update();
                return menu_out_sound;
            }
        }
        _ => {}
    }
    return Menu_DefaultKey(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        key,
    );
}
/*
=================
Controls_ResetDefaults_Action
=================
*/

unsafe extern "C" fn Controls_ResetDefaults_Action(mut result: qboolean) {
    if result as u64 == 0 {
        return;
    }
    s_controls.changesmade = qtrue;
    Controls_SetDefaults();
    Controls_Update();
}
/*
=================
Controls_ResetDefaults_Draw
=================
*/

unsafe extern "C" fn Controls_ResetDefaults_Draw() {
    UI_DrawProportionalString(
        640 as i32 / 2 as i32,
        356 as i32 + 27 as i32 * 0 as i32,
        b"WARNING: This will reset all\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x10 as i32,
        color_yellow.as_mut_ptr(),
    );
    UI_DrawProportionalString(
        640 as i32 / 2 as i32,
        356 as i32 + 27 as i32 * 1 as i32,
        b"controls to their default values.\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x10 as i32,
        color_yellow.as_mut_ptr(),
    );
}
/*
=================
Controls_MenuEvent
=================
*/

unsafe extern "C" fn Controls_MenuEvent(mut ptr: *mut libc::c_void, mut event: i32) {
    match (*(ptr as *mut menucommon_s)).id {
        100 => {
            if event == 3 as i32 {
                s_controls.section = 0 as i32;
                Controls_Update();
            }
        }
        101 => {
            if event == 3 as i32 {
                s_controls.section = 1 as i32;
                Controls_Update();
            }
        }
        102 => {
            if event == 3 as i32 {
                s_controls.section = 2 as i32;
                Controls_Update();
            }
        }
        103 => {
            if event == 3 as i32 {
                s_controls.section = 3 as i32;
                Controls_Update();
            }
        }
        104 => {
            if event == 3 as i32 {
                UI_ConfirmMenu(
                    b"SET TO DEFAULTS?\x00" as *const u8 as *const libc::c_char,
                    Some(Controls_ResetDefaults_Draw as unsafe extern "C" fn() -> ()),
                    Some(Controls_ResetDefaults_Action as unsafe extern "C" fn(_: qboolean) -> ()),
                );
            }
        }
        105 => {
            if event == 3 as i32 {
                if s_controls.changesmade as u64 != 0 {
                    Controls_SetConfig();
                }
                UI_PopMenu();
            }
        }
        106 => {
            if event == 3 as i32 {
                Controls_SetConfig();
                UI_PopMenu();
            }
        }
        107 => {
            if event == 3 as i32 {
                UI_PopMenu();
            }
        }
        35 | 39 | 36 | 42 | 37 | 38 | 40 | 41 => {
            if event == 3 as i32 {
                s_controls.changesmade = qtrue
            }
        }
        _ => {}
    };
}
/*
=================
Controls_ActionEvent
=================
*/

unsafe extern "C" fn Controls_ActionEvent(mut ptr: *mut libc::c_void, mut event: i32) {
    if event == 2 as i32 {
        Controls_UpdateModel(0 as i32);
    } else if event == 1 as i32 {
        Controls_UpdateModel(g_bindings[(*(ptr as *mut menucommon_s)).id as usize].anim);
    } else if event == 3 as i32 && s_controls.waitingforkey as u64 == 0 {
        s_controls.waitingforkey = qtrue;
        Controls_Update();
    };
}
/*
=================
Controls_InitModel
=================
*/

unsafe extern "C" fn Controls_InitModel() {
    crate::stdlib::memset(
        &mut s_controls.playerinfo as *mut playerInfo_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<playerInfo_t>() as usize,
    );
    UI_PlayerInfo_SetModel(
        &mut s_controls.playerinfo as *mut _ as *mut playerInfo_t,
        UI_Cvar_VariableString(b"model\x00" as *const u8 as *const libc::c_char),
    );
    Controls_UpdateModel(0 as i32);
}
/*
=================
Controls_InitWeapons
=================
*/

unsafe extern "C" fn Controls_InitWeapons() {
    let mut item: *mut gitem_t = std::ptr::null_mut();
    item = bg_itemlist.as_mut_ptr().offset(1 as i32 as isize);
    while !(*item).classname.is_null() {
        if !((*item).giType as u32 != IT_WEAPON as i32 as u32) {
            trap_R_RegisterModel((*item).world_model[0 as i32 as usize]);
        }
        item = item.offset(1)
    }
}
/*
=================
Controls_MenuInit
=================
*/

unsafe extern "C" fn Controls_MenuInit() {
    static mut playername: [libc::c_char; 32] = [0; 32];
    // zero set all our globals
    crate::stdlib::memset(
        &mut s_controls as *mut controls_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<controls_t>() as usize,
    );
    Controls_Cache();
    s_controls.menu.key = Some(Controls_MenuKey as unsafe extern "C" fn(_: i32) -> sfxHandle_t);
    s_controls.menu.wrapAround = qtrue;
    s_controls.menu.fullscreen = qtrue;
    s_controls.banner.generic.type_0 = 10 as i32;
    s_controls.banner.generic.flags = 0x8 as i32 as u32;
    s_controls.banner.generic.x = 320 as i32;
    s_controls.banner.generic.y = 16 as i32;
    s_controls.banner.string =
        b"CONTROLS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_controls.banner.color = color_white.as_mut_ptr();
    s_controls.banner.style = 0x1 as i32;
    s_controls.framel.generic.type_0 = 6 as i32;
    s_controls.framel.generic.name = b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char;
    s_controls.framel.generic.flags = 0x4 as i32 as u32 | 0x4000 as i32 as u32;
    s_controls.framel.generic.x = 0 as i32;
    s_controls.framel.generic.y = 78 as i32;
    s_controls.framel.width = 256 as i32;
    s_controls.framel.height = 329 as i32;
    s_controls.framer.generic.type_0 = 6 as i32;
    s_controls.framer.generic.name = b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char;
    s_controls.framer.generic.flags = 0x4 as i32 as u32 | 0x4000 as i32 as u32;
    s_controls.framer.generic.x = 376 as i32;
    s_controls.framer.generic.y = 76 as i32;
    s_controls.framer.width = 256 as i32;
    s_controls.framer.height = 334 as i32;
    s_controls.looking.generic.type_0 = 9 as i32;
    s_controls.looking.generic.flags = 0x10 as i32 as u32 | 0x100 as i32 as u32;
    s_controls.looking.generic.id = 101 as i32;
    s_controls.looking.generic.callback =
        Some(Controls_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.looking.generic.x = 152 as i32;
    s_controls.looking.generic.y = 240 as i32 - 2 as i32 * 27 as i32;
    s_controls.looking.string =
        b"LOOK\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_controls.looking.style = 0x2 as i32;
    s_controls.looking.color = color_red.as_mut_ptr();
    s_controls.movement.generic.type_0 = 9 as i32;
    s_controls.movement.generic.flags = 0x10 as i32 as u32 | 0x100 as i32 as u32;
    s_controls.movement.generic.id = 100 as i32;
    s_controls.movement.generic.callback =
        Some(Controls_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.movement.generic.x = 152 as i32;
    s_controls.movement.generic.y = 240 as i32 - 27 as i32;
    s_controls.movement.string =
        b"MOVE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_controls.movement.style = 0x2 as i32;
    s_controls.movement.color = color_red.as_mut_ptr();
    s_controls.weapons.generic.type_0 = 9 as i32;
    s_controls.weapons.generic.flags = 0x10 as i32 as u32 | 0x100 as i32 as u32;
    s_controls.weapons.generic.id = 102 as i32;
    s_controls.weapons.generic.callback =
        Some(Controls_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.weapons.generic.x = 152 as i32;
    s_controls.weapons.generic.y = 240 as i32;
    s_controls.weapons.string =
        b"SHOOT\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_controls.weapons.style = 0x2 as i32;
    s_controls.weapons.color = color_red.as_mut_ptr();
    s_controls.misc.generic.type_0 = 9 as i32;
    s_controls.misc.generic.flags = 0x10 as i32 as u32 | 0x100 as i32 as u32;
    s_controls.misc.generic.id = 103 as i32;
    s_controls.misc.generic.callback =
        Some(Controls_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.misc.generic.x = 152 as i32;
    s_controls.misc.generic.y = 240 as i32 + 27 as i32;
    s_controls.misc.string = b"MISC\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_controls.misc.style = 0x2 as i32;
    s_controls.misc.color = color_red.as_mut_ptr();
    s_controls.back.generic.type_0 = 6 as i32;
    s_controls.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    s_controls.back.generic.flags = 0x4 as i32 as u32 | 0x100 as i32 as u32;
    s_controls.back.generic.x = 0 as i32;
    s_controls.back.generic.y = 480 as i32 - 64 as i32;
    s_controls.back.generic.id = 105 as i32;
    s_controls.back.generic.callback =
        Some(Controls_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.back.width = 128 as i32;
    s_controls.back.height = 64 as i32;
    s_controls.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_controls.player.generic.type_0 = 6 as i32;
    s_controls.player.generic.flags = 0x4000 as i32 as u32;
    s_controls.player.generic.ownerdraw =
        Some(Controls_DrawPlayer as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.player.generic.x = 400 as i32;
    s_controls.player.generic.y = -(40 as i32);
    s_controls.player.width = 32 as i32 * 10 as i32;
    s_controls.player.height = 56 as i32 * 10 as i32;
    s_controls.walkforward.generic.type_0 = 2 as i32;
    s_controls.walkforward.generic.flags =
        0x4 as i32 as u32 | 0x100 as i32 as u32 | 0x2000 as i32 as u32 | 0x1000 as i32 as u32;
    s_controls.walkforward.generic.callback =
        Some(Controls_ActionEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.walkforward.generic.ownerdraw =
        Some(Controls_DrawKeyBinding as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.walkforward.generic.id = 3 as i32;
    s_controls.backpedal.generic.type_0 = 2 as i32;
    s_controls.backpedal.generic.flags =
        0x4 as i32 as u32 | 0x100 as i32 as u32 | 0x2000 as i32 as u32 | 0x1000 as i32 as u32;
    s_controls.backpedal.generic.callback =
        Some(Controls_ActionEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.backpedal.generic.ownerdraw =
        Some(Controls_DrawKeyBinding as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.backpedal.generic.id = 4 as i32;
    s_controls.stepleft.generic.type_0 = 2 as i32;
    s_controls.stepleft.generic.flags =
        0x4 as i32 as u32 | 0x100 as i32 as u32 | 0x2000 as i32 as u32 | 0x1000 as i32 as u32;
    s_controls.stepleft.generic.callback =
        Some(Controls_ActionEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.stepleft.generic.ownerdraw =
        Some(Controls_DrawKeyBinding as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.stepleft.generic.id = 5 as i32;
    s_controls.stepright.generic.type_0 = 2 as i32;
    s_controls.stepright.generic.flags =
        0x4 as i32 as u32 | 0x100 as i32 as u32 | 0x2000 as i32 as u32 | 0x1000 as i32 as u32;
    s_controls.stepright.generic.callback =
        Some(Controls_ActionEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.stepright.generic.ownerdraw =
        Some(Controls_DrawKeyBinding as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.stepright.generic.id = 6 as i32;
    s_controls.moveup.generic.type_0 = 2 as i32;
    s_controls.moveup.generic.flags =
        0x4 as i32 as u32 | 0x100 as i32 as u32 | 0x2000 as i32 as u32 | 0x1000 as i32 as u32;
    s_controls.moveup.generic.callback =
        Some(Controls_ActionEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.moveup.generic.ownerdraw =
        Some(Controls_DrawKeyBinding as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.moveup.generic.id = 7 as i32;
    s_controls.movedown.generic.type_0 = 2 as i32;
    s_controls.movedown.generic.flags =
        0x4 as i32 as u32 | 0x100 as i32 as u32 | 0x2000 as i32 as u32 | 0x1000 as i32 as u32;
    s_controls.movedown.generic.callback =
        Some(Controls_ActionEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.movedown.generic.ownerdraw =
        Some(Controls_DrawKeyBinding as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.movedown.generic.id = 8 as i32;
    s_controls.turnleft.generic.type_0 = 2 as i32;
    s_controls.turnleft.generic.flags =
        0x4 as i32 as u32 | 0x100 as i32 as u32 | 0x2000 as i32 as u32 | 0x1000 as i32 as u32;
    s_controls.turnleft.generic.callback =
        Some(Controls_ActionEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.turnleft.generic.ownerdraw =
        Some(Controls_DrawKeyBinding as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.turnleft.generic.id = 9 as i32;
    s_controls.turnright.generic.type_0 = 2 as i32;
    s_controls.turnright.generic.flags =
        0x4 as i32 as u32 | 0x100 as i32 as u32 | 0x2000 as i32 as u32 | 0x1000 as i32 as u32;
    s_controls.turnright.generic.callback =
        Some(Controls_ActionEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.turnright.generic.ownerdraw =
        Some(Controls_DrawKeyBinding as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.turnright.generic.id = 10 as i32;
    s_controls.sidestep.generic.type_0 = 2 as i32;
    s_controls.sidestep.generic.flags =
        0x4 as i32 as u32 | 0x100 as i32 as u32 | 0x2000 as i32 as u32 | 0x1000 as i32 as u32;
    s_controls.sidestep.generic.callback =
        Some(Controls_ActionEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.sidestep.generic.ownerdraw =
        Some(Controls_DrawKeyBinding as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.sidestep.generic.id = 11 as i32;
    s_controls.run.generic.type_0 = 2 as i32;
    s_controls.run.generic.flags =
        0x4 as i32 as u32 | 0x100 as i32 as u32 | 0x2000 as i32 as u32 | 0x1000 as i32 as u32;
    s_controls.run.generic.callback =
        Some(Controls_ActionEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.run.generic.ownerdraw =
        Some(Controls_DrawKeyBinding as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.run.generic.id = 2 as i32;
    s_controls.chainsaw.generic.type_0 = 2 as i32;
    s_controls.chainsaw.generic.flags =
        0x4 as i32 as u32 | 0x100 as i32 as u32 | 0x2000 as i32 as u32 | 0x1000 as i32 as u32;
    s_controls.chainsaw.generic.callback =
        Some(Controls_ActionEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.chainsaw.generic.ownerdraw =
        Some(Controls_DrawKeyBinding as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.chainsaw.generic.id = 17 as i32;
    s_controls.machinegun.generic.type_0 = 2 as i32;
    s_controls.machinegun.generic.flags =
        0x4 as i32 as u32 | 0x100 as i32 as u32 | 0x2000 as i32 as u32 | 0x1000 as i32 as u32;
    s_controls.machinegun.generic.callback =
        Some(Controls_ActionEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.machinegun.generic.ownerdraw =
        Some(Controls_DrawKeyBinding as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.machinegun.generic.id = 18 as i32;
    s_controls.shotgun.generic.type_0 = 2 as i32;
    s_controls.shotgun.generic.flags =
        0x4 as i32 as u32 | 0x100 as i32 as u32 | 0x2000 as i32 as u32 | 0x1000 as i32 as u32;
    s_controls.shotgun.generic.callback =
        Some(Controls_ActionEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.shotgun.generic.ownerdraw =
        Some(Controls_DrawKeyBinding as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.shotgun.generic.id = 19 as i32;
    s_controls.grenadelauncher.generic.type_0 = 2 as i32;
    s_controls.grenadelauncher.generic.flags =
        0x4 as i32 as u32 | 0x100 as i32 as u32 | 0x2000 as i32 as u32 | 0x1000 as i32 as u32;
    s_controls.grenadelauncher.generic.callback =
        Some(Controls_ActionEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.grenadelauncher.generic.ownerdraw =
        Some(Controls_DrawKeyBinding as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.grenadelauncher.generic.id = 20 as i32;
    s_controls.rocketlauncher.generic.type_0 = 2 as i32;
    s_controls.rocketlauncher.generic.flags =
        0x4 as i32 as u32 | 0x100 as i32 as u32 | 0x2000 as i32 as u32 | 0x1000 as i32 as u32;
    s_controls.rocketlauncher.generic.callback =
        Some(Controls_ActionEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.rocketlauncher.generic.ownerdraw =
        Some(Controls_DrawKeyBinding as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.rocketlauncher.generic.id = 21 as i32;
    s_controls.lightning.generic.type_0 = 2 as i32;
    s_controls.lightning.generic.flags =
        0x4 as i32 as u32 | 0x100 as i32 as u32 | 0x2000 as i32 as u32 | 0x1000 as i32 as u32;
    s_controls.lightning.generic.callback =
        Some(Controls_ActionEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.lightning.generic.ownerdraw =
        Some(Controls_DrawKeyBinding as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.lightning.generic.id = 22 as i32;
    s_controls.railgun.generic.type_0 = 2 as i32;
    s_controls.railgun.generic.flags =
        0x4 as i32 as u32 | 0x100 as i32 as u32 | 0x2000 as i32 as u32 | 0x1000 as i32 as u32;
    s_controls.railgun.generic.callback =
        Some(Controls_ActionEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.railgun.generic.ownerdraw =
        Some(Controls_DrawKeyBinding as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.railgun.generic.id = 23 as i32;
    s_controls.plasma.generic.type_0 = 2 as i32;
    s_controls.plasma.generic.flags =
        0x4 as i32 as u32 | 0x100 as i32 as u32 | 0x2000 as i32 as u32 | 0x1000 as i32 as u32;
    s_controls.plasma.generic.callback =
        Some(Controls_ActionEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.plasma.generic.ownerdraw =
        Some(Controls_DrawKeyBinding as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.plasma.generic.id = 24 as i32;
    s_controls.bfg.generic.type_0 = 2 as i32;
    s_controls.bfg.generic.flags =
        0x4 as i32 as u32 | 0x100 as i32 as u32 | 0x2000 as i32 as u32 | 0x1000 as i32 as u32;
    s_controls.bfg.generic.callback =
        Some(Controls_ActionEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.bfg.generic.ownerdraw =
        Some(Controls_DrawKeyBinding as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.bfg.generic.id = 25 as i32;
    s_controls.attack.generic.type_0 = 2 as i32;
    s_controls.attack.generic.flags =
        0x4 as i32 as u32 | 0x100 as i32 as u32 | 0x2000 as i32 as u32 | 0x1000 as i32 as u32;
    s_controls.attack.generic.callback =
        Some(Controls_ActionEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.attack.generic.ownerdraw =
        Some(Controls_DrawKeyBinding as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.attack.generic.id = 26 as i32;
    s_controls.prevweapon.generic.type_0 = 2 as i32;
    s_controls.prevweapon.generic.flags =
        0x4 as i32 as u32 | 0x100 as i32 as u32 | 0x2000 as i32 as u32 | 0x1000 as i32 as u32;
    s_controls.prevweapon.generic.callback =
        Some(Controls_ActionEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.prevweapon.generic.ownerdraw =
        Some(Controls_DrawKeyBinding as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.prevweapon.generic.id = 27 as i32;
    s_controls.nextweapon.generic.type_0 = 2 as i32;
    s_controls.nextweapon.generic.flags =
        0x4 as i32 as u32 | 0x100 as i32 as u32 | 0x2000 as i32 as u32 | 0x1000 as i32 as u32;
    s_controls.nextweapon.generic.callback =
        Some(Controls_ActionEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.nextweapon.generic.ownerdraw =
        Some(Controls_DrawKeyBinding as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.nextweapon.generic.id = 28 as i32;
    s_controls.lookup.generic.type_0 = 2 as i32;
    s_controls.lookup.generic.flags =
        0x4 as i32 as u32 | 0x100 as i32 as u32 | 0x2000 as i32 as u32 | 0x1000 as i32 as u32;
    s_controls.lookup.generic.callback =
        Some(Controls_ActionEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.lookup.generic.ownerdraw =
        Some(Controls_DrawKeyBinding as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.lookup.generic.id = 12 as i32;
    s_controls.lookdown.generic.type_0 = 2 as i32;
    s_controls.lookdown.generic.flags =
        0x4 as i32 as u32 | 0x100 as i32 as u32 | 0x2000 as i32 as u32 | 0x1000 as i32 as u32;
    s_controls.lookdown.generic.callback =
        Some(Controls_ActionEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.lookdown.generic.ownerdraw =
        Some(Controls_DrawKeyBinding as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.lookdown.generic.id = 13 as i32;
    s_controls.mouselook.generic.type_0 = 2 as i32;
    s_controls.mouselook.generic.flags =
        0x4 as i32 as u32 | 0x80 as i32 as u32 | 0x2000 as i32 as u32 | 0x1000 as i32 as u32;
    s_controls.mouselook.generic.callback =
        Some(Controls_ActionEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.mouselook.generic.ownerdraw =
        Some(Controls_DrawKeyBinding as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.mouselook.generic.id = 14 as i32;
    s_controls.freelook.generic.type_0 = 5 as i32;
    s_controls.freelook.generic.flags = 0x2 as i32 as u32;
    s_controls.freelook.generic.x = 640 as i32 / 2 as i32;
    s_controls.freelook.generic.name = b"free look\x00" as *const u8 as *const libc::c_char;
    s_controls.freelook.generic.id = 35 as i32;
    s_controls.freelook.generic.callback =
        Some(Controls_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.freelook.generic.statusbar =
        Some(Controls_StatusBar as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.centerview.generic.type_0 = 2 as i32;
    s_controls.centerview.generic.flags =
        0x4 as i32 as u32 | 0x100 as i32 as u32 | 0x2000 as i32 as u32 | 0x1000 as i32 as u32;
    s_controls.centerview.generic.callback =
        Some(Controls_ActionEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.centerview.generic.ownerdraw =
        Some(Controls_DrawKeyBinding as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.centerview.generic.id = 15 as i32;
    s_controls.zoomview.generic.type_0 = 2 as i32;
    s_controls.zoomview.generic.flags =
        0x4 as i32 as u32 | 0x100 as i32 as u32 | 0x2000 as i32 as u32 | 0x1000 as i32 as u32;
    s_controls.zoomview.generic.callback =
        Some(Controls_ActionEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.zoomview.generic.ownerdraw =
        Some(Controls_DrawKeyBinding as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.zoomview.generic.id = 16 as i32;
    s_controls.useitem.generic.type_0 = 2 as i32;
    s_controls.useitem.generic.flags =
        0x4 as i32 as u32 | 0x100 as i32 as u32 | 0x2000 as i32 as u32 | 0x1000 as i32 as u32;
    s_controls.useitem.generic.callback =
        Some(Controls_ActionEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.useitem.generic.ownerdraw =
        Some(Controls_DrawKeyBinding as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.useitem.generic.id = 1 as i32;
    s_controls.showscores.generic.type_0 = 2 as i32;
    s_controls.showscores.generic.flags =
        0x4 as i32 as u32 | 0x100 as i32 as u32 | 0x2000 as i32 as u32 | 0x1000 as i32 as u32;
    s_controls.showscores.generic.callback =
        Some(Controls_ActionEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.showscores.generic.ownerdraw =
        Some(Controls_DrawKeyBinding as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.showscores.generic.id = 0 as i32;
    s_controls.invertmouse.generic.type_0 = 5 as i32;
    s_controls.invertmouse.generic.flags = 0x2 as i32 as u32;
    s_controls.invertmouse.generic.x = 640 as i32 / 2 as i32;
    s_controls.invertmouse.generic.name = b"invert mouse\x00" as *const u8 as *const libc::c_char;
    s_controls.invertmouse.generic.id = 36 as i32;
    s_controls.invertmouse.generic.callback =
        Some(Controls_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.invertmouse.generic.statusbar =
        Some(Controls_StatusBar as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.smoothmouse.generic.type_0 = 5 as i32;
    s_controls.smoothmouse.generic.flags = 0x2 as i32 as u32;
    s_controls.smoothmouse.generic.x = 640 as i32 / 2 as i32;
    s_controls.smoothmouse.generic.name = b"smooth mouse\x00" as *const u8 as *const libc::c_char;
    s_controls.smoothmouse.generic.id = 42 as i32;
    s_controls.smoothmouse.generic.callback =
        Some(Controls_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.smoothmouse.generic.statusbar =
        Some(Controls_StatusBar as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.alwaysrun.generic.type_0 = 5 as i32;
    s_controls.alwaysrun.generic.flags = 0x2 as i32 as u32;
    s_controls.alwaysrun.generic.x = 640 as i32 / 2 as i32;
    s_controls.alwaysrun.generic.name = b"always run\x00" as *const u8 as *const libc::c_char;
    s_controls.alwaysrun.generic.id = 37 as i32;
    s_controls.alwaysrun.generic.callback =
        Some(Controls_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.alwaysrun.generic.statusbar =
        Some(Controls_StatusBar as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.autoswitch.generic.type_0 = 5 as i32;
    s_controls.autoswitch.generic.flags = 0x2 as i32 as u32;
    s_controls.autoswitch.generic.x = 640 as i32 / 2 as i32;
    s_controls.autoswitch.generic.name =
        b"autoswitch weapons\x00" as *const u8 as *const libc::c_char;
    s_controls.autoswitch.generic.id = 38 as i32;
    s_controls.autoswitch.generic.callback =
        Some(Controls_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.autoswitch.generic.statusbar =
        Some(Controls_StatusBar as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.sensitivity.generic.type_0 = 1 as i32;
    s_controls.sensitivity.generic.x = 640 as i32 / 2 as i32;
    s_controls.sensitivity.generic.flags = 0x2 as i32 as u32;
    s_controls.sensitivity.generic.name = b"mouse speed\x00" as *const u8 as *const libc::c_char;
    s_controls.sensitivity.generic.id = 39 as i32;
    s_controls.sensitivity.generic.callback =
        Some(Controls_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.sensitivity.minvalue = 2 as i32 as f32;
    s_controls.sensitivity.maxvalue = 30 as i32 as f32;
    s_controls.sensitivity.generic.statusbar =
        Some(Controls_StatusBar as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.gesture.generic.type_0 = 2 as i32;
    s_controls.gesture.generic.flags =
        0x4 as i32 as u32 | 0x100 as i32 as u32 | 0x2000 as i32 as u32 | 0x1000 as i32 as u32;
    s_controls.gesture.generic.callback =
        Some(Controls_ActionEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.gesture.generic.ownerdraw =
        Some(Controls_DrawKeyBinding as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.gesture.generic.id = 29 as i32;
    s_controls.chat.generic.type_0 = 2 as i32;
    s_controls.chat.generic.flags =
        0x4 as i32 as u32 | 0x100 as i32 as u32 | 0x2000 as i32 as u32 | 0x1000 as i32 as u32;
    s_controls.chat.generic.callback =
        Some(Controls_ActionEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.chat.generic.ownerdraw =
        Some(Controls_DrawKeyBinding as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.chat.generic.id = 30 as i32;
    s_controls.chat2.generic.type_0 = 2 as i32;
    s_controls.chat2.generic.flags =
        0x4 as i32 as u32 | 0x100 as i32 as u32 | 0x2000 as i32 as u32 | 0x1000 as i32 as u32;
    s_controls.chat2.generic.callback =
        Some(Controls_ActionEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.chat2.generic.ownerdraw =
        Some(Controls_DrawKeyBinding as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.chat2.generic.id = 31 as i32;
    s_controls.chat3.generic.type_0 = 2 as i32;
    s_controls.chat3.generic.flags =
        0x4 as i32 as u32 | 0x100 as i32 as u32 | 0x2000 as i32 as u32 | 0x1000 as i32 as u32;
    s_controls.chat3.generic.callback =
        Some(Controls_ActionEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.chat3.generic.ownerdraw =
        Some(Controls_DrawKeyBinding as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.chat3.generic.id = 32 as i32;
    s_controls.chat4.generic.type_0 = 2 as i32;
    s_controls.chat4.generic.flags =
        0x4 as i32 as u32 | 0x100 as i32 as u32 | 0x2000 as i32 as u32 | 0x1000 as i32 as u32;
    s_controls.chat4.generic.callback =
        Some(Controls_ActionEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.chat4.generic.ownerdraw =
        Some(Controls_DrawKeyBinding as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.chat4.generic.id = 33 as i32;
    s_controls.togglemenu.generic.type_0 = 2 as i32;
    s_controls.togglemenu.generic.flags =
        0x4 as i32 as u32 | 0x100 as i32 as u32 | 0x2000 as i32 as u32 | 0x1000 as i32 as u32;
    s_controls.togglemenu.generic.callback =
        Some(Controls_ActionEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.togglemenu.generic.ownerdraw =
        Some(Controls_DrawKeyBinding as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.togglemenu.generic.id = 34 as i32;
    s_controls.joyenable.generic.type_0 = 5 as i32;
    s_controls.joyenable.generic.flags = 0x2 as i32 as u32;
    s_controls.joyenable.generic.x = 640 as i32 / 2 as i32;
    s_controls.joyenable.generic.name = b"joystick\x00" as *const u8 as *const libc::c_char;
    s_controls.joyenable.generic.id = 40 as i32;
    s_controls.joyenable.generic.callback =
        Some(Controls_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.joyenable.generic.statusbar =
        Some(Controls_StatusBar as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.joythreshold.generic.type_0 = 1 as i32;
    s_controls.joythreshold.generic.x = 640 as i32 / 2 as i32;
    s_controls.joythreshold.generic.flags = 0x2 as i32 as u32;
    s_controls.joythreshold.generic.name =
        b"joystick threshold\x00" as *const u8 as *const libc::c_char;
    s_controls.joythreshold.generic.id = 41 as i32;
    s_controls.joythreshold.generic.callback =
        Some(Controls_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_controls.joythreshold.minvalue = 0.05f32;
    s_controls.joythreshold.maxvalue = 0.75f32;
    s_controls.joythreshold.generic.statusbar =
        Some(Controls_StatusBar as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_controls.name.generic.type_0 = 9 as i32;
    s_controls.name.generic.flags = 0x8 as i32 as u32 | 0x4000 as i32 as u32;
    s_controls.name.generic.x = 320 as i32;
    s_controls.name.generic.y = 440 as i32;
    s_controls.name.string = playername.as_mut_ptr();
    s_controls.name.style = 0x1 as i32;
    s_controls.name.color = text_color_normal.as_mut_ptr();
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.banner as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.framel as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.framer as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.player as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.name as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.looking as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.movement as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.weapons as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.misc as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.sensitivity as *mut menuslider_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.smoothmouse as *mut menuradiobutton_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.invertmouse as *mut menuradiobutton_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.lookup as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.lookdown as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.mouselook as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.freelook as *mut menuradiobutton_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.centerview as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.zoomview as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.joyenable as *mut menuradiobutton_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.joythreshold as *mut menuslider_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.alwaysrun as *mut menuradiobutton_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.run as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.walkforward as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.backpedal as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.stepleft as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.stepright as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.moveup as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.movedown as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.turnleft as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.turnright as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.sidestep as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.attack as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.nextweapon as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.prevweapon as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.autoswitch as *mut menuradiobutton_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.chainsaw as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.machinegun as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.shotgun as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.grenadelauncher as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.rocketlauncher as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.lightning as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.railgun as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.plasma as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.bfg as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.showscores as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.useitem as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.gesture as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.chat as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.chat2 as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.chat3 as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.chat4 as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.togglemenu as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu as *mut _ as *mut _tag_menuframework,
        &mut s_controls.back as *mut menubitmap_s as *mut libc::c_void,
    );
    trap_Cvar_VariableStringBuffer(
        b"name\x00" as *const u8 as *const libc::c_char,
        s_controls.name.string,
        16 as i32,
    );
    Q_CleanStr(s_controls.name.string);
    // initialize the configurable cvars
    Controls_InitCvars();
    // initialize the current config
    Controls_GetConfig();
    // intialize the model
    Controls_InitModel();
    // intialize the weapons
    Controls_InitWeapons();
    // initial default section
    s_controls.section = 1 as i32;
    // update the ui
    Controls_Update();
}
/*
=================
Controls_Cache
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Controls_Cache() {
    trap_R_RegisterShaderNoMip(b"menu/art/back_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char);
}
/*
=================
UI_ControlsMenu
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_ControlsMenu() {
    Controls_MenuInit();
    UI_PushMenu(&mut s_controls.menu as *mut _ as *mut _tag_menuframework);
}
unsafe extern "C" fn run_static_initializers() {
    g_movement_controls = [
        &mut s_controls.alwaysrun as *mut menuradiobutton_s as *mut menucommon_s,
        &mut s_controls.run as *mut menuaction_s as *mut menucommon_s,
        &mut s_controls.walkforward as *mut menuaction_s as *mut menucommon_s,
        &mut s_controls.backpedal as *mut menuaction_s as *mut menucommon_s,
        &mut s_controls.stepleft as *mut menuaction_s as *mut menucommon_s,
        &mut s_controls.stepright as *mut menuaction_s as *mut menucommon_s,
        &mut s_controls.moveup as *mut menuaction_s as *mut menucommon_s,
        &mut s_controls.movedown as *mut menuaction_s as *mut menucommon_s,
        &mut s_controls.turnleft as *mut menuaction_s as *mut menucommon_s,
        &mut s_controls.turnright as *mut menuaction_s as *mut menucommon_s,
        &mut s_controls.sidestep as *mut menuaction_s as *mut menucommon_s,
        std::ptr::null_mut(),
    ];
    g_weapons_controls = [
        &mut s_controls.attack as *mut menuaction_s as *mut menucommon_s,
        &mut s_controls.nextweapon as *mut menuaction_s as *mut menucommon_s,
        &mut s_controls.prevweapon as *mut menuaction_s as *mut menucommon_s,
        &mut s_controls.autoswitch as *mut menuradiobutton_s as *mut menucommon_s,
        &mut s_controls.chainsaw as *mut menuaction_s as *mut menucommon_s,
        &mut s_controls.machinegun as *mut menuaction_s as *mut menucommon_s,
        &mut s_controls.shotgun as *mut menuaction_s as *mut menucommon_s,
        &mut s_controls.grenadelauncher as *mut menuaction_s as *mut menucommon_s,
        &mut s_controls.rocketlauncher as *mut menuaction_s as *mut menucommon_s,
        &mut s_controls.lightning as *mut menuaction_s as *mut menucommon_s,
        &mut s_controls.railgun as *mut menuaction_s as *mut menucommon_s,
        &mut s_controls.plasma as *mut menuaction_s as *mut menucommon_s,
        &mut s_controls.bfg as *mut menuaction_s as *mut menucommon_s,
        std::ptr::null_mut(),
    ];
    g_looking_controls = [
        &mut s_controls.sensitivity as *mut menuslider_s as *mut menucommon_s,
        &mut s_controls.smoothmouse as *mut menuradiobutton_s as *mut menucommon_s,
        &mut s_controls.invertmouse as *mut menuradiobutton_s as *mut menucommon_s,
        &mut s_controls.lookup as *mut menuaction_s as *mut menucommon_s,
        &mut s_controls.lookdown as *mut menuaction_s as *mut menucommon_s,
        &mut s_controls.mouselook as *mut menuaction_s as *mut menucommon_s,
        &mut s_controls.freelook as *mut menuradiobutton_s as *mut menucommon_s,
        &mut s_controls.centerview as *mut menuaction_s as *mut menucommon_s,
        &mut s_controls.zoomview as *mut menuaction_s as *mut menucommon_s,
        &mut s_controls.joyenable as *mut menuradiobutton_s as *mut menucommon_s,
        &mut s_controls.joythreshold as *mut menuslider_s as *mut menucommon_s,
        std::ptr::null_mut(),
    ];
    g_misc_controls = [
        &mut s_controls.showscores as *mut menuaction_s as *mut menucommon_s,
        &mut s_controls.useitem as *mut menuaction_s as *mut menucommon_s,
        &mut s_controls.gesture as *mut menuaction_s as *mut menucommon_s,
        &mut s_controls.chat as *mut menuaction_s as *mut menucommon_s,
        &mut s_controls.chat2 as *mut menuaction_s as *mut menucommon_s,
        &mut s_controls.chat3 as *mut menuaction_s as *mut menucommon_s,
        &mut s_controls.chat4 as *mut menuaction_s as *mut menucommon_s,
        &mut s_controls.togglemenu as *mut menuaction_s as *mut menucommon_s,
        std::ptr::null_mut(),
    ]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
