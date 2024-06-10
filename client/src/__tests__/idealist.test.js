import React from 'react';
import { render, waitFor } from '@testing-library/react';
import axios from 'axios';
import IdeasList from '../components/idealist';
import { BrowserRouter } from 'react-router-dom'; // Import BrowserRouter for wrapping

// Mock Axios to prevent actual network requests
jest.mock('axios');

// Wrap IdeasList component in BrowserRouter for the tests
const renderWithRouter = (ui, { route = '/' } = {}) => {
  window.history.pushState({}, 'Test page', route);

  return render(ui, { wrapper: BrowserRouter });
};

describe('IdeasList Component', () => {
  it('displays ideas after successful fetch', async () => {
    // Mock Axios response
    const mockIdeas = [
      { _id: '1', title: 'Idea 1', description: 'Description for Idea 1' },
      { _id: '2', title: 'Idea 2', description: 'Description for Idea 2' },
    ];
    axios.get.mockResolvedValueOnce({ data: mockIdeas });

    const { getByText } = renderWithRouter(<IdeasList />);
    
    // Wait for the ideas to be displayed
    await waitFor(() => {
      expect(getByText('Idea 1')).toBeInTheDocument();
      expect(getByText('Description for Idea 1')).toBeInTheDocument();
      expect(getByText('Idea 2')).toBeInTheDocument();
      expect(getByText('Description for Idea 2')).toBeInTheDocument();
    });
  });

  it('displays a message when no ideas are submitted yet', async () => {
    // Mock Axios response to return an empty array
    axios.get.mockResolvedValueOnce({ data: [] });

    const { getByText } = renderWithRouter(<IdeasList />);
    
    // Wait for the "No ideas submitted yet." message to appear
    await waitFor(() => {
      expect(getByText('No ideas submitted yet.')).toBeInTheDocument();
    });
  });

  it('handles errors during fetch operation', async () => {
    // Mock a fetch error
    axios.get.mockRejectedValueOnce(new Error('Failed to fetch'));

    const { getByText, queryByText } = renderWithRouter(<IdeasList />);
    
    // Wait for the error message to be displayed. Ensure the text matches exactly what your component renders.
    await waitFor(() => {
      expect(queryByText(/No ideas submitted yet./i)).not.toBeInTheDocument(); // Ensure "No ideas" message is not shown when there's an error
      expect(getByText("Error: An error occurred while fetching ideas.")).toBeInTheDocument(); // This text should match the error message in your component
    });
  });
});
