use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, strum_macros::Display, Clone)]
pub enum RouteEnum {
    #[serde(rename = "math/tex_to_svg")]
    #[strum(to_string = "math/tex_to_svg")]
    Math_TexToSvg,
    #[serde(rename = "study/quiz_me")]
    #[strum(to_string = "study/quiz_me")]
    Study_QuizMe,
    #[serde(rename = "study/random_question")]
    #[strum(to_string = "study/random_question")]
    Study_RandomQuestion,
    #[serde(rename = "study/save_flashcard")]
    #[strum(to_string = "study/save_flashcard")]
    Study_SaveFlashcard,
}
