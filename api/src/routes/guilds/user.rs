use crate::{
    client::DiscordClient,
    error::ApiError,
    routes::{events::check_token, get_client},
};
use actix_session::Session;
use actix_web::{get, web, HttpRequest, HttpResponse};

#[derive(Deserialize)]
struct MemberPath {
    guild_id: String,
}

#[get("/{guild_id}/@me/permissions")]
async fn get_member(
    session: Session,
    path: web::Path<MemberPath>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let client = get_client(&req)?;
    let token = client.get_user_token(&req)?;
    // let pool = get_connection(&req)?;
    let path = path.into_inner();
    let guild_id = &path.guild_id;
    check_token(&session, guild_id, &token, client).await?;
    let guilds = client.get_current_user_guilds(&token).await?;
    let guild = guilds
        .iter()
        .find(|&guild| &guild.id.0.to_string() == guild_id)
        .ok_or(ApiError::NotFound)?;
    println!("{:?}", guild);
    let permissions = guild
        .permissions
        .clone()
        .unwrap_or_else(|| "0".to_string())
        .parse::<u64>()
        .unwrap_or(0);

    let permissions: Permissions = serenity::model::Permissions { bits: permissions }.into();
    Ok(HttpResponse::Ok().json(permissions))
}

#[derive(Serialize, Deserialize)]
pub struct Permissions {
    add_reactions: bool,
    administrator: bool,
    attach_files: bool,
    ban_members: bool,
    change_nickname: bool,
    connect: bool,
    create_invite: bool,
    deafen_members: bool,
    embed_links: bool,
    external_emojis: bool,
    kick_members: bool,
    manage_channels: bool,
    manage_emojis: bool,
    manage_guild: bool,
    manage_messages: bool,
    manage_nicknames: bool,
    manage_roles: bool,
    manage_webhooks: bool,
    mention_everyone: bool,
    move_members: bool,
    mute_members: bool,
    priority_speaker: bool,
    read_message_history: bool,
    request_to_speak: bool,
    read_messages: bool,
    send_messages: bool,
    send_tts_messages: bool,
    speak: bool,
    stream: bool,
    use_external_emojis: bool,
    use_slash_commands: bool,
    use_vad: bool,
    view_audit_log: bool,
}

impl From<serenity::model::Permissions> for Permissions {
    fn from(permissions: serenity::model::Permissions) -> Self {
        Self {
            add_reactions: permissions.add_reactions(),
            administrator: permissions.administrator(),
            attach_files: permissions.attach_files(),
            ban_members: permissions.ban_members(),
            change_nickname: permissions.change_nickname(),
            connect: permissions.connect(),
            create_invite: permissions.create_invite(),
            deafen_members: permissions.deafen_members(),
            embed_links: permissions.embed_links(),
            external_emojis: permissions.external_emojis(),
            kick_members: permissions.kick_members(),
            manage_channels: permissions.manage_channels(),
            manage_emojis: permissions.manage_emojis(),
            manage_guild: permissions.manage_guild(),
            manage_messages: permissions.manage_messages(),
            manage_nicknames: permissions.manage_nicknames(),
            manage_roles: permissions.manage_roles(),
            manage_webhooks: permissions.manage_webhooks(),
            mention_everyone: permissions.mention_everyone(),
            move_members: permissions.move_members(),
            mute_members: permissions.mute_members(),
            priority_speaker: permissions.priority_speaker(),
            read_message_history: permissions.read_message_history(),
            request_to_speak: permissions.request_to_speak(),
            read_messages: permissions.read_messages(),
            send_messages: permissions.send_messages(),
            send_tts_messages: permissions.send_tts_messages(),
            speak: permissions.speak(),
            stream: permissions.stream(),
            use_external_emojis: permissions.use_external_emojis(),
            use_slash_commands: permissions.use_slash_commands(),
            use_vad: permissions.use_vad(),
            view_audit_log: permissions.view_audit_log(),
        }
    }
}

impl Permissions {
    pub fn can_manage_server(&self) -> bool {
        self.administrator
            || self.manage_guild
            || self.manage_messages
            || self.manage_roles
    }
}