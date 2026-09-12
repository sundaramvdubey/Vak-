# Vāk Browser Playground Plan

**Status:** Architecture defined; implementation intentionally deferred until a secure compiler backend exists.

The current Vāk compiler is a native Rust binary that invokes Clang. A browser page must not execute arbitrary compiler commands, accept unrestricted filesystem access, or expose a long-lived privileged compiler service. The website therefore presents documentation and examples today, while the live playground remains a clearly labelled future capability.

## Approved implementation paths

### Path A: WebAssembly compiler backend

Compile the lexer, parser, semantic analyzer, and LLVM/text emitter to WebAssembly. The browser worker receives source text and returns diagnostics plus generated IR. The worker must have no DOM, filesystem, network, or host-process capability. A hard input-size limit, execution timeout, and memory cap are required.

The preferred first version is a check-only playground. Native linking and execution should not happen in the browser unless a separate sandboxed runtime is specified.

### Path B: Isolated compiler service

Expose a narrowly scoped HTTPS endpoint that accepts source text and returns structured diagnostics and IR. The service must run in an isolated container or microVM with:

- no host filesystem access;
- no network egress from compilation workers;
- strict CPU, memory, wall-clock, and output-size limits;
- a non-root user;
- an allowlisted compiler command;
- request authentication and rate limiting;
- structured JSON responses only;
- no arbitrary command, path, or environment input from the browser.

The browser client should use a short-lived request and show a clear “server-side compilation” indicator. Source retention must be disabled by default.

## Frontend contract

The future playground should provide:

1. A Monaco- or CodeMirror-style editor with Vāk examples.
2. `Check` as the first available action.
3. Diagnostics mapped to source line and column.
4. An expandable LLVM IR panel.
5. A deterministic offline error state when the backend is unavailable.
6. A visible maximum source size and execution timeout.
7. No arbitrary native executable download or execution from the page.

## Acceptance criteria before enabling the live editor

The playground may be marked live only after one backend path is implemented, security tests cover resource exhaustion and hostile input, CI exercises the backend, and the website has a documented privacy policy for submitted source. Until then, the website must keep the playground CTA labelled as planned rather than implying that compilation already works in-browser.
