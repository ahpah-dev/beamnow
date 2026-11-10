/* =========================================================================
   BEAMNOW — EDITORIAL COMMERCIAL ENGINE & INTERACTIVE THEME ATELIER
   ========================================================================= */

const PRESETS = [
  {
    id: "ember",
    name: "Ember",
    sub: "Warm charcoal & amber",
    bg: "#18181b",
    sidebar: "#121214",
    input: "#202024",
    bubble: "#27272c",
    tool: "#1d1d21",
    border: "#2e2e34",
    hover: "#26262b",
    selected: "#2d2d34",
    text: "#e4e4e8",
    muted: "#888894",
    accent: "#e09260",
    accentSoft: "#30241c",
    ok: "#74c082",
    dark: true,
  },
  {
    id: "midnight",
    name: "Midnight",
    sub: "Deep obsidian & blue",
    bg: "#0e1117",
    sidebar: "#0a0c11",
    input: "#151922",
    bubble: "#1a212e",
    tool: "#12161e",
    border: "#222938",
    hover: "#1b222f",
    selected: "#212a3a",
    text: "#e4e9f2",
    muted: "#7e8aa0",
    accent: "#60a5fa",
    accentSoft: "#16243c",
    ok: "#52d082",
    dark: true,
  },
  {
    id: "nord",
    name: "Nord",
    sub: "Arctic slate & frost",
    bg: "#2e3440",
    sidebar: "#272c36",
    input: "#373e4c",
    bubble: "#40495a",
    tool: "#333a47",
    border: "#454e60",
    hover: "#3e4656",
    selected: "#465062",
    text: "#eceff4",
    muted: "#9ea9bc",
    accent: "#88c0d0",
    accentSoft: "#34444f",
    ok: "#a3be8c",
    dark: true,
  },
  {
    id: "forest",
    name: "Forest",
    sub: "Deep pine & emerald",
    bg: "#111714",
    sidebar: "#0d120f",
    input: "#18201c",
    bubble: "#1e2923",
    tool: "#151c18",
    border: "#25322b",
    hover: "#1d2722",
    selected: "#233029",
    text: "#e2ece6",
    muted: "#84988c",
    accent: "#58c692",
    accentSoft: "#172e24",
    ok: "#68d28c",
    dark: true,
  },
  {
    id: "rose",
    name: "Rosé",
    sub: "Velvet plum & rose gold",
    bg: "#191724",
    sidebar: "#13111c",
    input: "#201d2e",
    bubble: "#29253a",
    tool: "#1d1a29",
    border: "#302c44",
    hover: "#262336",
    selected: "#2e2a41",
    text: "#eae4f2",
    muted: "#948ca8",
    accent: "#eb98b2",
    accentSoft: "#342230",
    ok: "#94d2ac",
    dark: true,
  },
  {
    id: "daylight",
    name: "Daylight",
    sub: "Crisp alabaster & cobalt",
    bg: "#fafafc",
    sidebar: "#f2f3f7",
    input: "#ffffff",
    bubble: "#ecf0f8",
    tool: "#f4f5f8",
    border: "#e0e3ea",
    hover: "#eaedf3",
    selected: "#e2e7f0",
    text: "#1a1d24",
    muted: "#707684",
    accent: "#2563eb",
    accentSoft: "#e4ecfc",
    ok: "#18944a",
    dark: false,
  },
  {
    id: "paper",
    name: "Paper",
    sub: "Warm parchment & sienna",
    bg: "#f8f4ec",
    sidebar: "#f0eadf",
    input: "#fdfaf4",
    bubble: "#eee6d8",
    tool: "#f3ede3",
    border: "#ded5c6",
    hover: "#eae2d4",
    selected: "#e2d8c8",
    text: "#2c2620",
    muted: "#7c7164",
    accent: "#c05c36",
    accentSoft: "#f2e1d6",
    ok: "#2e8a4e",
    dark: false,
  },
];

let activePreset = PRESETS[0];
let activeRadius = 8;

/* =========================================================================
   1. EDITORIAL READING MODE TOGGLE (PAPER / INK)
   ========================================================================= */
const editorialModeToggle = document.getElementById("editorialModeToggle");
const editorialModeLabel = document.getElementById("editorialModeLabel");

editorialModeToggle.addEventListener("click", () => {
  const html = document.documentElement;
  const current = html.getAttribute("data-editorial-mode");
  const next = current === "paper" ? "ink" : "paper";
  html.setAttribute("data-editorial-mode", next);
  editorialModeLabel.textContent = next === "paper" ? "Edition: Paper" : "Edition: Ink";
});

/* =========================================================================
   2. THEME ATELIER & INTERACTIVE SPECIMEN
   ========================================================================= */
const presetStrip = document.getElementById("presetStrip");
const radiusSlider = document.getElementById("radiusSlider");
const radiusValueLabel = document.getElementById("radiusValueLabel");
const customBgPicker = document.getElementById("customBgPicker");
const customSidebarPicker = document.getElementById("customSidebarPicker");
const customAccentPicker = document.getElementById("customAccentPicker");
const customTextPicker = document.getElementById("customTextPicker");
const simSidebarThemeDot = document.getElementById("simSidebarThemeDot");
const copyTomlBtn = document.getElementById("copyTomlBtn");

function applySpecimenTheme(preset) {
  activePreset = { ...preset };
  const root = document.documentElement.style;
  root.setProperty("--app-bg", activePreset.bg);
  root.setProperty("--app-sidebar", activePreset.sidebar);
  root.setProperty("--app-input", activePreset.input);
  root.setProperty("--app-bubble", activePreset.bubble);
  root.setProperty("--app-tool", activePreset.tool);
  root.setProperty("--app-border", activePreset.border);
  root.setProperty("--app-hover", activePreset.hover);
  root.setProperty("--app-selected", activePreset.selected);
  root.setProperty("--app-text", activePreset.text);
  root.setProperty("--app-muted", activePreset.muted);
  root.setProperty("--app-accent", activePreset.accent);
  root.setProperty("--app-accent-soft", activePreset.accentSoft);
  root.setProperty("--app-ok", activePreset.ok);

  customBgPicker.value = activePreset.bg;
  customSidebarPicker.value = activePreset.sidebar;
  customAccentPicker.value = activePreset.accent;
  customTextPicker.value = activePreset.text;
  simSidebarThemeDot.textContent = `● ${activePreset.name}`;

  document.querySelectorAll(".preset-card").forEach((el) => {
    el.classList.toggle("active", el.dataset.id === activePreset.id);
  });
}

function renderPresetStrip() {
  presetStrip.innerHTML = "";
  PRESETS.forEach((p) => {
    const btn = document.createElement("button");
    btn.type = "button";
    btn.className = `preset-card ${p.id === activePreset.id ? "active" : ""}`;
    btn.dataset.id = p.id;
    btn.innerHTML = `
      <div class="preset-card-top">
        <span class="preset-card-name">${p.name}</span>
        <span style="color:${p.accent}">●</span>
      </div>
      <span class="preset-card-sub">${p.sub}</span>
      <div class="preset-swatches">
        <span class="preset-swatch" style="background:${p.bg}"></span>
        <span class="preset-swatch" style="background:${p.sidebar}"></span>
        <span class="preset-swatch" style="background:${p.input}"></span>
        <span class="preset-swatch" style="background:${p.accent}"></span>
        <span class="preset-swatch" style="background:${p.text}"></span>
      </div>
    `;
    btn.addEventListener("click", () => applySpecimenTheme(p));
    presetStrip.appendChild(btn);
  });
}

radiusSlider.addEventListener("input", (e) => {
  activeRadius = Number(e.target.value);
  radiusValueLabel.textContent = `${activeRadius}px`;
  document.documentElement.style.setProperty("--app-radius", `${activeRadius}px`);
});

[customBgPicker, customSidebarPicker, customAccentPicker, customTextPicker].forEach((picker) => {
  picker.addEventListener("input", () => {
    applySpecimenTheme({
      ...activePreset,
      id: "custom",
      name: "Custom",
      bg: customBgPicker.value,
      sidebar: customSidebarPicker.value,
      accent: customAccentPicker.value,
      text: customTextPicker.value,
    });
  });
});

function hexToRgbArray(hex) {
  const clean = hex.replace("#", "");
  return [
    parseInt(clean.substring(0, 2), 16),
    parseInt(clean.substring(2, 4), 16),
    parseInt(clean.substring(4, 6), 16),
  ];
}

copyTomlBtn.addEventListener("click", () => {
  const toml = `[theme]
preset = "${activePreset.id}"
corner_radius = ${activeRadius}

[theme.custom]
dark_mode = ${activePreset.dark}
bg = [${hexToRgbArray(activePreset.bg).join(", ")}]
sidebar = [${hexToRgbArray(activePreset.sidebar).join(", ")}]
accent = [${hexToRgbArray(activePreset.accent).join(", ")}]
text = [${hexToRgbArray(activePreset.text).join(", ")}]`;

  navigator.clipboard.writeText(toml);
  const prev = copyTomlBtn.innerHTML;
  copyTomlBtn.textContent = "✓ Copied to Clipboard";
  setTimeout(() => {
    copyTomlBtn.innerHTML = prev;
  }, 1800);
});

/* Simulated Sessions & Interactive Chat */
const SESSIONS = {
  theme: {
    title: "Refactor theme engine",
    items: [
      { type: "user", text: "Make the interface minimalist and add customizable themes in config.toml." },
      { type: "tool", name: "read_file", arg: "src/theme.rs" },
      { type: "tool", name: "edit_file", arg: "src/theme.rs (+218 -42)" },
      { type: "tool", name: "edit_file", arg: "src/views/settings.rs (+164 -38)" },
      {
        type: "assistant",
        text: "Done. Added 7 curated dark and light presets (`Ember`, `Midnight`, `Nord`, `Forest`, `Rosé`, `Daylight`, `Paper`) plus a custom RGB palette drawer and borderless low-chrome widgets.",
      },
    ],
  },
  audit: {
    title: "Audit HTTP tool calls",
    items: [
      { type: "user", text: "Check how tool schemas are serialized for Anthropic vs OpenAI endpoints." },
      { type: "tool", name: "grep", arg: "Wire::Anthropic in src/" },
      { type: "tool", name: "read_file", arg: "src/llm.rs" },
      {
        type: "assistant",
        text: "`src/llm.rs` maps the 6 tool definitions cleanly to both `POST /chat/completions` (`tools[].function`) and Anthropic's Messages API (`tools[].input_schema`).",
      },
    ],
  },
  empty: {
    title: "New chat",
    items: [
      {
        type: "assistant",
        text: "What should we build? Type a message below to test the interactive specimen.",
      },
    ],
  },
};

const simTranscript = document.getElementById("simTranscript");
const simChatTitle = document.getElementById("simChatTitle");
const simComposerInput = document.getElementById("simComposerInput");
const simSendBtn = document.getElementById("simSendBtn");

function renderSession(key) {
  const session = SESSIONS[key] || SESSIONS.theme;
  simChatTitle.textContent = session.title;
  simTranscript.innerHTML = "";
  session.items.forEach((item) => appendTranscriptEntry(item));
}

function appendTranscriptEntry(item) {
  const div = document.createElement("div");
  if (item.type === "user") {
    div.className = "sim-user-bubble";
    div.textContent = item.text;
  } else if (item.type === "tool") {
    div.className = "sim-tool-pill";
    div.innerHTML = `<span class="sim-tool-ok">✓</span><span class="sim-tool-name">${item.name}</span><span class="sim-tool-arg">${item.arg}</span>`;
  } else {
    div.className = "sim-assistant-text";
    div.textContent = item.text;
  }
  simTranscript.appendChild(div);
  simTranscript.scrollTop = simTranscript.scrollHeight;
}

document.querySelectorAll(".beam-session-row").forEach((btn) => {
  btn.addEventListener("click", () => {
    document.querySelectorAll(".beam-session-row").forEach((b) => b.classList.remove("active"));
    btn.classList.add("active");
    renderSession(btn.dataset.session);
  });
});

document.getElementById("simNewChatBtn").addEventListener("click", () => {
  renderSession("empty");
});

function handleSimSend() {
  const val = simComposerInput.value.trim();
  if (!val) return;
  simComposerInput.value = "";
  appendTranscriptEntry({ type: "user", text: val });
  setTimeout(() => {
    appendTranscriptEntry({ type: "tool", name: "read_file", arg: "src/main.rs" });
  }, 350);
  setTimeout(() => {
    appendTranscriptEntry({
      type: "assistant",
      text: `Inspected workspace for "${val}". Ready to apply surgical edits—no API key required with kilo-auto/free.`,
    });
  }, 850);
}

simSendBtn.addEventListener("click", handleSimSend);
simComposerInput.addEventListener("keydown", (e) => {
  if (e.key === "Enter") handleSimSend();
});

renderPresetStrip();
applySpecimenTheme(PRESETS[0]);
renderSession("theme");

/* =========================================================================
   3. EDITORIAL COMMERCIAL ENGINE ("BELIEVE NOW" — 0:17)
   ========================================================================= */
const audio = document.getElementById("commercialAudio");
const canvas = document.getElementById("commercialCanvas");
const ctx = canvas.getContext("2d");
const curtain = document.getElementById("cinemaCurtain");
const curtainPlayBtn = document.getElementById("curtainPlayBtn");
const heroWatchFilmBtn = document.getElementById("heroWatchFilmBtn");
const filmPlayPauseBtn = document.getElementById("filmPlayPauseBtn");
const filmPlayPauseIcon = document.getElementById("filmPlayPauseIcon");
const filmPlayPauseText = document.getElementById("filmPlayPauseText");
const filmRestartBtn = document.getElementById("filmRestartBtn");
const filmLoopBtn = document.getElementById("filmLoopBtn");
const filmExportBtn = document.getElementById("filmExportBtn");
const filmFullscreenBtn = document.getElementById("filmFullscreenBtn");
const filmTimelineTrack = document.getElementById("filmTimelineTrack");
const filmTimelineProgress = document.getElementById("filmTimelineProgress");
const cinemaSceneLabel = document.getElementById("cinemaSceneLabel");
const cinemaLyricCue = document.getElementById("cinemaLyricCue");
const cinemaTimecode = document.getElementById("cinemaTimecode");
const cinemaRecDot = document.getElementById("cinemaRecDot");
const cinemaShell = document.getElementById("cinemaShell");

audio.loop = true;

let audioCtx = null;
let analyser = null;
let sourceNode = null;
let streamDest = null;
let freqData = new Uint8Array(64);

function initAudioGraph() {
  if (audioCtx) return;
  const AudioContextClass = window.AudioContext || window.webkitAudioContext;
  audioCtx = new AudioContextClass();
  analyser = audioCtx.createAnalyser();
  analyser.fftSize = 128;
  analyser.smoothingTimeConstant = 0.78;
  sourceNode = audioCtx.createMediaElementSource(audio);
  streamDest = audioCtx.createMediaStreamDestination();
  sourceNode.connect(analyser);
  analyser.connect(audioCtx.destination);
  sourceNode.connect(streamDest);
  freqData = new Uint8Array(analyser.frequencyBinCount);
}

async function startCommercial(fromBeginning = false) {
  initAudioGraph();
  if (audioCtx && audioCtx.state === "suspended") {
    await audioCtx.resume();
  }
  if (fromBeginning) {
    audio.currentTime = 0;
  }
  await audio.play();
  curtain.classList.add("hidden");
  updateTransportUI();
}

function toggleCommercial() {
  if (audio.paused) {
    startCommercial(false);
  } else {
    audio.pause();
    updateTransportUI();
  }
}

function updateTransportUI() {
  const playing = !audio.paused;
  filmPlayPauseIcon.textContent = playing ? "⏸" : "▶";
  filmPlayPauseText.textContent = playing ? "Pause" : "Play";
  cinemaRecDot.classList.toggle("playing", playing);
}

curtainPlayBtn.addEventListener("click", () => startCommercial(true));
canvas.addEventListener("click", toggleCommercial);
filmPlayPauseBtn.addEventListener("click", toggleCommercial);
filmRestartBtn.addEventListener("click", () => startCommercial(true));

heroWatchFilmBtn.addEventListener("click", () => {
  document.getElementById("film").scrollIntoView({ behavior: "smooth" });
  setTimeout(() => startCommercial(true), 350);
});

filmLoopBtn.addEventListener("click", () => {
  audio.loop = !audio.loop;
  filmLoopBtn.classList.toggle("active", audio.loop);
  filmLoopBtn.textContent = audio.loop ? "∞ Loop: On" : "∞ Loop: Off";
});

filmFullscreenBtn.addEventListener("click", () => {
  if (!document.fullscreenElement) {
    cinemaShell.requestFullscreen?.();
  } else {
    document.exitFullscreen?.();
  }
});

filmTimelineTrack.addEventListener("click", (e) => {
  const rect = filmTimelineTrack.getBoundingClientRect();
  const ratio = Math.max(0, Math.min(1, (e.clientX - rect.left) / rect.width));
  const dur = audio.duration || 16.8;
  audio.currentTime = ratio * dur;
});

audio.addEventListener("ended", () => {
  updateTransportUI();
  if (!audio.loop) {
    curtain.classList.remove("hidden");
  }
});

/* Export Commercial as .webm video with audio */
filmExportBtn.addEventListener("click", async () => {
  if (typeof MediaRecorder === "undefined") {
    alert("MediaRecorder is not supported in this browser.");
    return;
  }
  const prevLoop = audio.loop;
  audio.loop = false;
  await startCommercial(true);

  const canvasStream = canvas.captureStream(60);
  const combinedTracks = [
    ...canvasStream.getVideoTracks(),
    ...(streamDest ? streamDest.stream.getAudioTracks() : []),
  ];
  const combinedStream = new MediaStream(combinedTracks);
  const mimeType = MediaRecorder.isTypeSupported("video/webm;codecs=vp9,opus")
    ? "video/webm;codecs=vp9,opus"
    : "video/webm";
  const recorder = new MediaRecorder(combinedStream, {
    mimeType,
    videoBitsPerSecond: 8_000_000,
  });

  const chunks = [];
  recorder.ondataavailable = (e) => {
    if (e.data && e.data.size > 0) chunks.push(e.data);
  };
  recorder.onstop = () => {
    audio.loop = prevLoop;
    filmExportBtn.textContent = "↓ Export Video (.webm)";
    const blob = new Blob(chunks, { type: mimeType });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = "beamnow-believe-now-commercial.webm";
    a.click();
  };

  filmExportBtn.textContent = "● Recording (0:17)…";
  recorder.start();

  const stopHandler = () => {
    if (recorder.state === "recording") recorder.stop();
    audio.removeEventListener("ended", stopHandler);
  };
  audio.addEventListener("ended", stopHandler);
});

/* =========================================================================
   FRAME-BY-FRAME EDITORIAL COMMERCIAL CHOREOGRAPHY (1600x900)
   Track Structure (16.8s):
   0.00 - 3.00s : Scene I   — Prelude ("Ooh... ooh...")
   3.00 - 5.85s : Scene II  — The Noise vs. The Signal
   5.85 - 9.50s : Scene III — "Believe, believe, believe now" (Minimalist Agent Reveal)
   9.50 - 12.85s: Scene IV  — "Believe now" Bass Swell (Rapid Beat-Matched Theme Showcase)
   12.85 - 16.8s: Scene V   — "Believe, believe, believe now" (Editorial Lockup Finale)
   ========================================================================= */

function formatTime(sec) {
  const s = Math.floor(sec);
  const ms = Math.floor((sec - s) * 100);
  return `00:${String(s).padStart(2, "0")}.${String(ms).padStart(2, "0")}`;
}

function getAudioEnergy() {
  if (!analyser || audio.paused) {
    // Gentle idle breathing when paused
    const idle = (Math.sin(performance.now() * 0.0025) + 1) * 0.12;
    return { bass: idle, mid: idle * 0.8, high: idle * 0.5 };
  }
  analyser.getByteFrequencyData(freqData);
  let bassSum = 0;
  let midSum = 0;
  let highSum = 0;
  for (let i = 0; i < 8; i++) bassSum += freqData[i];
  for (let i = 8; i < 28; i++) midSum += freqData[i];
  for (let i = 28; i < 56; i++) highSum += freqData[i];
  return {
    bass: bassSum / (8 * 255),
    mid: midSum / (20 * 255),
    high: highSum / (28 * 255),
  };
}

function drawRoundedRect(ctx, x, y, w, h, r) {
  ctx.beginPath();
  ctx.roundRect(x, y, w, h, r);
  ctx.closePath();
}

function drawEditorialFrame(ctx, w, h, palette, t, energy) {
  // Background fill
  ctx.fillStyle = palette.bg;
  ctx.fillRect(0, 0, w, h);

  // Radial audio-reactive ambient glow
  const glowRadius = 380 + energy.bass * 360;
  const grad = ctx.createRadialGradient(w * 0.5, h * 0.5, 20, w * 0.5, h * 0.5, glowRadius);
  grad.addColorStop(0, palette.accent + "30");
  grad.addColorStop(1, palette.accent + "00");
  ctx.fillStyle = grad;
  ctx.fillRect(0, 0, w, h);

  // Outer editorial border & corner registration marks
  const pad = 44;
  ctx.strokeStyle = palette.border;
  ctx.lineWidth = 1.5;
  ctx.strokeRect(pad, pad, w - pad * 2, h - pad * 2);

  // Top & Bottom Editorial Running Headers inside canvas
  ctx.fillStyle = palette.muted;
  ctx.font = '500 17px "JetBrains Mono", monospace';
  ctx.textAlign = "left";
  ctx.fillText("BEAMNOW // EDITORIAL FILM NO. 01", pad + 24, pad + 34);

  ctx.textAlign = "right";
  ctx.fillText(`SOUNDTRACK: "BELIEVE NOW" · ${formatTime(t)}`, w - pad - 24, pad + 34);

  // Audio Spectrum Visualizer Bar along bottom frame
  const barCount = 42;
  const barAreaW = 360;
  const startX = w - pad - 24 - barAreaW;
  const baseY = h - pad - 22;
  for (let i = 0; i < barCount; i++) {
    const val = analyser && !audio.paused ? freqData[i] / 255 : 0.12 + 0.08 * Math.sin(t * 4 + i * 0.3);
    const bh = Math.max(3, val * 34);
    ctx.fillStyle = i % 3 === 0 ? palette.accent : palette.muted;
    ctx.fillRect(startX + i * 8.5, baseY - bh, 5, bh);
  }

  ctx.textAlign = "left";
  ctx.fillStyle = palette.muted;
  ctx.fillText("PURE RUST · ZERO KEYS · THE UNLICENSE", pad + 24, h - pad - 22);
}

function drawSimulatedAgentCard(ctx, cx, cy, cw, ch, preset, progress, stepText, toolName, toolArg) {
  ctx.save();
  // Drop shadow
  ctx.shadowColor = "rgba(0,0,0,0.45)";
  ctx.shadowBlur = 45;
  ctx.shadowOffsetY = 18;

  // Window body
  drawRoundedRect(ctx, cx, cy, cw, ch, 14);
  ctx.fillStyle = preset.bg;
  ctx.fill();
  ctx.lineWidth = 2;
  ctx.strokeStyle = preset.border;
  ctx.stroke();
  ctx.shadowColor = "transparent";

  // Left sidebar
  ctx.save();
  drawRoundedRect(ctx, cx, cy, cw, ch, 14);
  ctx.clip();
  ctx.fillStyle = preset.sidebar;
  ctx.fillRect(cx, cy, 210, ch);
  ctx.strokeStyle = preset.border;
  ctx.beginPath();
  ctx.moveTo(cx + 210, cy);
  ctx.lineTo(cx + 210, cy + ch);
  ctx.stroke();

  // Sidebar content
  ctx.fillStyle = preset.text;
  ctx.font = '700 20px "Plus Jakarta Sans", sans-serif';
  ctx.fillText("BeamNow", cx + 22, cy + 40);

  // Selected session pill
  drawRoundedRect(ctx, cx + 14, cy + 64, 182, 36, 8);
  ctx.fillStyle = preset.selected;
  ctx.fill();
  ctx.fillStyle = preset.accent;
  ctx.font = '600 14px "Plus Jakarta Sans", sans-serif';
  ctx.fillText("●  " + preset.name + " workspace", cx + 26, cy + 87);

  // Theme indicator at bottom of sidebar
  ctx.fillStyle = preset.muted;
  ctx.font = '500 13px "JetBrains Mono", monospace';
  ctx.fillText("THEME: " + preset.name.toUpperCase(), cx + 22, cy + ch - 24);

  // Main chat area
  const mx = cx + 240;
  // User bubble
  drawRoundedRect(ctx, mx, cy + 36, cw - 275, 52, 10);
  ctx.fillStyle = preset.bubble;
  ctx.fill();
  ctx.fillStyle = preset.text;
  ctx.font = '500 16px "Plus Jakarta Sans", sans-serif';
  ctx.fillText(stepText, mx + 18, cy + 68);

  // Tool pill
  if (progress > 0.22) {
    drawRoundedRect(ctx, mx, cy + 106, cw - 275, 42, 8);
    ctx.fillStyle = preset.tool;
    ctx.fill();
    ctx.strokeStyle = preset.border;
    ctx.lineWidth = 1;
    ctx.stroke();

    ctx.fillStyle = preset.ok;
    ctx.font = '600 15px "JetBrains Mono", monospace';
    ctx.fillText("✓", mx + 16, cy + 132);
    ctx.fillStyle = preset.text;
    ctx.fillText(toolName, mx + 40, cy + 132);
    ctx.fillStyle = preset.muted;
    ctx.fillText(toolArg, mx + 150, cy + 132);
  }

  // Assistant markdown response
  if (progress > 0.45) {
    ctx.fillStyle = preset.text;
    ctx.font = '400 17px "Plus Jakarta Sans", sans-serif';
    ctx.fillText("Minimalist native interface compiled. Zero chrome, instant response.", mx, cy + 186);
  }

  // Composer at bottom
  drawRoundedRect(ctx, mx, cy + ch - 78, cw - 275, 54, 12);
  ctx.fillStyle = preset.input;
  ctx.fill();
  ctx.strokeStyle = preset.border;
  ctx.stroke();

  ctx.fillStyle = preset.muted;
  ctx.font = '400 14px "JetBrains Mono", monospace';
  ctx.fillText("kilo-auto/free · Ask before changes", mx + 18, cy + ch - 45);

  // Send button
  drawRoundedRect(ctx, cx + cw - 115, cy + ch - 67, 66, 32, 7);
  ctx.fillStyle = preset.accent;
  ctx.fill();
  ctx.fillStyle = "#18181b";
  ctx.font = '700 13px "Plus Jakarta Sans", sans-serif';
  ctx.fillText("Send", cx + cw - 96, cy + ch - 46);

  ctx.restore();
  ctx.restore();
}

function renderCommercialFrame() {
  const dur = audio.duration || 16.8;
  const t = audio.currentTime || 0;
  const energy = getAudioEnergy();

  // Update DOM progress & timecode
  const pct = Math.min(100, (t / dur) * 100);
  filmTimelineProgress.style.width = `${pct}%`;
  cinemaTimecode.textContent = `${formatTime(t)} / ${formatTime(dur)}`;

  const w = canvas.width;
  const h = canvas.height;

  // SCENE I: 0.00 - 3.00s ("Ooh... ooh...")
  if (t < 3.0) {
    cinemaSceneLabel.textContent = "SCENE I — PRELUDE";
    cinemaLyricCue.textContent = "♪ “Ooh… ooh…”";
    const pal = PRESETS[6]; // Warm Paper Editorial
    drawEditorialFrame(ctx, w, h, pal, t, energy);

    ctx.textAlign = "center";
    ctx.fillStyle = pal.accent;
    ctx.font = '500 18px "JetBrains Mono", monospace';
    ctx.fillText("ISSUE NO. 01 — AN EDITORIAL FILM FOR BEAMNOW", w / 2, h / 2 - 90);

    ctx.fillStyle = pal.text;
    ctx.font = 'italic 96px "Instrument Serif", Georgia, serif';
    ctx.fillText("In an era of loud software,", w / 2, h / 2 + 10);

    if (t > 1.45) {
      ctx.font = '400 68px "Instrument Serif", Georgia, serif';
      ctx.fillStyle = pal.muted;
      ctx.fillText("where do you go to think?", w / 2, h / 2 + 95);
    }
  }
  // SCENE II: 3.00 - 5.85s (The Restraint)
  else if (t < 5.85) {
    cinemaSceneLabel.textContent = "SCENE II — THE SIGNAL";
    cinemaLyricCue.textContent = "♪ “Ooh… ooh…”";
    const pal = PRESETS[0]; // Ember Dark
    drawEditorialFrame(ctx, w, h, pal, t, energy);

    const subT = t - 3.0;
    ctx.textAlign = "center";
    ctx.fillStyle = pal.muted;
    ctx.font = '500 20px "JetBrains Mono", monospace';
    ctx.fillText("STRIP AWAY THE NOISE", w / 2, h / 2 - 110);

    ctx.fillStyle = pal.text;
    ctx.font = '400 104px "Instrument Serif", Georgia, serif';
    if (subT < 0.95) {
      ctx.fillText("No Electron bloat.", w / 2, h / 2 + 15);
    } else if (subT < 1.9) {
      ctx.fillText("No mandatory keys.", w / 2, h / 2 + 15);
    } else {
      ctx.fillStyle = pal.accent;
      ctx.font = 'italic 112px "Instrument Serif", Georgia, serif';
      ctx.fillText("Pure native Rust.", w / 2, h / 2 + 15);
    }

    ctx.fillStyle = pal.muted;
    ctx.font = '400 22px "JetBrains Mono", monospace';
    ctx.fillText("read_file · write_file · edit_file · list_dir · grep · run_shell", w / 2, h / 2 + 105);
  }
  // SCENE III: 5.85 - 9.50s ("Believe, believe, believe now")
  else if (t < 9.5) {
    cinemaSceneLabel.textContent = "SCENE III — BELIEVE NOW";
    cinemaLyricCue.textContent = "♪ “Believe, believe, believe now…”";
    const pal = PRESETS[0]; // Ember
    drawEditorialFrame(ctx, w, h, pal, t, energy);

    const localP = (t - 5.85) / (9.5 - 5.85);

    // Big kinetic background lyric
    ctx.textAlign = "center";
    ctx.fillStyle = pal.accent + "22";
    ctx.font = 'italic 230px "Instrument Serif", Georgia, serif';
    ctx.fillText("Believe now.", w / 2, h / 2 - 135);

    // Foreground simulated agent window
    drawSimulatedAgentCard(
      ctx,
      w / 2 - 470,
      h / 2 - 125,
      940,
      340,
      pal,
      localP,
      "Refactor the UI into a quiet, minimalist workspace.",
      "edit_file",
      "src/views/chat.rs (+142 -310)"
    );
  }
  // SCENE IV: 9.50 - 12.85s ("Believe now" Bass Drop — Rapid Theme Atelier Morph)
  else if (t < 12.85) {
    cinemaSceneLabel.textContent = "SCENE IV — PALETTE METAMORPHOSIS";
    cinemaLyricCue.textContent = "♪ “Believe now…” (Bass Drop)";

    // Cycle through all presets on the beat (~0.55s per cut)
    const idx = Math.floor((t - 9.5) / 0.55) % PRESETS.length;
    const pal = PRESETS[idx];
    drawEditorialFrame(ctx, w, h, pal, t, energy);

    ctx.textAlign = "left";
    ctx.fillStyle = pal.accent;
    ctx.font = '600 20px "JetBrains Mono", monospace';
    ctx.fillText(`ATELIER PRESET 0${idx + 1} // ${pal.name.toUpperCase()}`, 110, 175);

    ctx.fillStyle = pal.text;
    ctx.font = 'italic 86px "Instrument Serif", Georgia, serif';
    ctx.fillText(`${pal.name} Edition.`, 110, 260);

    ctx.fillStyle = pal.muted;
    ctx.font = '400 24px "Plus Jakarta Sans", sans-serif';
    ctx.fillText(pal.sub + " — 100% customizable in config.toml", 110, 305);

    drawSimulatedAgentCard(
      ctx,
      110,
      345,
      1380,
      390,
      pal,
      1.0,
      `Switch active palette to ${pal.name} and run live workspace tools.`,
      "read_file",
      "Cargo.toml · license = \"Unlicense\""
    );
  }
  // SCENE V: 12.85 - 16.80s ("Believe, believe, believe now" — Editorial Lockup Finale)
  else {
    cinemaSceneLabel.textContent = "SCENE V — FINALE";
    cinemaLyricCue.textContent = "♪ “Believe, believe, believe now.”";
    const pal = PRESETS[6]; // Warm Paper Finale
    drawEditorialFrame(ctx, w, h, pal, t, energy);

    ctx.textAlign = "center";
    ctx.fillStyle = pal.accent;
    ctx.font = '600 22px "JetBrains Mono", monospace';
    ctx.fillText("◆  PUBLIC DOMAIN · THE UNLICENSE · v0.1.0", w / 2, h / 2 - 120);

    ctx.fillStyle = pal.text;
    ctx.font = '400 138px "Instrument Serif", Georgia, serif';
    ctx.fillText("BeamNow.", w / 2, h / 2 + 5);

    ctx.fillStyle = pal.accent;
    ctx.font = 'italic 54px "Instrument Serif", Georgia, serif';
    ctx.fillText("Quiet software for loud ideas.", w / 2, h / 2 + 78);

    ctx.fillStyle = pal.muted;
    ctx.font = '500 20px "JetBrains Mono", monospace';
    ctx.fillText("BEAMNOW.VERCEL.APP   ·   GITHUB.COM/AHPAH-DEV/BEAMNOW", w / 2, h / 2 + 150);
  }

  requestAnimationFrame(renderCommercialFrame);
}

requestAnimationFrame(renderCommercialFrame);
