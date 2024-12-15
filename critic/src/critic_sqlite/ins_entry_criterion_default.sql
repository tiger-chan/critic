WITH RECURSIVE criterion_ids(id) AS (
    SELECT 1
    UNION ALL
    SELECT id + 1
    FROM criterion_ids
    WHERE id < 11
)
INSERT INTO entry_criteria (entry_id, criterion_id)
SELECT (SELECT id FROM entries WHERE name = ?1), id
FROM criterion_ids;
