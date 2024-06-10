import ideasReducer, { fetchIdeas } from '../features/ideas/ideaslice';
import { configureStore } from '@reduxjs/toolkit';
import axios from 'axios';

jest.mock('axios');

describe('ideasSlice', () => {
  let store;

  beforeEach(() => {
    store = configureStore({ reducer: { ideas: ideasReducer } });
    axios.get.mockClear();
  });

  it('should handle initial state', () => {
    expect(store.getState().ideas).toEqual({
      ideasList: [],
      status: 'idle',
      error: null,
    });
  });

  it('handles fetchIdeas thunk for pending, fulfilled, and rejected states', async () => {
    const mockIdeas = [{ id: 1, title: 'Test Idea', description: 'Test idea description.' }];

    // Test the pending state
    const pendingAction = fetchIdeas.pending.type;
    let newState = ideasReducer(undefined, { type: pendingAction });
    expect(newState).toEqual({
      ideasList: [],
      status: 'loading',
      error: null,
    });

    // Test the fulfilled state
    axios.get.mockResolvedValueOnce({ data: mockIdeas });
    await store.dispatch(fetchIdeas());
    newState = store.getState().ideas;
    expect(newState).toEqual({
      ideasList: mockIdeas,
      status: 'succeeded',
      error: null,
    });

    // Test the rejected state
    const error = 'Fetch failed';
    axios.get.mockRejectedValueOnce(new Error(error));
    await store.dispatch(fetchIdeas());
    newState = store.getState().ideas;
    expect(newState.status).toEqual('failed');
    // Note: The exact error message checking might vary depending on how you handle the error in your slice.
  });
});
