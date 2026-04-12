use serde::Serialize;
use winnow::error::ErrMode;

use crate::{
    lang::{
        lib::ui::ui_traits::jsx_prop_representable::{FromJsxPropsOptional, JsxPropRepresentable},
        runtime::state::{
            conundrum_error::ConundrumError,
            conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
        },
    },
    parsers::conundrum::logic::{
        bool::boolean::ConundrumBoolean, number::conundrum_int::ConundrumInt, object::object::ConundrumObject,
        string::conundrum_string::ConundrumString,
    },
};

#[typeshare::typeshare]
#[derive(Serialize, Debug, Clone)]
#[serde(tag = "tag", content = "content")]
pub enum GridColumnPropUnion {
    Number(ConundrumInt),
    String(ConundrumString),
}

impl JsxPropRepresentable for GridColumnPropUnion {
    fn to_jsx_prop(&self, key: &str) -> String {
        match self {
            GridColumnPropUnion::String(s) => s.to_jsx_prop(key),
            GridColumnPropUnion::Number(n) => n.to_jsx_prop(key),
        }
    }
}

impl FromJsxPropsOptional for GridColumnPropUnion {
    fn from_jsx_props(props: &ConundrumObject, key: &str) -> ConundrumModalResult<Self>
        where Self: Sized {
        if let Ok(n_int) = ConundrumInt::from_jsx_props(&props, key) {
            Ok(GridColumnPropUnion::Number(n_int))
        } else if let Ok(s) = ConundrumString::from_jsx_props(&props, key) {
            Ok(GridColumnPropUnion::String(s))
        } else {
            Err(
                ErrMode::Backtrack(ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Property error", format!("Conundrum expected either an integer or a string at the `{}` key but found something else.", key).as_str())))
            )
        }
    }
}

#[typeshare::typeshare]
#[derive(Serialize, Debug, Clone)]
pub struct GridColumnGroup {
    pub none: Option<GridColumnPropUnion>,
    pub small: Option<GridColumnPropUnion>,
    pub smedium: Option<GridColumnPropUnion>,
    pub medium: Option<GridColumnPropUnion>,
    pub large: Option<GridColumnPropUnion>,
    pub xl: Option<GridColumnPropUnion>,
    pub xxl: Option<GridColumnPropUnion>,
    pub full: Option<GridColumnPropUnion>,
    pub fit: Option<ConundrumBoolean>,
    pub responsive: Option<ConundrumBoolean>,
}

impl JsxPropRepresentable for GridColumnGroup {
    fn to_jsx_prop(&self, _: &str) -> String {
        let mut props: Vec<String> = Vec::new();
        if let Some(none) = &self.none {
            props.push(none.to_jsx_prop("none"));
        }

        if let Some(small) = &self.small {
            props.push(small.to_jsx_prop("small"));
        }
        if let Some(smedium) = &self.smedium {
            props.push(smedium.to_jsx_prop("smedium"));
        }
        if let Some(medium) = &self.medium {
            props.push(medium.to_jsx_prop("medium"));
        }
        if let Some(large) = &self.large {
            props.push(large.to_jsx_prop("large"));
        }
        if let Some(xl) = &self.xl {
            props.push(xl.to_jsx_prop("xl"));
        }
        if let Some(xxl) = &self.xxl {
            props.push(xxl.to_jsx_prop("xxl"));
        }
        if let Some(full) = &self.full {
            props.push(full.to_jsx_prop("full"));
        }

        if let Some(fg) = self.fit {
            props.push(fg.to_jsx_prop("fit"));
        }

        if let Some(fg) = self.responsive {
            props.push(fg.to_jsx_prop("responsive"));
        }

        props.join(" ")
    }
}

impl FromJsxPropsOptional for GridColumnGroup {
    fn from_jsx_props(props: &ConundrumObject, _: &str) -> ConundrumModalResult<Self>
        where Self: Sized {
        let none = GridColumnPropUnion::from_jsx_props(props, "none").ok();
        let small = GridColumnPropUnion::from_jsx_props(props, "small").ok();
        let smedium = GridColumnPropUnion::from_jsx_props(props, "smedium").ok();
        let medium = GridColumnPropUnion::from_jsx_props(props, "medium").ok();
        let large = GridColumnPropUnion::from_jsx_props(props, "large").ok();
        let xl = GridColumnPropUnion::from_jsx_props(props, "xl").ok();
        let xxl = GridColumnPropUnion::from_jsx_props(props, "xxl").ok();
        let full = GridColumnPropUnion::from_jsx_props(props, "full").ok();
        let fit = ConundrumBoolean::from_jsx_props(props, "fit").ok();
        let responsive = ConundrumBoolean::from_jsx_props(props, "responsive").ok();
        Ok(GridColumnGroup { none,
                             small,
                             smedium,
                             medium,
                             large,
                             xl,
                             xxl,
                             full,
                             fit,
                             responsive })
    }
}
