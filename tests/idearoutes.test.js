const request = require('supertest'); // Import supertest for making requests
const app = require('../index'); // Adjust this path to your app's main file
const { setupDB, teardownDB, clearDB } = require('./testsetup'); // Import your test setup utilities

describe('Idea CRUD operations', () => {
  let token;

  // Setup and teardown your test database
  beforeAll(async () => {
    await setupDB();

    // Register the user
    await request(app).post('/auth/register').send({
      username: "testUser",
      email: "test@testing.com",
      password: "hardcodedPassword",
      // Include any other required fields according to your user model
    });

    // Now, log in to get the token using the same credentials
    const loginResponse = await request(app).post('/auth/login').send({
      email: "test@testing.com",
      password: "hardcodedPassword",
    });
    token = loginResponse.body.token; // Ensure this path correctly matches the structure of your login response
    console.log("Token obtained for tests:", token);
  });

  afterAll(async () => {
    await teardownDB();
  });

  beforeEach(async () => {
    await clearDB();
  });

  test('Create a new idea - Success', async () => {
    const newIdea = {
      title: 'Test Idea',
      description: 'This is a test idea',
      author: '507f1f77bcf86cd799439011', // Make sure this author ID is valid or relevant to your test setup
      votes: 0,
    };

    const response = await request(app).post('/ideas')
      .set('Authorization', `Bearer ${token}`)
      .send(newIdea);
    expect(response.statusCode).toBe(201);
    expect(response.body.title).toBe(newIdea.title);
  });

  // Continue with other tests
});
