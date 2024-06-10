import React from 'react';

const NewParentIdeaDetails = ({ parentIdea }) => {
  if (!parentIdea || Object.keys(parentIdea).length === 0) {
    return <p>No parent idea details available.</p>;
  }

  return (
    <div style={{ marginBottom: '20px', textAlign: 'left', fontSize: '14px', lineHeight: '1.5' }}>
      <h3>Parent Idea Details</h3>
      <p><strong>Title:</strong> {parentIdea.title || 'N/A'}</p>
      <p><strong>Tier:</strong> {parentIdea.tier || 'N/A'}</p>
      <p><strong>Sentence Description:</strong> {parentIdea.sentenceDescription || 'N/A'}</p>
      <p><strong>Paragraph Description:</strong> {parentIdea.paragraphDescription || 'N/A'}</p>
      <p><strong>Full Description:</strong> {parentIdea.fullDescription || 'N/A'}</p>
      {parentIdea.parentId && (
        <p><strong>Direct Parent Idea:</strong> {parentIdea.parentId}</p>
      )}
      {parentIdea.otherParents && parentIdea.otherParents.length > 0 && (
        <div>
          <strong>Other Parent Ideas:</strong>
          <ul>
            {parentIdea.otherParents.map((otherParent, index) => (
              <li key={index}>{otherParent}</li>
            ))}
          </ul>
        </div>
      )}
    </div>
  );
};

export default NewParentIdeaDetails;
