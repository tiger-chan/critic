SELECT cg.id, cg.value FROM criteria_group cg
JOIN title_criteria tc ON tc.group_id = cg.id
WHERE tc.title_id = ?1
ORDER BY cg.value ASC
