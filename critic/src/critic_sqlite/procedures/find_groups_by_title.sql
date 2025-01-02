SELECT cg.id, cg.value FROM criteria_group cg
JOIN entry_criteria ec ON ec.group_id = cg.id
WHERE ec.entry_id = ?1
ORDER BY cg.value ASC
