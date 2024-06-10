const Challenge = require('../models/challenge'); // Assuming the model file is named challenge.js
const Idea = require('../models/idea'); 
const Argument = require('../models/argument');  // Adjust the path as necessary


exports.initiateChallenge = async (req, res) => {
    console.log("Received challenge data:", req.body);
    const { associatedIdea, initiator, challengeType, argumentsFor, argumentsAgainst } = req.body;

    try {
        // Create Argument documents for argumentsFor
        const argumentsForIds = await Promise.all(argumentsFor.map(async (arg) => {
            const newArgument = new Argument({
                ...arg,
                author: initiator,
                side: 'for',
                challenge: undefined  // This will need to be set once the Challenge ID is known if you add a `challenge` field in Argument model
            });
            await newArgument.save();
            return newArgument._id;
        }));

        // Similarly handle argumentsAgainst
        const argumentsAgainstIds = await Promise.all(argumentsAgainst.map(async (arg) => {
            const newArgument = new Argument({
                ...arg,
                author: initiator,
                side: 'against',
                challenge: undefined  // This will need to be set once the Challenge ID is known
            });
            await newArgument.save();
            return newArgument._id;
        }));

        const newChallenge = new Challenge({
            associatedIdea,
            initiator,
            challengeType,
            argumentsFor: argumentsForIds,
            argumentsAgainst: argumentsAgainstIds,
            status: 'pending'
        });

        await newChallenge.save();

        // Optionally, update arguments to include the challenge ID, if you add a `challenge` field to Argument
        await Argument.updateMany(
            { _id: { $in: argumentsForIds.concat(argumentsAgainstIds) }},
            { $set: { challenge: newChallenge._id }}
        );

        console.log("Challenge saved:", newChallenge);
        res.status(201).json(newChallenge);
    } catch (error) {
        console.error("Error initiating challenge:", error);
        res.status(400).json({ message: 'Error initiating challenge', error: error.message });
    }
};



exports.getRandomChallenge = async (req, res) => {
    console.log("Received userId:", req.query.userId);
    const userId = req.query.userId; // Make sure the userId is being passed as a query parameter

    const matchConditions = { // Define this before using it in the aggregation
        status: 'pending',
        'votes.voter': { $ne: userId } // Exclude challenges the user has voted on
    };

    const aggregationPipeline = [ // Define the aggregation pipeline as a variable for logging
        { $match: matchConditions },
        {
            $lookup: {
                from: "ideas",
                localField: "associatedIdea",
                foreignField: "_id",
                as: "associatedIdeaDetails"
            }
        },
        {
            $unwind: {
                path: "$associatedIdeaDetails",
                preserveNullAndEmptyArrays: true
            }
        },
        {
            $match: {
                "associatedIdeaDetails.isApproved": false
            }
        },
        { $sample: { size: 1 } }
    ];

    try {
        console.log("Aggregation Pipeline:", JSON.stringify(aggregationPipeline)); // Log the pipeline before execution
        const randomChallenge = await Challenge.aggregate(aggregationPipeline).exec();

        if (!randomChallenge || randomChallenge.length === 0) {
            console.log("No challenges available after aggregation"); // Log when no challenges are found
            return res.status(404).json({ message: 'No challenges available for voting' });
        }

        console.log("Challenge fetched:", randomChallenge[0]); // Log the fetched challenge
        res.json(randomChallenge[0]);
    } catch (error) {
        console.error("Error in getRandomChallenge:", error);
        res.status(500).json({ message: 'Error fetching random challenge', error: error.message });
    }
};



exports.getAllChallenges = async (req, res) => {
    try {
        const challenges = await Challenge.find({});
        res.json(challenges);
    } catch (error) {
        console.error("Error fetching challenges:", error);
        res.status(500).json({ message: 'Error fetching challenges', error: error.message });
    }
};


exports.getChallengeById = async (req, res) => {
    const { challengeId } = req.params;
    console.log(`Fetching challenge with ID: ${challengeId}`);

    try {
        const challenge = await Challenge.findById(challengeId)
            .populate({
                path: 'associatedIdea mainIdea parentIdea',
                select: 'title sentenceDescription paragraphDescription fullDescription tier author upvotes downvotes',
                populate: {
                    path: 'author', // Populate author details in the idea documents
                    select: 'name email'
                }
            })
            .populate({
                path: 'argumentsFor argumentsAgainst', // Populate both arguments arrays
                populate: { // Nested population for each argument
                    path: 'author upvotes downvotes', // Populate author and votes for arguments
                    select: 'name email' // Select only name and email for users
                }
            });

        if (!challenge) {
            console.log(`No challenge found with ID: ${challengeId}`);
            return res.status(404).json({ message: 'Challenge not found' });
        }

        console.log(`Challenge found: ${JSON.stringify(challenge)}`); // This will give a detailed log of the challenge, including populated fields
        res.json(challenge);
    } catch (error) {
        console.error(`Error fetching challenge by ID: ${error}`);
        res.status(500).json({ message: 'Error fetching challenge', error: error.message });
    }
};


// Retrieves the status of a challenge
exports.getChallengeStatus = async (req, res) => {
    try {
        const { challengeId } = req.params;
        const challenge = await Challenge.findById(challengeId);
        if (!challenge) {
            return res.status(404).json({ message: 'Challenge not found' });
        }
        res.status(200).json({ challenge });
    } catch (error) {
        res.status(400).json({ message: 'Error fetching challenge status', error: error.message });
    }
};
