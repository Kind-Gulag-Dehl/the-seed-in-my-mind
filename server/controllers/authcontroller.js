const User = require('../models/user'); // Ensure the path matches your project structure
const bcrypt = require('bcryptjs');
const jwt = require('jsonwebtoken');

exports.registerUser = async (req, res) => {
    console.log("Received registration request:", req.body);
    const { username, email, password } = req.body;

    try {
        console.log(`Attempting to find user with email: ${email}`);
        let user = await User.findOne({ email });
        if (user) {
            console.log('User already exists with email:', email);
            return res.status(400).json({ message: 'User already exists' });
        }

        console.log(`Creating new user: ${username}`);
        user = new User({
            username,
            email,
            password,
        });

        await user.save();
        console.log('User saved successfully with ID:', user._id);

        const token = jwt.sign(
            { userId: user._id },
            process.env.JWT_SECRET,
            { expiresIn: '1h' }
        );
        console.log('Token created:', token);

        res.status(201).json({ message: 'User registered successfully', token });
    } catch (error) {
        console.error('Error registering user:', error);
        res.status(500).json({ message: 'Server error' });
    }
};

// authcontroller.js
exports.loginUser = async (req, res) => {
    console.log("Received login request:", req.body);
    const { email, password } = req.body;

    try {
        console.log(`Attempting to find user with email: ${email}`);
        const user = await User.findOne({ email });
        if (!user) {
            console.log('No user found with email:', email);
            return res.status(400).json({ message: 'Invalid Credentials' });
        }

        console.log(`Checking password for user: ${user.email}`);
        const isMatch = await bcrypt.compare(password, user.password);
        if (!isMatch) {
            console.log('Password does not match for user:', user.email);
            return res.status(400).json({ message: 'Invalid Credentials' });
        }

        console.log('Password matches, creating token.');
        const token = jwt.sign(
            { userId: user._id },
            process.env.JWT_SECRET,
            { expiresIn: '1h' }
        );
        console.log('Token created:', token);

        // Include user ID in the response
        console.log('Token and userId being sent:', { token, userId: user._id.toString() });
        res.json({ token, userId: user._id.toString() }); // Ensure to convert ObjectId to string if necessary
    } catch (error) {
        console.error('Error logging in user:', error);
        res.status(500).send('Server error');
    }
};
