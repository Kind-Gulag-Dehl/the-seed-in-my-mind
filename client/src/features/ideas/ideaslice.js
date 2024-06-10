// src/features/ideas/ideaslice.js
import { createSlice, createAsyncThunk } from '@reduxjs/toolkit';
import axios from 'axios';

// Async thunk for fetching ideas from your backend
export const fetchIdeas = createAsyncThunk('ideas/fetchIdeas', async () => {
  // Updated to use the environment variable for the base URL
  const baseUrl = process.env.REACT_APP_API_BASE_URL || 'http://localhost:5000';
  const response = await axios.get(`${baseUrl}/api/ideas`);
  return response.data; // The response data is expected to be the list of ideas
});

// Initial state for the ideas slice
const initialState = {
  ideasList: [],
  status: 'idle', // Represents the loading status ('idle', 'loading', 'succeeded', 'failed')
  error: null, // To store any error message in case of failure
};

// The ideas slice containing reducers and extra reducers for handling async thunks
const ideasSlice = createSlice({
  name: 'ideas',
  initialState,
  reducers: {
    // You can add reducers here for synchronous actions if needed
  },
  extraReducers: (builder) => {
    builder
      .addCase(fetchIdeas.pending, (state) => {
        state.status = 'loading';
      })
      .addCase(fetchIdeas.fulfilled, (state, action) => {
        state.status = 'succeeded';
        state.ideasList = action.payload; // Update the ideas list with fetched data
      })
      .addCase(fetchIdeas.rejected, (state, action) => {
        state.status = 'failed';
        state.error = action.error.message; // Store the error message
      });
  },
});

// Export the reducer as the default export of the file
export default ideasSlice.reducer;
