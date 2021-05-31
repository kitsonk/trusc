// Copyright 2021 Kitson P. Kelly. All rights reserved. MIT License.

import * as _ts from "../third_party/typescript/lib/typescript";

declare global {
  namespace ts {
    export = _ts;
  }

  interface DenoCore {
    // build time only ops
    opSync(name: "op_build_info", args: undefined): string[];
    opSync(name: "op_load", args: OpLoadArgs): string;

    // run time only ops
    opSync(name: "op_create_directory", args: OpPathArgs): void;
    opSync(name: "op_create_hash", args: OpCreateHashArgs): string;
    opSync(name: "op_delete_file", args: OpPathArgs): void;
    opSync(name: "op_directory_exists", args: OpPathArgs): boolean;
    opSync(name: "op_file_exists", args: OpPathArgs): boolean;
    opSync(name: "op_get_current_directory", args: undefined): string;
    opSync(name: "op_get_directories", args: OpPathArgs): string[];
    opSync(name: "op_get_env", args: OpGetEnvArgs): string | undefined;
    opSync(name: "op_get_file_size", args: OpPathArgs): number;
    opSync(name: "op_get_memory_usage", args: undefined): number;
    opSync(name: "op_get_modified_time", args: OpPathArgs): number;
    opSync(name: "op_read_directory", args: OpReadDirectoryArgs): string[];
    opSync(name: "op_read_file", args: OpReadFileArgs): string;
    opSync(name: "op_set_modified_time", args: OpSetModifiedTimeArgs): void;
    opSync(name: "op_watch_directory", args: OpWatchDirectoryArgs): number;
    opSync(name: "op_watch_file", args: OpWatchFileArgs): number;
    opSync(name: "op_write_file", args: OpWriteFileArgs): void;

    // default overload
    opSync<T>(name: string, params: T): unknown;

    print(msg: string, code?: number): void;
  }

  interface OpCreateHashArgs {
    data: string;
  }

  interface OpGetEnvArgs {
    name: string;
  }

  interface OpLoadArgs {
    fileName: string;
  }

  interface OpPathArgs {
    path: string;
  }

  interface OpReadDirectoryArgs extends OpPathArgs {
    extensions?: readonly string[];
    include?: readonly string[];
    exclude?: readonly string[];
    depth?: number;
  }

  interface OpReadFileArgs extends OpPathArgs {
    encoding: string;
  }

  interface OpSetModifiedTimeArgs extends OpPathArgs {
    time: number;
  }

  interface OpWatchDirectoryArgs extends OpPathArgs {
    recursive: boolean;
    options?: ts.WatchOptions;
  }

  interface OpWatchFileArgs extends OpPathArgs {
    pollingInterval: number;
    options?: ts.WatchOptions;
  }

  interface OpWriteFileArgs extends OpPathArgs {
    data: string;
    writeByteOrderMark?: boolean;
  }

  interface ExecOptions {
    args: readonly string[];
    debugFlag: boolean;
  }
}
