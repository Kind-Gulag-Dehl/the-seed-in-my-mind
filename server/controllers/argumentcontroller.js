const Argument = require('../models/argument');
const Challenge = require('../models/challenge');

exports.addArgument = async (req, res) => {
  try {
    const { title, sentenceDescription, paragraphDescription, fullDescription, side, challengeId } = req.body;
    const argument = new Argument({
      title,
      sentenceDescription,
      paragraphDescription,
      fullDescription,
      author: req.user.userId,
      side,
      challenge: challengeId
    });

    await argument.save();

    await Challenge.findByIdAndUpdate(
      challengeId,
      {
        $push: {
          [side === 'for' ? 'argumentsFor' : 'argumentsAgainst']: argument._id
        }
      }
    );

    res.status(201).json(argument);
  } catch (error) {
    res.status(500).json({ message: 'Error adding argument', error: error.message });
  }
};

exports.getArgumentsByChallenge = async (req, res) => {
    try {
      const arguments = await Argument.find({ challenge: req.params.challengeId })
        .populate('upvotes downvotes', 'userId');
      console.log("Arguments fetched with vote details:", arguments); // Detailed log
      res.status(200).json(arguments);
    } catch (error) {
      console.error("Error fetching arguments with votes:", error); // Log errors
      res.status(500).json({ message: 'Error fetching arguments', error: error.message });
    }
  };
  

exports.voteOnArgument = async (req, res) => {
    try {
      const { argumentId, type } = req.params;
      const userId = req.user.userId;
  
      const argument = await Argument.findById(argumentId);
      if (!argument) {
        return res.status(404).json({ message: 'Argument not found' });
      }
  
      if (type === 'upvote') {
        if (!argument.upvotes.includes(userId)) {
          argument.upvotes.push(userId);
          argument.downvotes = argument.downvotes.filter(id => id.toString() !== userId);
        }
      } else if (type === 'downvote') {
        if (!argument.downvotes.includes(userId)) {
          argument.downvotes.push(userId);
          argument.upvotes = argument.upvotes.filter(id => id.toString() !== userId);
        }
      } else {
        return res.status(400).json({ message: 'Invalid vote type' });
      }
  
      await argument.save();
      console.log('Updated argument:', argument);
      res.status(200).json(argument);
    } catch (error) {
      res.status(500).json({ message: 'Error voting on argument', error: error.message });
    }
  };
  

exports.deleteArgument = async (req, res) => {
  try {
    const { argumentId } = req.params;
    const argument = await Argument.findByIdAndDelete(argumentId);
    if (!argument) {
      return res.status(404).json({ message: 'Argument not found' });
    }
    res.status(200).json({ message: 'Argument deleted successfully' });
  } catch (error) {
    res.status(500).json({ message: 'Error deleting argument', error: error.message });
  }
};
