import React from 'react';
import { useNavigate } from 'react-router-dom';  // Import useNavigate

const ListChallengeSingle = ({ challenge }) => {
  const navigate = useNavigate();  // Hook to handle navigation

  // Function to navigate to challenge details page
  const handleNavigate = () => {
    navigate(`/challenge/${challenge._id}`);  // Navigate to the specific challenge details page
  };

  // Format dates
  const formatDateTime = (dateString) => {
    const date = new Date(dateString);
    return date.toLocaleDateString() + ' ' + date.toLocaleTimeString();
  };

  return (
    <div onClick={handleNavigate} style={{
      background: '#f0f0f0',
      borderRadius: '8px',
      padding: '8px 12px',
      margin: '5px 0',
      display: 'flex',
      alignItems: 'center',
      justifyContent: 'space-between',
      boxShadow: '0 1px 3px rgba(0,0,0,0.1)',
      fontSize: '14px',
      cursor: 'pointer'  // Change cursor to indicate it's clickable
    }}>
      <span><strong>Challenge:</strong> {challenge.subIdeaTitle}</span>
      <div style={{ margin: '0 10px', height: '100%', borderLeft: '1px solid #ccc' }}></div>
      <span><strong>Main Idea:</strong> {challenge.mainIdeaTitle}</span>
      <div style={{ margin: '0 10px', height: '100%', borderLeft: '1px solid #ccc' }}></div>
      <span><strong>Type:</strong> {challenge.challengeType}</span>
      <div style={{ margin: '0 10px', height: '100%', borderLeft: '1px solid #ccc' }}></div>
      <span><strong>Status:</strong> {challenge.status}</span>
      <div style={{ margin: '0 10px', height: '100%', borderLeft: '1px solid #ccc' }}></div>
      <span><strong>Start:</strong> {formatDateTime(challenge.startTime)}</span>
      <div style={{ margin: '0 10px', height: '100%', borderLeft: '1px solid #ccc' }}></div>
      <span><strong>End:</strong> {formatDateTime(challenge.endTime)}</span>
    </div>
  );
};

export default ListChallengeSingle;
