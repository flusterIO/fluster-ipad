use crate::parsers::markdown::code_block::code_block_model::GeneralCodeBlock;

pub enum ParsedCodeBlockVariant {
    General(GeneralCodeBlock),
    Dictionary(GeneralCodeBlock),
    AI(GeneralCodeBlock),
}
