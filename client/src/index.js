import React from 'react';
import ReactDOM from 'react-dom';
import App from './App'; // Adjust the path if necessary
import { Provider } from 'react-redux';
import { store } from './store'; // Ensure this points to your store.js file

ReactDOM.render(
  <React.StrictMode>
    <Provider store={store}>
      <App />
    </Provider>
  </React.StrictMode>,
  document.getElementById('root')
);
