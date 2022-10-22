// graph object
//
use std::collections::HashMap;
use std::fmt;

pub trait GraphObject: fmt::Display {
    fn id(&self) -> &String;
    fn data(&self) -> &HashMap<String, Vec<String>>;
}
