use serde::{Deserialize, Serialize};

#[allow(nonstandard_style)]
#[derive(Serialize, Deserialize, Default, Clone, strum_macros::Display)]
pub enum EmbeddedCSLFile {
    #[strum(to_string = "csl/chicago-notes-bibliography.csl")]
    #[serde(rename = "csl/chicago-notes-bibliography.csl")]
    Chicago_notes_bibliography,
    #[strum(to_string = "csl/american-medical-association.csl")]
    #[serde(rename = "csl/american-medical-association.csl")]
    American_medical_association,
    #[strum(to_string = "csl/american-political-science-association.csl")]
    #[serde(rename = "csl/american-political-science-association.csl")]
    American_political_science_association,
    #[strum(to_string = "csl/association-for-computing-machinery.csl")]
    #[serde(rename = "csl/association-for-computing-machinery.csl")]
    Association_for_computing_machinery,
    #[strum(to_string = "csl/american-chemical-society.csl")]
    #[serde(rename = "csl/american-chemical-society.csl")]
    American_chemical_society,
    #[strum(to_string = "csl/cell.csl")]
    #[serde(rename = "csl/cell.csl")]
    Cell,
    #[strum(to_string = "csl/ieee.csl")]
    #[serde(rename = "csl/ieee.csl")]
    Ieee,
    #[strum(to_string = "csl/nature.csl")]
    #[serde(rename = "csl/nature.csl")]
    Nature,
    #[strum(to_string = "csl/american-institute-of-physics.csl")]
    #[serde(rename = "csl/american-institute-of-physics.csl")]
    American_institute_of_physics,
    #[strum(to_string = "csl/springer-vancouver.csl")]
    #[serde(rename = "csl/springer-vancouver.csl")]
    Springer_vancouver,
    #[strum(to_string = "csl/harvard-cite-them-right.csl")]
    #[serde(rename = "csl/harvard-cite-them-right.csl")]
    Harvard_cite_them_right,
    #[strum(to_string = "csl/apa.csl")]
    #[serde(rename = "csl/apa.csl")]
    #[default]
    Apa,
    #[strum(to_string = "csl/chicago-author-date.csl")]
    #[serde(rename = "csl/chicago-author-date.csl")]
    Chicago_author_date,
    #[strum(to_string = "csl/science.csl")]
    #[serde(rename = "csl/science.csl")]
    Science,
    #[strum(to_string = "csl/modern-language-association.csl")]
    #[serde(rename = "csl/modern-language-association.csl")]
    Modern_language_association,
    #[strum(to_string = "csl/american-sociological-association.csl")]
    #[serde(rename = "csl/american-sociological-association.csl")]
    American_sociological_association,
}
