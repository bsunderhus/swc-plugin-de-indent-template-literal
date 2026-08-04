## `swc-plugin-de-indent-template-literal`

## Usage

This plugin is used to de-indent template literal,
very useful for logging errors and warns.

```js
// BEFORE
function someFunction() {
  console.log(/** #__DE-INDENT__ */ `
    <div>
      <p>hello</p>
    </div>
  `);
}
someFunction();
//    <div>
//      <p>hello</p>
//    </div>

// AFTER
function someFunction() {
  console.log(`<div>
  <p>hello</p>
</div>`);
}
someFunction();
// <div>
//   <p>hello</p>
// </div>
```

#### Setup

```sh
npm install --save-dev swc-plugin-de-indent-template-literal @swc/core
```

Then update your `.swcrc` file like below:

```json
{
  "jsc": {
    "experimental": {
      "plugins": [["swc-plugin-de-indent-template-literal", {}]]
    }
  }
}
```

#### Options

##### `indentStyle`

If your project uses tab instead of space, you can set `indentStyle` to `tab`.

- Type: `string`
- Default: `space`
- Valid values: `space`, `tab`

example:

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        [
          "swc-plugin-de-indent-template-literal",
          {
            "indentStyle": "tab"
          }
        ]
      ]
    }
  }
}
```

##### `tag`

If you want to use a custom tag instead of `#__DE-INDENT__`, you can set `tag` to your custom tag.

- Type: `string`
- Default: `#__DE-INDENT__`
- Valid values: any string

example:

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        [
          "swc-plugin-de-indent-template-literal",
          {
            "tag": "de-indent"
          }
        ]
      ]
    }
  }
}
```

```js
// BEFORE
function someFunction() {
  console.log(/** de-indent */ `
    <div>
      <p>hello</p>
    </div>
  `);
}
someFunction();
//    <div>
//      <p>hello</p>
//    </div>

// AFTER
function someFunction() {
  console.log(`<div>
  <p>hello</p>
</div>`);
}
someFunction();
// <div>
//   <p>hello</p>
// </div>
```

#### Maintaining

This crate is never published to crates.io (`publish = false` in `Cargo.toml`);
the only published artifact is the compiled `.wasm` binary distributed via npm.
Keep `Cargo.toml`'s `version` in sync with `package.json`'s `version` on every
release so the two manifests don't drift, even though only the npm version is
consumer-facing.
