use std::ffi::{c_char, c_float, c_int, c_uchar, c_uint, c_void};

pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct WebSocketPlayerInfo {
    pub identifier_bytes: Vec<u8>,
    pub position: Vec3,
    pub rotation: Vec3,
    pub forwardfrac: Vec<u8>,
}

#[repr(C)]
#[derive(Debug)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct LocalPlayer {
    pub actor: *mut c_void, // 0x00
    pub pad0x00: [u8; 0x6C],
    pub IPlayer: *mut c_void,      //0x70
    pub m_characterId: c_uint,     // 0x74
    pub m_name: *const c_char,     //0x78
    pub m_teamName: *const c_char, // 0x90
    pub m_avatarIndex: c_uchar,    // 0xA8
    pub colors: [u8; 4],           // 0xAC
    pub pad0xAC: [u8; 0xC],
    pub m_inventory: *mut c_void, // std::map<IItem *,ItemAndCount> // 0xBC
    pub pad0xBC: [u8; 0x4],
    pub m_pickups: [u8; 0x8],     // 0xC4
    pub m_cooldowns: *mut c_void, // std::map<IItem *,float> // 0xCC
    pub pad0xCC: [u8; 0x4],
    pub m_circuitInputs: [u8; 0x8],  // 0xD4
    pub m_circuitOutputs: [u8; 0x8], // 0xDC
    pub m_admin: bool,               // 0xE4
    pub m_pvpEnabled: bool,          // 0xE5
    pub m_pvpDesired: bool,          // 0xE6
    pub pad0xE6: [u8; 0x1],
    pub m_pvpChangeTimer: c_float,                // 0xE8
    pub m_pvpChangeReportedTimer: c_int,          // 0xEC
    pub m_changingServerRegion: bool,             // 0xF0
    pub m_currentRegion: *const c_char,           // 0xF4
    pub m_changeRegionDestination: *const c_char, // 0x10C
    pub m_aiZones: [u8; 0x18],                    // 0x124
    pub m_mana: c_int,                            // 0x12C
    pub m_manaRegenTimer: c_float,                // 0x130
    pub m_healthRegenCooldown: c_float,           // 0x134
    pub m_healthRegenTimer: c_float,              // 0x138
    pub m_countdown: c_uint,                      // 0x13C
    pub m_remoteLookPosition: [c_float; 3],       // 0x140
    pub m_remoteLookRotation: [c_float; 3],       // 0x14C
    pub m_equipped: [*mut c_void; 10],            // 0x158
    pub pad0x158: [u8; 0x24],
    pub m_currentSlot: c_int,       // 0x180
    pub m_questStates: *mut c_void, // std::map<IQuest *,PlayerQuestState> // 0x184
    pub pad0x184: [u8; 0x4],
    pub m_currentQuest: *mut c_void,          // 0x18C
    pub m_walkingSpeed: c_float,              // 0x190
    pub m_jumpSpeed: c_float,                 // 0x194
    pub m_jumpHoldTime: c_float,              // 0x198
    pub m_currentNPC: *mut c_void,            // ActorRef<NPC> // 0x19C
    pub m_currentNPCState: *const c_char,     // 0x1A0
    pub m_localPlayer: *mut c_void,           // 0x1B8
    pub m_eventsToSend: c_int,                // 0x1BC
    pub m_itemsUpdated: bool,                 // 0x1C0
    pub m_itemSyncTimer: c_float,             // 0x1C4
    pub m_chatMessageCounter: c_uint,         // 0x1C8
    pub m_chatFloodDecayTimer: c_float,       // 0x1CC
    pub m_lastHitByItem: c_int,               // 0x1D0
    pub m_lastHitItemTimeLeft: c_float,       // 0x1D4
    pub m_circuitStateCooldownTimer: c_float, // 0x1D8
}
