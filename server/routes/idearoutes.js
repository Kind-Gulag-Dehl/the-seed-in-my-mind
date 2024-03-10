const express = require('express');
const router = express.Router();
const ideaController = require('../controllers/ideacontroller');
const { protect } = require('../middleware/authmiddleware');

router.post('/', protect, ideaController.createIdea);
router.get('/', ideaController.getAllIdeas);
router.get('/:id', ideaController.getIdeaById);
router.put('/:id', protect, ideaController.updateIdeaById);
router.delete('/:id', protect, ideaController.deleteIdeaById);

module.exports = router;
