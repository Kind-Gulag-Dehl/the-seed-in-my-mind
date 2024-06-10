import React, { useState, useEffect } from 'react';
import axios from 'axios';
import './homepage.css';
import IdeaSubmissionForm from '../components/ideasubmissionform';
import IdeaList from '../components/idealist';
import ListChallenge from '../components/listchallenge';

function HomePage() {
    const [challenges, setChallenges] = useState([]);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState('');

    useEffect(() => {
        setLoading(true);
        axios.get('http://localhost:5000/api/challenges')
            .then(response => {
                console.log("Challenges fetched successfully:", response.data);
                setChallenges(response.data);
                setLoading(false);
            })
            .catch(err => {
                console.error("Failed to fetch challenges:", err);
                setError('Failed to fetch challenges');
                setLoading(false);
            });
    }, []);

    console.log("Current challenges state:", challenges); // Log current state of challenges

    return (
        <div className="homepage">
            <h1>Welcome to The Mirror</h1>
            <p>A platform for collective reasoning, ideation, and truth determination.</p>
            <IdeaSubmissionForm />
            <IdeaList />
            {loading ? (
                <p>Loading challenges...</p>
            ) : error ? (
                <p>{error}</p>
            ) : (
                <ListChallenge challenges={challenges} />
            )}
        </div>
    );
}

export default HomePage;
