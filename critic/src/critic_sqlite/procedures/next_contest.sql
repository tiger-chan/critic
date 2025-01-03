WITH 
unevaluated_pairs AS (
    SELECT 
        tc1.title_id AS title1_id,
        tc2.title_id AS title2_id,
        c.id as criterion_id,
        ABS(tc1.elo - tc2.elo) AS elo_distance,
        tc1.elo as t1_elo,
        tc2.elo as t2_elo,
        RANDOM() as rng
    FROM title_criteria tc1
    JOIN title_criteria tc2 
        ON tc1.group_id = tc2.group_id
        AND tc1.title_id < tc2.title_id
    JOIN titles t1 ON tc1.title_id = t1.id
    JOIN titles t2 ON tc2.title_id = t2.id
    JOIN criteria c ON c.group_id = tc1.group_id
    LEFT JOIN match_history mh ON 
        (
            (mh.a_id = tc1.title_id AND mh.b_id = tc2.title_id) OR
            (mh.a_id = tc2.title_id AND mh.b_id = tc1.title_id)
        ) 
        AND mh.criterion_id = c.id
    WHERE mh.id IS NULL
),
next_comparison AS (
    SELECT 
        title1_id, 
        title2_id, 
        criterion_id,
        t1_elo,
        t2_elo
    FROM unevaluated_pairs
    ORDER BY elo_distance ASC, rng ASC
    LIMIT 1
)
SELECT 
    t1.id, t1.name, nc.t1_elo,
    t2.id, t2.name, nc.t2_elo,
    c.group_id AS criteria_group,
    c.id AS criterion_id,
    c.value AS criterion_name,
    cg.value AS criteria_group
FROM next_comparison nc
JOIN titles t1 ON nc.title1_id = t1.id
JOIN titles t2 ON nc.title2_id = t2.id
JOIN criteria c on nc.criterion_id = c.id
JOIN criteria_group cg ON cg.id = c.group_id;
