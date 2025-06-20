use ::libc;

pub use crate::bg_public_h::animation_s;
pub use crate::bg_public_h::animation_t;
pub use crate::bg_public_h::gametype_t;
pub use crate::bg_public_h::gender_t;
pub use crate::bg_public_h::gitem_s;
pub use crate::bg_public_h::gitem_t;
pub use crate::bg_public_h::holdable_t;
pub use crate::bg_public_h::itemType_t;
pub use crate::bg_public_h::team_t;
pub use crate::bg_public_h::BOTH_DEAD1;
pub use crate::bg_public_h::BOTH_DEAD2;
pub use crate::bg_public_h::BOTH_DEAD3;
pub use crate::bg_public_h::BOTH_DEATH1;
pub use crate::bg_public_h::BOTH_DEATH2;
pub use crate::bg_public_h::BOTH_DEATH3;
pub use crate::bg_public_h::ET_BEAM;
pub use crate::bg_public_h::ET_EVENTS;
pub use crate::bg_public_h::ET_GENERAL;
pub use crate::bg_public_h::ET_GRAPPLE;
pub use crate::bg_public_h::ET_INVISIBLE;
pub use crate::bg_public_h::ET_ITEM;
pub use crate::bg_public_h::ET_MISSILE;
pub use crate::bg_public_h::ET_MOVER;
pub use crate::bg_public_h::ET_PLAYER;
pub use crate::bg_public_h::ET_PORTAL;
pub use crate::bg_public_h::ET_PUSH_TRIGGER;
pub use crate::bg_public_h::ET_SPEAKER;
pub use crate::bg_public_h::ET_TEAM;
pub use crate::bg_public_h::ET_TELEPORT_TRIGGER;
pub use crate::bg_public_h::EV_BULLET;
pub use crate::bg_public_h::EV_BULLET_HIT_FLESH;
pub use crate::bg_public_h::EV_BULLET_HIT_WALL;
pub use crate::bg_public_h::EV_CHANGE_WEAPON;
pub use crate::bg_public_h::EV_DEATH1;
pub use crate::bg_public_h::EV_DEATH2;
pub use crate::bg_public_h::EV_DEATH3;
pub use crate::bg_public_h::EV_DEBUG_LINE;
pub use crate::bg_public_h::EV_FALL_FAR;
pub use crate::bg_public_h::EV_FALL_MEDIUM;
pub use crate::bg_public_h::EV_FALL_SHORT;
pub use crate::bg_public_h::EV_FIRE_WEAPON;
pub use crate::bg_public_h::EV_FOOTSPLASH;
pub use crate::bg_public_h::EV_FOOTSTEP;
pub use crate::bg_public_h::EV_FOOTSTEP_METAL;
pub use crate::bg_public_h::EV_FOOTWADE;
pub use crate::bg_public_h::EV_GENERAL_SOUND;
pub use crate::bg_public_h::EV_GIB_PLAYER;
pub use crate::bg_public_h::EV_GLOBAL_ITEM_PICKUP;
pub use crate::bg_public_h::EV_GLOBAL_SOUND;
pub use crate::bg_public_h::EV_GLOBAL_TEAM_SOUND;
pub use crate::bg_public_h::EV_GRENADE_BOUNCE;
pub use crate::bg_public_h::EV_INVUL_IMPACT;
pub use crate::bg_public_h::EV_ITEM_PICKUP;
pub use crate::bg_public_h::EV_ITEM_POP;
pub use crate::bg_public_h::EV_ITEM_RESPAWN;
pub use crate::bg_public_h::EV_JUICED;
pub use crate::bg_public_h::EV_JUMP;
pub use crate::bg_public_h::EV_JUMP_PAD;
pub use crate::bg_public_h::EV_KAMIKAZE;
pub use crate::bg_public_h::EV_LIGHTNINGBOLT;
pub use crate::bg_public_h::EV_MISSILE_HIT;
pub use crate::bg_public_h::EV_MISSILE_MISS;
pub use crate::bg_public_h::EV_MISSILE_MISS_METAL;
pub use crate::bg_public_h::EV_NOAMMO;
pub use crate::bg_public_h::EV_NONE;
pub use crate::bg_public_h::EV_OBELISKEXPLODE;
pub use crate::bg_public_h::EV_OBELISKPAIN;
pub use crate::bg_public_h::EV_OBITUARY;
pub use crate::bg_public_h::EV_PAIN;
pub use crate::bg_public_h::EV_PLAYER_TELEPORT_IN;
pub use crate::bg_public_h::EV_PLAYER_TELEPORT_OUT;
pub use crate::bg_public_h::EV_POWERUP_BATTLESUIT;
pub use crate::bg_public_h::EV_POWERUP_QUAD;
pub use crate::bg_public_h::EV_POWERUP_REGEN;
pub use crate::bg_public_h::EV_PROXIMITY_MINE_STICK;
pub use crate::bg_public_h::EV_PROXIMITY_MINE_TRIGGER;
pub use crate::bg_public_h::EV_RAILTRAIL;
pub use crate::bg_public_h::EV_SCOREPLUM;
pub use crate::bg_public_h::EV_SHOTGUN;
pub use crate::bg_public_h::EV_STEP_12;
pub use crate::bg_public_h::EV_STEP_16;
pub use crate::bg_public_h::EV_STEP_4;
pub use crate::bg_public_h::EV_STEP_8;
pub use crate::bg_public_h::EV_STOPLOOPINGSOUND;
pub use crate::bg_public_h::EV_SWIM;
pub use crate::bg_public_h::EV_TAUNT;
pub use crate::bg_public_h::EV_TAUNT_FOLLOWME;
pub use crate::bg_public_h::EV_TAUNT_GETFLAG;
pub use crate::bg_public_h::EV_TAUNT_GUARDBASE;
pub use crate::bg_public_h::EV_TAUNT_NO;
pub use crate::bg_public_h::EV_TAUNT_PATROL;
pub use crate::bg_public_h::EV_TAUNT_YES;
pub use crate::bg_public_h::EV_USE_ITEM0;
pub use crate::bg_public_h::EV_USE_ITEM1;
pub use crate::bg_public_h::EV_USE_ITEM10;
pub use crate::bg_public_h::EV_USE_ITEM11;
pub use crate::bg_public_h::EV_USE_ITEM12;
pub use crate::bg_public_h::EV_USE_ITEM13;
pub use crate::bg_public_h::EV_USE_ITEM14;
pub use crate::bg_public_h::EV_USE_ITEM15;
pub use crate::bg_public_h::EV_USE_ITEM2;
pub use crate::bg_public_h::EV_USE_ITEM3;
pub use crate::bg_public_h::EV_USE_ITEM4;
pub use crate::bg_public_h::EV_USE_ITEM5;
pub use crate::bg_public_h::EV_USE_ITEM6;
pub use crate::bg_public_h::EV_USE_ITEM7;
pub use crate::bg_public_h::EV_USE_ITEM8;
pub use crate::bg_public_h::EV_USE_ITEM9;
pub use crate::bg_public_h::EV_WATER_CLEAR;
pub use crate::bg_public_h::EV_WATER_LEAVE;
pub use crate::bg_public_h::EV_WATER_TOUCH;
pub use crate::bg_public_h::EV_WATER_UNDER;
pub use crate::bg_public_h::FLAG_RUN;
pub use crate::bg_public_h::FLAG_STAND;
pub use crate::bg_public_h::FLAG_STAND2RUN;
pub use crate::bg_public_h::GENDER_FEMALE;
pub use crate::bg_public_h::GENDER_MALE;
pub use crate::bg_public_h::GENDER_NEUTER;
pub use crate::bg_public_h::GTS_BLUEOBELISK_ATTACKED;
pub use crate::bg_public_h::GTS_BLUETEAM_SCORED;
pub use crate::bg_public_h::GTS_BLUETEAM_TOOK_LEAD;
pub use crate::bg_public_h::GTS_BLUE_CAPTURE;
pub use crate::bg_public_h::GTS_BLUE_RETURN;
pub use crate::bg_public_h::GTS_BLUE_TAKEN;
pub use crate::bg_public_h::GTS_KAMIKAZE;
pub use crate::bg_public_h::GTS_REDOBELISK_ATTACKED;
pub use crate::bg_public_h::GTS_REDTEAM_SCORED;
pub use crate::bg_public_h::GTS_REDTEAM_TOOK_LEAD;
pub use crate::bg_public_h::GTS_RED_CAPTURE;
pub use crate::bg_public_h::GTS_RED_RETURN;
pub use crate::bg_public_h::GTS_RED_TAKEN;
pub use crate::bg_public_h::GTS_TEAMS_ARE_TIED;
pub use crate::bg_public_h::GT_1FCTF;
pub use crate::bg_public_h::GT_CTF;
pub use crate::bg_public_h::GT_FFA;
pub use crate::bg_public_h::GT_HARVESTER;
pub use crate::bg_public_h::GT_MAX_GAME_TYPE;
pub use crate::bg_public_h::GT_OBELISK;
pub use crate::bg_public_h::GT_SINGLE_PLAYER;
pub use crate::bg_public_h::GT_TEAM;
pub use crate::bg_public_h::GT_TOURNAMENT;
pub use crate::bg_public_h::HI_INVULNERABILITY;
pub use crate::bg_public_h::HI_KAMIKAZE;
pub use crate::bg_public_h::HI_MEDKIT;
pub use crate::bg_public_h::HI_NONE;
pub use crate::bg_public_h::HI_NUM_HOLDABLE;
pub use crate::bg_public_h::HI_PORTAL;
pub use crate::bg_public_h::HI_TELEPORTER;
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
pub use crate::bg_public_h::MOD_BFG;
pub use crate::bg_public_h::MOD_BFG_SPLASH;
pub use crate::bg_public_h::MOD_CRUSH;
pub use crate::bg_public_h::MOD_FALLING;
pub use crate::bg_public_h::MOD_GAUNTLET;
pub use crate::bg_public_h::MOD_GRAPPLE;
pub use crate::bg_public_h::MOD_GRENADE;
pub use crate::bg_public_h::MOD_GRENADE_SPLASH;
pub use crate::bg_public_h::MOD_LAVA;
pub use crate::bg_public_h::MOD_LIGHTNING;
pub use crate::bg_public_h::MOD_MACHINEGUN;
pub use crate::bg_public_h::MOD_PLASMA;
pub use crate::bg_public_h::MOD_PLASMA_SPLASH;
pub use crate::bg_public_h::MOD_RAILGUN;
pub use crate::bg_public_h::MOD_ROCKET;
pub use crate::bg_public_h::MOD_ROCKET_SPLASH;
pub use crate::bg_public_h::MOD_SHOTGUN;
pub use crate::bg_public_h::MOD_SLIME;
pub use crate::bg_public_h::MOD_SUICIDE;
pub use crate::bg_public_h::MOD_TARGET_LASER;
pub use crate::bg_public_h::MOD_TELEFRAG;
pub use crate::bg_public_h::MOD_TRIGGER_HURT;
pub use crate::bg_public_h::MOD_UNKNOWN;
pub use crate::bg_public_h::MOD_WATER;
pub use crate::bg_public_h::PERS_ASSIST_COUNT;
pub use crate::bg_public_h::PERS_ATTACKEE_ARMOR;
pub use crate::bg_public_h::PERS_ATTACKER;
pub use crate::bg_public_h::PERS_CAPTURES;
pub use crate::bg_public_h::PERS_DEFEND_COUNT;
pub use crate::bg_public_h::PERS_EXCELLENT_COUNT;
pub use crate::bg_public_h::PERS_GAUNTLET_FRAG_COUNT;
pub use crate::bg_public_h::PERS_HITS;
pub use crate::bg_public_h::PERS_IMPRESSIVE_COUNT;
pub use crate::bg_public_h::PERS_KILLED;
pub use crate::bg_public_h::PERS_PLAYEREVENTS;
pub use crate::bg_public_h::PERS_RANK;
pub use crate::bg_public_h::PERS_SCORE;
pub use crate::bg_public_h::PERS_SPAWN_COUNT;
pub use crate::bg_public_h::PERS_TEAM;
pub use crate::bg_public_h::PW_AMMOREGEN;
pub use crate::bg_public_h::PW_BATTLESUIT;
pub use crate::bg_public_h::PW_BLUEFLAG;
pub use crate::bg_public_h::PW_DOUBLER;
pub use crate::bg_public_h::PW_FLIGHT;
pub use crate::bg_public_h::PW_GUARD;
pub use crate::bg_public_h::PW_HASTE;
pub use crate::bg_public_h::PW_INVIS;
pub use crate::bg_public_h::PW_INVULNERABILITY;
pub use crate::bg_public_h::PW_NEUTRALFLAG;
pub use crate::bg_public_h::PW_NONE;
pub use crate::bg_public_h::PW_NUM_POWERUPS;
pub use crate::bg_public_h::PW_QUAD;
pub use crate::bg_public_h::PW_REDFLAG;
pub use crate::bg_public_h::PW_REGEN;
pub use crate::bg_public_h::PW_SCOUT;
pub use crate::bg_public_h::TEAM_BLUE;
pub use crate::bg_public_h::TEAM_FREE;
pub use crate::bg_public_h::TEAM_NUM_TEAMS;
pub use crate::bg_public_h::TEAM_RED;
pub use crate::bg_public_h::TEAM_SPECTATOR;
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
pub use crate::cg_public_h::snapshot_t;
pub use crate::src::game::bg_misc::bg_itemlist;
pub use crate::src::game::bg_misc::bg_numItems;
pub use crate::src::game::bg_misc::BG_EvaluateTrajectory;
pub use crate::src::game::bg_misc::BG_FindItemForHoldable;
pub use crate::src::qcommon::q_math::ByteToDir;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cvarHandle_t;
pub use crate::src::qcommon::q_shared::entityState_s;
pub use crate::src::qcommon::q_shared::entityState_t;
pub use crate::src::qcommon::q_shared::gameState_t;
pub use crate::src::qcommon::q_shared::playerState_s;
pub use crate::src::qcommon::q_shared::playerState_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::trType_t;
pub use crate::src::qcommon::q_shared::trajectory_t;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::vmCvar_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Info_ValueForKey;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::CHAN_ANNOUNCER;
pub use crate::src::qcommon::q_shared::CHAN_AUTO;
pub use crate::src::qcommon::q_shared::CHAN_BODY;
pub use crate::src::qcommon::q_shared::CHAN_ITEM;
pub use crate::src::qcommon::q_shared::CHAN_LOCAL;
pub use crate::src::qcommon::q_shared::CHAN_LOCAL_SOUND;
pub use crate::src::qcommon::q_shared::CHAN_VOICE;
pub use crate::src::qcommon::q_shared::CHAN_WEAPON;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;
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

pub use crate::cg_local_h::centity_s;
pub use crate::cg_local_h::centity_t;
pub use crate::cg_local_h::cgMedia_t;
pub use crate::cg_local_h::cg_t;
pub use crate::cg_local_h::cgs_t;
pub use crate::cg_local_h::clientInfo_t;
pub use crate::cg_local_h::footstep_t;
pub use crate::cg_local_h::impactSound_t;
pub use crate::cg_local_h::leBounceSoundType_t;
pub use crate::cg_local_h::leMarkType_t;
pub use crate::cg_local_h::leType_t;
pub use crate::cg_local_h::lerpFrame_t;
pub use crate::cg_local_h::localEntity_s;
pub use crate::cg_local_h::localEntity_t;
pub use crate::cg_local_h::playerEntity_t;
pub use crate::cg_local_h::score_t;
pub use crate::cg_local_h::FOOTSTEP_BOOT;
pub use crate::cg_local_h::FOOTSTEP_ENERGY;
pub use crate::cg_local_h::FOOTSTEP_FLESH;
pub use crate::cg_local_h::FOOTSTEP_MECH;
pub use crate::cg_local_h::FOOTSTEP_METAL;
pub use crate::cg_local_h::FOOTSTEP_NORMAL;
pub use crate::cg_local_h::FOOTSTEP_SPLASH;
pub use crate::cg_local_h::FOOTSTEP_TOTAL;
pub use crate::cg_local_h::IMPACTSOUND_DEFAULT;
pub use crate::cg_local_h::IMPACTSOUND_FLESH;
pub use crate::cg_local_h::IMPACTSOUND_METAL;
pub use crate::cg_local_h::LEBS_BLOOD;
pub use crate::cg_local_h::LEBS_BRASS;
pub use crate::cg_local_h::LEBS_NONE;
pub use crate::cg_local_h::LEF_PUFF_DONT_SCALE;
pub use crate::cg_local_h::LEF_SOUND1;
pub use crate::cg_local_h::LEF_SOUND2;
pub use crate::cg_local_h::LEF_TUMBLE;
pub use crate::cg_local_h::LEMT_BLOOD;
pub use crate::cg_local_h::LEMT_BURN;
pub use crate::cg_local_h::LEMT_NONE;
pub use crate::cg_local_h::LE_EXPLOSION;
pub use crate::cg_local_h::LE_FADE_RGB;
pub use crate::cg_local_h::LE_FALL_SCALE_FADE;
pub use crate::cg_local_h::LE_FRAGMENT;
pub use crate::cg_local_h::LE_MARK;
pub use crate::cg_local_h::LE_MOVE_SCALE_FADE;
pub use crate::cg_local_h::LE_SCALE_FADE;
pub use crate::cg_local_h::LE_SCOREPLUM;
pub use crate::cg_local_h::LE_SPRITE_EXPLOSION;
pub use crate::src::cgame::cg_draw::CG_CenterPrint;
pub use crate::src::cgame::cg_effects::CG_GibPlayer;
pub use crate::src::cgame::cg_effects::CG_ScorePlum;
pub use crate::src::cgame::cg_effects::CG_SmokePuff;
pub use crate::src::cgame::cg_effects::CG_SpawnEffect;
pub use crate::src::cgame::cg_ents::CG_Beam;
pub use crate::src::cgame::cg_ents::CG_SetEntitySoundPosition;
pub use crate::src::cgame::cg_main::cg;
pub use crate::src::cgame::cg_main::cg_autoswitch;
pub use crate::src::cgame::cg_main::cg_debugEvents;
pub use crate::src::cgame::cg_main::cg_drawGun;
pub use crate::src::cgame::cg_main::cg_footsteps;
pub use crate::src::cgame::cg_main::cg_nopredict;
pub use crate::src::cgame::cg_main::cg_synchronousClients;
pub use crate::src::cgame::cg_main::cgs;
pub use crate::src::cgame::cg_main::CG_ConfigString;
pub use crate::src::cgame::cg_main::CG_Error;
pub use crate::src::cgame::cg_main::CG_Printf;
pub use crate::src::cgame::cg_players::CG_CustomSound;
pub use crate::src::cgame::cg_predict::CG_PointContents;
pub use crate::src::cgame::cg_syscalls::trap_S_RegisterSound;
pub use crate::src::cgame::cg_syscalls::trap_S_StartSound;
pub use crate::src::cgame::cg_syscalls::trap_S_StopLoopingSound;
pub use crate::src::cgame::cg_view::CG_AddBufferedSound;
pub use crate::src::cgame::cg_weapons::CG_Bullet;
pub use crate::src::cgame::cg_weapons::CG_FireWeapon;
pub use crate::src::cgame::cg_weapons::CG_MissileHitPlayer;
pub use crate::src::cgame::cg_weapons::CG_MissileHitWall;
pub use crate::src::cgame::cg_weapons::CG_OutOfAmmoChange;
pub use crate::src::cgame::cg_weapons::CG_RailTrail;
pub use crate::src::cgame::cg_weapons::CG_ShotgunFire;

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
// cg_event.c -- handle entity events at snapshot or playerstate transitions
// for the voice chats
//==========================================================================
/*
===================
CG_PlaceString

Also called by scoreboard drawing
===================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_PlaceString(mut rank: i32) -> *const libc::c_char {
    static mut str: [libc::c_char; 64] = [0; 64];
    let mut s: *mut libc::c_char = std::ptr::null_mut();
    let mut t: *mut libc::c_char = std::ptr::null_mut();
    if rank & 0x4000 as i32 != 0 {
        rank &= !(0x4000 as i32);
        t = b"Tied for \x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else {
        t = b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    if rank == 1 as i32 {
        s = b"^41st^7\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    // draw in blue
    } else if rank == 2 as i32 {
        s = b"^12nd^7\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    // draw in red
    } else if rank == 3 as i32 {
        s = b"^33rd^7\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    // draw in yellow
    } else if rank == 11 as i32 {
        s = b"11th\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if rank == 12 as i32 {
        s = b"12th\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if rank == 13 as i32 {
        s = b"13th\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if rank % 10 as i32 == 1 as i32 {
        s = va(
            b"%ist\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            rank,
        )
    } else if rank % 10 as i32 == 2 as i32 {
        s = va(
            b"%ind\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            rank,
        )
    } else if rank % 10 as i32 == 3 as i32 {
        s = va(
            b"%ird\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            rank,
        )
    } else {
        s = va(
            b"%ith\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            rank,
        )
    }
    Com_sprintf(
        str.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
        b"%s%s\x00" as *const u8 as *const libc::c_char,
        t,
        s,
    );
    return str.as_mut_ptr();
}
/*
=============
CG_Obituary
=============
*/

unsafe extern "C" fn CG_Obituary(mut ent: *mut entityState_t) {
    let mut mod_0: i32 = 0;
    let mut target: i32 = 0;
    let mut attacker: i32 = 0;
    let mut message: *mut libc::c_char = std::ptr::null_mut();
    let mut message2: *mut libc::c_char = std::ptr::null_mut();
    let mut targetInfo: *const libc::c_char = std::ptr::null();
    let mut attackerInfo: *const libc::c_char = std::ptr::null();
    let mut targetName: [libc::c_char; 32] = [0; 32];
    let mut attackerName: [libc::c_char; 32] = [0; 32];
    let mut gender: gender_t = GENDER_MALE;
    let mut ci: *mut clientInfo_t = std::ptr::null_mut();
    target = (*ent).otherEntityNum;
    attacker = (*ent).otherEntityNum2;
    mod_0 = (*ent).eventParm;
    if target < 0 as i32 || target >= 64 as i32 {
        CG_Error(b"CG_Obituary: target out of range\x00" as *const u8 as *const libc::c_char);
    }
    ci = &mut *cgs.clientinfo.as_mut_ptr().offset(target as isize) as *mut clientInfo_t;
    if attacker < 0 as i32 || attacker >= 64 as i32 {
        attacker = ((1 as i32) << 10 as i32) - 2 as i32;
        attackerInfo = std::ptr::null()
    } else {
        attackerInfo = CG_ConfigString(32 as i32 + 256 as i32 + 256 as i32 + attacker)
    }
    targetInfo = CG_ConfigString(32 as i32 + 256 as i32 + 256 as i32 + target);
    if targetInfo.is_null() {
        return;
    }
    Q_strncpyz(
        targetName.as_mut_ptr(),
        Info_ValueForKey(targetInfo, b"n\x00" as *const u8 as *const libc::c_char),
        (::std::mem::size_of::<[libc::c_char; 32]>() as usize).wrapping_sub(2 as i32 as usize)
            as i32,
    );
    libc::strcat(
        targetName.as_mut_ptr(),
        b"^7\x00" as *const u8 as *const libc::c_char,
    );
    message2 = b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    // check for single client messages
    match mod_0 {
        20 => message = b"suicides\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        19 => message = b"cratered\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        17 => {
            message = b"was squished\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
        }
        14 => {
            message =
                b"sank like a rock\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
        }
        15 => message = b"melted\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        16 => {
            message = b"does a back flip into the lava\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char
        }
        21 => {
            message = b"saw the light\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
        }
        22 => {
            message = b"was in the wrong place\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char
        }
        _ => message = std::ptr::null_mut(),
    }
    if attacker == target {
        gender = (*ci).gender;
        match mod_0 {
            5 => {
                if gender as u32 == GENDER_FEMALE as i32 as u32 {
                    message = b"tripped on her own grenade\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char
                } else if gender as u32 == GENDER_NEUTER as i32 as u32 {
                    message = b"tripped on its own grenade\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char
                } else {
                    message = b"tripped on his own grenade\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char
                }
            }
            7 => {
                if gender as u32 == GENDER_FEMALE as i32 as u32 {
                    message = b"blew herself up\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char
                } else if gender as u32 == GENDER_NEUTER as i32 as u32 {
                    message = b"blew itself up\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char
                } else {
                    message = b"blew himself up\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char
                }
            }
            9 => {
                if gender as u32 == GENDER_FEMALE as i32 as u32 {
                    message = b"melted herself\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char
                } else if gender as u32 == GENDER_NEUTER as i32 as u32 {
                    message = b"melted itself\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char
                } else {
                    message = b"melted himself\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char
                }
            }
            13 => {
                message = b"should have used a smaller gun\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
            }
            _ => {
                if gender as u32 == GENDER_FEMALE as i32 as u32 {
                    message = b"killed herself\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char
                } else if gender as u32 == GENDER_NEUTER as i32 as u32 {
                    message = b"killed itself\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char
                } else {
                    message = b"killed himself\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char
                }
            }
        }
    }
    if !message.is_null() {
        CG_Printf(
            b"%s %s.\n\x00" as *const u8 as *const libc::c_char,
            targetName.as_mut_ptr(),
            message,
        );
        return;
    }
    // check for kill messages from the current clientNum
    if attacker == (*cg.snap).ps.clientNum {
        let mut s: *mut libc::c_char = std::ptr::null_mut();
        if (cgs.gametype as u32) < GT_TEAM as i32 as u32 {
            s = va(
                b"You fragged %s\n%s place with %i\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                targetName.as_mut_ptr(),
                CG_PlaceString((*cg.snap).ps.persistant[PERS_RANK as i32 as usize] + 1 as i32),
                (*cg.snap).ps.persistant[PERS_SCORE as i32 as usize],
            )
        } else {
            s = va(
                b"You fragged %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                targetName.as_mut_ptr(),
            )
        }
        CG_CenterPrint(s, (480 as i32 as f64 * 0.30f64) as i32, 16 as i32);
        // print the text message as well
    }
    // check for double client messages
    if attackerInfo.is_null() {
        attacker = ((1 as i32) << 10 as i32) - 2 as i32;
        libc::strcpy(
            attackerName.as_mut_ptr(),
            b"noname\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        Q_strncpyz(
            attackerName.as_mut_ptr(),
            Info_ValueForKey(attackerInfo, b"n\x00" as *const u8 as *const libc::c_char),
            (::std::mem::size_of::<[libc::c_char; 32]>() as usize).wrapping_sub(2 as i32 as usize)
                as i32,
        );
        libc::strcat(
            attackerName.as_mut_ptr(),
            b"^7\x00" as *const u8 as *const libc::c_char,
        );
        // check for kill messages about the current clientNum
        if target == (*cg.snap).ps.clientNum {
            Q_strncpyz(
                cg.killerName.as_mut_ptr(),
                attackerName.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 32]>() as usize as i32,
            );
        }
    }
    if attacker != ((1 as i32) << 10 as i32) - 2 as i32 {
        match mod_0 {
            23 => {
                message =
                    b"was caught by\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            }
            2 => {
                message =
                    b"was pummeled by\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            }
            3 => {
                message = b"was machinegunned by\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
            }
            1 => {
                message = b"was gunned down by\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
            }
            4 => {
                message = b"ate\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
                message2 =
                    b"\'s grenade\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            }
            5 => {
                message =
                    b"was shredded by\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
                message2 =
                    b"\'s shrapnel\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            }
            6 => {
                message = b"ate\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
                message2 =
                    b"\'s rocket\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            }
            7 => {
                message =
                    b"almost dodged\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
                message2 =
                    b"\'s rocket\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            }
            8 => {
                message =
                    b"was melted by\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
                message2 =
                    b"\'s plasmagun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            }
            9 => {
                message =
                    b"was melted by\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
                message2 =
                    b"\'s plasmagun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            }
            10 => {
                message =
                    b"was railed by\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            }
            11 => {
                message = b"was electrocuted by\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
            }
            12 | 13 => {
                message =
                    b"was blasted by\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
                message2 = b"\'s BFG\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            }
            18 => {
                message =
                    b"tried to invade\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
                message2 = b"\'s personal space\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
            }
            _ => {
                message =
                    b"was killed by\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            }
        }
        if !message.is_null() {
            CG_Printf(
                b"%s %s %s%s\n\x00" as *const u8 as *const libc::c_char,
                targetName.as_mut_ptr(),
                message,
                attackerName.as_mut_ptr(),
                message2,
            );
            return;
        }
    }
    // we don't know what it was
    CG_Printf(
        b"%s died.\n\x00" as *const u8 as *const libc::c_char,
        targetName.as_mut_ptr(),
    );
}
//==========================================================================
/*
===============
CG_UseItem
===============
*/

unsafe extern "C" fn CG_UseItem(mut cent: *mut centity_t) {
    let mut ci: *mut clientInfo_t = std::ptr::null_mut();
    let mut itemNum: i32 = 0;
    let mut clientNum: i32 = 0;
    let mut item: *mut gitem_t = std::ptr::null_mut();
    let mut es: *mut entityState_t = std::ptr::null_mut();
    es = &mut (*cent).currentState;
    itemNum = ((*es).event & !(0x100 as i32 | 0x200 as i32)) - EV_USE_ITEM0 as i32;
    if itemNum < 0 as i32 || itemNum > HI_NUM_HOLDABLE as i32 {
        itemNum = 0 as i32
    }
    // print a message if the local player
    if (*es).number == (*cg.snap).ps.clientNum {
        if itemNum == 0 {
            CG_CenterPrint(
                b"No item to use\x00" as *const u8 as *const libc::c_char,
                (480 as i32 as f64 * 0.30f64) as i32,
                16 as i32,
            );
        } else {
            item = BG_FindItemForHoldable(itemNum as holdable_t) as *mut gitem_s;
            CG_CenterPrint(
                va(
                    b"Use %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    (*item).pickup_name,
                ),
                (480 as i32 as f64 * 0.30f64) as i32,
                16 as i32,
            );
        }
    }
    match itemNum {
        1 => {}
        2 => {
            clientNum = (*cent).currentState.clientNum;
            if clientNum >= 0 as i32 && clientNum < 64 as i32 {
                ci = &mut *cgs.clientinfo.as_mut_ptr().offset(clientNum as isize)
                    as *mut clientInfo_t;
                (*ci).medkitUsageTime = cg.time
            }
            trap_S_StartSound(
                std::ptr::null_mut(),
                (*es).number,
                CHAN_BODY as i32,
                cgs.media.medkitSound,
            );
        }
        0 | _ => {
            trap_S_StartSound(
                std::ptr::null_mut(),
                (*es).number,
                CHAN_BODY as i32,
                cgs.media.useNothingSound,
            );
        }
    };
}
/*
================
CG_ItemPickup

A new item was picked up this frame
================
*/

unsafe extern "C" fn CG_ItemPickup(mut itemNum: i32) {
    cg.itemPickup = itemNum;
    cg.itemPickupTime = cg.time;
    cg.itemPickupBlendTime = cg.time;
    // see if it should be the grabbed weapon
    if (*bg_itemlist.as_mut_ptr().offset(itemNum as isize)).giType as u32 == IT_WEAPON as i32 as u32
    {
        // select it immediately
        if cg_autoswitch.integer != 0
            && (*bg_itemlist.as_mut_ptr().offset(itemNum as isize)).giTag != WP_MACHINEGUN as i32
        {
            cg.weaponSelectTime = cg.time;
            cg.weaponSelect = (*bg_itemlist.as_mut_ptr().offset(itemNum as isize)).giTag
        }
    };
}
/*
================
CG_WaterLevel

Returns waterlevel for entity origin
================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_WaterLevel(mut cent: *mut centity_t) -> i32 {
    let mut point: vec3_t = [0.; 3];
    let mut contents: i32 = 0;
    let mut sample1: i32 = 0;
    let mut sample2: i32 = 0;
    let mut anim: i32 = 0;
    let mut waterlevel: i32 = 0;
    let mut viewheight: i32 = 0;
    anim = (*cent).currentState.legsAnim & !(128 as i32);
    if anim == LEGS_WALKCR as i32 || anim == LEGS_IDLECR as i32 {
        viewheight = 12 as i32
    } else {
        viewheight = 26 as i32
    }
    //
    // get waterlevel, accounting for ducking
    //
    waterlevel = 0 as i32;
    point[0 as i32 as usize] = (*cent).lerpOrigin[0 as i32 as usize];
    point[1 as i32 as usize] = (*cent).lerpOrigin[1 as i32 as usize];
    point[2 as i32 as usize] =
        (*cent).lerpOrigin[2 as i32 as usize] + -(24 as i32) as f32 + 1 as i32 as f32;
    contents = CG_PointContents(point.as_mut_ptr() as *const vec_t, -(1 as i32));
    if contents & (32 as i32 | 8 as i32 | 16 as i32) != 0 {
        sample2 = viewheight - -(24 as i32);
        sample1 = sample2 / 2 as i32;
        waterlevel = 1 as i32;
        point[2 as i32 as usize] =
            (*cent).lerpOrigin[2 as i32 as usize] + -(24 as i32) as f32 + sample1 as f32;
        contents = CG_PointContents(point.as_mut_ptr() as *const vec_t, -(1 as i32));
        if contents & (32 as i32 | 8 as i32 | 16 as i32) != 0 {
            waterlevel = 2 as i32;
            point[2 as i32 as usize] =
                (*cent).lerpOrigin[2 as i32 as usize] + -(24 as i32) as f32 + sample2 as f32;
            contents = CG_PointContents(point.as_mut_ptr() as *const vec_t, -(1 as i32));
            if contents & (32 as i32 | 8 as i32 | 16 as i32) != 0 {
                waterlevel = 3 as i32
            }
        }
    }
    return waterlevel;
}
/*
================
CG_PainEvent

Also called by playerstate transition
================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_PainEvent(mut cent: *mut centity_t, mut health: i32) {
    let mut snd: *mut libc::c_char = std::ptr::null_mut();
    // don't do more than two pain sounds a second
    if cg.time - (*cent).pe.painTime < 500 as i32 {
        return;
    }
    if health < 25 as i32 {
        snd = b"*pain25_1.wav\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if health < 50 as i32 {
        snd = b"*pain50_1.wav\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if health < 75 as i32 {
        snd = b"*pain75_1.wav\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else {
        snd = b"*pain100_1.wav\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    // play a gurp sound instead of a normal pain sound
    if CG_WaterLevel(cent) == 3 as i32 {
        if libc::rand() & 1 as i32 != 0 {
            trap_S_StartSound(
                std::ptr::null_mut(),
                (*cent).currentState.number,
                CHAN_VOICE as i32,
                CG_CustomSound(
                    (*cent).currentState.number,
                    b"sound/player/gurp1.wav\x00" as *const u8 as *const libc::c_char,
                ),
            );
        } else {
            trap_S_StartSound(
                std::ptr::null_mut(),
                (*cent).currentState.number,
                CHAN_VOICE as i32,
                CG_CustomSound(
                    (*cent).currentState.number,
                    b"sound/player/gurp2.wav\x00" as *const u8 as *const libc::c_char,
                ),
            );
        }
    } else {
        trap_S_StartSound(
            std::ptr::null_mut(),
            (*cent).currentState.number,
            CHAN_VOICE as i32,
            CG_CustomSound((*cent).currentState.number, snd),
        );
    }
    // save pain time for programitic twitch animation
    (*cent).pe.painTime = cg.time;
    (*cent).pe.painDirection ^= 1 as i32;
}
/*
==============
CG_EntityEvent

An entity has an event value
also called by CG_CheckPlayerstateEvents
==============
*/
#[no_mangle]

pub unsafe extern "C" fn CG_EntityEvent(mut cent: *mut centity_t, mut position: *mut vec_t) {
    let mut es: *mut entityState_t = std::ptr::null_mut();
    let mut event: i32 = 0;
    let mut dir: vec3_t = [0.; 3];
    let mut s: *const libc::c_char = std::ptr::null();
    let mut clientNum: i32 = 0;
    let mut ci: *mut clientInfo_t = std::ptr::null_mut();
    es = &mut (*cent).currentState;
    event = (*es).event & !(0x100 as i32 | 0x200 as i32);
    if cg_debugEvents.integer != 0 {
        CG_Printf(
            b"ent:%3i  event:%3i \x00" as *const u8 as *const libc::c_char,
            (*es).number,
            event,
        );
    }
    if event == 0 {
        if cg_debugEvents.integer != 0 {
            CG_Printf(b"ZEROEVENT\n\x00" as *const u8 as *const libc::c_char);
        }
        return;
    }
    clientNum = (*es).clientNum;
    if clientNum < 0 as i32 || clientNum >= 64 as i32 {
        clientNum = 0 as i32
    }
    ci = &mut *cgs.clientinfo.as_mut_ptr().offset(clientNum as isize) as *mut clientInfo_t;
    match event {
        1 => {
            //
            // movement generated events
            //
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_FOOTSTEP\n\x00" as *const u8 as *const libc::c_char);
            }
            if cg_footsteps.integer != 0 {
                trap_S_StartSound(
                    std::ptr::null_mut(),
                    (*es).number,
                    CHAN_BODY as i32,
                    cgs.media.footsteps[(*ci).footsteps as usize]
                        [(libc::rand() & 3 as i32) as usize],
                );
            }
        }
        2 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_FOOTSTEP_METAL\n\x00" as *const u8 as *const libc::c_char);
            }
            if cg_footsteps.integer != 0 {
                trap_S_StartSound(
                    std::ptr::null_mut(),
                    (*es).number,
                    CHAN_BODY as i32,
                    cgs.media.footsteps[FOOTSTEP_METAL as i32 as usize]
                        [(libc::rand() & 3 as i32) as usize],
                );
            }
        }
        3 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_FOOTSPLASH\n\x00" as *const u8 as *const libc::c_char);
            }
            if cg_footsteps.integer != 0 {
                trap_S_StartSound(
                    std::ptr::null_mut(),
                    (*es).number,
                    CHAN_BODY as i32,
                    cgs.media.footsteps[FOOTSTEP_SPLASH as i32 as usize]
                        [(libc::rand() & 3 as i32) as usize],
                );
            }
        }
        4 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_FOOTWADE\n\x00" as *const u8 as *const libc::c_char);
            }
            if cg_footsteps.integer != 0 {
                trap_S_StartSound(
                    std::ptr::null_mut(),
                    (*es).number,
                    CHAN_BODY as i32,
                    cgs.media.footsteps[FOOTSTEP_SPLASH as i32 as usize]
                        [(libc::rand() & 3 as i32) as usize],
                );
            }
        }
        5 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_SWIM\n\x00" as *const u8 as *const libc::c_char);
            }
            if cg_footsteps.integer != 0 {
                trap_S_StartSound(
                    std::ptr::null_mut(),
                    (*es).number,
                    CHAN_BODY as i32,
                    cgs.media.footsteps[FOOTSTEP_SPLASH as i32 as usize]
                        [(libc::rand() & 3 as i32) as usize],
                );
            }
        }
        10 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_FALL_SHORT\n\x00" as *const u8 as *const libc::c_char);
            }
            trap_S_StartSound(
                std::ptr::null_mut(),
                (*es).number,
                CHAN_AUTO as i32,
                cgs.media.landSound,
            );
            if clientNum == cg.predictedPlayerState.clientNum {
                // smooth landing z changes
                cg.landChange = -(8 as i32) as f32;
                cg.landTime = cg.time
            }
        }
        11 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_FALL_MEDIUM\n\x00" as *const u8 as *const libc::c_char);
            }
            // use normal pain sound
            trap_S_StartSound(
                std::ptr::null_mut(),
                (*es).number,
                CHAN_VOICE as i32,
                CG_CustomSound(
                    (*es).number,
                    b"*pain100_1.wav\x00" as *const u8 as *const libc::c_char,
                ),
            );
            if clientNum == cg.predictedPlayerState.clientNum {
                // smooth landing z changes
                cg.landChange = -(16 as i32) as f32; // don't play a pain sound right after this
                cg.landTime = cg.time
            }
        }
        12 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_FALL_FAR\n\x00" as *const u8 as *const libc::c_char);
            }
            trap_S_StartSound(
                std::ptr::null_mut(),
                (*es).number,
                CHAN_AUTO as i32,
                CG_CustomSound(
                    (*es).number,
                    b"*fall1.wav\x00" as *const u8 as *const libc::c_char,
                ),
            );
            (*cent).pe.painTime = cg.time;
            if clientNum == cg.predictedPlayerState.clientNum {
                // smooth landing z changes
                cg.landChange = -(24 as i32) as f32;
                cg.landTime = cg.time
            }
        }
        6 | 7 | 8 | 9 => {
            // smooth out step up transitions
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_STEP\n\x00" as *const u8 as *const libc::c_char);
            }
            let mut oldStep: f32 = 0.;
            let mut delta: i32 = 0;
            let mut step: i32 = 0;
            if !(clientNum != cg.predictedPlayerState.clientNum) {
                // if we are interpolating, we don't need to smooth steps
                if !(cg.demoPlayback as u32 != 0
                    || (*cg.snap).ps.pm_flags & 4096 as i32 != 0
                    || cg_nopredict.integer != 0
                    || cg_synchronousClients.integer != 0)
                {
                    // check for stepping up before a previous step is completed
                    delta = cg.time - cg.stepTime;
                    if delta < 200 as i32 {
                        oldStep = cg.stepChange * (200 as i32 - delta) as f32 / 200 as i32 as f32
                    } else {
                        oldStep = 0 as i32 as f32
                    }
                    // add this amount
                    step = 4 as i32 * (event - EV_STEP_4 as i32 + 1 as i32);
                    cg.stepChange = oldStep + step as f32;
                    if cg.stepChange > 32 as i32 as f32 {
                        cg.stepChange = 32 as i32 as f32
                    }
                    cg.stepTime = cg.time
                }
            }
        }
        13 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_JUMP_PAD\n\x00" as *const u8 as *const libc::c_char);
            }
            //		CG_Printf( "EV_JUMP_PAD w/effect #%i\n", es->eventParm );
            let mut up: vec3_t = [0 as i32 as vec_t, 0 as i32 as vec_t, 1 as i32 as vec_t];

            CG_SmokePuff(
                (*cent).lerpOrigin.as_mut_ptr() as *const vec_t,
                up.as_mut_ptr() as *const vec_t,
                32 as i32 as f32,
                1 as i32 as f32,
                1 as i32 as f32,
                1 as i32 as f32,
                0.33f32,
                1000 as i32 as f32,
                cg.time,
                0 as i32,
                LEF_PUFF_DONT_SCALE as i32,
                cgs.media.smokePuffShader,
            ) as *mut localEntity_s;
            // boing sound at origin, jump sound on player
            trap_S_StartSound(
                (*cent).lerpOrigin.as_mut_ptr(),
                -(1 as i32),
                CHAN_VOICE as i32,
                cgs.media.jumpPadSound,
            ); // player predicted
            trap_S_StartSound(
                std::ptr::null_mut(),
                (*es).number,
                CHAN_VOICE as i32,
                CG_CustomSound(
                    (*es).number,
                    b"*jump1.wav\x00" as *const u8 as *const libc::c_char,
                ),
            );
        }
        14 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_JUMP\n\x00" as *const u8 as *const libc::c_char);
            }
            trap_S_StartSound(
                std::ptr::null_mut(),
                (*es).number,
                CHAN_VOICE as i32,
                CG_CustomSound(
                    (*es).number,
                    b"*jump1.wav\x00" as *const u8 as *const libc::c_char,
                ),
            );
        }
        76 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_TAUNT\n\x00" as *const u8 as *const libc::c_char);
            }
            trap_S_StartSound(
                std::ptr::null_mut(),
                (*es).number,
                CHAN_VOICE as i32,
                CG_CustomSound(
                    (*es).number,
                    b"*taunt.wav\x00" as *const u8 as *const libc::c_char,
                ),
            );
        }
        15 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_WATER_TOUCH\n\x00" as *const u8 as *const libc::c_char);
            }
            trap_S_StartSound(
                std::ptr::null_mut(),
                (*es).number,
                CHAN_AUTO as i32,
                cgs.media.watrInSound,
            );
        }
        16 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_WATER_LEAVE\n\x00" as *const u8 as *const libc::c_char);
            }
            trap_S_StartSound(
                std::ptr::null_mut(),
                (*es).number,
                CHAN_AUTO as i32,
                cgs.media.watrOutSound,
            );
        }
        17 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_WATER_UNDER\n\x00" as *const u8 as *const libc::c_char);
            }
            trap_S_StartSound(
                std::ptr::null_mut(),
                (*es).number,
                CHAN_AUTO as i32,
                cgs.media.watrUnSound,
            );
        }
        18 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_WATER_CLEAR\n\x00" as *const u8 as *const libc::c_char);
            }
            trap_S_StartSound(
                std::ptr::null_mut(),
                (*es).number,
                CHAN_AUTO as i32,
                CG_CustomSound(
                    (*es).number,
                    b"*gasp.wav\x00" as *const u8 as *const libc::c_char,
                ),
            );
        }
        19 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_ITEM_PICKUP\n\x00" as *const u8 as *const libc::c_char);
            }
            let mut item: *mut gitem_t = std::ptr::null_mut();
            let mut index: i32 = 0;
            index = (*es).eventParm;
            if !(index < 1 as i32 || index >= bg_numItems) {
                item = &mut *bg_itemlist.as_mut_ptr().offset(index as isize) as *mut gitem_t;
                // powerups and team items will have a separate global sound, this one
                // will be played at prediction time
                if (*item).giType as u32 == IT_POWERUP as i32 as u32
                    || (*item).giType as u32 == IT_TEAM as i32 as u32
                {
                    trap_S_StartSound(
                        std::ptr::null_mut(),
                        (*es).number,
                        CHAN_AUTO as i32,
                        cgs.media.n_healthSound,
                    );
                } else if !((*item).giType as u32 == IT_PERSISTANT_POWERUP as i32 as u32) {
                    trap_S_StartSound(
                        std::ptr::null_mut(),
                        (*es).number,
                        CHAN_AUTO as i32,
                        trap_S_RegisterSound((*item).pickup_sound, qfalse),
                    );
                }
                // show icon and name on status bar
                if (*es).number == (*cg.snap).ps.clientNum {
                    CG_ItemPickup(index); // player predicted
                }
            }
        }
        20 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_GLOBAL_ITEM_PICKUP\n\x00" as *const u8 as *const libc::c_char);
            }
            let mut item_0: *mut gitem_t = std::ptr::null_mut();
            let mut index_0: i32 = 0;
            index_0 = (*es).eventParm;
            if !(index_0 < 1 as i32 || index_0 >= bg_numItems) {
                item_0 = &mut *bg_itemlist.as_mut_ptr().offset(index_0 as isize) as *mut gitem_t;
                // powerup pickups are global
                if !(*item_0).pickup_sound.is_null() {
                    trap_S_StartSound(
                        std::ptr::null_mut(),
                        (*cg.snap).ps.clientNum,
                        CHAN_AUTO as i32,
                        trap_S_RegisterSound((*item_0).pickup_sound, qfalse),
                    );
                }
                // show icon and name on status bar
                if (*es).number == (*cg.snap).ps.clientNum {
                    CG_ItemPickup(index_0);
                }
            }
        }
        21 => {
            //
            // weapon events
            //
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_NOAMMO\n\x00" as *const u8 as *const libc::c_char);
            }
            //		trap_S_StartSound (NULL, es->number, CHAN_AUTO, cgs.media.noAmmoSound );
            if (*es).number == (*cg.snap).ps.clientNum {
                CG_OutOfAmmoChange();
            }
        }
        22 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_CHANGE_WEAPON\n\x00" as *const u8 as *const libc::c_char);
            }
            trap_S_StartSound(
                std::ptr::null_mut(),
                (*es).number,
                CHAN_AUTO as i32,
                cgs.media.selectSound,
            );
        }
        23 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_FIRE_WEAPON\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_FireWeapon(cent as *mut centity_s);
        }
        24 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_USE_ITEM0\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_UseItem(cent);
        }
        25 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_USE_ITEM1\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_UseItem(cent);
        }
        26 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_USE_ITEM2\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_UseItem(cent);
        }
        27 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_USE_ITEM3\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_UseItem(cent);
        }
        28 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_USE_ITEM4\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_UseItem(cent);
        }
        29 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_USE_ITEM5\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_UseItem(cent);
        }
        30 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_USE_ITEM6\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_UseItem(cent);
        }
        31 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_USE_ITEM7\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_UseItem(cent);
        }
        32 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_USE_ITEM8\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_UseItem(cent);
        }
        33 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_USE_ITEM9\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_UseItem(cent);
        }
        34 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_USE_ITEM10\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_UseItem(cent);
        }
        35 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_USE_ITEM11\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_UseItem(cent);
        }
        36 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_USE_ITEM12\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_UseItem(cent);
        }
        37 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_USE_ITEM13\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_UseItem(cent);
        }
        38 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_USE_ITEM14\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_UseItem(cent);
        }
        39 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_USE_ITEM15\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_UseItem(cent);
        }
        42 => {
            //=================================================================
            //
            // other events
            //
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_PLAYER_TELEPORT_IN\n\x00" as *const u8 as *const libc::c_char);
                // scale up from this
            }
            trap_S_StartSound(
                std::ptr::null_mut(),
                (*es).number,
                CHAN_AUTO as i32,
                cgs.media.teleInSound,
            );
            CG_SpawnEffect(position);
        }
        43 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_PLAYER_TELEPORT_OUT\n\x00" as *const u8 as *const libc::c_char);
            }
            trap_S_StartSound(
                std::ptr::null_mut(),
                (*es).number,
                CHAN_AUTO as i32,
                cgs.media.teleOutSound,
            );
            CG_SpawnEffect(position);
        }
        41 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_ITEM_POP\n\x00" as *const u8 as *const libc::c_char);
            }
            trap_S_StartSound(
                std::ptr::null_mut(),
                (*es).number,
                CHAN_AUTO as i32,
                cgs.media.respawnSound,
            );
        }
        40 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_ITEM_RESPAWN\n\x00" as *const u8 as *const libc::c_char);
            }
            (*cent).miscTime = cg.time;
            trap_S_StartSound(
                std::ptr::null_mut(),
                (*es).number,
                CHAN_AUTO as i32,
                cgs.media.respawnSound,
            );
        }
        44 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_GRENADE_BOUNCE\n\x00" as *const u8 as *const libc::c_char);
            }
            if libc::rand() & 1 as i32 != 0 {
                trap_S_StartSound(
                    std::ptr::null_mut(),
                    (*es).number,
                    CHAN_AUTO as i32,
                    cgs.media.hgrenb1aSound,
                );
            } else {
                trap_S_StartSound(
                    std::ptr::null_mut(),
                    (*es).number,
                    CHAN_AUTO as i32,
                    cgs.media.hgrenb2aSound,
                );
            }
        }
        65 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_SCOREPLUM\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_ScorePlum(
                (*cent).currentState.otherEntityNum,
                (*cent).lerpOrigin.as_mut_ptr(),
                (*cent).currentState.time,
            );
        }
        50 => {
            //
            // missile impacts
            //
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_MISSILE_HIT\n\x00" as *const u8 as *const libc::c_char);
            }
            ByteToDir((*es).eventParm, dir.as_mut_ptr());
            CG_MissileHitPlayer(
                (*es).weapon,
                position,
                dir.as_mut_ptr(),
                (*es).otherEntityNum,
            );
        }
        51 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_MISSILE_MISS\n\x00" as *const u8 as *const libc::c_char);
            }
            ByteToDir((*es).eventParm, dir.as_mut_ptr());
            CG_MissileHitWall(
                (*es).weapon,
                0 as i32,
                position,
                dir.as_mut_ptr(),
                IMPACTSOUND_DEFAULT,
            );
        }
        52 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_MISSILE_MISS_METAL\n\x00" as *const u8 as *const libc::c_char);
            }
            ByteToDir((*es).eventParm, dir.as_mut_ptr());
            CG_MissileHitWall(
                (*es).weapon,
                0 as i32,
                position,
                dir.as_mut_ptr(),
                IMPACTSOUND_METAL,
            );
        }
        53 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_RAILTRAIL\n\x00" as *const u8 as *const libc::c_char);
            }
            (*cent).currentState.weapon = WP_RAILGUN as i32;
            if (*es).clientNum == (*cg.snap).ps.clientNum && cg.renderingThirdPerson as u64 == 0 {
                if cg_drawGun.integer == 2 as i32 {
                    (*es).origin2[0 as i32 as usize] = (*es).origin2[0 as i32 as usize]
                        + cg.refdef.viewaxis[1 as i32 as usize][0 as i32 as usize]
                            * 8 as i32 as f32;
                    (*es).origin2[1 as i32 as usize] = (*es).origin2[1 as i32 as usize]
                        + cg.refdef.viewaxis[1 as i32 as usize][1 as i32 as usize]
                            * 8 as i32 as f32;
                    (*es).origin2[2 as i32 as usize] = (*es).origin2[2 as i32 as usize]
                        + cg.refdef.viewaxis[1 as i32 as usize][2 as i32 as usize] * 8 as i32 as f32
                } else if cg_drawGun.integer == 3 as i32 {
                    (*es).origin2[0 as i32 as usize] = (*es).origin2[0 as i32 as usize]
                        + cg.refdef.viewaxis[1 as i32 as usize][0 as i32 as usize]
                            * 4 as i32 as f32;
                    (*es).origin2[1 as i32 as usize] = (*es).origin2[1 as i32 as usize]
                        + cg.refdef.viewaxis[1 as i32 as usize][1 as i32 as usize]
                            * 4 as i32 as f32;
                    (*es).origin2[2 as i32 as usize] = (*es).origin2[2 as i32 as usize]
                        + cg.refdef.viewaxis[1 as i32 as usize][2 as i32 as usize] * 4 as i32 as f32
                }
            }
            CG_RailTrail(
                ci as *mut clientInfo_t,
                (*es).origin2.as_mut_ptr(),
                (*es).pos.trBase.as_mut_ptr(),
            );
            // if the end was on a nomark surface, don't make an explosion
            if (*es).eventParm != 255 as i32 {
                ByteToDir((*es).eventParm, dir.as_mut_ptr());
                CG_MissileHitWall(
                    (*es).weapon,
                    (*es).clientNum,
                    position,
                    dir.as_mut_ptr(),
                    IMPACTSOUND_DEFAULT,
                );
            }
        }
        49 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_BULLET_HIT_WALL\n\x00" as *const u8 as *const libc::c_char);
            }
            ByteToDir((*es).eventParm, dir.as_mut_ptr());
            CG_Bullet(
                (*es).pos.trBase.as_mut_ptr(),
                (*es).otherEntityNum,
                dir.as_mut_ptr(),
                qfalse,
                ((1 as i32) << 10 as i32) - 2 as i32,
            );
        }
        48 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_BULLET_HIT_FLESH\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_Bullet(
                (*es).pos.trBase.as_mut_ptr(),
                (*es).otherEntityNum,
                dir.as_mut_ptr(),
                qtrue,
                (*es).eventParm,
            );
        }
        54 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_SHOTGUN\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_ShotgunFire(es as *mut entityState_s);
        }
        45 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_GENERAL_SOUND\n\x00" as *const u8 as *const libc::c_char);
            }
            if cgs.gameSounds[(*es).eventParm as usize] != 0 {
                trap_S_StartSound(
                    std::ptr::null_mut(),
                    (*es).number,
                    CHAN_VOICE as i32,
                    cgs.gameSounds[(*es).eventParm as usize],
                );
            } else {
                s = CG_ConfigString(32 as i32 + 256 as i32 + (*es).eventParm);
                trap_S_StartSound(
                    std::ptr::null_mut(),
                    (*es).number,
                    CHAN_VOICE as i32,
                    CG_CustomSound((*es).number, s),
                );
            }
        }
        46 => {
            // play from the player's head so it never diminishes
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_GLOBAL_SOUND\n\x00" as *const u8 as *const libc::c_char);
            }
            if cgs.gameSounds[(*es).eventParm as usize] != 0 {
                trap_S_StartSound(
                    std::ptr::null_mut(),
                    (*cg.snap).ps.clientNum,
                    CHAN_AUTO as i32,
                    cgs.gameSounds[(*es).eventParm as usize],
                );
            } else {
                s = CG_ConfigString(32 as i32 + 256 as i32 + (*es).eventParm);
                trap_S_StartSound(
                    std::ptr::null_mut(),
                    (*cg.snap).ps.clientNum,
                    CHAN_AUTO as i32,
                    CG_CustomSound((*es).number, s),
                );
            }
        }
        47 => {
            // play from the player's head so it never diminishes
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_GLOBAL_TEAM_SOUND\n\x00" as *const u8 as *const libc::c_char);
            }
            match (*es).eventParm {
                0 => {
                    // CTF: red team captured the blue flag, 1FCTF: red team captured the neutral flag
                    if (*cg.snap).ps.persistant[PERS_TEAM as i32 as usize] == TEAM_RED as i32 {
                        CG_AddBufferedSound(cgs.media.captureYourTeamSound);
                    } else {
                        CG_AddBufferedSound(cgs.media.captureOpponentSound);
                    }
                }
                1 => {
                    // CTF: blue team captured the red flag, 1FCTF: blue team captured the neutral flag
                    if (*cg.snap).ps.persistant[PERS_TEAM as i32 as usize] == TEAM_BLUE as i32 {
                        CG_AddBufferedSound(cgs.media.captureYourTeamSound);
                    } else {
                        CG_AddBufferedSound(cgs.media.captureOpponentSound);
                    }
                }
                2 => {
                    // CTF: blue flag returned, 1FCTF: never used
                    if (*cg.snap).ps.persistant[PERS_TEAM as i32 as usize] == TEAM_RED as i32 {
                        CG_AddBufferedSound(cgs.media.returnYourTeamSound);
                    } else {
                        CG_AddBufferedSound(cgs.media.returnOpponentSound);
                    }
                    //
                    CG_AddBufferedSound(cgs.media.blueFlagReturnedSound);
                }
                3 => {
                    // CTF red flag returned, 1FCTF: neutral flag returned
                    if (*cg.snap).ps.persistant[PERS_TEAM as i32 as usize] == TEAM_BLUE as i32 {
                        CG_AddBufferedSound(cgs.media.returnYourTeamSound);
                    } else {
                        CG_AddBufferedSound(cgs.media.returnOpponentSound);
                    }
                    //
                    CG_AddBufferedSound(cgs.media.redFlagReturnedSound);
                }
                4 => {
                    // CTF: red team took blue flag, 1FCTF: blue team took the neutral flag
                    // if this player picked up the flag then a sound is played in CG_CheckLocalSounds
                    if !((*cg.snap).ps.powerups[PW_BLUEFLAG as i32 as usize] != 0
                        || (*cg.snap).ps.powerups[PW_NEUTRALFLAG as i32 as usize] != 0)
                    {
                        if (*cg.snap).ps.persistant[PERS_TEAM as i32 as usize] == TEAM_BLUE as i32 {
                            CG_AddBufferedSound(cgs.media.enemyTookYourFlagSound);
                        } else if (*cg.snap).ps.persistant[PERS_TEAM as i32 as usize]
                            == TEAM_RED as i32
                        {
                            CG_AddBufferedSound(cgs.media.yourTeamTookEnemyFlagSound);
                        }
                    }
                }
                5 => {
                    // CTF: blue team took the red flag, 1FCTF red team took the neutral flag
                    // if this player picked up the flag then a sound is played in CG_CheckLocalSounds
                    if !((*cg.snap).ps.powerups[PW_REDFLAG as i32 as usize] != 0
                        || (*cg.snap).ps.powerups[PW_NEUTRALFLAG as i32 as usize] != 0)
                    {
                        if (*cg.snap).ps.persistant[PERS_TEAM as i32 as usize] == TEAM_RED as i32 {
                            CG_AddBufferedSound(cgs.media.enemyTookYourFlagSound);
                        } else if (*cg.snap).ps.persistant[PERS_TEAM as i32 as usize]
                            == TEAM_BLUE as i32
                        {
                            CG_AddBufferedSound(cgs.media.yourTeamTookEnemyFlagSound);
                        }
                    }
                }
                8 => {
                    CG_AddBufferedSound(cgs.media.redScoredSound);
                }
                9 => {
                    CG_AddBufferedSound(cgs.media.blueScoredSound);
                }
                10 => {
                    CG_AddBufferedSound(cgs.media.redLeadsSound);
                }
                11 => {
                    CG_AddBufferedSound(cgs.media.blueLeadsSound);
                }
                12 => {
                    CG_AddBufferedSound(cgs.media.teamsTiedSound);
                }
                _ => {}
            }
        }
        56 => {
            // local player sounds are triggered in CG_CheckLocalSounds,
            // so ignore events on the player
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_PAIN\n\x00" as *const u8 as *const libc::c_char);
            }
            if (*cent).currentState.number != (*cg.snap).ps.clientNum {
                CG_PainEvent(cent, (*es).eventParm);
            }
        }
        57 | 58 | 59 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_DEATHx\n\x00" as *const u8 as *const libc::c_char);
            }
            if CG_WaterLevel(cent) == 3 as i32 {
                trap_S_StartSound(
                    std::ptr::null_mut(),
                    (*es).number,
                    CHAN_VOICE as i32,
                    CG_CustomSound(
                        (*es).number,
                        b"*drown.wav\x00" as *const u8 as *const libc::c_char,
                    ),
                );
            } else {
                trap_S_StartSound(
                    std::ptr::null_mut(),
                    (*es).number,
                    CHAN_VOICE as i32,
                    CG_CustomSound(
                        (*es).number,
                        va(
                            b"*death%i.wav\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            event - EV_DEATH1 as i32 + 1 as i32,
                        ),
                    ),
                );
            }
        }
        60 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_OBITUARY\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_Obituary(es);
        }
        61 => {
            //
            // powerup events
            //
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_POWERUP_QUAD\n\x00" as *const u8 as *const libc::c_char);
            }
            if (*es).number == (*cg.snap).ps.clientNum {
                cg.powerupActive = PW_QUAD as i32;
                cg.powerupTime = cg.time
            }
            trap_S_StartSound(
                std::ptr::null_mut(),
                (*es).number,
                CHAN_ITEM as i32,
                cgs.media.quadSound,
            );
        }
        62 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_POWERUP_BATTLESUIT\n\x00" as *const u8 as *const libc::c_char);
            }
            if (*es).number == (*cg.snap).ps.clientNum {
                cg.powerupActive = PW_BATTLESUIT as i32;
                cg.powerupTime = cg.time
            }
            trap_S_StartSound(
                std::ptr::null_mut(),
                (*es).number,
                CHAN_ITEM as i32,
                cgs.media.protectSound,
            );
        }
        63 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_POWERUP_REGEN\n\x00" as *const u8 as *const libc::c_char);
            }
            if (*es).number == (*cg.snap).ps.clientNum {
                cg.powerupActive = PW_REGEN as i32;
                cg.powerupTime = cg.time
            }
            trap_S_StartSound(
                std::ptr::null_mut(),
                (*es).number,
                CHAN_ITEM as i32,
                cgs.media.regenSound,
            );
        }
        64 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_GIB_PLAYER\n\x00" as *const u8 as *const libc::c_char);
            }
            // don't play gib sound when using the kamikaze because it interferes
            // with the kamikaze sound, downside is that the gib sound will also
            // not be played when someone is gibbed while just carrying the kamikaze
            if (*es).eFlags & 0x200 as i32 == 0 {
                trap_S_StartSound(
                    std::ptr::null_mut(),
                    (*es).number,
                    CHAN_BODY as i32,
                    cgs.media.gibSound,
                );
            }
            CG_GibPlayer((*cent).lerpOrigin.as_mut_ptr());
        }
        75 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_STOPLOOPINGSOUND\n\x00" as *const u8 as *const libc::c_char);
            }
            trap_S_StopLoopingSound((*es).number);
            (*es).loopSound = 0 as i32
        }
        74 => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"EV_DEBUG_LINE\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_Beam(cent as *mut centity_s);
        }
        _ => {
            if cg_debugEvents.integer != 0 {
                CG_Printf(b"UNKNOWN\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_Error(
                b"Unknown event: %i\x00" as *const u8 as *const libc::c_char,
                event,
            );
        }
    };
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
// The entire cgame module is unloaded and reloaded on each level change,
// so there is NO persistant data between levels on the client side.
// If you absolutely need something stored, it can either be kept
// by the server in the server stored userinfos, or stashed in a cvar.
// time for fragments to sink into ground before going away
// amount to scale up the icons when activating
// num frame for '-' stats digit
// very large characters
//=================================================
// player entities need to track more information
// than any other type of entity.
// note that not every player entity is a client entity,
// because corpses after respawn are outside the normal
// client numbering range
// when changing animation, set animationTime to frameTime + lerping time
// The current lerp will finish out, then it will lerp to the new animation
// time when ->oldFrame was exactly on
// time when ->frame will be exactly on
// may include ANIM_TOGGLEBIT
// time when the first frame of the animation will be exact
// flip from 0 to 1
// machinegun spinning
//=================================================
// centity_t have a direct corespondence with gentity_t in the game, but
// only the entityState_t is directly communicated to the cgame
// from cg.frame
// from cg.nextFrame, if available
// true if next is valid to interpolate to
// true if cg.frame holds this entity
// move to playerEntity?
// so missile trails can handle dropped initial packets
// last time this entity was found in a snapshot
// decay the error from this time
// false if origin / angles is an interpolation
// exact interpolated position of entity on this frame
//======================================================================
// local entities are created as a result of events or predicted actions,
// and live independently from all server transmitted entities
// fade alpha instead of rgb
// do not scale size over time
// tumble over time, used for ejecting shells
// sound 1 for kamikaze
// sound 2 for kamikaze
// fragment local entities can leave marks on walls
// fragment local entities can make sounds on impacts
// 1.0 / (endTime - startTime)
// 0.0 = no bounce, 1.0 = perfect
// mark to leave on fragment impact
//======================================================================
// each client has an associated clientInfo_t
// that contains media references necessary to present the
// client model and other color coded effects
// this is regenerated each time a client's configstring changes,
// usually as a result of a userinfo (name, model, etc) change
// 0 = not bot, 1-5 = bot
// updated by score servercmds
// location index for team mode
// you only get this info about your teammates
// in tourney mode
// task in teamplay (offence/defence)
// true when this is a team leader
// so can display quad/flag status
// when clientinfo is changed, the loading of models/skins/sounds
// can be deferred until you are dead, to prevent hitches in
// gameplay
// true if using the new mission pack animations
// true if legs yaw is always the same as torso yaw
// true if torso never changes yaw
// move head in icon views
// from model
// each WP_* weapon enum has an associated weaponInfo_t
// that contains media references necessary to present the
// weapon and its effects
// the hands don't actually draw, they just position the weapon
// so it will rotate centered instead of by tag
// fast firing weapons randomly choose
// each IT_* item has an associated itemInfo_t
// that constains media references necessary to present the
// item and its effects
//======================================================================
// all cg.stepTime, cg.duckTime, cg.landTime, etc are set to cg.time when the action
// occurs, and they will have visible effects for #define STEP_TIME or whatever msec after
// incremented each frame
// taking a level menu screenshot
// don't defer players at initial startup
// don't play voice rewards, because game will end shortly
// there are only one or two snapshot_t that are relevant at a time
// the number of snapshots the client system has received
// the time from latestSnapshotNum, so we don't need to read the snapshot yet
// cg.snap->serverTime <= cg.time
// cg.nextSnap->serverTime > cg.time, or NULL
// (float)( cg.time - cg.frame->serverTime ) / (cg.nextFrame->serverTime - cg.frame->serverTime)
// cg.time - cg.oldTime
// this is the time value that the client
// is rendering at.
// time at last frame, used for missile trails and prediction checking
// either cg.snap->time or cg.nextSnap->time
// 5 min, 1 min, overtime
// set on a map restart to set back the weapon
// during deaths, chasecams, etc
// prediction state
// true if prediction has hit a trigger_teleport
// clear until the first call to CG_PredictPlayerState
// for stair up smoothing
// for duck viewheight smoothing
// for landing hard
// input state sent to server
// auto rotating items
// view rendering
// will be converted to refdef.viewaxis
// zoom key
// information screen text during loading
// scoreboard
// list of names
// length of list
// width in device units
// next time to offset
// current paint x
// current paint x
// current offset from start
// current offset from start
// centerprinting
// low ammo warning state
// 1 = low, 2 = empty
// crosshair client ID
// powerup active flashing
// attacking player
// reward medals
// sound buffer mainly for announcer sounds
// warmup countdown
//==========================
// the pulse around the crosshair is timed separately
// blend blobs
// status bar head
// view movement
// temp working variables for player view
//qboolean cameraMode;		// if rendering from a loaded camera
// development tool
// all of the model, shader, and sound references that are
// loaded at gamestate time are stored in cgMedia_t
// Other media that can be tied to clients, weapons, or items are
// stored in the clientInfo_t, itemInfo_t, weaponInfo_t, and powerupInfo_t
// gib explosions
// wall mark shaders
// powerup shaders
// weapon effect models
// weapon effect shaders
// special effects models
// scoreboard headers
// medals shown during gameplay
// sounds
//sfxHandle_t	sfx_railg;
// teamplay sounds
// tournament sounds
// The client game static (cgs) structure hold everything
// loaded or calculated from the gamestate.  It will NOT
// be cleared when a tournement restart is done, allowing
// all clients to begin playing instantly
// gamestate from server
// rendering configuration
// derived from glconfig
// reliable command stream counter
// the number of snapshots cgame has requested
// detected on startup by checking sv_running
// parsed from serverinfo
// beep whenever changed
// beep whenever changed
// from configstrings
// flag status from configstrings
//
// locally derived information from gamestate
//
// teamchat width is *3 because of embedded color codes
// orders
// media
//==============================================================================
//extern	vmCvar_t		cg_pmove_fixed;
//
// cg_main.c
//
//
// cg_view.c
//
//
// cg_drawtools.c
//
//
// cg_draw.c, cg_newDraw.c
//
//
// cg_player.c
//
//
// cg_predict.c
//
//
// cg_events.c
//
/*
==============
CG_CheckEvents

==============
*/
#[no_mangle]

pub unsafe extern "C" fn CG_CheckEvents(mut cent: *mut centity_t) {
    // check for event-only entities
    if (*cent).currentState.eType > ET_EVENTS as i32 {
        if (*cent).previousEvent != 0 {
            return;
            // already fired
        }
        // if this is a player event set the entity number of the client entity number
        if (*cent).currentState.eFlags & 0x10 as i32 != 0 {
            (*cent).currentState.number = (*cent).currentState.otherEntityNum
        }
        (*cent).previousEvent = 1 as i32;
        (*cent).currentState.event = (*cent).currentState.eType - ET_EVENTS as i32
    } else {
        // check for events riding with another entity
        if (*cent).currentState.event == (*cent).previousEvent {
            return;
        }
        (*cent).previousEvent = (*cent).currentState.event;
        if (*cent).currentState.event & !(0x100 as i32 | 0x200 as i32) == 0 as i32 {
            return;
        }
    }
    // calculate the position at exactly the frame time
    BG_EvaluateTrajectory(
        &mut (*cent).currentState.pos as *mut _ as *const trajectory_t,
        (*cg.snap).serverTime,
        (*cent).lerpOrigin.as_mut_ptr(),
    );
    CG_SetEntitySoundPosition(cent as *mut centity_s);
    CG_EntityEvent(cent, (*cent).lerpOrigin.as_mut_ptr());
}
