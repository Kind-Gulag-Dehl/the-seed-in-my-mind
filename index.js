// Import necessary modules
const express = require('express');
const connectDB = require('./server/config/db');

// Load environment variables
require('dotenv').config();

// Initialize Express
const app = express();

// Middleware
app.use(express.json({ extended: false }));

// Define a simple route for testing
app.get('/', (req, res) => res.send('API Running'));

// Define routes
app.use('/ideas', require('./server/routes/idearoutes'));

// UPDATED: Import the auth router with the testing route included
// Make sure the path matches the location of your updated authcontroller.js
app.use('/auth', require('./server/routes/authroutes'));

// Conditional Database Connection
// Only connect to MongoDB if not in a test environment
if (process.env.NODE_ENV !== 'test') {
  connectDB();
}

// Conditional check to prevent auto-listening during tests
if (process.env.NODE_ENV !== 'test') {
  const PORT = process.env.PORT || 5000;
  app.listen(PORT, () => console.log(`Server started on port ${PORT}`));
}

module.exports = app; // Export the app for testing purposes
