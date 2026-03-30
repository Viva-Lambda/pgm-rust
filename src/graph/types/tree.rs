// tree trait
//! A base graph which implements the Graph trait for doing graph theoretical
//! operations

use crate::errors::{PGMRustError, PGMRustResult};
use crate::graph::traits::edge::Edge as EdgeTrait;
use crate::graph::traits::graph::Graph as GraphTrait;
use crate::graph::traits::graph_obj::GraphObject as GraphObjectTrait;
use crate::graph::traits::node::Node as NodeTrait;
use crate::graph::traits::path::Path as PathTrait;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::marker::PhantomData;
use crate::graph::traits::generic::default_with_hash_partial_eq_impl;

use crate::graph::traits::generic::{ // required for main macro
    default_getter_impl, default_hash_id_impl,
    default_idchanger_impl, default_identified_impl, default_loadchanger_impl, default_loaded_impl,
    default_named_impl, default_partial_eq_impl, default_setter_impl,
};

use crate::graph::traits::generic::Identified;


