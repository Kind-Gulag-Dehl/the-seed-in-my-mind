import { store } from '../store';
import { fetchIdeas } from '../features/ideas/ideaslice';
import axios from 'axios';

jest.mock('axios');

describe('Redux Store', () => {
  beforeEach(() => {
    axios.get.mockClear();
  });

  it('should initialize with the correct initial state', () => {
    const state = store.getState();
    expect(state.ideas).toBeDefined();
    expect(state.user).toBeDefined();
    // Initial state specifics
    expect(state.ideas.ideasList).toEqual([]);
    expect(state.ideas.status).toEqual('idle');
  });

  it('handles fetchIdeas action successfully', async () => {
    const mockIdeas = []; // Assuming an empty array for the mock response
    axios.get.mockResolvedValue({ data: mockIdeas });

    await store.dispatch(fetchIdeas());
    const state = store.getState();

    expect(state.ideas.ideasList).toEqual(mockIdeas);
    expect(state.ideas.status).toEqual('succeeded');
  });

  it('handles fetchIdeas action with a failure', async () => {
    axios.get.mockRejectedValue(new Error('Failed to fetch'));

    await store.dispatch(fetchIdeas());
    const state = store.getState();

    expect(state.ideas.ideasList).toEqual([]); // Assuming initial state doesn't change
    expect(state.ideas.status).toEqual('failed');
  });
});
