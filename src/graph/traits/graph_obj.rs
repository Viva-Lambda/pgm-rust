// graph object
//
use crate::graph::traits::generic::{IdChanger, LoadChanger};
use crate::graph::traits::generic::{Identified, Loaded, Named};
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;

/// Promotes anything that is hashable and converted to string to a [GraphObject]
/// This is almost exchangeable with being a [Node]
pub trait GraphObject: Named + Loaded + Identified + LoadChanger + IdChanger {
    /// null constructor
    fn null() -> Self;
}
