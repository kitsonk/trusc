// Copyright 2021 Kitson P. Kelly. All rights reserved. MIT License.

use deno_core::error::AnyError;
use deno_core::op_sync;
use deno_core::serde::Deserialize;
use deno_core::JsRuntime;
use deno_core::OpState;
use std::path::Path;
use std::env::current_dir;
use std::fs;

#[derive(Debug, Deserialize)]
struct OpPathArgs {
  path: String,
}

fn op_create_directory(
  _state: &mut OpState,
  _args: OpPathArgs,
  _: (),
) -> Result<(), AnyError> {
  Ok(())
}

#[derive(Debug, Deserialize)]
struct OpCreateHashArgs {
  data: String,
}

fn op_create_hash(
  _state: &mut OpState,
  _args: OpCreateHashArgs,
  _: (),
) -> Result<String, AnyError> {
  Ok("".to_string())
}

fn op_delete_file(
  _state: &mut OpState,
  _args: OpPathArgs,
  _: (),
) -> Result<(), AnyError> {
  Ok(())
}

fn op_directory_exists(
  _state: &mut OpState,
  _args: OpPathArgs,
  _: (),
) -> Result<bool, AnyError> {
  Ok(false)
}

fn op_file_exists(
  _state: &mut OpState,
  args: OpPathArgs,
  _: (),
) -> Result<bool, AnyError> {
  let path = Path::new(&args.path);
  Ok(path.is_file())
}

fn op_get_current_directory(
  _state: &mut OpState,
  _args: (),
  _: (),
) -> Result<String, AnyError> {
  current_dir()
    .map(|p| p.to_string_lossy().to_string())
    .map_err(|err| err.into())
}

fn op_get_directories(
  _state: &mut OpState,
  _args: OpPathArgs,
  _: (),
) -> Result<Vec<String>, AnyError> {
  Ok(Vec::new())
}

#[derive(Debug, Deserialize)]
struct OpGetEnvArgs {
  name: String,
}

fn op_get_env(
  _state: &mut OpState,
  _args: OpGetEnvArgs,
  _: (),
) -> Result<Option<String>, AnyError> {
  Ok(None)
}

fn op_get_file_size(
  _state: &mut OpState,
  _args: OpPathArgs,
  _: (),
) -> Result<u32, AnyError> {
  Ok(0)
}

fn op_get_memory_usage(
  _state: &mut OpState,
  _args: (),
  _: (),
) -> Result<u32, AnyError> {
  Ok(0)
}

fn op_get_modified_time(
  _state: &mut OpState,
  _args: OpPathArgs,
  _: (),
) -> Result<u32, AnyError> {
  Ok(0)
}

#[derive(Debug, Deserialize)]
struct OpReadDirectoryArgs {
  extensions: Option<Vec<String>>,
  include: Option<Vec<String>>,
  exclude: Option<Vec<String>>,
  depth: Option<u32>,
}

fn op_read_directory(
  _state: &mut OpState,
  _args: OpReadDirectoryArgs,
  _: (),
) -> Result<Vec<String>, AnyError> {
  Ok(Vec::new())
}

#[derive(Debug, Deserialize)]
struct OpReadFileArgs {
  path: String,
  encoding: String,
}

fn op_read_file(
  _state: &mut OpState,
  args: OpReadFileArgs,
  _: (),
) -> Result<String, AnyError> {
  fs::read_to_string(&args.path).map_err(|err| err.into())
}

#[derive(Debug, Deserialize)]
struct OpSetModifiedTimeArgs {
  path: String,
  time: u32,
}

fn op_set_modified_time(
  _state: &mut OpState,
  _args: OpSetModifiedTimeArgs,
  _: (),
) -> Result<(), AnyError> {
  Ok(())
}

#[derive(Debug, Deserialize)]
enum WatchFileKind {
  FixedPollingInterval = 0,
  PriorityPollingInterval = 1,
  DynamicPriorityPolling = 2,
  FixedChunkSizePolling = 3,
  UseFsEvents = 4,
  UseFsEventsOnParentDirectory = 5,
}

#[derive(Debug, Deserialize)]
enum WatchDirectoryKind {
  UseFsEvents = 0,
  FixedPollingInterval = 1,
  DynamicPriorityPolling = 2,
  FixedChunkSizePolling = 3,
}

#[derive(Debug, Deserialize)]
enum PollingWatchKind {
  FixedInterval = 0,
  PriorityInterval = 1,
  DynamicPriority = 2,
  FixedChunkSize = 3,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WatchOptions {
  watch_file: Option<WatchFileKind>,
  watch_directory: Option<WatchDirectoryKind>,
  fallback_polling: Option<PollingWatchKind>,
  synchronous_watch_directory: Option<bool>,
  exclude_directories: Option<Vec<String>>,
  exclude_files: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct OpWatchDirectoryArgs {
  path: String,
  recursive: bool,
  options: Option<WatchOptions>,
}

fn op_watch_directory(
  _state: &mut OpState,
  _args: OpWatchDirectoryArgs,
  _: (),
) -> Result<u32, AnyError> {
  Ok(1)
}

#[derive(Debug, Deserialize)]
struct OpWatchFileArgs {
  path: String,
  polling_interval: u32,
  options: Option<WatchOptions>,
}

fn op_watch_file(
  _state: &mut OpState,
  _args: OpWatchFileArgs,
  _: (),
) -> Result<u32, AnyError> {
  Ok(2)
}

#[derive(Debug, Deserialize)]
struct OpWriteFileArgs {
  path: String,
  data: String,
  write_byte_order_mark: Option<bool>,
}

fn op_write_file(
  _state: &mut OpState,
  args: OpWriteFileArgs,
  _: (),
) -> Result<(), AnyError> {
  fs::write(args.path, args.data).map_err(|err| err.into())
}

pub(crate) fn register(runtime: &mut JsRuntime) {
  runtime.register_op("op_create_directory", op_sync(op_create_directory));
  runtime.register_op("op_create_hash", op_sync(op_create_hash));
  runtime.register_op("op_delete_file", op_sync(op_delete_file));
  runtime.register_op("op_directory_exists", op_sync(op_directory_exists));
  runtime.register_op("op_file_exists", op_sync(op_file_exists));
  runtime.register_op(
    "op_get_current_directory",
    op_sync(op_get_current_directory),
  );
  runtime.register_op("op_get_directories", op_sync(op_get_directories));
  runtime.register_op("op_get_env", op_sync(op_get_env));
  runtime.register_op("op_get_file_size", op_sync(op_get_file_size));
  runtime.register_op("op_get_memory_usage", op_sync(op_get_memory_usage));
  runtime.register_op("op_get_modified_time", op_sync(op_get_modified_time));
  runtime.register_op("op_read_directory", op_sync(op_read_directory));
  runtime.register_op("op_read_file", op_sync(op_read_file));
  runtime.register_op("op_set_modified_time", op_sync(op_set_modified_time));
  runtime.register_op("op_watch_directory", op_sync(op_watch_directory));
  runtime.register_op("op_watch_file", op_sync(op_watch_file));
  runtime.register_op("op_write_file", op_sync(op_write_file));

  runtime.sync_ops_cache();
}
