const request = require('supertest');
const app = require('../index'); // Adjust this path to correctly point to your server's entry file

describe('Authentication Flow Tests', () => {
  // Test user registration
  test('User Registration', async () => {
    console.log("Testing User Registration");
    const userCredentials = {
      username: "testUser",
      email: "test@testing.com",
      password: "hardcodedPassword",
    };

    const res = await request(app)
      .post('/api/auth/register') // Corrected route path
      .send(userCredentials);
    console.log("Registration response:", res.body);

    // Check for successful registration response
    expect(res.statusCode).toEqual(201);
    expect(res.body).toHaveProperty('message', 'User registered successfully');
  });

  // Test user login and token retrieval
  test('User Login and Token Retrieval', async () => {
    console.log("Testing User Login and Token Retrieval");
    const userCredentials = {
      email: "test@testing.com",
      password: "hardcodedPassword",
    };

    const res = await request(app)
      .post('/api/auth/login')
      .send(userCredentials);
    console.log("Login response:", res.body);

    // Check for successful login response and token presence
    expect(res.statusCode).toEqual(200);
    expect(res.body).toHaveProperty('token');
    expect(res.body.token).toBeDefined();

    // Check if the token structure is valid (contains 3 parts separated by dots)
    expect(res.body.token.split('.').length).toEqual(3);
  });

  // Further tests can follow here
});
