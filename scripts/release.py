#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.12"
# dependencies = []
# ///
"""Update the crate version, changelog, and lockfile for a release."""

from __future__ import annotations

import datetime as dt
import re
import subprocess
import sys
from pathlib import Path

PACKAGE_NAME = "char_str"
ROOT = Path(__file__).resolve().parent.parent
CHANGELOG_COMPARE_URL = "https://github.com/astral-sh/char_str/compare/"
UNRELEASED_HEADING = "## [Unreleased]"
UNRELEASED_LINK = re.compile(
    rf"(?m)^\[Unreleased\]: {re.escape(CHANGELOG_COMPARE_URL)}"
    r"(?P<previous_tag>\S+)\.\.\.HEAD$"
)


def run(*args: str, capture_output: bool = False) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        args,
        cwd=ROOT,
        check=True,
        capture_output=capture_output,
        text=True,
    )


def update_manifest(version: str) -> None:
    if not re.fullmatch(r"[0-9A-Za-z.+-]+", version) or not version[0].isdigit():
        raise SystemExit(f"invalid Cargo version: {version!r}")

    manifest = ROOT / "Cargo.toml"
    contents = manifest.read_text()
    package_start = contents.index("[package]")
    package_end = contents.find("\n[", package_start + len("[package]"))
    if package_end == -1:
        package_end = len(contents)

    package = contents[package_start:package_end]
    match = re.search(r'(?m)^version\s*=\s*"([^"]+)"$', package)
    if match is None:
        raise SystemExit("Cargo.toml [package] table has no version")
    if match.group(1) == version:
        raise SystemExit(f"Cargo.toml is already at version {version}")

    package = (
        package[: match.start()] + f'version = "{version}"' + package[match.end() :]
    )
    manifest.write_text(contents[:package_start] + package + contents[package_end:])


def rewrite_changelog(contents: str, version: str, release_date: dt.date) -> str:
    """Move the unreleased changes into a dated release section."""
    if contents.count(UNRELEASED_HEADING) != 1:
        raise SystemExit(
            f"CHANGELOG.md must contain exactly one {UNRELEASED_HEADING!r} heading"
        )

    if f"## [{version}]" in contents or f"[{version}]:" in contents:
        raise SystemExit(f"CHANGELOG.md already contains release {version}")

    link = UNRELEASED_LINK.search(contents)
    if link is None:
        raise SystemExit("CHANGELOG.md has no valid [Unreleased] comparison link")

    release_heading = f"## [{version}] - {release_date.isoformat()}"
    contents = contents.replace(
        UNRELEASED_HEADING,
        f"{UNRELEASED_HEADING}\n\n{release_heading}",
        1,
    )

    previous_tag = link.group("previous_tag")
    release_tag = f"v{version}"
    return UNRELEASED_LINK.sub(
        (
            f"[Unreleased]: {CHANGELOG_COMPARE_URL}{release_tag}...HEAD\n"
            f"[{version}]: "
            f"{CHANGELOG_COMPARE_URL}{previous_tag}...{release_tag}"
        ),
        contents,
        count=1,
    )


def update_changelog(version: str) -> None:
    changelog = ROOT / "CHANGELOG.md"
    changelog.write_text(
        rewrite_changelog(
            changelog.read_text(),
            version,
            dt.datetime.now(dt.UTC).date(),
        )
    )


def main() -> None:
    if len(sys.argv) != 2:
        raise SystemExit(f"usage: {Path(sys.argv[0]).name} <version>")

    version = sys.argv[1]
    update_manifest(version)
    update_changelog(version)

    # Let Cargo validate the version and refresh only the root package entry.
    run("cargo", "update", "-p", PACKAGE_NAME)
    run(
        "cargo",
        "metadata",
        "--locked",
        "--no-deps",
        "--format-version",
        "1",
        capture_output=True,
    )
    run("git", "diff", "--check")
    changed = set(
        run("git", "diff", "--name-only", capture_output=True).stdout.splitlines()
    )
    expected = {"Cargo.toml", "Cargo.lock", "CHANGELOG.md"}
    if changed != expected:
        raise SystemExit(
            f"release preparation changed {sorted(changed)}, expected {sorted(expected)}"
        )


if __name__ == "__main__":
    main()
