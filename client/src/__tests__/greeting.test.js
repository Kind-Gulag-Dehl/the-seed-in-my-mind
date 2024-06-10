// src/__tests__/components/greeting.test.js

/**
 * @jest-environment jsdom
 */

import React from 'react';
import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import Greeting from '../components/greeting';

test('renders the correct content', () => {
  render(<Greeting />);
  expect(screen.getByText('Hello, world!')).toBeInTheDocument();
});
