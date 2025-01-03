INSERT INTO title_criteria (title_id, group_id)
VALUES (
    (SELECT id FROM titles WHERE name = ?1),
    (SELECT id FROM criteria_group WHERE value = ?2)
) ON CONFLICT DO NOTHING
