import React, { useState } from 'react';
import axios from 'axios';

function Signup() {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [username, setUsername] = useState(''); // Add a state for username

  const handleSubmit = async (e) => {
    e.preventDefault();
    try {
      // Include username in the POST request body
      const response = await axios.post('http://localhost:5000/api/auth/register', { email, password, username });
      console.log(response.data);
      // Handle signup success
    } catch (error) {
      console.error("Signup error:", error.response ? error.response.data : "Server error");
      // Handle signup error
    }
  };

  return (
    <form onSubmit={handleSubmit}>
      <h2>Signup</h2>
      <div>
        <label htmlFor="signup-email">Email:</label>
        <input id="signup-email" type="email" value={email} onChange={(e) => setEmail(e.target.value)} required />
      </div>
      <div>
        <label htmlFor="signup-username">Username:</label>
        <input id="signup-username" type="text" value={username} onChange={(e) => setUsername(e.target.value)} required />
      </div>
      <div>
        <label htmlFor="signup-password">Password:</label>
        <input id="signup-password" type="password" value={password} onChange={(e) => setPassword(e.target.value)} required />
      </div>
      <button type="submit">Signup</button>
    </form>
  );
}

export default Signup;
