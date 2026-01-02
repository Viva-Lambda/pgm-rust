//! generic traits
use std::collections::HashMap;

/// Promotes anything to something identifiable
pub trait Identified {
    fn id(&self) -> &str;
}
macro_rules! default_identified_impl {

    // generic types
    // Usage: (StructName, <T, E>, T: Trait, E: Trait)
    ($name:ident, <$($params:ident),+>, $($gen:tt)*) => {
        impl<$($gen)*> $crate::graph::traits::generic::Identified for
            $name<$($params),+> {
            fn id(&self) -> &str {
                let id = &self._id;
                id
            }
        }
    };

    // simple types
    ($t:ty) => {
        impl $crate::graph::traits::generic::Identified for $t {
            fn id(&self) -> &str {
                let id = &self._id;
                id
            }
        }
    };
}
pub(crate) use default_identified_impl;

/// Promotes anything to something identifiable
pub trait IdChanger: Identified + Clone {
    /// set id, notice ref is immutable
    fn set_id(&self, idstr: &str) -> Self;
}

macro_rules! default_idchanger_impl {
    //
    ($t:ident, <$($params:ident),+>, $($gen:tt)*) => {
        impl<$($gen)*> $crate::graph::traits::generic::IdChanger for
            $t<$($params),+> {
            fn set_id(&self, idstr: &str) -> Self {
                let mut this = self.clone();
                this._id = String::from(idstr);
                this
            }
        }
    };

    // simple type
    ($t:ty) => {
        impl $crate::graph::traits::generic::IdChanger for $t {
            fn set_id(&self, idstr: &str) -> Self {
                let mut this = self.clone();
                this._id = String::from(idstr);
                this
            }
        }
    };
}

pub(crate) use default_idchanger_impl;

/// Promotes anything to something that has data
pub trait Loaded {
    /// data that is associated to graph object
    fn data(&self) -> HashMap<&str, Vec<&str>>;
}

macro_rules! default_loaded_impl {

    // generic type
    ($t:ident, <$($params:ident),+>, $($gen:tt)*) => {
        impl<$($gen)*> $crate::graph::traits::generic::Loaded for $t<$($params),+> {
            fn data(&self) -> HashMap<&str, Vec<&str>> {
                let data = $crate::graph::traits::utils::to_borrowed_data(&self._data);
                data
            }
        }
    };

    // simple type
    ($t:ty) => {
        impl $crate::graph::traits::generic::Loaded for $t {
            fn data(&self) -> HashMap<&str, Vec<&str>> {
                let data = $crate::graph::traits::utils::to_borrowed_data(&self._data);
                data
            }
        }
    };
}
pub(crate) use default_loaded_impl;

pub trait LoadChanger: Loaded + Clone {
    /// set data, notice ref is immutable
    fn set_data(&self, data: HashMap<&str, Vec<&str>>) -> Self;
}

macro_rules! default_loadchanger_impl {

    // generic type
    ($name:ident, <$($params:ident),+>, $($gen:tt)*) => {
        impl<$($gen)*> $crate::graph::traits::generic::LoadChanger for $name<$($params),+> {
            fn set_data(&self, data: HashMap<&str, Vec<&str>>) -> Self {
                let mut this = self.clone();
                this._data = $crate::graph::traits::utils::from_borrowed_data(&data);
                this
            }
        }
    };
    // simple type
    ($t:ty) => {
        impl $crate::graph::traits::generic::LoadChanger for $t {
            fn set_data(&self, data: HashMap<&str, Vec<&str>>) -> Self {
                let mut this = self.clone();
                this._data = $crate::graph::traits::utils::from_borrowed_data(&data);
                this
            }
        }
    };
}
pub(crate) use default_loadchanger_impl;

/// Promotes anything to something that has a name
pub trait Named {
    /// name that is associated to graph object
    fn name(&self) -> String;
}

macro_rules! default_named_impl {

    // generic type
    ($name:ident, <$($params:ident),+>, $($gen:tt)*) => {
        impl<$($gen)*> $crate::graph::traits::generic::Named for $name<$($params),+> {
            fn name(&self) -> String {
                stringify!($t).to_string()
            }
        }
    };

    // simple type
    ($t:ty) => {
        impl $crate::graph::traits::generic::Named for $t {
            fn name(&self) -> String {
                stringify!($t).to_string()
            }
        }
    };
}
pub(crate) use default_named_impl;

macro_rules! default_display_identified_impl {

    // generic type
    ($name:ident, <$($params:ident),+>, $($gen:tt)*) => {
        impl<$($gen)*> fmt::Display for $name<$($params),+>
        {
            // add code here
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let id = &self.id();
                let name = &self.name();
                write!(f, "<{} id='{}'></{}>", name, id, name)
            }
        }
    };

    ($t:ty) => {
        impl fmt::Display for $t
        where
            $t: $crate::graph::traits::generic::Identified + $crate::graph::traits::generic::Named,
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
pub(crate) use default_display_identified_impl;

pub fn render_hashmap(data: &HashMap<&str, Vec<&str>>) -> String {
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

    ($name:ident, <$($params:ident),+>, $($gen:tt)*) => {
        impl<$($gen)*> fmt::Display for $name<$($params),+>
        {
            // add code here
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let data = &self.data();
                let result = $crate::graph::traits::generic::render_hashmap(&data);
                write!(f, "'{}'", &result)
            }
        }
    };

    //
    ($t:ty) => {
        impl fmt::Display for $t
        where
            $t: $crate::graph::traits::generic::Loaded,
        {
            // add code here
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let data = &self.data();
                let result = $crate::graph::traits::generic::render_hashmap(&data);
                write!(f, "'{}'", &result)
            }
        }
    };
}
pub(crate) use default_display_load_impl;

macro_rules! default_display_with_data_impl {
// generic types
    // Usage: (StructName, <T, E>, T: Trait, E: Trait)
    ($name:ident, <$($params:ident),+>, $($gen:tt)*) => {
        impl<$($gen)*> fmt::Display for $name<$($params),+>
        {
            // add code here
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let id = &self.id();
                let name = &self.name();
                let data_result = $crate::graph::traits::generic::render_hashmap(&self.data());
                write!(f, "<{} id='{}'>\n{}\n</{}>", name, id, data_result, name)
            }
        }
    };

    ($t:ty) => {
        impl fmt::Display for $t
        where
            $t: $crate::graph::traits::generic::Identified
                + $crate::graph::traits::generic::Named
                + $crate::graph::traits::generic::Loaded,
        {
            // add code here
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let id = &self.id();
                let name = &self.name();
                let data_result = $crate::graph::traits::generic::render_hashmap(&self.data());
                write!(f, "<{} id='{}'>\n{}\n</{}>", name, id, data_result, name)
            }
        }
    };
}
pub(crate) use default_display_with_data_impl;

macro_rules! default_hash_id_impl {
    ($name:ident, <$($params:ident),+>, $($gen:tt)*) => {
    impl<$($gen)*> std::hash::Hash for $name<$($params),+>
        {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                let id = self.id();
                id.hash(state);
            }
        }
    };

    ($t:ty) => {
        impl std::hash::Hash for $t
        where
            $t: $crate::graph::traits::generic::Identified,
        {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                let id = self.id();
                id.hash(state);
            }
        }
    };
}
pub(crate) use default_hash_id_impl;

macro_rules! default_partial_eq_impl {

    ($name:ident, <$($params:ident),+>, $($gen:tt)*) => {
    impl<$($gen)*> PartialEq for $name<$($params),+>
        {
            fn eq(&self, other: &Self) -> bool {
                // Equality compares ID ONLY (to satisfy hash collision requirements)
                self.id() == other.id()
            }
        }

        impl<$($gen)*> Eq for $name<$($params),+> {}
    };

    ($t:ty) => {
        impl PartialEq for $t
        where
            $t: $crate::graph::traits::generic::Identified,
        {
            fn eq(&self, other: &Self) -> bool {
                // Equality compares ID ONLY (to satisfy hash collision requirements)
                self.id() == other.id()
            }
        }

        impl Eq for $t where $t: $crate::graph::traits::generic::Identified {}
    };
}
pub(crate) use default_partial_eq_impl;

macro_rules! default_getter_impl {

    ($my_type:ident, <$($params:ident),+>, $($gen:tt)*) => {
        default_named_impl!($my_type, <$($params),+>, $($gen)*);
        default_identified_impl!($my_type,<$($params),+>, $($gen)*);
        default_loaded_impl!($my_type, <$($params),+>, $($gen)*);
    };

    ($my_type:ty) => {
        default_named_impl!($my_type);
        default_identified_impl!($my_type);
        default_loaded_impl!($my_type);
    };
}
pub(crate) use default_getter_impl;

macro_rules! default_setter_impl {

    ($my_type:ident, <$($params:ident),+>, $($gen:tt)*) => {
        default_idchanger_impl!($my_type, <$($params),+>, $($gen)*);
        default_loadchanger_impl!($my_type,  <$($params),+>, $($gen)*);
    };

    ($my_type:ty) => {
        default_idchanger_impl!($my_type);
        default_loadchanger_impl!($my_type);
    };
}
pub(crate) use default_setter_impl;

macro_rules! default_all_impl {
    ($my_type:ident, <$($params:ident),+>, $($gen:tt)*) => {
        default_getter_impl!($my_type, <$($params),+>, $($gen)*);
        default_setter_impl!($my_type, <$($params),+>, $($gen)*);
        default_display_with_data_impl!($my_type, <$($params),+>, $($gen)*);
        default_hash_id_impl!($my_type, <$($params),+>, $($gen)*);
        default_partial_eq_impl!($my_type, <$($params),+>, $($gen)*);
    };
    ($my_type:ty) => {
        default_getter_impl!($my_type);
        default_setter_impl!($my_type);
        default_display_with_data_impl!($my_type);
        default_hash_id_impl!($my_type);
        default_partial_eq_impl!($my_type);
    };
}

pub(crate) use default_all_impl;
macro_rules! default_with_display_impl {

    ($my_type:ident, <$($params:ident),+>, $($gen:tt)*) => {
        default_getter_impl!($my_type, <$($params),+>, $($gen)*);
        default_setter_impl!($my_type, <$($params),+>, $($gen)*);
        default_display_with_data_impl!($my_type, <$($params),+>, $($gen)*);
    };
    ($my_type:ty) => {
        default_getter_impl!($my_type);
        default_setter_impl!($my_type);
        default_display_with_data_impl!($my_type);
    };
}
pub(crate) use default_with_display_impl;

macro_rules! default_with_id_display_impl {
    ($my_type:ident, <$($params:ident),+>, $($gen:tt)*) => {
        default_getter_impl!($my_type, <$($params),+>, $($gen)*);
        default_setter_impl!($my_type, <$($params),+>, $($gen)*);
        default_display_identified_impl!($my_type, <$($params),+>, $($gen)*);
    };
    ($my_type:ty) => {
        default_getter_impl!($my_type);
        default_setter_impl!($my_type);
        default_display_identified_impl!($my_type);
    };
}
pub(crate) use default_with_id_display_impl;

macro_rules! default_with_hash_partial_eq_impl {
    ($my_type:ident, <$($params:ident),+>, $($gen:tt)*) => {
        default_getter_impl!($my_type, <$($params),+>, $($gen)*);
        default_setter_impl!($my_type, <$($params),+>, $($gen)*);
        default_hash_id_impl!($my_type, <$($params),+>, $($gen)*);
        default_partial_eq_impl!($my_type, <$($params),+>, $($gen)*);
    };

    ($my_type:ty) => {
        default_getter_impl!($my_type);
        default_setter_impl!($my_type);
        default_hash_id_impl!($my_type);
        default_partial_eq_impl!($my_type);
    };
}
pub(crate) use default_with_hash_partial_eq_impl;
