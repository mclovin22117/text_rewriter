# Provider Strategy: Hybrid Local + Cloud Model

## Overview

Text Rewriter will support TWO provider modes to give users maximum flexibility:

1. **Local LLM Mode** (default, no API key needed)
   - Zero cost, full privacy
   - User runs Ollama/LM Studio locally
   - All text transformation happens on user's machine

2. **Cloud API Mode** (optional, API key required)
   - Use user's preferred cloud provider (OpenAI, Anthropic, etc)
   - Access to powerful models
   - User billed by provider

## Design Principle

**User Gets to Choose, Not Us**

- We don't pick the provider
- We don't charge for the service
- We don't store keys on our servers
- User controls cost, privacy, and model selection

## Implementation Roadmap

### Phase 2a: Local Ollama Support (Week 1)

**Goal**: Make local LLM the default working path.

1. Add `ollama` provider adapter
   - New crate: `crates/providers/src/ollama.rs`
   - Implements HTTP calls to `http://localhost:11434/api/generate`
   - Minimal error handling (auto-detect if unreachable)

2. Update desktop CLI to try Ollama first
   - If local endpoint responds, use it
   - Else fall back to mock provider with helpful error

3. Test manually with local Ollama running

**Why Ollama first:**
- Lightest setup (single `docker run` or download)
- Good model selection
- Perfect for development and testing
- No API key needed

### Phase 2b: OpenAI-Compatible Adapter (Week 1-2)

**Goal**: Support any OpenAI-compatible endpoint.

1. Add `openai_compatible` provider adapter
   - Takes base URL and model name from config
   - Works with: OpenAI, Azure OpenAI, LocalAI, text-generation-webui, etc
   - Requires API key (stored in OS keychain)

2. Config loader in [crates/core/src/lib.rs](crates/core/src/lib.rs)
   - Load TOML config file from `~/.config/text_rewriter/config.toml`
   - Parse provider name, base URL, model, API key reference

3. Secure key retrieval
   - On request, fetch API key from OS vault
   - Never write key to logs or disk

### Phase 2c: Additional Cloud Providers (Weeks 3-4)

- Anthropic (Claude)
- Google Gemini
- Cohere
- Others as needed

All follow same pattern:
1. New provider adapter
2. Config support
3. Tests with mocked API responses

## Configuration File Format

### ~/.config/text_rewriter/config.toml

Local Ollama example:
```toml
[provider]
# Provider type: "ollama", "openai_compatible", "anthropic", etc
type = "ollama"

# LLM model name available on the local provider
model = "mistral"

# Base URL (Ollama runs on localhost:11434 by default)
base_url = "http://localhost:11434"

# API key not needed for local providers
# For cloud providers, the key is stored in OS keychain, not here
```

Cloud OpenAI example:
```toml
[provider]
type = "openai_compatible"
model = "gpt-4o-mini"
base_url = "https://api.openai.com/v1"
# api_key is stored in OS keychain with key: "text-rewriter.openai.api_key"
```

Anthropic example:
```toml
[provider]
type = "anthropic"
model = "claude-3-haiku"
# api_key stored in keychain: "text-rewriter.anthropic.api_key"
```

## Credential Storage

### macOS
- Use Keychain
- Key pattern: `text-rewriter.<provider_name>.api_key`
- Crate: `security-framework`

### Windows
- Use Credential Manager
- Key pattern: `text-rewriter.<provider_name>.api_key`
- Crate: `winapi` or `credential-manager`

### Linux
- Use Secret Service (D-Bus)
- Key pattern: `text-rewriter.<provider_name>.api_key`
- Crate: `secret-service`

**Implementation detail**: [crates/secure_store/src/lib.rs](crates/secure_store/src/lib.rs) will wrap platform-specific libraries behind a common interface.

## Migration Path: Mock → Real Providers

### Current (Phase 1): Mock Only
```rust
let provider = MockProvider::new(config);
let result = provider.rewrite(&request)?;
```

### Phase 2: Config-Driven Selection
```rust
// Load config from user's file
let provider_config = load_config_from_file()?;

// Create provider based on type
let provider: Box<dyn RewriteProvider> = match provider_config.provider_type {
    "ollama" => Box::new(OllamaProvider::new(provider_config)),
    "openai_compatible" => Box::new(OpenAICompatibleProvider::new(provider_config)),
    "anthropic" => Box::new(AnthropicProvider::new(provider_config)),
    _ => Box::new(MockProvider::new(provider_config)),  // Fallback
};

let result = provider.rewrite(&request)?;
```

## First-Run Experience (Future)

### Desktop App Flow

1. **Start app** → "Welcome to Text Rewriter"
2. **Choose provider**
   - Option A: Use free local model (Ollama) – recommended
   - Option B: Use my cloud API key – power user
3. **If Option A (Ollama)**
   - Show: "Do you have Ollama installed?"
   - Yes → Pick model (mistral, neural-chat, etc)
   - No → Quick install link + Docker command
4. **If Option B (Cloud)**
   - Dropdown to select: OpenAI / Anthropic / Other
   - Paste API key (stored securely, never shown again)
   - Paste model name (e.g., gpt-4o-mini)
   - Test connection button
5. **Save config** → Ready to use

## Error Handling Strategy

### If local Ollama is unreachable
- Warning: "Local LLM not responding. Make sure Ollama is running."
- Show: `docker run -d -p 11434:11434 ollama/ollama`
- Fall back to mock provider for demo

### If cloud API key is invalid
- Clear error: "Invalid API key for OpenAI"
- Show: "Check your API key in Settings ("

### If user hits rate limit
- Friendly message: "Rate limited. Retrying in 30 seconds…"
- Implement exponential backoff

## Costs and Privacy Matrix

| Approach | Cost | Privacy | Setup | Offline |
|----------|------|---------|-------|---------|
| Local Ollama | $0 | Excellent | Easy | ✓ Works offline |
| OpenAI API | Pay per token | Poor | Easy (API key) | ✗ Requires internet |
| Anthropic API | Pay per token | Poor | Easy (API key) | ✗ Requires internet |
| Self-hosted (VPS) | Cost of hardware | Good | Complex | ✓ If on VPS |

## Why This Approach?

1. **User autonomy**: No vendor lock-in
2. **Sustainability**: No cloud costs for us
3. **Privacy-first**: Local by default
4. **Flexibility**: Users choose
5. **Trust**: Users control their data
6. **Scaling**: We don't need expensive inference infrastructure

---

**Next Action**: Implement Phase 2a (Ollama adapter) so developers and users can test immediately with a free, local stack.
