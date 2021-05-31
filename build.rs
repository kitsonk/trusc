// Copyright 2021 Kitson P. Kelly. All rights reserved. MIT License.

use deno_core::error::anyhow;
use deno_core::op_sync;
use deno_core::serde::Deserialize;
use deno_core::serde_json::json;
use deno_core::JsRuntime;
use deno_core::RuntimeOptions;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OpLoadArgs {
  file_name: String,
}

fn tsc_version() -> String {
  "4.3.2".to_string()
}

fn main() {
  // skip building on docs.rs.
  if env::var_os("DOCS_RS").is_some() {
    return;
  }

  println!("cargo:rustc-env=TS_VERSION={}", tsc_version());

  let c = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
  let o = PathBuf::from(env::var_os("OUT_DIR").unwrap());

  let snapshot_path = o.join("SNAPSHOT.bin");

  let typescript_lib_path =
    c.join("third_party").join("typescript").join("lib");

  let display_root = c.parent().unwrap();
  let tsc_js_path = typescript_lib_path.join("tsc.js");
  println!("cargo:rerun-if-changed={}", tsc_js_path.display());
  let tsc_js_str = typescript_lib_path
    .strip_prefix(display_root)
    .unwrap()
    .display()
    .to_string()
    .replace('\\', "/");
  let main_js_path = c.join("src").join("main.js");
  println!("cargo:rerun-if-changed={}", main_js_path.display());
  let main_js_str = main_js_path
    .strip_prefix(display_root)
    .unwrap()
    .display()
    .to_string()
    .replace('\\', "/");

  let lib_dts_paths: Vec<PathBuf> = fs::read_dir(typescript_lib_path)
    .unwrap()
    .filter_map(|r| {
      let dir_entry = r.unwrap();
      let path = dir_entry.path();
      let is_lib = match path.file_name() {
        Some(path_os_str) => {
          let path_str = path_os_str.to_string_lossy();
          path_str.ends_with("d.ts") && path_str.starts_with("lib.")
        }
        None => false,
      };
      if is_lib {
        Some(path)
      } else {
        None
      }
    })
    .collect();

  for path in lib_dts_paths.iter() {
    println!("cargo:rerun-if-changed={}", path.display());
  }

  let lib_map: HashMap<String, PathBuf> = lib_dts_paths
    .iter()
    .map(|p| {
      (
        format!("asset:///{}", p.file_name().unwrap().to_string_lossy()),
        p.clone(),
      )
    })
    .collect();

  let libs: Vec<String> = lib_map.iter().map(|(k, _)| k.clone()).collect();

  let mut js_runtime = JsRuntime::new(RuntimeOptions {
    will_snapshot: true,
    ..Default::default()
  });
  js_runtime.register_op(
    "op_build_info",
    op_sync(move |_state, _args: (), _: ()| {
      Ok(json!(libs))
    }),
  );
  js_runtime.register_op(
    "op_load",
    op_sync(move |_state, args: OpLoadArgs, _:()| {
      lib_map
        .get(&args.file_name)
        .map(|path| std::fs::read_to_string(path).unwrap())
        .ok_or_else(|| anyhow!("Invalid file name: {}", args.file_name))
    }),
  );
  js_runtime.sync_ops_cache();

  js_runtime
    .execute(
      &format!("trusc:{}", tsc_js_str),
      &fs::read_to_string(&tsc_js_path).unwrap(),
    )
    .unwrap();
  js_runtime
    .execute(
      &format!("trusc:{}", main_js_str),
      &fs::read_to_string(&main_js_path).unwrap(),
    )
    .unwrap();

  let snapshot = js_runtime.snapshot();
  let snapshot_slice: &[u8] = &*snapshot;
  println!("Snapshot size: {}", snapshot_slice.len());
  fs::write(&snapshot_path, snapshot_slice).unwrap();
  println!("Snapshot written to: {}", snapshot_path.display());

  #[cfg(target_os = "windows")]
  {
    let mut res = winres::WindowsResource::new();
    res.set_icon("trusc.ico");
    res.set_language(winapi::um::winnt::MAKELANGID(
      winapi::um::winnt::LANG_ENGLISH,
      winapi::um::winnt::SUBLANG_ENGLISH_US,
    ));
    res.compile().unwrap();
  }
}
