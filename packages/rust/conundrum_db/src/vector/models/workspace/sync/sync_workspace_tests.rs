use std::{
    collections::HashSet,
    env,
    path::{Path, PathBuf},
    sync::Arc,
};

use conundrum::{
    ecosystem::{
        db::{
            db_client::db_client::DBClient,
            db_traits::{db_entity::DBSchema, into_partial::IntoPartial},
            tables::DatabaseTable,
        },
        error_handling::db_error::DatabaseResult,
    },
    lang::constants::file_types::ParsableFileType,
};
use conundrum_fs::workspace_management::{
    file_walk_config::FileWalkConfig, get_filetype_recursively::get_filetype_in_workspace_recursively,
};
use lancedb::{connect, database::CreateTableMode, Connection};
use tempfile::TempDir;
use test_log::env_logger;
use tokio::sync::Mutex;

use crate::vector::models::{
    ecosystem_data::ecosytem_setting_types::stringified_setting::EcosystemSettingEntity,
    meta::front_matter::front_matter::FrontMatter,
    taggables::auto_taggable::AutoTaggable,
    text::{
        cdrm::{cdrm_content::CdrmContent, cdrm_model::CdrmModel},
        text_based_content::text_based_content::TextBasedContent,
    },
};

use super::sync_workspace;

const SYNC_TEST_DIRECTORY_ENV: &str = "CONUNDRUM_SYNC_TEST_DIR";

struct TestDatabase {
    client: DBClient,
    _directory: TempDir,
}

async fn create_test_table<T: DBSchema>(connection: &Connection, table: DatabaseTable) -> DatabaseResult<()> {
    connection.create_empty_table(table.to_string(), Arc::new(T::schema()?))
              .mode(CreateTableMode::Create)
              .execute()
              .await
              .map_err(|error| {
                  log::error!("Could not create isolated sync test table {table}: {error}");
                  conundrum::ecosystem::error_handling::db_error::DatabaseError::FailToCreateTable(table)
              })?;
    Ok(())
}

async fn test_database() -> TestDatabase {
    let directory = tempfile::tempdir().expect("Create temporary LanceDB directory");
    let connection =
        connect(directory.path().to_str().expect("Temporary DB path is UTF-8")).execute()
                                                                               .await
                                                                               .expect("Connect to isolated LanceDB");

    create_test_table::<CdrmModel>(&connection, DatabaseTable::Cdrm).await.expect("Create CDRM table");
    create_test_table::<FrontMatter>(&connection, DatabaseTable::FrontMatter).await.expect("Create front matter table");
    create_test_table::<AutoTaggable>(&connection, DatabaseTable::AutoTaggable).await
                                                                               .expect("Create auto-taggable table");
    create_test_table::<EcosystemSettingEntity>(&connection, DatabaseTable::EcosystemSetting).await.expect("Create settings table");

    TestDatabase { client: DBClient(Arc::new(Mutex::new(connection))),
                   _directory: directory }
}

async fn cdrm_row_count(db: &DBClient) -> usize {
    let connection = db.0.clone().lock_owned().await;
    let table = conundrum::ecosystem::db::helpers::open_table::open_table(connection, DatabaseTable::Cdrm)
        .await
        .expect("Open synchronized CDRM table");
    table.count_rows(None).await.expect("Count synchronized CDRM rows")
}

fn walk_config(root: &Path) -> FileWalkConfig {
    FileWalkConfig { root: root.to_str().expect("Workspace path is UTF-8").to_string(),
                     respect_git_ignore: false,
                     ignore_hidden: true }
}

async fn write_file(root: &Path, relative_path: &str, content: &str) {
    let path = root.join(relative_path);
    tokio::fs::create_dir_all(path.parent().expect("Fixture file has a parent")).await
                                                                                .expect("Create fixture directory");
    tokio::fs::write(path, content).await.expect("Write fixture file");
}

fn file_count(context: &super::super::sync_context::SyncContext, file_type: ParsableFileType) -> u32 {
    context.count.update_count.get(&file_type).copied().unwrap_or_default()
}

fn assert_cdrm_type_counts(context: &super::super::sync_context::SyncContext, cdrm: u32, markdown: u32, mdx: u32) {
    assert_eq!(file_count(context, ParsableFileType::Cdrm), cdrm, "Incorrect .cdrm synchronization count");
    assert_eq!(file_count(context, ParsableFileType::Markdown), markdown, "Incorrect .md synchronization count");
    assert_eq!(file_count(context, ParsableFileType::Mdx), mdx, "Incorrect .mdx synchronization count");
}

#[tokio::test]
async fn syncs_cdrm_extensions_recursively_and_ignores_other_file_types() {
    let _ = env_logger::builder().is_test(true).try_init();
    let workspace = tempfile::tempdir().expect("Create temporary workspace");
    write_file(workspace.path(), "root.cdrm", "# CDRM root\n\nBody").await;
    write_file(workspace.path(), "nested/markdown.md", "# Markdown\n\nBody").await;
    write_file(workspace.path(), "nested/deeper/component.mdx", "# MDX\n\nBody").await;
    write_file(workspace.path(), "ignored.typst", "= Not a CDRM document").await;
    write_file(workspace.path(), "ignored.json", "{} ").await;
    write_file(workspace.path(), "ignored.csv", "header,value\n1,2\n").await;
    write_file(workspace.path(), "ignored.png", "not an image").await;
    write_file(workspace.path(), ".hidden/hidden.md", "# Hidden").await;

    let discovered =
        get_filetype_in_workspace_recursively(walk_config(workspace.path())).await.expect("Walk fixture workspace");
    let discovered = discovered.lock_arc();
    assert_eq!(discovered.values().map(Vec::len).sum::<usize>(),
               5,
               "Fixture walk should see the five visible parsable files");
    drop(discovered);

    let database = test_database().await;
    let context =
        sync_workspace(database.client.clone(), walk_config(workspace.path())).await.expect("Synchronize CDRM files");

    assert_cdrm_type_counts(&context, 1, 1, 1);
    assert_eq!(cdrm_row_count(&database.client).await, 3, "Each supported file should be persisted");
    assert_eq!(file_count(&context, ParsableFileType::Typst),
               0,
               "Other parsable types are not synchronized by the CDRM worker");
}

#[tokio::test]
async fn repeated_sync_updates_the_existing_model_without_duplicating_it() {
    let workspace = tempfile::tempdir().expect("Create temporary workspace");
    write_file(workspace.path(), "notes/revise.md", "# Before\n\nOriginal body").await;
    let database = test_database().await;

    let first =
        sync_workspace(database.client.clone(), walk_config(workspace.path())).await.expect("Synchronize initial file");
    assert_cdrm_type_counts(&first, 0, 1, 0);
    let model = CdrmModel(TextBasedContent::new(CdrmContent("# Before\n\nOriginal body".to_string()),
                                                None,
                                                workspace.path().to_string_lossy().to_string(),
                                                "notes/revise.md".to_string()));
    let model_id = model.0.id.clone();
    let partial = serde_json::to_value(model.into_partial()).expect("Serialize model partial update");
    let partial_id = partial.get("id").expect("Partial update includes its ID");
    assert_eq!(partial_id, &serde_json::to_value(model_id).expect("Serialize model ID"));
    assert_eq!(cdrm_row_count(&database.client).await, 1, "Initial sync should persist one CDRM model");

    write_file(workspace.path(), "notes/revise.md", "# After\n\nUpdated body").await;
    let second =
        sync_workspace(database.client.clone(), walk_config(workspace.path())).await
                                                                              .expect("Synchronize modified file");
    assert_cdrm_type_counts(&second, 0, 1, 0);
    assert_eq!(cdrm_row_count(&database.client).await, 1, "Update should merge without duplicating the model");

    sync_workspace(database.client.clone(), walk_config(workspace.path())).await.expect("Synchronize unchanged file");
    assert_eq!(cdrm_row_count(&database.client).await, 1, "Unchanged sync should remain idempotent");
}

#[tokio::test]
async fn failed_file_does_not_count_as_synced_or_block_valid_siblings() {
    let workspace = tempfile::tempdir().expect("Create temporary workspace");
    write_file(workspace.path(), "valid.md", "# Valid\n\nBody").await;
    tokio::fs::write(workspace.path().join("invalid.md"), [0xff, 0xfe]).await.expect("Write invalid UTF-8 fixture");
    let database = test_database().await;

    let context = sync_workspace(database.client.clone(), walk_config(workspace.path()))
        .await
        .expect("A bad file should not prevent other files from synchronizing");

    assert_cdrm_type_counts(&context, 0, 1, 0);
    assert_eq!(cdrm_row_count(&database.client).await, 1, "The valid sibling should be persisted");
}

#[tokio::test]
async fn front_matter_subject_and_topic_are_collected_in_their_own_buckets() {
    let workspace = tempfile::tempdir().expect("Create temporary workspace");
    write_file(workspace.path(), "categorized.md", "---\nsubject: Science\ntopic: Astronomy\n---\n# Stars\n\nBody").await;
    let database = test_database().await;

    let context = sync_workspace(database.client.clone(), walk_config(workspace.path()))
        .await
        .expect("Synchronize categorized document");

    assert_cdrm_type_counts(&context, 0, 1, 0);
    let subjects: HashSet<_> = context.subjects.iter().map(|item| item.value.as_str()).collect();
    let topics: HashSet<_> = context.topics.iter().map(|item| item.value.as_str()).collect();
    assert_eq!(subjects, HashSet::from(["Science"]));
    assert_eq!(topics, HashSet::from(["Astronomy"]));
    assert!(context.tags.is_empty(), "Subject and topic values must not be recorded as tags");
}

#[tokio::test]
#[ignore = "integration test reads the configured notes directory; run with CONUNDRUM_SYNC_TEST_DIR set"]
async fn synchronizes_configured_notes_directory_and_reports_successful_counts() {
    let _ = env_logger::builder().is_test(true).try_init();
    let root =
        env::var_os(SYNC_TEST_DIRECTORY_ENV).map(PathBuf::from)
                                            .unwrap_or_else(|| PathBuf::from("/Users/bigsexy/Desktop/notes/content"));
    assert!(root.is_dir(), "Configured sync test directory is not a directory");

    let config = walk_config(&root);
    let discovered = get_filetype_in_workspace_recursively(config.clone()).await.expect("Walk configured workspace");
    let expected = {
        let paths = discovered.lock_arc();
        [ParsableFileType::Cdrm, ParsableFileType::Markdown, ParsableFileType::Mdx].map(|file_type| {
                                                                                       (file_type.clone(),
                                                                                        paths.get(&file_type)
                                                                                             .map_or(0, |items| {
                                                                                                 items.len() as u32
                                                                                             }))
                                                                                   })
    };

    let database = test_database().await;
    let context =
        sync_workspace(database.client.clone(), config).await.expect("Synchronize configured notes directory");
    for (file_type, expected_count) in &expected {
        let successful_count = file_count(&context, file_type.clone());
        assert!(successful_count <= *expected_count, "Sync reported more {file_type} files than the walker discovered");
        eprintln!("{file_type}: synchronized {successful_count} of {expected_count} discovered files");
    }
    let discovered_total: u32 = expected.iter().map(|(_, count)| *count).sum();
    let successful_total: u32 = expected.iter().map(|(file_type, _)| file_count(&context, file_type.clone())).sum();
    assert!(discovered_total == 0 || successful_total > 0,
            "The configured workspace contained supported files but none synchronized successfully");
    assert_eq!(cdrm_row_count(&database.client).await as u32,
               successful_total,
               "Persisted models should match successfully synchronized files");
}
