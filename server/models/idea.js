const mongoose = require('mongoose');

const ideaSchema = new mongoose.Schema({
  title: {
    type: String,
    required: true,
    trim: true,
  },
  sentenceDescription: {
    type: String,
    trim: true,
    maxLength: 150,
  },
  paragraphDescription: {
    type: String,
    trim: true,
    maxLength: 500,
  },
  fullDescription: {
    type: String,
    trim: true,
    maxLength: 2000,
  },
  parentId: {
    type: mongoose.Schema.Types.ObjectId,
    ref: 'Idea',
    default: null,
  },
  tier: {
    type: Number,
    required: true,
   
    default: 1, // Start with tier 1 for main ideas, increment for sub-ideas
  },
  author: {
    type: mongoose.Schema.Types.ObjectId,
    ref: 'User',
    required: true,
  },
  upvotes: [{
    type: mongoose.Schema.Types.ObjectId,
    ref: 'User',
  }],
  downvotes: [{
    type: mongoose.Schema.Types.ObjectId,
    ref: 'User',
  }],
  createdAt: {
    type: Date,
    default: Date.now,
  },
  challenges: [{
    type: mongoose.Schema.Types.ObjectId,
    ref: 'Challenge',
  }],
  isApproved: {
    type: Boolean,
    default: false,
  },
});

// Virtuals to get upvotes and downvotes count
ideaSchema.virtual('upvoteCount').get(function() {
  return this.upvotes.length;
});
ideaSchema.virtual('downvoteCount').get(function() {
  return this.downvotes.length;
});

// Ensure virtual fields are included when converting documents to JSON
ideaSchema.set('toJSON', { virtuals: true });
ideaSchema.set('toObject', { virtuals: true });

const Idea = mongoose.model('Idea', ideaSchema);

module.exports = Idea;
