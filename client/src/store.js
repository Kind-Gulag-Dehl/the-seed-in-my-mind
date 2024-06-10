import { configureStore } from '@reduxjs/toolkit';
import userReducer from './features/user/userslice';
import ideasReducer from './features/ideas/ideaslice';

export const store = configureStore({
  reducer: {
    user: userReducer,
    ideas: ideasReducer,
  },
});
