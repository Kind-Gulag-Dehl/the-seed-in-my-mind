const request = require('supertest');
const app = require('../index'); // Adjust the path as necessary to your Express app entry point
const { MongoMemoryServer } = require('mongodb-memory-server');
const mongoose = require('mongoose');

describe('Auth Routes', () => {
  let mongoServer;

  beforeAll(async () => {
    mongoServer = await MongoMemoryServer.create();
    const uri = mongoServer.getUri();
    await mongoose.connect(uri);
  });

  afterAll(async () => {
    await mongoose.disconnect();
    await mongoServer.stop();
  });

  it('should register a new user', async () => {
    const res = await request(app)
      .post('/auth/register') // Adjust if your route is different
      .send({
        username: 'testuser',
        email: 'testuser@example.com',
        password: 'password123'
      });
    
    expect(res.statusCode).toEqual(201);
    expect(res.body).toHaveProperty('message', 'User registered successfully');
  });

  it('should login the registered user', async () => {
    const res = await request(app)
      .post('/auth/login') // Adjust if your route is different
      .send({
        email: 'testuser@example.com',
        password: 'password123'
      });

    expect(res.statusCode).toEqual(200);
    expect(res.body).toHaveProperty('token');
  });
});
