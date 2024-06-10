const express = require('express');
const router = express.Router();
const challengeController = require('../controllers/challengecontroller');
const voteController = require('../controllers/votecontroller');


// Route to initiate a new challenge
router.post('/initiate', challengeController.initiateChallenge);

// Route to get a random challenge
router.get('/random', challengeController.getRandomChallenge);

// Route to get all challenges
router.get('/', challengeController.getAllChallenges);

// Route to get challenges by id
router.get('/:challengeId', challengeController.getChallengeById);  // Corrected this line

// Route to vote on a challenge
router.post('/vote', voteController.voteOnChallenge);

// Route to check the status of a challenge
router.get('/status/:challengeId', challengeController.getChallengeStatus);

module.exports = router;
