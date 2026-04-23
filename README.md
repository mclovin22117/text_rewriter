# Text Rewriter

Cross-platform command-based text rewriting for any app.

Text Rewriter is a desktop-first utility that rewrites selected text using commands like `?fix`, `?improve`, and `?enhance`, powered by each user's own LLM API configuration.

## Why this project
- Works across macOS, Windows, and Linux.
- Scales with BYO-LLM: users bring their own provider, model, and key.
- Secure by design: API keys are stored in OS credential vaults.
- Built for reliability: graceful fallbacks for platform-specific text integration limits.

## Core concept
1. User selects text in any app.
2. User triggers Text Rewriter via hotkey/command palette.
3. User picks a command like `?fix` or `?enhance`.
4. App sends rewrite request through selected provider.
5. Rewritten text is inserted back into the source app.

## Command examples (MVP)
- `?fix`: grammar, spelling, punctuation.
- `?improve`: clearer and smoother phrasing.
- `?enhance`: stronger style and readability.
- `?shorten`: concise rewrite.
- `?formal`: professional tone.
- `?casual`: conversational tone.

## Recommended stack
- Core runtime: Rust
- Desktop shell: Tauri
- UI: Svelte (or React)
- Local metadata storage: SQLite
- Secret storage:
  - macOS Keychain
  - Windows Credential Manager
  - Linux Secret Service (libsecret)

## BYO-LLM provider support
Planned adapters:
- OpenAI-compatible endpoints
- Anthropic
- Google Gemini
- Local/self-hosted endpoints (Ollama, LM Studio, compatible APIs)

## High-level architecture
- Trigger Manager
- Text Capture Adapter
- Command Parser
- Prompt Builder
- LLM Provider Abstraction
- Rewrite Orchestrator
- Text Replace Adapter
- Settings and Secrets Manager

See full architecture in [SYSTEM_DESIGN.md](SYSTEM_DESIGN.md).

## Cross-platform strategy
- macOS: accessibility APIs + clipboard fallback
- Windows: UI automation/text services + clipboard fallback
- Linux: X11/Wayland-aware behavior + robust fallback workflow

## Security and privacy
- No plaintext API key storage.
- Minimal operational logging by default.
- Explicit user awareness that text is sent to their selected provider.
- Optional local-only mode for local model endpoints.

## Repository status
Current stage: planning and architecture documentation.

## Next implementation milestones
1. Bootstrap Rust workspace and Tauri app shell.
2. Build provider settings + secure key storage.
3. Implement command parser and prompt builder.
4. Ship selected-text rewrite loop on one OS, then expand.
5. Add error taxonomy, retries, and release pipeline.

## Documentation
- Architecture and implementation plan: [SYSTEM_DESIGN.md](SYSTEM_DESIGN.md)

## License
License will be added in a follow-up commit.
