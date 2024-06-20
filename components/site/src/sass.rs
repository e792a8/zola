use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, RwLock};

use config::Config;
use libs::globset::Glob;
use libs::grass_compiler::codemap::Span;
use libs::grass_compiler::sass_value::{Brackets, ListSeparator, QuoteKind, SassMap};
use libs::grass_compiler::{
    from_path as compile_file,
    sass_value::{ArgumentResult, SassNumber, Value},
    Builtin, Options, OutputStyle, Result as SassResult, Visitor,
};
use libs::serde_json::{to_value, Value as JsonValue};
use libs::walkdir::{DirEntry, WalkDir};

use crate::anyhow;
use errors::{bail, Result};
use utils::fs::{create_directory, create_file};

static CONFIG_VALUE: LazyLock<RwLock<JsonValue>> = LazyLock::new(|| RwLock::new(JsonValue::Null));

fn map_value(j: &JsonValue, span: Span) -> Value {
    match j {
        JsonValue::String(x) => Value::String(x.to_owned(), QuoteKind::Quoted),
        JsonValue::Bool(x) => Value::bool(*x),
        JsonValue::Number(x) => Value::Dimension(SassNumber::new_unitless(x.as_f64().unwrap())),
        JsonValue::Array(x) => Value::List(
            x.iter().map(|v| map_value(v, span)).collect(),
            ListSeparator::Comma,
            Brackets::Bracketed,
        ),
        JsonValue::Object(m) => Value::Map(SassMap::new_with(
            m.iter()
                .map(|v| {
                    (
                        Value::String(v.0.to_owned(), QuoteKind::Quoted).span(span),
                        map_value(v.1, span),
                    )
                })
                .collect(),
        )),
        _ => Value::Null,
    }
}

fn sass_get_config(args: ArgumentResult, _visitor: &mut Visitor) -> SassResult<Value> {
    let span = args.span();

    Ok(map_value(&CONFIG_VALUE.read().unwrap(), span))
}

pub fn compile_sass(base_path: &Path, output_path: &Path, config: &Config) -> Result<()> {
    create_directory(output_path)?;

    let sass_path = {
        let mut sass_path = PathBuf::from(base_path);
        sass_path.push("sass");
        sass_path
    };

    *CONFIG_VALUE.write().unwrap() = to_value(config).unwrap();

    let options = Options::default()
        .style(OutputStyle::Compressed)
        .add_custom_fn("zola-config", Builtin::new(sass_get_config));

    let files = get_non_partial_scss(&sass_path);
    let mut compiled_paths = Vec::new();

    for file in files {
        let css = compile_file(&file, &options).map_err(|e| anyhow!(e))?;

        let path_inside_sass = file.strip_prefix(&sass_path).unwrap();
        let parent_inside_sass = path_inside_sass.parent();
        let css_output_path = output_path.join(path_inside_sass).with_extension("css");

        if parent_inside_sass.is_some() {
            create_dir_all(css_output_path.parent().unwrap())?;
        }

        create_file(&css_output_path, &css)?;
        compiled_paths.push((path_inside_sass.to_owned(), css_output_path));
    }

    compiled_paths.sort();
    for window in compiled_paths.windows(2) {
        if window[0].1 == window[1].1 {
            bail!(
                "SASS path conflict: \"{}\" and \"{}\" both compile to \"{}\"",
                window[0].0.display(),
                window[1].0.display(),
                window[0].1.display(),
            );
        }
    }

    Ok(())
}

fn is_partial_scss(entry: &DirEntry) -> bool {
    entry.file_name().to_str().map(|s| s.starts_with('_')).unwrap_or(false)
}

fn get_non_partial_scss(sass_path: &Path) -> Vec<PathBuf> {
    let glob = Glob::new("*.{sass,scss}").expect("Invalid glob for sass").compile_matcher();

    WalkDir::new(sass_path)
        .into_iter()
        .filter_entry(|e| !is_partial_scss(e))
        .filter_map(|e| e.ok())
        .map(|e| e.into_path())
        .filter(|e| glob.is_match(e))
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_non_partial_scss() {
        use std::env;

        let mut path =
            env::current_dir().unwrap().parent().unwrap().parent().unwrap().to_path_buf();
        path.push("test_site");
        path.push("sass");

        let result = get_non_partial_scss(&path);

        assert!(!result.is_empty());
        assert!(result.iter().filter_map(|path| path.file_name()).any(|file| file == "scss.scss"))
    }

    #[test]
    fn test_get_non_partial_scss_underscores() {
        use std::env;

        let mut path =
            env::current_dir().unwrap().parent().unwrap().parent().unwrap().to_path_buf();
        path.push("test_site");
        path.push("_dir_with_underscores");
        path.push("..");
        path.push("sass");

        let result = get_non_partial_scss(&path);

        assert!(!result.is_empty());
        assert!(result.iter().filter_map(|path| path.file_name()).any(|file| file == "scss.scss"))
    }
}
