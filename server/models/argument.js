const mongoose = require('mongoose');

const argumentSchema = new mongoose.Schema({
  title: {
    type: String,
    required: true
  },
  sentenceDescription: {
    type: String,
    required: true
  },
  paragraphDescription: {
    type: String,
    required: true
  },
  fullDescription: {
    type: String,
    required: true
  },
  author: {
    type: mongoose.Schema.Types.ObjectId,
    ref: 'User',
    required: true
  },
  side: {
    type: String,
    enum: ['for', 'against'],
    required: true
  },
  upvotes: [{
    type: mongoose.Schema.Types.ObjectId,
    ref: 'User',
    default: []  // Ensures that upvotes starts as an empty array
  }],
  downvotes: [{
    type: mongoose.Schema.Types.ObjectId,
    ref: 'User',
    default: []  // Ensures that downvotes starts as an empty array
  }]
});

const Argument = mongoose.model('Argument', argumentSchema);

module.exports = Argument;
