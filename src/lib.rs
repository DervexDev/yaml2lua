//! # yaml2lua
//!
//! Convert YAML to Lua table
//!
//! ## Example:
//! ```rust
//! use yaml2lua::parse;
//!
//! let yaml = r#"
//! string: yaml2lua
//! int: 420
//! bool: true
//!
//! array:
//!   - abc
//!   - 123
//! "#;
//!
//! let lua = parse(yaml).unwrap();
//! // Output:
//! // {
//! //   ["string"] = "yaml2lua",
//! //   ["int"] = 420,
//! //   ["bool"] = true,
//! //   ["array"] = {
//! //      "abc",
//! //      123,
//! //   },
//! // }
//! ```
//!
//! Made with <3 by Dervex

#![allow(clippy::tabs_in_doc_comments)]

use indexmap::IndexMap;
use serde::Deserialize;
use serde_yaml::{from_str, Result, Value};

#[derive(Deserialize)]
#[serde(untagged)]
enum Yaml {
	Sequence(Vec<Value>),
	Map(IndexMap<Value, Value>),
}

/// Parse YAML string into a Lua table
///
/// ```rust
/// use yaml2lua::parse;
///
/// let yaml = r#"
/// string: abc
/// int: 123
/// bool: true
///
/// array:
///   - xyz
///   - 456
/// "#;
///
/// let lua = r#"{
/// 	["string"] = "abc",
/// 	["int"] = 123,
/// 	["bool"] = true,
/// 	["array"] = {
/// 		"xyz",
/// 		456,
/// 	},
/// }"#;
///
/// assert_eq!(parse(yaml).unwrap(), lua);
/// ```
pub fn parse(yaml: &str) -> Result<String> {
	let mut lua = String::from("{\n");

	match from_str(yaml)? {
		Yaml::Sequence(yaml) => {
			for value in yaml {
				lua.push_str(&walk(None, &value, 1));
			}
		}
		Yaml::Map(yaml) => {
			for (key, value) in yaml {
				lua.push_str(&walk(Some(&key), &value, 1));
			}
		}
	}

	lua.push('}');

	Ok(lua)
}

fn walk(key: Option<&Value>, value: &Value, depth: usize) -> String {
	let mut lua = String::new();

	lua.push_str(&get_indent(depth));

	if let Some(key) = key {
		match key {
			Value::String(s) => {
				lua.push_str(&format!("[\"{}\"] = ", escape_string(s)));
			}
			Value::Number(n) => {
				lua.push_str(&format!("[{}] = ", n));
			}
			Value::Bool(b) => {
				lua.push_str(&format!("[{}] = ", b));
			}
			_ => return String::new(),
		};
	}

	match value {
		Value::String(s) => lua.push_str(&format!("\"{}\"", &escape_string(s))),
		Value::Number(n) => lua.push_str(&n.to_string()),
		Value::Bool(b) => lua.push_str(&b.to_string()),
		Value::Null => lua.push_str("nil"),
		Value::Sequence(s) => {
			lua.push_str("{\n");

			for v in s {
				lua.push_str(&walk(None, v, depth + 1));
			}

			lua.push_str(&get_indent(depth));
			lua.push('}');
		}
		Value::Mapping(m) => {
			lua.push_str("{\n");

			for (k, v) in m {
				lua.push_str(&walk(Some(k), v, depth + 1));
			}

			lua.push_str(&get_indent(depth));
			lua.push('}');
		}
		Value::Tagged(t) => {
			lua.push_str("{\n");

			lua.push_str(&get_indent(depth + 1));
			lua.push_str(&format!(
				"[\"{}\"] = {}",
				t.tag.to_string().strip_prefix('!').unwrap(),
				&walk(None, &t.value, depth + 1)
					.strip_prefix(&"\t".repeat(depth + 1))
					.unwrap()
			));

			lua.push_str(&get_indent(depth));
			lua.push('}');
		}
	}

	lua.push_str(",\n");

	lua
}

fn get_indent(depth: usize) -> String {
	let mut indent = String::new();

	for _ in 0..depth {
		indent.push('\t');
	}

	indent
}

fn escape_string(string: &str) -> String {
	let mut chars = string.chars();

	while let Some(char) = chars.next() {
		if char == '\\' {
			match chars.next() {
				Some('n' | 't' | 'r' | '\\' | '"') => {}
				_ => return string.escape_default().to_string(),
			}
		} else {
			match char {
				'\n' | '\t' | '\r' | '"' => return string.escape_default().to_string(),
				_ => {}
			}
		}
	}

	string.to_owned()
}

#[cfg(test)]
mod test {
	#[test]
	fn all_values() {
		use crate::parse;

		let yaml = r#"
string: str
int: 420
float: 4.2
bool: true
nil: null
array:
  - string
  - 12345
  - false
  - k: v
object:
  key: value"#;

		let lua = r#"{
	["string"] = "str",
	["int"] = 420,
	["float"] = 4.2,
	["bool"] = true,
	["nil"] = nil,
	["array"] = {
		"string",
		12345,
		false,
		{
			["k"] = "v",
		},
	},
	["object"] = {
		["key"] = "value",
	},
}"#;

		assert_eq!(parse(yaml).unwrap(), lua);
	}

	// 	#[test]
	// 	fn tagged_value() {
	// 		use crate::parse;

	// 		let yaml = r#"test: !SomeTag { x: 5 }"#;

	// 		let lua = r#"{
	// 	["test"] = {
	// 		["SomeTag"] = {
	// 			["x"] = 5,
	// 		},
	// 	},
	// }"#;

	// 		assert_eq!(parse(yaml).unwrap(), lua);
	// 	}

	#[test]
	fn malformed_strings() {
		use crate::parse;

		let yaml = r#"
1: ..\n..
2: ..\t..
3: ..\r..
4: ..\\..
5: ..\"..
6: "..\n.."
7: "..\t.."
8: "..\r.."
9: "..\\.."
10: "..\"..""#;

		let lua = r#"{
	[1] = "..\n..",
	[2] = "..\t..",
	[3] = "..\r..",
	[4] = "..\\..",
	[5] = "..\"..",
	[6] = "..\n..",
	[7] = "..\t..",
	[8] = "..\r..",
	[9] = "..\\..",
	[10] = "..\"..",
}"#;

		assert_eq!(parse(yaml).unwrap(), lua);
	}

	#[test]
	fn root_array() {
		use crate::parse;

		let yaml = r#"
- a
- b
- c"#;

		let lua = r#"{
	"a",
	"b",
	"c",
}"#;

		assert_eq!(parse(yaml).unwrap(), lua);
	}
}
