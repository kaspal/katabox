// Release versions (docs/packaging.md, "Versions"). Run with `pnpm versions:check`.
//
// 1. The app version lives in three files, and all three must match:
//    Cargo.toml ([workspace.package] version), apps/desktop/package.json and
//    apps/desktop/src-tauri/tauri.conf.json. Cargo.lock must have been refreshed.
// 2. The MSI (WiX) needs a numeric version, `bundle.windows.wix.version`, and Windows
//    Installer compares only the first three fields. It is derived from the app version
//    by `msiVersion` below, so that every later release compares higher:
//      M.m.p            -> M.m.p             (a final release, as is)
//      M.0.0-beta.N     -> (M-1).99.N        (N = 1..99)
//      M.0.0-rc.N       -> (M-1).99.(100+N)  (N = 1..99)
//    so 1.0.0-beta.1 -> 0.99.1 < 1.0.0-beta.2 -> 0.99.2 < 1.0.0-rc.1 -> 0.99.101
//    < 1.0.0 -> 1.0.0 < 1.0.1. Minor version 99 is reserved for those pre-releases.
//    Any other pre-release (alpha, a pre-release of a minor or patch version) is refused:
//    extend the rule, and its self-test below, first.
// The NSIS installer takes the app version as is (semver, pre-releases included).
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.join(path.dirname(fileURLToPath(import.meta.url)), "..");
const read = (p) => fs.readFileSync(path.join(root, p), "utf8");

const SEMVER = /^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-(beta|rc)\.([1-9]\d*))?$/;

/** The MSI version of an app version; throws for a version the rule doesn't cover. */
export function msiVersion(version) {
  const m = SEMVER.exec(version);
  if (!m) throw new Error(`${version}: expected M.m.p, M.0.0-beta.N or M.0.0-rc.N`);
  const [major, minor, patch] = [m[1], m[2], m[3]].map(Number);
  const [stage, n] = [m[4], m[5] === undefined ? 0 : Number(m[5])];
  let fields;
  if (!stage) {
    if (minor === 99) throw new Error(`${version}: minor version 99 is reserved for pre-releases`);
    fields = [major, minor, patch];
  } else {
    if (major < 1 || minor !== 0 || patch !== 0) {
      throw new Error(`${version}: pre-releases are mapped only for M.0.0 (M ≥ 1); extend the rule first`);
    }
    if (n > 99) throw new Error(`${version}: at most 99 betas or release candidates`);
    fields = [major - 1, 99, stage === "beta" ? n : 100 + n];
  }
  const [a, b, c] = fields;
  if (a > 255 || b > 255 || c > 65535) throw new Error(`${version}: ${fields.join(".")} is out of the MSI range`);
  return fields.join(".");
}

/** Semver order for the versions the rule covers (a pre-release sorts before its final). */
function semverCompare(x, y) {
  const parse = (v) => {
    const m = SEMVER.exec(v);
    const stage = m[4] === "beta" ? 1 : m[4] === "rc" ? 2 : 3;
    return [Number(m[1]), Number(m[2]), Number(m[3]), stage, m[5] ? Number(m[5]) : 0];
  };
  const [a, b] = [parse(x), parse(y)];
  for (let i = 0; i < a.length; i++) if (a[i] !== b[i]) return a[i] - b[i];
  return 0;
}

const msiCompare = (x, y) => {
  const [a, b] = [x.split(".").map(Number), y.split(".").map(Number)];
  for (let i = 0; i < 3; i++) if (a[i] !== b[i]) return a[i] - b[i];
  return 0;
};

/** Self-test: a release sequence must stay strictly increasing as MSI versions. */
function selfTest() {
  const sequence = [
    "0.1.0",
    "1.0.0-beta.1",
    "1.0.0-beta.2",
    "1.0.0-beta.99",
    "1.0.0-rc.1",
    "1.0.0-rc.99",
    "1.0.0",
    "1.0.1",
    "1.1.0",
    "1.98.7",
    "2.0.0-beta.1",
    "2.0.0-rc.3",
    "2.0.0",
  ];
  for (let i = 1; i < sequence.length; i++) {
    const [prev, next] = [sequence[i - 1], sequence[i]];
    if (semverCompare(prev, next) >= 0) throw new Error(`self-test: ${prev} should sort before ${next}`);
    if (msiCompare(msiVersion(prev), msiVersion(next)) >= 0) {
      throw new Error(`self-test: MSI ${msiVersion(prev)} (${prev}) is not below ${msiVersion(next)} (${next})`);
    }
  }
  const expect = { "1.0.0-beta.1": "0.99.1", "1.0.0-beta.2": "0.99.2", "1.0.0": "1.0.0", "1.0.0-rc.1": "0.99.101" };
  for (const [v, want] of Object.entries(expect)) {
    if (msiVersion(v) !== want) throw new Error(`self-test: ${v} -> ${msiVersion(v)}, want ${want}`);
  }
  for (const bad of ["1.1.0-beta.1", "1.0.1-rc.1", "0.1.0-beta.1", "1.0.0-alpha.1", "1.99.0", "1.0.0-beta.100"]) {
    let threw = false;
    try {
      msiVersion(bad);
    } catch {
      threw = true;
    }
    if (!threw) throw new Error(`self-test: ${bad} should be refused`);
  }
}

function main() {
  selfTest();
  const errors = [];
  const cargo = /\[workspace\.package\][^[]*?\nversion = "([^"]+)"/.exec(read("Cargo.toml"))?.[1];
  const pkg = JSON.parse(read("apps/desktop/package.json")).version;
  const conf = JSON.parse(read("apps/desktop/src-tauri/tauri.conf.json"));
  const wix = conf.bundle?.windows?.wix?.version;
  if (!cargo) errors.push("Cargo.toml: no [workspace.package] version");
  for (const [file, v] of [
    ["apps/desktop/package.json", pkg],
    ["apps/desktop/src-tauri/tauri.conf.json", conf.version],
  ]) {
    if (v !== cargo) errors.push(`${file}: version ${v}, Cargo.toml has ${cargo}`);
  }
  const lock = /name = "katabox-desktop"\r?\nversion = "([^"]+)"/.exec(read("Cargo.lock"))?.[1];
  if (lock !== cargo) errors.push(`Cargo.lock: katabox-desktop ${lock}, run \`cargo check -p katabox-desktop\``);
  let want;
  try {
    want = msiVersion(cargo);
  } catch (e) {
    errors.push(e.message);
  }
  if (want && wix !== want) errors.push(`tauri.conf.json: bundle.windows.wix.version is ${wix}, the rule gives ${want}`);
  if (errors.length) {
    for (const e of errors) console.error(`error: ${e}`);
    process.exit(1);
  }
  console.log(`versions ok: ${cargo} everywhere; MSI ${want}`);
}

main();
