const Idea = require('../models/idea');
const User = require('../models/user'); // Assuming User model is properly set up for referencing in Idea creation

// Method to get an idea by ID, including its hierarchical structure
exports.getIdeaById = async (req, res) => {
  try {
    const idea = await Idea.findById(req.params.id);
    if (!idea) {
      return res.status(404).json({ message: "Idea not found" });
    }
    const ideaHierarchy = await buildHierarchy(idea._id);
    res.json(ideaHierarchy);
  } catch (error) {
    console.error('Error retrieving idea by ID:', error);
    res.status(500).json({ message: error.message });
  }
};

// Recursive function to build the hierarchy of ideas
async function buildHierarchy(ideaId) {
  let idea = await Idea.findById(ideaId).populate('author', 'username');
  if (!idea) return null;

  idea = idea.toObject();

  const children = await Idea.find({ parentId: ideaId, isApproved: true });
  if (children.length > 0) {
    idea.subIdeas = await Promise.all(children.map(child => buildHierarchy(child._id)));
  }
  return idea;
}



exports.createIdea = async (req, res) => {
  try {
    const {
      title,
      sentenceDescription,
      paragraphDescription,
      fullDescription,
      parentId
    } = req.body;

    const author = req.user.userId; // This line needs to be inside the async function.

    const newIdea = new Idea({
      title,
      sentenceDescription,
      paragraphDescription,
      fullDescription,
      parentId,
      author, // Assuming 'author' is necessary; make sure this ID exists in your database
      isApproved: false
    });
    
    console.log("New Idea Instance:", newIdea);

    const savedIdea = await newIdea.save();
    console.log("Saved Idea:", savedIdea);
    res.status(201).json(savedIdea);
  } catch (error) {
    console.error("Failed to create idea", error);
    res.status(500).json({ message: "Failed to create idea", error: error.message });
  }
};

// Get all main ideas regardless of isApproved status
exports.getMainIdeas = async (req, res) => {
  try {
    // Adjusted query to match documents where tier is 1
    const mainIdeas = await Idea.find({ tier: 1 });

    console.log('Retrieved all main ideas:', mainIdeas);
    res.status(200).json(mainIdeas);
  } catch (error) {
    console.error('Error retrieving all main ideas:', error);
    res.status(500).json({ message: error.message });
  }
};



// Create a sub-idea under a specific parent idea
exports.createSubIdea = async (req, res) => {
  console.log("Received request body for createSubIdea:", req.body); // Add this line

  try {
    const {
      title,
      sentenceDescription,
      paragraphDescription,
      fullDescription,
      parentId
    } = req.body;
    const author = req.user.userId;

    const parentIdea = await Idea.findById(parentId);
    if (!parentIdea) {
      return res.status(404).json({ message: "Parent idea not found" });
    }

    const newSubIdea = new Idea({
      title,
      sentenceDescription,
      paragraphDescription,
      fullDescription,
      parentId,
      author,
      tier: parentIdea.tier + 1,
    });

    const savedSubIdea = await newSubIdea.save();
    res.status(201).json(savedSubIdea);
  } catch (error) {
    res.status(500).json({ message: "Failed to create sub-idea", error: error.message });
  }
};


// Get all approved ideas
exports.getAllIdeas = async (req, res) => {
  try {
    const ideas = await Idea.find({ isApproved: true })
                            .populate('author', 'username')
                            .populate('parentId');
    console.log('Retrieved all approved ideas:', ideas);
    res.status(200).json(ideas);
  } catch (error) {
    console.error('Error retrieving all approved ideas:', error);
    res.status(500).json({ message: error.message });
  }
};


// New controller function to fetch sub-ideas by parent ID
exports.getSubIdeasByParentId = async (req, res) => {
  try {
    // Change from req.params.parentId to req.params.id to match the route parameter
    const subIdeas = await Idea.find({ parentId: req.params.id })
                               .populate('author', 'username'); // Optionally populate author or other fields
    if (!subIdeas.length) return res.status(404).json({ message: "No sub-ideas found for this parent ID" });
    res.status(200).json(subIdeas);
  } catch (error) {
    console.error('Error retrieving sub-ideas by parent ID:', error);
    res.status(500).json({ message: error.message });
  }
};

exports.getProposedIdeas = async (req, res) => {
  try {
    const parentId = req.query.parentId;
    console.log("Received parentId:", parentId); // Debug log

    let query = { isApproved: false };
    if (parentId) {
      query.parentId = parentId;
      console.log("Query with parentId:", query); // Debug log
    }

    const proposedIdeas = await Idea.find(query)
      .populate('author', 'username');
    console.log("Proposed ideas found:", proposedIdeas); // Debug log

    res.status(200).json(proposedIdeas);
  } catch (error) {
    console.error("Error fetching proposed ideas:", error);
    res.status(500).send("Error fetching proposed ideas");
  }
};

// In ideaController.js
exports.getProposedIdeasByParentId = async (req, res) => {
  try {
      const proposedIdeas = await Idea.find({
          parentId: req.params.parentId,
          isApproved: false
      }).populate('author', 'username');
      
      res.status(200).json(proposedIdeas);
  } catch (error) {
      console.error('Error fetching proposed ideas by parent ID:', error);
      res.status(500).json({ message: error.message });
  }
};



// Method to approve an idea
exports.approveIdea = async (req, res) => {
  try {
    const ideaId = req.params.id;
    // Update the idea setting isApproved to true
    const updatedIdea = await Idea.findByIdAndUpdate(ideaId, { $set: { isApproved: true } }, { new: true });
    if (!updatedIdea) return res.status(404).json({ message: "Idea not found" });
    res.status(200).json(updatedIdea);
  } catch (error) {
    console.error('Error approving idea:', error);
    res.status(500).json({ message: error.message });
  }
};

// Method to upvote an idea
exports.upvoteIdea = async (req, res) => {
  try {
    const userId = req.user.userId; // Ensure this uses the correct property
    const ideaId = req.params.id;
    console.log(`Attempting to upvote idea: ${ideaId} by user: ${userId}`);

    const updatedIdea = await Idea.findByIdAndUpdate(
      ideaId,
      {
        $addToSet: { upvotes: userId }, // Ensure the user is added if not already present
        $pull: { downvotes: userId } // Ensure the user is removed if present
      },
      { new: true }
    );
    
    if (!updatedIdea) {
      console.log(`No idea found with ID: ${ideaId}`);
      return res.status(404).json({ message: "Idea not found" });
    }
    console.log(`Upvote successful for idea: ${ideaId}, updated document: `, updatedIdea);
    res.status(200).json(updatedIdea);
  } catch (error) {
    console.error('Error upvoting idea:', error);
    res.status(500).json({ message: error.message });
  }
};

// Method to downvote an idea
exports.downvoteIdea = async (req, res) => {
  try {
    const userId = req.user.userId; // This must match the upvote method
    const ideaId = req.params.id;

    const updatedIdea = await Idea.findByIdAndUpdate(
      ideaId,
      {
        $addToSet: { downvotes: userId }, // Ensure the user is added if not already present
        $pull: { upvotes: userId } // Ensure the user is removed if present
      },
      { new: true }
    );
    
    if (!updatedIdea) {
      return res.status(404).json({ message: "Idea not found" });
    }

    res.status(200).json(updatedIdea);
  } catch (error) {
    console.error('Error downvoting idea:', error);
    res.status(500).json({ message: error.message });
  }
};


// Function to move an idea within the hierarchy by changing its parent
exports.moveIdeaWithinHierarchy = async (req, res) => {
  try {
      const { newParentId } = req.body; // Assuming the new parent's ID is passed in the body
      const ideaId = req.params.id; // The ID of the idea to move

      if (!newParentId) {
          return res.status(400).json({ message: "New parent ID is required" });
      }

      const ideaToUpdate = await Idea.findById(ideaId);
      if (!ideaToUpdate) {
          return res.status(404).json({ message: "Idea not found" });
      }

      // Optionally, you might want to check if newParentId is a valid idea itself
      const newParentIdea = await Idea.findById(newParentId);
      if (!newParentIdea) {
          return res.status(404).json({ message: "New parent idea not found" });
      }

      // Update the idea's parent
      ideaToUpdate.parentId = newParentId;
      const updatedIdea = await ideaToUpdate.save();

      res.status(200).json(updatedIdea);
  } catch (error) {
      console.error('Error moving idea within hierarchy:', error);
      res.status(500).json({ message: "Failed to move idea", error: error.message });
  }
};




// Update an idea
exports.updateIdeaById = async (req, res) => {
  try {
    const {
      title,
      sentenceDescription,
      paragraphDescription,
      fullDescription
    } = req.body;
    // Update the idea with new description fields
    const idea = await Idea.findByIdAndUpdate(
      req.params.id,
      {
        $set: {
          title,
          description: sentenceDescription, // Update sentenceDescription as the primary description
          additionalDescriptions: {
            paragraphDescription,
            fullDescription
          }
        }
      },
      { new: true }
    );
    if (!idea) return res.status(404).json({ message: "Idea not found" });
    res.status(200).json(idea);
  } catch (error) {
    console.error('Error updating idea by ID:', error);
    res.status(500).json({ message: error.message });
  }
};
// Delete idea by ID
exports.deleteIdeaById = async (req, res) => {
  try {
    const idea = await Idea.findByIdAndDelete(req.params.id);
    if (!idea) return res.status(404).json({ message: "Idea not found" });
    res.status(200).json({ message: "Idea deleted successfully" });
  } catch (error) {
    console.error('Error deleting idea by ID:', error);
    res.status(500).json({ message: error.message });
  }
};
