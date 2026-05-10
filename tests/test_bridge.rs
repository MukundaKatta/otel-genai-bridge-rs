use otel_genai_bridge::{
    known_mappings, to_openinference, to_otel_genai, OPENINFERENCE_TO_OTEL, OTEL_TO_OPENINFERENCE,
};
use serde_json::{json, Value};
use std::collections::HashMap;

fn map(items: &[(&str, Value)]) -> HashMap<String, Value> {
    items.iter().map(|(k, v)| (k.to_string(), v.clone())).collect()
}

#[test]
fn translates_model_name() {
    let oi = map(&[("llm.model_name", json!("claude-sonnet-4"))]);
    let otel = to_otel_genai(&oi);
    assert_eq!(otel.get("gen_ai.request.model").unwrap(), &json!("claude-sonnet-4"));
    assert!(!otel.contains_key("llm.model_name"));
}

#[test]
fn translates_token_counts() {
    let oi = map(&[
        ("llm.token_count.prompt", json!(1234)),
        ("llm.token_count.completion", json!(56)),
        ("llm.token_count.total", json!(1290)),
    ]);
    let otel = to_otel_genai(&oi);
    assert_eq!(otel["gen_ai.usage.input_tokens"], json!(1234));
    assert_eq!(otel["gen_ai.usage.output_tokens"], json!(56));
    assert_eq!(otel["gen_ai.usage.total_tokens"], json!(1290));
}

#[test]
fn translates_cache_tokens() {
    let oi = map(&[
        ("llm.token_count.prompt_details.cache_read", json!(800)),
        ("llm.token_count.prompt_details.cache_write", json!(200)),
    ]);
    let otel = to_otel_genai(&oi);
    assert_eq!(otel["gen_ai.usage.cache_read_input_tokens"], json!(800));
    assert_eq!(otel["gen_ai.usage.cache_creation_input_tokens"], json!(200));
}

#[test]
fn translates_invocation_parameters() {
    let oi = map(&[
        ("llm.invocation_parameters.temperature", json!(0.7)),
        ("llm.invocation_parameters.top_p", json!(0.9)),
        ("llm.invocation_parameters.max_tokens", json!(4096)),
    ]);
    let otel = to_otel_genai(&oi);
    assert_eq!(otel["gen_ai.request.temperature"], json!(0.7));
    assert_eq!(otel["gen_ai.request.top_p"], json!(0.9));
    assert_eq!(otel["gen_ai.request.max_tokens"], json!(4096));
}

#[test]
fn unknown_keys_pass_through() {
    let oi = map(&[("custom.span.kind", json!("LLM"))]);
    let otel = to_otel_genai(&oi);
    assert_eq!(otel["custom.span.kind"], json!("LLM"));
}

#[test]
fn round_trip_otel_then_back() {
    let oi = map(&[
        ("llm.model_name", json!("claude")),
        ("llm.token_count.prompt", json!(100)),
        ("llm.token_count.completion", json!(50)),
        ("llm.invocation_parameters.temperature", json!(0.5)),
    ]);
    let otel = to_otel_genai(&oi);
    let back = to_openinference(&otel);
    assert_eq!(back, oi);
}

#[test]
fn translates_response_fields() {
    let oi = map(&[
        ("llm.response.id", json!("msg_abc")),
        ("llm.response.model", json!("claude-sonnet-4-20250514")),
        ("llm.response.finish_reason", json!("end_turn")),
    ]);
    let otel = to_otel_genai(&oi);
    assert_eq!(otel["gen_ai.response.id"], json!("msg_abc"));
    assert_eq!(otel["gen_ai.response.model"], json!("claude-sonnet-4-20250514"));
    assert_eq!(otel["gen_ai.response.finish_reasons"], json!("end_turn"));
}

#[test]
fn known_mappings_returns_pairs() {
    let pairs = known_mappings();
    assert!(!pairs.is_empty());
    assert!(pairs.iter().any(|(oi, _)| *oi == "llm.model_name"));
    assert!(pairs.iter().any(|(_, otel)| *otel == "gen_ai.request.model"));
}

#[test]
fn const_tables_match_in_both_directions_for_core_attrs() {
    // Check that for every entry in OPENINFERENCE_TO_OTEL whose key has a
    // canonical inverse, the inverse table contains it. (Some OI keys map
    // to the same OTel attr — `llm.system` and `llm.provider` both map to
    // `gen_ai.system` — so we don't enforce strict bijection.)
    let inv: HashMap<&str, &str> = OTEL_TO_OPENINFERENCE.iter().copied().collect();
    for (oi, otel) in OPENINFERENCE_TO_OTEL {
        if let Some(back) = inv.get(otel) {
            // Must point back to *some* OI key that maps forward to this OTel attr.
            let forwards_again = OPENINFERENCE_TO_OTEL
                .iter()
                .find(|(k, _)| *k == *back)
                .map(|(_, v)| *v == *otel)
                .unwrap_or(false);
            assert!(
                forwards_again,
                "round-trip broken: oi={oi} -> otel={otel} -> oi={back} (didn't forward)",
            );
        }
    }
}

#[test]
fn empty_input_yields_empty_output() {
    let empty: HashMap<String, Value> = HashMap::new();
    assert!(to_otel_genai(&empty).is_empty());
    assert!(to_openinference(&empty).is_empty());
}
