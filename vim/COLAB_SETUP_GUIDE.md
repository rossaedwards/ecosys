# VIM Figure Scripts — Colab & Local Setup Guide

## Run All Locally First

From the `main/vim` directory:

```bash
cd main/vim/fig_scripts
python run_all_local.py
```

Output goes to `fig_scripts/` by default. All `.png` files appear there.

**Options:**
```bash
python run_all_local.py -o ./output     # Save figures to ./output
python run_all_local.py -n 10          # Run first 10 scripts only (smoke test)
python run_all_local.py --list         # List all scripts without running
```

**Requirements:** `numpy`, `matplotlib`, `scipy` (for some scripts). Install with:
```bash
pip install numpy matplotlib scipy
```

---

## Google Colab Setup

### Option A: Upload `fig_scripts` folder (recommended)

1. **Zip and upload**
   - Zip the entire `fig_scripts` folder (including `vim_common.py`)
   - In Colab: **Files** → **Upload** → select the zip
   - Unzip: add a cell and run:
     ```python
     !unzip -o fig_scripts.zip && mv fig_scripts /content/
     ```

2. **First cell — setup path and imports**
   ```python
   import sys
   sys.path.insert(0, '/content/fig_scripts')

   # Install deps if needed
   !pip install -q numpy matplotlib scipy
   ```

3. **Run a single figure**
   ```python
   from pathlib import Path
   from importlib.util import spec_from_file_location, module_from_spec

   script = Path('/content/fig_scripts/fig_001_001.py')
   spec = spec_from_file_location('fig', script)
   mod = module_from_spec(spec)
   spec.loader.exec_module(mod)
   mod.run_simulation(Path('/content/figures'))  # or None for cwd
   ```

4. **Run all figures**
   ```python
   from pathlib import Path
   import sys
   sys.path.insert(0, '/content/fig_scripts')

   out = Path('/content/figures')
   out.mkdir(exist_ok=True)

   for script in sorted(Path('/content/fig_scripts').glob('fig_*.py')):
       if script.name in ('run_all_local.py', 'vim_common.py'):
           continue
       try:
           from importlib.util import spec_from_file_location, module_from_spec
           spec = spec_from_file_location(script.stem, script)
           mod = module_from_spec(spec)
           spec.loader.exec_module(mod)
           mod.run_simulation(out)
           print(f"OK {script.name}")
       except Exception as e:
           print(f"FAIL {script.name}: {e}")
   ```

5. **Download figures**
   ```python
   from google.colab import files
   import shutil
   shutil.make_archive('vim_figures', 'zip', '/content/figures')
   files.download('vim_figures.zip')
   ```

---

### Option B: Clone from GitHub (if repo is public)

1. **First cell**
   ```python
   !git clone https://github.com/YOUR_USER/rossaedwards.git
   %cd rossaedwards/main/vim/fig_scripts
   !pip install -q numpy matplotlib scipy
   ```

2. **Run all**
   ```python
   import sys
   sys.path.insert(0, '/content/rossaedwards/main/vim/fig_scripts')

   from pathlib import Path
   from importlib.util import spec_from_file_location, module_from_spec

   base = Path('/content/rossaedwards/main/vim/fig_scripts')
   out = Path('/content/figures')
   out.mkdir(exist_ok=True)

   for script in sorted(base.glob('fig_*.py')):
       try:
           spec = spec_from_file_location(script.stem, script)
           mod = module_from_spec(spec)
           spec.loader.exec_module(mod)
           mod.run_simulation(out)
           print(f"OK {script.name}")
       except Exception as e:
           print(f"FAIL {script.name}: {e}")
   ```

---

### Option C: Paste `vim_common` as first cell (single-script use)

If you only run one figure at a time and paste its code:

1. **Cell 1 — vim_common as a module**
   ```python
   import importlib.util
   from pathlib import Path

   # After uploading vim_common.py to /content/:
   spec = importlib.util.spec_from_file_location("vim_common", Path("/content/vim_common.py"))
   vim_common = importlib.util.module_from_spec(spec)
   sys.modules["vim_common"] = vim_common
   spec.loader.exec_module(vim_common)
   ```
   This creates a real `vim_common` module so `from vim_common import beta` works in later cells.

2. **Cell 2 — paste figure script** (or use `exec(open(...).read())`)

---

## Quick Reference

| Task              | Command / Action                                      |
|-------------------|--------------------------------------------------------|
| Run all locally   | `python fig_scripts/run_all_local.py`                  |
| Run 5 locally     | `python fig_scripts/run_all_local.py -n 5`            |
| List scripts      | `python fig_scripts/run_all_local.py --list`          |
| Colab: upload zip | Upload `fig_scripts.zip`, unzip, add to `sys.path`    |
| Colab: run one    | `importlib` + `run_simulation(Path('/content/figures'))` |
