# yaml2lua

Convert YAML to Lua table

<div>
  <a href="https://crates.io/crates/yaml2lua"><img alt='Version badge' src='https://img.shields.io/crates/v/yaml2lua.svg'></a>
  <a href="https://crates.io/crates/yaml2lua"><img alt='Downloads badge' src='https://img.shields.io/crates/d/yaml2lua.svg'></a>
  <a href="https://crates.io/crates/yaml2lua"><img alt='License badge' src='https://img.shields.io/crates/l/yaml2lua.svg'></a>
  <a href="https://docs.rs/yaml2lua"><img alt="Docs badge" src="https://img.shields.io/docsrs/yaml2lua"></a>
</div>

## Example:

```rust
use yaml2lua::parse;

let yaml = r#"
string: yaml2lua
int: 420
bool: true

array:
  - abc
  - 123
"#;

let lua = parse(yaml).unwrap();
// Output:
// {
//   ["string"] = "yaml2lua",
//   ["int"] = 420,
//   ["bool"] = true,
//   ["array"] = {
//      "abc",
//      123,
//   },
// }
```

## Notes

- Mappings only support `String`, `Number` and `Bool` keys
- Tagged values are not supported yet
