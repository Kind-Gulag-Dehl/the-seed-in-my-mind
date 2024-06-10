import React, { useState } from 'react';
import axios from 'axios';

const ChallengeVote = ({ challengeId }) => {
    const [voteType, setVoteType] = useState('');
    const [isSubmitting, setIsSubmitting] = useState(false);
    const [error, setError] = useState('');

    const handleVoteChange = (e) => {
        setVoteType(e.target.value);
    };

    const handleSubmit = async (e) => {
        e.preventDefault();
        setIsSubmitting(true);
        setError('');

        if (!voteType) {
            setError('Please select a vote option.');
            setIsSubmitting(false);
            return;
        }

        try {
            await axios.post('http://localhost:5000/api/challenges/vote', { challengeId, voteType });

            alert('Thank you for voting!');
            setVoteType(''); // Reset vote after submission
        } catch (err) {
            setError('Failed to submit your vote. Please try again.');
            console.error('Voting error:', err);
        }
        setIsSubmitting(false);
    };

    return (
        <form onSubmit={handleSubmit}>
            <h3>Vote on Challenge</h3>
            <div>
                <label>
                    <input
                        type="radio"
                        name="vote"
                        value="yes"
                        checked={voteType === 'yes'}
                        onChange={handleVoteChange}
                    /> Yes
                </label>
                <label>
                    <input
                        type="radio"
                        name="vote"
                        value="no"
                        checked={voteType === 'no'}
                        onChange={handleVoteChange}
                    /> No
                </label>
            </div>
            {error && <p className="error">{error}</p>}
            <button type="submit" disabled={isSubmitting}>
                {isSubmitting ? 'Submitting...' : 'Submit Vote'}
            </button>
        </form>
    );
};

export default ChallengeVote;
