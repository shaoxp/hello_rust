//! # My Crate
//! 
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient

/// Add one to the number of given.
/// # Examples
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
/// assert_eq!(6,answer);
/// ```
/// ```
/// let arg = -1;
/// let answer = my_crate::add_one(arg);
/// assert_eq!(0,answer);
pub fn add_one(x: i32) -> i32{
    x+1
}
