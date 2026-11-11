# Release and signature procedure

Only **@fanthomas-creator** publishes project releases. The public verification key is stored in `signing/portage-simple-durability-tooltip.pub`. The private signing key must never be committed, uploaded to a release or entered in the Horizon registry form.

## Prepare a release

1. Review and merge the source changes through the protected branch process.
2. Assign a new version in both `Cargo.toml` and `horizon-mixin.toml`. Do not replace an already-published version.
3. Run `cargo fmt --check`, `cargo test --locked` and `python scripts/check_project.py`.
4. Build and test integration on each Horizon revision you claim to support. The lightweight CI is not a whole-game compatibility test.
5. Run `python scripts/package.py` and `cargo horizon-mod check dist/<package>.hnmod`.
6. Sign the final package bytes and verify the signature before tagging or uploading.

## Sign using the Windows-protected local key

The initial private key is stored outside the repository, encrypted with Windows DPAPI for the Windows user who generated it. The signing helper decrypts it only into a short-lived file in that private directory, then removes the temporary file. It does not send the key to GitHub or to CI.

```powershell
.\scripts\Sign-Release.ps1 `
  -EncryptedKeyPath "PATH\OUTSIDE\THE\REPOSITORY\portage-simple-durability-tooltip.key.dpapi" `
  -MinisignPath "PATH\TO\minisign.exe"
```

Keep a protected backup and a recovery plan for this key. A DPAPI-encrypted file is tied to the Windows account and its encryption material; copying only the encrypted file to an unrelated account does not make it usable there. Use the same project key for later registry updates. Coordinate any key change with registry maintainers.

If you manage your own password-encrypted minisign key instead, use the normal `minisign -S -s PATH -m PACKAGE` workflow locally. Do not regenerate or replace this project's published key as part of an ordinary release.

## Verify and publish

```powershell
minisign -V -p signing/portage-simple-durability-tooltip.pub -m dist/portage-simple-durability-tooltip-0.1.1.hnmod
```

The `.minisig` must be beside the package, or supplied with `-x`. Also validate it with the official Horizon tool:

```powershell
cargo horizon-mod check dist/portage-simple-durability-tooltip-0.1.1.hnmod --key "PUBLIC_KEY_LINE" --signature dist/portage-simple-durability-tooltip-0.1.1.hnmod.minisig
```

Publish a release from the reviewed commit with the tag `v<version>`. Attach the `.hnmod`, its `.minisig`, `SHA256SUMS` and the public `.pub` file. The source repository remains public for review. Never attach the `.key`, `.key.dpapi`, account tokens, player data or game resources.

For the Horizon registry, paste the public key's final non-comment line into **Public key** and upload the `.minisig` into **Signature**. Upload the matching `.hnmod` into the package field. Changing even one byte of the package invalidates its signature: rebuilds must reproduce the same hash or be signed again before publication.

## Current publication status

Public source: https://github.com/fanthomas-creator/portage-simple-durability-tooltip. Verify remote rules and CI before publishing a signed release. Registry submission is a separate maintainer action and has not been performed.
