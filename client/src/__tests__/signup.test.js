import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import Signup from '../components/signup'; // Ensure the path to your Signup component is correct
import axios from 'axios';

jest.mock('axios');

describe('Signup Component', () => {
  beforeEach(() => {
    // Clear mock call history between tests
    axios.post.mockClear();
  });

  test('renders the signup form with email, password, and username fields and a submit button', () => {
    render(<Signup />);
    expect(screen.getByLabelText(/email/i)).toBeInTheDocument();
    expect(screen.getByLabelText(/password/i)).toBeInTheDocument();
    expect(screen.getByLabelText(/username/i)).toBeInTheDocument(); // Ensure username field is checked
    expect(screen.getByRole('button', { name: /signup/i })).toBeInTheDocument();
  });

  test('allows entering email, password, and username', () => {
    render(<Signup />);
    fireEvent.change(screen.getByLabelText(/email/i), { target: { value: 'user@example.com' } });
    fireEvent.change(screen.getByLabelText(/password/i), { target: { value: 'password' } });
    fireEvent.change(screen.getByLabelText(/username/i), { target: { value: 'username' } }); // Simulate entering a username
    expect(screen.getByLabelText(/email/i).value).toBe('user@example.com');
    expect(screen.getByLabelText(/password/i).value).toBe('password');
    expect(screen.getByLabelText(/username/i).value).toBe('username'); // Check if username input reflects the change
  });

  test('submits the form with email, password, and username', async () => {
    axios.post.mockResolvedValue({ data: { message: 'Signup successful' } });

    render(<Signup />);
    fireEvent.change(screen.getByLabelText(/email/i), { target: { value: 'user@example.com' } });
    fireEvent.change(screen.getByLabelText(/password/i), { target: { value: 'password' } });
    fireEvent.change(screen.getByLabelText(/username/i), { target: { value: 'username' } }); // Simulate entering a username
    fireEvent.click(screen.getByRole('button', { name: /signup/i }));

    await waitFor(() => {
      // Verify the mock Axios post was called with the correct data
      expect(axios.post).toHaveBeenCalledWith('http://localhost:5000/auth/register', {
        email: 'user@example.com',
        password: 'password',
        username: 'username', // Include username in the expected data
      });
    });
  });
});
