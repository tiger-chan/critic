SELECT c.value, t.name, tc.elo FROM title_criteria tc
JOIN titles t ON t.id == tc.title_id
JOIN criteria_group c ON c.id == tc.group_id
WHERE ?1 IS NULL OR c.value == ?1
ORDER BY c.value ASC, tc.elo DESC 
LIMIT ?2 OFFSET ?3
