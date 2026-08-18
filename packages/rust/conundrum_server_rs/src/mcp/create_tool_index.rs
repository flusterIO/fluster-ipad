use arrow_array::{RecordBatch, RecordBatchIterator};
use conundrum::{
    ai::rig::rig_client_remote::RigClientRemote,
    ecosystem::{
        db::{db::ArcMutexDB, db_traits::db_entity::DBSchema, tables::DatabaseTable},
        error_handling::{
            db_error::DatabaseError,
            server_error::{ServerError, ServerResult},
        },
    },
};
use conundrum_db::vector::models::{
    ai::tool::{mcp_tool_record::MCPToolRecord, tool_definition_list::ToolDefinitionList},
    vector::vector::DB_VECTOR_DIMENSIONS,
};
use indoc::formatdoc;
use rig::{client::EmbeddingsClient, embeddings::EmbeddingModel};
use rig_lancedb::{LanceDbVectorIndex, SearchParams};
use serde_arrow::to_record_batch;
use std::sync::Arc;

pub async fn create_tool_index(db: &ArcMutexDB) -> ServerResult<()> {
    let tool_list = ToolDefinitionList::new_all_tools();
    let client = RigClientRemote::initialize().map_err(|e| {
                                                  let e: DatabaseError = e.into();
                                                  e
                                              })?;
    // TODO: Get the user's settings from the DB here if they exist and select the
    // proper model.
    let embedding_model = client.0.embedding_model_with_ndims("qwen3-embedding:4b", DB_VECTOR_DIMENSIONS as usize);

    let mut tool_records: Vec<MCPToolRecord> = Vec::new();

    for tool in tool_list.0 {
        let schema_json = serde_json::to_string(&tool.input_schema).map_err(|e| {
                              log::error!("Error: {:?}", e);
                              ServerError::SerializationError("Tool Input Schema".to_string())
                          })?;
        let desc = tool.description.clone();

        let embedding_text = formatdoc! {"
        Tool Name: {}
        Description: {}
        Parameters: {}
        ", tool.name, desc.unwrap_or_default(), schema_json};
        let embedding = embedding_model.embed_text(&embedding_text).await.map_err(|e| {
                                                                              log::error!("Embedding Error: {:?}", e);
                                                                              ServerError::EmbeddingError
                                                                          })?;

        let record = MCPToolRecord::from_tool_and_embedding(tool, schema_json, embedding.vec);
        tool_records.push(record);
    }

    let table_name = DatabaseTable::MCPToolRecord.to_string();
    let _db = db.clone().lock_owned().await;
    let tool_record_schema = MCPToolRecord::schema()?;
    let arc_schema = Arc::new(tool_record_schema);
    let _table = _db.create_empty_table(table_name, arc_schema.clone())
                    .mode(lancedb::database::CreateTableMode::Overwrite)
                    .execute()
                    .await
                    .map_err(|e| {
                        log::error!("Table Generation Error: {:?}", e);
                        ServerError::DatabaseError(DatabaseError::FailToCreateTable(DatabaseTable::MCPToolRecord))
                    })?;
    let tool_fields = MCPToolRecord::arrow_fields()?;
    let record_batch = to_record_batch(&tool_fields, &tool_records).map_err(|e| {
                                                                       log::error!("Error: {:?}", e);
                                                                       DatabaseError::SerializationError
                                                                   })?;
    let stream = Box::new(RecordBatchIterator::new(vec![Ok(record_batch)].into_iter(), arc_schema.clone()));
    _table.merge_insert(&["name"])
          .when_matched_update_all(None)
          .when_not_matched_insert_all()
          .clone()
          .execute(stream)
          .await
          .map_err(|e| {
              log::error!("Error: {:?}", e);
              DatabaseError::SerializationError
          })?;
    LanceDbVectorIndex::new(_table, embedding_model, "vector", SearchParams::default()).await
        .map_err(|e| {
            log::error!("Failed to generate vector index: {:?}", e);
            ServerError::EmbeddingError
        })?;

    log::info!("Successfully created an MCP tool index vector store. AI now has access to a growing list of tools that it can query and access dynamically.");
    Ok(())
}
