import React, { useState, useEffect } from 'react';
import axios from 'axios';

const ChallengeStatus = ({ challengeId }) => {
    const [status, setStatus] = useState('');
    const [details, setDetails] = useState({});
    const [error, setError] = useState('');
    const [isLoading, setIsLoading] = useState(false);

    useEffect(() => {
        const fetchStatus = async () => {
            setIsLoading(true);
            try {
                const response = await axios.get(`/api/challenges/status/${challengeId}`);
                setStatus(response.data.status);
                setDetails(response.data);
                setError(''); // Clear any previous errors
            } catch (err) {
                setError('Failed to fetch challenge status. Please try again.');
                console.error('Error fetching challenge status:', err);
            }
            setIsLoading(false);
        };

        fetchStatus();
    }, [challengeId]); // Re-fetch whenever challengeId changes

    return (
        <div>
            <h3>Challenge Status</h3>
            {isLoading ? (
                <p>Loading...</p>
            ) : error ? (
                <p className="error">{error}</p>
            ) : (
                <div>
                    <p>Status: {status}</p>
                    <p>Votes Yes: {details.votesYes || 0}</p>
                    <p>Votes No: {details.votesNo || 0}</p>
                    {details.outcome && <p>Outcome: {details.outcome}</p>}
                </div>
            )}
        </div>
    );
};

export default ChallengeStatus;
