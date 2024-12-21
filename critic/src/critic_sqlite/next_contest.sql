WITH 
unevaluated_pairs AS (
    SELECT 
        ec1.entry_id AS entry1_id,
        ec2.entry_id AS entry2_id,
        c.id as criterion_id,
        ABS(ec1.elo - ec2.elo) AS elo_distance,
        ec1.elo as e1_elo,
        ec2.elo as e2_elo,
        RANDOM() as rng
    FROM entry_criteria ec1
    JOIN entry_criteria ec2 
        ON ec1.group_id = ec2.group_id
        AND ec1.entry_id < ec2.entry_id
    JOIN entries e1 ON ec1.entry_id = e1.id
    JOIN entries e2 ON ec2.entry_id = e2.id
    JOIN criteria c ON c.group_id = ec1.group_id
    LEFT JOIN match_history mh ON 
        (
            (mh.a_id = ec1.entry_id AND mh.b_id = ec2.entry_id) OR
            (mh.a_id = ec2.entry_id AND mh.b_id = ec1.entry_id)
        ) 
        AND mh.criterion_id = c.id
    WHERE mh.id IS NULL
),
next_comparison AS (
    SELECT 
        entry1_id, 
        entry2_id, 
        criterion_id,
        e1_elo,
        e2_elo
    FROM unevaluated_pairs
    ORDER BY elo_distance ASC, rng ASC
    LIMIT 1
)
SELECT 
    e1.id, e1.name, nc.e1_elo,
    e2.id, e2.name, nc.e2_elo,
    c.group_id AS criteria_group,
    c.id AS criterion_id,
    c.value AS criterion_name,
    cg.value AS criteria_group
FROM next_comparison nc
JOIN entries e1 ON nc.entry1_id = e1.id
JOIN entries e2 ON nc.entry2_id = e2.id
JOIN criteria c on nc.criterion_id = c.id
JOIN criteria_group cg ON cg.id = c.group_id;
