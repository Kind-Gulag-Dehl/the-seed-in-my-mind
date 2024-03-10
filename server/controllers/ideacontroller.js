const Idea = require('../models/idea');

// Create a new Idea
exports.createIdea = async (req, res) => {
  try {
    const newIdea = new Idea(req.body);
    await newIdea.save();
    res.status(201).json(newIdea);
  } catch (error) {
    res.status(400).json({ message: error.message });
  }
};

// Retrieve all Ideas
exports.getAllIdeas = async (req, res) => {
  try {
    const ideas = await Idea.find();
    res.status(200).json(ideas);
  } catch (error) {
    res.status(500).json({ message: error.message });
  }
};

// Retrieve a single Idea by id
exports.getIdeaById = async (req, res) => {
  try {
    const idea = await Idea.findById(req.params.id);
    if (!idea) return res.status(404).json({ message: 'Idea not found' });
    res.status(200).json(idea);
  } catch (error) {
    res.status(500).json({ message: error.message });
  }
};

// Update an Idea by id
exports.updateIdeaById = async (req, res) => {
  try {
    const updatedIdea = await Idea.findByIdAndUpdate(req.params.id, req.body, { new: true });
    if (!updatedIdea) return res.status(404).json({ message: 'Idea not found' });
    res.status(200).json(updatedIdea);
  } catch (error) {
    res.status(400).json({ message: error.message });
  }
};

// Delete an Idea by id
exports.deleteIdeaById = async (req, res) => {
  try {
    const idea = await Idea.findByIdAndDelete(req.params.id);
    if (!idea) return res.status(404).json({ message: 'Idea not found' });
    res.status(200).json({ message: 'Idea deleted successfully' });
  } catch (error) {
    res.status(500).json({ message: error.message });
  }
};
