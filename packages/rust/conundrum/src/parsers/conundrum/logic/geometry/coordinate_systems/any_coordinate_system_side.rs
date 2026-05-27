use crate::parsers::conundrum::logic::geometry::coordinate_systems::{
    three_dimensional::three_dimensional_sides::CartesianSides,
    two_dimensional::two_dimensional_sides::{HorizontalSides, VerticalSides},
};

pub enum AnyCoordinateSystemSide {
    Vertical(VerticalSides),
    Horizontal(HorizontalSides),
    Cartesian(CartesianSides),
}
