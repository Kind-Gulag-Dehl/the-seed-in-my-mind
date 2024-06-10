import React, { useState, useEffect } from 'react';

const NewVoteControl = ({ onStartVoting, timeLeft, isActive }) => {
  const [timeString, setTimeString] = useState('');

  // Effect to handle the countdown timer display
  useEffect(() => {
    if (timeLeft > 0) {
      const hours = Math.floor(timeLeft / 3600);
      const minutes = Math.floor((timeLeft % 3600) / 60);
      const seconds = timeLeft % 60;
      setTimeString(`${hours}h ${minutes}m ${seconds}s`);
    } else {
      setTimeString('');
    }
  }, [timeLeft]);

  return (
    <div>
      <h2>Vote Control</h2>
      {isActive ?
        <p>Time remaining: {timeString || 'Calculating...'}</p>
        :
        <button onClick={onStartVoting} disabled={isActive}>
          {isActive ? 'Voting...' : 'Begin Vote'}
        </button>
      }
    </div>
  );
};

export default NewVoteControl;
