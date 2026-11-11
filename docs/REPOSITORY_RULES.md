# Repository rules

Repository policies and proposed machine-readable rules are kept here for review. Active enforcement is configured separately in GitHub Settings; these files alone do not activate protection.

Assign @fanthomas-creator as CODEOWNER. Import the JSON rulesets in Settings → Rules → Rulesets and verify Active enforcement. The proposed main rule requires a pull request, one code-owner approval, resolved conversations and the checks CI job; stale approvals are dismissed, and force pushes/deletions are blocked. An administrator exception through pull requests supports single-maintainer work; review the diff and test results before using it.

Release tags v* are restricted to the repository administrator and protected against updates, deletion and force pushes. Public visibility permits reading and forking, not direct write access. Do not grant write access merely because someone contributes.

Default workflow permissions should be read-only. Require approval for outside-contributor workflows and keep signing keys out of CI. The supplied workflow checks formatting, isolated logic and package structure, not a complete Horizon build. Configure private vulnerability reporting and remove merged branches when appropriate.

Rules cannot prevent a repository owner from changing administrative settings. The final setup report must distinguish local templates from remotely enforced rules.
