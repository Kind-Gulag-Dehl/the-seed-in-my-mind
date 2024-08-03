const express = require('express');
const router = express.Router();
const ideaController = require('../controllers/ideacontroller');
const rankingController = require('../controllers/rankingcontroller'); // Ensure this is properly set up and imported
const { protect } = require('../middleware/authmiddleware');

// Routes for creating ideas and sub-ideas
router.post('/', protect, ideaController.createIdea);
router.post('/:parentId/sub', protect, ideaController.createSubIdea);

// Specific routes for fetching ideas
router.get('/main', ideaController.getMainIdeas);
router.get('/proposed', ideaController.getProposedIdeas); // Fetch all proposed ideas without specifying a parentId
router.get('/proposed/:parentId', ideaController.getProposedIdeasByParentId); // Fetch proposed ideas by parentId

// General routes for fetching, updating, and deleting ideas
router.get('/', ideaController.getAllIdeas);
router.get('/:id', ideaController.getIdeaById);
router.get('/:id/sub', ideaController.getSubIdeasByParentId);

// Routes to handle voting
router.patch('/:id/upvote', protect, ideaController.upvoteIdea);
router.patch('/:id/downvote', protect, ideaController.downvoteIdea);

// Routes that modify data and thus require authentication
router.patch('/approve/:id', protect, ideaController.approveIdea);
router.put('/:id', protect, ideaController.updateIdeaById);
router.put('/:id/move', protect, ideaController.moveIdeaWithinHierarchy); // Correct typo from "mive" to "move"
router.delete('/:id', protect, ideaController.deleteIdeaById);

// New endpoints for the ranking system

router.post('/rank/recalculate', protect, rankingController.periodicReRanking); // Endpoint to recalculate rankings periodically

// Route to fetch current and historical rankings for display or analysis
router.get('/rank', rankingController.fetchRankings); // Assuming this function needs to be implemented

// Route to fetch importance data for a specific idea
router.get('/:id/importance', ideaController.getImportanceRanks);



// Catch-all route for undefined paths
router.all('*', (req, res) => {
    console.log(`404 Not Found: ${req.originalUrl}`);
    res.status(404).json({ message: 'Not found' });
});

module.exports = router;
