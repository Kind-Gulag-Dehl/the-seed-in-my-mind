import React from 'react';
import axios from 'axios';
import './updownvote.css';
import { FaThumbsUp, FaThumbsDown } from 'react-icons/fa';

const UpDownVote = ({ contentId, onVote, upvotes, downvotes, tier, voteType = 'ideas' }) => {
  const handleVote = async (type, event) => {
    event.stopPropagation();
    try {
      const endpoint = voteType === 'arguments'
        ? `http://localhost:5000/api/arguments/vote/${contentId}/${type}`
        : `http://localhost:5000/api/ideas/vote/${contentId}/${type}`;
      const response = await axios.patch(endpoint, {}, {
        headers: { Authorization: `Bearer ${localStorage.getItem('token')}` }
      });
      console.log('Vote response data:', response.data);
      onVote(response.data);
    } catch (error) {
      console.error(`UpDownVote: Failed to ${type} for contentId ${contentId}:`, error);
    }
  };

  const scale = Math.pow(0.8, tier - 1);

  return (
    <div className="vote-buttons" style={{ transform: `scale(${scale})` }}>
      <button onClick={(event) => handleVote('upvote', event)} className="upvote-button">
        <span>{upvotes}</span> <FaThumbsUp className="vote-icon"/>
      </button>
      <button onClick={(event) => handleVote('downvote', event)} className="downvote-button">
        <span>{downvotes}</span> <FaThumbsDown className="vote-icon"/>
      </button>
    </div>
  );
};

export default UpDownVote;
