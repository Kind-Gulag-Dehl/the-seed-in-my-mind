export interface ApiCapabilitiesResponse {
  api_contract_version: string;
  build_revision: string;
  migration_head: string;
  snapshot_format_version: string;
  active_feature_profile: "open_core" | "full";
  supported_canonical_signed_write_kinds: Array<"idea_create" | "connection_create">;
  public_contract_artifact: string;
}

export interface SnapshotBasis {
  snapshot_id: string;
  snapshot_height: string;
  snapshot_hash: string;
  state_root_hash: string;
  title_sentence_payload_root: string;
  shared_map_commitment: string;
  active_rulebook_set_hash: string;
  last_event_id: string;
  event_count: string;
}

export interface AuthorInfo {
  author_identity_id: string | null;
  author_identity_title: string | null;
  verification_level: string | null;
  persona_id: string | null;
}

export type RelativeImportanceDirection = "incoming" | "outgoing" | "both";
export type OrderingProfile = "vine" | "evidence_rail" | "action_rail";
export type VineType = "pathway_vine" | "narrative_vine";
export type OrderingItemRole =
  | "potential_evidence"
  | "actual_evidence"
  | "potential_action"
  | "proposed_action";
export type RepresentationTargetKind = "idea" | "ordering";
export type RepresentationKind = "title" | "description";
export type RepresentationTierLength = "sentence" | "paragraph" | "full";
export type RepresentationTierComplexity =
  | "fundamental"
  | "standard"
  | "advanced"
  | "canonical";

export interface OrderingItem {
  idx: string;
  idea_id: string;
  item_role: OrderingItemRole | null;
  via_connection_id?: string | null;
}

export interface CanonicalOrderingRepresentations {
  title_representation_id?: string | null;
  title_payload_hash?: string | null;
  sentence_representation_id?: string | null;
  sentence_payload_hash?: string | null;
}

export interface CanonicalOrderingSummary {
  ordering_id: string;
  ordering_profile: OrderingProfile;
  vine_type?: VineType | null;
  subject_idea_id: string | null;
}

export interface CanonicalOrderingDetail {
  ordering_id: string;
  ordering_profile: OrderingProfile;
  vine_type?: VineType | null;
  subject_idea_id: string | null;
  author_identity_id: string;
  canonical_representations: CanonicalOrderingRepresentations;
  items: OrderingItem[];
}

export interface CanonicalOrderingResponse {
  basis: SnapshotBasis;
  ordering: CanonicalOrderingDetail;
}

export interface CanonicalOrderingsResponse {
  basis: SnapshotBasis;
  orderings: CanonicalOrderingSummary[];
}

export interface CanonicalRepresentationDetail {
  representation_id: string;
  target_kind: RepresentationTargetKind;
  target_object_id: string;
  representation_kind: RepresentationKind;
  tier_length?: RepresentationTierLength;
  tier_complexity?: RepresentationTierComplexity;
  vocabulary_version_id?: string;
  payload_hash: string;
  payload_text: string | null;
  author_identity_id: string;
  language_locale: string | null;
  provenance: string | null;
  created_event_id: string;
  created_block_height: string;
  created_event_index: string;
}

export interface CanonicalRepresentationResponse {
  basis: SnapshotBasis;
  representation: CanonicalRepresentationDetail;
}

export interface CanonicalRepresentationsResponse {
  basis: SnapshotBasis;
  representations: CanonicalRepresentationDetail[];
  total: string;
  offset: string;
  limit: string;
}

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
  basis: SnapshotBasis;
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
  basis: SnapshotBasis;
  snapshot: SnapshotMetadata;
  preview_ideas?: IdeaSummary[];
}

export interface IdeasTopResponse {
  basis: SnapshotBasis;
  ideas: IdeaSummary[];
  total: string;
  offset: string;
  limit: string;
}

export interface IdeaDetailResponse {
  basis: SnapshotBasis;
  idea: IdeaDetail;
}

export interface SearchIdeasResponse {
  basis: SnapshotBasis;
  results: IdeaSummary[];
  total: string;
}

export interface IdeaBatchResolutionResponse {
  basis: SnapshotBasis;
  ideas: IdeaSummary[];
  missing_idea_ids: string[];
}

export interface ExactMatchIdeasResponse {
  basis: SnapshotBasis;
  field: "title" | "sentence";
  value: string;
  matches: IdeaSummary[];
  truncated: boolean;
  limit: string;
}

export interface RelativeImportanceConnectionsResponse {
  basis: SnapshotBasis;
  connections: ConnectionSummary[];
  truncated: boolean;
  limit: string;
}

export interface SnapshotByHeightResponse {
  basis: SnapshotBasis;
  snapshot: SnapshotMetadata;
}

export interface CoordinateNodeResponse {
  id: string;
  x: number;
  y: number;
  title: string;
  sentence: string | null;
  idea_type: string;
  derived_universal_rank: string | null;
  ri_in_count: string;
  ri_out_count: string;
}

export interface CoordinateMetaResponse {
  spacing: number;
  algo: string;
  relaxed: boolean;
}

export interface CoordinateViewResponse {
  basis: SnapshotBasis;
  mode: "global" | "reference";
  reference_id: string | null;
  coords: CoordinateNodeResponse[];
  meta: CoordinateMetaResponse;
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
