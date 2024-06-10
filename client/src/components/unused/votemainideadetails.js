import React from 'react';

const VoteMainIdeaDetails = ({ idea }) => {
  if (!idea || Object.keys(idea).length === 0) {
    return <p>No idea details available.</p>; // Ensures that there's a check for null or empty objects
  }

  return (
    <div style={{ marginBottom: '20px', textAlign: 'left', fontSize: '14px', lineHeight: '1.5' }}>
      <h3>Main Idea and Details</h3>
      <p><strong>Title:</strong> {idea.title || 'N/A'}</p>
      <p><strong>Tier:</strong> {idea.tier || 'N/A'}</p>
      <p><strong>Direct Parent Idea:</strong> {idea.parentId || 'N/A'}</p>
      <p><strong>Other Parent Ideas:</strong> {idea.otherParents || 'N/A'}</p>
      <p><strong>Sentence Description:</strong> {idea.sentenceDescription || 'N/A'}</p>
      <p><strong>Paragraph Description:</strong> {idea.paragraphDescription || 'N/A'}</p>
      <p><strong>Full Description:</strong> {idea.fullDescription || 'N/A'}</p>
    </div>
  );
};

export default VoteMainIdeaDetails;
