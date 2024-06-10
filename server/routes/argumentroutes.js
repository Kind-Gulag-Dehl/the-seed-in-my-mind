const express = require('express');
const router = express.Router();
const {
  addArgument,
  getArgumentsByChallenge,
  voteOnArgument,
  deleteArgument
} = require('../controllers/argumentcontroller');

const { protect } = require('../middleware/authmiddleware');

router.post('/', protect, addArgument);
router.get('/:challengeId', protect, getArgumentsByChallenge);
router.patch('/vote/:argumentId/:type', protect, voteOnArgument);
router.delete('/:argumentId', protect, deleteArgument);

module.exports = router;
