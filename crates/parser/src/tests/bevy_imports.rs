// Simple test to check #import parsing
#[test]
fn test_hash_import_parsing() {
    let input = "#import bevy_pbr::forward_io::VertexOutput;";
    
    let result = crate::parse_entrypoint(input, crate::ParseEntryPoint::File, crate::Edition::Wgsl);
    
    assert!(result.errors.is_empty(), "Parse errors: {:?}", result.errors);
}

#[test]
fn test_hash_import_with_alias() {
    let input = "#import bevy_pbr::mesh_view_bindings as view_bindings;";
    
    let result = crate::parse_entrypoint(input, crate::ParseEntryPoint::File, crate::Edition::Wgsl);
    
    assert!(result.errors.is_empty(), "Parse errors: {:?}", result.errors);
}

#[test]
fn test_hash_import_with_collection() {
    let input = "#import bevy_pbr::forward_io::{VertexOutput, FragmentOutput};";
    
    let result = crate::parse_entrypoint(input, crate::ParseEntryPoint::File, crate::Edition::Wgsl);
    
    assert!(result.errors.is_empty(), "Parse errors: {:?}", result.errors);
}

#[test]
fn test_hash_import_multiple() {
    // Multiple imports on separate lines
    let input = "#import bevy_pbr::forward_io::VertexOutput;\n#import bevy_pbr::mesh_view_bindings as view_bindings;\n";
    
    let result = crate::parse_entrypoint(input, crate::ParseEntryPoint::File, crate::Edition::Wgsl);
    
    assert!(result.errors.is_empty(), "Parse errors: {:?}", result.errors);
}
