pub type C2RustUnnamed_0 = u32;
pub const GT_FFA: C2RustUnnamed_0 = 0;
pub const GT_TOURNAMENT: C2RustUnnamed_0 = 1;
pub const GT_SINGLE_PLAYER: C2RustUnnamed_0 = 2;
pub const GT_TEAM: C2RustUnnamed_0 = 3;
pub const GT_CTF: C2RustUnnamed_0 = 4;
pub const GT_1FCTF: C2RustUnnamed_0 = 5;
pub const GT_OBELISK: C2RustUnnamed_0 = 6;
pub const GT_HARVESTER: C2RustUnnamed_0 = 7;
pub const GT_MAX_GAME_TYPE: C2RustUnnamed_0 = 8;
pub const PM_NORMAL: C2RustUnnamed_0 = 0;
pub const PM_NOCLIP: C2RustUnnamed_0 = 1;
pub const PM_SPECTATOR: C2RustUnnamed_0 = 2;
pub const PM_DEAD: C2RustUnnamed_0 = 3;
pub const PM_FREEZE: C2RustUnnamed_0 = 4;
pub const PM_INTERMISSION: C2RustUnnamed_0 = 5;
pub const PM_SPINTERMISSION: C2RustUnnamed_0 = 6;
pub const WEAPON_READY: C2RustUnnamed_0 = 0;
pub const WEAPON_RAISING: C2RustUnnamed_0 = 1;
pub const WEAPON_DROPPING: C2RustUnnamed_0 = 2;
pub const WEAPON_FIRING: C2RustUnnamed_0 = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmove_t {
    pub ps: *mut crate::src::qcommon::q_shared::playerState_t,
    pub cmd: crate::src::qcommon::q_shared::usercmd_t,
    pub tracemask: i32,
    pub debugLevel: i32,
    pub noFootsteps: crate::src::qcommon::q_shared::qboolean,
    pub gauntletHit: crate::src::qcommon::q_shared::qboolean,
    pub framecount: i32,
    pub numtouch: i32,
    pub touchents: [i32; 32],
    pub mins: crate::src::qcommon::q_shared::vec3_t,
    pub maxs: crate::src::qcommon::q_shared::vec3_t,
    pub watertype: i32,
    pub waterlevel: i32,
    pub xyspeed: f32,
    pub pmove_fixed: i32,
    pub pmove_msec: i32,
    pub trace: Option<
        unsafe extern "C" fn(
            _: *mut crate::src::qcommon::q_shared::trace_t,
            _: *const crate::src::qcommon::q_shared::vec_t,
            _: *const crate::src::qcommon::q_shared::vec_t,
            _: *const crate::src::qcommon::q_shared::vec_t,
            _: *const crate::src::qcommon::q_shared::vec_t,
            _: i32,
            _: i32,
        ) -> (),
    >,
    pub pointcontents:
        Option<unsafe extern "C" fn(_: *const crate::src::qcommon::q_shared::vec_t, _: i32) -> i32>,
}
pub const STAT_HEALTH: C2RustUnnamed_0 = 0;
pub const STAT_HOLDABLE_ITEM: C2RustUnnamed_0 = 1;
pub const STAT_WEAPONS: C2RustUnnamed_0 = 2;
pub const STAT_ARMOR: C2RustUnnamed_0 = 3;
pub const STAT_DEAD_YAW: C2RustUnnamed_0 = 4;
pub const STAT_CLIENTS_READY: C2RustUnnamed_0 = 5;
pub const STAT_MAX_HEALTH: C2RustUnnamed_0 = 6;
pub const PERS_SCORE: C2RustUnnamed_0 = 0;
pub const PERS_HITS: C2RustUnnamed_0 = 1;
pub const PERS_RANK: C2RustUnnamed_0 = 2;
pub const PERS_TEAM: C2RustUnnamed_0 = 3;
pub const PERS_SPAWN_COUNT: C2RustUnnamed_0 = 4;
pub const PERS_PLAYEREVENTS: C2RustUnnamed_0 = 5;
pub const PERS_ATTACKER: C2RustUnnamed_0 = 6;
pub const PERS_ATTACKEE_ARMOR: C2RustUnnamed_0 = 7;
pub const PERS_KILLED: C2RustUnnamed_0 = 8;
pub const PERS_IMPRESSIVE_COUNT: C2RustUnnamed_0 = 9;
pub const PERS_EXCELLENT_COUNT: C2RustUnnamed_0 = 10;
pub const PERS_DEFEND_COUNT: C2RustUnnamed_0 = 11;
pub const PERS_ASSIST_COUNT: C2RustUnnamed_0 = 12;
pub const PERS_GAUNTLET_FRAG_COUNT: C2RustUnnamed_0 = 13;
pub const PERS_CAPTURES: C2RustUnnamed_0 = 14;
pub type powerup_t = u32;
pub const PW_NONE: C2RustUnnamed_0 = 0;
pub const PW_QUAD: C2RustUnnamed_0 = 1;
pub const PW_BATTLESUIT: C2RustUnnamed_0 = 2;
pub const PW_HASTE: C2RustUnnamed_0 = 3;
pub const PW_INVIS: C2RustUnnamed_0 = 4;
pub const PW_REGEN: C2RustUnnamed_0 = 5;
pub const PW_FLIGHT: C2RustUnnamed_0 = 6;
pub const PW_REDFLAG: C2RustUnnamed_0 = 7;
pub const PW_BLUEFLAG: C2RustUnnamed_0 = 8;
pub const PW_NEUTRALFLAG: C2RustUnnamed_0 = 9;
pub const PW_SCOUT: C2RustUnnamed_0 = 10;
pub const PW_GUARD: C2RustUnnamed_0 = 11;
pub const PW_DOUBLER: C2RustUnnamed_0 = 12;
pub const PW_AMMOREGEN: C2RustUnnamed_0 = 13;
pub const PW_INVULNERABILITY: C2RustUnnamed_0 = 14;
pub const PW_NUM_POWERUPS: C2RustUnnamed_0 = 15;
pub type holdable_t = u32;
pub const HI_NONE: holdable_t = 0;
pub const HI_TELEPORTER: holdable_t = 1;
pub const HI_MEDKIT: holdable_t = 2;
pub const HI_KAMIKAZE: holdable_t = 3;
pub const HI_PORTAL: holdable_t = 4;
pub const HI_INVULNERABILITY: holdable_t = 5;
pub const HI_NUM_HOLDABLE: holdable_t = 6;
pub type weapon_t = u32;
pub const WP_NONE: C2RustUnnamed_0 = 0;
pub const WP_GAUNTLET: C2RustUnnamed_0 = 1;
pub const WP_MACHINEGUN: C2RustUnnamed_0 = 2;
pub const WP_SHOTGUN: C2RustUnnamed_0 = 3;
pub const WP_GRENADE_LAUNCHER: C2RustUnnamed_0 = 4;
pub const WP_ROCKET_LAUNCHER: C2RustUnnamed_0 = 5;
pub const WP_LIGHTNING: C2RustUnnamed_0 = 6;
pub const WP_RAILGUN: C2RustUnnamed_0 = 7;
pub const WP_PLASMAGUN: C2RustUnnamed_0 = 8;
pub const WP_BFG: C2RustUnnamed_0 = 9;
pub const WP_GRAPPLING_HOOK: C2RustUnnamed_0 = 10;
pub const WP_NUM_WEAPONS: C2RustUnnamed_0 = 11;
pub const EV_NONE: C2RustUnnamed_0 = 0;
pub const EV_FOOTSTEP: C2RustUnnamed_0 = 1;
pub const EV_FOOTSTEP_METAL: C2RustUnnamed_0 = 2;
pub const EV_FOOTSPLASH: C2RustUnnamed_0 = 3;
pub const EV_FOOTWADE: C2RustUnnamed_0 = 4;
pub const EV_SWIM: C2RustUnnamed_0 = 5;
pub const EV_STEP_4: C2RustUnnamed_0 = 6;
pub const EV_STEP_8: C2RustUnnamed_0 = 7;
pub const EV_STEP_12: C2RustUnnamed_0 = 8;
pub const EV_STEP_16: C2RustUnnamed_0 = 9;
pub const EV_FALL_SHORT: C2RustUnnamed_0 = 10;
pub const EV_FALL_MEDIUM: C2RustUnnamed_0 = 11;
pub const EV_FALL_FAR: C2RustUnnamed_0 = 12;
pub const EV_JUMP_PAD: C2RustUnnamed_0 = 13;
pub const EV_JUMP: C2RustUnnamed_0 = 14;
pub const EV_WATER_TOUCH: C2RustUnnamed_0 = 15;
pub const EV_WATER_LEAVE: C2RustUnnamed_0 = 16;
pub const EV_WATER_UNDER: C2RustUnnamed_0 = 17;
pub const EV_WATER_CLEAR: C2RustUnnamed_0 = 18;
pub const EV_ITEM_PICKUP: C2RustUnnamed_0 = 19;
pub const EV_GLOBAL_ITEM_PICKUP: C2RustUnnamed_0 = 20;
pub const EV_NOAMMO: C2RustUnnamed_0 = 21;
pub const EV_CHANGE_WEAPON: C2RustUnnamed_0 = 22;
pub const EV_FIRE_WEAPON: C2RustUnnamed_0 = 23;
pub const EV_USE_ITEM0: C2RustUnnamed_0 = 24;
pub const EV_USE_ITEM1: C2RustUnnamed_0 = 25;
pub const EV_USE_ITEM2: C2RustUnnamed_0 = 26;
pub const EV_USE_ITEM3: C2RustUnnamed_0 = 27;
pub const EV_USE_ITEM4: C2RustUnnamed_0 = 28;
pub const EV_USE_ITEM5: C2RustUnnamed_0 = 29;
pub const EV_USE_ITEM6: C2RustUnnamed_0 = 30;
pub const EV_USE_ITEM7: C2RustUnnamed_0 = 31;
pub const EV_USE_ITEM8: C2RustUnnamed_0 = 32;
pub const EV_USE_ITEM9: C2RustUnnamed_0 = 33;
pub const EV_USE_ITEM10: C2RustUnnamed_0 = 34;
pub const EV_USE_ITEM11: C2RustUnnamed_0 = 35;
pub const EV_USE_ITEM12: C2RustUnnamed_0 = 36;
pub const EV_USE_ITEM13: C2RustUnnamed_0 = 37;
pub const EV_USE_ITEM14: C2RustUnnamed_0 = 38;
pub const EV_USE_ITEM15: C2RustUnnamed_0 = 39;
pub const EV_ITEM_RESPAWN: C2RustUnnamed_0 = 40;
pub const EV_ITEM_POP: C2RustUnnamed_0 = 41;
pub const EV_PLAYER_TELEPORT_IN: C2RustUnnamed_0 = 42;
pub const EV_PLAYER_TELEPORT_OUT: C2RustUnnamed_0 = 43;
pub const EV_GRENADE_BOUNCE: C2RustUnnamed_0 = 44;
pub const EV_GENERAL_SOUND: C2RustUnnamed_0 = 45;
pub const EV_GLOBAL_SOUND: C2RustUnnamed_0 = 46;
pub const EV_GLOBAL_TEAM_SOUND: C2RustUnnamed_0 = 47;
pub const EV_BULLET_HIT_FLESH: C2RustUnnamed_0 = 48;
pub const EV_BULLET_HIT_WALL: C2RustUnnamed_0 = 49;
pub const EV_MISSILE_HIT: C2RustUnnamed_0 = 50;
pub const EV_MISSILE_MISS: C2RustUnnamed_0 = 51;
pub const EV_MISSILE_MISS_METAL: C2RustUnnamed_0 = 52;
pub const EV_RAILTRAIL: C2RustUnnamed_0 = 53;
pub const EV_SHOTGUN: C2RustUnnamed_0 = 54;
pub const EV_BULLET: C2RustUnnamed_0 = 55;
pub const EV_PAIN: C2RustUnnamed_0 = 56;
pub const EV_DEATH1: C2RustUnnamed_0 = 57;
pub const EV_DEATH2: C2RustUnnamed_0 = 58;
pub const EV_DEATH3: C2RustUnnamed_0 = 59;
pub const EV_OBITUARY: C2RustUnnamed_0 = 60;
pub const EV_POWERUP_QUAD: C2RustUnnamed_0 = 61;
pub const EV_POWERUP_BATTLESUIT: C2RustUnnamed_0 = 62;
pub const EV_POWERUP_REGEN: C2RustUnnamed_0 = 63;
pub const EV_GIB_PLAYER: C2RustUnnamed_0 = 64;
pub const EV_SCOREPLUM: C2RustUnnamed_0 = 65;
pub const EV_PROXIMITY_MINE_STICK: C2RustUnnamed_0 = 66;
pub const EV_PROXIMITY_MINE_TRIGGER: C2RustUnnamed_0 = 67;
pub const EV_KAMIKAZE: C2RustUnnamed_0 = 68;
pub const EV_OBELISKEXPLODE: C2RustUnnamed_0 = 69;
pub const EV_OBELISKPAIN: C2RustUnnamed_0 = 70;
pub const EV_INVUL_IMPACT: C2RustUnnamed_0 = 71;
pub const EV_JUICED: C2RustUnnamed_0 = 72;
pub const EV_LIGHTNINGBOLT: C2RustUnnamed_0 = 73;
pub const EV_DEBUG_LINE: C2RustUnnamed_0 = 74;
pub const EV_STOPLOOPINGSOUND: C2RustUnnamed_0 = 75;
pub const EV_TAUNT: C2RustUnnamed_0 = 76;
pub const EV_TAUNT_YES: C2RustUnnamed_0 = 77;
pub const EV_TAUNT_NO: C2RustUnnamed_0 = 78;
pub const EV_TAUNT_FOLLOWME: C2RustUnnamed_0 = 79;
pub const EV_TAUNT_GETFLAG: C2RustUnnamed_0 = 80;
pub const EV_TAUNT_GUARDBASE: C2RustUnnamed_0 = 81;
pub const EV_TAUNT_PATROL: C2RustUnnamed_0 = 82;
pub const GTS_RED_CAPTURE: C2RustUnnamed_0 = 0;
pub const GTS_BLUE_CAPTURE: C2RustUnnamed_0 = 1;
pub const GTS_RED_RETURN: C2RustUnnamed_0 = 2;
pub const GTS_BLUE_RETURN: C2RustUnnamed_0 = 3;
pub const GTS_RED_TAKEN: C2RustUnnamed_0 = 4;
pub const GTS_BLUE_TAKEN: C2RustUnnamed_0 = 5;
pub const GTS_REDOBELISK_ATTACKED: C2RustUnnamed_0 = 6;
pub const GTS_BLUEOBELISK_ATTACKED: C2RustUnnamed_0 = 7;
pub const GTS_REDTEAM_SCORED: C2RustUnnamed_0 = 8;
pub const GTS_BLUETEAM_SCORED: C2RustUnnamed_0 = 9;
pub const GTS_REDTEAM_TOOK_LEAD: C2RustUnnamed_0 = 10;
pub const GTS_BLUETEAM_TOOK_LEAD: C2RustUnnamed_0 = 11;
pub const GTS_TEAMS_ARE_TIED: C2RustUnnamed_0 = 12;
pub const GTS_KAMIKAZE: C2RustUnnamed_0 = 13;
pub const BOTH_DEATH1: C2RustUnnamed_0 = 0;
pub const BOTH_DEAD1: C2RustUnnamed_0 = 1;
pub const BOTH_DEATH2: C2RustUnnamed_0 = 2;
pub const BOTH_DEAD2: C2RustUnnamed_0 = 3;
pub const BOTH_DEATH3: C2RustUnnamed_0 = 4;
pub const BOTH_DEAD3: C2RustUnnamed_0 = 5;
pub const TORSO_GESTURE: C2RustUnnamed_0 = 6;
pub const TORSO_ATTACK: C2RustUnnamed_0 = 7;
pub const TORSO_ATTACK2: C2RustUnnamed_0 = 8;
pub const TORSO_DROP: C2RustUnnamed_0 = 9;
pub const TORSO_RAISE: C2RustUnnamed_0 = 10;
pub const TORSO_STAND: C2RustUnnamed_0 = 11;
pub const TORSO_STAND2: C2RustUnnamed_0 = 12;
pub const LEGS_WALKCR: C2RustUnnamed_0 = 13;
pub const LEGS_WALK: C2RustUnnamed_0 = 14;
pub const LEGS_RUN: C2RustUnnamed_0 = 15;
pub const LEGS_BACK: C2RustUnnamed_0 = 16;
pub const LEGS_SWIM: C2RustUnnamed_0 = 17;
pub const LEGS_JUMP: C2RustUnnamed_0 = 18;
pub const LEGS_LAND: C2RustUnnamed_0 = 19;
pub const LEGS_JUMPB: C2RustUnnamed_0 = 20;
pub const LEGS_LANDB: C2RustUnnamed_0 = 21;
pub const LEGS_IDLE: C2RustUnnamed_0 = 22;
pub const LEGS_IDLECR: C2RustUnnamed_0 = 23;
pub const LEGS_TURN: C2RustUnnamed_0 = 24;
pub const TORSO_GETFLAG: C2RustUnnamed_0 = 25;
pub const TORSO_GUARDBASE: C2RustUnnamed_0 = 26;
pub const TORSO_PATROL: C2RustUnnamed_0 = 27;
pub const TORSO_FOLLOWME: C2RustUnnamed_0 = 28;
pub const TORSO_AFFIRMATIVE: C2RustUnnamed_0 = 29;
pub const TORSO_NEGATIVE: C2RustUnnamed_0 = 30;
pub const MAX_ANIMATIONS: C2RustUnnamed_0 = 31;
pub const LEGS_BACKCR: C2RustUnnamed_0 = 32;
pub const LEGS_BACKWALK: C2RustUnnamed_0 = 33;
pub const FLAG_RUN: C2RustUnnamed_0 = 34;
pub const FLAG_STAND: C2RustUnnamed_0 = 35;
pub const FLAG_STAND2RUN: C2RustUnnamed_0 = 36;
pub const MAX_TOTALANIMATIONS: C2RustUnnamed_0 = 37;
pub type team_t = u32;
pub const TEAM_FREE: team_t = 0;
pub const TEAM_RED: team_t = 1;
pub const TEAM_BLUE: team_t = 2;
pub const TEAM_SPECTATOR: team_t = 3;
pub const TEAM_NUM_TEAMS: team_t = 4;
pub const TEAMTASK_NONE: C2RustUnnamed_0 = 0;
pub const TEAMTASK_OFFENSE: C2RustUnnamed_0 = 1;
pub const TEAMTASK_DEFENSE: C2RustUnnamed_0 = 2;
pub const TEAMTASK_PATROL: C2RustUnnamed_0 = 3;
pub const TEAMTASK_FOLLOW: C2RustUnnamed_0 = 4;
pub const TEAMTASK_RETRIEVE: C2RustUnnamed_0 = 5;
pub const TEAMTASK_ESCORT: C2RustUnnamed_0 = 6;
pub const TEAMTASK_CAMP: C2RustUnnamed_0 = 7;
pub const MOD_UNKNOWN: C2RustUnnamed_0 = 0;
pub const MOD_SHOTGUN: C2RustUnnamed_0 = 1;
pub const MOD_GAUNTLET: C2RustUnnamed_0 = 2;
pub const MOD_MACHINEGUN: C2RustUnnamed_0 = 3;
pub const MOD_GRENADE: C2RustUnnamed_0 = 4;
pub const MOD_GRENADE_SPLASH: C2RustUnnamed_0 = 5;
pub const MOD_ROCKET: C2RustUnnamed_0 = 6;
pub const MOD_ROCKET_SPLASH: C2RustUnnamed_0 = 7;
pub const MOD_PLASMA: C2RustUnnamed_0 = 8;
pub const MOD_PLASMA_SPLASH: C2RustUnnamed_0 = 9;
pub const MOD_RAILGUN: C2RustUnnamed_0 = 10;
pub const MOD_LIGHTNING: C2RustUnnamed_0 = 11;
pub const MOD_BFG: C2RustUnnamed_0 = 12;
pub const MOD_BFG_SPLASH: C2RustUnnamed_0 = 13;
pub const MOD_WATER: C2RustUnnamed_0 = 14;
pub const MOD_SLIME: C2RustUnnamed_0 = 15;
pub const MOD_LAVA: C2RustUnnamed_0 = 16;
pub const MOD_CRUSH: C2RustUnnamed_0 = 17;
pub const MOD_TELEFRAG: C2RustUnnamed_0 = 18;
pub const MOD_FALLING: C2RustUnnamed_0 = 19;
pub const MOD_SUICIDE: C2RustUnnamed_0 = 20;
pub const MOD_TARGET_LASER: C2RustUnnamed_0 = 21;
pub const MOD_TRIGGER_HURT: C2RustUnnamed_0 = 22;
pub const MOD_GRAPPLE: C2RustUnnamed_0 = 23;
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
pub const ET_GENERAL: C2RustUnnamed_0 = 0;
pub const ET_PLAYER: C2RustUnnamed_0 = 1;
pub const ET_ITEM: C2RustUnnamed_0 = 2;
pub const ET_MISSILE: C2RustUnnamed_0 = 3;
pub const ET_MOVER: C2RustUnnamed_0 = 4;
pub const ET_BEAM: C2RustUnnamed_0 = 5;
pub const ET_PORTAL: C2RustUnnamed_0 = 6;
pub const ET_SPEAKER: C2RustUnnamed_0 = 7;
pub const ET_PUSH_TRIGGER: C2RustUnnamed_0 = 8;
pub const ET_TELEPORT_TRIGGER: C2RustUnnamed_0 = 9;
pub const ET_INVISIBLE: C2RustUnnamed_0 = 10;
pub const ET_GRAPPLE: C2RustUnnamed_0 = 11;
// any of the EV_* events can be added freestanding

// by setting eType to ET_EVENTS + eventNum

// this avoids having to set eFlags and eventNum

// grapple hooked on wall
pub const ET_TEAM: C2RustUnnamed_0 = 12;
pub const ET_EVENTS: C2RustUnnamed_0 = 13;
