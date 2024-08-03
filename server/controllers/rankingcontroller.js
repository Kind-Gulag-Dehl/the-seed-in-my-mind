const Idea = require('../models/idea'); // Make sure the Idea model is correctly imported



exports.periodicReRanking = async (req, res) => {
    // This function is meant to be run periodically and incrementally
    try {
        const batchSize = 10;  // Number of ideas to process at one time
        const lastProcessed = req.query.lastId || null;  // Assuming you pass the last processed ID as a query parameter

        let query = lastProcessed ? { '_id': { '$gt': lastProcessed } } : {}; // Ensure this uses the correct variable name

        // Fetch a batch of ideas starting after the last processed one
        const ideas = await Idea.find(query).sort('_id').limit(batchSize);

        if (!ideas.length) {
            return res.status(200).json({ message: "No more ideas to process for ranking" });
        }

        // Example processing logic
        ideas.forEach((idea, index) => {
            // Here you would adjust the ranking based on the specific rules of your system
            idea.importanceRanks.currentRank += index;
        });

        // Save all changes
        await Promise.all(ideas.map(idea => idea.save()));

        // Send back the ID of the last processed idea to facilitate subsequent calls
        const lastId = ideas[ideas.length - 1]._id;

        res.status(200).json({ message: "Batch ranking update successful", lastId });
    } catch (error) {
        console.error("Failed to periodically re-rank ideas", error);
        res.status(500).json({ message: "Failed to periodically re-rank ideas", error: error.message });
    }
};

exports.fetchRankings = async (req, res) => {
    // Placeholder for fetching rankings
    try {
        // Implement the logic to fetch and return rankings
        res.status(200).json({ message: "Rankings fetched successfully" });
    } catch (error) {
        console.error("Failed to fetch rankings", error);
        res.status(500).json({ message: "Failed to fetch rankings", error: error.message });
    }
};
