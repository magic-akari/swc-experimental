pub fn map_field_type_to_extra_field(field_type: &str) -> &str {
    match field_type {
        "AtomRef" => "atom",
        "OptionalAtomRef" => "optional_atom",
        "BigIntId" => "bigint",
        "bool" => "bool",
        "f64" => "number",

        "TypedSubRange" => "sub_range",
        "TypedNode" => "node",
        "TypedOptionalNode" => "optional_node",
        _ => panic!("Unsupport field type {field_type}"),
    }
}
