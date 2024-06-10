import React, { useState, useEffect } from 'react';
import { useParams } from 'react-router-dom';
import axios from 'axios';
import ChallengeDisplay from '../components/newchallengedisplay';  // Import the ChallengeDisplay component

const ChallengeDetailsPage = () => {
    const [challenge, setChallenge] = useState(null);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState('');
    const { challengeId } = useParams(); // Get the challenge ID from the URL

    useEffect(() => {
        if (challengeId) {
            fetchChallengeDetails(challengeId);
        }
    }, [challengeId]);  // Re-run this effect if challengeId changes

    const fetchChallengeDetails = async (challengeId) => {
        setLoading(true);
        try {
            const response = await axios.get(`http://localhost:5000/api/challenges/${challengeId}`);
            if (response.data) {
                setChallenge(response.data);  // Assuming response.data contains the full challenge details including arguments with votes
            } else {
                throw new Error('No challenge data received');
            }
        } catch (error) {
            setError(`Failed to fetch challenge details: ${error.message}`);
        } finally {
            setLoading(false);
        }
    };

    if (loading) return <div>Loading challenge details...</div>;
    if (error) return <div>Error: {error}</div>;

    return (
        <div>
            <h1>Challenge Details</h1>
            {challenge ? (
                <ChallengeDisplay challenge={challenge} />
            ) : <p>No challenge details available.</p>}
        </div>
    );
};

export default ChallengeDetailsPage;
