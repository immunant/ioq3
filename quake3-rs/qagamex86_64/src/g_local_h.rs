pub type moverState_t = libc::c_uint;
pub const MOVER_POS1: moverState_t = 0;
pub const MOVER_POS2: moverState_t = 1;
pub const MOVER_1TO2: moverState_t = 2;
pub const MOVER_2TO1: moverState_t = 3;
pub type gentity_t = gentity_s;
pub type gclient_t = gclient_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gentity_s {
    pub s: crate::src::qcommon::q_shared::entityState_t,
    pub r: crate::g_public_h::entityShared_t,
    pub client: *mut gclient_s,
    pub inuse: crate::src::qcommon::q_shared::qboolean,
    pub classname: *mut libc::c_char,
    pub spawnflags: libc::c_int,
    pub neverFree: crate::src::qcommon::q_shared::qboolean,
    pub flags: libc::c_int,
    pub model: *mut libc::c_char,
    pub model2: *mut libc::c_char,
    pub freetime: libc::c_int,
    pub eventTime: libc::c_int,
    pub freeAfterEvent: crate::src::qcommon::q_shared::qboolean,
    pub unlinkAfterEvent: crate::src::qcommon::q_shared::qboolean,
    pub physicsObject: crate::src::qcommon::q_shared::qboolean,
    pub physicsBounce: libc::c_float,
    pub clipmask: libc::c_int,
    pub moverState: moverState_t,
    pub soundPos1: libc::c_int,
    pub sound1to2: libc::c_int,
    pub sound2to1: libc::c_int,
    pub soundPos2: libc::c_int,
    pub soundLoop: libc::c_int,
    pub parent: *mut gentity_t,
    pub nextTrain: *mut gentity_t,
    pub prevTrain: *mut gentity_t,
    pub pos1: crate::src::qcommon::q_shared::vec3_t,
    pub pos2: crate::src::qcommon::q_shared::vec3_t,
    pub message: *mut libc::c_char,
    pub timestamp: libc::c_int,
    pub target: *mut libc::c_char,
    pub targetname: *mut libc::c_char,
    pub team: *mut libc::c_char,
    pub targetShaderName: *mut libc::c_char,
    pub targetShaderNewName: *mut libc::c_char,
    pub target_ent: *mut gentity_t,
    pub speed: libc::c_float,
    pub movedir: crate::src::qcommon::q_shared::vec3_t,
    pub nextthink: libc::c_int,
    pub think: Option<unsafe extern "C" fn(_: *mut gentity_t) -> ()>,
    pub reached: Option<unsafe extern "C" fn(_: *mut gentity_t) -> ()>,
    pub blocked: Option<
        unsafe extern "C" fn(
            _: *mut gentity_t,
            _: *mut gentity_t,
        ) -> (),
    >,
    pub touch: Option<
        unsafe extern "C" fn(
            _: *mut gentity_t,
            _: *mut gentity_t,
            _: *mut crate::src::qcommon::q_shared::trace_t,
        ) -> (),
    >,
    pub use_0: Option<
        unsafe extern "C" fn(
            _: *mut gentity_t,
            _: *mut gentity_t,
            _: *mut gentity_t,
        ) -> (),
    >,
    pub pain: Option<
        unsafe extern "C" fn(
            _: *mut gentity_t,
            _: *mut gentity_t,
            _: libc::c_int,
        ) -> (),
    >,
    pub die: Option<
        unsafe extern "C" fn(
            _: *mut gentity_t,
            _: *mut gentity_t,
            _: *mut gentity_t,
            _: libc::c_int,
            _: libc::c_int,
        ) -> (),
    >,
    pub pain_debounce_time: libc::c_int,
    pub fly_sound_debounce_time: libc::c_int,
    pub last_move_time: libc::c_int,
    pub health: libc::c_int,
    pub takedamage: crate::src::qcommon::q_shared::qboolean,
    pub damage: libc::c_int,
    pub splashDamage: libc::c_int,
    pub splashRadius: libc::c_int,
    pub methodOfDeath: libc::c_int,
    pub splashMethodOfDeath: libc::c_int,
    pub count: libc::c_int,
    pub chain: *mut gentity_t,
    pub enemy: *mut gentity_t,
    pub activator: *mut gentity_t,
    pub teamchain: *mut gentity_t,
    pub teammaster: *mut gentity_t,
    pub watertype: libc::c_int,
    pub waterlevel: libc::c_int,
    pub noise_index: libc::c_int,
    pub wait: libc::c_float,
    pub random: libc::c_float,
    pub item: *mut crate::bg_public_h::gitem_t,
}
pub type clientConnected_t = libc::c_uint;
pub const CON_DISCONNECTED: clientConnected_t = 0;
pub const CON_CONNECTING: clientConnected_t = 1;
pub const CON_CONNECTED: clientConnected_t = 2;
pub type spectatorState_t = libc::c_uint;
pub const SPECTATOR_NOT: spectatorState_t = 0;
pub const SPECTATOR_FREE: spectatorState_t = 1;
pub const SPECTATOR_FOLLOW: spectatorState_t = 2;
pub const SPECTATOR_SCOREBOARD: spectatorState_t = 3;
pub type playerTeamStateState_t = libc::c_uint;
pub const TEAM_BEGIN: playerTeamStateState_t = 0;
pub const TEAM_ACTIVE: playerTeamStateState_t = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct playerTeamState_t {
    pub state: playerTeamStateState_t,
    pub location: libc::c_int,
    pub captures: libc::c_int,
    pub basedefense: libc::c_int,
    pub carrierdefense: libc::c_int,
    pub flagrecovery: libc::c_int,
    pub fragcarrier: libc::c_int,
    pub assists: libc::c_int,
    pub lasthurtcarrier: libc::c_float,
    pub lastreturnedflag: libc::c_float,
    pub flagsince: libc::c_float,
    pub lastfraggedcarrier: libc::c_float,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clientSession_t {
    pub sessionTeam: crate::bg_public_h::team_t,
    pub spectatorNum: libc::c_int,
    pub spectatorState: spectatorState_t,
    pub spectatorClient: libc::c_int,
    pub wins: libc::c_int,
    pub losses: libc::c_int,
    pub teamLeader: crate::src::qcommon::q_shared::qboolean,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clientPersistant_t {
    pub connected: clientConnected_t,
    pub cmd: crate::src::qcommon::q_shared::usercmd_t,
    pub localClient: crate::src::qcommon::q_shared::qboolean,
    pub initialSpawn: crate::src::qcommon::q_shared::qboolean,
    pub predictItemPickup: crate::src::qcommon::q_shared::qboolean,
    pub pmoveFixed: crate::src::qcommon::q_shared::qboolean,
    pub netname: [libc::c_char; 36],
    pub maxHealth: libc::c_int,
    pub enterTime: libc::c_int,
    pub teamState: playerTeamState_t,
    pub voteCount: libc::c_int,
    pub teamVoteCount: libc::c_int,
    pub teamInfo: crate::src::qcommon::q_shared::qboolean,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gclient_s {
    pub ps: crate::src::qcommon::q_shared::playerState_t,
    pub pers: clientPersistant_t,
    pub sess: clientSession_t,
    pub readyToExit: crate::src::qcommon::q_shared::qboolean,
    pub noclip: crate::src::qcommon::q_shared::qboolean,
    pub lastCmdTime: libc::c_int,
    pub buttons: libc::c_int,
    pub oldbuttons: libc::c_int,
    pub latched_buttons: libc::c_int,
    pub oldOrigin: crate::src::qcommon::q_shared::vec3_t,
    pub damage_armor: libc::c_int,
    pub damage_blood: libc::c_int,
    pub damage_knockback: libc::c_int,
    pub damage_from: crate::src::qcommon::q_shared::vec3_t,
    pub damage_fromWorld: crate::src::qcommon::q_shared::qboolean,
    pub accurateCount: libc::c_int,
    pub accuracy_shots: libc::c_int,
    pub accuracy_hits: libc::c_int,
    pub lastkilled_client: libc::c_int,
    pub lasthurt_client: libc::c_int,
    pub lasthurt_mod: libc::c_int,
    pub respawnTime: libc::c_int,
    pub inactivityTime: libc::c_int,
    pub inactivityWarning: crate::src::qcommon::q_shared::qboolean,
    pub rewardTime: libc::c_int,
    pub airOutTime: libc::c_int,
    pub lastKillTime: libc::c_int,
    pub fireHeld: crate::src::qcommon::q_shared::qboolean,
    pub hook: *mut gentity_t,
    pub switchTeamTime: libc::c_int,
    pub timeResidual: libc::c_int,
    pub areabits: *mut libc::c_char,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct level_locals_t {
    pub clients: *mut gclient_s,
    pub gentities: *mut gentity_s,
    pub gentitySize: libc::c_int,
    pub num_entities: libc::c_int,
    pub warmupTime: libc::c_int,
    pub logFile: crate::src::qcommon::q_shared::fileHandle_t,
    pub maxclients: libc::c_int,
    pub framenum: libc::c_int,
    pub time: libc::c_int,
    pub previousTime: libc::c_int,
    pub startTime: libc::c_int,
    pub teamScores: [libc::c_int; 4],
    pub lastTeamLocationTime: libc::c_int,
    pub newSession: crate::src::qcommon::q_shared::qboolean,
    pub restarted: crate::src::qcommon::q_shared::qboolean,
    pub numConnectedClients: libc::c_int,
    pub numNonSpectatorClients: libc::c_int,
    pub numPlayingClients: libc::c_int,
    pub sortedClients: [libc::c_int; 64],
    pub follow1: libc::c_int,
    pub follow2: libc::c_int,
    pub snd_fry: libc::c_int,
    pub warmupModificationCount: libc::c_int,
    pub voteString: [libc::c_char; 1024],
    pub voteDisplayString: [libc::c_char; 1024],
    pub voteTime: libc::c_int,
    pub voteExecuteTime: libc::c_int,
    pub voteYes: libc::c_int,
    pub voteNo: libc::c_int,
    pub numVotingClients: libc::c_int,
    pub teamVoteString: [[libc::c_char; 1024]; 2],
    pub teamVoteTime: [libc::c_int; 2],
    pub teamVoteYes: [libc::c_int; 2],
    pub teamVoteNo: [libc::c_int; 2],
    pub numteamVotingClients: [libc::c_int; 2],
    pub spawning: crate::src::qcommon::q_shared::qboolean,
    pub numSpawnVars: libc::c_int,
    pub spawnVars: [[*mut libc::c_char; 2]; 64],
    pub numSpawnVarChars: libc::c_int,
    pub spawnVarChars: [libc::c_char; 4096],
    pub intermissionQueued: libc::c_int,
    pub intermissiontime: libc::c_int,
    pub changemap: *mut libc::c_char,
    pub readyToExit: crate::src::qcommon::q_shared::qboolean,
    pub exitTime: libc::c_int,
    pub intermission_origin: crate::src::qcommon::q_shared::vec3_t,
    pub intermission_angle: crate::src::qcommon::q_shared::vec3_t,
    pub locationLinked: crate::src::qcommon::q_shared::qboolean,
    pub locationHead: *mut gentity_t,
    pub bodyQueIndex: libc::c_int,
    pub bodyQue: [*mut gentity_t; 8],
}
pub type bot_settings_t = bot_settings_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_settings_s {
    pub characterfile: [libc::c_char; 144],
    pub skill: libc::c_float,
}
