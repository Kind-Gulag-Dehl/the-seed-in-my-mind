import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import IdeaSubmissionForm from './ideasubmissionform';

// Note: Rename SubIdeaModalContent to MainIdeaModalContent to reflect its purpose
const MainIdeaModalContent = ({ idea, onClose }) => {
  const [showForm, setShowForm] = useState(false);
  const navigate = useNavigate();

  // Define the default color for the main idea
  const defaultColor = '#b3afaf';

  const navigateToDetails = () => {
    navigate(`/idea-details/${idea._id}`);
    onClose(); // Close the modal upon navigation
  };

  const toggleShowForm = () => setShowForm(!showForm);

  return (
    <div style={{ backgroundColor: defaultColor, padding: '20px', color: 'white' }}>
      {/* Use idea.title and ensure it is not null or undefined */}
      <h2>{idea.title || 'No Title'}</h2>
      {/* Display the sentence description */}
      <p>{idea.sentenceDescription}</p>
      {!showForm ? (
        <>
          <button onClick={toggleShowForm}>Add new sub idea</button>
          <button onClick={navigateToDetails}>View Full Details</button>
          {/* Add any other buttons or information you need here */}
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
