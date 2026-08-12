use conundrum::ecosystem::{db::tables::DatabaseTable, error_handling::db_error::DatabaseError};
use conundrum_db::vector::{
    database::db::ArcMutexDB,
    models::{ai::tool::mcp_tool_record::MCPToolRecord, vector::vector::DB_VECTOR_DIMENSIONS},
};
use indoc::formatdoc;
use lancedb::Table;
use lancedb::table::WriteOptions;
use rig::{client::EmbeddingsClient, embeddings::EmbeddingModel};
use rig_lancedb::{LanceDbVectorIndex, SearchParams};

use crate::{
    errors::server_error::{ServerError, ServerResult},
    mcp::tools::tool_list::tool_list::ToolList,
    rig::rig_client::RigClient,
};

pub async fn create_tool_index(db: &ArcMutexDB) -> ServerResult<()> {
    let tool_list = ToolList::all_tools();
    let client = RigClient::initialize()?;
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
        ", tool.name, desc, schema_json};
        let embedding = embedding_model.embed_text(&embedding_text).await.map_err(|e| {
                                                                              log::error!("Embedding Error: {:?}", e);
                                                                              ServerError::EmbeddingError
                                                                          })?;

        let record = MCPToolRecord::from_tool_and_embedding(tool, schema_json, embedding.vec);
        tool_records.push(record);
    }

    let table_name = DatabaseTable::MCPToolRecord.to_string();
    let _db = db.clone().lock_owned().await;
    let _table = _db.create_table(table_name, tool_records)
                    .mode(lancedb::database::CreateTableMode::Overwrite)
                    .execute()
                    .await
                    .map_err(|e| {
                        log::error!("Table Generation Error: {:?}", e);
                        ServerError::DatabaseError(DatabaseError::FailToCreateTable(DatabaseTable::MCPToolRecord))
                    })?;
    let vector_index = LanceDbVectorIndex::new(_table, embedding_model, "vector", SearchParams::default()).await
        .map_err(|e| {
            log::error!("Failed to generate vector index: {:?}", e);
            ServerError::EmbeddingError
        })?;

    log::info!("Successfully created MCP tool index vector store. AI now has access to a growing list of tools that it can query and access dynamically.");
}
