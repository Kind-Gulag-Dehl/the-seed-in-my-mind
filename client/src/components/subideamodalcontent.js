import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import IdeaSubmissionForm from './ideasubmissionform';

const SubIdeaModalContent = ({ idea, color, onClose }) => {
  const [showForm, setShowForm] = useState(false);
  const navigate = useNavigate();

  const navigateToDetails = () => {
    navigate(`/idea-details/${idea._id}`);
    onClose(); // Close the modal upon navigation
  };

  const toggleShowForm = () => setShowForm(!showForm);

  return (
    <div style={{ backgroundColor: color, padding: '20px', color: 'white' }}> {/* Adjust color and padding as needed */}
      <h2>{idea.title}</h2>
      {/* Display the sentence description */}
      <p>{idea.sentenceDescription}</p>
      {!showForm ? (
        <>
          <button onClick={() => toggleShowForm()}>Add new sub idea</button>
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

export default SubIdeaModalContent;
