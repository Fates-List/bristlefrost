use strum_macros::EnumIter;
use serde::{Deserialize, Serialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use num_enum::TryFromPrimitive;

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct User {
    pub id: String,
    pub username: String,
    pub disc: String,
    pub avatar: String,
    pub bot: bool,
    pub status: Status,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct StaffRole {
    pub id: String,
    pub staff_id: String,
    pub perm: f32,
    pub fname: String
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Default, Debug, EnumIter)]
pub enum Status {
    #[default]
    Unknown = 0,
    Online = 1,
    Offline = 2, // Or invisible
    Idle = 3,
    DoNotDisturb = 4,
}

#[derive(
    Eq, TryFromPrimitive, Serialize_repr, Deserialize_repr, PartialEq, Clone, Copy, Debug, Default, EnumIter
)]
#[repr(i32)]
pub enum State {
    #[default]
    Approved = 0,
    Pending = 1,
    Denied = 2,
    Hidden = 3,
    Banned = 4,
    UnderReview = 5,
    Certified = 6,
    Archived = 7,
    PrivateViewable = 8,
    PrivateStaffOnly = 9,
}


#[derive(
    Eq, TryFromPrimitive, Serialize_repr, Deserialize_repr, PartialEq, Clone, Copy, Debug, Default, EnumIter
)]
#[repr(i32)]
pub enum Flags {
    #[default]
    Unlocked = 0,
    EditLocked = 1,
    StaffLocked = 2,
    StatsLocked = 3,
    VoteLocked = 4,
    System = 5,
    WhitelistOnly = 6,
    KeepBannerDecor = 7,
    NSFW = 8,
    LoginRequired = 9,
}

#[derive(
    Eq, TryFromPrimitive, Serialize_repr, Deserialize_repr, PartialEq, Clone, Copy, Debug, Default, EnumIter
)]
#[repr(i32)]
pub enum UserFlags {
    #[default]
    Unknown = 0,
    VotesPrivate = 1, // The user wishes to make their votes private
    Staff = 2, // The user is a public staff member
    AvidVoter = 3, // This user has voted on our site multiple times :)
    Premium = 4, // The user is a premium member
    Failure = 5, // The user is in Failure Management 
}


#[derive(
    Eq, TryFromPrimitive, Serialize_repr, Deserialize_repr, PartialEq, Clone, Copy, Default, Debug, EnumIter
)]
#[repr(i32)]
pub enum UserState {
    #[default]
    Normal = 0,
    GlobalBan = 1,
    ProfileEditBan = 2,
}

#[derive(
    Eq, TryFromPrimitive, Serialize_repr, Deserialize_repr, PartialEq, Clone, Copy, Default, Debug, EnumIter
)]
#[repr(i32)]
pub enum LongDescriptionType {
    Html = 0,
    #[default]
    MarkdownServerSide = 1,
}

#[derive(
    Eq, TryFromPrimitive, Serialize_repr, Deserialize_repr, PartialEq, Clone, Copy, Default, Debug, EnumIter
)]
#[repr(i32)]
pub enum WebhookType {
    #[default]
    Vote = 0,
    DiscordIntegration = 1,
    DeprecatedFatesClient = 2,
}

#[derive(Hash, Eq, Serialize_repr, Deserialize_repr, PartialEq, Clone, Copy, Default, Debug, EnumIter)]
#[repr(i32)]
pub enum TargetType {
    #[default]
    Bot = 0,
    Server = 1,
}

impl TargetType {
    #[must_use]
    pub fn to_arg(t: TargetType) -> &'static str {
        match t {
            TargetType::Bot => "bot",
            TargetType::Server => "server",
        }
    }
}
