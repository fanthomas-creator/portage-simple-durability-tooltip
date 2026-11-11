//! Isolated behavior tests of the real helper. These API doubles are not Horizon API validation.
#![allow(dead_code)]
extern crate self as horizon_client;
extern crate self as horizon_world;

pub mod nbt {
    #[derive(Clone)]
    pub enum Tag {
        Int(i32),
        Other,
    }
}
pub mod text {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Text(pub String, pub Vec<Text>);
    impl Text {
        pub fn literal(s: String) -> Self {
            Self(s, vec![])
        }
        pub fn translate(key: &str, args: &[Self]) -> Self {
            Self(key.into(), args.to_vec())
        }
    }
}
pub mod items {
    use std::collections::HashMap;
    #[derive(Default)]
    pub struct Data(pub HashMap<String, crate::nbt::Tag>);
    impl Data {
        pub fn other(&self, id: &str) -> Option<&crate::nbt::Tag> {
            self.0.get(id)
        }
    }
    pub struct ItemStack {
        pub count: u32,
        pub item: String,
        pub damage: u32,
        pub components: Data,
        pub shows_damage: bool,
    }
    impl ItemStack {
        pub fn data(&self) -> Option<&Data> {
            Some(&self.components)
        }
    }
    pub struct Info {
        pub max_damage: u32,
    }
    pub struct Registry(pub Option<Info>);
    impl Registry {
        pub fn get(&self, _: &str) -> Option<&Info> {
            self.0.as_ref()
        }
    }
}
pub mod item_tooltip {
    pub struct TooltipDisplay(bool);
    impl TooltipDisplay {
        pub fn for_stack(stack: &crate::items::ItemStack) -> Self {
            Self(stack.shows_damage)
        }
        pub fn shows(&self, key: &str) -> bool {
            assert_eq!(key, "minecraft:damage");
            self.0
        }
    }
}
pub mod game {
    pub struct File(pub bool);
    impl File {
        pub fn bool(&self, key: &str) -> bool {
            assert_eq!(key, "advancedItemTooltips");
            self.0
        }
    }
    pub struct Settings {
        pub file: File,
    }
    pub struct Game<'a> {
        pub settings: Settings,
        pub enabled: bool,
        pub registry: &'a crate::items::Registry,
    }
    impl Game<'_> {
        pub fn native_on(&self, id: &str) -> bool {
            assert_eq!(id, "portage_simple_durability_tooltip");
            self.enabled
        }
        pub fn items(&self) -> &crate::items::Registry {
            self.registry
        }
    }
}

include!("../mixins/portage_simple_durability_tooltip/tooltip.rs");

#[cfg(test)]
mod tests {
    use super::*;
    fn stack() -> items::ItemStack {
        items::ItemStack {
            count: 1,
            item: "test:tool".into(),
            damage: 244,
            components: items::Data::default(),
            shows_damage: true,
        }
    }
    fn run(stack: &items::ItemStack, enabled: bool, advanced: bool) -> Vec<(text::Text, u32)> {
        let registry = items::Registry(Some(items::Info { max_damage: 250 }));
        let game = game::Game {
            settings: game::Settings {
                file: game::File(advanced),
            },
            enabled,
            registry: &registry,
        };
        let mut lines = vec![(text::Text::literal("Existing lore".into()), 0xff00ff00)];
        tooltip(&game, stack, &mut lines);
        lines
    }
    #[test]
    fn damaged_item_uses_translation_and_preserves_lore() {
        let lines = run(&stack(), true, false);
        assert_eq!(lines.len(), 2);
        assert_eq!(
            lines[0],
            (text::Text::literal("Existing lore".into()), 0xff00ff00)
        );
        assert_eq!(
            lines[1],
            (
                text::Text::translate(
                    "item.durability",
                    &[
                        text::Text::literal("6".into()),
                        text::Text::literal("250".into())
                    ]
                ),
                u32::MAX
            )
        );
    }
    #[test]
    fn off_advanced_undamaged_and_hidden_are_skipped() {
        assert_eq!(run(&stack(), false, false).len(), 1);
        assert_eq!(run(&stack(), true, true).len(), 1);
        let mut s = stack();
        s.damage = 0;
        assert_eq!(run(&s, true, false).len(), 1);
        s.damage = 244;
        s.shows_damage = false;
        assert_eq!(run(&s, true, false).len(), 1);
    }
    #[test]
    fn empty_air_unbreakable_and_removed_components_are_skipped() {
        let mut s = stack();
        s.count = 0;
        assert_eq!(run(&s, true, false).len(), 1);
        s = stack();
        s.item = "minecraft:air".into();
        assert_eq!(run(&s, true, false).len(), 1);
        for key in [
            "minecraft:unbreakable",
            "!minecraft:max_damage",
            "!minecraft:damage",
        ] {
            s = stack();
            s.components.0.insert(key.into(), nbt::Tag::Other);
            assert_eq!(run(&s, true, false).len(), 1, "{key}");
        }
    }
    #[test]
    fn component_override_is_used_and_remaining_is_saturated() {
        let mut s = stack();
        s.components
            .0
            .insert("minecraft:max_damage".into(), nbt::Tag::Int(500));
        assert_eq!(run(&s, true, false)[1].0.1[0].0, "256");
        s.components
            .0
            .insert("minecraft:max_damage".into(), nbt::Tag::Int(100));
        assert_eq!(run(&s, true, false)[1].0.1[0].0, "0");
        for value in [nbt::Tag::Int(0), nbt::Tag::Int(-1), nbt::Tag::Other] {
            s.components.0.insert("minecraft:max_damage".into(), value);
            assert_eq!(run(&s, true, false).len(), 1);
        }
    }
    #[test]
    fn missing_or_non_damageable_registry_entry_is_skipped() {
        for info in [None, Some(items::Info { max_damage: 0 })] {
            let registry = items::Registry(info);
            let game = game::Game {
                settings: game::Settings {
                    file: game::File(false),
                },
                enabled: true,
                registry: &registry,
            };
            let mut lines = vec![];
            tooltip(&game, &stack(), &mut lines);
            assert!(lines.is_empty());
        }
    }
    #[test]
    fn repeated_hook_does_not_duplicate_the_line() {
        let s = stack();
        let registry = items::Registry(Some(items::Info { max_damage: 250 }));
        let game = game::Game {
            settings: game::Settings {
                file: game::File(false),
            },
            enabled: true,
            registry: &registry,
        };
        let mut lines = vec![];
        tooltip(&game, &s, &mut lines);
        tooltip(&game, &s, &mut lines);
        assert_eq!(lines.len(), 1);
    }
}
