use std::sync::Arc;

use conundrum::{
    ai::models::{
        agent::agent_description::AgentDescription,
        chat::{
            chat_conversation::chat_conversation::ChatConversation,
            chat_message::{
                ai::{ai_message::AIMessage, reasoning_block::ReasoningBlock},
                chat_message::ChatMessage,
                system::system_prompt_message::SystemPromptMessage,
                user::user_message::UserMessage,
            },
        },
        tool::tool_execution::ToolExecution,
    },
    ecosystem::{
        db::{db::get_database, db_traits::db_entity::DBSchema, tables::DatabaseTable},
        error_handling::db_error::{DatabaseError, DatabaseResult},
    },
};
use lancedb::{Table, arrow::arrow_schema::Schema};
use log::warn;

use crate::vector::{
    database::inititialize_db::seed_db::seed_db,
    models::{
        academic::{
            assignment::{
                academic_assignment_entity::AssignmentEntity,
                assignment_subject::AssignmentSubject,
                assignment_tag::AssignmentTag,
                assignment_topic::AssignmentTopic,
                milestone::{milestone_alarm::MilestoneAlarm, milestone_entity::MilestoneEntity},
            },
            question::flashcard::flashcard_entity::FlashCardEntity,
            result::academic_result_metric::AcademicResultMetric,
        },
        ai::tool::mcp_tool_record::MCPToolRecord,
        bib::bib_entry::BibEntryModel,
        date_time::alarm::alarm::Alarm,
        ecosystem_data::{
            documentation::documentation_chunk::DocumentationChunk,
            ecosystem_application_settings::keyboard_shortcut::KeyboardShortcut,
            ecosytem_setting_types::ecosystem_setting_model::EcosystemSettingModel, log::ecosystem_log::EcosystemLog,
            server_state::server_state::ServerState,
        },
        git::git_repository_entity::GitRepositoryEntity,
        lifestyle::life_connections::models::{
            long_term_goal::LongTermGoal, phone_contact::PhoneContact, physical_address_type::PhysicalAddressType,
            physical_street_address::PhysicalStreetAddress, short_term_goal::ShortTermGoal,
        },
        meta::front_matter::front_matter::FrontMatter,
        notebook::{notebook_cell_chunk::NotebookCellChunk, notebook_model::NotebookModel},
        pdf::pdf_model::PdfModel,
        taggables::{auto_taggable::AutoTaggable, subject::Subject, tag::Tag, topic::Topic},
        text::{
            cdrm::{cdrm_chunk::CdrmChunk, cdrm_model::CdrmModel},
            html::html_content_model::HTMLModel,
            text_based_content::text_based_chunk::TextBasedChunk,
            typst::{typst_content::TypstContent, typst_model::TypstModel},
        },
        workspace::user_workspace::UserWorkspace,
    },
};

pub type DatabaseIndexSetupFunction = fn(&Table) -> DatabaseResult<()>;

struct TableInitData {
    pub table: DatabaseTable,
    pub schema: Arc<Schema>,
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
          log::error!("Create Table Error: {:#?}", e);
          DatabaseError::FailToCreateTable(table.clone())
      })
}

pub async fn initialize_local_database(state: &Arc<ServerState>) -> DatabaseResult<()> {
    let table_data: Vec<TableInitData> = vec![TableInitData { table: DatabaseTable::MCPToolRecord,
                             schema: <MCPToolRecord as DBSchema>::schema().map(Arc::new)?,
                             set_indices: None },
             TableInitData { table: DatabaseTable::DocumentationChunk,
                             schema: <DocumentationChunk as DBSchema>::schema().map(Arc::new)?,
                             set_indices: None },
             TableInitData { table: DatabaseTable::EcosystemSetting,
                             schema: <EcosystemSettingModel as DBSchema>::schema().map(Arc::new)?,
                             set_indices: None },
             TableInitData { table: DatabaseTable::AgentDescription,
                             schema: <AgentDescription as DBSchema>::schema().map(Arc::new)?,
                             set_indices: None },
             TableInitData { table: DatabaseTable::ChatConversation,
                             schema: <ChatConversation as DBSchema>::schema().map(Arc::new)?,
                             set_indices: None },
             TableInitData { table: DatabaseTable::UserMessage,
                             schema: <UserMessage as DBSchema>::schema().map(Arc::new)?,
                             set_indices: None },
             TableInitData { table: DatabaseTable::AgentMessage,
                             schema: <AIMessage as DBSchema>::schema().map(Arc::new)?,
                             set_indices: None },
             TableInitData { table: DatabaseTable::SystemPromptMessage,
                             schema: <SystemPromptMessage as DBSchema>::schema().map(Arc::new)?,
                             set_indices: None },
             TableInitData { table: DatabaseTable::AgentReasoning,
                             schema: <ReasoningBlock as DBSchema>::schema().map(Arc::new)?,
                             set_indices: None },
             TableInitData { table: DatabaseTable::ToolExecution,
                             schema: <ToolExecution as DBSchema>::schema().map(Arc::new)?,
                             set_indices: None },
             TableInitData { table: DatabaseTable::EcosystemLog,
                             schema: <EcosystemLog as DBSchema>::schema().map(Arc::new)?,
                             set_indices: None },
             TableInitData { table: DatabaseTable::KeyboardShortcut,
                             schema: <KeyboardShortcut as DBSchema>::schema().map(Arc::new)?,
                             set_indices: None },
             TableInitData { table: DatabaseTable::Tag,
                             schema: <Tag as DBSchema>::schema().map(Arc::new)?,
                             set_indices: None },
             TableInitData { table: DatabaseTable::Topic,
                             schema: <Topic as DBSchema>::schema().map(Arc::new)?,
                             set_indices: None },
             TableInitData { table: DatabaseTable::Subject,
                             schema: <Subject as DBSchema>::schema().map(Arc::new)?,
                             set_indices: None },
             TableInitData { table: DatabaseTable::AutoTaggable,
                             schema: <AutoTaggable as DBSchema>::schema().map(Arc::new)?,
                             set_indices: None },
             TableInitData { table: DatabaseTable::UserWorkspace,
                             schema: <UserWorkspace as DBSchema>::schema().map(Arc::new)?,
                             set_indices: None },
             TableInitData { table: DatabaseTable::Cdrm,
                             schema: <CdrmModel as DBSchema>::schema().map(Arc::new)?,
                             set_indices: None },
             TableInitData { table: DatabaseTable::FrontMatter,
                             schema: <FrontMatter as DBSchema>::schema().map(Arc::new)?,
                             set_indices: None },
             TableInitData { table: DatabaseTable::Milestone,
                             schema: <MilestoneEntity as DBSchema>::schema().map(Arc::new)?,
                             set_indices: None },
             TableInitData { table: DatabaseTable::MilestoneAlarm,
                             schema: <MilestoneAlarm as DBSchema>::schema().map(Arc::new)?,
                             set_indices: None },
             TableInitData { table: DatabaseTable::Assignment,
                             schema: <AssignmentEntity as DBSchema>::schema().map(Arc::new)?,
                             set_indices: None },
             TableInitData { table: DatabaseTable::AssignmentTopic,
                             schema: <AssignmentTopic as DBSchema>::schema().map(Arc::new)?,
                             set_indices: None },
             TableInitData { table: DatabaseTable::AssignmentSubject,
                             schema: <AssignmentSubject as DBSchema>::schema().map(Arc::new)?,
                             set_indices: None },
             TableInitData { table: DatabaseTable::AssignmentTag,
                             schema: <AssignmentTag as DBSchema>::schema().map(Arc::new)?,
                             set_indices: None },
             TableInitData { table: DatabaseTable::QAPair,
                             schema: <FlashCardEntity as DBSchema>::schema().map(Arc::new)?,
                             set_indices: None },
             TableInitData { table: DatabaseTable::GitRepository,
                             schema: <GitRepositoryEntity as DBSchema>::schema().map(Arc::new)?,
                             set_indices: None },
             TableInitData { table: DatabaseTable::Alarm,
                             schema: <Alarm as DBSchema>::schema().map(Arc::new)?,
                             set_indices: None },
             TableInitData { table: DatabaseTable::BibEntry,
                             schema: <BibEntryModel as DBSchema>::schema().map(Arc::new)?,
                             set_indices: None },
             TableInitData { table: DatabaseTable::CdrmChunk,
                             schema: <CdrmChunk as DBSchema>::schema().map(Arc::new)?,
                             set_indices: None },
             TableInitData { table: DatabaseTable::HTML,
                             schema: <HTMLModel as DBSchema>::schema().map(Arc::new)?,
                             set_indices: None },
             TableInitData { table: DatabaseTable::LongTermGoal,
                             schema: <LongTermGoal as DBSchema>::schema().map(Arc::new)?,
                             set_indices: None },
             TableInitData { table: DatabaseTable::Notebook,
                             schema: <NotebookModel as DBSchema>::schema().map(Arc::new)?,
                             set_indices: None },
             TableInitData { table: DatabaseTable::NotebookCellChunk,
                             schema: <NotebookCellChunk as DBSchema>::schema().map(Arc::new)?,
                             set_indices: None },
             TableInitData { table: DatabaseTable::Pdf,
                             schema: <PdfModel as DBSchema>::schema().map(Arc::new)?,
                             set_indices: None },
             TableInitData { table: DatabaseTable::PhoneContact,
                             schema: <PhoneContact as DBSchema>::schema().map(Arc::new)?,
                             set_indices: None },
             TableInitData { table: DatabaseTable::ShortTermGoal,
                             schema: <ShortTermGoal as DBSchema>::schema().map(Arc::new)?,
                             set_indices: None },
             TableInitData { table: DatabaseTable::StreetAddress,
                             schema: <PhysicalStreetAddress as DBSchema>::schema().map(Arc::new)?,
                             set_indices: None },
             TableInitData { table: DatabaseTable::Typst,
                             schema: <TypstModel as DBSchema>::schema().map(Arc::new)?,
                             set_indices: None },];
    let db_arc = get_database().await?;
    let db = db_arc.inner_arc().lock_owned().await;

    for td in table_data.iter() {
        log::info!("Initializing the {} table for the {} model", td.table, td.table.to_model_name());
        if !td.table.is_temporary_vector_table() {
            let arc_schema = Arc::clone(&td.schema);
            match create_table(&db, &arc_schema, &td.table).await {
                Err(e) => {
                    let s = td.table.to_model_name();
                    warn!("Conundrum failed while attempting to generate a database table for the `{:?}` model.", s);
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
    drop(db);
    log::info!("Conundrum successfully initialized {} tables.", table_data.len());
    let _ = seed_db(db_arc.clone(), Arc::clone(state)).await.inspect_err(|e| {
                                                                log::error!("Error: {:#?}", e);
                                                            });
    log::info!("Conundrum successfully seeded your database with documentation and some initial settings. Add a workspace to start adding actually meaningful content to your database.");
    Ok(())
}
