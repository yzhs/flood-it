#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Colour {
    Red,
    Yellow,
    Green,
    LightBrown,
    Purple,
    Cyan,
    Blue,
    Fuchsia,
}

pub const AllColours: [Colour; 8] = [
    Colour::Red,
    Colour::Yellow,
    Colour::Green,
    Colour::LightBrown,
    Colour::Purple,
    Colour::Cyan,
    Colour::Blue,
    Colour::Fuchsia,
];
