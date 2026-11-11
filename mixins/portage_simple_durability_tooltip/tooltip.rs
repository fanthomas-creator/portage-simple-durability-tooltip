// Adapted from Infinidoge's ItemStackMixin.java, MIT, copyright 2022 Infinidoge.
fn tooltip(
    game: &crate::game::Game<'_>,
    stack: &horizon_client::items::ItemStack,
    lines: &mut Vec<(horizon_client::text::Text, u32)>,
) {
    use horizon_client::text::Text;
    if !game.native_on("portage_simple_durability_tooltip")
        || game.settings.file.bool("advancedItemTooltips")
        || stack.damage == 0
        || !horizon_client::item_tooltip::TooltipDisplay::for_stack(stack).shows("minecraft:damage")
    {
        return;
    }
    let component = |id| stack.data().and_then(|data| data.other(id));
    if stack.count == 0
        || &*stack.item == "minecraft:air"
        || component("minecraft:unbreakable").is_some()
        || component("!minecraft:max_damage").is_some()
        || component("!minecraft:damage").is_some()
    {
        return;
    }
    let max = match component("minecraft:max_damage") {
        Some(horizon_world::nbt::Tag::Int(value)) => u32::try_from(*value).unwrap_or(0),
        Some(_) => 0,
        None => game
            .items()
            .get(&stack.item)
            .map_or(0, |info| info.max_damage),
    };
    if max == 0 {
        return;
    }
    let text = Text::translate(
        "item.durability",
        &[
            Text::literal(max.saturating_sub(stack.damage).to_string()),
            Text::literal(max.to_string()),
        ],
    );
    // Keep other mods' tooltip contributions and avoid adding the same line twice.
    if !lines.iter().any(|(line, _)| line == &text) {
        lines.push((text, u32::MAX));
    }
}
