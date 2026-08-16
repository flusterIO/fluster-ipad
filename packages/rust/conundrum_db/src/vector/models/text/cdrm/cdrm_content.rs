use arrow_schema::Field;
use conundrum::{
    ecosystem::error_handling::db_error::DatabaseError,
    lang::runtime::queries::get_title::get_title_group,
    lang::runtime::run_conundrum::{ParseConundrumOptions, run_conundrum},
};
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::{
    database::db_traits::db_field::DatabaseField,
    models::text::text_based_content::{text_based_chunk::TextBasedChunk, text_based_content_trait::TextBasedContent},
};

#[derive(Serialize, Deserialize, Clone, Debug, Dummy)]
pub struct CdrmContent(String);

impl DatabaseField for CdrmContent {
    fn field_definition(field_key: &'static str, nullable: bool) -> Field {
        String::field_definition(field_key, nullable)
    }
}

impl TextBasedContent<ParseConundrumOptions> for CdrmContent {
    fn get_parsed_content(&self,
                          opts: ParseConundrumOptions)
                          -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<String> {
        let x = run_conundrum(opts).map_err(|e| {
                                       log::error!("Fail to parse Conundrum content: {:#?}", e);
                                       DatabaseError::ConundrumError(e)
                                   })?;
        Ok(x.content)
    }

    fn get_title(&self,
                 modifiers: Vec<conundrum::lang::runtime::state::parse_state::ConundrumModifier>,
                 target: conundrum::lang::runtime::state::parse_state::ConundrumCompileTarget)
                 -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Option<String>> {
        let r = get_title_group(self.0.clone(), modifiers, target).map_err(|e| {
                    log::error!("Failed to get Conundrum title: {:#?}", e);
                    DatabaseError::ConundrumError(e)
                })?;
        if r.title.trim().is_empty() {
            Ok(None)
        } else {
            Ok(Some(r.title))
        }
    }

    fn try_chunk(&self) -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<TextBasedChunk>> {
        todo!()
    }
}
