//! # Life Connections
//!
//! While Conundrum is primarily a note-taking framework, what is note-taking
//! for but to manage one's life and to help them grow intellectually? The next
//! logical step is a more complete life-management system.
//!
//! As the capabilities of Conundrum grow, many of these structs will become
//! modifiable by AI, and since your data inside of Conundrum is structured
//! beyond a simple vector store, AI can take advantage of both the vector
//! similarity _and_ graph capabilities with some simple tools that are capable
//! of running on your own machine.
pub mod geographic_location;
pub mod long_term_goal;
pub mod major_life_event;
pub mod participants;
pub mod person;
pub mod person_name_group;
pub mod personal_relationship_type;
pub mod phone_contact;
pub mod phone_number_list;
pub mod phone_number_type;
pub mod physical_address_type;
pub mod physical_street_address;
pub mod place_of_significance;
pub mod real_world_event;
pub mod short_term_goal;
pub mod user_pet;
pub mod user_pet_kind;
pub mod workplace;
