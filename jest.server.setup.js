const { MongoMemoryServer } = require('mongodb-memory-server');

let mongoServer;

global.beforeAll(async () => {
  mongoServer = await MongoMemoryServer.create();
  process.env.MONGODB_URI = mongoServer.getUri();
  // Set JWT_SECRET for testing
  process.env.JWT_SECRET = 'N3wP2xpZm9yzW5haGtqdWQzcWx5dnd6kGZxcg';
});

global.afterAll(async () => {
  if (mongoServer) {
    await mongoServer.stop();
  }
});
