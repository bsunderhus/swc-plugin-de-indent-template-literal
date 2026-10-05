"use strict";

const assert = require("node:assert/strict");
const { spawnSync } = require("node:child_process");
const fs = require("node:fs");
const path = require("node:path");
const { test } = require("node:test");

const scriptPath = __filename;
const fixtureRoot = path.resolve(__dirname, "fixture", "ts");
const pluginPath = path.resolve(
  __dirname,
  "..",
  "swc_plugin_de_indent_template_literal.wasm",
);
const isWorker = process.argv[2] === "--worker";

function collectInputs(directory) {
  return fs.readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const entryPath = path.join(directory, entry.name);
    return entry.isDirectory()
      ? collectInputs(entryPath)
      : entry.name === "input.ts"
        ? [entryPath]
        : [];
  });
}

async function transform(source, config) {
  const swc = require("@swc/core");
  const options = {
    filename: "fixture.ts",
    swcrc: false,
    configFile: false,
    jsc: {
      parser: { syntax: "typescript", tsx: true },
      target: "es2019",
    },
  };

  if (config) {
    options.jsc.experimental = { plugins: [[pluginPath, config]] };
  }

  return swc.transform(source, options);
}

async function verifyFixture(inputPath) {
  const expectedPath = path.join(path.dirname(inputPath), "output.ts");
  const input = fs.readFileSync(inputPath, "utf8");
  const expected = fs.readFileSync(expectedPath, "utf8");
  const isTabFixture = inputPath.split(path.sep).includes("tab");
  const config = isTabFixture ? { indentStyle: "tab" } : {};

  const actualResult = await transform(input, config);
  const expectedResult = await transform(expected);

  assert.equal(actualResult.code, expectedResult.code);
}

if (isWorker) {
  verifyFixture(process.argv[4]).catch((error) => {
    console.error(error);
    process.exitCode = 1;
  });
} else {
  const fixtures = collectInputs(fixtureRoot).sort();
  assert.ok(fixtures.length > 0, "expected TypeScript fixture inputs");

  for (const inputPath of fixtures) {
    const name = path
      .relative(fixtureRoot, inputPath)
      .split(path.sep)
      .join("/");

    test(name, () => {
      const result = spawnSync(
        process.execPath,
        [scriptPath, "--worker", pluginPath, inputPath],
        {
          encoding: "utf8",
          timeout: 30000,
          killSignal: "SIGKILL",
        },
      );

      if (result.error) {
        throw new Error(`${name}: ${result.error.message}`);
      }

      assert.equal(result.status, 0, result.stderr || result.stdout);
    });
  }
}
