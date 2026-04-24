use dashmap::DashMap;
use strum::IntoEnumIterator;
use winnow::error::ErrMode;

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        lib::ui::{
            shared_props::sizable_option::SizableOption, ui_traits::jsx_prop_representable::FromJsxPropsOptional,
        },
        runtime::state::{
            conundrum_error::ConundrumError,
            conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
        },
    },
    parsers::conundrum::logic::{
        number::{conundrum_int::ConundrumInt, conundrum_number::ConundrumNumber},
        object::object::ConundrumObject,
        token::ConundrumLogicToken,
    },
};

pub type GridColumnsMap = DashMap<SizableOption, ConundrumInt>;

#[typeshare::typeshare]
#[derive(serde::Serialize, Debug, Clone)]
pub struct GridColumnProps(GridColumnsMap);

impl GridColumnProps {
    pub fn to_css_classes(&self) -> String {
        let mut classes: Vec<String> = Vec::new();
        for size in SizableOption::iter() {
            if let Some(size_data) = self.0.get(&size) {
                let size_value = size_data.value();
                let class = size.to_css_column_class(Some(size_value.0 as i32));
                classes.push(class);
            }
        }
        classes.join(" ")
    }

    /// From an object when the user inserts a key of `columns` to create a
    /// static, non-responsive layout.
    pub fn from_grid_column_props(data: &ConundrumObject) -> ConundrumModalResult<Self> {
        if let Some(res) = data.data.get("columns") {
            if let Ok(res_data) = match res.value() {
                ParsedElement::Logic(l) => match l {
                    ConundrumLogicToken::Number(n) => match n {
                        ConundrumNumber::Int(w) => Ok(w),
                        _ => Err(
                            ErrMode::Backtrack(
                                ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Invalid number", "You provided a float where an integer belongs. Please provide all columns as an integer between 1 and 12."))
                            )
                        )
                    },
                        _=> Err(
                            ErrMode::Backtrack(
                                ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Invalid number", "The grid accepts columns as a map of `Map<SizableOption, Option<ConundrumInt>>`. This means that for each sizable option, there is an optional integer that you can provide, but I don't know what to do with anything else..."))
                            )
                        )
                },
                        _=> Err(
                            ErrMode::Backtrack(
                                ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Invalid number", "You provided an invalid type where an integer belongs. Please provide all columns as an integer between 1 and 12."))
                            )
                        )
            } {
                let m: DashMap<SizableOption, ConundrumInt> = DashMap::new();
                SizableOption::iter().for_each(|size| {
                    m.insert(size, res_data.clone());
                });
                Ok(GridColumnProps(m))
            } else {
            Err(ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Missing property",
                                                                                                             "Conundrum was looking for a valid `column` property and could not find one."))))
            }
        } else {
            Err(ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Missing property",
                                                                                                             "Conundrum was looking for a valid `column` property and could not find one."))))
        }
    }

    /// From an object where the columns are represented individually for a
    /// responsive layout.
    pub fn from_grid_responsive_props(data: &ConundrumObject) -> ConundrumModalResult<Self> {
        let mut have_found_column = false;
        let column_data: DashMap<SizableOption, ConundrumInt> = DashMap::new();
        for size in SizableOption::iter() {
            if let Some(size_data) = data.data.get(&size.to_string()) {
                let size_value = match size_data.value() {
                    ParsedElement::Logic(l ) => match l {
                        ConundrumLogicToken::Number(n) => match n {
                            ConundrumNumber::Int(w) => Ok(w),
                            _ => Err(
                                ErrMode::Backtrack(
                                    ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Invalid number", "You provided a float where an integer belongs. Please provide all columns as an integer between 1 and 12."))
                                )
                            )
                        },
                        _ => Err(
                            ErrMode::Backtrack(
                                ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Invalid number", "Conundrum requires at least one sizable property be set to a number so that we can determine the break-points that your grid should respond to. Try setting something like `large={2}` to create 2 columns on 'large' screen sizes."))
                            )
                        )
                    },
                    _ => Err(
                        ErrMode::Backtrack(
                            ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Invalid number", "You provided a float where an integer belongs. Please provide all columns as an integer between 1 and 12."))
                        )
                    )
                };
                println!("Size Value: {:#?}", size_value);
                if let Some(err) = match size_value {
                    Ok(s) => {
                        have_found_column = true;
                        column_data.insert(size, *s);
                        None
                    }
                    Err(e) => Some(e),
                } {
                    return Err(err);
                }
            }
        }
        if have_found_column {
            Ok(GridColumnProps(column_data))
        } else {
            Err(ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Missing or Invalid Property",
                                                                                                             "Conundrum was looking for at least one sizable option set to a number indicating the number of columns to use. Something like `large={2}` to use 2 columns on 'large' screen sizes."))))
        }
    }
}

impl FromJsxPropsOptional for GridColumnProps {
    fn from_jsx_props(props: &ConundrumObject, key: &str) -> ConundrumModalResult<Self>
        where Self: Sized {
        if let Ok(from_static_props) = GridColumnProps::from_grid_column_props(&props) {
            Ok(from_static_props)
        } else if let Ok(from_responsive_props) = GridColumnProps::from_grid_responsive_props(&props) {
            Ok(from_responsive_props)
        } else {
            Err(ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Missing or Invalid Props",
                                                                                                             "The grid component is looking for some valid columns values. Either provide a number for at least one `SizableOption` or a number for a `column` property to create a static, non-responsive layout. See the `Grid??` docs for more information."))))
        }
    }
}

impl GridColumnProps {
    pub fn get_smaller_item_or_default(idx: usize,
                                       data: &DashMap<SizableOption, Option<ConundrumInt>>,
                                       default_value: ConundrumInt)
                                       -> ConundrumInt {
        let iterable = SizableOption::iter();
        if idx >= iterable.len() {
            return default_value;
        };
        let size = &iterable.collect::<Vec<SizableOption>>()[idx];
        if let Some(val) = data.get(&size) {
            if let Some(user_opt) = val.value().clone() {
                return user_opt;
            }
        }
        if idx < 1 {
            return default_value;
        }
        return GridColumnProps::get_smaller_item_or_default(idx - 1, data, default_value);
    }
}
