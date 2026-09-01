use std::sync::Arc;

use crate::crud_router;
use crate::rpc::routers::crud::expanded::chat_conversation_crud::get_chat_conversation_crud;
use conundrum::ai::models::agent::agent_description::AgentDescription;
use conundrum::ecosystem::db::db_traits::db_entity::DBSchema;
use conundrum::ecosystem::error_handling::server_error::ServerError;
use conundrum::{
    ai::models::{
        chat::{
            chat_conversation::chat_conversation::ChatConversation,
            chat_message::{
                ai::ai_message::AIMessage, system::system_prompt_message::SystemPromptMessage,
                user::user_message::UserMessage,
            },
        },
        tool::tool_execution::ToolExecution,
    },
    ecosystem::db::{db_traits::entity_crud::EntityCRUD, tables::DatabaseTable},
};
use conundrum_db::vector::models::{
    academic::{
        assignment::academic_assignment_entity::AssignmentEntity,
        question::flashcard::flashcard_entity::FlashCardEntity,
    },
    ecosystem_data::{
        ecosystem_application_settings::keyboard_shortcut::KeyboardShortcut, server_state::server_state::ServerState,
    },
    git::git_repository_entity::GitRepositoryEntity,
    taggables::{auto_taggable::AutoTaggable, subject::Subject, tag::Tag, topic::Topic},
    workspace::user_workspace::UserWorkspace,
};
use rspc::{Procedure, Router};

pub fn get_nested_crud_router() -> Router<Arc<ServerState>> {
    let workspace_crud = crud_router!(UserWorkspace, <UserWorkspace as DBSchema>::PartialUpdateType);
    let tag_crud = crud_router!(Tag, <Tag as DBSchema>::PartialUpdateType);
    let topic_crud = crud_router!(Topic, <Topic as DBSchema>::PartialUpdateType);
    let subject_crud = crud_router!(Subject, <Subject as DBSchema>::PartialUpdateType);
    let git_repo_crud = crud_router!(GitRepositoryEntity, <GitRepositoryEntity as DBSchema>::PartialUpdateType);
    let auto_taggable_crud = crud_router!(AutoTaggable, <AutoTaggable as DBSchema>::PartialUpdateType);
    let assignment_crud = crud_router!(AssignmentEntity, <AssignmentEntity as DBSchema>::PartialUpdateType);
    let flashcard_crud = crud_router!(FlashCardEntity, <FlashCardEntity as DBSchema>::PartialUpdateType);
    let keyboard_shortcut_crud = crud_router!(KeyboardShortcut, <KeyboardShortcut as DBSchema>::PartialUpdateType);
    let user_message_crud = crud_router!(UserMessage, <UserMessage as DBSchema>::PartialUpdateType);
    let ai_message_crud = crud_router!(AIMessage, <AIMessage as DBSchema>::PartialUpdateType);
    let system_prompt_message_crud =
        crud_router!(SystemPromptMessage, <SystemPromptMessage as DBSchema>::PartialUpdateType);
    let chat_conversation_crud = get_chat_conversation_crud();
    let agent_description_crud = crud_router!(AgentDescription, <AgentDescription as DBSchema>::PartialUpdateType);
    let tool_execution_crud = crud_router!(ToolExecution, ToolExecution);
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
                                     .nest(DatabaseTable::UserMessage.to_string(), user_message_crud)
                                     .nest(DatabaseTable::AgentMessage.to_string(), ai_message_crud)
                                     .nest(DatabaseTable::SystemPromptMessage.to_string(), system_prompt_message_crud)
                                     .nest(DatabaseTable::ToolExecution.to_string(), tool_execution_crud)
}
