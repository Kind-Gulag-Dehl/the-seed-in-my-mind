const mongoose = require('mongoose');

const challengeSchema = new mongoose.Schema({
  associatedIdea: {
    type: mongoose.Schema.Types.ObjectId,
    ref: 'Idea',
    required: true
  },
  mainIdea: {
    type: mongoose.Schema.Types.ObjectId,
    ref: 'Idea'
  },
  mainIdeaTitle: String,
  subIdeaTitle: String,
  parentIdea: {
    type: mongoose.Schema.Types.ObjectId,
    ref: 'Idea'
  },
  parentIdeaTitle: String,
  initiator: {
    type: mongoose.Schema.Types.ObjectId,
    ref: 'User',
    required: true
  },
  challengeType: {
    type: String,
    required: true,
    enum: ['approve', 'edit', 'move', 'remove', 'other']
  },
  status: {
    type: String,
    required: true,
    enum: ['pending', 'succeeded', 'failed', 'defended'],
    default: 'pending'
  },
  argumentsFor: [{
    type: mongoose.Schema.Types.ObjectId,
    ref: 'Argument'
  }],
  argumentsAgainst: [{
    type: mongoose.Schema.Types.ObjectId,
    ref: 'Argument'
  }],
  startTime: {
    type: Date,
    default: Date.now
  },
  endTime: {
    type: Date,
    default: () => new Date(+new Date() + 7*24*60*60*1000)
  },
  outcome: {
    type: String,
    enum: ['succeeded', 'failed']
  },
  votes: [{
    voter: {
      type: mongoose.Schema.Types.ObjectId,
      ref: 'User'
    },
    finalVote: {
      type: String,
      enum: ['yes', 'no']
    },
    rankedArgumentsFor: [{
      argument: {
        type: mongoose.Schema.Types.ObjectId,
        ref: 'Argument'
      },
      rank: Number
    }],
    rankedArgumentsAgainst: [{
      argument: {
        type: mongoose.Schema.Types.ObjectId,
        ref: 'Argument'
      },
      rank: Number
    }]
  }],
  yesVoteCount: {
    type: Number,
    default: 0
  },
  noVoteCount: {
    type: Number,
    default: 0
  }
});

// Recursive function to find the main idea
async function findMainIdea(ideaId) {
  const Idea = mongoose.model('Idea');
  const idea = await Idea.findById(ideaId).populate('parentId');
  if (idea.parentId) {
    return findMainIdea(idea.parentId);
  } else {
    return { mainIdeaId: idea._id, mainIdeaTitle: idea.title };
  }
}

// Pre-save hook to set mainIdea, mainIdeaTitle, and subIdeaTitle, and to update outcome
challengeSchema.pre('save', async function (next) {
  const Idea = mongoose.model('Idea');
  const idea = await Idea.findById(this.associatedIdea).populate('parentId');
  this.subIdeaTitle = idea.title;
  this.parentIdea = idea.parentId;
  if (idea.parentId) {
    const parent = await Idea.findById(idea.parentId);
    this.parentIdeaTitle = parent.title;
  }
  const mainIdeaInfo = await findMainIdea(idea._id);
  this.mainIdea = mainIdeaInfo.mainIdeaId;
  this.mainIdeaTitle = mainIdeaInfo.mainIdeaTitle;

  // Check vote counts to determine outcome
  if (this.yesVoteCount >= 3) {
    this.outcome = 'succeeded';
    this.status = 'succeeded';
    if (idea) {
      idea.isApproved = true;
      await idea.save();
    }
  } else if (this.noVoteCount >= 3) {
    this.outcome = 'failed';
    this.status = 'failed';
  }

  next();
});

const Challenge = mongoose.model('Challenge', challengeSchema);

module.exports = Challenge;
