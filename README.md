# HashLink debugger for Zed

A Zed extension backed by the standalone DAP adapter from
[vshaxe/hashlink-debugger](https://github.com/vshaxe/hashlink-debugger).
The Rust extension starts the Haxe/Node.js adapter; the original debugger core
and native binding communicate with HashLink. VS Code is not required.

Supports launch, attach on a local debug port, source and conditional breakpoints,
pause/continue, step over/into/out, threads, call stacks, program output, and
stopping on exceptions. The upstream variable/evaluation functionality is also
retained, but has not been validated in Zed.

## Build and try

Requires Git, Haxe 4, Node.js, HashLink, Rust and a Zed version supporting debugger
extensions. Put `haxe`, `node`, and `hl` on PATH. The bundled upstream native
bindings target x64 Windows, Linux, and Intel macOS; only Windows was verified here.

From this repository:

```sh
cd adapter
npm ci --ignore-scripts
npm run build
cd ..
rustup target add wasm32-wasip1
cargo build --release --target wasm32-wasip1
```

The adapter build downloads pinned Haxe dependencies into `.deps/`, without
changing your global haxelib configuration. `--ignore-scripts` uses the bundled
native binding instead of rebuilding it with node-gyp.

In Zed, open the command palette, run **zed: install dev extension**, and select
this repository. Zed builds the extension for installation. Open this repository
as a project, run **debugger: start**, and select **HashLink example**. Its build
step compiles `examples/Main.hx` with debug information. The example prints `42`,
then throws an exception. Set a breakpoint on the `twice(21)` call to try stepping.

The debugger stops on uncaught exceptions by default. HashLink reports these
without enabling the **Stop on all exceptions** filter. Enable that filter in
Zed's debugger breakpoints view only when you also want to stop on caught throws.
If Zed saved the filter as enabled from an earlier session, disable it once.
Native access violations and stack overflows also cause exception stops.

## Use in another project

Add a configuration to the project's `.zed/debug.json`:

```json
[
  {
    "label": "Debug HashLink",
    "adapter": "HashLink",
    "request": "launch",
    "adapterPath": "D:/zed-hashlink-debugger/adapter/adapter.js",
    "program": "$ZED_WORKTREE_ROOT/bin/game.hl",
    "cwd": "$ZED_WORKTREE_ROOT",
    "classPaths": ["$ZED_WORKTREE_ROOT/src"],
    "args": []
  }
]
```

Change `adapterPath` to your checkout's absolute path and `program` to your
bytecode output. Build your application with `haxe -debug`. Add library source
roots and the Haxe standard-library directory to `classPaths` when needed.
Unlike VS Code's vshaxe integration, this extension does not infer these from HXML.
You can add a Zed `build` task as in this repository's `.zed/debug.json`.

Optional settings include `hl` (runtime path), `env` (merged with the inherited
environment), `port` (default `6112`), and `connectionTimeout` (seconds, default `10`).
Environment values can reference inherited variables with `${env:NAME}`; for
example, `"PATH": "D:/my/runtime;${env:PATH}"` prepends a directory on Windows.
Environment variable names are case-insensitive on Windows. References always
read the inherited environment, before applying this configuration's overrides.
`cwd` defaults to the project root, and `classPaths` defaults to `cwd`.
To avoid repeating `adapterPath`, set Zed's `dap.HashLink.binary` to the absolute
path of `adapter.js`. The extension invokes that JavaScript file with Node.js.

For attach, start your application separately:

```sh
hl --debug-wait --debug 6112 bin/game.hl
```

Use the same configuration with `"request": "attach"` and `"port": 6112`.
The `program` file is still required to read matching bytecode/debug information.
Disconnecting an attached session detaches; stopping a launched session kills its
child process. Attach only supports the local machine.

Stop on entry, automatic HXML discovery, terminal input, and automatic adapter
installation are not implemented. On macOS the upstream adapter disables the
all-exceptions filter because of a known memory-corruption issue; native debugging
also requires a correctly signed HashLink runtime. Linux inherits the upstream
ptrace restrictions. Zed UI integration still needs a manual check.

## Upstream

Sources in `adapter/src`, `adapter/hld`, and `adapter/hldebug-wrapper` come from
upstream revision `71408bf886666a25c314ee2d8fca6cbbe90ff77e` under MIT; its license
is preserved in `adapter/LICENSE.md`. Local changes stop only on uncaught
exceptions by default, remove the VS Code active-window filter, resolve source roots against
the working directory, preserve the launch environment, expose connection timeout,
correct DAP stop-event fields, and make disconnect safe after process exit.
Unused native build dependencies were removed; the original prebuilt binaries
are retained.
