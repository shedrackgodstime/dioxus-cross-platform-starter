#!/usr/bin/env python3
"""Check the workspace's documented crate dependency boundaries."""

import pathlib
import json
import re
import subprocess
import sys
import tomllib

ROOT = pathlib.Path(__file__).resolve().parents[1]
METADATA = json.loads(
    subprocess.run(
        ["cargo", "metadata", "--no-deps", "--format-version", "1"],
        cwd=ROOT,
        check=True,
        capture_output=True,
        text=True,
    ).stdout
)
ROOT_MANIFEST = tomllib.loads((ROOT / "Cargo.toml").read_text(encoding="utf-8"))
EXPECTED = {
    "starter-core": "crates/core",
    "starter-api": "crates/api",
    "starter-server": "crates/server",
    "starter-ui": "crates/ui",
    "starter-web": "crates/web",
    "starter-web-client": "crates/web-client",
    "starter-desktop": "crates/desktop",
    "starter-mobile": "crates/mobile",
}


def dependencies(crate: str) -> set[str]:
    package = next(item for item in METADATA["packages"] if item["name"] == crate)
    return {dependency["name"] for dependency in package["dependencies"]}


def check_manifest_locations(errors: list[str]) -> None:
    for crate, relative_path in EXPECTED.items():
        manifest = ROOT / relative_path / "Cargo.toml"
        if not manifest.is_file():
            errors.append(f"{crate}: expected manifest at {relative_path}/Cargo.toml")
    actual = {path.parent.relative_to(ROOT).as_posix() for path in (ROOT / "crates").glob("*/Cargo.toml")}
    expected = set(EXPECTED.values())
    if actual != expected:
        errors.append(f"workspace crate layout differs: expected {sorted(expected)}, got {sorted(actual)}")
    if "package" in ROOT_MANIFEST:
        errors.append("root Cargo.toml must remain a workspace manifest, not a package")
    workspace = ROOT_MANIFEST.get("workspace", {})
    if workspace.get("package", {}).get("edition") != "2024":
        errors.append("workspace edition must stay 2024")
    if workspace.get("package", {}).get("rust-version") != "1.97.1":
        errors.append("workspace rust-version must stay 1.97.1")
    if workspace.get("dependencies", {}).get("dioxus", {}).get("version") != "0.7.10":
        errors.append("Dioxus workspace dependency must stay on 0.7.10")
    for crate, relative_path in EXPECTED.items():
        manifest = tomllib.loads((ROOT / relative_path / "Cargo.toml").read_text(encoding="utf-8"))
        if manifest.get("package", {}).get("edition") != {"workspace": True}:
            errors.append(f"{crate}: inherit workspace edition")
        if manifest.get("package", {}).get("rust-version") != {"workspace": True}:
            errors.append(f"{crate}: inherit workspace rust-version")


def check_forbidden_dependencies(errors: list[str]) -> None:
    rules = {
        "starter-core": {"dioxus", "web-sys", "jni", "ndk-context", "directories"},
        "starter-ui": {"starter-server", "libsql", "bcrypt", "jsonwebtoken"},
        "starter-web": {"starter-mobile", "starter-desktop"},
        "starter-desktop": {"starter-mobile", "starter-web"},
        "starter-mobile": {"starter-desktop", "starter-web"},
    }
    for crate, forbidden in rules.items():
        for name in sorted(dependencies(crate) & forbidden):
            errors.append(f"{crate}: forbidden direct dependency `{name}`")
    if dependencies("starter-core") != {"serde", "serde_json", "thiserror"}:
        errors.append("starter-core: dependencies must remain serde, serde_json, and thiserror only")


def check_routes(errors: list[str]) -> None:
    route_file = ROOT / "crates/ui/src/routes.rs"
    source = route_file.read_text(encoding="utf-8")
    enum_match = re.search(r"pub\s+enum\s+Route\s*\{(.*?)\n\}", source, re.S)
    if enum_match is None:
        errors.append("starter-ui: Route enum not found")
    elif "#[cfg" in enum_match.group(1):
        errors.append("starter-ui: Route variants must not be conditionally compiled")


def main() -> int:
    errors: list[str] = []
    check_manifest_locations(errors)
    check_forbidden_dependencies(errors)
    check_routes(errors)
    if errors:
        print("Workspace boundary checks failed:", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1
    print("Workspace crate boundaries are consistent.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
