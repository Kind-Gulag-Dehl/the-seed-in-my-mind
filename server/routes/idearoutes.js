const express = require('express');
const router = express.Router();
const ideaController = require('../controllers/ideacontroller');
const { protect } = require('../middleware/authmiddleware');

// Routes for creating ideas and sub-ideas
router.post('/', protect, ideaController.createIdea);
router.post('/:parentId/sub', protect, ideaController.createSubIdea);

// Specific routes for fetching ideas
router.get('/main', ideaController.getMainIdeas);

// This route fetches all proposed ideas without specifying a parentId and does not require authentication
router.get('/proposed', ideaController.getProposedIdeas);

// This route fetches proposed ideas by parentId and does not require authentication
router.get('/proposed/:parentId', ideaController.getProposedIdeasByParentId);

// General routes for fetching, updating, and deleting ideas
router.get('/', ideaController.getAllIdeas); // Added back from the previous version to fetch all ideas
router.get('/:id', ideaController.getIdeaById);
router.get('/:id/sub', ideaController.getSubIdeasByParentId);

// Route to handle upvoting an idea
router.patch('/:id/upvote', protect, ideaController.upvoteIdea);

// Route to handle downvoting an idea
router.patch('/:id/downvote', protect, ideaController.downvoteIdea);

// Routes that modify data and thus require authentication
router.patch('/approve/:id', protect, ideaController.approveIdea);
router.put('/:id', protect, ideaController.updateIdeaById);
router.put('/:id/move', protect, ideaController.moveIdeaWithinHierarchy);
router.delete('/:id', protect, ideaController.deleteIdeaById);

// Catch-all route for undefined paths
router.all('*', (req, res) => {
    console.log(`404 Not Found: ${req.originalUrl}`);
    res.status(404).json({ message: 'Not found' });
});

module.exports = router;
