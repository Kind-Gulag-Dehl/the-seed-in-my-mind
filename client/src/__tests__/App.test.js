import React from 'react';
import { Provider } from 'react-redux';
import { MemoryRouter, Routes, Route } from 'react-router-dom';
import { render, screen } from '@testing-library/react';
// Import individual components that are routed in your App component
import HomePage from '../pages/homepage';
import IdeaSubmissionForm from '../components/ideasubmissionform';
import IdeaDetailsPage from '../pages/ideadetailspage';
import Login from '../components/login';
import Signup from '../components/signup';
import IdeaSubmission from '../pages/ideasubmissionpage';
import LoginPage from '../pages/loginpage';

// Define your mock Redux store with an accurate initial state
const initialState = {
  user: {
    status: 'idle',
    error: null,
  },
  // Add other slices of state as needed
};

// Mock Redux store
const mockStore = {
  getState: jest.fn(() => initialState),
  dispatch: jest.fn(),
  subscribe: jest.fn(),
};

describe('Component Rendering with Routes', () => {
  beforeEach(() => {
    // Reset mocks before each test
    mockStore.dispatch.mockClear();
    mockStore.getState.mockClear().mockReturnValue(initialState);
  });

  const renderComponentWithRoute = (component, initialEntry) => {
    render(
      <Provider store={mockStore}>
        <MemoryRouter initialEntries={[initialEntry]}>
          <Routes>
            <Route path={initialEntry} element={component} />
          </Routes>
        </MemoryRouter>
      </Provider>
    );
  };

  it('renders the login component when visiting /login', async () => {
    renderComponentWithRoute(<Login />, '/login');
    // Update your assertions to match the actual login page content
  });

  it('renders the signup component when visiting /signup', async () => {
    renderComponentWithRoute(<Signup />, '/signup');
    // Update your assertions to match the actual signup page content
  });

  it('renders the home page component when visiting /', async () => {
    renderComponentWithRoute(<HomePage />, '/');
    // Update your assertions to match the actual home page content
  });

  it('renders the idea submission form component when visiting /submit-idea', async () => {
    renderComponentWithRoute(<IdeaSubmissionForm />, '/submit-idea');
    // Update your assertions to match the actual idea submission form content
  });

  // Repeat for other routes/components as needed
});
