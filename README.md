# BeamNow

A minimal, pure-Rust desktop coding agent with a clean single-window UI and customizable themes.

- **Works out of the box:** Kilo Code's free auto-router (`kilo-auto/free`) needs no API key.
- **Free models:** The model picker lists zero-cost, tool-capable models from Kilo Code and OpenRouter, fetched at startup.
- **Popular providers (BYOK):** OpenRouter, OpenAI, NVIDIA NIM, Anthropic, DeepSeek, plus any custom OpenAI-compatible endpoint (Ollama, LM Studio, vLLM, Groq, Together…).
- **Customizable themes:** Switch instantly between built-in dark and light presets (**Ember**, **Midnight**, **Nord**, **Forest**, **Rosé**, **Daylight**, **Paper**) or customize every surface, accent, border, status color, and corner radius with live preview.
- **Agent tools:** `read_file`, `write_file`, `edit_file`, `list_dir`, `grep`, `run_shell`. Writes and shell commands ask for inline approval unless auto-approve is enabled.
- **Persistent sessions:** Conversations are automatically saved locally per session.

## Build and run

```bash
cargo run --release
```

Configuration (including your active model, API keys, and custom theme palette) is stored at:
- **Windows:** `%APPDATA%\BeamNow\config.toml`
- **macOS / Linux:** `~/.config/BeamNow/config.toml`

## Notes

- OpenRouter's free models still need an OpenRouter API key (free to create).
- Live check against Kilo's free gateway: `cargo test live_kilo -- --ignored --nocapture`.

## License

Released into the public domain under the [Unlicense](UNLICENSE).
