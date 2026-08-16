use std::sync::Arc;

use conundrum::ecosystem::{
    db::{tables::DatabaseTable, traits::db_entity::DBSchema},
    error_handling::db_error::{DatabaseError, DatabaseResult},
};
use conundrum_fs::path_utils::ecosystem_paths::get_app_database_dir;
use lancedb::{Table, arrow::arrow_schema::Schema, connect};
use log::warn;

use crate::vector::models::{
    academic::{
        assignment::{
            academic_assignment_entity::AssignmentEntity,
            assignment_subject::AssignmentSubject,
            assignment_tag::AssignmentTag,
            assignment_topic::AssignmentTopic,
            milestone::{milestone_alarm::MilestoneAlarm, milestone_entity::MilestoneEntity},
        },
        question::flashcard::flashcard_entity::FlashCardEntity,
    },
    ai::{
        agent::agent_description::AgentDescription,
        chat::{chat_conversation::chat_conversation::ChatConversation, chat_message::chat_message::ChatMessage},
        tool::mcp_tool_record::MCPToolRecord,
    },
    ecosystem_data::{
        ecosystem_application_settings::keyboard_shortcut::KeyboardShortcut, log::ecosystem_log::EcosystemLog,
    },
    git::git_repository_entity::GitRepositoryEntity,
    taggables::{auto_taggable::AutoTaggable, subject::Subject, tag::Tag, topic::Topic},
    text::cdrm::{cdrm_content::CdrmContent, cdrm_model::CdrmModel},
    workspace::user_workspace::UserWorkspace,
};

pub type DatabaseIndexSetupFunction = fn(&Table) -> DatabaseResult<()>;

struct TableInitData {
    pub table: DatabaseTable,
    pub schema: Schema,
    /// An optional function called after the table is created so indices can be
    /// applied.
    pub set_indices: Option<DatabaseIndexSetupFunction>,
}

async fn create_table(db: &lancedb::Connection, schema: &Arc<Schema>, table: &DatabaseTable) -> DatabaseResult<Table> {
    db.create_empty_table(table.to_string(), schema.clone())
      .mode(lancedb::database::CreateTableMode::Create)
      .execute()
      .await
      .map_err(|e| {
          log::error!("Create Table Error: {:?}", e);
          DatabaseError::FailToCreateTable(table.clone())
      })
}

pub async fn initialize_local_database() -> DatabaseResult<()> {
    let table_data: Vec<TableInitData> = vec![TableInitData { table: DatabaseTable::MCPToolRecord,
                                                              schema: MCPToolRecord::schema()?,
                                                              set_indices: None },
                                              TableInitData { table: DatabaseTable::AgentDescription,
                                                              schema: AgentDescription::schema()?,
                                                              set_indices: None },
                                              TableInitData { table: DatabaseTable::ChatConversation,
                                                              schema: ChatConversation::schema()?,
                                                              set_indices: None },
                                              TableInitData { table: DatabaseTable::ChatMessage,
                                                              schema: ChatMessage::schema()?,
                                                              set_indices: None },
                                              TableInitData { table: DatabaseTable::EcosystemLog,
                                                              schema: EcosystemLog::schema()?,
                                                              set_indices: None },
                                              TableInitData { table: DatabaseTable::KeyboardShortcut,
                                                              schema: KeyboardShortcut::schema()?,
                                                              set_indices: None },
                                              TableInitData { table: DatabaseTable::Tag,
                                                              schema: Tag::schema()?,
                                                              set_indices: None },
                                              TableInitData { table: DatabaseTable::Topic,
                                                              schema: Topic::schema()?,
                                                              set_indices: None },
                                              TableInitData { table: DatabaseTable::Subject,
                                                              schema: Subject::schema()?,
                                                              set_indices: None },
                                              TableInitData { table: DatabaseTable::AutoTaggable,
                                                              schema: AutoTaggable::schema()?,
                                                              set_indices: None },
                                              TableInitData { table: DatabaseTable::UserWorkspace,
                                                              schema: UserWorkspace::schema()?,
                                                              set_indices: None },
                                              TableInitData { table: DatabaseTable::Cdrm,
                                                              schema: CdrmModel::schema()?,
                                                              set_indices: None },
                                              TableInitData { table: DatabaseTable::Milestone,
                                                              schema: MilestoneEntity::schema()?,
                                                              set_indices: None },
                                              TableInitData { table: DatabaseTable::MilestoneAlarm,
                                                              schema: MilestoneAlarm::schema()?,
                                                              set_indices: None },
                                              TableInitData { table: DatabaseTable::Assignment,
                                                              schema: AssignmentEntity::schema()?,
                                                              set_indices: None },
                                              TableInitData { table: DatabaseTable::AssignmentTopic,
                                                              schema: AssignmentTopic::schema()?,
                                                              set_indices: None },
                                              TableInitData { table: DatabaseTable::AssignmentSubject,
                                                              schema: AssignmentSubject::schema()?,
                                                              set_indices: None },
                                              TableInitData { table: DatabaseTable::AssignmentTag,
                                                              schema: AssignmentTag::schema()?,
                                                              set_indices: None },
                                              TableInitData { table: DatabaseTable::QAPair,
                                                              schema: FlashCardEntity::schema()?,
                                                              set_indices: None },
                                              TableInitData { table: DatabaseTable::GitRepository,
                                                              schema: GitRepositoryEntity::schema()?,
                                                              set_indices: None },];
    if let Ok(db_path) = get_app_database_dir() {
        let db = connect(db_path.to_str().unwrap()).execute().await.map_err(|e| {
                                                                        println!("Error in initialize_database: {:?}",
                                                                                 e);
                                                                        DatabaseError::FailToConnect
                                                                    })?;

        for td in table_data.iter() {
            log::info!("Initializing the {} table for the {} model", td.table, td.table.to_model_name());
            if !td.table.is_temporary_vector_table() {
                let arc_schema = Arc::new(td.schema.clone());
                match create_table(&db, &arc_schema, &td.table).await {
                    Err(e) => {
                        let s = td.table.to_model_name();
                        warn!("Conundrum failed while attempting to generate a database table for the `{:?}` model.",
                              s);
                    }
                    Ok(r) => {
                        if let Some(si) = td.set_indices {
                            si(&r)?;
                        }
                    }
                }
            } else {
                log::info!("Ignoring initialization of temporary vector table {:?}", td.table.to_string());
            }
        }
        Ok(())
    } else {
        log::error!("Failed to locate data directory. Cannot continue.");
        Err(DatabaseError::FailToConnect)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test(tokio::test)]
    async fn initializes_database() {
        initialize_local_database().await
                                   .inspect_err(|e| {
                                       log::error!("Error: {:?}", e);
                                   })
                                   .expect("Initializes database.")
    }
}
