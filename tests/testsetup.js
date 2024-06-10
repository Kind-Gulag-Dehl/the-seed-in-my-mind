const mongoose = require('mongoose');
const { MongoMemoryServer } = require('mongodb-memory-server');
const Idea = require('../server/models/idea'); // Make sure this path correctly points to your Idea model
const User = require('../server/models/user'); // Add this line to include your User model

const mongoServer = new MongoMemoryServer();

/**
 * Connect to a new in-memory database before running any tests.
 */
module.exports.setupDB = async () => {
  // Ensure the MongoDB Memory Server is started before attempting to connect
  await mongoServer.start();
  
  const uri = await mongoServer.getUri();
  console.log(`Connecting to MongoDB URI: ${uri}`);
  await mongoose.connect(uri);

  // Create a dummy user and save its ID for use in tests
  const dummyUser = new User({
    // Provide necessary fields according to your User model's schema
    username: 'testuser',
    email: 'test@example.com',
    password: 'testpassword',
  });
  await dummyUser.save();
  global.dummyUserId = dummyUser._id;
  console.log("Generated dummy user ID:", global.dummyUserId);
  
  // Optionally, create a global parent idea here if needed for all tests
  // Uncomment and adjust as needed if you plan to use a parent idea in multiple tests
  /*
  global.parentIdeaId = await module.exports.createParentIdea(global.dummyUserId);
  console.log("Created parent idea ID:", global.parentIdeaId);
  */
};

/**
 * Drop database, close the connection and stop mongod.
 */
module.exports.teardownDB = async () => {
  await mongoose.connection.dropDatabase();
  await mongoose.connection.close();
  await mongoServer.stop();
};

/**
 * Remove all data from all db collections.
 */
module.exports.clearDB = async () => {
  const collections = mongoose.connection.collections;
  for (const key in collections) {
    const collection = collections[key];
    await collection.deleteMany();
  }
};

// Example function to create a parent idea, call this in specific tests where needed
module.exports.createParentIdea = async (authorId) => {
  const parentIdea = new Idea({
    title: 'Parent Idea',
    description: 'This is a parent idea for testing',
    author: authorId, // Use the generated dummyUserId for authorId
  });

  await parentIdea.save();
  return parentIdea._id; // Return the ID of the created parent idea
};
