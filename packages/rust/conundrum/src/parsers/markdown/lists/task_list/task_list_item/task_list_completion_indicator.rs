use crate::{
    lang::{
        lib::ui::ui_types::emphasis::Emphasis,
        runtime::{
            state::conundrum_error_variant::ConundrumModalResult,
            traits::{
                conundrum_input::ConundrumInput, enum_variant_label::EnumVariantLabel,
                icon_representable::IconRepresentable,
            },
        },
    },
    output::html::{
        web_specific_models::{
            data_attributes::DataAttributeKeys, tailwind_class_representable::TailwindClassRepresentable,
        },
        web_specific_traits::data_attribute_representable::DataAttributeRepresentable,
    },
    parsers::parser_trait::ConundrumParser,
};
use lucide_icons::Icon;
use serde::{Deserialize, Serialize};
use winnow::{Parser, token::take};

#[derive(Serialize, Deserialize, strum_macros::Display, strum_macros::EnumString, Debug, Clone)]
#[strum(serialize_all = "kebab-case")]
pub enum TaskListCompletionToken {
    #[serde(rename = "?")]
    #[strum(to_string = "?")]
    Uncertain,
    #[serde(rename = "x")]
    #[strum(to_string = "x")]
    Complete,
    #[serde(rename = "-")]
    #[strum(to_string = "-")]
    Pending,
    #[serde(rename = "!")]
    #[strum(to_string = "!")]
    Important,
    #[serde(rename = " ")]
    #[strum(to_string = " ")]
    Incomplete,
}

impl TaskListCompletionToken {
    pub fn is_checked(&self) -> bool {
        !matches!(self, Self::Incomplete)
    }

    pub fn get_toggled_variant(&self) -> TaskListCompletionToken {
        if self.is_checked() {
            TaskListCompletionToken::Incomplete
        } else {
            TaskListCompletionToken::Complete
        }
    }
}

impl IconRepresentable for TaskListCompletionToken {
    fn is_icon(&self) -> bool {
        match self {
            TaskListCompletionToken::Complete => true,
            TaskListCompletionToken::Pending => true,
            _ => false,
        }
    }

    fn as_icon(&self) -> char {
        match self {
            TaskListCompletionToken::Uncertain => '?',
            TaskListCompletionToken::Complete => Icon::Check.unicode(),
            TaskListCompletionToken::Pending => Icon::Timer.unicode(),
            TaskListCompletionToken::Important => '!',
            TaskListCompletionToken::Incomplete => ' ',
        }
    }
}

impl TailwindClassRepresentable for TaskListCompletionToken {
    fn as_tailwind_class(&self) -> String {
        if matches!(self, TaskListCompletionToken::Incomplete) {
            String::from("border bg-muted")
        } else {
            let emph: Emphasis = self.into();
            emph.to_background_color_classes()
        }
    }
}

impl EnumVariantLabel for TaskListCompletionToken {
    fn to_variant_label(&self) -> String {
        match self {
            TaskListCompletionToken::Uncertain => "uncertain".to_string(),
            TaskListCompletionToken::Complete => "complete".to_string(),
            TaskListCompletionToken::Pending => "pending".to_string(),
            TaskListCompletionToken::Important => "important".to_string(),
            TaskListCompletionToken::Incomplete => "incomplete".to_string(),
        }
    }
}

impl DataAttributeRepresentable for TaskListCompletionToken {
    fn as_html_data_attribute(&self) -> String {
        format!("{}=\"{}\"", DataAttributeKeys::TaskListCompleteStatus, self.to_variant_label())
    }
}

impl ConundrumParser<TaskListCompletionToken> for TaskListCompletionToken {
    fn parse_input_string(input: &mut ConundrumInput) -> ConundrumModalResult<Self> {
        take(1usize).verify_map(|c: &str| {
                        let _c = c.chars().next().unwrap_or(' ');
                        match _c {
                            '?' => Some(TaskListCompletionToken::Uncertain),
                            'x' => Some(TaskListCompletionToken::Complete),
                            '-' => Some(TaskListCompletionToken::Pending),
                            '!' => Some(TaskListCompletionToken::Important),
                            ' ' => Some(TaskListCompletionToken::Incomplete),
                            _ => None,
                        }
                    })
                    .parse_next(input)
    }

    fn matches_first_char(char: char) -> bool {
        ['-', '?', 'x', '!', ' '].contains(&char)
    }
}
