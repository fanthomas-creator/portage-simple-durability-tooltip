"""Lightweight checks of the project contract, credits, assets and deterministic package."""
from pathlib import Path
import hashlib
import json
import re
import struct
import subprocess
import sys
import tomllib
import zipfile

root = Path(__file__).resolve().parents[1]
cargo = tomllib.loads((root / "Cargo.toml").read_text(encoding="utf-8"))
manifests = list((root / "mixins").glob("*/horizon-mixin.toml"))
assert len(manifests) == 1, "Exactly one native bundle is expected"
bundle = manifests[0].parent
manifest = tomllib.loads(manifests[0].read_text(encoding="utf-8"))
compat = json.loads((root / "compatibility.json").read_text())
assert manifest["mod"]["id"] == bundle.name
assert re.fullmatch(r"[a-z0-9_]+", bundle.name)
assert manifest["mod"]["version"] == cargo["package"]["version"]
assert manifest["compatibility"]["horizon_revision"] == compat["horizon_revision"]
assert "Copyright (c) 2022 Infinidoge" in (root / "LICENSE").read_text()
for required in ("README.md", "CONTRIBUTING.md", "SECURITY.md", "CODE_OF_CONDUCT.md", "NOTICE.md", "LICENSE-Horizon", "docs/USAGE.md", "docs/CREDITS.md"):
    assert (root / required).is_file(), required
for path in root.rglob("*.md"):
    if any(part in (".git", "target", "dist") for part in path.relative_to(root).parts):
        continue
    for target in re.findall(r"!?\[[^\]]*\]\(([^)]+)\)", path.read_text(encoding="utf-8")):
        if re.match(r"^[a-zA-Z]+://", target) or target.startswith("#"):
            continue
        assert (path.parent / target.split("#")[0]).is_file(), (path, target)
icon = root / "icon.png"
width, height = struct.unpack(">II", icon.read_bytes()[16:24])
assert width == height and 64 <= width <= 1024 and icon.stat().st_size <= 256 * 1024
for ruleset in (root / ".github/rulesets").glob("*.json"):
    json.loads(ruleset.read_text())
for path in bundle.rglob("*"):
    assert not path.is_symlink(), f"Unexpected bundle symlink: {path}"
    if path.is_file():
        assert path.suffix in (".rs", ".toml", ".md") or path.name.startswith("LICENSE"), path
first = json.loads(subprocess.check_output([sys.executable, str(root / "scripts/package.py")], text=True))
second = json.loads(subprocess.check_output([sys.executable, str(root / "scripts/package.py")], text=True))
assert first["sha256"] == second["sha256"], "Package must be reproducible"
with zipfile.ZipFile(first["file"]) as package:
    metadata = json.loads(package.read("horizon-native.json"))
    assert metadata["id"] == bundle.name
    assert metadata["version"] == cargo["package"]["version"]
    assert package.testzip() is None
    for name in package.namelist():
        assert name == "horizon-native.json" or name.startswith(f"mixins/{bundle.name}/"), name
        assert not name.endswith((".key", ".dpapi", ".exe", ".dll")), name
print(json.dumps({"status": "passed", "version": cargo["package"]["version"], "package_sha256": first["sha256"]}))
