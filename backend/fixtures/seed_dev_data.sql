--
-- PostgreSQL database dump
--

\restrict LJV2NsYxEipi9W4Ulc4FJlca1JLHmQLNldemdh2NnEeiXz2yqMNp3ffpGj7nR5q

-- Dumped from database version 18.1
-- Dumped by pg_dump version 18.1

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Data for Name: blocks; Type: TABLE DATA; Schema: public; Owner: seed_app
--

INSERT INTO public.blocks (block_height, block_hash, prev_block_hash, created_at) VALUES (0, 'aa', NULL, '2026-01-28 09:36:03.399285-05');
INSERT INTO public.blocks (block_height, block_hash, prev_block_hash, created_at) VALUES (1, '01aa', 'aa', '2026-01-28 20:01:05.423493-05');


--
-- Data for Name: events; Type: TABLE DATA; Schema: public; Owner: seed_app
--

INSERT INTO public.events (block_height, event_index, event_id, event_type, speaker_identity_id, payload_json, signature, ingested_at) VALUES (0, 0, '00000000-0000-7000-8000-000000000001', 'genesis', NULL, '{"note": "genesis fixture"}', NULL, '2026-01-28 09:36:03.399285-05');
INSERT INTO public.events (block_height, event_index, event_id, event_type, speaker_identity_id, payload_json, signature, ingested_at) VALUES (0, 1, '00000000-0000-7000-8000-000000000002', 'noop', NULL, '{"note": "second fixture event"}', NULL, '2026-01-28 09:36:03.399285-05');
INSERT INTO public.events (block_height, event_index, event_id, event_type, speaker_identity_id, payload_json, signature, ingested_at) VALUES (1, 0, '00000000-0000-7000-8000-000000000101', 'idea_create', '00000000-0000-7000-8000-00000000a001', '{"note": "idea 1", "title": "the currently existing individual human", "sentence": "the currently existing individual human", "payload_hash": "6dd6d8eac41307e5baf8759c5fe4ba382b06d5f4c2ca3876bb5e9ee257e62c9d"}', NULL, '2026-01-28 20:01:05.423493-05');
INSERT INTO public.events (block_height, event_index, event_id, event_type, speaker_identity_id, payload_json, signature, ingested_at) VALUES (1, 1, '00000000-0000-7000-8000-000000000102', 'idea_create', '00000000-0000-7000-8000-00000000a001', '{"note": "idea 2", "title": "all life, consciousness, and intelligence in the universe through time", "sentence": "all life, consciousness, and intelligence in the universe through time", "payload_hash": "45515b2fa8cdd69db97cb06729c113effc07f5caed72dfce79cf3a3e8cc46b77"}', NULL, '2026-01-28 20:01:05.423493-05');
INSERT INTO public.events (block_height, event_index, event_id, event_type, speaker_identity_id, payload_json, signature, ingested_at) VALUES (1, 2, '00000000-0000-7000-8000-000000000103', 'idea_create', '00000000-0000-7000-8000-00000000a002', '{"note": "idea 3", "title": "all life, consciousness, and intelligence in the universe through time", "sentence": "all life, consciousness, and intelligence in the universe through time", "payload_hash": "45515b2fa8cdd69db97cb06729c113effc07f5caed72dfce79cf3a3e8cc46b77"}', NULL, '2026-01-28 20:01:05.423493-05');


--
-- Data for Name: ideas; Type: TABLE DATA; Schema: public; Owner: seed_app
--

INSERT INTO public.ideas (idea_id, idea_type, speaker_identity_id, created_block_height, created_event_index, created_event_id) VALUES ('00000000-0000-7000-8000-00000000b001', 'truth_claim', '00000000-0000-7000-8000-00000000a001', 1, 0, '00000000-0000-7000-8000-000000000101');
INSERT INTO public.ideas (idea_id, idea_type, speaker_identity_id, created_block_height, created_event_index, created_event_id) VALUES ('00000000-0000-7000-8000-00000000b002', 'conceptual_idea', '00000000-0000-7000-8000-00000000a001', 1, 1, '00000000-0000-7000-8000-000000000102');
INSERT INTO public.ideas (idea_id, idea_type, speaker_identity_id, created_block_height, created_event_index, created_event_id) VALUES ('00000000-0000-7000-8000-00000000b003', 'actionable_idea', '00000000-0000-7000-8000-00000000a002', 1, 2, '00000000-0000-7000-8000-000000000103');


--
-- Data for Name: connections; Type: TABLE DATA; Schema: public; Owner: seed_app
--



--
-- PostgreSQL database dump complete
--

\unrestrict LJV2NsYxEipi9W4Ulc4FJlca1JLHmQLNldemdh2NnEeiXz2yqMNp3ffpGj7nR5q

