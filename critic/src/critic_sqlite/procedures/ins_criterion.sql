INSERT INTO criteria (group_id, value)
VALUES(?1, ?2) ON CONFLICT DO NOTHING;
