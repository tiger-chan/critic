INSERT INTO entry_criteria (entry_id, group_id)
VALUES (
    (SELECT id FROM entries WHERE name = ?1),
    (SELECT id FROM criteria_group WHERE value = ?2)
) ON CONFLICT DO NOTHING
