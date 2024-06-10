const express = require('express');
const router = express.Router();
const { upvoteIdea, downvoteIdea } = require('../controllers/ideacontroller');

// Route to handle upvoting an idea
router.patch('/:id/upvote', upvoteIdea);

// Route to handle downvoting an idea
router.patch('/:id/downvote', downvoteIdea);

module.exports = router;
