fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    #[cfg(feature = "lua51")]
    let version = lua_src::Lua51;
    #[cfg(feature = "lua52")]
    let version = lua_src::Lua52;
    #[cfg(feature = "lua53")]
    let version = lua_src::Lua53;
    #[cfg(feature = "lua54")]
    let version = lua_src::Lua54;

    let artifacts = lua_src::Build::new().build(version);
    let mut has_lib = false;
    let lib_ext = if cfg!(target_env = "msvc") {
        ".lib"
    } else {
        ".a"
    };
    for dir_entry in std::fs::read_dir(artifacts.lib_dir()).unwrap() {
        let entry = dir_entry.unwrap().path();
        if entry.to_path_buf().to_string_lossy().ends_with(lib_ext) {
            has_lib = true;
        }
    }
    assert!(has_lib);
    artifacts.print_cargo_metadata();
}
