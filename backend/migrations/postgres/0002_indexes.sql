BEGIN;

CREATE INDEX events_event_id_idx ON events(event_id);
CREATE INDEX ideas_created_order_idx ON ideas(created_block_height, created_event_index);
CREATE INDEX ideas_idea_type_idx ON ideas(idea_type);
CREATE INDEX connections_from_idea_id_idx ON connections(from_idea_id);
CREATE INDEX connections_to_idea_id_idx ON connections(to_idea_id);
CREATE INDEX connections_connection_type_idx ON connections(connection_type);
CREATE INDEX events_payload_json_gin_idx ON events USING gin (payload_json);

COMMIT;
