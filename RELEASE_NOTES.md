# tiny-agent v0.1.0 — Initial release

Tag: v0.1.0

This is the initial public release of tiny-agent, a zero-overhead Rust harness for running agentic ReAct-style loops on sub-500M parameter language models.

Highlights
- Zero-overhead execution: small single-binary Rust harness (Tokio + reqwest) designed for low-latency local LLMs.
- Forgiving XML-like protocol: models return <thought>, <call>, <arg>, <observation>, and <answer> tags parsed by the ForgivingParser.
- Trait-based tool registry: implement the `Tool` trait and register tools with MicroAgent.
- Example tools included: CalculatorTool (math) and WeatherTool (mocked example).

Notes
- Package version is 0.1.0 (see Cargo.toml). Suggested release tag: `v0.1.0`.
- WeatherTool returns mocked data in this release; wire a real weather API before using in production.

How to run

Build and run locally (requires a small LLM runner exposing an OpenAI-compatible chat completions endpoint, e.g., Ollama):

```bash
cargo build --release
# run the example CLI
cargo run --release
# or run the compiled binary
./target/release/tiny-agent-harness
```

If you want me to create the GitHub Release object (publish the release, attach assets, or mark as draft/prerelease), I can't create GitHub releases directly from here. Instead, run one of these commands locally or in CI to publish the release using the tag above:

Using the gh CLI (recommended):

```bash
# create and publish the release using the notes file
gh release create v0.1.0 --title "tiny-agent v0.1.0 — Initial release" --notes-file RELEASE_NOTES.md --target main
```

Using the GitHub API (curl):

```bash
curl -H "Authorization: token $GITHUB_TOKEN" \
  -d '{"tag_name":"v0.1.0","target_commitish":"main","name":"tiny-agent v0.1.0","body":"Initial release of tiny-agent. See RELEASE_NOTES.md in the repo for details.","draft":false,"prerelease":false}' \
  https://api.github.com/repos/zencoder01/tiny-agent/releases
```

Want me to also add a GitHub Actions workflow that builds release artifacts and uploads them to Releases? Say the word and I will add a minimal workflow that builds the binary on push to tags and uploads artifacts.