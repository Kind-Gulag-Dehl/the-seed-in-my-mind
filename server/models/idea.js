const mongoose = require('mongoose');

const rankHistoryEntrySchema = new mongoose.Schema({
  date: { type: Date, default: Date.now },
  rank: { type: Number }
}, { _id: false });

const rankSchema = new mongoose.Schema({
  currentRank: { type: Number, default: 0 },
  previousRank: { type: Number, default: 0 },
  hasBeenAssessed: { type: Boolean, default: false },
  rankHistory: [rankHistoryEntrySchema]  // Array to track history of ranks
});

// Schema to hold the total ranks for categories and timeframes
const totalRankSchema = new mongoose.Schema({
  immediate: rankSchema,
  shortTerm: rankSchema,
  mediumTerm: rankSchema,
  longTerm: rankSchema,
  century: rankSchema,
  categoryTotal: rankSchema  // Sum of all timeframes for a category
}, { _id: false });

const importanceRankCategorySchema = new mongoose.Schema({
  forIndividuals: totalRankSchema,
  toIndividuals: totalRankSchema,
  forUniverseLife: totalRankSchema,
  toUniverseLife: totalRankSchema,
  timeframeTotal: rankSchema  // Sum of all categories for a timeframe
});

const ideaSchema = new mongoose.Schema({
  title: { type: String, required: true, trim: true, maxLength: 50 },
  sentenceDescription: { type: String, trim: true, maxLength: 150 },
  paragraphDescription: { type: String, trim: true, maxLength: 500 },
  fullDescription: { type: String, trim: true, maxLength: 2000 },
  parentId: { type: mongoose.Schema.Types.ObjectId, ref: 'Idea', default: null },
  tier: { type: Number, required: true, default: 1 },
  author: { type: mongoose.Schema.Types.ObjectId, ref: 'User', required: true },
  upvotes: [{ type: mongoose.Schema.Types.ObjectId, ref: 'User' }],
  downvotes: [{ type: mongoose.Schema.Types.ObjectId, ref: 'User' }],
  createdAt: { type: Date, default: Date.now },
  challenges: [{ type: mongoose.Schema.Types.ObjectId, ref: 'Challenge' }],
  isApproved: { type: Boolean, default: false },
  challengeHistory: [{
    challengeId: { type: mongoose.Schema.Types.ObjectId, ref: 'Challenge' },
    outcome: { type: String },
    date: { type: Date, default: Date.now }
  }],
  importanceRanks: importanceRankCategorySchema,
  overallRank: rankSchema  // Overall rank summing all metrics
});

// Virtuals for vote counts
ideaSchema.virtual('upvoteCount').get(function() {
  return this.upvotes.length;
});
ideaSchema.virtual('downvoteCount').get(function() {
  return this.downvotes.length;
});

// Ensure inclusion of virtual fields in JSON and objects
ideaSchema.set('toJSON', { virtuals: true });
ideaSchema.set('toObject', { virtuals: true });

const Idea = mongoose.model('Idea', ideaSchema);

module.exports = Idea;
