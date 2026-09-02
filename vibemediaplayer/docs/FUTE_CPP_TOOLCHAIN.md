# C/C++ toolchain for FUTE language transmute

FUTE’s `v01d lang --from cpp --to rust` path gets **much better** when a real C/C++ frontend is available. Two layers:

| Layer | Purpose | Packages |
|-------|---------|----------|
| **A. Compiler + headers** | Compile/reference Mixxx & parse realistically | `gcc-c++`, `clang`, headers |
| **B. libclang (AST)** | Accurate C/C++ AST for FUTE (not regex) | `clang-devel`, `llvm-devel` |
| **C. Mixxx build deps** | Only when cloning/building Mixxx itself | Qt6, PortAudio, … (below) |

You already have **g++ 16** and **cmake**. Missing for FUTE AST: **libclang**.

---

## Omarchy Linux — install (recommended)

```bash
# Core C++ + libclang + LLVM for FUTE
sudo pacman -S --needed \
  base-devel clang llvm \
  cmake ninja pkgconf \
  git ccache mold

# For Mixxx / Audio dependencies on Omarchy
sudo pacman -S --needed \
  qt6-base qt6-declarative qt6-svg \
  portaudio libsndfile chromaprint taglib \
  rubberband soundtouch protobuf
```

---

## Fedora (Legacy)

```bash
# Core C++ + libclang for FUTE
sudo dnf install -y \
  gcc-c++ clang clang-devel llvm-devel \
  clang-tools-extra \
  cmake ninja-build pkgconf-pkg-config \
  make git ccache mold
```


Verify:

```bash
clang++ --version
pkg-config --modversion libclang || ls /usr/lib64/libclang.so*
llvm-config --version
```

Then rebuild FUTE with the clang feature:

```bash
cd ~/rossaedwards/main/vibeaudio
cargo test -p fute --features clang-ast
cargo run -p fute --bin v01d --features clang-ast -- lang some.cpp -o out.rs --from cpp
```

---

## What each package is for

| Package | Why FUTE/Vibe needs it |
|---------|------------------------|
| `gcc-c++` | Baseline C++ ABI / system headers (`you have this`) |
| `clang` | Frontend FUTE/libclang uses; consistent AST |
| `clang-devel` | **`libclang` + `clang-c` headers** — FUTE AST feature |
| `llvm-devel` | `llvm-config`, linking |
| `clang-tools-extra` | `clang-format`, optional tidy for pre-normalize |
| `cmake` `ninja` | Mixxx & many C++ libs use these |
| `ccache` / `mold` | Faster rebuilds when iterating transmute |

---

## Mixxx-only (later — not required for FUTE itself)

When you clone Mixxx and want to **build** it or parse its full tree with correct includes:

```bash
# Fedora Mixxx-ish dependency set (approximate; Mixxx docs win if they differ)
sudo dnf install -y \
  qt6-qtbase-devel qt6-qtdeclarative-devel qt6-qtsvg-devel \
  qt6-qtshadertools-devel \
  portaudio-devel libebur128-devel \
  ffmpeg-free-devel \
  libchromaprint-devel taglib-devel \
  protobuf-devel \
  hidapi-devel libusb1-devel \
  rubberband-devel \
  sqlite-devel \
  upower-devel
```

Clone next to the monorepo:

```bash
git clone --depth 1 https://github.com/mixxxdj/mixxx.git ~/rossaedwards/main/mixxx
```

Then FUTE can target real headers:

```bash
cargo run -p fute --bin v01d --features clang-ast -- lang \
  ../mixxx/src/engine/enginebuffer.h \
  -o crates/vmp-vinyl/transmute_raw/enginebuffer.rs --from cpp
```

---

## How FUTE plugs into libclang

```
C++ source
   │
   ▼
libclang (parse translation unit)
   │
   ▼
FUTE Universal / intermediate IR
   │
   ▼
Rust generator  →  .rs scaffold
   │
   ▼
Human polish → crates/vmp-vinyl, vmp-viz, …
```

Without libclang, FUTE still does **structural** transmute (regex/line rules).  
With **`--features clang-ast`**, it prefers **AST-driven** extraction of structs, methods, and includes when `libclang` loads.

---

## Windows / macOS (reference)

| OS | Notes |
|----|--------|
| **Windows** | LLVM official installer or `winget install LLVM.LLVM`; set `LIBCLANG_PATH` |
| **macOS** | `xcode-select --install` + `brew install llvm`; often `export LIBCLANG_PATH="$(brew --prefix llvm)/lib"` |

---

## After install — smoke check

```bash
# 1. libclang visible
ls /usr/lib64/libclang.so*

# 2. FUTE clang feature
cd ~/rossaedwards/main/vibeaudio
cargo test -p fute --features clang-ast

# 3. Language transmute still works without clang (fallback)
cargo run -p fute --bin v01d -- lang \
  ../vibe-audio-visualizer/src/dsp_engine.c \
  -o /tmp/dsp_engine_fute.rs --from c
```

When packages are installed, say the word and we’ll turn on `clang-ast` end-to-end against Mixxx headers.
