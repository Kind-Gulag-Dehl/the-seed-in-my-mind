import { describe, expect, it } from "vitest";
import { normalizeIdeas, shortHash, type IdeaSummary } from "./referenceapp";

describe("normalizeIdeas", () => {
  it("deduplicates ideas by idea_id while preserving order", () => {
    const ideas: IdeaSummary[] = [
      {
        idea_id: "a",
        idea_type: "conceptual_idea",
        speaker_identity_id: "speaker-a",
        speaker_identity_title: "Speaker A",
        created_event_id: "event-a",
        title: "A",
        sentence: "first",
        derived_universal_rank: "1",
        author: {
          author_identity_id: "speaker-a",
          author_identity_title: "Speaker A",
          verification_level: null,
          persona_id: null
        }
      },
      {
        idea_id: "b",
        idea_type: "conceptual_idea",
        speaker_identity_id: "speaker-b",
        speaker_identity_title: "Speaker B",
        created_event_id: "event-b",
        title: "B",
        sentence: null,
        derived_universal_rank: "2",
        author: {
          author_identity_id: "speaker-b",
          author_identity_title: "Speaker B",
          verification_level: null,
          persona_id: null
        }
      },
      {
        idea_id: "a",
        idea_type: "conceptual_idea",
        speaker_identity_id: "speaker-a",
        speaker_identity_title: "Speaker A",
        created_event_id: "event-a-duplicate",
        title: "A2",
        sentence: "duplicate",
        derived_universal_rank: "3",
        author: {
          author_identity_id: "speaker-a",
          author_identity_title: "Speaker A",
          verification_level: null,
          persona_id: null
        }
      }
    ];

    const normalized = normalizeIdeas(ideas);
    expect(normalized.map((idea) => idea.idea_id)).toEqual(["a", "b"]);
    expect(normalized[0]?.sentence).toBe("first");
  });
});

describe("shortHash", () => {
  it("keeps short values intact and truncates longer hashes", () => {
    expect(shortHash("short")).toBe("short");
    expect(shortHash("0123456789abcdef0123456789abcdef")).toBe("0123456789...89abcdef");
    expect(shortHash(null)).toBe("unknown");
  });
});
