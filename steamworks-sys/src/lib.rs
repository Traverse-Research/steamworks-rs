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
