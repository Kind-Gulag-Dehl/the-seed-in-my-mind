import usersReducer, { loginUser, logoutUser } from '../features/user/userslice';
import { configureStore } from '@reduxjs/toolkit';
import axios from 'axios';

jest.mock('axios');

describe('userSlice', () => {
  let store;

  beforeEach(() => {
    store = configureStore({ reducer: { user: usersReducer } });
  });

  describe('reducers and actions', () => {
    it('should return the initial state', () => {
      expect(store.getState().user).toEqual({
        user: null,
        token: null,
        status: 'idle',
        error: null,
      });
    });

    it('should handle logoutUser action', () => {
      store.dispatch(logoutUser());
      expect(store.getState().user).toEqual({
        user: null,
        token: null,
        status: 'idle',
        error: null,
      });
    });
  });

  describe('loginUser async thunk', () => {
    it('handles successful loginUser', async () => {
      const mockUserData = { email: 'test@example.com', password: 'password' };
      const mockResponseData = { user: { id: '123', name: 'John Doe' }, token: 'fakeToken' };
      
      axios.post.mockResolvedValueOnce({ data: mockResponseData });

      await store.dispatch(loginUser(mockUserData));

      const state = store.getState().user;
      expect(state.user).toEqual(mockResponseData.user);
      expect(state.token).toEqual(mockResponseData.token);
      expect(state.status).toEqual('succeeded');
    });

    // Here you could add more tests for handling login failure, etc.
  });
});
