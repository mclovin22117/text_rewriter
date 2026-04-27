# Development Guide

This document explains how to set up, run, and develop the Text Rewriter project locally.

## Prerequisites

- Rust 1.95+ (installed via rustup)
- Cargo (included with Rust)
- Git
- For local LLM testing: Docker (optional)

## Quick Setup

### 1. Clone and Enter Project

```bash
cd /path/to/text_rewriter
```

### 2. Verify Your Environment

```bash
cargo --version
rustc --version
```

Should output version info for both.

## Running Tests

Run all unit tests across the workspace:

```bash
cargo test
```

Run tests for a specific crate:

```bash
cargo test -p text-rewriter-core
cargo test -p text-rewriter-providers
```

Run with output printed:

```bash
cargo test -- --nocapture
```

## Running the Desktop App (Current Phase)

The desktop app is currently a CLI that takes a command and source text.

### Default smoke test:

```bash
cargo run -p text-rewriter-desktop
```

Output:
```
provider=mock model=starter-model
rewritten_text=[mock:starter-model] teh quick brown fox jump over teh lazy dog
```

### Custom command and text:

```bash
cargo run -p text-rewriter-desktop -- ?improve "this are bad text"
```

```bash
cargo run -p text-rewriter-desktop -- ?formal "yo, what's up?"
```

### Supported commands:

- `?fix` – grammar, spelling, punctuation
- `?improve` – clarity and flow
- `?enhance` – readability and style
- `?shorten` – concise version
- `?formal` – professional tone
- `?casual` – friendly tone

## Provider Configuration

Currently, the desktop app hardcodes a mock provider. The next phase will add real provider support and configuration.

### Two Recommended Approaches

#### Approach 1: Bring Your Own API Key (BYO-LLM)

**How it works:**
- User provides API key from their chosen provider (OpenAI, Anthropic, etc).
- Key is stored securely in OS keychain/credential manager.
- Rewrite requests are sent to user's provider endpoint.
- User is billed by their provider per request.

**Pros:**
- Immediate access to powerful models
- No local resource overhead
- Easy multi-model switching

**Cons:**
- Requires API key setup
- User pays per request
- Data is sent to external provider

**Example config (future):**

```toml
[provider]
name = "openai"
model = "gpt-4o-mini"
api_key = "<stored in OS keychain>"
base_url = "https://api.openai.com/v1"
```

#### Approach 2: Local LLM (Self-Hosted, No API Key)

**How it works:**
- User runs a local LLM server via Docker (e.g., Ollama or LMStudio)
- Text Rewriter connects to localhost endpoint
- All text transformation happens on user's machine
- Zero API costs, full privacy

**Pros:**
- Complete privacy (no external API calls)
- No API key management
- No usage costs
- Works offline
- Full control over model selection

**Cons:**
- Requires local resources (GPU recommended)
- Slightly slower than cloud APIs
- Setup requires Docker/container knowledge

**Example config (future):**

```toml
[provider]
name = "ollama"
model = "mistral"  # or neural-chat, orca-mini, etc
api_key = ""  # not needed
base_url = "http://localhost:11434/api"
```

### Recommended Hybrid Model

**Best practice (future implementation):**

1. **Default to local Ollama** with one-click Docker setup
   - Users can optionally run: `docker run -d -p 11434:11434 ollama/ollama`
   - Text Rewriter auto-detects local endpoint
   
2. **Fall back to OpenAI** if user provides API key
   - If local endpoint unavailable and OpenAI key is set, use that
   
3. **Let user choose** in Settings
   - Simple toggle: "Local (free)" vs "Cloud (API key required)"

This gives best-of-both-worlds:
- New users can start instantly with no setup (local Ollama)
- Power users can use their preferred cloud provider
- Expert users can run any compatible local endpoint

## Local LLM Setup for Development

### Option A: Ollama (Recommended for Development)

Ollama is lightweight and supports many models.

**Install Ollama:**
- macOS/Windows: https://ollama.ai/download
- Linux:
  ```bash
  curl https://ollama.ai/install.sh | sh
  ```

**Run a local model:**
```bash
ollama run mistral
```

First run downloads the model (~5-15GB depending on model choice).

Available lightweight models good for text rewriting:
- `ollama run neural-chat` (~4GB) – fastest, good for quick rewrites
- `ollama run mistral` (~5GB) – balanced quality/speed
- `ollama run orca-mini` (~2GB) – smallest, good for testing

**Verify it's running:**
```bash
curl http://localhost:11434/api/generate -d '{"model":"mistral","prompt":"hello"}'
```

### Option B: Docker + Ollama

```bash
# Pull Ollama image
docker pull ollama/ollama

# Run container with GPU support (macOS/Linux/Windows WSL2)
docker run -d --gpus all -v ollama:/root/.ollama -p 11434:11434 --name ollama ollama/ollama

# Or run without GPU (slower but works on any machine)
docker run -d -v ollama:/root/.ollama -p 11434:11434 --name ollama ollama/ollama

# Pull and run a model
docker exec -it ollama ollama run mistral
```

**Verify:**
```bash
curl http://localhost:11434/api/generate -d '{"model":"mistral","prompt":"hello"}'
```

### Option C: LM Studio (GUI)

https://lmstudio.ai/

- Download and install
- Select a model
- Start local server (runs on http://localhost:1234)
- Text Rewriter connects automatically

## Development Workflow

### 1. Start Your Typical Dev Session

**Terminal 1 (optional, if using local LLM):**
```bash
# If using Ollama
ollama run mistral

# Or Docker
docker start ollama
```

**Terminal 2 (main work):**
```bash
cd /path/to/text_rewriter
cargo test
```

### 2. Make Code Changes

Edit source files as usual:
- [crates/core/src/lib.rs](crates/core/src/lib.rs) – command parsing, prompt building
- [crates/providers/src/lib.rs](crates/providers/src/lib.rs) – provider adapters
- [crates/platform/src/lib.rs](crates/platform/src/lib.rs) – OS-specific logic
- [crates/secure_store/src/lib.rs](crates/secure_store/src/lib.rs) – credential storage
- [apps/desktop/src/main.rs](apps/desktop/src/main.rs) – CLI or app entry

### 3. Validate Changes

```bash
# Run tests
cargo test

# Build desktop app
cargo build -p text-rewriter-desktop

# Run with custom input
cargo run -p text-rewriter-desktop -- ?fix "my text here"
```

### 4. Commit and Push

```bash
git add .
git commit -m "description of changes"
git push origin main
```

## Common Development Tasks

### Add a New Command Type

1. Add variant to `RewriteCommand` enum in [crates/core/src/lib.rs](crates/core/src/lib.rs)
2. Add parsing case in `from_token()`
3. Add system instruction in `system_instruction()` method
4. Add test case

### Add a New Provider Adapter

1. Create new type in [crates/providers/src/lib.rs](crates/providers/src/lib.rs)
2. Implement `RewriteProvider` trait
3. Add tests
4. Wire into provider factory

Example skeleton:
```rust
pub struct MyProvider {
    config: ProviderConfig,
}

impl RewriteProvider for MyProvider {
    fn name(&self) -> &'static str {
        "my-provider"
    }

    fn rewrite(&self, request: &RewriteRequest) -> Result<RewriteResult, ProviderError> {
        // Implementation
    }
}
```

### Run Linter and Format

```bash
cargo fmt              # Format code
cargo clippy           # Lint with suggestions
cargo clippy --fix     # Auto-fix where possible
```

## Troubleshooting

### Cargo compilation errors

```bash
# Clean and rebuild
cargo clean
cargo build
```

### Test failures after changes

```bash
# Run tests with backtrace
RUST_BACKTRACE=1 cargo test -- --nocapture
```

### Local LLM endpoint not responding

```bash
# Check if Ollama/service is running
curl http://localhost:11434/api/generate -d '{"model":"mistral","prompt":"hi"}'

# If not, restart
docker restart ollama  # or just run ollama run mistral again
```

## Next Development Phases

### Phase 1 (Current): Workspace bootstrap ✓
- CLI smoke test working
- Core domain models in place
- Unit tests passing

### Phase 2: Real Provider Integration
- Replace mock provider with real adapters (OpenAI, Ollama)
- Add config loader
- Add secure key storage

### Phase 3: Settings UI
- Tauri app shell
- Provider configuration UI
- Test connection button

### Phase 4: Text Capture/Replace
- OS-specific text selection
- Text injection back to source app
- Hotkey triggering

### Phase 5: Production Hardening
- Error handling and recovery
- Retry logic
- Packaging and signing

## Resources

- **Rust Guide**: https://doc.rust-lang.org/book/
- **Cargo Guide**: https://doc.rust-lang.org/cargo/
- **Ollama Models**: https://ollama.ai/library
- **Tauri Docs**: https://tauri.app/en/docs/
- **OpenAI API**: https://platform.openai.com/docs/

---

**Next Step**: Phase 2 should implement real provider adapters and allow users to configure their chosen provider (local or cloud) via simple settings.
