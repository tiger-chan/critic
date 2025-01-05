INSERT INTO title_criteria (title_id, group_id)
SELECT t.id, ?1
FROM titles t
WHERE NOT EXISTS (
    SELECT 1
    FROM title_criteria tc
    WHERE t.id = tc.title_id
        AND tc.group_id == ?1
);
