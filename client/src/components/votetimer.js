import React, { useState, useEffect } from 'react';

const VotingTimer = ({ challengeId }) => {
  const [timeLeft, setTimeLeft] = useState(null);

  useEffect(() => {
    const loadTimer = () => {
      // Get start time from localStorage or set it if it's not there
      let startTime = sessionStorage.getItem(`startTime-${challengeId}`);
      if (!startTime) {
        startTime = Date.now();
        sessionStorage.setItem(`startTime-${challengeId}`, startTime);
        setTimeLeft(7200); // 2 hours in seconds
      } else {
        const elapsedTime = Math.floor((Date.now() - startTime) / 1000);
        const remainingTime = Math.max(7200 - elapsedTime, 0);
        setTimeLeft(remainingTime);
      }

      // Update timer every minute
      const intervalId = setInterval(() => {
        const elapsedTime = Math.floor((Date.now() - startTime) / 1000);
        const remainingTime = Math.max(7200 - elapsedTime, 0);
        setTimeLeft(remainingTime);
        if (remainingTime <= 0) {
          clearInterval(intervalId);
        }
      }, 60000); // Update every minute

      return () => clearInterval(intervalId);
    };

    loadTimer();
  }, [challengeId]);

  const formatTimeLeft = () => {
    const hours = Math.floor(timeLeft / 3600);
    const minutes = Math.floor((timeLeft % 3600) / 60);
    return `${hours}h ${minutes}m`;
  };

  return (
    <div>
      <h3>Time Left to Vote: {formatTimeLeft()}</h3>
      {/* Render challenge voting component here */}
    </div>
  );
};

export default VotingTimer;
