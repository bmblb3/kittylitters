use indexmap::IndexMap;

pub fn get_first_values<T>(map: &IndexMap<String, Vec<T>>) -> IndexMap<String, Vec<T>>
where
    T: Clone,
{
    let mut result = IndexMap::new();
    for (key, value) in map.iter() {
        if let Some(first) = value.first() {
            result.insert(key.clone(), vec![first.clone()]);
        }
    }
    result
}
