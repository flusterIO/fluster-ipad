import sys
from pathlib import Path
import os

feature_name = sys.argv[1]

root = Path(__file__).parent.parent


typescript_dir = root / "apps" / "fluster_desktop" / "src" / "features" / feature_name
rust_dir = (
    root / "apps" / "fluster_desktop" / "src-tauri" / "src" / "features" / feature_name
)

rust_features_mod = (
    root / "apps" / "fluster_desktop" / "src-tauri" / "src" / "features" / "mod.rs"
)

if not typescript_dir.exists():
    os.mkdir(typescript_dir)
if not (typescript_dir / "presentation").exists():
    os.mkdir(typescript_dir / "presentation")
if not (typescript_dir / "data").exists():
    os.mkdir(typescript_dir / "data")
if not rust_dir.exists():
    os.mkdir(rust_dir)
if not (rust_dir / "data").exists():
    os.mkdir(rust_dir / "data")
if not (rust_dir / "methods").exists():
    os.mkdir(rust_dir / "methods")

rust_new_mod = rust_dir / "mod.rs"

rust_new_mod.write_text("pub mod data;\npub mod methods;")

t = "" if not rust_features_mod.exists() else f"{rust_features_mod.read_text()}\n"

if not t.__contains__(f"pub mod {feature_name};"):
    rust_features_mod.write_text(f"""{t}pub mod {feature_name};""")
