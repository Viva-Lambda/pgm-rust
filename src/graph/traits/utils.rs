// some utility functions for types
use std::collections::HashMap;

/// Converts a HashMap containing borrowed string slices to a new HashMap
/// that owns all its data.
///
/// This function performs a deep copy, iterating over the input map and
/// cloning every key (`&str`) into a new `String` and every element in the
/// `Vec<&str>` value into a new `Vec<String>`. The resulting map is entirely
/// independent of the input map.
///
/// # Arguments
///
/// * `hmap` - A reference to the input HashMap with borrowed keys and values.
///
/// # Returns
///
/// A new `HashMap<String, Vec<String>>` where all data is owned.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use pgm_rust::graph::traits::utils::from_borrowed_data;
/// // Assume from_borrowed_data is available in the current scope
/// let borrowed_map: HashMap<&str, Vec<&str>> = HashMap::from([
///     ("key1", vec!["a", "b"]),
/// ]);
///
/// let owned_map = from_borrowed_data(&borrowed_map);
/// assert_eq!(owned_map["key1"], vec!["a".to_string(), "b".to_string()]);
/// ```
pub fn from_borrowed_data(hmap: &HashMap<&str, Vec<&str>>) -> HashMap<String, Vec<String>> {
    let data_iter = hmap.into_iter();
    let data_mapped = data_iter.map(|(key, value)| {
        let new_key = key.to_string();
        let new_value = value.into_iter().map(|v| v.to_string()).collect();
        (new_key, new_value)
    });
    let data = data_mapped.collect();
    data
}

/// Creates a new HashMap that borrows the key and value data from the
/// input HashMap.
///
/// This is used to create a lightweight, non-owning view of an existing owned map
/// (`HashMap<String, Vec<String>>`). The keys and vector elements in the
/// returned map are references (`&str`) that point back to the strings
/// held in the original `hmap`.
///
/// The resulting map's lifetime is guaranteed to be no longer than the lifetime
/// of the input map via the `'data_lifetime` parameter.
///
/// # Type Parameters
///
/// * `'data_lifetime` - The lifetime of the data in the input HashMap.
///
/// # Arguments
///
/// * `hmap` - A reference to the input HashMap containing owned String data.
///
/// # Returns
///
/// A new `HashMap<&str, Vec<&str>>` that borrows its data from `hmap`.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use pgm_rust::graph::traits::utils::to_borrowed_data;
/// // Assume to_borrowed_data is available in the current scope
/// let owned_map: HashMap<String, Vec<String>> = HashMap::from([
///     ("key1".to_string(), vec!["a".to_string(), "b".to_string()]),
/// ]);
///
/// let borrowed_map = to_borrowed_data(&owned_map);
/// assert_eq!(borrowed_map["key1"], vec!["a", "b"]);
/// ```
pub fn to_borrowed_data<'data_lifetime>(
    hmap: &'data_lifetime HashMap<String, Vec<String>>,
) -> HashMap<&'data_lifetime str, Vec<&'data_lifetime str>> {
    let data_iter = hmap.into_iter();
    let data_mapped = data_iter.map(|(key, value)| {
        let new_key = &key[..];
        let new_value = value.into_iter().map(|v| &v[..]).collect();
        (new_key, new_value)
    });
    let data = data_mapped.collect();
    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_borrowed_hashmap_conversion() {
        // --- 1. SETUP INPUT HASHMAP ---
        // HashMap<&str, Vec<&str>>
        let input_data: HashMap<&str, Vec<&str>> = HashMap::from([
            ("countries", vec!["USA", "Canada", "Mexico"]),
            ("colors", vec!["Red", "Green", "Blue"]),
            ("empty", vec![]),
        ]);

        // --- 2. DEFINE EXPECTED OUTPUT HASHMAP ---
        // HashMap<String, Vec<String>>
        let expected_data: HashMap<String, Vec<String>> = HashMap::from([
            (
                "countries".to_string(),
                vec![
                    "USA".to_string(),
                    "Canada".to_string(),
                    "Mexico".to_string(),
                ],
            ),
            (
                "colors".to_string(),
                vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()],
            ),
            ("empty".to_string(), vec![]),
        ]);

        // --- 3. CALL THE FUNCTION ---
        let actual_data = from_borrowed_data(&input_data);

        // --- 4. ASSERTION ---
        // For HashMaps, using `assert_eq!` compares all keys and values.
        assert_eq!(
            actual_data, expected_data,
            "The converted HashMap did not match the expected structure and ownership."
        );

        // --- 5. VERIFY OWNERSHIP (Optional but good practice) ---
        // We can assert that the types match the expected owned types.
        // If the code above compiles and the assertion passes, ownership is correct,
        // but we'll print a confirmation just to be explicit about the type:
        let _: HashMap<String, Vec<String>> = actual_data;
    }

    #[test]
    fn test_from_borrowed_hashmap_empty() {
        let input_data: HashMap<&str, Vec<&str>> = HashMap::new();
        let expected_data: HashMap<String, Vec<String>> = HashMap::new();

        let actual_data = from_borrowed_data(&input_data);

        assert_eq!(
            actual_data, expected_data,
            "Should return an empty HashMap when given an empty input."
        );
    }

    #[test]
    fn test_to_borrowed_data_conversion() {
        // 1. SETUP INPUT HASHMAP (Owned Data)
        let owned_data: HashMap<String, Vec<String>> = HashMap::from([
            (
                "fruits".to_string(),
                vec![
                    "apple".to_string(),
                    "banana".to_string(),
                    "kiwi".to_string(),
                ],
            ),
            (
                "devices".to_string(),
                vec!["laptop".to_string(), "phone".to_string()],
            ),
            ("empty".to_string(), vec![]),
        ]);

        // 2. DEFINE EXPECTED OUTPUT HASHMAP (Borrowed Data)
        // Note: The values here are &str literals, which is exactly what we expect
        // the slices to look like.
        let expected_data: HashMap<&str, Vec<&str>> = HashMap::from([
            ("fruits", vec!["apple", "banana", "kiwi"]),
            ("devices", vec!["laptop", "phone"]),
            ("empty", vec![]),
        ]);

        // 3. CALL THE FUNCTION
        let actual_data = to_borrowed_data(&owned_data);

        // 4. ASSERTION
        assert_eq!(
            actual_data, expected_data,
            "The borrowed HashMap did not correctly mirror the owned data and values."
        );

        // 5. LIFETIME CHECK (Implicit)
        // The test ensures the actual_data can be used as long as owned_data is in scope.
    }

    #[test]
    fn test_to_borrowed_data_empty() {
        // Test case for an empty input map
        let owned_data: HashMap<String, Vec<String>> = HashMap::new();
        let expected_data: HashMap<&str, Vec<&str>> = HashMap::new();

        let actual_data = to_borrowed_data(&owned_data);

        assert_eq!(
            actual_data, expected_data,
            "Should return an empty borrowed HashMap when given an empty input."
        );
    }
}
