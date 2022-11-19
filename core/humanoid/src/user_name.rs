use player::names::UsedNames;

/// Get a dummy character name as a function.
#[cfg(feature = "server")]
pub fn get_dummy_name(used_names: &mut UsedNames) -> String {
    let return_name = format!("Dummy {}", used_names.dummy_i);

    used_names.dummy_i += 1;

    return_name
}
