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
}

#[derive(
    Eq, TryFromPrimitive, Serialize_repr, Deserialize_repr, PartialEq, Clone, Copy, Debug, Default, EnumIter
)]
#[repr(i32)]
pub enum UserFlags {
    VotesPrivate = 0,
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

#[derive(Eq, Serialize_repr, Deserialize_repr, PartialEq, Clone, Copy, Default, Debug, EnumIter)]
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
