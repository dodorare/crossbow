use serde::Deserialize;
use serde_json::Value;
use std::{borrow::Cow, collections::BTreeMap, fmt};

const PLACEHOLDER_PREFIX: &str = "{{crossbow.";

/// Allow-listed build-environment values used in platform configuration.
#[derive(Clone, Default, PartialEq, Eq)]
pub struct BuildVariables(BTreeMap<String, Value>);

// Resolved values are public application configuration, but still must not leak into diagnostics.
impl fmt::Debug for BuildVariables {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("BuildVariables")
            .field("names", &self.0.keys())
            .finish()
    }
}

impl BuildVariables {
    pub(crate) fn get(&self, name: &str) -> Option<&Value> {
        self.0.get(name)
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Clone, Copy, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
enum BuildVariableType {
    #[default]
    String,
    Integer,
    Boolean,
}

impl BuildVariableType {
    fn name(self) -> &'static str {
        match self {
            Self::String => "string",
            Self::Integer => "integer",
            Self::Boolean => "boolean",
        }
    }

    fn accepts(self, value: &Value) -> bool {
        match self {
            Self::String => value.is_string(),
            Self::Integer => value.as_i64().is_some(),
            Self::Boolean => value.is_boolean(),
        }
    }

    fn parse(self, name: &str, value: String) -> anyhow::Result<Value> {
        match self {
            Self::String => Ok(Value::String(value)),
            Self::Integer => value
                .parse::<i64>()
                .map(Value::from)
                .map_err(|_| anyhow::anyhow!("build variable `{name}` must be an integer")),
            Self::Boolean => value
                .parse::<bool>()
                .map(Value::from)
                .map_err(|_| anyhow::anyhow!("build variable `{name}` must be `true` or `false`")),
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct BuildVariableDefinition {
    env: String,
    #[serde(rename = "type", default)]
    value_type: BuildVariableType,
    default: Option<Value>,
}

pub(crate) type BuildVariableDefinitions = BTreeMap<String, BuildVariableDefinition>;

pub(crate) fn resolve_process_environment(
    definitions: &BuildVariableDefinitions,
) -> anyhow::Result<BuildVariables> {
    resolve_definitions(definitions, |name| match std::env::var(name) {
        Ok(value) => Ok(Some(value)),
        Err(std::env::VarError::NotPresent) => Ok(None),
        Err(std::env::VarError::NotUnicode(_)) => {
            anyhow::bail!("environment variable `{name}` is not valid Unicode")
        }
    })
}

pub(crate) fn interpolate_metadata(
    metadata: &mut Value,
    variables: &BuildVariables,
) -> anyhow::Result<()> {
    // Limit expansion to the public platform documents named by the feature. In particular,
    // values must never flow into paths, plugin configuration, or other generated files.
    for pointer in ["/android/manifest", "/apple/info_plist"] {
        if let Some(value) = metadata.pointer_mut(pointer) {
            interpolate_json(value, variables)?;
        }
    }
    Ok(())
}

pub(crate) fn take_definitions(metadata: &mut Value) -> anyhow::Result<BuildVariableDefinitions> {
    let Some(raw) = metadata
        .as_object_mut()
        .and_then(|metadata| metadata.remove("build_variables"))
    else {
        return Ok(BuildVariableDefinitions::new());
    };
    let definitions: BuildVariableDefinitions = serde_json::from_value(raw)
        .map_err(|error| anyhow::anyhow!("invalid `package.metadata.build_variables`: {error}"))?;
    validate_definitions(&definitions)?;
    Ok(definitions)
}

fn validate_definitions(definitions: &BuildVariableDefinitions) -> anyhow::Result<()> {
    for (name, definition) in definitions {
        validate_name(name)?;
        if definition.env.is_empty() {
            anyhow::bail!("build variable `{name}` has an empty environment variable name");
        }
        if definition
            .default
            .as_ref()
            .is_some_and(|value| !definition.value_type.accepts(value))
        {
            anyhow::bail!(
                "default for build variable `{name}` does not match its declared {} type",
                definition.value_type.name()
            );
        }
        if let Some(default) = &definition.default {
            reject_nested_placeholder(name, default)?;
        }
    }
    Ok(())
}

pub(crate) fn resolve_definitions(
    definitions: &BuildVariableDefinitions,
    mut environment: impl FnMut(&str) -> anyhow::Result<Option<String>>,
) -> anyhow::Result<BuildVariables> {
    let mut values = BTreeMap::new();
    for (name, definition) in definitions {
        let value = if let Some(value) = environment(&definition.env)? {
            definition.value_type.parse(name, value)?
        } else if let Some(value) = &definition.default {
            value.clone()
        } else {
            anyhow::bail!(
                "build variable `{name}` requires environment variable `{}` or a default",
                definition.env
            );
        };
        reject_nested_placeholder(name, &value)?;
        values.insert(name.clone(), value);
    }
    Ok(BuildVariables(values))
}

fn reject_nested_placeholder(name: &str, value: &Value) -> anyhow::Result<()> {
    if value
        .as_str()
        .is_some_and(|value| value.contains(PLACEHOLDER_PREFIX))
    {
        anyhow::bail!("build variable `{name}` must not contain another build placeholder");
    }
    Ok(())
}

fn validate_name(name: &str) -> anyhow::Result<()> {
    let mut chars = name.chars();
    if !chars
        .next()
        .is_some_and(|character| character == '_' || character.is_ascii_alphabetic())
        || !chars.all(|character| character == '_' || character.is_ascii_alphanumeric())
    {
        anyhow::bail!(
            "invalid build variable name `{name}`; use ASCII letters, digits, and underscores, starting with a letter or underscore"
        );
    }
    Ok(())
}

fn interpolate_json(value: &mut Value, variables: &BuildVariables) -> anyhow::Result<()> {
    match value {
        Value::Array(values) => {
            for value in values {
                interpolate_json(value, variables)?;
            }
        }
        Value::Object(values) => {
            for value in values.values_mut() {
                interpolate_json(value, variables)?;
            }
        }
        Value::String(template) => match exact_variable(template, variables)? {
            Some(resolved) => *value = resolved.clone(),
            None => *template = interpolate_string(template, variables)?,
        },
        _ => {}
    }
    Ok(())
}

pub(crate) fn exact_variable<'a>(
    template: &str,
    variables: &'a BuildVariables,
) -> anyhow::Result<Option<&'a Value>> {
    let Some(name) = template
        .strip_prefix(PLACEHOLDER_PREFIX)
        .and_then(|value| value.strip_suffix("}}"))
        .filter(|name| !name.contains("}}"))
    else {
        return Ok(None);
    };
    validate_name(name)?;
    variable(variables, name).map(Some)
}

pub(crate) fn interpolate_string(
    template: &str,
    variables: &BuildVariables,
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
        validate_name(name)?;
        output.push_str(&display(variable(variables, name)?));
        remaining = &placeholder[end + 2..];
    }
    output.push_str(remaining);
    Ok(output)
}

fn variable<'a>(variables: &'a BuildVariables, name: &str) -> anyhow::Result<&'a Value> {
    variables.get(name).ok_or_else(|| {
        anyhow::anyhow!(
            "build variable `{name}` is used but not declared in `package.metadata.build_variables`"
        )
    })
}

fn display(value: &Value) -> Cow<'_, str> {
    match value {
        Value::String(value) => Cow::Borrowed(value),
        Value::Number(value) => Cow::Owned(value.to_string()),
        Value::Bool(value) => Cow::Owned(value.to_string()),
        _ => unreachable!("build variables are validated scalar values"),
    }
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
    fn environment_wins_while_defaults_keep_their_types() {
        let values = resolve_definitions(&definitions(), |name| {
            Ok((name == "API_HOST").then(|| "例.example".to_owned()))
        })
        .unwrap();
        assert_eq!(values.get("HOST"), Some(&serde_json::json!("例.example")));
        assert_eq!(values.get("BUILD"), Some(&serde_json::json!(7)));
        assert_eq!(values.get("ENABLED"), Some(&serde_json::json!(false)));
        assert!(!format!("{values:?}").contains("例.example"));
    }

    #[test]
    fn empty_strings_override_defaults() {
        let values = resolve_definitions(&definitions(), |name| {
            Ok((name == "API_HOST").then(String::new))
        })
        .unwrap();
        assert_eq!(values.get("HOST"), Some(&serde_json::json!("")));
    }

    #[test]
    fn interpolation_is_typed_scoped_and_platform_safe() {
        let mut metadata = serde_json::json!({
            "build_variables": {
                "HOST": { "env": "API_HOST", "default": "localhost" },
                "BUILD": { "env": "BUILD_NUMBER", "type": "integer", "default": 7 }
            },
            "app_name": "{{crossbow.HOST}}",
            "android": { "manifest": {
                "version_code": "{{crossbow.BUILD}}",
                "application": { "label": "{{crossbow.HOST}}/${applicationId}/$(PRODUCT_NAME)" }
            }}
        });
        let definitions = take_definitions(&mut metadata).unwrap();
        let variables = resolve_definitions(&definitions, |_| Ok(None)).unwrap();
        interpolate_metadata(&mut metadata, &variables).unwrap();
        assert_eq!(metadata["app_name"], "{{crossbow.HOST}}");
        assert_eq!(metadata["android"]["manifest"]["version_code"], 7);
        assert_eq!(
            metadata["android"]["manifest"]["application"]["label"],
            "localhost/${applicationId}/$(PRODUCT_NAME)"
        );
    }

    #[test]
    fn rejects_missing_malformed_undeclared_and_nested_values() {
        let mut missing = definitions();
        missing.get_mut("HOST").unwrap().default = None;
        assert!(
            resolve_definitions(&missing, |_| Ok(None))
                .unwrap_err()
                .to_string()
                .contains("requires environment variable")
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

        let nested: BuildVariableDefinitions = serde_json::from_value(serde_json::json!({
            "BAD": { "env": "BAD", "default": "{{crossbow.OTHER}}" }
        }))
        .unwrap();
        assert!(
            validate_definitions(&nested)
                .unwrap_err()
                .to_string()
                .contains("must not contain")
        );
    }

    #[test]
    fn rejects_invalid_names_and_types() {
        let invalid: BuildVariableDefinitions = serde_json::from_value(serde_json::json!({
            "not-valid": { "env": "VALUE", "default": "value" }
        }))
        .unwrap();
        assert!(validate_definitions(&invalid).is_err());

        let integer: BuildVariableDefinitions = serde_json::from_value(serde_json::json!({
            "BUILD": { "env": "BUILD_NUMBER", "type": "integer" }
        }))
        .unwrap();
        assert!(
            resolve_definitions(&integer, |_| Ok(Some(String::new())))
                .unwrap_err()
                .to_string()
                .contains("must be an integer")
        );

        let boolean: BuildVariableDefinitions = serde_json::from_value(serde_json::json!({
            "ENABLED": { "env": "FEATURE_ENABLED", "type": "boolean", "default": "true" }
        }))
        .unwrap();
        assert!(
            validate_definitions(&boolean)
                .unwrap_err()
                .to_string()
                .contains("declared boolean type")
        );
    }
}
