const request = require('supertest');
const app = require('../index'); // Adjust this path to correctly point to your server's entry file
const { setupDB, teardownDB, clearDB, createParentIdea } = require('./testsetup');

describe('Idea CRUD operations', () => {
  let token;
  let parentIdeaId;

  beforeAll(async () => {
    await setupDB();

    // Create a new user and login to obtain a token
    const userCredentials = {
      username: "testUser",
      email: "test@testing.com",
      password: "hardcodedPassword",
    };

    await request(app)
      .post('/api/auth/register')
      .send(userCredentials);

    const loginResponse = await request(app)
      .post('/api/auth/login')
      .send({
        email: userCredentials.email,
        password: userCredentials.password,
      });

    token = loginResponse.body.token; // Assuming your login response includes the token
    console.log("Token obtained for tests:", token);

    // Create a parent idea for use in sub-idea tests
    // Make sure to pass global.dummyUserId to createParentIdea function
    parentIdeaId = await createParentIdea(global.dummyUserId);
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
      // The author field is now automatically set by the middleware based on the token, so it's not needed here
    };

    console.log("Sending request to create a new idea", newIdea);
    const response = await request(app)
      .post('/api/ideas')
      .set('Authorization', `Bearer ${token}`)
      .send(newIdea);

    console.log("Received response from create idea request", response.body);

    expect(response.statusCode).toBe(201);
    expect(response.body.title).toBe(newIdea.title);
    expect(response.body.author).toBeTruthy(); // This checks if an author ID is present
  });

  test('Create a new sub-idea under a parent idea - Success', async () => {
    const newSubIdea = {
      title: 'Test Sub Idea',
      description: 'This is a test sub-idea',
      parentId: parentIdeaId, // Ensure this matches your model's field for linking to a parent idea
    };

    console.log("Sending request to create a new sub-idea", newSubIdea);
    const response = await request(app)
      .post('/api/ideas') // Ensure this is the correct endpoint for creating a sub-idea. Adjust if necessary.
      .set('Authorization', `Bearer ${token}`)
      .send(newSubIdea);

    console.log("Received response from create sub-idea request", response.body);

    expect(response.statusCode).toBe(201);
    expect(response.body.title).toBe(newSubIdea.title);
    expect(response.body.parentId.toString()).toBe(parentIdeaId.toString()); // Verify the sub-idea is linked to the correct parent
  });

  // Additional CRUD operation tests for sub-ideas can follow here
});
