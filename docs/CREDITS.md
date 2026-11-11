# Credits and provenance

## Original mod

- Name: Simple Durability Tooltip.
- Author: **Infinidoge**.
- Page and logo: https://modrinth.com/mod/simple-durability-tooltip
- Source: https://github.com/Infinidoge/simple-durability-tooltip
- Revision consulted: `35cad4b42a6be409e92dedb2093b470f09052031`.
- Behavior adapted from `src/main/java/moe/inx/simple_durability_tooltip/mixin/ItemStackMixin.java`.
- License: MIT, copyright (c) 2022 Infinidoge. The full original notice is retained in ../LICENSE and inside the native bundle.

The upstream Java mixin adds the translated durability line for damaged items when Advanced Tooltips is off. This port reimplements that behavior in Rust using Horizon APIs; a Java source file cannot simply be pasted into Rust. Infinidoge is credited as the original creator, not as the maintainer of this port.

## Horizon

Native-hook, item-component and translation APIs: **Ludovic Marie and the Horizon contributors**, MIT. Target revision: `fdc87aecdbd116bf1e08adcd062fcbaea8c2e2ca`; horizon-mixins revision: `e59da9094f0c2fdda6c627fe9eff22dec6a906f0`. The Horizon copyright notice is in ../LICENSE-Horizon and the native bundle.

## Community port and images

Maintainer: **fanthomas-creator**. Prepared with Codex assistance. The abstract durability-bar icon and tooltip behavior diagram are original project illustrations, licensed with this project under MIT. They do not use extracted game textures or Minecraft's font and do not depict a tested gameplay session.

The original logo can be obtained from the original mod page or the upstream source tree at `src/main/resources/assets/simple_durability_tooltip/icon.png`. It has not been relabeled as artwork made by this port. No original-mod logo or game assets are bundled here.

The exact source references and illustration provenance are recorded in ../SOURCES.json. Not an official Minecraft product; not affiliated with Mojang or Microsoft.
