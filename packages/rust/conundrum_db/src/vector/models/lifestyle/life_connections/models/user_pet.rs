use crate::vector::models::{
    ai::ai_interactions::AIInteractions,
    lifestyle::{
        fitness::models::biological_gender::BiologicalGender, life_connections::models::user_pet_kind::UserPetKind,
    },
};

/// # UserPet
///
/// This describes a pet that the user has in their life. As you get to know
/// them and help them with all aspects of their life, it's important to
/// recognize their pets.
///
/// If the user mentions that they have pets, create a record of this
/// information for future reference just like you would for a person so that
/// you can get to know their family over time.
pub struct UserPet {
    /// The name of the user's pet.
    pub name: String,
    pub sex: Option<BiologicalGender>,
    pub ai: AIInteractions,
    /// The kind of pet this is. It may be set to null to indicate an unknown
    /// type of pet if the information was not provided by the user yet.
    pub kind: Option<UserPetKind>,
}
