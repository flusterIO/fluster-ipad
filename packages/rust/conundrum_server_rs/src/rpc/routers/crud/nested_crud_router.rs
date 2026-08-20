use std::sync::Arc;

use crate::crud_router;
use conundrum::ai::models::agent::agent_description::AgentDescription;
use conundrum::ai::models::agent::agent_description_partial::AgentDescriptionPartial;
use conundrum::ecosystem::error_handling::server_error::{ServerError, ServerResult};
use conundrum::{
    ai::models::chat::{
        chat_conversation::{chat_conversation::ChatConversation, chat_conversation_partial::ChatConversationPartial},
        chat_message::chat_message::ChatMessage,
    },
    ecosystem::db::{db_traits::entity_crud::EntityCRUD, tables::DatabaseTable},
};
use conundrum_db::vector::models::{
    academic::{
        assignment::{
            academic_assignment_entity::AssignmentEntity, academic_assignment_entity_partial::AssignmentEntityPartial,
        },
        question::flashcard::{flashcard_entity::FlashCardEntity, flashcard_entity_partial::FlashCardEntityPartial},
    },
    ecosystem_data::{
        ecosystem_application_settings::{
            keyboard_shortcut::KeyboardShortcut, keyboard_shortcut_partial::KeyboardShortcutPartial,
        },
        server_state::server_state::ServerState,
    },
    git::{git_repository_entity::GitRepositoryEntity, git_repository_partial::GitRepositoryPartial},
    primitives::helper_models::label_and_id::IDAndOptionalLabel,
    taggables::{
        auto_taggable::AutoTaggable, auto_taggable_partial::AutoTaggablePartial, subject::Subject, tag::Tag,
        taggable_update_partial::TaggablePartial, topic::Topic,
    },
    workspace::{user_workspace::UserWorkspace, user_workspace_partial::UserWorkspacePartial},
};
use rspc::{Procedure, Router};

pub fn get_nested_crud_router() -> Router<Arc<ServerState>> {
    let workspace_crud = crud_router!(UserWorkspace, UserWorkspacePartial);
    let tag_crud = crud_router!(Tag, TaggablePartial);
    let topic_crud = crud_router!(Topic, TaggablePartial);
    let subject_crud = crud_router!(Subject, TaggablePartial);
    let git_repo_crud = crud_router!(GitRepositoryEntity, GitRepositoryPartial);
    let auto_taggable_crud = crud_router!(AutoTaggable, AutoTaggablePartial);
    let assignment_crud = crud_router!(AssignmentEntity, AssignmentEntityPartial);
    let flashcard_crud = crud_router!(FlashCardEntity, FlashCardEntityPartial);
    let keyboard_shortcut_crud = crud_router!(KeyboardShortcut, KeyboardShortcutPartial);
    let chat_conversation_crud = crud_router!(ChatConversation, ChatConversationPartial);
    let chat_message_crud = crud_router!(ChatMessage, ChatMessage);
    let agent_description_crud = crud_router!(AgentDescription, AgentDescriptionPartial);
    Router::<Arc<ServerState>>::new().nest(DatabaseTable::UserWorkspace.to_string(), workspace_crud)
                                     .nest(DatabaseTable::GitRepository.to_string(), git_repo_crud)
                                     .nest(DatabaseTable::Topic.to_string(), topic_crud)
                                     .nest(DatabaseTable::Subject.to_string(), subject_crud)
                                     .nest(DatabaseTable::Tag.to_string(), tag_crud)
                                     .nest(DatabaseTable::AutoTaggable.to_string(), auto_taggable_crud)
                                     .nest(DatabaseTable::Assignment.to_string(), assignment_crud)
                                     .nest(DatabaseTable::KeyboardShortcut.to_string(), keyboard_shortcut_crud)
                                     .nest(DatabaseTable::QAPair.to_string(), flashcard_crud)
                                     .nest(DatabaseTable::AgentDescription.to_string(), agent_description_crud)
                                     .nest(DatabaseTable::ChatConversation.to_string(), chat_conversation_crud)
                                     .nest(DatabaseTable::UserMessage.to_string(), chat_message_crud)
}
