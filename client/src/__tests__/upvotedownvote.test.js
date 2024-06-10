import React from 'react';
import axios from 'axios';
import '@testing-library/jest-dom';
import { render, fireEvent, waitFor } from '@testing-library/react';
import Idea from '../components/Idea'; // Import your Idea component

// Mock Axios to prevent actual network requests
jest.mock('axios');

describe('Idea Upvote and Downvote', () => {
  const token = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IlRlc3QgVXNlciIsImlhdCI6MTUxNjIzOTAyMn0.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c'; // Replace with a real token for testing
  const ideaId = '6617fe5f5dae5a076f7d0522'; // Replace with a real idea ID for testing
  const userId = '65f3708ffdef9143c057d5f4'; // Replace with a real user ID for testing

  beforeEach(() => {
    // Setup localStorage mock
    Storage.prototype.getItem = jest.fn(() => token);
    axios.patch.mockClear();
  });

  test('renders the idea component with upvote and downvote buttons', () => {
    const { getByText } = render(<Idea ideaId={ideaId} />);
    expect(getByText(/upvote/i)).toBeInTheDocument();
    expect(getByText(/downvote/i)).toBeInTheDocument();
  });

  test('sends an upvote request when the upvote button is clicked', async () => {
    axios.patch.mockResolvedValueOnce({
      data: { upvotes: [userId] }, // Mock response of updated upvotes array
    });

    const { getByText } = render(<Idea ideaId={ideaId} />);
    const upvoteButton = getByText(/upvote/i);

    fireEvent.click(upvoteButton);

    await waitFor(() => {
      expect(axios.patch).toHaveBeenCalledWith(
        `http://localhost:5000/api/ideas/${ideaId}/upvote`,
        {},
        {
          headers: {
            Authorization: `Bearer ${token}`,
          },
        }
      );
    });
  });

  test('sends a downvote request when the downvote button is clicked', async () => {
    axios.patch.mockResolvedValueOnce({
      data: { downvotes: [userId] }, // Mock response of updated downvotes array
    });

    const { getByText } = render(<Idea ideaId={ideaId} />);
    const downvoteButton = getByText(/downvote/i);

    fireEvent.click(downvoteButton);

    await waitFor(() => {
      expect(axios.patch).toHaveBeenCalledWith(
        `http://localhost:5000/api/ideas/${ideaId}/downvote`,
        {},
        {
          headers: {
            Authorization: `Bearer ${token}`,
          },
        }
      );
    });
  });
});
