BEGIN;

UPDATE events
SET payload_json = jsonb_set(
    jsonb_set(payload_json, '{title}', to_jsonb('the currently existing individual human'::text), true),
    '{sentence}', to_jsonb('the currently existing individual human'::text), true
)
WHERE event_id = '00000000-0000-7000-8000-000000000101';

UPDATE events
SET payload_json = jsonb_set(
    jsonb_set(payload_json, '{title}', to_jsonb('all life, consciousness, and intelligence in the universe through time'::text), true),
    '{sentence}', to_jsonb('all life, consciousness, and intelligence in the universe through time'::text), true
)
WHERE event_id = '00000000-0000-7000-8000-000000000102';

UPDATE events
SET payload_json = jsonb_set(
    jsonb_set(payload_json, '{title}', to_jsonb('all life, consciousness, and intelligence in the universe through time'::text), true),
    '{sentence}', to_jsonb('all life, consciousness, and intelligence in the universe through time'::text), true
)
WHERE event_id = '00000000-0000-7000-8000-000000000103';

COMMIT;
