// Copyright 2021 Kitson P. Kelly. All rights reserved. MIT License.

// @ts-check
/// <reference path="./trusc.d.ts" />

((window) => {
  /** @type {{ core: DenoCore }} */
  const { core } = window.Deno;

  const ASSET = "asset:///";

  let debugMode = false;
  let logSource = "TRUSC";

  /**
   * @param {boolean} debug
   */
  function setDebug(debug) {
    debugMode = debug;
  }

  /**
   * @param  {...unknown} args
   */
  function debug(...args) {
    if (debugMode) {
      const stringifiedArgs = args.map((arg) =>
        typeof arg === "string" ? arg : JSON.stringify(arg)
      ).join(" ");
      core.print(`DEBUG ${logSource} - ${stringifiedArgs}\n`, 1);
    }
  }

  /**
   * @param  {...unknown} args
   */
  function error(...args) {
    const stringifiedArgs = args.map((arg) =>
      typeof arg === "string" ? arg : JSON.stringify(arg)
    ).join(" ");
    core.print(`DEBUG ${logSource} - ${stringifiedArgs}\n`, 1);
  }

  class AssertionError extends Error {
    /**
     * @param {string=} msg
     */
    constructor(msg) {
      super(msg);
      this.name = "AssertionError";
    }
  }

  /**
   * @param {unknown} cond
   * @param {string=} msg
   * @returns {asserts cond}
   */
  function assert(cond, msg = "Assertion failed.") {
    if (!cond) {
      throw new AssertionError(msg);
    }
  }

  /** @type {Map<string, string>} */
  const libDataCache = new Map();

  /**
   * @implements {ts.FileWatcher}
   */
  class FileWatcher {
    /** @type {ts.FileWatcherCallback} */
    #callback;
    /** @type {number} */
    #rid;

    /**
     * @param {ts.FileWatcherCallback} callback
     * @param {number} rid
     */
    constructor(callback, rid) {
      this.#callback = callback;
      this.#rid = rid;
    }

    close() {
      core.opSync("op_watch_close", { rid: this.#rid });
    }
  }

  /**
   * @type {ts.System}
   */
  const sys = {
    get args() {
      return [];
    },
    get newLine() {
      return "\n";
    },
    get useCaseSensitiveFileNames() {
      return true;
    },
    write(s) {
      core.print(s);
    },
    writeOutputIsTTY() {
      return true;
    },
    readFile(path, encoding = "utf8") {
      debug("op_read_file", { path, encoding });
      if (path.startsWith(ASSET)) {
        return libDataCache.get(path);
      } else {
        return core.opSync("op_read_file", { path, encoding });
      }
    },
    getFileSize(path) {
      return core.opSync("op_get_file_size", { path });
    },
    writeFile(path, data, writeByteOrderMark) {
      debug("op_write_file", { path, data, writeByteOrderMark });
      core.opSync("op_write_file", { path, data, writeByteOrderMark });
    },
    watchFile(path, callback, pollingInterval, options) {
      return new FileWatcher(
        callback,
        core.opSync("op_watch_file", {
          path,
          pollingInterval,
          options,
        }),
      );
    },
    watchDirectory(path, callback, recursive, options) {
      return new FileWatcher(
        callback,
        core.opSync("op_watch_directory", {
          path,
          recursive,
          options,
        }),
      );
    },
    resolvePath(path) {
      return path;
    },
    fileExists(path) {
      debug("op_file_exists", { path });
      if (path.startsWith(ASSET)) {
        return libDataCache.has(path.replace(ASSET, ""));
      } else {
        return core.opSync("op_file_exists", { path });
      }
    },
    directoryExists(path) {
      return core.opSync("op_directory_exists", { path });
    },
    createDirectory(path) {
      core.opSync("op_create_directory", { path });
    },
    getExecutingFilePath() {
      return ASSET;
    },
    getCurrentDirectory() {
      return core.opSync("op_get_current_directory", undefined);
    },
    getDirectories(path) {
      return core.opSync("op_get_directories", { path });
    },
    readDirectory(path, extensions, exclude, include, depth) {
      return core.opSync("op_read_directory", {
        path,
        extensions,
        exclude,
        include,
        depth,
      });
    },
    getModifiedTime(path) {
      return new Date(core.opSync("op_get_modified_time", { path }));
    },
    setModifiedTime(path, time) {
      core.opSync("op_set_modified_time", { path, time: time.getTime() });
    },
    deleteFile(path) {
      core.opSync("op_delete_file", { path });
    },
    createHash(data) {
      return core.opSync("op_create_hash", { data });
    },
    createSHA256Hash(data) {
      return core.opSync("op_create_hash", { data });
    },
    getMemoryUsage() {
      return core.opSync("op_get_memory_usage", undefined);
    },
    exit(exitCode) {},
    realpath(path) {
      return path;
    },
    getEnvironmentVariable(name) {
      return core.opSync("op_get_env", { name });
    },
    tryEnableSourceMapsForHost() {},
    get debugMode() {
      return debugMode;
    },
    setTimeout(callback) {},
    clearTimeout(timeoutId) {},
    clearScreen() {},
    setBlocking() {},
    base64decode(input) {},
    base64encode(input) {},
    now() {
      return core.opSync("op_performance_now", undefined);
    },
    disableUseFileVersionAsSignature() {},
    defaultWatchFileKind() {
      return 3;
    },
  };

  const libs = core.opSync("op_build_info", undefined);
  for (const fileName of libs) {
    const result = core.opSync("op_load", { fileName });
    assert(
      result !== null,
      `"data" is unexpectedly null for "${fileName}".`,
    );
    libDataCache.set(fileName, result);
  }

  globalThis.trusc = {
    setDebug,
  };

  if (globalThis.ts) {
    Object.assign(globalThis.ts, {
      sys,
    });
  }
})(this);
