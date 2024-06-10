import React, { useState, useEffect } from 'react';
import ListChallengeSingle from './listchallengesingle';

const ListChallenge = ({ challenges }) => {
  const [filteredChallenges, setFilteredChallenges] = useState([]);

  // Effect to handle initializing and updating the list when the `challenges` prop changes
  useEffect(() => {
    if (Array.isArray(challenges)) {
      setFilteredChallenges(challenges);
    }
  }, [challenges]);

  return (
    <div>
      <h2>Challenge List</h2> {/* Header added here */}
      {filteredChallenges.length > 0 ? (
        filteredChallenges.map(challenge => (
          <ListChallengeSingle key={challenge._id} challenge={challenge} />
        ))
      ) : (
        <p>No challenges available.</p> // Display a message if there are no challenges
      )}
    </div>
  );
};

export default ListChallenge;
