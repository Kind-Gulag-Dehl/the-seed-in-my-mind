import React, { useState } from 'react';
import axios from 'axios';
import { useNavigate } from 'react-router-dom';

const LoginPage = () => {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');
  const navigate = useNavigate();

  const handleLogin = async (event) => {
    event.preventDefault();
    try {
      const response = await axios.post('/api/auth/login', { email, password });
      console.log("Response from login:", response.data);   // Check what is received from the server
      if(response.data.token && response.data.userId) {
        localStorage.setItem('token', response.data.token);
        console.log('Logged in User ID:', localStorage.getItem('userId'));
        localStorage.setItem('userId', response.data.userId);
        navigate('/'); // Redirect to homepage after successful login
      } else {
        throw new Error('Missing token or userId from the response');
      }
    } catch (err) {
      console.error('Login Error:', err);
      setError('Failed to login. Please check your credentials and try again.');
    }
  };

  return (
    <div>
      <h1>Login</h1>
      {error && <p className="error">{error}</p>}
      <form onSubmit={handleLogin}>
        <div>
          <label htmlFor="email">Email</label>
          <input
            type="email"
            id="email"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            required
          />
        </div>
        <div>
          <label htmlFor="password">Password</label>
          <input
            type="password"
            id="password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            required
          />
        </div>
        <button type="submit">Login</button>
      </form>
    </div>
  );
};

export default LoginPage;
