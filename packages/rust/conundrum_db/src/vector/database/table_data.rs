use conundrum::{
    bibliography::bib_entry::BibEntry, parsers::conundrum::logic::number::conundrum_float::ConundrumFloat,
};

use crate::vector::models::{
    academic::{
        assignment::{academic_assignment::Assignment, milestone::Milestone},
        class::class_model::ClassModel,
        data::numeric::{numeric_data::NumericData, numeric_result::numeric_result::NumericResult},
        question::flashcard::flashcard_model::FlashCardModel,
        result::{academic_result::AcademicResult, academic_result_metric::AcademicResultMetric},
    },
    application_support::application_data::ApplicationData,
    bib::bibliography::Bibliography,
    date_time::{alarm::Alarm, alert::alert::Alert, schedule::time_block::TimeBlock},
    ecosystem_data::ecosystem_data::EcosystemData,
    meta::{front_matter::front_matter::FrontMatter, summary::summary::SummaryModel},
    taggables::{subject::Subject, tag::Tag, topic::Topic},
    text::{cdrm::cdrm_content::CdrmContent, html::html_content::HTMLContent, typst::typst::TypstContent},
};

pub enum TableData {
    // --- Text Content ---
    Conundrum(CdrmContent),
    FrontMatter(FrontMatter),
    Typst(TypstContent),
    HTML(HTMLContent),
    Summary(SummaryModel),
    // --- Searchability ---
    Tag(Tag),
    Topic(Topic),
    Subject(Subject),
    // --- Media ---
    // --- Academic Tools ---
    Class(ClassModel),
    AcademicAssignment(Assignment),
    Milestone(Milestone),
    Flashcard(FlashCardModel),
    AcademicResult(AcademicResult),
    AcademicResultMetric(AcademicResultMetric),
    // StudyList(StudyList),
    // --- Research Tools ---
    NumericResult(NumericResult),
    NumericFloatData(NumericData<Vec<ConundrumFloat>>),
    BibEntry(BibEntry),
    Bibliography(Bibliography),
    // --- Shared model components ---
    Alarm(Alarm),
    Alert(Alert),
    TimeBlock(TimeBlock),
    // --- Stuff that makes things work ---
    EcosystemData(EcosystemData),
    WhiteListedApplications(ApplicationData),
}
