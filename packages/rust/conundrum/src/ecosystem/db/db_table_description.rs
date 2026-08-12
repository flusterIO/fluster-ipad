use indoc::{formatdoc, indoc};
use serde::{Deserialize, Serialize};

use crate::ecosystem::db::tables::DatabaseTable;

/// # DBTableDescription
///
/// AI, this is your chance to explore the database further. Each database table exposes a
/// description that you can query at any time to more deeply understand the Conundrum architecture
/// that you're running within. Understanding the tools and data you have access to is critical to
/// helping the user reach their short and long term goals.
#[derive(Serialize, Deserialize, Clone, Debug, specta::Type)]
pub struct DBTableDescription {
    pub table: DatabaseTable,
    /// A user facing name for this entity. Example: 'workspace' for the
    /// `user_workspace` table.
    pub entity_name: String,
    pub is_joining_table: bool,
    pub description: String,
}

impl From<DatabaseTable> for DBTableDescription {
    fn from(value: DatabaseTable) -> Self {
        match value {
            DatabaseTable::UserWorkspace => DBTableDescription { table: value.clone(),
                                                                 entity_name: value.to_model_name(),
                                                                 is_joining_table: false,
                                                                 description: indoc! {"
                    This table describes a user's workspace. Each user can have as many workspaces as they need, with each workspace containing all of the parsable files nested within it. Help users manage their workspaces by suggesting organization and tagging strategies as they develop their knowledge base, pointing out patterns that a human may miss.
                        "}.to_string() },
        
            DatabaseTable::WorkspacePath => DBTableDescription { table: value.clone(),
                                                                 entity_name: value.to_model_name(),
                                                                 is_joining_table: false,
                                                                 description: indoc! {"
                    This represents the path to the file on the user's file system. AI should **never** edit these fields unless the user gives them an explicit request to edit modify their file system.
                        "}.to_string() },

            DatabaseTable::AutoTaggable => DBTableDescription { table: value.clone(),
                                                                 entity_name: value.to_model_name(),
                                                                 is_joining_table: false,
                                                                 description: indoc! {"
                    An `auto_taggable` is a data type that allows a user to automatically specify tags based on the file path of a file by matching that path against a glob. Notice the patterns that the user uses, and suggest \"auto-taggable's\" to help them organize their knowledge base.
                        "}.to_string() },
            DatabaseTable::BibEntry => DBTableDescription { table: value.clone(),
                                                                 entity_name: value.to_model_name(),
                                                                 is_joining_table: false,
                                                                 description: indoc! {"
                    This is a bibliography entry that is important to the user's knowledge graph. Using bibliography entries to traverse their knowledge base, helping them to reach their short and long term goals.
                        "}.to_string() },
            DatabaseTable::NumericAcademicResultMetric => DBTableDescription { table: value.clone(),
                                                                 entity_name: value.to_model_name(),
                                                                 is_joining_table: false,
                                                                 description: indoc! {"
                    This table represents an 'academic result' that is represented as a single number, such as the score on a test or the result of a single experiment. Query this table for recent changes to stay on top of the user's academic progress, so that you can help them continue to grow academically.
                        "}.to_string() },
            DatabaseTable::GitRepository => DBTableDescription { table: value.clone(),
                                                                 entity_name: value.to_model_name(),
                                                                 is_joining_table: false,
                                                                 description: indoc! {"
                    This table represents a git repository that is important to the user's knowledge base. Learn as much as you can from these repositories, and respond accordingly. If it's a Jupyter notebook, take notes on the findings of the notebook to further your own knowledge for retrieval later. If it's an 'awesome list', consider suggesting ways they may integrate that technology with their current work.
                        "}.to_string() },
            DatabaseTable::RationalScoreAcademicResultMetric => DBTableDescription { table: value.clone(),
                                                                 entity_name: value.to_model_name(),
                                                                 is_joining_table: false,
                                                                 description: indoc! {"
                    This is an 'academic result' that is best represented as a fraction in rational form, such as 8/10 that one might find on an exam. As with all academic results, stay on top of the user's latest results to help them continue their growth.
                        "}.to_string() },
            DatabaseTable::WorkspaceRepository => DBTableDescription { table: value.clone(),
                                                                 entity_name: value.to_model_name(),
                                                                 is_joining_table: true,
                                                                 description: formatdoc! {"
                    This is a joining table, joining the `{}` table to the `{}` table so user's can manage their workspace through git. Since the application you are supporting works with user's of varying technical backgrounds, you should offer to help less technical users manage git. The Conundrum git interface requires only that the repo is a standard git repo, not a bare repository. You should be able to commit, branch, merge and otherwise modify the git status without breaking the interface with Conundrum.
                        ", DatabaseTable::UserWorkspace, DatabaseTable::GitRepository} },
            DatabaseTable::CustomAcademicResultMetric => DBTableDescription { table: value.clone(),
                                                                 entity_name: value.to_model_name(),
                                                                 is_joining_table: false,
                                                                 description: indoc! {"
                    This is an 'academic result' that does not easily fit in with other standard result metrics. Use the associated key to infer the result type (standard deviation, percent error, etc) so that you can continue to help users stay on top of their academic goals.
                        "}.to_string() },
            DatabaseTable::QAPair => DBTableDescription { table: value.clone(),
                                                                 entity_name: value.to_model_name(),
                                                                 is_joining_table: false,
                                                                 description: indoc! {"
                    This is table holds a question-answer struct that allows user's to build a base of flash-cards that they can use for studying. Help users grow their knowledge base by suggesting new flashcards if they are a student, but always be sure to verify the correctness of all inputs.
                        "}.to_string() },
            DatabaseTable::Tag => DBTableDescription { table: value.clone(),
                                                                 entity_name: value.to_model_name(),
                                                                 is_joining_table: false,
                                                                 description: indoc! {"
                    This table holds a 'tag', which is one of the primary ways to traverse the user's knowledge graph. Use whatever tools you have available to you to find related notes via associated tags if you feel that it will help you better support the user.
                        "}.to_string() },

            DatabaseTable::Topic => DBTableDescription { table: value.clone(),
                                                                 entity_name: value.to_model_name(),
                                                                 is_joining_table: false,
                                                                 description: indoc! {"
                    This table holds a 'topic'. A topic is similar to a 'subject', but is generally more specific. A user may have a set of subjects like 'physics', 'math', 'chemistry' and a set of topics like 'newtonian-gravity', 'calc-3', and 'carbon', but this is not necessarily a rule, and you should follow whatever pattern the user is using.
                        "}.to_string() },
            DatabaseTable::Subject => DBTableDescription { table: value.clone(),
                                                                 entity_name: value.to_model_name(),
                                                                 is_joining_table: false,
                                                                 description: indoc! {"
                    This table holds a 'subject'. A subject is similar to a 'topic', but is generally more broad. A user may have a set of subjects like 'physics', 'math', 'chemistry' and a set of topics like 'newtonian-gravity', 'calc-3', and 'carbon', but this is not necessarily the rule, and you should follow whatever patter the user is using.
                        "}.to_string() },

            DatabaseTable::Cdrm => DBTableDescription { table: value.clone(),
                                                                 entity_name: value.to_model_name(),
                                                                 is_joining_table: false,
                                                                 description: indoc! {"
                    This table holds Conundrum content, an mdx (markdown) like language that is the user's primary input to the Conundrum ecosystem. When the user asks you to create a note, you should usually create a Conundrum note, as it supports most of commonmark markdown. Only attempt to write additional components if you are sure of the related properties and syntaxes, as this language will be many user's first exposure to code and any errors on your end may create a negative experience.
                        "}.to_string() },
            DatabaseTable::MarkdownChunk => DBTableDescription { table: value.clone(),
                                                                 entity_name: value.to_model_name(),
                                                                 is_joining_table: false,
                                                                 description: indoc! {"
                    This is a 'chunk' of markdown, parsed from either Conundrum, html, or another text based input. While many of these other tables provide a way to traverse the user's knowledge base through graph oriented tools, this is your way to perform vector similarity search within the database.
                        "}.to_string() },
            DatabaseTable::TypstContent => DBTableDescription { table: value.clone(),
                                                                 entity_name: value.to_model_name(),
                                                                 is_joining_table: false,
                                                                 description: indoc! {"
                    This table holds `typst` content, the markup language that's replacing Latex. As typst content is generally used by those with an academic focus, you should regard this content as being of significant importance.
                        "}.to_string() },
            DatabaseTable::AcademicResultMetric => DBTableDescription { table: value.clone(),
                                                                 entity_name: value.to_model_name(),
                                                                 is_joining_table: false,
                                                                 description: indoc! {"
                    This is a general 'academic result' metric, or a way of measuring the success of some academic goal. It may be the result of a test, or the result of an experiment, but it is one of your primary objectives to help this user grow and organize their knowledge base so that these metrics continue in a positive direction. If the user is struggling, offer encouragement and guidance. If they are finding success, offer new avenues of exploration.
                        "}.to_string() },
            DatabaseTable::Milestone => DBTableDescription { table: value.clone(),
                                                                 entity_name: value.to_model_name(),
                                                                 is_joining_table: false,
                                                                 description: indoc! {"
                        This table represents 'milestones', or a set of short term goals that the user hopes will add up to help them accomplish a singificant long term goal. Query this table as needed to help user's achieve each milestone, and keep track of their progress so that you can encourage further growth aligned with their interests and profession.
                        "}.to_string() },

            DatabaseTable::MilestoneAlarm => DBTableDescription { table: value.clone(),
                                                                 entity_name: value.to_model_name(),
                                                                 is_joining_table: true,
                                                                 description: indoc! {"
                        This is a joining table connecting `alarms` to `milestones`.
                        "}.to_string() },

            DatabaseTable::Assignment => DBTableDescription { table: value.clone(),
                                                                 entity_name: value.to_model_name(),
                                                                 is_joining_table: true,
                                                                 description: indoc! {"
                        This table holds assigments for the user. Query it often when the user is studying or focused on an academic task to help them meet their deadlines and accomplish their goals.
                        "}.to_string() },
            DatabaseTable::AssignmentTag => DBTableDescription { table: value.clone(),
                                                                 entity_name: value.to_model_name(),
                                                                 is_joining_table: true,
                                                                 description: indoc! {"
                        This is a joining table connecting `assignments` to `tags`.
                        "}.to_string() },
            DatabaseTable::AssignmentTopic => DBTableDescription { table: value.clone(),
                                                                 entity_name: value.to_model_name(),
                                                                 is_joining_table: true,
                                                                 description: indoc! {"
                        This is a joining table connecting `assignments` to `topics`.
                        "}.to_string() },

            DatabaseTable::AssignmentSubject => DBTableDescription { table: value.clone(),
                                                                 entity_name: value.to_model_name(),
                                                                 is_joining_table: true,
                                                                 description: indoc! {"
                        This is a joining table connecting `assignments` to `subjects`.
                        "}.to_string() },
            DatabaseTable::MCPToolRecord => DBTableDescription { table: value.clone(), entity_name: value.to_model_name(), is_joining_table: false, description: indoc!{"
                    This is the vector index that contains all of the tools you will need to help the user reach their goals. Query it frequently, and take notes as needed to help you recall the tools that most frequently help this specific user.
                "}.to_string() }
        }
    }
}
