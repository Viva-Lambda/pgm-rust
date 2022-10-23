// graph object
//
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;

pub trait GraphObject: fmt::Display + Hash {
    fn id(&self) -> &String;
    fn data(&self) -> &HashMap<String, Vec<String>>;
}
