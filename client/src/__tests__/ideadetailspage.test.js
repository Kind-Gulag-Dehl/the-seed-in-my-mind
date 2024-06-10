import React from 'react';
import { render } from '@testing-library/react';
import axios from 'axios';
import IdeaDetailsPage from '../pages/ideadetailspage';
import { MemoryRouter } from 'react-router-dom';

jest.mock('axios');

jest.mock('react-router-dom', () => ({
  ...jest.requireActual('react-router-dom'),
  useHistory: () => ({
    push: jest.fn(),
  }),
  useParams: () => ({
    ideaId: '1',
  }),
}));

describe('IdeaDetailsPage Component', () => {
  beforeEach(() => {
    axios.get.mockClear();
  });

  it('fetches and displays idea details including sub-ideas', async () => {
    const mockIdea = {
      _id: '1',
      title: 'Unique Parent Idea Title',
      description: 'Unique Parent Idea Description',
      subIdeas: [{
        _id: '2',
        title: 'Unique Sub-Idea Title',
        description: 'Unique Sub Idea Description',
        subIdeas: []
      }]
    };
    axios.get.mockResolvedValueOnce({ data: mockIdea });

    const { findByText, queryAllByTestId } = render(
      <MemoryRouter>
        <IdeaDetailsPage />
      </MemoryRouter>
    );
    
    // Assert on the parent idea details being present
    const parentTitleElement = await findByText(mockIdea.title);
    expect(parentTitleElement).toBeInTheDocument();
    const parentDescriptionElement = await findByText(mockIdea.description);
    expect(parentDescriptionElement).toBeInTheDocument();

    // Check for the presence of any sub-idea sections
    const subIdeaSections = queryAllByTestId((content, element) => 
      element.getAttribute('data-testid')?.startsWith('sub-idea-section')
    );
    expect(subIdeaSections.length).toBeGreaterThan(0); // Expecting at least one sub-idea section to be present

    // Assert on the sub-idea details being present
    const subIdeaTitleElement = await findByText(mockIdea.subIdeas[0].title);
    expect(subIdeaTitleElement).toBeInTheDocument();
    const subIdeaDescriptionElement = await findByText(mockIdea.subIdeas[0].description);
    expect(subIdeaDescriptionElement).toBeInTheDocument();
  });
});
