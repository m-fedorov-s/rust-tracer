use rstest::rstest;
use rust_tracer::scene::{
    import_material_file, IlluminationModel, Material, ParsingError, Scene, UnderlyingPasingError,
};

// =============================================================================
// import_material_file tests - Success cases
// =============================================================================

#[test]
fn parse_valid_full_mtl() {
    let result = import_material_file("tests/data/example.mtl");
    assert!(result.is_ok(), "Expected Ok, got {:?}", result);

    let materials = result.unwrap();
    assert_eq!(materials.len(), 3, "Expected 3 materials");

    // Verify flatwhite material
    let flatwhite = materials
        .get("flatwhite")
        .expect("flatwhite material not found");
    assert_eq!(flatwhite.ambient, (0.5, 0.5, 0.5));
    assert_eq!(flatwhite.diffuse, (1.0, 1.0, 1.0));
    assert_eq!(
        flatwhite.illumination_model,
        IlluminationModel::ColorAndAmbient
    );

    // Verify shinyred material
    let shinyred = materials
        .get("shinyred")
        .expect("shinyred material not found");
    assert_eq!(shinyred.ambient, (0.1985, 0.0, 0.0));
    assert_eq!(shinyred.diffuse, (0.5921, 0.0167, 0.0));
    assert_eq!(shinyred.spectral, (0.5973, 0.2083, 0.2083));
    assert_eq!(shinyred.specular_exponent, 100.2235);
    assert_eq!(
        shinyred.illumination_model,
        IlluminationModel::ColorHighlight
    );

    // Verify clearblue material
    let clearblue = materials
        .get("clearblue")
        .expect("clearblue material not found");
    assert_eq!(clearblue.ambient, (0.0394, 0.0394, 0.33));
    assert_eq!(clearblue.diffuse, (0.142, 0.142, 0.95));
    assert_eq!(clearblue.transparency, 0.43);
    assert_eq!(
        clearblue.illumination_model,
        IlluminationModel::ColorAndAmbient
    );
}

#[test]
fn parse_optical_density_and_d_transparency() {
    let result = import_material_file("tests/data/optical_density.mtl");
    assert!(result.is_ok(), "Expected Ok, got {:?}", result);

    let materials = result.unwrap();
    let glass = materials.get("glass").expect("glass material not found");

    assert_eq!(glass.optical_density, 1.5);
    // d 0.7 means transparency = 1.0 - 0.7 = 0.3
    assert!((glass.transparency - 0.3).abs() < 1e-10);
}

#[rstest]
#[case(0, IlluminationModel::ColorOnly)]
#[case(1, IlluminationModel::ColorAndAmbient)]
#[case(2, IlluminationModel::ColorHighlight)]
#[case(3, IlluminationModel::ReflectionOn)]
#[case(4, IlluminationModel::TransparencyOn)]
#[case(5, IlluminationModel::ReflectionFresnel)]
#[case(6, IlluminationModel::RefractionAndReflectionOn)]
#[case(7, IlluminationModel::RefractionAndReflectionFresnel)]
#[case(8, IlluminationModel::ReflectionAndRayTraceOff)]
#[case(9, IlluminationModel::TransparencyOnAndRayTraceOff)]
#[case(10, IlluminationModel::Shadows)]
fn parse_illumination_models(#[case] model_num: i32, #[case] expected: IlluminationModel) {
    let result = import_material_file("tests/data/optical_density.mtl");
    assert!(result.is_ok());

    let materials = result.unwrap();
    let material_name = format!("illum{}", model_num);
    let material = materials
        .get(&material_name)
        .expect(&format!("{} material not found", material_name));

    assert_eq!(material.illumination_model, expected);
}

#[test]
fn parse_comments_ignored() {
    // Create a temp file with only comments
    let content = "# This is a comment\n# Another comment\n";
    let temp_path = std::env::temp_dir().join("test_comments.mtl");
    std::fs::write(&temp_path, content).expect("Failed to write temp file");

    let result = import_material_file(temp_path.to_str().unwrap());
    assert!(result.is_ok());

    let materials = result.unwrap();
    assert_eq!(
        materials.len(),
        0,
        "Expected empty map for comments-only file"
    );

    // Cleanup
    let _ = std::fs::remove_file(&temp_path);
}

#[test]
fn parse_empty_file() {
    let content = "";
    let temp_path = std::env::temp_dir().join("test_empty.mtl");
    std::fs::write(&temp_path, content).expect("Failed to write temp file");

    let result = import_material_file(temp_path.to_str().unwrap());
    assert!(result.is_ok());

    let materials = result.unwrap();
    assert_eq!(materials.len(), 0, "Expected empty map for empty file");

    // Cleanup
    let _ = std::fs::remove_file(&temp_path);
}

#[test]
fn parse_whitespace_only_file() {
    let content = "   \n\t\n   \n";
    let temp_path = std::env::temp_dir().join("test_whitespace.mtl");
    std::fs::write(&temp_path, content).expect("Failed to write temp file");

    let result = import_material_file(temp_path.to_str().unwrap());
    assert!(result.is_ok());

    let materials = result.unwrap();
    assert_eq!(
        materials.len(),
        0,
        "Expected empty map for whitespace-only file"
    );

    // Cleanup
    let _ = std::fs::remove_file(&temp_path);
}

#[test]
fn material_default_values() {
    let content = "newmtl default_material\n";
    let temp_path = std::env::temp_dir().join("test_defaults.mtl");
    std::fs::write(&temp_path, content).expect("Failed to write temp file");

    let result = import_material_file(temp_path.to_str().unwrap());
    assert!(result.is_ok());

    let materials = result.unwrap();
    let mat = materials
        .get("default_material")
        .expect("Material not found");

    // Check default values match Material::default()
    let default_mat: Material = Default::default();
    assert_eq!(mat.ambient, default_mat.ambient);
    assert_eq!(mat.spectral, default_mat.spectral);
    assert_eq!(mat.diffuse, default_mat.diffuse);
    assert_eq!(mat.specular_exponent, default_mat.specular_exponent);
    assert_eq!(mat.transparency, default_mat.transparency);
    assert_eq!(mat.optical_density, default_mat.optical_density);
    assert_eq!(mat.illumination_model, default_mat.illumination_model);

    // Cleanup
    let _ = std::fs::remove_file(&temp_path);
}

#[test]
fn material_overwrite() {
    // When the same material name appears twice, the last one should win
    let content = r#"
newmtl mymat
Ka 0.1 0.1 0.1
newmtl mymat
Ka 0.9 0.9 0.9
"#;
    let temp_path = std::env::temp_dir().join("test_overwrite.mtl");
    std::fs::write(&temp_path, content).expect("Failed to write temp file");

    let result = import_material_file(temp_path.to_str().unwrap());
    assert!(result.is_ok());

    let materials = result.unwrap();
    assert_eq!(materials.len(), 1, "Expected only one material");

    let mat = materials.get("mymat").expect("Material not found");
    assert_eq!(
        mat.ambient,
        (0.9, 0.9, 0.9),
        "Expected last definition to win"
    );

    // Cleanup
    let _ = std::fs::remove_file(&temp_path);
}

#[test]
fn ke_token_same_as_ka() {
    // Ke should be handled the same as Ka
    let content = "newmtl emissive\nKe 0.5 0.6 0.7\n";
    let temp_path = std::env::temp_dir().join("test_ke.mtl");
    std::fs::write(&temp_path, content).expect("Failed to write temp file");

    let result = import_material_file(temp_path.to_str().unwrap());
    assert!(result.is_ok());

    let materials = result.unwrap();
    let mat = materials.get("emissive").expect("Material not found");
    assert_eq!(mat.ambient, (0.5, 0.6, 0.7));

    // Cleanup
    let _ = std::fs::remove_file(&temp_path);
}

// =============================================================================
// import_material_file tests - Error cases
// =============================================================================

#[test]
fn error_file_not_found() {
    let result = import_material_file("nonexistent_file_that_does_not_exist.mtl");
    assert!(result.is_err(), "Expected Err for non-existent file");

    let err = result.unwrap_err();
    match err.error {
        UnderlyingPasingError::IoError(_) => {} // Expected
        _ => panic!("Expected IoError, got {:?}", err.error),
    }
}

#[test]
fn error_newmtl_no_name() {
    let content = "newmtl\n";
    let temp_path = std::env::temp_dir().join("test_newmtl_no_name.mtl");
    std::fs::write(&temp_path, content).expect("Failed to write temp file");

    let result = import_material_file(temp_path.to_str().unwrap());
    assert!(result.is_err(), "Expected Err for newmtl without name");

    let err = result.unwrap_err();
    assert_eq!(err.line_number, 0);
    match err.error {
        UnderlyingPasingError::Other(msg) => {
            assert!(msg.contains("Expected material name after newmtl"));
        }
        _ => panic!("Expected Other error, got {:?}", err.error),
    }

    // Cleanup
    let _ = std::fs::remove_file(&temp_path);
}

#[rstest]
#[case("Ka 0.5 0.5", 2)] // Too few args
#[case("Ka 0.5 0.5 0.5 0.5", 4)] // Too many args
fn error_ka_wrong_arg_count(#[case] line: &str, #[case] arg_count: usize) {
    let content = format!("newmtl test\n{}\n", line);
    let temp_path = std::env::temp_dir().join(format!("test_ka_args_{}.mtl", arg_count));
    std::fs::write(&temp_path, &content).expect("Failed to write temp file");

    let result = import_material_file(temp_path.to_str().unwrap());
    assert!(result.is_err(), "Expected Err for wrong Ka argument count");

    let err = result.unwrap_err();
    match err.error {
        UnderlyingPasingError::Other(msg) => {
            println!("{}", msg);
            assert!(msg.contains("Expected three floats"));
            assert!(msg.contains(&format!("{}", arg_count)));
        }
        _ => panic!("Expected Other error, got {:?}", err.error),
    }

    // Cleanup
    let _ = std::fs::remove_file(&temp_path);
}

#[test]
fn error_invalid_float() {
    let content = "newmtl test\nKa 0.5 notafloat 0.5\n";
    let temp_path = std::env::temp_dir().join("test_invalid_float.mtl");
    std::fs::write(&temp_path, content).expect("Failed to write temp file");

    let result = import_material_file(temp_path.to_str().unwrap());
    assert!(result.is_err(), "Expected Err for invalid float");

    let err = result.unwrap_err();
    match err.error {
        UnderlyingPasingError::ParseFloatError(_) => {} // Expected
        _ => panic!("Expected ParseFloatError, got {:?}", err.error),
    }

    // Cleanup
    let _ = std::fs::remove_file(&temp_path);
}

#[test]
fn error_invalid_int_illum() {
    let content = "newmtl test\nillum notanint\n";
    let temp_path = std::env::temp_dir().join("test_invalid_int.mtl");
    std::fs::write(&temp_path, content).expect("Failed to write temp file");

    let result = import_material_file(temp_path.to_str().unwrap());
    assert!(result.is_err(), "Expected Err for invalid int in illum");

    let err = result.unwrap_err();
    match err.error {
        UnderlyingPasingError::ParseIntError(_) => {} // Expected
        _ => panic!("Expected ParseIntError, got {:?}", err.error),
    }

    // Cleanup
    let _ = std::fs::remove_file(&temp_path);
}

#[test]
fn error_unsupported_illum() {
    let content = "newmtl test\nillum 99\n";
    let temp_path = std::env::temp_dir().join("test_unsupported_illum.mtl");
    std::fs::write(&temp_path, content).expect("Failed to write temp file");

    let result = import_material_file(temp_path.to_str().unwrap());
    assert!(result.is_err(), "Expected Err for unsupported illum value");

    let err = result.unwrap_err();
    match err.error {
        UnderlyingPasingError::Other(msg) => {
            assert!(msg.contains("Unsupported illumination model"));
        }
        _ => panic!("Expected Other error, got {:?}", err.error),
    }

    // Cleanup
    let _ = std::fs::remove_file(&temp_path);
}

// =============================================================================
// Scene::from_file tests
// =============================================================================

#[rstest]
#[case("tests/data/example.obj")]
#[case("tests/data/example_with_mtllib.obj")]
fn scene_from_file(#[case] filename: &str) {
    // This test verifies that Scene::from_file can parse a simple OBJ file
    // with vertexes, normals, and light sources without panicking
    let scene = Scene::from_file(filename);
    scene.expect(format!("File {} not parsed", filename).as_str());
}

#[test]
fn scene_from_file_usemtl_unknown() {
    // Create a temp OBJ file with usemtl referencing non-existent material
    let content = r#"
v 0.0 0.0 0.0
usemtl nonexistent
"#;
    let temp_path = std::env::temp_dir().join("test_unknown_usemtl.obj");
    std::fs::write(&temp_path, content).expect("Failed to write temp file");

    let _scene = Scene::from_file(temp_path.to_str().unwrap());
    // Should panic before reaching here

    let err = _scene.unwrap_err();

    assert_eq!(err.line_number, 2);
    match err.error {
        UnderlyingPasingError::Other(msg) => {
            assert!(msg.contains("nonexistent"));
        }
        _ => panic!("Expected Other error, got {:?}", err.error),
    }

    // Cleanup (if we somehow get here)
    let _ = std::fs::remove_file(&temp_path);
}

// =============================================================================
// ParsingError Display and Error trait tests
// =============================================================================

#[test]
fn parsing_error_display() {
    let err = ParsingError::new(
        "test.mtl",
        42,
        UnderlyingPasingError::Other("test error".to_string()),
    );

    let display = format!("{}", err);
    assert!(display.contains("test.mtl"));
    assert!(display.contains("42"));
}

#[test]
fn parsing_error_source() {
    // Test that source() returns the underlying error
    use std::error::Error;

    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "test");
    let err = ParsingError::new("test.mtl", 1, UnderlyingPasingError::IoError(io_err));
    assert!(err.source().is_some());

    let parse_float_err = "not a number".parse::<f64>().unwrap_err();
    let err = ParsingError::new(
        "test.mtl",
        1,
        UnderlyingPasingError::ParseFloatError(parse_float_err),
    );
    assert!(err.source().is_some());

    let parse_int_err = "not a number".parse::<i32>().unwrap_err();
    let err = ParsingError::new(
        "test.mtl",
        1,
        UnderlyingPasingError::ParseIntError(parse_int_err),
    );
    assert!(err.source().is_some());

    let err = ParsingError::new(
        "test.mtl",
        1,
        UnderlyingPasingError::Other("test".to_string()),
    );
    assert!(err.source().is_none());
}

// =============================================================================
// IlluminationModel tests
// =============================================================================

#[test]
fn illumination_model_default() {
    let model: IlluminationModel = Default::default();
    assert_eq!(model, IlluminationModel::ColorOnly);
}

#[test]
fn illumination_model_debug_clone_partial_eq() {
    let model = IlluminationModel::ColorHighlight;
    let cloned = model.clone();
    assert_eq!(model, cloned);

    // Test Debug trait
    let debug_str = format!("{:?}", model);
    assert!(debug_str.contains("ColorHighlight"));
}
