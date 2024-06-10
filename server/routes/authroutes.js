const express = require('express');
const router = express.Router();
const { registerUser, loginUser } = require('../controllers/authcontroller');

// Middleware for logging route hits
const logRouteAccess = (routeName) => (req, res, next) => {
  console.log(`Accessing ${routeName} route`);
  next();
};

// Route for user registration
router.post('/register', logRouteAccess('/register'), registerUser);

// Route for user login
router.post('/login', logRouteAccess('/login'), loginUser);

module.exports = router;
