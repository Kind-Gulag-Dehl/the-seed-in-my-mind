export interface AuthorInfo {
  author_identity_id: string | null;
  author_identity_title: string | null;
  verification_level: string | null;
  persona_id: string | null;
}

export type RelativeImportanceDirection = "incoming" | "outgoing" | "both";

export interface IdeaSummary {
  idea_id: string;
  idea_type: string;
  is_personal_space_organizer?: boolean;
  speaker_identity_id: string;
  speaker_identity_title: string | null;
  created_event_id: string;
  title: string;
  sentence: string | null;
  derived_universal_rank: string | null;
  ri_in_count?: string | null;
  ri_out_count?: string | null;
  author: AuthorInfo;
}

export interface ConnectionSummary {
  connection_id: string;
  from_idea_id: string;
  to_idea_id: string;
  connection_type: string;
  created_by_event_id: string;
  usage: string | null;
  axis: string | null;
  timeframe: string | null;
  scope: string | null;
  value_representation: string | null;
  certainty_band: string | null;
  weight: string | null;
}

export interface IdeaDetail extends IdeaSummary {
  payload_hash: string;
  incoming_connections: ConnectionSummary[];
  outgoing_connections: ConnectionSummary[];
}

export interface NeighborhoodResponse {
  central_idea: IdeaDetail;
  adjacent_ideas: IdeaSummary[];
  connections: ConnectionSummary[];
  depth_reached: string;
}

export interface SnapshotMetadata {
  snapshot_id: string;
  height: string;
  snapshot_hash: string;
  state_root_hash: string;
  title_sentence_payload_root: string;
  shared_map_commitment: string;
  prev_snapshot_hash: string | null;
  event_count: string;
  approximate_timestamp: string;
  cycle_index: string | null;
  cycle_close_height: string | null;
}

export interface SnapshotLatestResponse {
  snapshot: SnapshotMetadata;
  preview_ideas?: IdeaSummary[];
}

export interface IdeasTopResponse {
  ideas: IdeaSummary[];
  total: string;
  offset: string;
  limit: string;
}

export interface IdeaDetailResponse {
  idea: IdeaDetail;
}

export interface SearchIdeasResponse {
  results: IdeaSummary[];
  total: string;
}

export interface RelativeImportanceConnectionsResponse {
  connections: ConnectionSummary[];
}

export interface SnapshotByHeightResponse {
  snapshot: SnapshotMetadata;
}

export interface CanonicalEventLogEvent {
  event_id: string;
  global_index: string;
  block_height: string;
  block_event_index: string;
  event_type: string;
  authorship_status: string;
  author_identity_id?: string | null;
  speaker_identity_id?: string | null;
  signature_profile?: string | null;
  signature?: string | null;
  public_key_ref?: string | null;
  payload_hash?: string | null;
  payload_binding_mode?: string | null;
  authored_candidate_hash_v0?: string | null;
  publication_profile?: string | null;
}

export interface CanonicalEventLogBlockBand {
  id: string;
  block_height: string;
  start_global_index: string;
  end_global_index: string;
  label: string;
}

export interface CanonicalEventLogCycleBand {
  id: string;
  cycle_index: string;
  start_global_index: string;
  end_global_index: string;
  label: string;
  closure_event_id?: string | null;
}

export interface CanonicalEventLogResponse {
  events: CanonicalEventLogEvent[];
  blocks: CanonicalEventLogBlockBand[];
  cycles: CanonicalEventLogCycleBand[];
}
