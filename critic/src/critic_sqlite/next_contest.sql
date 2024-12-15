WITH 
random_criterion AS (
    SELECT id AS criterion_id 
    FROM criteria
    ORDER BY RANDOM()
    LIMIT 1
),
unevaluated_pairs AS (
    SELECT 
        ec1.entry_id AS entry1_id,
        ec2.entry_id AS entry2_id,
        ABS(e1.elo - e2.elo) AS elo_distance,
        rc.criterion_id
    FROM entry_criteria ec1
    JOIN entry_criteria ec2 
        ON ec1.criterion_id = ec2.criterion_id AND ec1.entry_id < ec2.entry_id
    JOIN entries e1 ON ec1.entry_id = e1.id
    JOIN entries e2 ON ec2.entry_id = e2.id
    CROSS JOIN random_criterion rc
    LEFT JOIN match_history mh ON 
        ((mh.winner_id = ec1.entry_id AND mh.loser_id = ec2.entry_id) OR 
        (mh.winner_id = ec2.entry_id AND mh.loser_id = ec1.entry_id)) 
        AND mh.criterion_id = rc.criterion_id
    WHERE ec1.criterion_id = rc.criterion_id AND mh.id IS NULL
),
next_comparison AS (
    SELECT 
        entry1_id, 
        entry2_id, 
        criterion_id
    FROM unevaluated_pairs
    ORDER BY elo_distance ASC
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
JOIN criteria c ON nc.criterion_id = c.id;
