use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(uniffi::Enum, strum_macros::Display, EnumIter, Clone, Serialize, Deserialize)]
pub enum EmbeddedCslFile {
    #[serde(rename = "american-chemical-society.csl")]
    #[strum(to_string = "american-chemical-society.csl")]
    AmericanChemicalSociety,
    #[serde(rename = "american-institute-of-physics.csl")]
    #[strum(to_string = "american-institute-of-physics.csl")]
    AmericanInstituteOfPhysics,
    #[serde(rename = "american-medical-association.csl")]
    #[strum(to_string = "american-medical-association.csl")]
    AmericanMedicalAssociation,
    #[serde(rename = "american-political-science-association.csl")]
    #[strum(to_string = "american-political-science-association.csl")]
    AmericanPoliticalScienceAssociation,
    #[serde(rename = "american-sociological-association.csl")]
    #[strum(to_string = "american-sociological-association.csl")]
    AmericanSociologicalAssociation,
    #[serde(rename = "apa.csl")]
    #[strum(to_string = "apa.csl")]
    Apa,
    #[serde(rename = "association-for-computing-machinery.csl")]
    #[strum(to_string = "association-for-computing-machinery.csl")]
    AssociationForComputingMachinery,
    #[serde(rename = "cell.csl")]
    #[strum(to_string = "cell.csl")]
    Cell,
    #[serde(rename = "chicago-author-date.csl")]
    #[strum(to_string = "chicago-author-date.csl")]
    ChicagoAuthorDate,
    #[serde(rename = "chicago-notes-bibliography.csl")]
    #[strum(to_string = "chicago-notes-bibliography.csl")]
    ChicagoNotesBibliography,
    #[serde(rename = "harvard-cite-them-right.csl")]
    #[strum(to_string = "harvard-cite-them-right.csl")]
    HarvardCiteThemRight,
    #[serde(rename = "ieee.csl")]
    #[strum(to_string = "ieee.csl")]
    Ieee,
    #[serde(rename = "modern-language-association.csl")]
    #[strum(to_string = "modern-language-association.csl")]
    ModernLanguageAssociation,
    #[serde(rename = "nature.csl")]
    #[strum(to_string = "nature.csl")]
    Nature,
    #[serde(rename = "science.csl")]
    #[strum(to_string = "science.csl")]
    Science,
    #[serde(rename = "springer-vancouver.csl")]
    #[strum(to_string = "springer-vancouver.csl")]
    SpringerVancouver,
}
