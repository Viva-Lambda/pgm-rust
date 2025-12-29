//! generic traits
use crate::graph::traits::utils::{from_borrowed_data, to_borrowed_data};
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::hash::{Hash, Hasher};

/// Promotes anything to something identifiable
pub trait Identified {
    fn id(&self) -> &str;
}
macro_rules! default_identified_impl {
    ($t:ty) => {
        impl Identified for $t
        {
            fn id(&self) &str {
                let id = &self._id;
                    id
            }
        }
    };
}

/// Promotes anything to something identifiable
pub trait IdChanger: Identified + Clone {
    /// set id, notice ref is immutable
    fn set_id(&self, idstr: &str) -> Self;
}

macro_rules! default_idchanger_impl {
    ($t:ty) => {
        impl IdChanger for $t {
            fn set_id(&self, idstr: &str) -> Self {
                let mut this = &self.clone();
                this._id = String::from(idstr);
                this
            }
        }
    };
}

/// Promotes anything to something that has data
pub trait Loaded {
    /// data that is associated to graph object
    fn data(&self) -> HashMap<&str, Vec<&str>>;
}

macro_rules! default_loaded_impl {
    ($t:ty) => {
        impl Loaded for $t {
            fn data(&self) -> HashMap<&str, Vec<&str>> {
                let data = to_borrowed_data(&self._data);
                data
            }
        }
    };
}

pub trait LoadChanger: Loaded + Clone {
    /// set data, notice ref is immutable
    fn set_data(&self, data: HashMap<&str, Vec<&str>>) -> Self;
}

macro_rules! default_loadchanger_impl {
    ($t:ty) => {
        impl LoadChanger for $t {
            fn data(&self, data: HashMap<&str, Vec<&str>>) -> Self {
                let mut this = &self.clone();
                let this._data = from_borrowed_data(data);
                this
            }
        }
    };
}

/// Promotes anything to something that has a name
pub trait Named {
    /// name that is associated to graph object
    fn name(&self) -> String;
}

macro_rules! default_named_impl {
    ($t:ty) => {
        impl Named for $t {
            fn name(&self) -> String {
                stringify!($t).to_string()
            }
        }
    };
}

macro_rules! default_display_identified_impl {
    ($t:ty) => {
        impl fmt::Display for T
        where
            $t: Identified + Named,
        {
            // add code here
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let id = &self.id();
                let name = &self.name();
                write!(f, "<{} id='{}'></{}>", name, id, name)
            }
        }
    };
}

fn render_hashmap(data: &HashMap<&str, Vec<&str>>) -> String {
    let mut result = String::from("<data>\n");
    for (k, vs) in data.iter() {
        let mut kdata = String::from("<");
        kdata.push_str(&k.to_string());
        kdata.push_str(">\n");
        for v in vs.iter() {
            let mut vdata = String::from("<");
            vdata.push_str(v);
            vdata.push_str("/>\n");
            kdata.push_str(&vdata);
        }
        kdata.push_str("</");
        kdata.push_str(k);
        kdata.push_str(">\n");
        result.push_str(&kdata);
    }
    result.push_str("</data>");
    result
}

macro_rules! default_display_load_impl {
    ($t:ty) => {
        impl fmt::Display for T
        where
            $t: Loaded,
        {
            // add code here
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let data = &self.data();
                let result = render_hashmap(&data);
                write!(f, "'{}'", &result)
            }
        }
    };
}

macro_rules! default_display_with_data_impl {
    ($t:ty) => {
        impl fmt::Display for T
        where
            $t: Identified + Named + Loaded,
        {
            // add code here
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let id = &self.id();
                let name = &self.name();
                let data_result = render_hashmap(&self.data());
                write!(f, "<{} id='{}'>\n{}\n</{}>", name, id, data_result, name);
            }
        }
    };
}

macro_rules! default_hash_id_impl {
    ($t:ty) => {
        impl Hash for $t
        where
            $t: Identified,
        {
            fn hash<H: Hasher>(&self, state: &mut H) {
                let id = self.id();
                id.hash(state);
            }
        }
    };
}

macro_rules! default_partial_eq_impl {
    ($t:ty) => {
        impl PartialEq for $t
        where
            $t: Identified,
        {
            fn eq(&self, other: &Self) -> bool {
                // Equality compares ID ONLY (to satisfy hash collision requirements)
                self.id() == other.id()
            }
        }

        impl Eq for $t where $t: Identified {}
    };
}

macro_rules! default_getter_impl {
    ($my_type:ty) => {
        default_named_impl!($my_type);
        default_identified_impl!($my_type);
        default_loaded_impl!($my_type);
    };
}

macro_rules! default_setter_impl {
    ($my_type:ty) => {
        default_idchanger_impl!($my_type);
        default_loadchanger_impl!($my_type);
    };
}

pub(crate) use default_all_impl;
macro_rules! default_all_impl {
    ($my_type:ty) => {
        default_getter_impl!($my_type);
        default_setter_impl!($my_type);
        default_display_with_data_impl!($my_type);
        default_hash_id_impl!($my_type);
        default_partial_eq_impl!($my_type);
    };
}

pub(crate) use default_with_display_impl;
macro_rules! default_with_display_impl {
    ($my_type:ty) => {
        default_getter_impl!($my_type);
        default_setter_impl!($my_type);
        default_display_with_data_impl!($my_type);
    };
}

pub(crate) use default_with_id_display_impl;
macro_rules! default_with_id_display_impl {
    ($my_type:ty) => {
        default_getter_impl!($my_type);
        default_setter_impl!($my_type);
        default_display_identified_impl!($my_type);
    };
}

pub(crate) use default_with_hash_partial_eq_impl;
macro_rules! default_with_hash_partial_eq_impl {
    ($my_type:ty) => {
        default_getter_impl!($my_type);
        default_setter_impl!($my_type);
        default_hash_id_impl!($my_type);
        default_partial_eq_impl!($my_type);
    };
}
