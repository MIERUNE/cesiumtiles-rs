use serde::{Deserialize, Serialize};

use cesiumtiles::gltf_extensions::gltf;
use cesiumtiles::gltf_extensions::mesh;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gltf {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub meshes: Vec<Mesh>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<GltfExtension>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
#[serde[rename_all = "camelCase"]]
pub struct Mesh {
    pub primitives: Vec<MeshPrimitive>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
#[serde[rename_all = "camelCase"]]
pub struct MeshPrimitive {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<MeshPrimitiveExtension>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct MeshPrimitiveExtension {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "EXT_mesh_features")]
    pub ext_mesh_features: Option<mesh::ext_mesh_features::ExtMeshFeatures>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "EXT_structural_metadata")]
    pub ext_structural_metadata: Option<mesh::ext_structural_metadata::ExtStructuralMetadata>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct GltfExtension {
    #[serde(rename = "EXT_structural_metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_structural_metadata: Option<gltf::ext_structural_metadata::ExtStructuralMetadata>,
}

#[test]
fn load_gltf_json() {
    for path in glob::glob("./tests/samples/**/*.gltf").unwrap() {
        let path = path.unwrap();
        println!("loading {:?}", path);
        let src = std::fs::read_to_string(path).unwrap();
        let a: Gltf = serde_json::from_str(&src).unwrap();
        let _ = format!("{:?}", a);

        // 'null' should not appear in output
        let a = serde_json::to_string(&a).unwrap();
        assert!(!a.contains("null"));
    }
}
