pub const GT_FFA: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 0;
pub const GT_TOURNAMENT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 1;
pub const GT_SINGLE_PLAYER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 2;
pub const GT_TEAM: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 3;
pub const GT_CTF: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 4;
pub const GT_1FCTF: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 5;
pub const GT_OBELISK: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 6;
pub const GT_HARVESTER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 7;
pub const GT_MAX_GAME_TYPE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 8;
pub const PM_NORMAL: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 0;
pub const PM_NOCLIP: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 1;
pub const PM_SPECTATOR: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 2;
pub const PM_DEAD: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 3;
pub const PM_FREEZE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 4;
pub const PM_INTERMISSION: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 5;
pub const PM_SPINTERMISSION: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 6;
pub const STAT_HEALTH: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 0;
pub const STAT_HOLDABLE_ITEM: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 1;
pub const STAT_WEAPONS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 2;
pub const STAT_ARMOR: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 3;
pub const STAT_DEAD_YAW: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 4;
pub const STAT_CLIENTS_READY: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 5;
pub const STAT_MAX_HEALTH: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 6;
pub const PERS_SCORE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 0;
pub const PERS_HITS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 1;
pub const PERS_RANK: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 2;
pub const PERS_TEAM: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 3;
pub const PERS_SPAWN_COUNT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 4;
pub const PERS_PLAYEREVENTS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 5;
pub const PERS_ATTACKER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 6;
pub const PERS_ATTACKEE_ARMOR: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 7;
pub const PERS_KILLED: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 8;
pub const PERS_IMPRESSIVE_COUNT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 9;
pub const PERS_EXCELLENT_COUNT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 10;
pub const PERS_DEFEND_COUNT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 11;
pub const PERS_ASSIST_COUNT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 12;
pub const PERS_GAUNTLET_FRAG_COUNT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 13;
pub const PERS_CAPTURES: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 14;
pub type powerup_t = u32;
pub const PW_NONE: powerup_t = 0;
pub const PW_QUAD: powerup_t = 1;
pub const PW_BATTLESUIT: powerup_t = 2;
pub const PW_HASTE: powerup_t = 3;
pub const PW_INVIS: powerup_t = 4;
pub const PW_REGEN: powerup_t = 5;
pub const PW_FLIGHT: powerup_t = 6;
pub const PW_REDFLAG: powerup_t = 7;
pub const PW_BLUEFLAG: powerup_t = 8;
pub const PW_NEUTRALFLAG: powerup_t = 9;
pub const PW_SCOUT: powerup_t = 10;
pub const PW_GUARD: powerup_t = 11;
pub const PW_DOUBLER: powerup_t = 12;
pub const PW_AMMOREGEN: powerup_t = 13;
pub const PW_INVULNERABILITY: powerup_t = 14;
pub const PW_NUM_POWERUPS: powerup_t = 15;
pub type holdable_t = u32;
pub const HI_NONE: holdable_t = 0;
pub const HI_TELEPORTER: holdable_t = 1;
pub const HI_MEDKIT: holdable_t = 2;
pub const HI_KAMIKAZE: holdable_t = 3;
pub const HI_PORTAL: holdable_t = 4;
pub const HI_INVULNERABILITY: holdable_t = 5;
pub const HI_NUM_HOLDABLE: holdable_t = 6;
pub type weapon_t = u32;
pub const WP_NONE: weapon_t = 0;
pub const WP_GAUNTLET: weapon_t = 1;
pub const WP_MACHINEGUN: weapon_t = 2;
pub const WP_SHOTGUN: weapon_t = 3;
pub const WP_GRENADE_LAUNCHER: weapon_t = 4;
pub const WP_ROCKET_LAUNCHER: weapon_t = 5;
pub const WP_LIGHTNING: weapon_t = 6;
pub const WP_RAILGUN: weapon_t = 7;
pub const WP_PLASMAGUN: weapon_t = 8;
pub const WP_BFG: weapon_t = 9;
pub const WP_GRAPPLING_HOOK: weapon_t = 10;
pub const WP_NUM_WEAPONS: weapon_t = 11;
pub const EV_NONE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 0;
pub const EV_FOOTSTEP: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 1;
pub const EV_FOOTSTEP_METAL: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 2;
pub const EV_FOOTSPLASH: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 3;
pub const EV_FOOTWADE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 4;
pub const EV_SWIM: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 5;
pub const EV_STEP_4: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 6;
pub const EV_STEP_8: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 7;
pub const EV_STEP_12: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 8;
pub const EV_STEP_16: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 9;
pub const EV_FALL_SHORT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 10;
pub const EV_FALL_MEDIUM: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 11;
pub const EV_FALL_FAR: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 12;
pub const EV_JUMP_PAD: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 13;
pub const EV_JUMP: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 14;
pub const EV_WATER_TOUCH: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 15;
pub const EV_WATER_LEAVE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 16;
pub const EV_WATER_UNDER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 17;
pub const EV_WATER_CLEAR: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 18;
pub const EV_ITEM_PICKUP: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 19;
pub const EV_GLOBAL_ITEM_PICKUP: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 20;
pub const EV_NOAMMO: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 21;
pub const EV_CHANGE_WEAPON: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 22;
pub const EV_FIRE_WEAPON: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 23;
pub const EV_USE_ITEM0: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 24;
pub const EV_USE_ITEM1: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 25;
pub const EV_USE_ITEM2: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 26;
pub const EV_USE_ITEM3: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 27;
pub const EV_USE_ITEM4: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 28;
pub const EV_USE_ITEM5: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 29;
pub const EV_USE_ITEM6: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 30;
pub const EV_USE_ITEM7: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 31;
pub const EV_USE_ITEM8: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 32;
pub const EV_USE_ITEM9: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 33;
pub const EV_USE_ITEM10: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 34;
pub const EV_USE_ITEM11: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 35;
pub const EV_USE_ITEM12: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 36;
pub const EV_USE_ITEM13: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 37;
pub const EV_USE_ITEM14: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 38;
pub const EV_USE_ITEM15: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 39;
pub const EV_ITEM_RESPAWN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 40;
pub const EV_ITEM_POP: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 41;
pub const EV_PLAYER_TELEPORT_IN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 42;
pub const EV_PLAYER_TELEPORT_OUT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 43;
pub const EV_GRENADE_BOUNCE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 44;
pub const EV_GENERAL_SOUND: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 45;
pub const EV_GLOBAL_SOUND: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 46;
pub const EV_GLOBAL_TEAM_SOUND: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 47;
pub const EV_BULLET_HIT_FLESH: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 48;
pub const EV_BULLET_HIT_WALL: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 49;
pub const EV_MISSILE_HIT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 50;
pub const EV_MISSILE_MISS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 51;
pub const EV_MISSILE_MISS_METAL: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 52;
pub const EV_RAILTRAIL: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 53;
pub const EV_SHOTGUN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 54;
pub const EV_BULLET: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 55;
pub const EV_PAIN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 56;
pub const EV_DEATH1: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 57;
pub const EV_DEATH2: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 58;
pub const EV_DEATH3: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 59;
pub const EV_OBITUARY: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 60;
pub const EV_POWERUP_QUAD: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 61;
pub const EV_POWERUP_BATTLESUIT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 62;
pub const EV_POWERUP_REGEN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 63;
pub const EV_GIB_PLAYER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 64;
pub const EV_SCOREPLUM: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 65;
pub const EV_PROXIMITY_MINE_STICK: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 66;
pub const EV_PROXIMITY_MINE_TRIGGER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 67;
pub const EV_KAMIKAZE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 68;
pub const EV_OBELISKEXPLODE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 69;
pub const EV_OBELISKPAIN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 70;
pub const EV_INVUL_IMPACT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 71;
pub const EV_JUICED: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 72;
pub const EV_LIGHTNINGBOLT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 73;
pub const EV_DEBUG_LINE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 74;
pub const EV_STOPLOOPINGSOUND: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 75;
pub const EV_TAUNT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 76;
pub const EV_TAUNT_YES: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 77;
pub const EV_TAUNT_NO: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 78;
pub const EV_TAUNT_FOLLOWME: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 79;
pub const EV_TAUNT_GETFLAG: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 80;
pub const EV_TAUNT_GUARDBASE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 81;
pub const EV_TAUNT_PATROL: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 82;
pub const BOTH_DEATH1: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 0;
pub const BOTH_DEAD1: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 1;
pub const BOTH_DEATH2: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 2;
pub const BOTH_DEAD2: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 3;
pub const BOTH_DEATH3: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 4;
pub const BOTH_DEAD3: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 5;
pub const TORSO_GESTURE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 6;
pub const TORSO_ATTACK: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 7;
pub const TORSO_ATTACK2: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 8;
pub const TORSO_DROP: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 9;
pub const TORSO_RAISE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 10;
pub const TORSO_STAND: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 11;
pub const TORSO_STAND2: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 12;
pub const LEGS_WALKCR: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 13;
pub const LEGS_WALK: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 14;
pub const LEGS_RUN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 15;
pub const LEGS_BACK: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 16;
pub const LEGS_SWIM: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 17;
pub const LEGS_JUMP: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 18;
pub const LEGS_LAND: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 19;
pub const LEGS_JUMPB: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 20;
pub const LEGS_LANDB: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 21;
pub const LEGS_IDLE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 22;
pub const LEGS_IDLECR: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 23;
pub const LEGS_TURN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 24;
pub const TORSO_GETFLAG: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 25;
pub const TORSO_GUARDBASE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 26;
pub const TORSO_PATROL: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 27;
pub const TORSO_FOLLOWME: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 28;
pub const TORSO_AFFIRMATIVE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 29;
pub const TORSO_NEGATIVE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 30;
pub const MAX_ANIMATIONS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 31;
pub const LEGS_BACKCR: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 32;
pub const LEGS_BACKWALK: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 33;
pub const FLAG_RUN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 34;
pub const FLAG_STAND: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 35;
pub const FLAG_STAND2RUN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 36;
pub const MAX_TOTALANIMATIONS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 37;
pub type animation_t = animation_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct animation_s {
    pub firstFrame: i32,
    pub numFrames: i32,
    pub loopFrames: i32,
    pub frameLerp: i32,
    pub initialLerp: i32,
    pub reversed: i32,
    pub flipflop: i32,
}
pub const TEAM_FREE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 0;
pub const TEAM_RED: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 1;
pub const TEAM_BLUE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 2;
pub const TEAM_SPECTATOR: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 3;
pub const TEAM_NUM_TEAMS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 4;
pub type itemType_t = u32;
pub const IT_BAD: itemType_t = 0;
pub const IT_WEAPON: itemType_t = 1;
pub const IT_AMMO: itemType_t = 2;
pub const IT_ARMOR: itemType_t = 3;
pub const IT_HEALTH: itemType_t = 4;
pub const IT_POWERUP: itemType_t = 5;
pub const IT_HOLDABLE: itemType_t = 6;
pub const IT_PERSISTANT_POWERUP: itemType_t = 7;
pub const IT_TEAM: itemType_t = 8;
pub type gitem_t = gitem_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gitem_s {
    pub classname: *mut libc::c_char,
    pub pickup_sound: *mut libc::c_char,
    pub world_model: [*mut libc::c_char; 4],
    pub icon: *mut libc::c_char,
    pub pickup_name: *mut libc::c_char,
    pub quantity: i32,
    pub giType: itemType_t,
    pub giTag: i32,
    pub precaches: *mut libc::c_char,
    pub sounds: *mut libc::c_char,
}
pub const ET_GENERAL: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 0;
pub const ET_PLAYER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 1;
pub const ET_ITEM: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 2;
pub const ET_MISSILE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 3;
pub const ET_MOVER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 4;
pub const ET_BEAM: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 5;
pub const ET_PORTAL: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 6;
pub const ET_SPEAKER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 7;
pub const ET_PUSH_TRIGGER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 8;
pub const ET_TELEPORT_TRIGGER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 9;
pub const ET_INVISIBLE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 10;
pub const ET_GRAPPLE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 11;
pub const ET_TEAM: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 12;
pub const ET_EVENTS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 13;
