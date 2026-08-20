use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

const PLACEHOLDER_PREFIX: &str = "{{crossbow.";

/// Values explicitly imported from the build environment.
///
/// Only variables declared in `package.metadata.build_variables` are present. Values are
/// configuration embedded in the application bundle and must not be treated as secrets.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BuildVariables(BTreeMap<String, BuildVariableValue>);

impl BuildVariables {
    pub fn get(&self, name: &str) -> Option<&BuildVariableValue> {
        self.0.get(name)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &BuildVariableValue)> {
        self.0.iter().map(|(name, value)| (name.as_str(), value))
    }
}

/// A resolved build variable, retaining its declared type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum BuildVariableValue {
    String(String),
    Integer(i64),
    Boolean(bool),
}

impl BuildVariableValue {
    pub fn as_string(&self) -> String {
        match self {
            Self::String(value) => value.clone(),
            Self::Integer(value) => value.to_string(),
            Self::Boolean(value) => value.to_string(),
        }
    }

    fn as_json(&self) -> serde_json::Value {
        match self {
            Self::String(value) => serde_json::Value::String(value.clone()),
            Self::Integer(value) => serde_json::Value::Number((*value).into()),
            Self::Boolean(value) => serde_json::Value::Bool(*value),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
enum BuildVariableType {
    #[default]
    String,
    Integer,
    Boolean,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct BuildVariableDefinition {
    env: String,
    #[serde(rename = "type", default)]
    value_type: BuildVariableType,
    default: Option<serde_json::Value>,
}

type BuildVariableDefinitions = BTreeMap<String, BuildVariableDefinition>;

/// Resolves declared variables and expands placeholders throughout package metadata before it is
/// deserialized into the platform-specific models.
pub(crate) fn resolve_metadata_build_variables(
    metadata: &mut serde_json::Value,
) -> anyhow::Result<BuildVariables> {
    let definitions = take_definitions(metadata)?;
    let variables = resolve_definitions(&definitions, |name| match std::env::var(name) {
        Ok(value) => Ok(Some(value)),
        Err(std::env::VarError::NotPresent) => Ok(None),
        Err(std::env::VarError::NotUnicode(_)) => {
            anyhow::bail!("environment variable `{name}` is not valid Unicode")
        }
    })?;
    interpolate_json_build_variables(metadata, &variables)?;
    Ok(variables)
}

fn take_definitions(metadata: &mut serde_json::Value) -> anyhow::Result<BuildVariableDefinitions> {
    let Some(table) = metadata.as_object_mut() else {
        return Ok(BuildVariableDefinitions::new());
    };
    let Some(raw) = table.remove("build_variables") else {
        return Ok(BuildVariableDefinitions::new());
    };
    serde_json::from_value(raw)
        .map_err(|error| anyhow::anyhow!("invalid `package.metadata.build_variables`: {error}"))
}

fn resolve_definitions<F>(
    definitions: &BuildVariableDefinitions,
    mut environment: F,
) -> anyhow::Result<BuildVariables>
where
    F: FnMut(&str) -> anyhow::Result<Option<String>>,
{
    let mut values = BTreeMap::new();
    for (name, definition) in definitions {
        validate_variable_name(name)?;
        if definition.env.is_empty() {
            anyhow::bail!("build variable `{name}` has an empty environment variable name");
        }
        let value = match environment(&definition.env)? {
            Some(value) => parse_environment_value(name, value, definition.value_type)?,
            None => match &definition.default {
                Some(default) => parse_default_value(name, default, definition.value_type)?,
                None => anyhow::bail!(
                    "build variable `{name}` requires environment variable `{}` or a default",
                    definition.env
                ),
            },
        };
        values.insert(name.clone(), value);
    }
    Ok(BuildVariables(values))
}

fn validate_variable_name(name: &str) -> anyhow::Result<()> {
    let mut chars = name.chars();
    let valid = chars
        .next()
        .is_some_and(|character| character == '_' || character.is_ascii_alphabetic())
        && chars.all(|character| character == '_' || character.is_ascii_alphanumeric());
    if !valid {
        anyhow::bail!(
            "invalid build variable name `{name}`; use ASCII letters, digits, and underscores, starting with a letter or underscore"
        );
    }
    Ok(())
}

fn parse_environment_value(
    name: &str,
    value: String,
    value_type: BuildVariableType,
) -> anyhow::Result<BuildVariableValue> {
    match value_type {
        BuildVariableType::String => Ok(BuildVariableValue::String(value)),
        BuildVariableType::Integer => value
            .parse()
            .map(BuildVariableValue::Integer)
            .map_err(|_| anyhow::anyhow!("build variable `{name}` must be an integer")),
        BuildVariableType::Boolean => value
            .parse()
            .map(BuildVariableValue::Boolean)
            .map_err(|_| anyhow::anyhow!("build variable `{name}` must be `true` or `false`")),
    }
}

fn parse_default_value(
    name: &str,
    value: &serde_json::Value,
    value_type: BuildVariableType,
) -> anyhow::Result<BuildVariableValue> {
    let resolved = match value_type {
        BuildVariableType::String => value
            .as_str()
            .map(|value| BuildVariableValue::String(value.to_owned())),
        BuildVariableType::Integer => value.as_i64().map(BuildVariableValue::Integer),
        BuildVariableType::Boolean => value.as_bool().map(BuildVariableValue::Boolean),
    };
    resolved.ok_or_else(|| {
        anyhow::anyhow!(
            "default for build variable `{name}` does not match its declared {} type",
            match value_type {
                BuildVariableType::String => "string",
                BuildVariableType::Integer => "integer",
                BuildVariableType::Boolean => "boolean",
            }
        )
    })
}

/// Recursively expands Crossbow placeholders in JSON-like typed configuration data. An exact
/// placeholder retains its declared integer or Boolean type.
pub fn interpolate_json_build_variables(
    value: &mut serde_json::Value,
    variables: &BuildVariables,
) -> anyhow::Result<()> {
    match value {
        serde_json::Value::Array(values) => {
            for value in values {
                interpolate_json_build_variables(value, variables)?;
            }
        }
        serde_json::Value::Object(values) => {
            for value in values.values_mut() {
                interpolate_json_build_variables(value, variables)?;
            }
        }
        serde_json::Value::String(template) => {
            if let Some(resolved) = exact_build_variable(template, variables)? {
                *value = resolved.as_json();
            } else {
                *template = interpolate_string(template, variables)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn exact_placeholder(template: &str) -> Option<&str> {
    template
        .strip_prefix(PLACEHOLDER_PREFIX)?
        .strip_suffix("}}")
        .filter(|name| !name.contains("}}"))
}

/// Returns the typed value when the entire string is one Crossbow placeholder.
pub fn exact_build_variable<'a>(
    template: &str,
    variables: &'a BuildVariables,
) -> anyhow::Result<Option<&'a BuildVariableValue>> {
    let Some(name) = exact_placeholder(template) else {
        return Ok(None);
    };
    validate_variable_name(name)?;
    variable(variables, name).map(Some)
}

/// Expands all Crossbow placeholders in a string while leaving Android `${...}` and Xcode
/// `$(...)` build-setting syntax untouched.
pub fn interpolate_build_variables(
    template: &str,
    variables: &BuildVariables,
) -> anyhow::Result<String> {
    interpolate_string(template, variables)
}

/// Expands integer, Boolean, and XML-safe string variables before Android's typed XML parser.
/// Strings requiring XML entities remain as placeholders until after parsing, avoiding a
/// double-unescape limitation in the upstream manifest deserializer.
pub fn interpolate_typed_build_variables(
    template: &str,
    variables: &BuildVariables,
) -> anyhow::Result<String> {
    interpolate_string_with(template, variables, |placeholder, value| match value {
        BuildVariableValue::String(value)
            if xml::escape::escape_str_attribute(value).as_ref() == value =>
        {
            value.clone()
        }
        BuildVariableValue::String(_) => placeholder.to_owned(),
        BuildVariableValue::Integer(_) | BuildVariableValue::Boolean(_) => value.as_string(),
    })
}

fn interpolate_string(template: &str, variables: &BuildVariables) -> anyhow::Result<String> {
    interpolate_string_with(template, variables, |_, value| value.as_string())
}

fn interpolate_string_with(
    template: &str,
    variables: &BuildVariables,
    mut replacement: impl FnMut(&str, &BuildVariableValue) -> String,
) -> anyhow::Result<String> {
    let mut output = String::with_capacity(template.len());
    let mut remaining = template;
    while let Some(start) = remaining.find(PLACEHOLDER_PREFIX) {
        output.push_str(&remaining[..start]);
        let placeholder = &remaining[start + PLACEHOLDER_PREFIX.len()..];
        let Some(end) = placeholder.find("}}") else {
            anyhow::bail!("unterminated Crossbow build variable placeholder");
        };
        let name = &placeholder[..end];
        validate_variable_name(name)?;
        let original = &remaining[start..start + PLACEHOLDER_PREFIX.len() + end + 2];
        output.push_str(&replacement(original, variable(variables, name)?));
        remaining = &placeholder[end + 2..];
    }
    output.push_str(remaining);
    Ok(output)
}

fn variable<'a>(
    variables: &'a BuildVariables,
    name: &str,
) -> anyhow::Result<&'a BuildVariableValue> {
    variables.get(name).ok_or_else(|| {
        anyhow::anyhow!(
            "build variable `{name}` is used but not declared in `package.metadata.build_variables`"
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn definitions() -> BuildVariableDefinitions {
        serde_json::from_value(serde_json::json!({
            "HOST": { "env": "API_HOST", "default": "localhost" },
            "BUILD": { "env": "BUILD_NUMBER", "type": "integer", "default": 7 },
            "ENABLED": { "env": "FEATURE_ENABLED", "type": "boolean", "default": false }
        }))
        .unwrap()
    }

    #[test]
    fn environment_wins_and_defaults_retain_types() {
        let values = resolve_definitions(&definitions(), |name| {
            Ok((name == "API_HOST").then(|| "api.example.com".to_owned()))
        })
        .unwrap();
        assert_eq!(
            values.get("HOST"),
            Some(&BuildVariableValue::String("api.example.com".into()))
        );
        assert_eq!(values.get("BUILD"), Some(&BuildVariableValue::Integer(7)));
        assert_eq!(
            values.get("ENABLED"),
            Some(&BuildVariableValue::Boolean(false))
        );
    }

    #[test]
    fn expands_embedded_unicode_and_preserves_platform_placeholders() {
        let values = resolve_definitions(&definitions(), |_| Ok(None)).unwrap();
        let result = interpolate_string(
            "https://{{crossbow.HOST}}/build/{{crossbow.BUILD}}/✓/${applicationId}/$(PRODUCT_NAME)",
            &values,
        )
        .unwrap();
        assert_eq!(
            result,
            "https://localhost/build/7/✓/${applicationId}/$(PRODUCT_NAME)"
        );
    }

    #[test]
    fn exact_json_placeholders_become_typed_values() {
        let values = resolve_definitions(&definitions(), |_| Ok(None)).unwrap();
        let mut metadata = serde_json::json!({
            "number": "{{crossbow.BUILD}}",
            "enabled": "{{crossbow.ENABLED}}",
            "label": "build-{{crossbow.BUILD}}"
        });
        interpolate_json_build_variables(&mut metadata, &values).unwrap();
        assert_eq!(metadata["number"], 7);
        assert_eq!(metadata["enabled"], false);
        assert_eq!(metadata["label"], "build-7");
    }

    #[test]
    fn rejects_missing_invalid_and_undeclared_variables() {
        let mut missing = definitions();
        missing.get_mut("HOST").unwrap().default = None;
        assert!(
            resolve_definitions(&missing, |_| Ok(None))
                .unwrap_err()
                .to_string()
                .contains("requires environment variable `API_HOST`")
        );
        assert!(
            interpolate_string("{{crossbow.SECRET}}", &BuildVariables::default())
                .unwrap_err()
                .to_string()
                .contains("not declared")
        );
        assert!(
            interpolate_string("{{crossbow.HOST", &BuildVariables::default())
                .unwrap_err()
                .to_string()
                .contains("unterminated")
        );
    }

    #[test]
    fn rejects_environment_and_default_type_mismatches() {
        let integer: BuildVariableDefinitions = serde_json::from_value(serde_json::json!({
            "BUILD": { "env": "BUILD_NUMBER", "type": "integer" }
        }))
        .unwrap();
        assert!(
            resolve_definitions(&integer, |_| Ok(Some("1.5".into())))
                .unwrap_err()
                .to_string()
                .contains("must be an integer")
        );

        let boolean: BuildVariableDefinitions = serde_json::from_value(serde_json::json!({
            "ENABLED": { "env": "FEATURE_ENABLED", "type": "boolean", "default": "true" }
        }))
        .unwrap();
        assert!(
            resolve_definitions(&boolean, |_| Ok(None))
                .unwrap_err()
                .to_string()
                .contains("does not match its declared boolean type")
        );
    }
}
