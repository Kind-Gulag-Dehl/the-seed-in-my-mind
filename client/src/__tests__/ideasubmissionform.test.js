import React from 'react';
import { render, fireEvent, waitFor } from '@testing-library/react';
import '@testing-library/jest-dom';
import axios from 'axios';
import IdeaSubmissionForm from '../components/ideasubmissionform';

// Mock Axios to prevent actual network requests
jest.mock('axios');

describe('IdeaSubmissionForm Component', () => {
  beforeEach(() => {
    // Clear all mocks between tests
    axios.post.mockClear();
  });

  test('renders the idea submission form with title and description fields and a submit button', () => {
    const { getByLabelText, getByRole } = render(<IdeaSubmissionForm />);
    expect(getByLabelText(/title/i)).toBeInTheDocument();
    expect(getByLabelText(/description/i)).toBeInTheDocument();
    expect(getByRole('button', { name: /submit idea/i })).toBeInTheDocument();
  });

  test('allows entering a title and description', () => {
    const { getByLabelText } = render(<IdeaSubmissionForm />);
    const titleInput = getByLabelText(/title/i);
    const descriptionTextarea = getByLabelText(/description/i);

    fireEvent.change(titleInput, { target: { value: 'New Idea' } });
    fireEvent.change(descriptionTextarea, { target: { value: 'Description of the new idea.' } });

    expect(titleInput.value).toBe('New Idea');
    expect(descriptionTextarea.value).toBe('Description of the new idea.');
  });

  test('submits the form with title and description', async () => {
    const mockIdeaResponse = { data: { message: 'Idea submitted successfully' } };
    axios.post.mockResolvedValueOnce(mockIdeaResponse);

    const { getByLabelText, getByRole } = render(<IdeaSubmissionForm />);
    const titleInput = getByLabelText(/title/i);
    const descriptionTextarea = getByLabelText(/description/i);
    const submitButton = getByRole('button', { name: /submit idea/i });

    fireEvent.change(titleInput, { target: { value: 'New Idea' } });
    fireEvent.change(descriptionTextarea, { target: { value: 'Description of the new idea.' } });
    fireEvent.click(submitButton);

    await waitFor(() => {
      expect(axios.post).toHaveBeenCalledWith(
        'http://localhost:5000/api/ideas', 
        {
          title: 'New Idea',
          description: 'Description of the new idea.',
        },
        {
          headers: {
            Authorization: `Bearer ${localStorage.getItem('token')}` // Mocking the retrieval of the token for the header
          }
        }
      );
    });
  });
});
