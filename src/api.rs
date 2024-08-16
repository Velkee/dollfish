use serde::{Deserialize, Serialize};

pub mod session_info;
pub mod user;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub configuration: user::Configuration,
    pub enable_auto_login: bool,
    pub has_configured_easy_password: bool,
    pub has_configured_password: bool,
    pub has_password: bool,
    pub id: String,
    pub last_activity_date: String,
    pub last_login_date: String,
    pub name: String,
    pub policy: user::Policy,
    pub primary_image_aspect_ratio: i32,
    pub primary_image_tag: String,
    pub server_id: String,
    pub server_name: String,
}

struct SessionInfo {
    additional_users: Vec<_>,
    application_version: String,
    capabilities: session_info::Capabilities,
    client: String,
    device_id: String,
    device_name: String,
    device_type: String,
    has_custom_device_name: bool,
    id: String,
    is_active: bool,
    last_activity_date: String,
    last_paused_date: String,
    last_playback_check_in: String,
    now_playing_item: session_info::NowPlayingItem,
    now_playing_queue: Vec<_>,
    now_playing_queue_full_items: Vec<_>,
    now_viewing_item: session_info::NowViewingItem,
    play_state: session_info::PlayState,
    playable_media_types: Vec<_>,
    playlist_item_id: String,
    remote_end_point: String,
    server_id: String,
    supported_commands: Vec<_>,
    supports_media_control: bool,
    supports_remote_control: bool,
    transcoding_info: session_info::TranscodingInfo,
    user_id: String,
    user_name: String,
    user_primary_image_tag: String,
}
