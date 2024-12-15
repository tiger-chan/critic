BEGIN;
INSERT INTO entries (name)
VALUES
("Legend of Zelda"),
("Legend of Zelda 2: The Adventure of Link"),
("Legend of Zelda: A Link to the Past"),
("Legend of Zelda: Link's Awakening"),
("Legend of Zelda: Ocarina of Time"),
("Legend of Zelda: Majora's Mask"),
("Legend of Zelda: Oracle of Seasons"),
("Legend of Zelda: Oracle of Ages"),
("Legend of Zelda: Four Swords"),
("Legend of Zelda: The Wind Waker"),
("Legend of Zelda: The Minish Cap"),
("Legend of Zelda: Twilight Princess"),
("Legend of Zelda: Phantom Hourglass"),
("Legend of Zelda: Spirit Tracks"),
("Legend of Zelda: Skyward Sword"),
("Legend of Zelda: A Link Between Worlds"),
("Legend of Zelda: Breath of the Wild"),
("Legend of Zelda: Tears of the Kingdom"),
("Legend of Zelda: Echoes of Wisdom");


INSERT INTO criteria (value)
VALUES
("gameplay"),
("replability"),
("difficulty"),
("story"),
("world-building"),
("writing/voice-acting"),
("graphics"),
("art-style"),
("ux"),
("sound-effects"),
("music"),
("action-adventure"),
("arpg"),
("open-world"),
("platformer");

-- Insert universal criteria for all games.
WITH RECURSIVE game_ids(game_id) AS (
    SELECT 1
    UNION ALL
    SELECT game_id + 1 FROM game_ids WHERE game_id < 18
)
INSERT INTO entry_criteria (entry_id, criterion_id)
SELECT game_id, criterion_id
FROM game_ids
CROSS JOIN (
    SELECT 1 AS criterion_id UNION ALL -- gameplay 
    SELECT 2 UNION ALL -- replayability
    SELECT 3 UNION ALL -- difficulty
    SELECT 4 UNION ALL -- story
    SELECT 5 UNION ALL -- world-building
    SELECT 6 UNION ALL -- writing/voice-acting
    SELECT 7 UNION ALL -- graphics
    SELECT 8 UNION ALL -- art-style
    SELECT 9 UNION ALL -- UX
    SELECT 10 UNION ALL -- sound-effects
    SELECT 11 -- music
) AS criteria;

-- Add genre and unique criteria for specific games.
INSERT OR IGNORE INTO entry_criteria (entry_id, criterion_id) VALUES
-- Legend of Zelda (Action-Adventure Core)
(1, 12), -- action-adventure
(1, 13), -- arpg

-- Legend of Zelda 2: The Adventure of Link (ARPG, Platformer)
(2, 12), -- action-adventure
(2, 13), -- arpg
(2, 15), -- platformer

-- Legend of Zelda: A Link to the Past (Action-Adventure)
(3, 12), -- action-adventure
(3, 13), -- arpg

-- Legend of Zelda: Link's Awakening (Action-Adventure)
(4, 12), -- action-adventure
(4, 13), -- arpg

-- Legend of Zelda: Ocarina of Time (Action-Adventure, Open-World)
(5, 12), -- action-adventure
(5, 13), -- arpg
(5, 14), -- open-world

-- Legend of Zelda: Majora's Mask (Action-Adventure, Open-World)
(6, 12), -- action-adventure
(6, 13), -- arpg
(6, 14), -- open-world

-- Legend of Zelda: Oracle of Seasons (Action-Adventure)
(7, 12), -- action-adventure
(7, 13), -- arpg

-- Legend of Zelda: Oracle of Ages (Action-Adventure)
(8, 12), -- action-adventure
(8, 13), -- arpg

-- Legend of Zelda: Four Swords (Action-Adventure, Multiplayer)
(9, 12), -- action-adventure
(9, 13), -- arpg

-- Legend of Zelda: The Wind Waker (Action-Adventure, Open-World)
(10, 12), -- action-adventure
(10, 13), -- arpg
(10, 14), -- open-world

-- Legend of Zelda: The Minish Cap (Action-Adventure)
(11, 12), -- action-adventure
(11, 13), -- arpg

-- Legend of Zelda: Twilight Princess (Action-Adventure, Open-World)
(12, 12), -- action-adventure
(12, 13), -- arpg
(12, 14), -- open-world

-- Legend of Zelda: Phantom Hourglass (Action-Adventure)
(13, 12), -- action-adventure
(13, 13), -- arpg

-- Legend of Zelda: Spirit Tracks (Action-Adventure)
(14, 12), -- action-adventure
(14, 13), -- arpg

-- Legend of Zelda: Skyward Sword (Action-Adventure)
(15, 12), -- action-adventure
(15, 13), -- arpg

-- Legend of Zelda: A Link Between Worlds (Action-Adventure)
(16, 12), -- action-adventure
(16, 13), -- arpg

-- Legend of Zelda: Breath of the Wild (Action-Adventure, Open-World)
(17, 12), -- action-adventure
(17, 13), -- arpg
(17, 14), -- open-world

-- Legend of Zelda: Tears of the Kingdom (Action-Adventure, Open-World)
(18, 12), -- action-adventure
(18, 13), -- arpg
(18, 14); -- open-world
COMMIT;
