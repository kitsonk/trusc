// Copyright 2021 Kitson P. Kelly. All rights reserved. MIT License.

#![deny(warnings)]

mod ops;

use deno_core::JsRuntime;
use deno_core::RuntimeOptions;
use deno_core::Snapshot;

static TSC_SNAPSHOT: &[u8] =
  include_bytes!(concat!(env!("OUT_DIR"), "/SNAPSHOT.BIN"));

static START_SOURCE: &str = r#"ts.Debug.loggingHost = {
  log: function (_level, s) {
      ts.sys.write("" + (s || "") + ts.sys.newLine);
  }
};
if (ts.Debug.isDebugging) {
  ts.Debug.enableDebugInfo();
}
if (ts.sys.tryEnableSourceMapsForHost && /^development$/i.test(ts.sys.getEnvironmentVariable("NODE_ENV"))) {
  ts.sys.tryEnableSourceMapsForHost();
}
if (ts.sys.setBlocking) {
  ts.sys.setBlocking();
}
ts.executeCommandLine(ts.sys, ts.noop, ts.sys.args);
"#;

fn tsc_snapshot() -> Snapshot {
  Snapshot::Static(TSC_SNAPSHOT)
}

fn main() {
  let mut runtime = JsRuntime::new(RuntimeOptions {
    startup_snapshot: Some(tsc_snapshot()),
    ..Default::default()
  });

  ops::register(&mut runtime);

  runtime
    .execute("[native code]", START_SOURCE)
    .expect("could not run boot");
}
