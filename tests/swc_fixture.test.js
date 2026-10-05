import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import { test } from "node:test";
import { fileURLToPath } from "node:url";
import swc from "@swc/core";

const scriptPath = fileURLToPath(import.meta.url);
const scriptDirectory = path.dirname(scriptPath);
const fixtureRoot = path.resolve(scriptDirectory, "fixture", "ts");
const pluginPath = path.resolve(
  scriptDirectory,
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

function normalizeAst(value) {
  if (Array.isArray(value)) {
    return value.map(normalizeAst);
  }

  if (value === null || typeof value !== "object") {
    return value;
  }

  return Object.fromEntries(
    Object.entries(value)
      .filter(
        ([key]) =>
          ![
            "span",
            "ctxt",
            "start",
            "end",
            "leadingComments",
            "trailingComments",
            "innerComments",
          ].includes(key) &&
          (key !== "raw" || value.type === "TplElement"),
      )
      .map(([key, child]) => [key, normalizeAst(child)]),
  );
}

async function transform(source, config) {
  const options = {
    filename: "fixture.tsx",
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
  const parserOptions = { syntax: "typescript", tsx: true };

  assert.deepEqual(
    normalizeAst(swc.parseSync(actualResult.code, parserOptions)),
    normalizeAst(swc.parseSync(expectedResult.code, parserOptions)),
  );
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