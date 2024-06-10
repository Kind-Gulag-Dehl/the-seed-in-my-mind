const Challenge = require('../models/challenge'); 
const Idea = require('../models/idea'); // Ensure this is correctly imported if used

exports.voteOnChallenge = async (req, res) => {
    const { challengeId, voter, voteType } = req.body;

    if (!voteType) {
        return res.status(400).json({ message: 'Vote type is required.' });
    }

    if (!voter) {
        return res.status(400).json({ message: 'Voter ID is required.' });
    }

    try {
        const challenge = await Challenge.findById(challengeId);
        if (!challenge) {
            return res.status(404).json({ message: 'Challenge not found' });
        }

        const hasVoted = challenge.votes.some(v => v.voter && v.voter.toString() === voter.toString());
        if (hasVoted) {
            return res.status(400).json({ message: 'You have already voted on this challenge' });
        }

        challenge.votes.push({ voter, finalVote: voteType });
        if (voteType === 'yes') {
            challenge.yesVoteCount += 1;
        } else if (voteType === 'no') {
            challenge.noVoteCount += 1;
        }

        if (challenge.yesVoteCount >= 3) {
            challenge.status = 'succeeded';
            challenge.outcome = 'approved';
            const idea = await Idea.findById(challenge.associatedIdea);
            if (idea) {
                idea.isApproved = true;
                await idea.save();
            }
        } else if (challenge.noVoteCount >= 3) {
            challenge.status = 'failed';
            challenge.outcome = 'not_approved';
        }

        await challenge.save();
        res.json({ message: 'Vote recorded', challenge });
    } catch (error) {
        console.error('Error voting on challenge:', error);
        res.status(500).json({ message: 'Error voting on challenge', error: error.message });
    }
};
