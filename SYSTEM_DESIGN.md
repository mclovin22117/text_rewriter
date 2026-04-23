# System Design: Cross-Platform Command-Based Text Rewriter

## 1. Vision
Build a cross-platform desktop service that rewrites user-selected text from any app when triggered with commands such as ?fix, ?improve, ?enhance, ?shorten, ?formal, and ?casual.

The product follows a BYO-LLM model:
- Users bring their own API key and model.
- Users choose provider and model in Settings.
- Rewrite requests are billed directly by the provider to the user.
- The app does not depend on a single centralized inference backend.

## 2. Product Goals
- Work reliably on macOS, Windows, and Linux.
- Keep user experience fast and low-friction.
- Keep secrets secure using platform keychain/credential stores.
- Support multiple LLM providers and custom endpoints.
- Be extensible to Android and iOS in a future phase.

## 3. Non-Goals (Initial Versions)
- Full real-time typing interception in every app field.
- Browser extension parity on day one.
- Team collaboration and shared org billing.
- Complex document editor features.

## 4. Recommended Tech Stack

### 4.1 Core Runtime
- Language: Rust
- Why:
  - Strong cross-platform support.
  - High performance for always-on background agent.
  - Memory safety and mature ecosystem.
  - Good fit for OS-level integration.

### 4.2 Desktop Shell + UI
- Framework: Tauri
- UI options: Svelte or React (Svelte preferred for lighter bundle)
- Why:
  - Native desktop packaging with low memory footprint.
  - Rust backend integration is first-class.

### 4.3 Persistence and Config
- Local metadata DB: SQLite
- Config serialization: JSON or TOML
- Secret storage: OS secure vault only
  - macOS Keychain
  - Windows Credential Manager
  - Linux Secret Service (libsecret)

### 4.4 Networking and Model Access
- HTTP client: reqwest (Rust)
- Provider support strategy:
  - OpenAI-compatible APIs
  - Anthropic
  - Google Gemini
  - Local servers (Ollama, LM Studio, self-hosted OpenAI-compatible)

### 4.5 Observability
- Logging: tracing + structured logs
- Crash reporting: optional and opt-in only
- Metrics: local-only diagnostics in early versions

## 5. High-Level Architecture

## 5.1 Modules
1. Trigger Manager
- Handles global hotkeys.
- Opens command palette or executes default rewrite command.

2. Text Capture Adapter
- Captures selected text from active application.
- Uses OS-specific methods and fallback clipboard strategy.

3. Command Parser
- Parses command tokens such as ?fix and ?improve.
- Resolves defaults and user preferences.

4. Prompt Builder
- Builds deterministic instruction templates per command.
- Injects language, tone, and constraint settings.

5. LLM Provider Abstraction
- Uniform interface for all providers.
- Handles auth, model name, endpoint format, retries, and errors.

6. Rewrite Orchestrator
- End-to-end flow coordinator.
- Handles timeout, cancellation, fallback, and final output.

7. Text Replace Adapter
- Replaces selected text in the active app.
- Preserves user workflow and cursor behavior where possible.

8. Settings and Secrets Manager
- Stores provider config and non-secret preferences.
- Stores API keys securely in OS vault.

9. Desktop UI
- Onboarding, provider setup, command presets, logs, and diagnostics.

10. Update and Versioning Service
- Safe auto-update channel for desktop app releases.

## 5.2 Data Flow (Primary Path)
1. User selects text in any app.
2. User presses hotkey (or invokes command palette).
3. Command is selected (?fix, ?enhance, etc).
4. Text Capture Adapter reads selected text.
5. Prompt Builder creates instruction payload.
6. Provider adapter sends request using user key.
7. Response is normalized and validated.
8. Text Replace Adapter writes rewritten text back to source app.
9. Local log entry records status (without storing sensitive content by default).

## 6. BYO-LLM Design

## 6.1 Provider Interface Contract
Single internal contract for all providers:

- Inputs:
  - command
  - source_text
  - model_config
  - rewrite_options (tone, length, language)
  - timeout
- Output:
  - rewritten_text
  - usage metadata (tokens if available)
  - provider metadata (model used, latency)
- Error Types:
  - InvalidAuth
  - QuotaExceeded
  - RateLimited
  - Timeout
  - ServiceUnavailable
  - ResponseParseError
  - PolicyBlocked

## 6.2 User Setup Flow
1. Select provider.
2. Enter API key.
3. Enter model name.
4. Optional custom base URL.
5. Click Test Connection.
6. Save config only if test succeeds.

## 6.3 Secrets Policy
- Never store plaintext API keys in DB or logs.
- Key retrieval only at request time.
- Zero secrets printed in diagnostics.
- Keys removable with one-click revoke action in settings.

## 7. Cross-Platform Strategy

## 7.1 macOS
- Primary: Accessibility API for text interactions.
- Fallback: clipboard-based replacement sequence.

## 7.2 Windows
- Primary: UI Automation/Text Services.
- Fallback: clipboard-based replacement sequence.

## 7.3 Linux
- X11 and Wayland handled separately.
- Wayland limitations expected for global text operations.
- Provide robust fallback:
  - selected text copy
  - rewrite
  - paste replacement with user confirmation where needed

## 7.4 Reliability Principle
Use capability detection at runtime. If native replace is unavailable, degrade gracefully to clipboard-assisted workflow and inform user.

## 8. Command and Prompt System

## 8.1 Command Set (MVP)
- ?fix: grammar, spelling, punctuation only.
- ?improve: clearer and smoother phrasing.
- ?enhance: stronger style and readability.
- ?shorten: concise rewrite.
- ?formal: professional tone.
- ?casual: conversational tone.

## 8.2 Prompt Design Rules
- Keep prompts deterministic and scoped.
- Preserve original language by default.
- Avoid adding facts not present in source text.
- Use structured response expectation:
  - plain rewritten text only

## 8.3 Safety and Quality Guards
- Maximum input length threshold per request.
- Optional profanity/offensive rewriting policy mode.
- Retry on transient transport errors only.

## 9. Security, Privacy, and Compliance

## 9.1 Privacy Defaults
- Do not persist source or rewritten text by default.
- Logging captures operational metadata only.
- Explicit opt-in required for storing prompt/response history.

## 9.2 User Transparency
- Clear notice in settings: text is sent to user-selected provider.
- Show provider, endpoint, and model for each request.

## 9.3 Hardening
- TLS validation enabled always.
- Disallow insecure HTTP unless developer mode enabled.
- Input sanitization for command and provider fields.

## 10. User Experience Design

## 10.1 Core UX Loop
- Select text -> Trigger -> Choose command -> Replace text.

## 10.2 Onboarding
1. Welcome screen with privacy explanation.
2. Provider selection.
3. API key and model setup.
4. Connection test.
5. First rewrite tutorial.

## 10.3 Settings Sections
- Account/Provider
- Commands and defaults
- Hotkeys
- Privacy and logs
- Advanced network settings

## 11. Deployment and Packaging

## 11.1 Desktop Packaging
- Tauri native bundles for:
  - macOS
  - Windows
  - Linux

## 11.2 Update Strategy
- Signed releases.
- Incremental updates where supported.
- Rollback support for failed updates.

## 11.3 CI/CD
- Build matrix across three desktop OSs.
- Unit and integration tests before release.
- Smoke tests for rewrite lifecycle.

## 12. Testing Strategy

## 12.1 Unit Tests
- Command parsing
- Prompt builder outputs
- Provider response normalization
- Error mapping

## 12.2 Integration Tests
- End-to-end rewrite flow with mocked providers
- Secrets storage retrieval
- Config migration compatibility

## 12.3 Platform Validation
- Manual and automated smoke tests per OS:
  - text capture success
  - rewrite request success
  - replacement success
  - fallback path success

## 13. Performance Targets (MVP)
- Trigger-to-request dispatch: under 150 ms local overhead.
- P50 end-to-end rewrite (network included): under 2.5 s.
- Idle memory budget for background agent: under 200 MB.

## 14. Extensibility for Mobile (Future)

## 14.1 Shared Core
Refactor rewrite orchestration into reusable Rust core library with stable API contract.

## 14.2 Mobile Clients
- Android: native app with accessibility/input-method integration options.
- iOS: extension-based workflow constraints expected; likely share settings and cloud profile but with platform-specific interaction model.

## 14.3 Architecture Rule
Keep provider abstraction, prompt templates, and policy engine platform-agnostic so desktop and mobile reuse the same core behavior.

## 15. Suggested Project Structure

text_rewriter/
- apps/
  - desktop/                # Tauri app
- crates/
  - core/                   # command parser, prompt builder, orchestration
  - providers/              # provider adapters
  - platform/               # OS-specific text capture/replace adapters
  - secure_store/           # keychain credential wrapper
  - telemetry/              # logging and metrics
- docs/
  - architecture.md
  - api_contracts.md
  - privacy_model.md
- tests/
  - integration/

## 16. Incremental Roadmap

## Phase 1: Foundation (2-3 weeks)
- Set up Rust workspace and Tauri shell.
- Build settings UI with provider setup and key storage.
- Implement command parser and prompt builder.

## Phase 2: Rewrite Loop MVP (2-4 weeks)
- Implement selected-text capture and replace for each OS (with fallback).
- Add provider adapters for OpenAI-compatible and one additional provider.
- Ship hotkey-triggered rewrite for ?fix, ?improve, ?enhance.

## Phase 3: Production Hardening (2-3 weeks)
- Retry/timeouts/error taxonomy.
- Better diagnostics and user-facing failure messages.
- Packaging, signing, updates, and release pipeline.

## Phase 4: Expansion
- More providers and local model endpoints.
- Advanced command presets and user custom prompts.
- Mobile discovery and architecture spike.

## 17. Risks and Mitigations

1. OS API differences for text replacement
- Mitigation: capability detection + clipboard fallback + user transparency.

2. Provider response variance
- Mitigation: strict normalization and parsing contract.

3. User trust/privacy concerns
- Mitigation: explicit disclosure, no plaintext key storage, minimal logging.

4. Linux desktop fragmentation
- Mitigation: test matrix for X11/Wayland and documented known limitations.

## 18. Immediate Next Actions
1. Create Rust workspace with crate boundaries listed above.
2. Implement provider abstraction and one working adapter.
3. Build settings screen with secure key storage and test connection.
4. Implement selected-text rewrite hotkey loop on one OS first, then expand.

---
This design prioritizes reliable cross-platform behavior, BYO-LLM scalability, secure credential handling, and an execution path that can ship quickly without sacrificing long-term architecture quality.
