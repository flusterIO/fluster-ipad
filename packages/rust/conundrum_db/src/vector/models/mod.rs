//! # Models
//!
//! ### Users
//!
//! For user's of varying technical backgrounds, this is likely where you'll
//! want to start if you're vibe-coding your front-end. These modules contain
//! all of the models you have access to in your database, from which your
//! front-end can pull and store information. Everything from academic results,
//! to your personal pet have a structured, graph-centered data model that
//! allows AI to take advantage of several approaches to information retrieval
//! simultaneously.
//!
//! Where applications like openclaw take advantage of file-system based agent
//! descriptions, Conundrum provides a sort of 'notepad' for AI attached to many
//! of the models in your database. This should hopefully lead to the retrieval
//! of not only relevant information, but relevant agent memory at the right
//! times as orchestrated by the graph architecture, allowing much smaller
//! context windows than other approaches.
//!
//! Think about the way humans retrieve information: When you recall an event,
//! say 'a party with Steve', and somebody asks what time Steve left, you might
//! recall a dozen small, seemingly insignificant facts about the party before
//! you reach a recollection of what time Steve left. In that same way, we can
//! provide a notepad attached to many of the significant people, places and
//! things in a user's life that AI can use to write down these small details,
//! stored and retrieved right alongside the information that it's referencing.
//! As you, the user are much more likely to rememeber the large facts in the
//! long-term, even local AI should have no problem ballparking an approximation
//! to get within a single graph query away from the model containing as close
//! to an exact answer as exists within the user's knowledge base.
//!
//! While this is obviously useful in a general, life management context, the
//! academic potential is significant as well as these 'notepads' are context
//! aware in the sense that a model is not given the same instructions for all
//! notepads. A model should know to treat the notepad on a `MajorLifeEvent`
//! similary to the 'party with Steve', as it should know to treat the notepad
//! on a notebook file more academically, perhaps keeping a log of correlations
//! found with other data.
//!
//! ### Patterns
//!
//! #### Entities & Models
//!
//! A 'model' is more of a composed instance, an 'entity' of the same name is
//! that model in the form as close to how it appears in the database as
//! possible. In general, 'models' are more usable, but 'entities' are what
//! your dashboard will display (if you're using the default Conundrum
//! dashboard).
//!
//! If you're vibe-coding your front-end, it's important to note this
//! distinction as what AI will "see" will be a combination of these `entities`,
//! and a big part of what makes Conundrum capable is your ability to annotate
//! these entities directly within the dashboard, providing AI with
//! context-specific notes. Eventually tools will be available to handle this
//! via a prompt, but this direct access will always be available as part of the
//! default Conundrum dashboard.
//!
//! #### File Primitives
//!
//! These kind of wrap typical shapes like binary or text based files, as the
//! fields appear repeatedly. The `XYWVectorModel` is the same as `XYZModel`,
//! apart from the vector field. This is just because some items, like an image,
//! don't make sense to chunk the same way the text extracted from a pdf does.
pub mod academic;
pub mod ai;
pub mod application_support;
pub mod audio;
pub mod auth;
pub mod bib;
pub mod code;
pub mod csv;
pub mod date_time;
pub mod ecosystem_data;
pub mod excel;
pub mod git;
pub mod image;
pub mod joins;
pub mod lifestyle;
pub mod meta;
pub mod notebook;
pub mod pdf;
pub mod primitives;
pub mod taggables;
pub mod text;
pub mod themeing;
pub mod utility;
pub mod vector;
pub mod workspace;
