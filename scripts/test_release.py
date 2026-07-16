import datetime as dt
import unittest

from release import rewrite_changelog


class RewriteChangelogTest(unittest.TestCase):
    def test_moves_unreleased_changes_into_release(self) -> None:
        changelog = """\
# Changelog

## [Unreleased]

### Added

- A change.

## [0.0.1] - 2026-07-15

- Initial release.

[Unreleased]: https://github.com/astral-sh/char_str/compare/0.0.1...HEAD
[0.0.1]: https://github.com/astral-sh/char_str/releases/tag/0.0.1
"""

        rewritten = rewrite_changelog(
            changelog,
            "0.0.2",
            dt.date(2026, 7, 16),
        )

        self.assertEqual(
            rewritten,
            """\
# Changelog

## [Unreleased]

## [0.0.2] - 2026-07-16

### Added

- A change.

## [0.0.1] - 2026-07-15

- Initial release.

[Unreleased]: https://github.com/astral-sh/char_str/compare/v0.0.2...HEAD
[0.0.2]: https://github.com/astral-sh/char_str/compare/0.0.1...v0.0.2
[0.0.1]: https://github.com/astral-sh/char_str/releases/tag/0.0.1
""",
        )

    def test_requires_unreleased_comparison_link(self) -> None:
        with self.assertRaisesRegex(SystemExit, "comparison link"):
            rewrite_changelog(
                "## [Unreleased]\n",
                "0.0.2",
                dt.date(2026, 7, 16),
            )


if __name__ == "__main__":
    unittest.main()
