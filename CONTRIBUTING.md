# Contributing

Keep this port small and focused on the original damaged-item tooltip behavior. Discuss a change before adding new features. Do not add telemetry, hidden network access, background polling, unrelated game modifications or bundled game resources.

Retain the original Infinidoge MIT notice and author credits, credit any adapted Horizon code, and document the source revision and license of externally adapted code. Disclose AI assistance and review generated changes.

Submit changes through pull requests for @fanthomas-creator to review. Include the problem, resulting player behavior, compatibility impact and validation. Run cargo fmt --check, cargo test --locked, python scripts/check_project.py and python scripts/package.py. Any hook/API change also needs a real Horizon build and in-game evidence on every advertised target; the mock-based harness is not that evidence.

Never commit private signing keys, encrypted private keys, account tokens, personal logs or Minecraft assets. Keep release signing local and preserve the published public key between releases. Read SECURITY.md, CODE_OF_CONDUCT.md and docs/REPOSITORY_RULES.md.

The .github templates describe the contribution and CI process. Ruleset JSON files alone do not enforce protection: enforcement is configured separately in GitHub Settings.
