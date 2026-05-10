# otel-genai-bridge

[![crates.io](https://img.shields.io/crates/v/otel-genai-bridge.svg)](https://crates.io/crates/otel-genai-bridge)
[![docs.rs](https://docs.rs/otel-genai-bridge/badge.svg)](https://docs.rs/otel-genai-bridge)
[![License: MIT](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

Translate LLM telemetry attributes between **OpenInference** and **OpenTelemetry GenAI** semantic conventions. Pure data, no telemetry SDK dependency.

```toml
[dependencies]
otel-genai-bridge = "0.1"
```

## Why

LLM telemetry has two competing attribute namings in flight:

- **OpenInference** — Arize's spec. Used by Phoenix, OpenLLMetry's older convention. `llm.model_name`, `llm.token_count.prompt`.
- **OpenTelemetry GenAI semconv** — the upcoming standard. `gen_ai.request.model`, `gen_ai.usage.input_tokens`.

Backends are converging slowly. Phoenix has [an open issue (#10622)](https://github.com/Arize-ai/phoenix/issues/10622) for OTel GenAI; OpenLLMetry has [deprecated the old keys (#3515)](https://github.com/traceloop/openllmetry/issues/3515). In the meantime you may ingest one shape and need to query, store, or emit the other.

`otel-genai-bridge` is the smallest possible primitive: a const lookup table plus a `HashMap` rename. No span SDK dep, no allocator behavior beyond cloning the values you supply.

## Quick start

```rust
use otel_genai_bridge::{to_otel_genai, to_openinference};
use serde_json::{json, Value};
use std::collections::HashMap;

let mut oi: HashMap<String, Value> = HashMap::new();
oi.insert("llm.model_name".into(), json!("claude-sonnet-4"));
oi.insert("llm.token_count.prompt".into(), json!(1234));
oi.insert("llm.token_count.completion".into(), json!(56));

let otel = to_otel_genai(&oi);
assert_eq!(otel["gen_ai.request.model"], json!("claude-sonnet-4"));
assert_eq!(otel["gen_ai.usage.input_tokens"], json!(1234));
```

Unknown keys pass through unchanged — safe to translate any attribute bag.

## Coverage (v0.1)

Model + provider, token counts (prompt/completion/total/cache-read/cache-creation), invocation parameters (temperature, top_p, top_k, max_tokens, stop_sequences, frequency_penalty, presence_penalty, seed), and response fields (id, model, finish_reason). Message arrays and content payloads are left as-is — translating them requires structural rewrites the v0.1 doesn't attempt.

## Custom mappings

If your span emitter uses a non-canonical key, override the table:

```rust
use otel_genai_bridge::to_otel_genai_with;

let custom = &[
    ("acme.llm.input_tokens", "gen_ai.usage.input_tokens"),
    // ...your overrides...
];
let otel = otel_genai_bridge::to_otel_genai_with(&attrs, custom);
```

## What it doesn't do

- Doesn't ingest spans from an OTel SDK; it operates on already-extracted attribute maps.
- Doesn't translate message-array payloads (OI `llm.input_messages.0.message.role` ↔ OTel events). v0.2.
- Doesn't validate values against either spec. Garbage in, garbage out.

## License

MIT
