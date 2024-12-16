WITH 
unevaluated_pairs AS (
    SELECT 
        ec1.entry_id AS entry1_id,
        ec2.entry_id AS entry2_id,
        ec1.criterion_id,
        ABS(e1.elo - e2.elo) AS elo_distance,
        RANDOM() as rng
    FROM entry_criteria ec1
    JOIN entry_criteria ec2 
        ON ec1.criterion_id = ec2.criterion_id AND ec1.entry_id < ec2.entry_id
    JOIN entries e1 ON ec1.entry_id = e1.id
    JOIN entries e2 ON ec2.entry_id = e2.id
    LEFT JOIN match_history mh ON 
        ((mh.a_id = ec1.entry_id AND mh.b_id = ec2.entry_id) OR 
        (mh.a_id = ec2.entry_id AND mh.b_id = ec1.entry_id)) 
        AND mh.criterion_id = ec1.criterion_id
    WHERE mh.id IS NULL
),
next_comparison AS (
    SELECT 
        entry1_id, 
        entry2_id, 
        criterion_id
    FROM unevaluated_pairs
    ORDER BY elo_distance ASC, rng ASC
    LIMIT 1
)
SELECT 
    e1.id, e1.name, e1.elo,
    e2.id, e2.name, e2.elo,
    c.id AS criterion_id,
    c.value AS criterion_name
FROM next_comparison nc
JOIN entries e1 ON nc.entry1_id = e1.id
JOIN entries e2 ON nc.entry2_id = e2.id
join criteria c on nc.criterion_id = c.id;
