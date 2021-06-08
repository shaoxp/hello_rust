//! # Art
//! A library for modeling artistics concepts
//! 

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds{
    /// The primary color according to RYB color model
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The Second color according to RYB color model
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils{
    use crate::kinds::*;
    
    /// Combine two primrary colors in equal ammount to create 
    /// a second color.
    pub fn mix(c1:PrimaryColor,c2:PrimaryColor) -> SecondaryColor{
        SecondaryColor::Orange
    }
}