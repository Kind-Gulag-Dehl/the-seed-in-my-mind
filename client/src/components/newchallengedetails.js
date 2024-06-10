import React from 'react';

const NewChallengeDetails = ({ challenge }) => {
    if (!challenge) {
        return <p>No challenge information available.</p>;
    }

    // Function to format date and time
    const formatDateTime = (dateString) => {
        const date = new Date(dateString);
        return date.toLocaleDateString() + ' ' + date.toLocaleTimeString();
    };

    return (
        <div style={{
            marginBottom: '10px',
            textAlign: 'left',
            fontSize: '10px',
            lineHeight: '0.2'
        }}>
            <h3>Challenge Details</h3>
            <p><strong>Type of Challenge:</strong> {challenge.challengeType || 'N/A'}</p>
            <p><strong>Status:</strong> {challenge.status || 'N/A'}</p>
            <p><strong>Proposed Idea Title:</strong> {challenge.subIdeaTitle || 'N/A'}</p>
            <p><strong>Main Idea Title:</strong> {challenge.mainIdeaTitle || 'N/A'}</p>
            <p><strong>Start Time:</strong> {formatDateTime(challenge.startTime)}</p>
            <p><strong>End Time:</strong> {formatDateTime(challenge.endTime)}</p>
            
        </div>
    );
};

export default NewChallengeDetails;
