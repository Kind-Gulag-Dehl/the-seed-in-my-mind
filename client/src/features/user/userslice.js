// src/features/user/userSlice.js

import { createSlice, createAsyncThunk } from '@reduxjs/toolkit';
import axios from 'axios';

// Async thunk for user login
export const loginUser = createAsyncThunk(
    'user/loginUser',
    async (userData, { rejectWithValue }) => {
        try {
            const response = await axios.post('http://localhost:5000/api/auth/login', userData);
            // Storing token
            localStorage.setItem('token', response.data.token);
            // Store user ID in localStorage
            localStorage.setItem('userId', response.data.userId); // Ensure this is being executed
            return { token: response.data.token, userId: response.data.userId };
        } catch (error) {
            return rejectWithValue(error.response.data);
        }
    }
);


const initialState = {
    user: null, // Adjust initial state to include user ID
    token: localStorage.getItem('token'),
    status: 'idle',
    error: null,
};

const userSlice = createSlice({
    name: 'user',
    initialState,
    reducers: {
        // Resets the user state
        logoutUser(state) {
            localStorage.removeItem('token');
            state.user = null;
            state.token = null;
            state.status = 'idle';
            state.error = null;
        },
    },
    extraReducers: (builder) => {
        builder
            .addCase(loginUser.pending, (state) => {
                state.status = 'loading';
            })
            .addCase(loginUser.fulfilled, (state, action) => {
                state.status = 'succeeded';
                // Store user ID
                state.user = { userId: action.payload.userId };
                state.token = action.payload.token;
            })
            .addCase(loginUser.rejected, (state, action) => {
                state.status = 'failed';
                state.error = action.payload;
            });
    },
});

export const { logoutUser } = userSlice.actions;
export default userSlice.reducer;
