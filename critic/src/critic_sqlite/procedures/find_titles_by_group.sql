SELECT t.id, t.name FROM titles t
JOIN title_criteria tc ON tc.title_id = t.id
WHERE tc.group_id = ?1
ORDER BY t.name ASC
