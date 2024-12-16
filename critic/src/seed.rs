pub(crate) struct SeedEntry {
    pub name: &'static str,
    pub criteria: &'static [&'static str],
}

const fn e(name: &'static str, criteria: &'static [&'static str]) -> SeedEntry {
    SeedEntry { name, criteria }
}

pub(crate) const DEFAULT_CRITERIA: &[&str] = &[
    "gameplay",
    "replaybility",
    "difficulty",
    "story",
    "world-building",
    "writing/voice-acting",
    "graphics",
    "art-style",
    "ux",
    "sound-effects",
    "music",
];

#[rustfmt::skip]
pub(crate) const ENTRIES: &[SeedEntry] = &[
    e("Legend of Zelda", &["action-adventure", "arpg"]),
    e("Legend of Zelda 2: The Adventure of Link", &["action-adventure", "arpg", "platformer"]),
    e("Legend of Zelda: A Link to the Past", &["action-adventure", "arpg"]),
    e("Legend of Zelda: Link's Awakening", &["action-adventure", "arpg"]),
    e("Legend of Zelda: Ocarina of Time", &["action-adventure", "arpg", "open-world"]),
    e("Legend of Zelda: Majora's Mask", &["action-adventure", "arpg", "open-world"]),
    e("Legend of Zelda: Oracle of Seasons", &["action-adventure", "arpg"]),
    e("Legend of Zelda: Oracle of Ages", &["action-adventure", "arpg"]),
    e("Legend of Zelda: Four Swords", &["action-adventure", "arpg"]),
    e("Legend of Zelda: The Wind Waker", &["action-adventure", "arpg", "open-world"]),
    e("Legend of Zelda: The Minish Cap", &["action-adventure", "arpg"]),
    e("Legend of Zelda: Twilight Princess", &["action-adventure", "arpg", "open-world"]),
    e("Legend of Zelda: Phantom Hourglass", &["action-adventure", "arpg"]),
    e("Legend of Zelda: Spirit Tracks", &["action-adventure", "arpg"]),
    e("Legend of Zelda: Skyward Sword", &["action-adventure", "arpg"]),
    e("Legend of Zelda: A Link Between Worlds", &["action-adventure", "arpg"]),
    e("Legend of Zelda: Breath of the Wild", &["action-adventure", "arpg", "open-world"]),
    e("Legend of Zelda: Tears of the Kingdom", &["action-adventure", "arpg", "open-world"]),
    e("Legend of Zelda: Echoes of Wisdom", &["action-adventure", "arpg"]),
    e("Final Fantasy", &["jrpg"]),
    e("Final Fantasy II", &["jrpg"]),
    e("Final Fantasy III", &["jrpg"]),
    e("Final Fantasy IV", &["jrpg"]),
    e("Final Fantasy V", &["jrpg"]),
    e("Final Fantasy VI", &["jrpg"]),
    e("Final Fantasy VII", &["jrpg"]),
    e("Final Fantasy VIII", &["jrpg"]),
    e("Final Fantasy IX", &["jrpg"]),
    e("Final Fantasy X", &["jrpg"]),
    e("Final Fantasy XI", &["jrpg", "mmo"]),
    e("Final Fantasy XII", &["jrpg"]),
    e("Final Fantasy XIII", &["jrpg"]),
    e("Final Fantasy XIV", &["jrpg", "mmo"]),
    e("Final Fantasy XV", &["jrpg"]),
    e("Final Fantasy XVI", &["jrpg"]),
];
