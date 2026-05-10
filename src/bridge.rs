use serde_json::Value;
use std::collections::HashMap;

/// OpenInference → OTel GenAI attribute name pairs.
///
/// Source: openinference-semantic-conventions v0.x and OTel GenAI semconv
/// drafts as of late 2025/early 2026. Values may shift as both specs
/// evolve; users can override via [`to_otel_genai_with`].
pub const OPENINFERENCE_TO_OTEL: &[(&str, &str)] = &[
    ("llm.model_name", "gen_ai.request.model"),
    ("llm.provider", "gen_ai.system"),
    ("llm.system", "gen_ai.system"),
    ("llm.token_count.prompt", "gen_ai.usage.input_tokens"),
    ("llm.token_count.completion", "gen_ai.usage.output_tokens"),
    ("llm.token_count.total", "gen_ai.usage.total_tokens"),
    ("llm.token_count.prompt_details.cache_read", "gen_ai.usage.cache_read_input_tokens"),
    ("llm.token_count.prompt_details.cache_write", "gen_ai.usage.cache_creation_input_tokens"),
    ("llm.invocation_parameters.temperature", "gen_ai.request.temperature"),
    ("llm.invocation_parameters.top_p", "gen_ai.request.top_p"),
    ("llm.invocation_parameters.top_k", "gen_ai.request.top_k"),
    ("llm.invocation_parameters.max_tokens", "gen_ai.request.max_tokens"),
    ("llm.invocation_parameters.stop_sequences", "gen_ai.request.stop_sequences"),
    ("llm.invocation_parameters.frequency_penalty", "gen_ai.request.frequency_penalty"),
    ("llm.invocation_parameters.presence_penalty", "gen_ai.request.presence_penalty"),
    ("llm.invocation_parameters.seed", "gen_ai.request.seed"),
    ("llm.response.id", "gen_ai.response.id"),
    ("llm.response.model", "gen_ai.response.model"),
    ("llm.response.finish_reason", "gen_ai.response.finish_reasons"),
];

/// OTel GenAI → OpenInference attribute name pairs (the inverse).
pub const OTEL_TO_OPENINFERENCE: &[(&str, &str)] = &[
    ("gen_ai.request.model", "llm.model_name"),
    ("gen_ai.system", "llm.provider"),
    ("gen_ai.usage.input_tokens", "llm.token_count.prompt"),
    ("gen_ai.usage.output_tokens", "llm.token_count.completion"),
    ("gen_ai.usage.total_tokens", "llm.token_count.total"),
    ("gen_ai.usage.cache_read_input_tokens", "llm.token_count.prompt_details.cache_read"),
    ("gen_ai.usage.cache_creation_input_tokens", "llm.token_count.prompt_details.cache_write"),
    ("gen_ai.request.temperature", "llm.invocation_parameters.temperature"),
    ("gen_ai.request.top_p", "llm.invocation_parameters.top_p"),
    ("gen_ai.request.top_k", "llm.invocation_parameters.top_k"),
    ("gen_ai.request.max_tokens", "llm.invocation_parameters.max_tokens"),
    ("gen_ai.request.stop_sequences", "llm.invocation_parameters.stop_sequences"),
    ("gen_ai.request.frequency_penalty", "llm.invocation_parameters.frequency_penalty"),
    ("gen_ai.request.presence_penalty", "llm.invocation_parameters.presence_penalty"),
    ("gen_ai.request.seed", "llm.invocation_parameters.seed"),
    ("gen_ai.response.id", "llm.response.id"),
    ("gen_ai.response.model", "llm.response.model"),
    ("gen_ai.response.finish_reasons", "llm.response.finish_reason"),
];

/// Pretty alias for the union of mappings (deduped).
///
/// Returns a `Vec<(openinference_name, otel_name)>` for introspection.
pub fn known_mappings() -> Vec<(&'static str, &'static str)> {
    OPENINFERENCE_TO_OTEL.to_vec()
}

/// Translate a flat attribute map from OpenInference to OTel GenAI.
///
/// Unknown keys pass through unchanged. The returned map is keyed by the
/// OTel GenAI attribute name where one is known.
pub fn to_otel_genai(attrs: &HashMap<String, Value>) -> HashMap<String, Value> {
    to_otel_genai_with(attrs, OPENINFERENCE_TO_OTEL)
}

/// Same as [`to_otel_genai`], but with a caller-supplied mapping table —
/// useful when you've patched a particular convention pair.
pub fn to_otel_genai_with(
    attrs: &HashMap<String, Value>,
    mapping: &[(&str, &str)],
) -> HashMap<String, Value> {
    let lookup: HashMap<&str, &str> = mapping.iter().copied().collect();
    let mut out: HashMap<String, Value> = HashMap::with_capacity(attrs.len());
    for (k, v) in attrs {
        let new_key = lookup.get(k.as_str()).copied().unwrap_or(k.as_str()).to_string();
        out.insert(new_key, v.clone());
    }
    out
}

/// Translate a flat attribute map from OTel GenAI to OpenInference.
pub fn to_openinference(attrs: &HashMap<String, Value>) -> HashMap<String, Value> {
    let lookup: HashMap<&str, &str> = OTEL_TO_OPENINFERENCE.iter().copied().collect();
    let mut out: HashMap<String, Value> = HashMap::with_capacity(attrs.len());
    for (k, v) in attrs {
        let new_key = lookup.get(k.as_str()).copied().unwrap_or(k.as_str()).to_string();
        out.insert(new_key, v.clone());
    }
    out
}
