// use std::cell::RefCell;

// use serde::{Deserialize, Serialize};

// use crate::parsers::markdown::heading::MarkdownHeadingStringifiedResult;

// #[typeshare::typeshare]
// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct TocResults(RefCell<Vec<MarkdownHeadingStringifiedResult>>);

// uniffi::custom_newtype!(TocResults, Vec<MarkdownHeadingStringifiedResult>);

// impl TocResults {
//     pub fn new() -> Self {
//         TocResults(RefCell::new(Vec::new()))
//     }

//     fn set_last_heading_recursively(&mut self,
//                                     items: &mut
// Vec<MarkdownHeadingStringifiedResult>,
// item: &mut MarkdownHeadingStringifiedResult) {         if let
// Some(last_heading) = items.last_mut() {             println!("Last heading:
// {:#?}", last_heading);             if item.depth > last_heading.depth {
//                 self.set_last_heading_recursively(&mut item.children, item);
//             } else if item.depth == last_heading.depth {
//                 last_heading.children.push(item.clone());
//             } else {
//                 items.push(item.clone());
//             }
//         } else {
//             let mut x = self.0.borrow_mut();
//             x.push(item.clone());
//         }
//     }

//     pub fn append_heading(&mut self, heading:
// MarkdownHeadingStringifiedResult) {         if let Some(last_heading) =
// self.0.borrow_mut().last_mut() {             println!("Last heading: {:#?}",
// last_heading);         } else {
//             self.0.push(heading);
//         }
//     }
// }
