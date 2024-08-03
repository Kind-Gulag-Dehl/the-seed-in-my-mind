// Import necessary modules
const express = require('express');
const cors = require('cors');
const connectDB = require('./server/config/db');
const path = require('path');

// Load environment variables
require('dotenv').config();

//scheduler
require('./scheduler');

// Initialize Express
const app = express();

// Use CORS middleware to allow requests from the frontend
app.use(cors({
  origin: 'http://localhost:3000' // Adjust as necessary for deployment
}));

// Middleware to parse JSON bodies
app.use(express.json({ extended: false }));

// Import the auth middleware
const { protect } = require('./server/middleware/authmiddleware');

// Import routes
const challengeRoutes = require('./server/routes/challengeroutes');
const argumentRoutes = require('./server/routes/argumentroutes');

// API Routes - Prefix all API routes with '/api'
console.log("Registering /api/auth routes");
app.use('/api/auth', require('./server/routes/authroutes'));

// Protected routes for voting
console.log("Registering protected /api/ideas/vote routes");
app.use('/api/ideas/vote', protect, require('./server/routes/voteroutes')); // Assuming voteRoutes handle only voting related routes

// Challenge routes
console.log("Registering /api/challenges routes");
app.use('/api/challenges', challengeRoutes); // This will attach your challenge API under /api/challenges

// Argument routes
console.log("Registering /api/arguments routes");
app.use('/api/arguments', protect, argumentRoutes);

require('./scheduler'); // Assuming the scheduler.js is in the root directory


// Regular idea routes (unprotected)
console.log("Registering /api/ideas routes");
app.use('/api/ideas', require('./server/routes/idearoutes'));

// Serve static files from the React build directory
app.use(express.static(path.join(__dirname, 'client', 'build')));

// Fallback to serve your React app for any other GET route not recognized by the server
// It's important to place this after your API routes, otherwise, it will catch all requests
app.get('*', (req, res) => {
  res.sendFile(path.join(__dirname, 'client', 'build', 'index.html'));
});

// Conditional Database Connection - Only if not in test mode
if (process.env.NODE_ENV !== 'test') {
  connectDB();
}




// Start server only if not in test mode
if (process.env.NODE_ENV !== 'test') {
  const PORT = process.env.PORT || 5000; // Default to 5000 unless specified otherwise
  app.listen(PORT, () => console.log(`Server started on port ${PORT}`));
} else {
  console.log("Server running in test mode, not listening on any port");
}

// Export the app for testing purposes
module.exports = app;
