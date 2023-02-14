//! The data types sent to and returned from the API client.
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// JSON template for Group resource
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct Groups {
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "allowExternalMembers"
    )]
    pub allow_external_members: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "allowGoogleCommunication"
    )]
    pub allow_google_communication: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "allowWebPosting"
    )]
    pub allow_web_posting: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "archiveOnly"
    )]
    pub archive_only: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "customFooterText"
    )]
    pub custom_footer_text: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "customReplyTo"
    )]
    pub custom_reply_to: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "customRolesEnabledForSettingsToBeMerged"
    )]
    pub custom_roles_enabled_for_settings_to_be_merged: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "defaultMessageDenyNotificationText"
    )]
    pub default_message_deny_notification_text: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub default_sender: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "enableCollaborativeInbox"
    )]
    pub enable_collaborative_inbox: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "favoriteRepliesOnTop"
    )]
    pub favorite_replies_on_top: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "includeCustomFooter"
    )]
    pub include_custom_footer: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "includeInGlobalAddressList"
    )]
    pub include_in_global_address_list: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "isArchived"
    )]
    pub is_archived: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub kind: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize",
        rename = "maxMessageBytes"
    )]
    pub max_message_bytes: i64,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "membersCanPostAsTheGroup"
    )]
    pub members_can_post_as_the_group: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "messageDisplayFont"
    )]
    pub message_display_font: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "messageModerationLevel"
    )]
    pub message_moderation_level: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "primaryLanguage"
    )]
    pub primary_language: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "replyTo"
    )]
    pub reply_to: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "sendMessageDenyNotification"
    )]
    pub send_message_deny_notification: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "showInGroupDirectory"
    )]
    pub show_in_group_directory: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "spamModerationLevel"
    )]
    pub spam_moderation_level: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "whoCanAdd"
    )]
    pub who_can_add: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "whoCanAddReferences"
    )]
    pub who_can_add_references: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "whoCanApproveMembers"
    )]
    pub who_can_approve_members: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "whoCanApproveMessages"
    )]
    pub who_can_approve_messages: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "whoCanAssignTopics"
    )]
    pub who_can_assign_topics: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "whoCanAssistContent"
    )]
    pub who_can_assist_content: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "whoCanBanUsers"
    )]
    pub who_can_ban_users: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "whoCanContactOwner"
    )]
    pub who_can_contact_owner: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "whoCanDeleteAnyPost"
    )]
    pub who_can_delete_any_post: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "whoCanDeleteTopics"
    )]
    pub who_can_delete_topics: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "whoCanDiscoverGroup"
    )]
    pub who_can_discover_group: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "whoCanEnterFreeFormTags"
    )]
    pub who_can_enter_free_form_tags: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "whoCanHideAbuse"
    )]
    pub who_can_hide_abuse: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "whoCanInvite"
    )]
    pub who_can_invite: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "whoCanJoin"
    )]
    pub who_can_join: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "whoCanLeaveGroup"
    )]
    pub who_can_leave_group: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "whoCanLockTopics"
    )]
    pub who_can_lock_topics: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "whoCanMakeTopicsSticky"
    )]
    pub who_can_make_topics_sticky: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "whoCanMarkDuplicate"
    )]
    pub who_can_mark_duplicate: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "whoCanMarkFavoriteReplyOnAnyTopic"
    )]
    pub who_can_mark_favorite_reply_on_any_topic: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "whoCanMarkFavoriteReplyOnOwnTopic"
    )]
    pub who_can_mark_favorite_reply_on_own_topic: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "whoCanMarkNoResponseNeeded"
    )]
    pub who_can_mark_no_response_needed: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "whoCanModerateContent"
    )]
    pub who_can_moderate_content: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "whoCanModerateMembers"
    )]
    pub who_can_moderate_members: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "whoCanModifyMembers"
    )]
    pub who_can_modify_members: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "whoCanModifyTagsAndCategories"
    )]
    pub who_can_modify_tags_and_categories: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "whoCanMoveTopicsIn"
    )]
    pub who_can_move_topics_in: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "whoCanMoveTopicsOut"
    )]
    pub who_can_move_topics_out: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "whoCanPostAnnouncements"
    )]
    pub who_can_post_announcements: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "whoCanPostMessage"
    )]
    pub who_can_post_message: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "whoCanTakeTopics"
    )]
    pub who_can_take_topics: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "whoCanUnassignTopic"
    )]
    pub who_can_unassign_topic: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "whoCanUnmarkFavoriteReplyOnAnyTopic"
    )]
    pub who_can_unmark_favorite_reply_on_any_topic: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "whoCanViewGroup"
    )]
    pub who_can_view_group: String,
    /**
     * JSON template for Group resource
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "whoCanViewMembership"
    )]
    pub who_can_view_membership: String,
}

/**
* Data format for the response.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub enum Alt {
    #[serde(rename = "atom")]
    Atom,
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Alt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Alt::Atom => "atom",
            Alt::Json => "json",
            Alt::Noop => "",
            Alt::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Alt {
    fn default() -> Alt {
        Alt::Noop
    }
}
impl Alt {
    pub fn is_noop(&self) -> bool {
        matches!(self, Alt::Noop)
    }
}
