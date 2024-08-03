import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import IdeaSubmissionForm from './ideasubmissionform';

const MainIdeaModalContent = ({ idea, onClose }) => {
  const [showForm, setShowForm] = useState(false);
  const navigate = useNavigate();

  const defaultColor = '#b3afaf';

  const navigateToDetails = () => {
    navigate(`/idea-details/${idea._id}`);
    onClose(); // Close the modal upon navigation
  };

  const navigateToImportanceRank = () => {
    navigate(`/importance-page/${idea._id}`); // Assuming 'importance-page' is the route for the importance page
    onClose(); // Close the modal upon navigation
  };

  const toggleShowForm = () => setShowForm(!showForm);

  return (
    <div style={{ backgroundColor: defaultColor, padding: '20px', color: 'white' }}>
      <h2>{idea.title || 'No Title'}</h2>
      <p>{idea.sentenceDescription}</p>
      {!showForm ? (
        <>
          <button onClick={toggleShowForm}>Add new sub idea</button>
          <button onClick={navigateToDetails}>View Full Details</button>
          <button onClick={navigateToImportanceRank}>Importance Rank Page</button> {/* New button for navigating to importance rank page */}
        </>
      ) : (
        <IdeaSubmissionForm
          parentId={idea._id}
          onSubmissionSuccess={() => {
            toggleShowForm(); // Optionally close the form after submission
            onClose(); // Close the modal
          }}
        />
      )}
    </div>
  );
};

export default MainIdeaModalContent;
