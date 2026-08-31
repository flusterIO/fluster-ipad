use std::sync::Arc;

use conundrum::{
    ecosystem::{
        db::{
            db::ArcMutexDB,
            db_traits::{db_entity::DBEntity, entity_crud::EntityCRUD},
        },
        error_handling::db_error::DatabaseResult,
    },
    lang::{lib::shared::utility_types::ArcTokioMutex, runtime::queries::get_title::get_title_group},
    lifted_models::primitives::{date_time::DateTime, db_id::DatabaseId},
    output::parsing_result::mdx_parsing_result::MdxParsingResult,
};

use crate::vector::models::{
    meta::front_matter::{front_matter::FrontMatter, front_matter_source_type::FrontMatterSourceType},
    taggables::{tag_location::TagLocation, taggable::TaggableVariant},
    text::{
        cdrm::{cdrm_content::CdrmContent, cdrm_model::CdrmModel},
        text_based_content::text_based_content::TextBasedContent,
    },
    workspace::sync::{
        ir::generic_tag_input::{GenericTagInput, TagSource},
        sync_context::SyncContext,
    },
};

pub async fn update_database_from_parsed_cdrm(content: MdxParsingResult,
                                              existing_note: Option<CdrmModel>,
                                              file_content: String,
                                              workspace_path: String,
                                              relative_path: String,
                                              db: ArcMutexDB,
                                              context: ArcTokioMutex<SyncContext>)
                                              -> DatabaseResult<()> {
    let title =
        get_title_group(file_content.clone(),
                        vec![],
                        conundrum::lang::runtime::state::parse_state::ConundrumCompileTarget::PlainText).ok()
                                                                                                        .map(|x| {
                                                                                                            x.title
                                                                                                        });
    let model = match existing_note {
        Some(mut s) => {
            s.0.utime = DateTime::new_now();
            s.0.content = CdrmContent(file_content.clone());
            s.0.title = title.clone();
            s
        }
        None => CdrmModel(TextBasedContent::new(CdrmContent(file_content),
                                                title,
                                                workspace_path.clone(),
                                                relative_path.clone())),
    };

    let cloned_db = Arc::clone(&db);
    <CdrmModel as EntityCRUD>::merge_by_primary_key(vec![model.clone()], cloned_db).await?;
    let existing_frontmatter = model.get_related_frontmatter(Arc::clone(&db)).await?;
    if let Some(new_frontmatter) = match existing_frontmatter {
        Some(mut fm) => {
            if let Some(content_fm) = &content.front_matter {
                fm.data = content_fm.clone();
                Some(fm)
            } else {
                Some(fm)
            }
        }
        None => content.front_matter.clone().map(|fm| FrontMatter { id: DatabaseId::new(),
                                                                    note_id: model.0.id.clone(),
                                                                    source_type: FrontMatterSourceType::Cdrm,
                                                                    data: fm.clone() }),
    } {
        <FrontMatter as EntityCRUD>::merge_by_primary_key(vec![new_frontmatter], Arc::clone(&db)).await?;
    }

    let mut ctx = context.clone().lock_owned().await;

    for tag in content.tags {
        ctx.append_tag(GenericTagInput::from_tag_result(tag, model.0.id.clone(), TagLocation::Body));
    }

    let matching_taggables = ctx.matching_autotaggables(relative_path.clone())?;
    for auto_tag in &matching_taggables {
        if auto_tag.variant == TaggableVariant::Tag {
            ctx.append_tag(GenericTagInput::from_auto_taggable(auto_tag.clone(), model.0.id.clone(), TagSource::Cdrm));
        }
    }

    let (subject_string, subject_location) = match &content.front_matter {
        Some(fm) => (fm.subject.clone(), TagLocation::FrontMatter),
        None => (matching_taggables.iter().find(|x| x.variant == TaggableVariant::Subject).map(|x| x.value.clone()),
                 TagLocation::AppInserted),
    };

    if let Some(subject) = subject_string {
        ctx.append_subject(GenericTagInput { value: subject.clone(),
                                             source_id: model.0.id.clone(),
                                             source_type: TagSource::Cdrm,
                                             location: subject_location });
    }

    let (topic_string, topic_location) = match &content.front_matter {
        Some(fm) => (fm.topic.clone(), TagLocation::FrontMatter),
        None => (matching_taggables.iter().find(|x| x.variant == TaggableVariant::Topic).map(|x| x.value.clone()),
                 TagLocation::AppInserted),
    };

    if let Some(topic) = topic_string {
        ctx.append_topic(GenericTagInput { value: topic.clone(),
                                           source_id: model.0.id.clone(),
                                           source_type: TagSource::Cdrm,
                                           location: topic_location });
    }
    Ok(())
}
