"""Create the native package reproducibly from this project's sources. Python 3.11+."""
from pathlib import Path
import hashlib
import json
import tomllib
import zipfile

root = Path(__file__).resolve().parents[1]
bundles = list((root / "mixins").glob("*/horizon-mixin.toml"))
if len(bundles) != 1:
    raise SystemExit("Expected exactly one native bundle")
bundle = bundles[0].parent
manifest = tomllib.loads(bundles[0].read_text(encoding="utf-8"))
compat = json.loads((root / "compatibility.json").read_text(encoding="utf-8"))
mod_id, version = manifest["mod"]["id"], manifest["mod"]["version"]
if bundle.name != mod_id or manifest["compatibility"]["horizon_revision"] != compat["horizon_revision"]:
    raise SystemExit("Bundle identity or Horizon revision does not match compatibility.json")
metadata = dict(schema=1, id=mod_id, version=version, game=compat["game"],
                horizon=compat["horizon_revision"], mixins=compat["horizon_mixins_revision"])
# The tooltip hook uses Horizon item/component APIs, so compatibility is pinned to the checked revision.
dist = root / "dist"
dist.mkdir(exist_ok=True)
name = tomllib.loads((root / "Cargo.toml").read_text(encoding="utf-8"))["package"]["name"]
destination = dist / f"{name}-{version}.hnmod"
files = {"horizon-native.json": (json.dumps(metadata, indent=2, sort_keys=True) + "\n").encode()}
for path in sorted(bundle.rglob("*")):
    if not path.is_file():
        continue
    relative = path.relative_to(bundle)
    if path.suffix not in (".rs", ".toml") and not path.name.startswith(("README", "LICENSE", "NOTICE")):
        raise SystemExit(f"Unexpected file in native bundle: {relative}")
    # Canonical UTF-8/LF bytes reproduce the same package from Windows and Linux checkouts.
    content = path.read_text(encoding="utf-8-sig").replace("\r\n", "\n").replace("\r", "\n")
    files[f"mixins/{mod_id}/{relative.as_posix()}"] = content.encode("utf-8")
with zipfile.ZipFile(destination, "w") as archive:
    for filename, data in sorted(files.items()):
        entry = zipfile.ZipInfo(filename, (2026, 1, 1, 0, 0, 0))
        entry.compress_type = zipfile.ZIP_DEFLATED
        entry.external_attr = 0o100644 << 16
        archive.writestr(entry, data)
print(json.dumps(dict(file=str(destination), size=destination.stat().st_size,
                      sha256=hashlib.sha256(destination.read_bytes()).hexdigest()), indent=2))
