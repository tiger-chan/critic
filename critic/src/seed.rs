pub(crate) struct SeedEntry {
    pub name: &'static str,
    pub groups: &'static [&'static str],
}

pub(crate) struct SeedCriteria {
    pub name: &'static str,
    pub sub_criteria: &'static [&'static str],
}

const fn e(name: &'static str, groups: &'static [&'static str]) -> SeedEntry {
    SeedEntry { name, groups }
}

const fn c(name: &'static str, sub_criteria: &'static [&'static str]) -> SeedCriteria {
    SeedCriteria { name, sub_criteria }
}

pub(crate) const DEFAULT_CRITERIA: &[SeedCriteria] = &[
    c(
        "General",
        &[
            "Gameplay",
            "Replaybility",
            "Difficulty",
            "Story",
            "World Building",
            "Writing/Voice Acting",
            "Graphics",
            "Art Style",
            "UI/UX",
            "Sound Effects",
            "Music",
        ],
    ),
    c("arpg", &["General"]),
    c("Action Adventure", &["General"]),
    c("jrpg", &["General"]),
    c("mmo", &["General"]),
    c("Platformer", &["General"]),
    c("Open World", &["General"]),
    c("Tactics", &["General"]),
    c("rts", &["General"]),
    c("Third Person Shooter", &["General"]),
    c("roguelike", &["General"]),
];

#[rustfmt::skip]
pub(crate) const ENTRIES: &[SeedEntry] = &[
    e("Legend of Zelda", &["Action Adventure", "arpg"]),
    e("Legend of Zelda 2: The Adventure of Link", &["Action Adventure", "arpg", "Platformer"]),
    e("Legend of Zelda: A Link to the Past", &["Action Adventure", "arpg"]),
    e("Legend of Zelda: Link's Awakening", &["Action Adventure", "arpg"]),
    e("Legend of Zelda: Ocarina of Time", &["Action Adventure", "arpg", "Open World"]),
    e("Legend of Zelda: Majora's Mask", &["Action Adventure", "arpg", "Open World"]),
    e("Legend of Zelda: Oracle of Seasons", &["Action Adventure", "arpg"]),
    e("Legend of Zelda: Oracle of Ages", &["Action Adventure", "arpg"]),
    e("Legend of Zelda: Four Swords", &["Action Adventure", "arpg"]),
    e("Legend of Zelda: The Wind Waker", &["Action Adventure", "arpg", "Open World"]),
    e("Legend of Zelda: The Minish Cap", &["Action Adventure", "arpg"]),
    e("Legend of Zelda: Twilight Princess", &["Action Adventure", "arpg", "Open World"]),
    e("Legend of Zelda: Phantom Hourglass", &["Action Adventure", "arpg"]),
    e("Legend of Zelda: Spirit Tracks", &["Action Adventure", "arpg"]),
    e("Legend of Zelda: Skyward Sword", &["Action Adventure", "arpg"]),
    e("Legend of Zelda: A Link Between Worlds", &["Action Adventure", "arpg"]),
    e("Legend of Zelda: Breath of the Wild", &["Action Adventure", "arpg", "Open World"]),
    e("Legend of Zelda: Tears of the Kingdom", &["Action Adventure", "arpg", "Open World"]),
    e("Legend of Zelda: Echoes of Wisdom", &["Action Adventure", "arpg"]),
    e("Final Fantasy", &["jrpg"]),
    e("Final Fantasy II", &["jrpg"]),
    e("Final Fantasy III", &["jrpg"]),
    e("Final Fantasy IV", &["jrpg"]),
    e("Final Fantasy IV: The After Years", &["jrpg"]),
    e("Final Fantasy V", &["jrpg"]),
    e("Final Fantasy VI", &["jrpg"]),
    e("Final Fantasy VII", &["jrpg"]),
    e("Final Fantasy VII Remake", &["arpg"]),
    e("Final Fantasy VIII", &["jrpg"]),
    e("Final Fantasy IX", &["jrpg"]),
    e("Final Fantasy X", &["jrpg"]),
    e("Final Fantasy X-2", &["jrpg"]),
    e("Final Fantasy XI: Online", &["jrpg", "mmo"]),
    e("Final Fantasy XII", &["jrpg"]),
    e("Final Fantasy XIII", &["jrpg"]),
    e("Final Fantasy XIV: A Realm Reborn", &["jrpg", "mmo"]),
    e("Final Fantasy XV", &["arpg"]),
    e("Final Fantasy XVI", &["arpg"]),
    e("Final Fantasy Tactics", &["Tactics"]),
    e("Final Fantasy Tactics Advance", &["Tactics"]),
    e("Final Fantasy Tactics A2: Grimoire of the Rift", &["Tactics"]),
    e("Final Fantasy Tactics XII: Revenant Wings", &["rts"]),
    e("Dirge of Cerberus: Final Fantasy VII", &["Third Person Shooter"]),
    e("Crisis Core: Final Fantasy VII", &["arpg"]),
    e("Mystery Dungeon: Every Buddy!", &["roguelike"]),
    e("Final Fantasy Crystal Chronicles", &["arpg"]),
    e("Final Fantasy Mystic Quest", &["jrpg"]),
    e("Breath of Fire", &["jrpg"]),
    e("Breath of Fire II", &["jrpg"]),
    e("Breath of Fire III", &["jrpg"]),
    e("Breath of Fire IV", &["jrpg"]),
    e("Dragon Quest", &["jrpg"]),
    e("Dragon Quest II", &["jrpg"]),
    e("Dragon Quest III", &["jrpg"]),
    e("Dragon Quest IV", &["jrpg"]),
    e("Dragon Quest V", &["jrpg"]),
    e("Dragon Quest VI", &["jrpg"]),
    e("Dragon Quest VII", &["jrpg"]),
    e("Dragon Quest VIII", &["jrpg"]),
    e("Dragon Quest IX", &["jrpg"]),
    e("Dragon Quest XI", &["jrpg", "mmo"]),
    e("Diablo", &["arpg"]),
    e("Diablo 2", &["arpg"]),
    e("Diablo 3", &["arpg"]),
    e("Diablo 4", &["arpg"]),
    e("Path of Exile 2", &["arpg"]),
];
