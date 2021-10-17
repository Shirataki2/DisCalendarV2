/* eslint-disable camelcase */
import { NuxtAxiosInstance } from '@nuxtjs/axios'
const JSONb = require('json-bigint')

export namespace Discord {
  export interface PartialGuild {
    id: bigint
    name: string
    icon?: string
    owner: boolean
    features: Array<string>
    permissions_new: string
    permissions: number
  }

  export interface Role {
    id: bigint
    name: string
    color: number
    hoist: boolean
    position: number
    permissions: string
    managed: boolean
    mentionable: boolean
  }

  export interface Emoji {
    id?: bigint
    name?: string
    roles?: Array<string>
    user?: User
    require_colons?: boolean
    managed?: boolean
    animated?: boolean
    available?: boolean
  }

  export interface User {
    id: bigint
    username: string
    discriminator: string
    avatar: string | null
    bot?: boolean
    system?: boolean
    mfa_enabled?: boolean
    locale?: string
    verified?: boolean
    email?: string | null
    flags?: number
    premium_type?: number
    public_flags?: number
  }

  export interface Member {
    user?: User
    nick?: string
    roles: Array<bigint>
    joined_at: Date
    premium_since?: Date | null
    deaf: boolean
    mute: boolean
  }

  export interface Guild extends PartialGuild {
    splash?: string
    discovery_splash?: string
    owner_id: bigint
    region: string
    afk_channel_id: bigint
    afk_timeout: number
    widget_enabled?: boolean
    widget_channel_id?: bigint
    verification_level: 0 | 1 | 2 | 3 | 4
    default_message_notifications: number
    explicit_content_filter: number
    roles: Array<Role>
    emojis: Array<Emoji>
    mfa_level: number
    application_id?: bigint
    system_channel_id?: bigint
    system_channel_flags: boolean
    rules_channel_id?: bigint
    joined_at?: Date
    large?: boolean
    unavailable?: boolean
    member_count?: number
    voice_states?: Array<any>
    members?: Array<Member>
    channels?: Array<any>
    presences?: Array<any>
    max_presences?: number
    max_members?: number
    vanity_url_code?: string
    description: string | null
    banner: string | null
    premium_tier: 0 | 1 | 2 | 3
    premium_subscription_count?: number
    preferred_locale: string
    public_updates_channel_id?: bigint
    max_video_channel_users?: number
    approximate_member_count?: number
    approximate_presence_count?: number
  }

  export interface MinimalGuild {
    id: bigint
    name: string
    guild_id: bigint
    avatar_url: string
  }

  export interface CheckResult {
    guild: Guild
    result: Array<string>
  }

  export interface RoleCheckResult {
    initialized: boolean
    authenticated: boolean
    authorized: boolean
    roles: Array<string>
    guild?: Guild
  }

  export interface Permissions {
    add_reactions: boolean,
    administrator: boolean,
    attach_files: boolean,
    ban_members: boolean,
    change_nickname: boolean,
    connect: boolean,
    create_invite: boolean,
    deafen_members: boolean,
    embed_links: boolean,
    external_emojis: boolean,
    kick_members: boolean,
    manage_channels: boolean,
    manage_emojis: boolean,
    manage_guild: boolean,
    manage_messages: boolean,
    manage_nicknames: boolean,
    manage_roles: boolean,
    manage_webhooks: boolean,
    mention_everyone: boolean,
    move_members: boolean,
    mute_members: boolean,
    priority_speaker: boolean,
    read_message_history: boolean,
    request_to_speak: boolean,
    read_messages: boolean,
    send_messages: boolean,
    send_tts_messages: boolean,
    speak: boolean,
    stream: boolean,
    use_external_emojis: boolean,
    use_slash_commands: boolean,
    use_vad: boolean,
    view_audit_log: boolean,
  }

  export interface GuildConfig {
    guild_id: string,
    restricted: boolean,
  }

  export interface GuildConfigQuery {
    restricted: boolean,
  }
}

export interface DiscordAPIInterface {
  getServerList: (
    axios: NuxtAxiosInstance
  ) => Promise<Array<Discord.PartialGuild>>

  getBotJoinedServerList: (
    axios: NuxtAxiosInstance,
    guild_ids: Array<bigint>
  ) => Promise<Array<Discord.MinimalGuild>>

  getServer: (
    axios: NuxtAxiosInstance,
    guild_id: string
  ) => Promise<Discord.Guild>

  getUserPermissions: (
    axios: NuxtAxiosInstance,
    guild_id: string,
  ) => Promise<Discord.Permissions>

  getGuildConfig: (
    axios: NuxtAxiosInstance,
    guild_id: string,
  ) => Promise<Discord.GuildConfig>

  setGuildConfig: (
    axios: NuxtAxiosInstance,
    guild_id: string,
    config: Discord.GuildConfigQuery,
  ) => Promise<Discord.GuildConfig>
}

export class DiscordAPI implements DiscordAPIInterface {
  getServerList = async (axios: NuxtAxiosInstance) => {
    const { data } = await axios.get<Array<Discord.PartialGuild>>(
      '/discord/api/users/@me/guilds'
    )
    return data
  }

  getBotJoinedServerList = async (
    axios: NuxtAxiosInstance,
    guild_ids: Array<BigInt>
  ) => {
    const { data } = await axios.get<Array<Discord.MinimalGuild>>(
      '/local/api/guilds/joined',
      {
        params: {
          guild_ids: guild_ids.join(',')
        },
        transformResponse: [(data: any) => JSONb.parse(data)]
      }
    )
    return data
  }

  getServer = async (axios: NuxtAxiosInstance, guild_id: string) => {
    const { data } = await axios.get<Discord.Guild>(
      `/local/api/guilds/check/${guild_id}`, {
        transformResponse: [data => {
          return JSONb.parse(data)
        }]
      }
    )
    return data
  }

  getUserPermissions = async (axios: NuxtAxiosInstance, guild_id: string) => {
    const { data } = await axios.get<Discord.Permissions>(
      `/local/api/guilds/${guild_id}/@me/permissions`, {
        transformResponse: [data => {
          return JSONb.parse(data)
        }]
      }
    )
    return data
  }

  getGuildConfig = async (axios: NuxtAxiosInstance, guild_id: string) => {
    const { data } = await axios.get<Discord.GuildConfig>(
      `/local/api/guilds/${guild_id}/config`, {
        transformResponse: [data => {
          return JSONb.parse(data)
        }]
      }
    )
    return data
  }

  setGuildConfig = async (axios: NuxtAxiosInstance, guild_id: string, config: Discord.GuildConfigQuery) => {
    const { data } = await axios.put<Discord.GuildConfig>(
      `/local/api/guilds/${guild_id}/config`, {
        ...config
      }
    )
    return data
  }
}
