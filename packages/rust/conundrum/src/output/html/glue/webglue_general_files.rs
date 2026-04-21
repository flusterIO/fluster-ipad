use serde::{Deserialize, Serialize};
#[derive(strum_macros::Display, Serialize, Deserialize, Clone)]
pub enum WebGlueCodeGeneralFiles {
    #[serde(rename = "styles.css")]
    #[strum(to_string = "styles.css")]
    Styles,
    #[serde(rename = "katex.min.css")]
    #[strum(to_string = "katex.min.css")]
    KatexCss,
    #[serde(rename = "katex_ams_regular.woff2")]
    #[strum(to_string = "katex_ams_regular.woff2")]
    #[allow(non_camel_case_types)]
    Katex_ams_regular,
    #[serde(rename = "katex_caligraphic_bold.woff2")]
    #[strum(to_string = "katex_caligraphic_bold.woff2")]
    #[allow(non_camel_case_types)]
    Katex_caligraphic_bold,
    #[serde(rename = "katex_caligraphic_regular.woff2")]
    #[strum(to_string = "katex_caligraphic_regular.woff2")]
    #[allow(non_camel_case_types)]
    Katex_caligraphic_regular,
    #[serde(rename = "katex_fraktur_bold.woff2")]
    #[strum(to_string = "katex_fraktur_bold.woff2")]
    #[allow(non_camel_case_types)]
    Katex_fraktur_bold,
    #[serde(rename = "katex_fraktur_regular.woff2")]
    #[strum(to_string = "katex_fraktur_regular.woff2")]
    #[allow(non_camel_case_types)]
    Katex_fraktur_regular,
    #[serde(rename = "katex_main_bold.woff2")]
    #[strum(to_string = "katex_main_bold.woff2")]
    #[allow(non_camel_case_types)]
    Katex_main_bold,
    #[serde(rename = "katex_main_bolditalic.woff2")]
    #[strum(to_string = "katex_main_bolditalic.woff2")]
    #[allow(non_camel_case_types)]
    Katex_main_bolditalic,
    #[serde(rename = "katex_main_italic.woff2")]
    #[strum(to_string = "katex_main_italic.woff2")]
    #[allow(non_camel_case_types)]
    Katex_main_italic,
    #[serde(rename = "katex_main_regular.woff2")]
    #[strum(to_string = "katex_main_regular.woff2")]
    #[allow(non_camel_case_types)]
    Katex_main_regular,
    #[serde(rename = "katex_math_bolditalic.woff2")]
    #[strum(to_string = "katex_math_bolditalic.woff2")]
    #[allow(non_camel_case_types)]
    Katex_math_bolditalic,
    #[serde(rename = "katex_math_italic.woff2")]
    #[strum(to_string = "katex_math_italic.woff2")]
    #[allow(non_camel_case_types)]
    Katex_math_italic,
    #[serde(rename = "katex_sansserif_bold.woff2")]
    #[strum(to_string = "katex_sansserif_bold.woff2")]
    #[allow(non_camel_case_types)]
    Katex_sansserif_bold,
    #[serde(rename = "katex_sansserif_italic.woff2")]
    #[strum(to_string = "katex_sansserif_italic.woff2")]
    #[allow(non_camel_case_types)]
    Katex_sansserif_italic,
    #[serde(rename = "katex_sansserif_regular.woff2")]
    #[strum(to_string = "katex_sansserif_regular.woff2")]
    #[allow(non_camel_case_types)]
    Katex_sansserif_regular,
    #[serde(rename = "katex_script_regular.woff2")]
    #[strum(to_string = "katex_script_regular.woff2")]
    #[allow(non_camel_case_types)]
    Katex_script_regular,
    #[serde(rename = "katex_size1_regular.woff2")]
    #[strum(to_string = "katex_size1_regular.woff2")]
    #[allow(non_camel_case_types)]
    Katex_size1_regular,
    #[serde(rename = "katex_size2_regular.woff2")]
    #[strum(to_string = "katex_size2_regular.woff2")]
    #[allow(non_camel_case_types)]
    Katex_size2_regular,
    #[serde(rename = "katex_size3_regular.woff2")]
    #[strum(to_string = "katex_size3_regular.woff2")]
    #[allow(non_camel_case_types)]
    Katex_size3_regular,
    #[serde(rename = "katex_size4_regular.woff2")]
    #[strum(to_string = "katex_size4_regular.woff2")]
    #[allow(non_camel_case_types)]
    Katex_size4_regular,
    #[serde(rename = "katex_typewriter_regular.woff2")]
    #[strum(to_string = "katex_typewriter_regular.woff2")]
    #[allow(non_camel_case_types)]
    Katex_typewriter_regular,
    #[serde(rename = "Fira_Code_Regular.ttf")]
    #[strum(to_string = "Fira_Code_Regular.ttf")]
    NerdFont,
}

