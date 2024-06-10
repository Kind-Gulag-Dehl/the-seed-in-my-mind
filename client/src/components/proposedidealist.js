import React, { useState, useEffect } from 'react';
import axios from 'axios';
import ChallengeForm from './challengeform'; // Updated import to use the new form name

const ProposedIdeaList = ({ ideaId }) => {
  const [proposedIdeas, setProposedIdeas] = useState([]);
  const [isCollapsed, setIsCollapsed] = useState(true);
  const [showChallengeForm, setShowChallengeForm] = useState(false);
  const [selectedIdeaId, setSelectedIdeaId] = useState(null);
  const [challengeType, setChallengeType] = useState('approve'); // Default challenge type, can be dynamic based on context

  useEffect(() => {
    const fetchProposedIdeas = async () => {
      try {
        const token = localStorage.getItem('token');
        const config = token ? { headers: { Authorization: `Bearer ${token}` } } : {};
        const response = await axios.get(`http://localhost:5000/api/ideas/proposed?parentId=${ideaId}`, config);
        setProposedIdeas(response.data);
      } catch ( error) {
        console.error("Error fetching proposed ideas:", error);
      }
    };

    fetchProposedIdeas();
  }, [ideaId]);

  const toggleCollapse = () => setIsCollapsed(!isCollapsed);

  const initiateChallenge = (ideaIdToChallenge, type) => {
    console.log("Initiating challenge for idea ID:", ideaIdToChallenge, "with type:", type);
    setSelectedIdeaId(ideaIdToChallenge);
    setChallengeType(type); // This can be dynamic based on where it's being called or what action triggers it
    setShowChallengeForm(true);
  };

  return (
    <div>
      <button onClick={toggleCollapse}>
        {isCollapsed ? 'Show Proposed Ideas' : 'Hide Proposed Ideas'}
      </button>
      {!isCollapsed && (
        <div style={{ maxHeight: '300px', overflowY: 'scroll' }}>
          {proposedIdeas.length > 0 ? proposedIdeas.map((idea) => (
            <div key={idea._id}>
              <p>{idea.title}</p>
              <button onClick={() => initiateChallenge(idea._id, 'approve')}>Initiate Challenge</button>
            </div>
          )) : <p>No proposed ideas yet.</p>}
        </div>
      )}
      {showChallengeForm && selectedIdeaId && (
        <ChallengeForm
          ideaId={selectedIdeaId}
          challengeType={challengeType}
          isOpen={showChallengeForm}
          onClose={() => setShowChallengeForm(false)}
        />
      )}
    </div>
  );
};

export default ProposedIdeaList;
