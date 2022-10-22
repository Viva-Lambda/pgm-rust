// graph object
//
use std::collections::HashMap;

pub trait GraphObject {
    fn id(&self) -> String;
    fn data(&self) -> HashMap<String, Vec<String>>;
}
