import React, { useEffect, useMemo, useState } from "react";

export interface AuthorInfo {
  author_identity_id: string | null;
  author_identity_title: string | null;
  verification_level: string | null;
  persona_id: string | null;
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

export interface IdeaSummary {
  idea_id: string;
  idea_type: string;
  speaker_identity_id: string;
  speaker_identity_title: string | null;
  created_event_id: string;
  title: string;
  sentence: string | null;
  derived_universal_rank: string | null;
  author: AuthorInfo;
}

export interface ConnectionSummary {
  connection_id: string;
  from_idea_id: string;
  to_idea_id: string;
  connection_type: string;
  usage: string | null;
  axis: string | null;
  timeframe: string | null;
  scope: string | null;
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

const apiBaseUrl = ((import.meta.env.VITE_API_BASE_URL as string | undefined) ?? "").replace(/\/+$/, "");

const apiUrl = (path: string): string => `${apiBaseUrl}${path.startsWith("/") ? path : `/${path}`}`;

export const normalizeIdeas = (ideas: IdeaSummary[]): IdeaSummary[] => {
  const seen = new Set<string>();
  const normalized: IdeaSummary[] = [];

  for (const idea of ideas) {
    if (!idea.idea_id || seen.has(idea.idea_id)) {
      continue;
    }
    seen.add(idea.idea_id);
    normalized.push(idea);
  }

  return normalized;
};

export const shortHash = (value: string | null | undefined): string => {
  if (!value) {
    return "unknown";
  }
  if (value.length <= 18) {
    return value;
  }
  return `${value.slice(0, 10)}...${value.slice(-8)}`;
};

const fetchJson = async <T,>(path: string): Promise<T> => {
  const response = await fetch(apiUrl(path), {
    method: "GET",
    headers: { "Content-Type": "application/json" }
  });
  if (!response.ok) {
    throw new Error(`request failed: ${response.status}`);
  }
  return (await response.json()) as T;
};

const cardStyle: React.CSSProperties = {
  background: "#ffffff",
  border: "1px solid #d4d4d8",
  borderRadius: 14,
  padding: 16,
  boxShadow: "0 12px 30px rgba(15, 23, 42, 0.06)"
};

const statLabelStyle: React.CSSProperties = {
  color: "#64748b",
  fontSize: 12,
  letterSpacing: "0.04em",
  textTransform: "uppercase"
};

const statValueStyle: React.CSSProperties = {
  color: "#0f172a",
  fontSize: 14,
  fontWeight: 600
};

const connectionSummary = (connection: ConnectionSummary): string => {
  if (connection.connection_type !== "relative_importance") {
    return connection.connection_type;
  }
  return [connection.axis, connection.timeframe, connection.scope].filter(Boolean).join(" / ");
};

export const ReferenceApp: React.FC = () => {
  const [snapshot, setSnapshot] = useState<SnapshotMetadata | null>(null);
  const [ideas, setIdeas] = useState<IdeaSummary[]>([]);
  const [selectedIdeaId, setSelectedIdeaId] = useState<string | null>(null);
  const [selectedIdea, setSelectedIdea] = useState<IdeaDetail | null>(null);
  const [selectedNeighborhood, setSelectedNeighborhood] = useState<NeighborhoodResponse | null>(null);
  const [loading, setLoading] = useState(true);
  const [detailLoading, setDetailLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;

    const load = async () => {
      setLoading(true);
      setError(null);

      try {
        const [snapshotResponse, ideasResponse] = await Promise.all([
          fetchJson<{ snapshot: SnapshotMetadata }>("/api/v0/snapshot/latest?include_preview=true"),
          fetchJson<{ ideas: IdeaSummary[] }>("/api/v0/ideas/top?limit=50&offset=0&order=asc")
        ]);

        if (cancelled) {
          return;
        }

        const nextIdeas = normalizeIdeas(ideasResponse.ideas);
        setSnapshot(snapshotResponse.snapshot);
        setIdeas(nextIdeas);
        setSelectedIdeaId((previous) => previous ?? nextIdeas[0]?.idea_id ?? null);
      } catch (loadError) {
        if (!cancelled) {
          setError(loadError instanceof Error ? loadError.message : "failed to load canonical state");
        }
      } finally {
        if (!cancelled) {
          setLoading(false);
        }
      }
    };

    load().catch(() => undefined);
    return () => {
      cancelled = true;
    };
  }, []);

  useEffect(() => {
    if (!selectedIdeaId) {
      setSelectedIdea(null);
      setSelectedNeighborhood(null);
      return;
    }

    let cancelled = false;

    const loadDetail = async () => {
      setDetailLoading(true);

      try {
        const [detailResponse, neighborhoodResponse] = await Promise.all([
          fetchJson<{ idea: IdeaDetail }>(`/api/v0/idea/${selectedIdeaId}`),
          fetchJson<NeighborhoodResponse>(`/api/v0/idea/${selectedIdeaId}/neighborhood?depth=1&limit_per_hop=12`)
        ]);

        if (cancelled) {
          return;
        }

        setSelectedIdea(detailResponse.idea);
        setSelectedNeighborhood(neighborhoodResponse);
      } catch (loadError) {
        if (!cancelled) {
          setError(loadError instanceof Error ? loadError.message : "failed to load selected idea");
        }
      } finally {
        if (!cancelled) {
          setDetailLoading(false);
        }
      }
    };

    loadDetail().catch(() => undefined);
    return () => {
      cancelled = true;
    };
  }, [selectedIdeaId]);

  const selectedIdeaSummary = useMemo(
    () => ideas.find((idea) => idea.idea_id === selectedIdeaId) ?? null,
    [ideas, selectedIdeaId]
  );

  const proofItems = [
    "The viewer reads only from the public read-only API.",
    "The basis snapshot commitment is visible and stable for this page load.",
    "The selected idea detail and neighborhood come from deterministic replay output.",
    "No private overlays or product-specific modules are required for this surface."
  ];

  return (
    <main
      style={{
        minHeight: "100vh",
        background: "linear-gradient(180deg, #f8fafc 0%, #eef2ff 45%, #f8fafc 100%)",
        color: "#0f172a",
        fontFamily: "\"Segoe UI\", sans-serif"
      }}
    >
      <div style={{ maxWidth: 1360, margin: "0 auto", padding: "24px 20px 40px" }}>
        <header style={{ marginBottom: 20 }}>
          <p style={{ margin: 0, color: "#0f766e", fontWeight: 700, letterSpacing: "0.08em", textTransform: "uppercase", fontSize: 12 }}>
            Open core reviewer surface
          </p>
          <h1 style={{ margin: "8px 0 10px", fontSize: 34, lineHeight: 1.05 }}>
            Verified canonical state, visible without the product layer.
          </h1>
          <p style={{ margin: 0, maxWidth: 860, color: "#334155", fontSize: 15 }}>
            This reference viewer is intentionally narrow. It proves that the public open core can ingest canonical data,
            replay it deterministically, commit it into snapshots, and serve stable read-only views that downstream
            interfaces can inspect.
          </p>
        </header>

        <section style={{ display: "grid", gap: 16, gridTemplateColumns: "1.6fr 1fr", marginBottom: 16 }}>
          <div style={cardStyle}>
            <h2 style={{ marginTop: 0, marginBottom: 12, fontSize: 18 }}>Snapshot proof</h2>
            <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(180px, 1fr))", gap: 14 }}>
              <div>
                <div style={statLabelStyle}>Snapshot height</div>
                <div style={statValueStyle}>{snapshot?.height ?? "unknown"}</div>
              </div>
              <div>
                <div style={statLabelStyle}>Event count</div>
                <div style={statValueStyle}>{snapshot?.event_count ?? "unknown"}</div>
              </div>
              <div>
                <div style={statLabelStyle}>Snapshot hash</div>
                <div style={statValueStyle}>{shortHash(snapshot?.snapshot_hash)}</div>
              </div>
              <div>
                <div style={statLabelStyle}>Shared map commitment</div>
                <div style={statValueStyle}>{shortHash(snapshot?.shared_map_commitment)}</div>
              </div>
              <div>
                <div style={statLabelStyle}>State root hash</div>
                <div style={statValueStyle}>{shortHash(snapshot?.state_root_hash)}</div>
              </div>
              <div>
                <div style={statLabelStyle}>Cycle index</div>
                <div style={statValueStyle}>{snapshot?.cycle_index ?? "n/a"}</div>
              </div>
            </div>
          </div>

          <div style={cardStyle}>
            <h2 style={{ marginTop: 0, marginBottom: 12, fontSize: 18 }}>What this page proves</h2>
            <ul style={{ margin: 0, paddingLeft: 18, display: "grid", gap: 8, color: "#334155", fontSize: 14 }}>
              {proofItems.map((item) => (
                <li key={item}>{item}</li>
              ))}
            </ul>
          </div>
        </section>

        {loading ? <p style={{ marginTop: 0 }}>Loading canonical state...</p> : null}
        {error ? <p style={{ color: "#b91c1c" }}>{error}</p> : null}

        <section style={{ display: "grid", gap: 16, gridTemplateColumns: "320px minmax(0, 1fr)", alignItems: "start" }}>
          <aside style={{ ...cardStyle, padding: 0, overflow: "hidden" }}>
            <div style={{ padding: 16, borderBottom: "1px solid #e2e8f0" }}>
              <h2 style={{ margin: 0, fontSize: 18 }}>Top canonical ideas</h2>
              <p style={{ margin: "6px 0 0", color: "#475569", fontSize: 13 }}>
                Stable list surface from the verified basis snapshot.
              </p>
            </div>
            <ul style={{ listStyle: "none", margin: 0, padding: 10, display: "grid", gap: 8, maxHeight: 720, overflowY: "auto" }}>
              {ideas.map((idea) => (
                <li key={idea.idea_id}>
                  <button
                    type="button"
                    onClick={() => setSelectedIdeaId(idea.idea_id)}
                    style={{
                      width: "100%",
                      textAlign: "left",
                      border: selectedIdeaId === idea.idea_id ? "1px solid #0f766e" : "1px solid #d4d4d8",
                      background: selectedIdeaId === idea.idea_id ? "#ecfeff" : "#ffffff",
                      borderRadius: 12,
                      padding: 12,
                      cursor: "pointer"
                    }}
                  >
                    <div style={{ fontWeight: 700, marginBottom: 4 }}>{idea.title || idea.idea_id}</div>
                    <div style={{ color: "#334155", fontSize: 12, marginBottom: 6 }}>{idea.sentence ?? "(no sentence provided)"}</div>
                    <div style={{ display: "flex", justifyContent: "space-between", gap: 8, color: "#64748b", fontSize: 11 }}>
                      <span>rank {idea.derived_universal_rank ?? "n/a"}</span>
                      <span>{shortHash(idea.idea_id)}</span>
                    </div>
                  </button>
                </li>
              ))}
            </ul>
          </aside>

          <div style={{ display: "grid", gap: 16 }}>
            <section style={cardStyle}>
              <h2 style={{ marginTop: 0, marginBottom: 10, fontSize: 18 }}>Selected idea</h2>
              {!selectedIdeaSummary ? <p style={{ marginBottom: 0 }}>Select an idea to inspect the canonical detail.</p> : null}
              {selectedIdeaSummary ? (
                <>
                  <p style={{ margin: "0 0 6px", fontSize: 22, fontWeight: 700 }}>{selectedIdeaSummary.title}</p>
                  <p style={{ margin: "0 0 14px", color: "#334155", fontSize: 15 }}>
                    {selectedIdea?.sentence ?? selectedIdeaSummary.sentence ?? "(no sentence provided)"}
                  </p>
                  <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(180px, 1fr))", gap: 14 }}>
                    <div>
                      <div style={statLabelStyle}>Idea id</div>
                      <div style={statValueStyle}>{shortHash(selectedIdeaSummary.idea_id)}</div>
                    </div>
                    <div>
                      <div style={statLabelStyle}>Idea type</div>
                      <div style={statValueStyle}>{selectedIdeaSummary.idea_type}</div>
                    </div>
                    <div>
                      <div style={statLabelStyle}>Universal rank</div>
                      <div style={statValueStyle}>{selectedIdeaSummary.derived_universal_rank ?? "n/a"}</div>
                    </div>
                    <div>
                      <div style={statLabelStyle}>Speaker identity</div>
                      <div style={statValueStyle}>{selectedIdeaSummary.speaker_identity_title ?? shortHash(selectedIdeaSummary.speaker_identity_id)}</div>
                    </div>
                    <div>
                      <div style={statLabelStyle}>Payload hash</div>
                      <div style={statValueStyle}>{shortHash(selectedIdea?.payload_hash)}</div>
                    </div>
                    <div>
                      <div style={statLabelStyle}>Connections</div>
                      <div style={statValueStyle}>
                        {selectedIdea ? `${selectedIdea.incoming_connections.length} in / ${selectedIdea.outgoing_connections.length} out` : "loading"}
                      </div>
                    </div>
                  </div>
                </>
              ) : null}
              {detailLoading ? <p style={{ marginTop: 14, color: "#475569" }}>Loading detail and neighborhood...</p> : null}
            </section>

            <section style={{ display: "grid", gap: 16, gridTemplateColumns: "1fr 1fr" }}>
              <div style={cardStyle}>
                <h2 style={{ marginTop: 0, marginBottom: 10, fontSize: 18 }}>Direct connections</h2>
                {selectedIdea?.outgoing_connections.length || selectedIdea?.incoming_connections.length ? (
                  <ul style={{ listStyle: "none", margin: 0, padding: 0, display: "grid", gap: 10 }}>
                    {[...(selectedIdea?.outgoing_connections ?? []), ...(selectedIdea?.incoming_connections ?? [])].map((connection) => (
                      <li key={connection.connection_id} style={{ border: "1px solid #e2e8f0", borderRadius: 12, padding: 12 }}>
                        <div style={{ fontWeight: 600 }}>{connection.connection_type}</div>
                        <div style={{ color: "#475569", fontSize: 13 }}>{connectionSummary(connection)}</div>
                      </li>
                    ))}
                  </ul>
                ) : (
                  <p style={{ marginBottom: 0 }}>{selectedIdea ? "No direct connections for this idea." : "Select an idea to inspect connections."}</p>
                )}
              </div>

              <div style={cardStyle}>
                <h2 style={{ marginTop: 0, marginBottom: 10, fontSize: 18 }}>Neighborhood</h2>
                {selectedNeighborhood?.adjacent_ideas.length ? (
                  <ul style={{ listStyle: "none", margin: 0, padding: 0, display: "grid", gap: 10 }}>
                    {selectedNeighborhood.adjacent_ideas.map((idea) => (
                      <li key={idea.idea_id} style={{ border: "1px solid #e2e8f0", borderRadius: 12, padding: 12 }}>
                        <div style={{ fontWeight: 600 }}>{idea.title}</div>
                        <div style={{ color: "#475569", fontSize: 13 }}>{idea.sentence ?? "(no sentence provided)"}</div>
                      </li>
                    ))}
                  </ul>
                ) : (
                  <p style={{ marginBottom: 0 }}>
                    {selectedNeighborhood ? "No adjacent ideas were returned for this neighborhood query." : "Select an idea to inspect its local graph."}
                  </p>
                )}
              </div>
            </section>
          </div>
        </section>
      </div>
    </main>
  );
};
