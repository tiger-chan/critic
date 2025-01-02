SELECT c.value, e.name, ec.elo FROM entry_criteria ec
JOIN entries e ON e.id == ec.entry_id
JOIN criteria_group c ON c.id == ec.group_id
WHERE ?1 IS NULL OR c.value == ?1
ORDER BY ec.elo DESC, c.value ASC
LIMIT ?2 OFFSET ?3
