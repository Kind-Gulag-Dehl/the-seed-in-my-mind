const jwt = require('jsonwebtoken');

const protect = (req, res, next) => {
  let token;

  if (req.headers.authorization && req.headers.authorization.startsWith('Bearer')) {
    console.log('Authorization Header:', req.headers.authorization); // Confirm the header is received
    
    try {
      token = req.headers.authorization.split(' ')[1];
      console.log('Extracted Token:', token); // Confirm the token is correctly split

      const decoded = jwt.verify(token, process.env.JWT_SECRET);
      console.log("JWT_SECRET used for verification:", process.env.JWT_SECRET);
      console.log('Decoded Token:', decoded); // Check the structure of decoded token

      req.user = { userId: decoded.userId }; // Ensure this matches the structure of the decoded token
      console.log('User set in req.user:', req.user); // Confirm user is set correctly

      next();
    } catch (error) {
      console.error('Token verification error:', error);
      res.status(401).json({ message: 'Not authorized, token failed' });
    }
  } else {
    console.log('No token found in authorization header.');
    res.status(401).json({ message: 'Not authorized, no token' });
  }
};

module.exports = { protect };
