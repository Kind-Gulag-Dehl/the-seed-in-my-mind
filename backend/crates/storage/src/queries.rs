pub const GET_IDEA: &str = r#"
WITH visible AS (
  SELECT
    i.idea_id,
    i.idea_type,
    i.is_personal_space_organizer,
    i.speaker_identity_id,
    speaker_ident.title AS speaker_identity_title,
    i.created_event_id,
    i.created_block_height,
    i.created_event_index,
    i.is_identity_idea,
    i.underlying_identity_id,
    e.payload_json->>'title' AS title,
    e.payload_json->>'sentence' AS sentence,
    e.payload_json->>'payload_hash' AS payload_hash
  FROM ideas i
  LEFT JOIN events e ON e.event_id = i.created_event_id
  LEFT JOIN identities_s0 speaker_ident ON speaker_ident.identity_id = i.speaker_identity_id
  WHERE i.created_block_height <= $2
),
ranked AS (
  SELECT
    i.idea_id,
    ROW_NUMBER() OVER (
      ORDER BY i.created_block_height ASC, i.created_event_index ASC
    ) AS derived_universal_rank
  FROM ideas i
  WHERE i.created_block_height <= $2
    AND i.is_personal_space_organizer = false
),
ri_counts AS (
  SELECT
    counts.idea_id,
    SUM(counts.ri_in_count)::bigint AS ri_in_count,
    SUM(counts.ri_out_count)::bigint AS ri_out_count
  FROM (
    SELECT
      c.to_idea_id AS idea_id,
      COUNT(*)::bigint AS ri_in_count,
      0::bigint AS ri_out_count
    FROM connections c
    WHERE c.created_block_height <= $2
      AND c.connection_type = 'relative_importance'
    GROUP BY c.to_idea_id
    UNION ALL
    SELECT
      c.from_idea_id AS idea_id,
      0::bigint AS ri_in_count,
      COUNT(*)::bigint AS ri_out_count
    FROM connections c
    WHERE c.created_block_height <= $2
      AND c.connection_type = 'relative_importance'
    GROUP BY c.from_idea_id
  ) counts
  GROUP BY counts.idea_id
)
SELECT
  v.idea_id,
  v.idea_type,
  v.is_personal_space_organizer,
  v.speaker_identity_id,
  v.speaker_identity_title,
  v.created_event_id,
  v.created_block_height,
  v.created_event_index,
  v.is_identity_idea,
  v.underlying_identity_id,
  v.title,
  v.sentence,
  v.payload_hash,
  r.derived_universal_rank,
  COALESCE(rc.ri_in_count, 0)::bigint AS ri_in_count,
  COALESCE(rc.ri_out_count, 0)::bigint AS ri_out_count
FROM visible v
LEFT JOIN ranked r ON r.idea_id = v.idea_id
LEFT JOIN ri_counts rc ON rc.idea_id = v.idea_id
WHERE v.idea_id = $1
"#;

pub const LIST_IDEAS_TOP: &str = r#"
WITH ranked AS (
  SELECT
    i.idea_id,
    ROW_NUMBER() OVER (
      ORDER BY i.created_block_height ASC, i.created_event_index ASC
    ) AS derived_universal_rank
  FROM ideas i
  WHERE i.created_block_height <= $1
    AND i.is_personal_space_organizer = false
),
ri_counts AS (
  SELECT
    counts.idea_id,
    SUM(counts.ri_in_count)::bigint AS ri_in_count,
    SUM(counts.ri_out_count)::bigint AS ri_out_count
  FROM (
    SELECT
      c.to_idea_id AS idea_id,
      COUNT(*)::bigint AS ri_in_count,
      0::bigint AS ri_out_count
    FROM connections c
    WHERE c.created_block_height <= $1
      AND c.connection_type = 'relative_importance'
    GROUP BY c.to_idea_id
    UNION ALL
    SELECT
      c.from_idea_id AS idea_id,
      0::bigint AS ri_in_count,
      COUNT(*)::bigint AS ri_out_count
    FROM connections c
    WHERE c.created_block_height <= $1
      AND c.connection_type = 'relative_importance'
    GROUP BY c.from_idea_id
  ) counts
  GROUP BY counts.idea_id
)
SELECT
  i.idea_id,
  i.idea_type,
  i.is_personal_space_organizer,
  i.speaker_identity_id,
  speaker_ident.title AS speaker_identity_title,
  i.created_event_id,
  i.created_block_height,
  i.created_event_index,
  e.payload_json->>'title' AS title,
  e.payload_json->>'sentence' AS sentence,
  ranked.derived_universal_rank,
  COALESCE(rc.ri_in_count, 0)::bigint AS ri_in_count,
  COALESCE(rc.ri_out_count, 0)::bigint AS ri_out_count
FROM ranked
JOIN ideas i ON i.idea_id = ranked.idea_id
LEFT JOIN events e ON e.event_id = i.created_event_id
LEFT JOIN identities_s0 speaker_ident ON speaker_ident.identity_id = i.speaker_identity_id
LEFT JOIN ri_counts rc ON rc.idea_id = i.idea_id
ORDER BY ranked.derived_universal_rank ASC
LIMIT $2 OFFSET $3
"#;

pub const LIST_IDEAS_TOP_DESC: &str = r#"
WITH ranked AS (
  SELECT
    i.idea_id,
    ROW_NUMBER() OVER (
      ORDER BY i.created_block_height ASC, i.created_event_index ASC
    ) AS derived_universal_rank
  FROM ideas i
  WHERE i.created_block_height <= $1
    AND i.is_personal_space_organizer = false
),
ri_counts AS (
  SELECT
    counts.idea_id,
    SUM(counts.ri_in_count)::bigint AS ri_in_count,
    SUM(counts.ri_out_count)::bigint AS ri_out_count
  FROM (
    SELECT
      c.to_idea_id AS idea_id,
      COUNT(*)::bigint AS ri_in_count,
      0::bigint AS ri_out_count
    FROM connections c
    WHERE c.created_block_height <= $1
      AND c.connection_type = 'relative_importance'
    GROUP BY c.to_idea_id
    UNION ALL
    SELECT
      c.from_idea_id AS idea_id,
      0::bigint AS ri_in_count,
      COUNT(*)::bigint AS ri_out_count
    FROM connections c
    WHERE c.created_block_height <= $1
      AND c.connection_type = 'relative_importance'
    GROUP BY c.from_idea_id
  ) counts
  GROUP BY counts.idea_id
)
SELECT
  i.idea_id,
  i.idea_type,
  i.is_personal_space_organizer,
  i.speaker_identity_id,
  speaker_ident.title AS speaker_identity_title,
  i.created_event_id,
  i.created_block_height,
  i.created_event_index,
  e.payload_json->>'title' AS title,
  e.payload_json->>'sentence' AS sentence,
  ranked.derived_universal_rank,
  COALESCE(rc.ri_in_count, 0)::bigint AS ri_in_count,
  COALESCE(rc.ri_out_count, 0)::bigint AS ri_out_count
FROM ranked
JOIN ideas i ON i.idea_id = ranked.idea_id
LEFT JOIN events e ON e.event_id = i.created_event_id
LEFT JOIN identities_s0 speaker_ident ON speaker_ident.identity_id = i.speaker_identity_id
LEFT JOIN ri_counts rc ON rc.idea_id = i.idea_id
ORDER BY ranked.derived_universal_rank DESC
LIMIT $2 OFFSET $3
"#;

pub const COUNT_IDEAS: &str = r#"
SELECT COUNT(*) AS total
FROM ideas
WHERE created_block_height <= $1
  AND is_personal_space_organizer = false
"#;

pub const SEARCH_IDEAS: &str = r#"
WITH visible AS (
  SELECT
    i.idea_id,
    i.idea_type,
    i.is_personal_space_organizer,
    i.speaker_identity_id,
    speaker_ident.title AS speaker_identity_title,
    i.created_event_id,
    i.created_block_height,
    i.created_event_index,
    e.payload_json->>'title' AS title,
    e.payload_json->>'sentence' AS sentence
  FROM ideas i
  LEFT JOIN events e ON e.event_id = i.created_event_id
  LEFT JOIN identities_s0 speaker_ident ON speaker_ident.identity_id = i.speaker_identity_id
  WHERE i.created_block_height <= $1
),
ranked AS (
  SELECT
    i.idea_id,
    ROW_NUMBER() OVER (
      ORDER BY i.created_block_height ASC, i.created_event_index ASC
    ) AS derived_universal_rank
  FROM ideas i
  WHERE i.created_block_height <= $1
    AND i.is_personal_space_organizer = false
),
ri_counts AS (
  SELECT
    counts.idea_id,
    SUM(counts.ri_in_count)::bigint AS ri_in_count,
    SUM(counts.ri_out_count)::bigint AS ri_out_count
  FROM (
    SELECT
      c.to_idea_id AS idea_id,
      COUNT(*)::bigint AS ri_in_count,
      0::bigint AS ri_out_count
    FROM connections c
    WHERE c.created_block_height <= $1
      AND c.connection_type = 'relative_importance'
    GROUP BY c.to_idea_id
    UNION ALL
    SELECT
      c.from_idea_id AS idea_id,
      0::bigint AS ri_in_count,
      COUNT(*)::bigint AS ri_out_count
    FROM connections c
    WHERE c.created_block_height <= $1
      AND c.connection_type = 'relative_importance'
    GROUP BY c.from_idea_id
  ) counts
  GROUP BY counts.idea_id
)
SELECT
  v.idea_id,
  v.idea_type,
  v.is_personal_space_organizer,
  v.speaker_identity_id,
  v.speaker_identity_title,
  v.created_event_id,
  v.created_block_height,
  v.created_event_index,
  v.title,
  v.sentence,
  r.derived_universal_rank,
  COALESCE(rc.ri_in_count, 0)::bigint AS ri_in_count,
  COALESCE(rc.ri_out_count, 0)::bigint AS ri_out_count
FROM visible v
LEFT JOIN ranked r ON r.idea_id = v.idea_id
LEFT JOIN ri_counts rc ON rc.idea_id = v.idea_id
WHERE (
    v.title ILIKE $2
    OR v.sentence ILIKE $2
  )
ORDER BY v.created_block_height ASC, v.created_event_index ASC
LIMIT $3 OFFSET $4
"#;

pub const COUNT_SEARCH_IDEAS: &str = r#"
SELECT COUNT(*) AS total
FROM ideas i
LEFT JOIN events e ON e.event_id = i.created_event_id
WHERE i.created_block_height <= $1
  AND (
    (e.payload_json->>'title') ILIKE $2
    OR (e.payload_json->>'sentence') ILIKE $2
  )
"#;

pub const LIST_IDEAS_BY_IDS: &str = r#"
WITH visible AS (
  SELECT
    i.idea_id,
    i.idea_type,
    i.is_personal_space_organizer,
    i.speaker_identity_id,
    speaker_ident.title AS speaker_identity_title,
    i.created_event_id,
    i.created_block_height,
    i.created_event_index,
    e.payload_json->>'title' AS title,
    e.payload_json->>'sentence' AS sentence
  FROM ideas i
  LEFT JOIN events e ON e.event_id = i.created_event_id
  LEFT JOIN identities_s0 speaker_ident ON speaker_ident.identity_id = i.speaker_identity_id
  WHERE i.created_block_height <= $2
),
ranked AS (
  SELECT
    i.idea_id,
    ROW_NUMBER() OVER (
      ORDER BY i.created_block_height ASC, i.created_event_index ASC
    ) AS derived_universal_rank
  FROM ideas i
  WHERE i.created_block_height <= $2
    AND i.is_personal_space_organizer = false
),
ri_counts AS (
  SELECT
    counts.idea_id,
    SUM(counts.ri_in_count)::bigint AS ri_in_count,
    SUM(counts.ri_out_count)::bigint AS ri_out_count
  FROM (
    SELECT
      c.to_idea_id AS idea_id,
      COUNT(*)::bigint AS ri_in_count,
      0::bigint AS ri_out_count
    FROM connections c
    WHERE c.created_block_height <= $2
      AND c.connection_type = 'relative_importance'
    GROUP BY c.to_idea_id
    UNION ALL
    SELECT
      c.from_idea_id AS idea_id,
      0::bigint AS ri_in_count,
      COUNT(*)::bigint AS ri_out_count
    FROM connections c
    WHERE c.created_block_height <= $2
      AND c.connection_type = 'relative_importance'
    GROUP BY c.from_idea_id
  ) counts
  GROUP BY counts.idea_id
)
SELECT
  v.idea_id,
  v.idea_type,
  v.is_personal_space_organizer,
  v.speaker_identity_id,
  v.speaker_identity_title,
  v.created_event_id,
  v.created_block_height,
  v.created_event_index,
  v.title,
  v.sentence,
  r.derived_universal_rank,
  COALESCE(rc.ri_in_count, 0)::bigint AS ri_in_count,
  COALESCE(rc.ri_out_count, 0)::bigint AS ri_out_count
FROM visible v
LEFT JOIN ranked r ON r.idea_id = v.idea_id
LEFT JOIN ri_counts rc ON rc.idea_id = v.idea_id
WHERE v.idea_id = ANY($1)
ORDER BY v.created_block_height ASC, v.created_event_index ASC
"#;

pub const LIST_IDEAS_BY_SPEAKER: &str = r#"
WITH visible AS (
  SELECT
    i.idea_id,
    i.idea_type,
    i.is_personal_space_organizer,
    i.speaker_identity_id,
    speaker_ident.title AS speaker_identity_title,
    i.created_event_id,
    i.created_block_height,
    i.created_event_index,
    e.payload_json->>'title' AS title,
    e.payload_json->>'sentence' AS sentence
  FROM ideas i
  LEFT JOIN events e ON e.event_id = i.created_event_id
  LEFT JOIN identities_s0 speaker_ident ON speaker_ident.identity_id = i.speaker_identity_id
  WHERE i.created_block_height <= $1
    AND i.speaker_identity_id = $2
),
ranked AS (
  SELECT
    i.idea_id,
    ROW_NUMBER() OVER (
      ORDER BY i.created_block_height ASC, i.created_event_index ASC
    ) AS derived_universal_rank
  FROM ideas i
  WHERE i.created_block_height <= $1
    AND i.speaker_identity_id = $2
    AND i.is_personal_space_organizer = false
),
ri_counts AS (
  SELECT
    counts.idea_id,
    SUM(counts.ri_in_count)::bigint AS ri_in_count,
    SUM(counts.ri_out_count)::bigint AS ri_out_count
  FROM (
    SELECT
      c.to_idea_id AS idea_id,
      COUNT(*)::bigint AS ri_in_count,
      0::bigint AS ri_out_count
    FROM connections c
    WHERE c.created_block_height <= $1
      AND c.connection_type = 'relative_importance'
    GROUP BY c.to_idea_id
    UNION ALL
    SELECT
      c.from_idea_id AS idea_id,
      0::bigint AS ri_in_count,
      COUNT(*)::bigint AS ri_out_count
    FROM connections c
    WHERE c.created_block_height <= $1
      AND c.connection_type = 'relative_importance'
    GROUP BY c.from_idea_id
  ) counts
  GROUP BY counts.idea_id
)
SELECT
  v.idea_id,
  v.idea_type,
  v.is_personal_space_organizer,
  v.speaker_identity_id,
  v.speaker_identity_title,
  v.created_event_id,
  v.created_block_height,
  v.created_event_index,
  v.title,
  v.sentence,
  r.derived_universal_rank,
  COALESCE(rc.ri_in_count, 0)::bigint AS ri_in_count,
  COALESCE(rc.ri_out_count, 0)::bigint AS ri_out_count
FROM visible v
LEFT JOIN ranked r ON r.idea_id = v.idea_id
LEFT JOIN ri_counts rc ON rc.idea_id = v.idea_id
ORDER BY v.created_block_height ASC, v.created_event_index ASC
LIMIT $3 OFFSET $4
"#;

pub const LIST_CONNECTIONS_FOR_IDEA: &str = r#"
SELECT
  connection_id,
  from_idea_id,
  to_idea_id,
  connection_type,
  usage,
  axis,
  timeframe,
  scope,
  created_by_event_id,
  created_block_height,
  created_event_index
FROM connections
WHERE created_block_height <= $1
  AND (from_idea_id = $2 OR to_idea_id = $2)
ORDER BY created_block_height ASC, created_event_index ASC
"#;

pub const LIST_CONNECTIONS_FOR_IDEAS: &str = r#"
SELECT
  connection_id,
  from_idea_id,
  to_idea_id,
  connection_type,
  usage,
  axis,
  timeframe,
  scope,
  created_by_event_id,
  created_block_height,
  created_event_index
FROM connections
WHERE created_block_height <= $1
  AND (from_idea_id = ANY($2) OR to_idea_id = ANY($2))
ORDER BY created_block_height ASC, created_event_index ASC
"#;

pub const GET_CANONICAL_RAIL: &str = r#"
SELECT
  r.rail_id,
  r.rail_kind,
  r.vine_type,
  r.speaker_identity_id AS author_identity_id,
  r.title_representation_id,
  r.sentence_representation_id,
  title_rep.payload_hash AS title_payload_hash,
  sentence_rep.payload_hash AS sentence_payload_hash
FROM rails r
LEFT JOIN representations title_rep ON title_rep.representation_id = r.title_representation_id
LEFT JOIN representations sentence_rep ON sentence_rep.representation_id = r.sentence_representation_id
WHERE r.rail_id = $1
  AND r.created_block_height <= $2
"#;

pub const LIST_CANONICAL_RAIL_ITEMS: &str = r#"
SELECT
  ri.idx,
  ri.idea_id,
  ri.via_connection_id
FROM rail_items ri
JOIN rails r ON r.rail_id = ri.rail_id
WHERE ri.rail_id = $1
  AND r.created_block_height <= $2
ORDER BY ri.idx ASC
"#;

pub const LIST_CANONICAL_RAILS_FOR_IDEA: &str = r#"
SELECT
  r.rail_id,
  r.rail_kind,
  r.vine_type
FROM rails r
WHERE r.created_block_height <= $2
  AND EXISTS (
    SELECT 1
    FROM rail_items ri
    WHERE ri.rail_id = r.rail_id
      AND ri.idea_id = $1
  )
ORDER BY r.created_block_height ASC, r.created_event_index ASC, r.rail_id ASC
"#;
