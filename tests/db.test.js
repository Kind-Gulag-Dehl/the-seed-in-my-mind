const mongoose = require('mongoose');
const connectDB = require('../server/config/db');

describe('Database Connection', () => {
  // Before any tests run, attempt to connect to the database
  beforeAll(async () => {
    await connectDB();
  });

  // After all tests are done, disconnect from the database
  afterAll(async () => {
    await mongoose.disconnect();
  });

  it('should connect and disconnect to MongoDB successfully', async () => {
    // The mere fact that the beforeAll and afterAll hooks run without error
    // indicates that the connection and disconnection are successful
    expect(mongoose.connection.readyState).toBe(1); // 1 means connected
    await mongoose.disconnect();
    expect(mongoose.connection.readyState).toBe(0); // 0 means disconnected
  });
});
