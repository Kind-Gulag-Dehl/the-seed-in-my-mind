import React from 'react';


const NewIdeaDetailsBox = ({ idea }) => {
    if (!idea) return <p>No idea details available.</p>;

    return (
        <div className="new-idea-details-box" style={{ padding: '10px', border: '1px solid gray', borderRadius: '5px', marginBottom: '10px' }}>
            <h3>{idea.title}</h3>
            <p><strong>Tier:</strong> {idea.tier || 'N/A'}</p>
            <p><strong>Description:</strong> {idea.sentenceDescription || 'No description provided'}</p>
            <p><strong>Detailed:</strong> {idea.paragraphDescription || 'No detailed description provided'}</p>
            <p><strong>Full Text:</strong> {idea.fullDescription || 'No full description available'}</p>
        </div>
    );
};

export default NewIdeaDetailsBox;
