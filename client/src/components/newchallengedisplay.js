import React, { useState, useEffect } from 'react';
import ArgumentList from './argumentlist';
import Modal from './modal';
import NewChallengeDetails from './newchallengedetails';
import NewIdeaDetailsBox from './newideadetailsbox'; // Make sure this component is correctly implemented

const ChallengeDisplay = ({ challenge }) => {
  const [selectedArgument, setSelectedArgument] = useState(null);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [updatedChallenge, setUpdatedChallenge] = useState(challenge);

  useEffect(() => {
    if (challenge._id) fetchChallengeDetails(challenge._id);
  }, [challenge]);

  const fetchChallengeDetails = async (challengeId) => {
    try {
      const response = await fetch(`http://localhost:5000/api/challenges/${challengeId}`);
      if (!response.ok) throw new Error('Failed to fetch challenge details.');
      const data = await response.json();
      setUpdatedChallenge(data);
    } catch (error) {
      console.error('Error fetching challenge details:', error);
    }
  };

  const handleArgumentSelect = (arg) => {
    setSelectedArgument(arg);
    setIsModalOpen(true);
  };

  const closeModal = () => {
    setIsModalOpen(false);
    setSelectedArgument(null);
  };

  const sectionStyle = {
    backgroundColor: '#f0f0f0', // Grey background for the content boxes
    padding: '10px',
    marginBottom: '20px',
    borderRadius: '8px',
    border: 'none', // Removing the black border
    width: '100%', // Ensure it does not overflow
    boxSizing: 'border-box' // Includes padding and border in the width
  };

  if (!updatedChallenge) {
    return <p>No challenge selected yet.</p>;
  }

  return (
    <div className="challenge-display" style={{ width: '100%', overflowX: 'hidden', backgroundColor: 'white' }}>
      {updatedChallenge.associatedIdea && (
        <div style={sectionStyle}>
          <h4>Proposed Idea:</h4>
          <NewIdeaDetailsBox idea={updatedChallenge.associatedIdea} />
        </div>
      )}

      <div style={sectionStyle}>
        <NewChallengeDetails challenge={updatedChallenge} />
        <h4>Arguments For:</h4>
        <ArgumentList argList={updatedChallenge.argumentsFor} onArgumentSelect={handleArgumentSelect} />
        <h4>Arguments Against:</h4>
        <ArgumentList argList={updatedChallenge.argumentsAgainst} onArgumentSelect={handleArgumentSelect} />
      </div>

      {updatedChallenge.mainIdea && (
        <div style={sectionStyle}>
          <h4>Main Idea:</h4>
          <NewIdeaDetailsBox idea={updatedChallenge.mainIdea} />
        </div>
      )}

      {updatedChallenge.parentIdea && (
        <div style={sectionStyle}>
          <h4>Direct Parent Idea:</h4>
          <NewIdeaDetailsBox idea={updatedChallenge.parentIdea} />
        </div>
      )}

      {isModalOpen && (
        <Modal isOpen={isModalOpen} onClose={closeModal}>
          <h2>Argument Details</h2>
          <p><strong>Title:</strong> {selectedArgument?.title}</p>
          <p><strong>Sentence Description:</strong> {selectedArgument?.sentenceDescription}</p>
          <p><strong>Paragraph Description:</strong> {selectedArgument?.paragraphDescription}</p>
          <p><strong>Full Description:</strong> {selectedArgument?.fullDescription}</p>
        </Modal>
      )}
    </div>
  );
};

export default ChallengeDisplay;
