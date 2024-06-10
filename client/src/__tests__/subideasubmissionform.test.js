// subideasubmissionform.test.js located in client/src/__tests__

import React from 'react';
import { render, fireEvent, waitFor, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import axios from 'axios';
import SubIdeaSubmissionForm from '../components/subideasubmissionform';

// Mock axios to prevent actual HTTP requests during tests
jest.mock('axios');

describe('SubIdeaSubmissionForm', () => {
  beforeEach(() => {
    // Setup necessary for each test, if any
  });

  it('renders correctly', () => {
    render(<SubIdeaSubmissionForm parentId="someParentId" />);
    expect(screen.getByLabelText(/Title/i)).toBeInTheDocument();
    expect(screen.getByLabelText(/Description/i)).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Submit Sub-Idea/i })).toBeInTheDocument();
  });

  it('submits form with title, description, and parentId', async () => {
    const mockPost = axios.post.mockResolvedValue({ data: { message: 'Sub-Idea submitted successfully' } });

    render(<SubIdeaSubmissionForm parentId="someParentId" />);
    
    // Fill out the form
    fireEvent.change(screen.getByLabelText(/Title/i), { target: { value: 'Sub-Idea Title' } });
    fireEvent.change(screen.getByLabelText(/Description/i), { target: { value: 'Sub-Idea Description' } });

    // Submit the form
    fireEvent.click(screen.getByRole('button', { name: /Submit Sub-Idea/i }));

    // Wait for the post request to complete
    await waitFor(() => expect(mockPost).toHaveBeenCalledWith(
      'http://localhost:5000/api/ideas/sub', // Ensure this matches your API endpoint
      {
        title: 'Sub-Idea Title',
        description: 'Sub-Idea Description',
        parentId: 'someParentId'
      },
      expect.objectContaining({
        headers: expect.objectContaining({
          Authorization: expect.any(String),
        }),
      })
    ));
  });

  // Add more tests as needed
});
