const { spawnSync, execFileSync } = require('node:child_process');
const { existsSync, mkdirSync } = require('node:fs');
const path = require('node:path');

const root = path.resolve(__dirname, '..');
const dependencies = [
  ['hxnodejs', 'HaxeFoundation/hxnodejs', 'f88685a70b2e4f03dbe0a824ce90ea8b56b27753'],
  ['format', 'HaxeFoundation/format', '775a06f0a7aa64cbd060b5c3ba62f65d7fed684a'],
  ['hscript', 'HaxeFoundation/hscript', '80e3610e15e97dd5e2223c93fa72e49e41c376d7'],
  ['vscode-debugadapter', 'vshaxe/vscode-debugadapter-extern', '384cf87c9b696075ffe1887acc262145823c5e33'],
];

function run(command, args, cwd) {
  const result = spawnSync(command, args, { cwd, stdio: 'inherit' });
  if (result.error) throw result.error;
  if (result.status !== 0) process.exit(result.status ?? 1);
}

mkdirSync(path.join(root, '.deps'), { recursive: true });
for (const [name, repository, revision] of dependencies) {
  const directory = path.join(root, '.deps', name);
  if (!existsSync(directory)) {
    mkdirSync(directory);
    run('git', ['init', '--quiet'], directory);
    run('git', ['fetch', '--depth', '1', `https://github.com/${repository}.git`, revision], directory);
    run('git', ['checkout', '--detach', 'FETCH_HEAD'], directory);
  }
  const actual = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: directory, encoding: 'utf8' }).trim();
  if (actual !== revision) throw new Error(`Unexpected ${name} revision: ${actual}; expected ${revision}`);
}
run('haxe', ['build.hxml'], __dirname);
