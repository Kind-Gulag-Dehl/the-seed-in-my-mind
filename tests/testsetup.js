const mongoose = require('mongoose');
const { MongoMemoryServer } = require('mongodb-memory-server');

const mongoServer = new MongoMemoryServer();

/**
 * Connect to a new in-memory database before running any tests.
 */
module.exports.setupDB = async () => {
  // Ensure the MongoDB Memory Server is started before attempting to connect
  await mongoServer.start();
  
  // With Mongoose 6 and later, no need to specify connection options that are now defaults
  const uri = await mongoServer.getUri();

  console.log(`Connecting to MongoDB URI: ${uri}`);  
  await mongoose.connect(uri);
}


/**
 * Drop database, close the connection and stop mongod.
 */
module.exports.teardownDB = async () => {
  await mongoose.connection.dropDatabase();
  await mongoose.connection.close();
  await mongoServer.stop();
}

/**
 * Remove all data from all db collections.
 */
module.exports.clearDB = async () => {
  const collections = mongoose.connection.collections;

  for (const key in collections) {
    const collection = collections[key];
    await collection.deleteMany();
  }
}
