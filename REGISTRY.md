# Horizon registry form preparation

This file prepares copyable form contents. The mod has not been submitted to the registry. Use this Rust port's source URL below, not the original Fabric repository.

| Field | Value |
| --- | --- |
| Type | Native mod (.hnmod) |
| File | dist/portage-simple-durability-tooltip-0.1.1.hnmod |
| ID | `portage_simple_durability_tooltip` |
| Name | `portage-simple-durability-tooltip` |
| Version | `0.1.1` |
| For Minecraft | `26.3` |
| License | `MIT` |
| Authors | fanthomas-creator (port), Infinidoge (original mod) |
| Categories | Utility, Equipment |
| Tags | durability, tooltip, client-side, quality-of-life |
| Source | https://github.com/fanthomas-creator/portage-simple-durability-tooltip |
| Icon | icon.png (original abstract bar illustration, not upstream logo) |

## Summary

Displays damaged-item durability in tooltips without enabling Advanced Tooltips. Unofficial Rust port for Horizon.

## Description

An unofficial community Rust port of Simple Durability Tooltip by Infinidoge, under the MIT license. Hover over a damaged tool, weapon or armor piece to see remaining and maximum durability in your selected game language, even with Advanced Tooltips disabled. No configuration or additional keybind is needed.

Install the .hnmod through a compatible Horizon native-mod build integration, rebuild the game, enable portage-simple-durability-tooltip in Options → Mods and hover over a damaged item in your inventory. A configured Prism integration normally reads minecraft/horizon-native-mods inside the instance.

Undamaged, unbreakable and hidden-damage items are skipped. Advanced Tooltips keeps its normal display without a duplicate mod line. No network polling, background service or gameplay changes are added.

Requires Minecraft 26.3 and the exact Horizon and horizon-mixins revisions listed in compatibility.json. Latest Nightly compatibility has not been established. Native packages require rebuilding Horizon; they are not Fabric JARs or DLLs.

Original mod and logo: https://modrinth.com/mod/simple-durability-tooltip
Original source: https://github.com/Infinidoge/simple-durability-tooltip
Original creator: Infinidoge. Port maintainer: fanthomas-creator, with Codex assistance. Original copyright and license notices are preserved. The original author does not maintain this Rust port.

## What changed?

Rename the native port, preserve the existing tooltip behavior, and add English documentation, attribution, illustrations, reproducible packaging and release signatures.

## Images

docs/images/tooltip-example.png is a labeled illustration, not a gameplay screenshot. It is 1200 × 600; the optional gallery should identify it as an illustration if used. No in-game screenshot of this release is supplied. The abstract 256 × 256 PNG icon is distinct from the original logo. Once sources are public, use real public image URLs in the registry description instead of local paths.

## Signature and submission

Copy the non-comment line from signing/portage-simple-durability-tooltip.pub into Public key. Upload the .hnmod.minisig into Signature and the exact matching .hnmod into File. Never upload private keys.

Before submitting, publish these Rust sources, check the advertised Horizon target and latest Nightly if claiming it, run the official package/signature check and read the mod policy. The maintainer must personally accept the responsibility declaration. A submission opens a review request; it is not immediate publication.

References: https://playhorizon.dev/docs/publish-a-mod and https://playhorizon.dev/mods/policy

## Public verification key

```text
RWRLiqviZ3I5J7gJBBVbpgKpOHXFBEolu6HF5y5Z43x8jUr8Ooxje7Zl
```
