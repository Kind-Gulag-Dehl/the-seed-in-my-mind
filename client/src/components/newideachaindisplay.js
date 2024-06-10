import React from 'react';

const NewIdeaChainDisplay = ({ ideaChain }) => {
  if (!ideaChain || ideaChain.length === 0) {
    return <p>No parent idea chain available.</p>;
  }

  return (
    <div style={{ marginBottom: '20px', textAlign: 'left', fontSize: '14px', lineHeight: '1.5' }}>
      <h3>Idea Chain from Main Idea to Direct Parent</h3>
      <ol>
        {ideaChain.map((idea, index) => (
          <li key={index}>
            <strong>Title:</strong> {idea.title}
            <p><strong>Tier:</strong> {idea.tier}</p>
            <p><strong>Description:</strong> {idea.sentenceDescription}</p>
          </li>
        ))}
      </ol>
    </div>
  );
};

export default NewIdeaChainDisplay;
