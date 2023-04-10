use std::path::Path;

// Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/detour/mod.rs");
    println!("cargo:rerun-if-changed=src/detour/extern.cpp");
    println!("cargo:rerun-if-changed=src/detour_tile_cache/mod.rs");
    println!("cargo:rerun-if-changed=src/detour_tile_cache/extern.cpp");

    // Use the `cc` crate to build a C file and statically link it.
    cc::Build::new()
        .cpp(true)
        .define("DT_POLYREF64", "1")
        .include(Path::new("recastnavigation/Detour/Include"))
        .file("recastnavigation/Detour/Source/DetourAlloc.cpp")
        .file("recastnavigation/Detour/Source/DetourAssert.cpp")
        .file("recastnavigation/Detour/Source/DetourCommon.cpp")
        .file("recastnavigation/Detour/Source/DetourNavMesh.cpp")
        .file("recastnavigation/Detour/Source/DetourNavMeshBuilder.cpp")
        .file("recastnavigation/Detour/Source/DetourNavMeshQuery.cpp")
        .file("recastnavigation/Detour/Source/DetourNode.cpp")
        .file("src/detour/extern.cpp")
        .compile("detour");

    let tile_cache_includes: [&Path; 2] = [
        Path::new("recastnavigation/Detour/Include"),
        Path::new("recastnavigation/DetourTileCache/Include"),
    ];

    cc::Build::new()
        .cpp(true)
        .define("DT_POLYREF64", "1")
        .includes(tile_cache_includes)
        .file("recastnavigation/DetourTileCache/Source/DetourTileCache.cpp")
        .file("recastnavigation/DetourTileCache/Source/DetourTileCacheBuilder.cpp")
        .file("src/detour_tile_cache/extern.cpp")
        .compile("detour_tile_cache");
}
