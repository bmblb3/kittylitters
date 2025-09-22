use indexmap::IndexMap;

pub fn get_first_values<T>(map: &IndexMap<String, Vec<T>>) -> IndexMap<&String, &T> {
    map.iter()
        .filter_map(|(k, v)| v.first().map(|first| (k, first)))
        .collect()
}
