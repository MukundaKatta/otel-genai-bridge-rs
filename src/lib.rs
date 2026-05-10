//! Translate LLM telemetry attributes between OpenInference and the
//! OpenTelemetry GenAI semantic conventions.
//!
//! Two specs in flight:
//!
//! - **OpenInference** — Arize's instrumentation lib. Names like
//!   `llm.model_name`, `llm.token_count.prompt`.
//! - **OpenTelemetry GenAI semantic conventions** — the upcoming standard.
//!   Names like `gen_ai.request.model`, `gen_ai.usage.input_tokens`.
//!
//! Phoenix wants OTel GenAI ([Arize-ai/phoenix#10622](https://github.com/Arize-ai/phoenix/issues/10622)),
//! Traceloop wants the new keys ([traceloop/openllmetry#3515](https://github.com/traceloop/openllmetry/issues/3515)),
//! but most existing instrumentation emits OpenInference. Until backends
//! converge, you ingest one and may want to read or emit the other.
//! `otel-genai-bridge` is the smallest possible primitive that translates.
//!
//! # Quick start
//!
//! ```
//! use otel_genai_bridge::{to_otel_genai, to_openinference};
//! use serde_json::{json, Value};
//! use std::collections::HashMap;
//!
//! let mut openinference: HashMap<String, Value> = HashMap::new();
//! openinference.insert("llm.model_name".into(), json!("claude-sonnet-4"));
//! openinference.insert("llm.token_count.prompt".into(), json!(1234));
//! openinference.insert("llm.token_count.completion".into(), json!(56));
//!
//! let otel = to_otel_genai(&openinference);
//! assert_eq!(otel.get("gen_ai.request.model").unwrap(), &json!("claude-sonnet-4"));
//! assert_eq!(otel.get("gen_ai.usage.input_tokens").unwrap(), &json!(1234));
//! assert_eq!(otel.get("gen_ai.usage.output_tokens").unwrap(), &json!(56));
//!
//! // Round-trip back to OpenInference:
//! let back = to_openinference(&otel);
//! assert_eq!(back.get("llm.model_name").unwrap(), &json!("claude-sonnet-4"));
//! ```
#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

mod bridge;

pub use crate::bridge::{
    known_mappings, to_openinference, to_otel_genai, OPENINFERENCE_TO_OTEL, OTEL_TO_OPENINFERENCE,
};
