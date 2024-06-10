import React, { useState } from 'react';
import axios from 'axios';
import Modal from '../components/modal';
import ChallengeDisplay from '../components/approvalchallengedisplay';

const VotePage = () => {
    const [modalIsOpen, setModalIsOpen] = useState(false);
    const [currentChallenge, setCurrentChallenge] = useState(null);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState('');
    const [timer, setTimer] = useState(null);
    const [timeLeft, setTimeLeft] = useState(null);
   
    const fetchRandomChallenge = async () => {
        setLoading(true);
        try {
            const response = await axios.get('http://localhost:5000/api/challenges/random');
            console.log("Fetched challenge with details:", response.data);  // Ensure this data includes associatedIdeaDetails
            setCurrentChallenge(response.data);
            setLoading(false);
        } catch (err) {
            setError('Failed to fetch challenge. Please try again.');
            console.error(err);
            setLoading(false);
        }
    };
    

    const handleBeginVote = () => {
        if (!currentChallenge) {
            fetchRandomChallenge(); // Fetch challenge when starting the vote
        }
        const endTime = Date.now() + 2 * 3600 * 1000; // 2 hours from now
        setTimer(endTime);
        setTimeLeft(2 * 3600); // 7200 seconds
    };

    const intervalId = setInterval(() => {
        if (timeLeft > 0) {
            const secondsLeft = Math.floor((timer - Date.now()) / 1000);
            setTimeLeft(secondsLeft);
        } else if (timeLeft === 0) {
            clearInterval(intervalId);
            setError("Time expired for voting.");
        }
    }, 1000);

    const handleVote = async (approve) => {
        if (Date.now() > timer) {
            setError("Time expired for voting.");
            return;
        }
        try {
            const voteType = approve ? 'approve' : 'not_approve';
            await axios.post('http://localhost:5000/api/challenges/vote', {
                challengeId: currentChallenge._id,
                voteType: voteType
            });
            fetchRandomChallenge(); // Refetch on vote submission
            setModalIsOpen(false);
        } catch (err) {
            setError('Failed to submit vote. Please try again.');
            console.error(err);
        }
    };

    const formatTimeLeft = () => {
        if (timeLeft != null) {
            const hours = Math.floor(timeLeft / 3600);
            const minutes = Math.floor((timeLeft % 3600) / 60);
            const seconds = timeLeft % 60;
            return `${hours}h ${minutes}m ${seconds}s`;
        }
        return '';
    };

    if (loading) return <p>Loading...</p>;
    if (error) return <p>Error: {error}</p>;

    return (
        <div className="vote-page">
            <h1>Vote on Challenge</h1>
            <p>Please read the following information carefully before you begin voting:</p>
            <ul>
                <li>When you click "Begin Vote" a random approval challenge will be selected for you to vote on.</li>
                <li>Ensure you review all provided information about the proposed idea and the challenge. Your job is to attempt to assess the information as objectively as possible before making a determination.</li>
                <li>Your vote is crucial in shaping the outcomes of proposed projects.</li>
                <li>You have 2 hours to cast your vote once you start the process.</li>
                <li>Each user is limited to one vote per 24-hour period. Once you click "Begin Vote" it will count as your vote for the day regardless of whether you submit a final vote.</li>
            </ul>
            <button onClick={handleBeginVote} disabled={loading || timeLeft !== null}>
                {loading ? 'Loading...' : 'Begin Vote'}
            </button>
            {timer && <p>Time remaining: {formatTimeLeft()}</p>}
            {timeLeft !== null && currentChallenge && (
                <>
                    <ChallengeDisplay challenge={currentChallenge} onStartVote={() => setModalIsOpen(true)} />
                    {modalIsOpen && (
                        <Modal isOpen={modalIsOpen} onClose={() => setModalIsOpen(false)}>
                            <h2>Vote Confirmation</h2>
                            <p>Does the idea meet the criteria to be added to the idea?</p>
                            <button onClick={() => handleVote(true)}>Approve</button>
                            <button onClick={() => handleVote(false)}>Not Approve</button>
                        </Modal>
                    )}
                </>
            )}
        </div>
    );
};

export default VotePage;
