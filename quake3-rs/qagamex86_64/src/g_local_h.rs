pub type moverState_t = u32;
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
    pub spawnflags: i32,
    pub neverFree: crate::src::qcommon::q_shared::qboolean,
    pub flags: i32,
    pub model: *mut libc::c_char,
    pub model2: *mut libc::c_char,
    pub freetime: i32,
    pub eventTime: i32,
    pub freeAfterEvent: crate::src::qcommon::q_shared::qboolean,
    pub unlinkAfterEvent: crate::src::qcommon::q_shared::qboolean,
    pub physicsObject: crate::src::qcommon::q_shared::qboolean,
    pub physicsBounce: f32,
    pub clipmask: i32,
    pub moverState: moverState_t,
    pub soundPos1: i32,
    pub sound1to2: i32,
    pub sound2to1: i32,
    pub soundPos2: i32,
    pub soundLoop: i32,
    pub parent: *mut gentity_t,
    pub nextTrain: *mut gentity_t,
    pub prevTrain: *mut gentity_t,
    pub pos1: crate::src::qcommon::q_shared::vec3_t,
    pub pos2: crate::src::qcommon::q_shared::vec3_t,
    pub message: *mut libc::c_char,
    pub timestamp: i32,
    pub target: *mut libc::c_char,
    pub targetname: *mut libc::c_char,
    pub team: *mut libc::c_char,
    pub targetShaderName: *mut libc::c_char,
    pub targetShaderNewName: *mut libc::c_char,
    pub target_ent: *mut gentity_t,
    pub speed: f32,
    pub movedir: crate::src::qcommon::q_shared::vec3_t,
    pub nextthink: i32,
    pub think: Option<unsafe extern "C" fn(_: *mut gentity_t) -> ()>,
    pub reached: Option<unsafe extern "C" fn(_: *mut gentity_t) -> ()>,
    pub blocked: Option<unsafe extern "C" fn(_: *mut gentity_t, _: *mut gentity_t) -> ()>,
    pub touch: Option<
        unsafe extern "C" fn(
            _: *mut gentity_t,
            _: *mut gentity_t,
            _: *mut crate::src::qcommon::q_shared::trace_t,
        ) -> (),
    >,
    pub use_0:
        Option<unsafe extern "C" fn(_: *mut gentity_t, _: *mut gentity_t, _: *mut gentity_t) -> ()>,
    pub pain:
        Option<unsafe extern "C" fn(_: *mut gentity_t, _: *mut gentity_t, _: i32) -> ()>,
    pub die: Option<
        unsafe extern "C" fn(
            _: *mut gentity_t,
            _: *mut gentity_t,
            _: *mut gentity_t,
            _: i32,
            _: i32,
        ) -> (),
    >,
    pub pain_debounce_time: i32,
    pub fly_sound_debounce_time: i32,
    pub last_move_time: i32,
    pub health: i32,
    pub takedamage: crate::src::qcommon::q_shared::qboolean,
    pub damage: i32,
    pub splashDamage: i32,
    pub splashRadius: i32,
    pub methodOfDeath: i32,
    pub splashMethodOfDeath: i32,
    pub count: i32,
    pub chain: *mut gentity_t,
    pub enemy: *mut gentity_t,
    pub activator: *mut gentity_t,
    pub teamchain: *mut gentity_t,
    pub teammaster: *mut gentity_t,
    pub watertype: i32,
    pub waterlevel: i32,
    pub noise_index: i32,
    pub wait: f32,
    pub random: f32,
    pub item: *mut crate::bg_public_h::gitem_t,
}
pub type clientConnected_t = u32;
pub const CON_DISCONNECTED: clientConnected_t = 0;
pub const CON_CONNECTING: clientConnected_t = 1;
pub const CON_CONNECTED: clientConnected_t = 2;
pub type spectatorState_t = u32;
pub const SPECTATOR_NOT: spectatorState_t = 0;
pub const SPECTATOR_FREE: spectatorState_t = 1;
pub const SPECTATOR_FOLLOW: spectatorState_t = 2;
pub const SPECTATOR_SCOREBOARD: spectatorState_t = 3;
pub type playerTeamStateState_t = u32;
pub const TEAM_BEGIN: playerTeamStateState_t = 0;
pub const TEAM_ACTIVE: playerTeamStateState_t = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct playerTeamState_t {
    pub state: playerTeamStateState_t,
    pub location: i32,
    pub captures: i32,
    pub basedefense: i32,
    pub carrierdefense: i32,
    pub flagrecovery: i32,
    pub fragcarrier: i32,
    pub assists: i32,
    pub lasthurtcarrier: f32,
    pub lastreturnedflag: f32,
    pub flagsince: f32,
    pub lastfraggedcarrier: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clientSession_t {
    pub sessionTeam: crate::bg_public_h::team_t,
    pub spectatorNum: i32,
    pub spectatorState: spectatorState_t,
    pub spectatorClient: i32,
    pub wins: i32,
    pub losses: i32,
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
    pub maxHealth: i32,
    pub enterTime: i32,
    pub teamState: playerTeamState_t,
    pub voteCount: i32,
    pub teamVoteCount: i32,
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
    pub lastCmdTime: i32,
    pub buttons: i32,
    pub oldbuttons: i32,
    pub latched_buttons: i32,
    pub oldOrigin: crate::src::qcommon::q_shared::vec3_t,
    pub damage_armor: i32,
    pub damage_blood: i32,
    pub damage_knockback: i32,
    pub damage_from: crate::src::qcommon::q_shared::vec3_t,
    pub damage_fromWorld: crate::src::qcommon::q_shared::qboolean,
    pub accurateCount: i32,
    pub accuracy_shots: i32,
    pub accuracy_hits: i32,
    pub lastkilled_client: i32,
    pub lasthurt_client: i32,
    pub lasthurt_mod: i32,
    pub respawnTime: i32,
    pub inactivityTime: i32,
    pub inactivityWarning: crate::src::qcommon::q_shared::qboolean,
    pub rewardTime: i32,
    pub airOutTime: i32,
    pub lastKillTime: i32,
    pub fireHeld: crate::src::qcommon::q_shared::qboolean,
    pub hook: *mut gentity_t,
    pub switchTeamTime: i32,
    pub timeResidual: i32,
    pub areabits: *mut libc::c_char,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct level_locals_t {
    pub clients: *mut gclient_s,
    pub gentities: *mut gentity_s,
    pub gentitySize: i32,
    pub num_entities: i32,
    pub warmupTime: i32,
    pub logFile: crate::src::qcommon::q_shared::fileHandle_t,
    pub maxclients: i32,
    pub framenum: i32,
    pub time: i32,
    pub previousTime: i32,
    pub startTime: i32,
    pub teamScores: [i32; 4],
    pub lastTeamLocationTime: i32,
    pub newSession: crate::src::qcommon::q_shared::qboolean,
    pub restarted: crate::src::qcommon::q_shared::qboolean,
    pub numConnectedClients: i32,
    pub numNonSpectatorClients: i32,
    pub numPlayingClients: i32,
    pub sortedClients: [i32; 64],
    pub follow1: i32,
    pub follow2: i32,
    pub snd_fry: i32,
    pub warmupModificationCount: i32,
    pub voteString: [libc::c_char; 1024],
    pub voteDisplayString: [libc::c_char; 1024],
    pub voteTime: i32,
    pub voteExecuteTime: i32,
    pub voteYes: i32,
    pub voteNo: i32,
    pub numVotingClients: i32,
    pub teamVoteString: [[libc::c_char; 1024]; 2],
    pub teamVoteTime: [i32; 2],
    pub teamVoteYes: [i32; 2],
    pub teamVoteNo: [i32; 2],
    pub numteamVotingClients: [i32; 2],
    pub spawning: crate::src::qcommon::q_shared::qboolean,
    pub numSpawnVars: i32,
    pub spawnVars: [[*mut libc::c_char; 2]; 64],
    pub numSpawnVarChars: i32,
    pub spawnVarChars: [libc::c_char; 4096],
    pub intermissionQueued: i32,
    pub intermissiontime: i32,
    pub changemap: *mut libc::c_char,
    pub readyToExit: crate::src::qcommon::q_shared::qboolean,
    pub exitTime: i32,
    pub intermission_origin: crate::src::qcommon::q_shared::vec3_t,
    pub intermission_angle: crate::src::qcommon::q_shared::vec3_t,
    pub locationLinked: crate::src::qcommon::q_shared::qboolean,
    pub locationHead: *mut gentity_t,
    pub bodyQueIndex: i32,
    pub bodyQue: [*mut gentity_t; 8],
}
pub type bot_settings_t = bot_settings_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_settings_s {
    pub characterfile: [libc::c_char; 144],
    pub skill: f32,
}
