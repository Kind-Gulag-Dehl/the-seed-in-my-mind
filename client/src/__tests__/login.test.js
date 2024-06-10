// In your login.test.js
import React from 'react';
import { Provider } from 'react-redux';
import { render, fireEvent, waitFor } from '@testing-library/react';
import { MemoryRouter } from 'react-router-dom';
import { configureStore } from '@reduxjs/toolkit';
import userReducer from '../features/user/userslice';
import Login from '../components/login';

// No need to mock useHistory if you're not testing navigation explicitly

describe('Login Component', () => {
  let store;

  beforeEach(() => {
    store = configureStore({
      reducer: {
        user: userReducer,
      },
    });
  });

  const setup = () => render(
    <Provider store={store}>
      <MemoryRouter>
        <Login />
      </MemoryRouter>
    </Provider>
  );

  test('renders login form with email and password inputs', () => {
    const { getByLabelText } = setup();
    // Assuming labels contain 'Email' and 'Password' exactly
    expect(getByLabelText('Email')).toBeInTheDocument();
    expect(getByLabelText('Password')).toBeInTheDocument();
  });

  test('allows entering email and password', () => {
    const { getByLabelText } = setup();
    // Correct the query to match exact label text if necessary
    const emailInput = getByLabelText('Email');
    const passwordInput = getByLabelText('Password');

    fireEvent.change(emailInput, { target: { value: 'test@example.com' } });
    fireEvent.change(passwordInput, { target: { value: 'password' } });

    expect(emailInput.value).toBe('test@example.com');
    expect(passwordInput.value).toBe('password');
  });

  test('submits the form and dispatches loginUser action', async () => {
    const { getByLabelText, getByRole } = setup();
    // Ensure these match the labels' inner text exactly
    const emailInput = getByLabelText('Email');
    const passwordInput = getByLabelText('Password');
    
    // Using getByRole to select the button, ensure button text matches exactly
    const loginButton = getByRole('button', { name: /Login/i });

    fireEvent.change(emailInput, { target: { value: 'test@example.com' } });
    fireEvent.change(passwordInput, { target: { value: 'password' } });
    fireEvent.click(loginButton);

    await waitFor(() => {
      // Modify this assertion according to what state change you expect after login
      // This might need to be adjusted based on your application's logic
    });
  });
});
