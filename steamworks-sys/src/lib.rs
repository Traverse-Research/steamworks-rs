#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

#[cfg(target_os = "windows")]
include!("windows_bindings.rs");

#[cfg(target_os = "macos")]
include!("macos_bindings.rs");

#[cfg(target_os = "linux")]
include!("linux_bindings.rs");

// User stats callback structures
// These are used for receiving callback data from Steam

/// Callback for RequestCurrentStats
#[repr(C, packed(4))]
#[derive(Copy, Clone)]
pub struct UserStatsReceived_t {
    pub m_nGameID: u64,
    pub m_eResult: EResult,
    pub m_steamIDUser: CSteamID,
}

/// Callback for StoreStats
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct UserStatsStored_t {
    pub m_nGameID: u64,
    pub m_eResult: EResult,
}

/// Callback for achievement storage
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct UserAchievementStored_t {
    pub m_nGameID: u64,
    pub m_bGroupAchievement: bool,
    pub m_rgchAchievementName: [::std::os::raw::c_char; 128],
    pub m_nCurProgress: u32,
    pub m_nMaxProgress: u32,
}

/// Callback for achievement icon fetch
#[repr(C, packed(4))]
#[derive(Copy, Clone)]
pub struct UserAchievementIconFetched_t {
    pub m_nGameID: CGameID,
    pub m_rgchAchievementName: [::std::os::raw::c_char; 128],
    pub m_bAchieved: bool,
    pub m_nIconHandle: ::std::os::raw::c_int,
}

/// Callback for FindLeaderboard
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct LeaderboardFindResult_t {
    pub m_hSteamLeaderboard: u64,
    pub m_bLeaderboardFound: u8,
}

/// Callback for UploadLeaderboardScore
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct LeaderboardScoreUploaded_t {
    pub m_bSuccess: u8,
    pub m_hSteamLeaderboard: u64,
    pub m_nScore: i32,
    pub m_bScoreChanged: u8,
    pub m_nGlobalRankNew: ::std::os::raw::c_int,
    pub m_nGlobalRankPrevious: ::std::os::raw::c_int,
}

/// Callback for DownloadLeaderboardEntries
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct LeaderboardScoresDownloaded_t {
    pub m_hSteamLeaderboard: u64,
    pub m_hSteamLeaderboardEntries: u64,
    pub m_cEntryCount: ::std::os::raw::c_int,
}

/// Callback for RequestGlobalAchievementPercentages
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct GlobalAchievementPercentagesReady_t {
    pub m_nGameID: u64,
    pub m_eResult: EResult,
}

// Friends and utils callback structures, copied from the upstream bindgen output.
// Steam packs callbacks to 4 bytes everywhere except Windows, which only matters for 64-bit fields.

/// Callback for persona state changes
#[cfg_attr(target_os = "windows", repr(C))]
#[cfg_attr(not(target_os = "windows"), repr(C, packed(4)))]
#[derive(Debug, Copy, Clone)]
pub struct PersonaStateChange_t {
    pub m_ulSteamID: uint64,
    pub m_nChangeFlags: ::std::os::raw::c_int,
}

/// Callback for the overlay being opened or closed
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GameOverlayActivated_t {
    pub m_bActive: uint8,
    pub m_bUserInitiated: bool,
    pub m_nAppID: AppId_t,
    pub m_dwOverlayPID: uint32,
}

/// Callback for a lobby join requested through the friends list
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GameLobbyJoinRequested_t {
    pub m_steamIDLobby: CSteamID,
    pub m_steamIDFriend: CSteamID,
}

/// Callback for ShowGamepadTextInput
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GamepadTextInputDismissed_t {
    pub m_bSubmitted: bool,
    pub m_unSubmittedText: uint32,
    pub m_unAppID: AppId_t,
}

/// Callback for ShowFloatingGamepadTextInput
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FloatingGamepadTextInputDismissed_t {
    pub _address: u8,
}
