import React, { useState, useEffect } from 'react';
import axios from 'axios';
import NewChallengeDisplay from '../components/newchallengedisplay';
import Modal from '../components/modal';
import VoteTimer from '../components/votetimer';

const NewVotePage = () => {
    const userId = localStorage.getItem('userId');
    const lastVoteTime = sessionStorage.getItem(`lastVoteTime-${userId}`);
    const canVoteAgainTime = lastVoteTime ? new Date(parseInt(lastVoteTime, 10) + 24 * 3600 * 1000) : new Date(0);
    const now = new Date();

    const [currentChallenge, setCurrentChallenge] = useState(JSON.parse(sessionStorage.getItem(`challenge-${userId}`)));
    const [timeLeft, setTimeLeft] = useState(parseInt(sessionStorage.getItem(`timeLeft-${userId}`), 10));
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState('');
    const [isVoting, setIsVoting] = useState(false);

    useEffect(() => {
        if (!currentChallenge && now > canVoteAgainTime) {
            fetchRandomChallenge();
        }
    }, []);

    const fetchRandomChallenge = async () => {
        if (!sessionStorage.getItem(`challenge-${userId}`) && now > canVoteAgainTime) {
            setLoading(true);
            setError('');
            try {
                const response = await axios.get('http://localhost:5000/api/challenges/random', {
                    params: { userId }
                });
                if (response.data) {
                    setCurrentChallenge(response.data);
                    sessionStorage.setItem(`challenge-${userId}`, JSON.stringify(response.data));
                    const timeForVote = 7200;
                    setTimeLeft(timeForVote);
                    sessionStorage.setItem(`timeLeft-${userId}`, timeForVote.toString());
                } else {
                    throw new Error("No data returned");
                }
            } catch (err) {
                setError('Failed to fetch challenge. Please try again.');
                console.error(err);
            } finally {
                setLoading(false);
            }
        }
    };

    useEffect(() => {
        let interval;
        if (timeLeft > 0) {
            interval = setInterval(() => {
                const newTimeLeft = Math.max(0, timeLeft - 60);
                setTimeLeft(newTimeLeft);
                sessionStorage.setItem(`timeLeft-${userId}`, newTimeLeft.toString());
                if (newTimeLeft === 0) {
                    setError("Time expired for voting.");
                    sessionStorage.removeItem(`challenge-${userId}`);
                    sessionStorage.removeItem(`timeLeft-${userId}`);
                    setCurrentChallenge(null);
                    setIsVoting(false);
                }
            }, 60000);
        }
        return () => clearInterval(interval);
    }, [timeLeft, userId]);

    const handleVote = async (voteType) => {
        setError('');
        try {
            const response = await axios.post('http://localhost:5000/api/challenges/vote', {
                challengeId: currentChallenge._id,
                voteType: voteType,
                voter: userId
            });
            sessionStorage.setItem(`lastVoteTime-${userId}`, `${now.getTime()}`);
            setIsVoting(false);
            setError("Your vote has been placed.");
            setCurrentChallenge(null);
            sessionStorage.removeItem(`challenge-${userId}`);
            sessionStorage.removeItem(`timeLeft-${userId}`);
        } catch (error) {
            setError('Failed to submit your vote. Please try again.');
            console.error('Error submitting vote:', error);
        }
    };

    return (
        <div className="new-vote-page">
            <h1>Vote on Challenge</h1>
            <p>Please read the following information carefully before you begin voting:</p>
            <ul>
                <li>When you click "Begin Vote" a random challenge will be selected for you to vote on.</li>
                <li>You have 2 hours to cast your vote once you start the process.</li>
                <li>You can only vote once every 24 hours.</li>
            </ul>
            {error && <p>Error: {error}</p>}
            {loading ? <p>Loading...</p> : currentChallenge ? (
                <>
                    <NewChallengeDisplay challenge={currentChallenge} />
                    <VoteTimer initialTime={timeLeft} challengeId={currentChallenge._id} />
                    <button onClick={() => setIsVoting(true)} disabled={loading || timeLeft === 0 || now < canVoteAgainTime}>
                        Place Vote
                    </button>
                    <Modal isOpen={isVoting} onClose={() => setIsVoting(false)}>
                        <div>
                            <h2>Confirm Your Vote</h2>
                            <button onClick={() => handleVote('yes')}>Yes</button>
                            <button onClick={() => handleVote('no')}>No</button>
                        </div>
                    </Modal>
                </>
            ) : (
                <button onClick={fetchRandomChallenge} disabled={loading || now < canVoteAgainTime}>
                    Begin Voting
                </button>
            )}
        </div>
    );
};

export default NewVotePage;
