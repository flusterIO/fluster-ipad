use crate::{crud_router, errors::server_error::ServerError, rpc::route_context::RouteContext};

use conundrum::ecosystem::db::tables::DatabaseTable;
use conundrum_db::vector::{
    database::db_traits::entity_crud::EntityCRUD,
    models::{
        academic::{
            assignment::{
                academic_assignment_entity::AssignmentEntity,
                academic_assignment_entity_partial::AssignmentEntityPartial,
            },
            question::flashcard::{
                flashcard_entity::FlashCardEntity, flashcard_entity_partial::FlashCardEntityPartial,
            },
        },
        git::{git_repository_entity::GitRepositoryEntity, git_repository_partial::GitRepositoryPartial},
        taggables::{
            auto_taggable::AutoTaggable, auto_taggable_partial::AutoTaggablePartial, subject::Subject, tag::Tag,
            taggable_update_partial::TaggablePartial, topic::Topic,
        },
        workspace::{user_workspace::UserWorkspace, user_workspace_partial::UserWorkspacePartial},
    },
    parameters::predicate_query_params::PredicateQueryParams,
};
use rspc::{Procedure, Router};

pub fn get_nested_crud_router() -> Router<RouteContext> {
    let workspace_crud = crud_router!(UserWorkspace, UserWorkspacePartial);
    let tag_crud = crud_router!(Tag, TaggablePartial);
    let topic_crud = crud_router!(Topic, TaggablePartial);
    let subject_crud = crud_router!(Subject, TaggablePartial);
    let git_repo_crud = crud_router!(GitRepositoryEntity, GitRepositoryPartial);
    let auto_taggable_crud = crud_router!(AutoTaggable, AutoTaggablePartial);
    let assignment_crud = crud_router!(AssignmentEntity, AssignmentEntityPartial);
    let flashcard_crud = crud_router!(FlashCardEntity, FlashCardEntityPartial);
    Router::<RouteContext>::new().nest(DatabaseTable::UserWorkspace.to_string(), workspace_crud)
                                 .nest(DatabaseTable::GitRepository.to_string(), git_repo_crud)
                                 .nest(DatabaseTable::Topic.to_string(), topic_crud)
                                 .nest(DatabaseTable::Subject.to_string(), subject_crud)
                                 .nest(DatabaseTable::Tag.to_string(), tag_crud)
                                 .nest(DatabaseTable::AutoTaggable.to_string(), auto_taggable_crud)
                                 .nest(DatabaseTable::Assignment.to_string(), assignment_crud)
                                 .nest(DatabaseTable::QAPair.to_string(), flashcard_crud)
}
