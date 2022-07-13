// The goal of this exercise is to create a way:
// - to look up the numerical value associated with a particular color band
// - to convert the numerical value into a string representing color
// - to list the different band colors
use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;
#[repr(usize)]
#[derive(Debug, PartialEq, Copy, Clone, IntEnum, IntoEnumIterator, Eq, Ord, PartialOrd)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    // match _color {
    //     ResistorColor::Black => 0,
    //     ResistorColor:: Brown => 1,
    //     ResistorColor:: Red => 2,
    //     ResistorColor:: Orange => 3,
    //     ResistorColor:: Yellow => 4,
    //     ResistorColor:: Green => 5,
    //     ResistorColor:: Blue => 6,
    //     ResistorColor:: Violet => 7,
    //     ResistorColor:: Grey => 8,
    //     ResistorColor:: White => 9,
    // unimplemented!("convert a color into a numerical representation")
    _color.int_value()
}


pub fn value_to_color_string(value: usize) -> String {
    match ResistorColor::from_int(value) {
        Ok(color) => format!("{:?}", color),
        _ => String::from("value out of range")
    }
}


pub fn colors() -> Vec<ResistorColor> {
    println!("submit test");
    let mut collected = ResistorColor::into_enum_iter().collect::<Vec<ResistorColor>>();
    collected.sort();
    collected
    

}

//submit test