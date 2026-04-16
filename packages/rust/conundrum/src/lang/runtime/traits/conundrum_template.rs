use askama::{FastWritable, Template};

pub trait ConundrumTemplate: Template + FastWritable {}
