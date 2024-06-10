// actions/userActions.js
export const fetchUserDetails = () => async (dispatch) => {
  dispatch({ type: 'FETCH_USER_DETAILS_REQUEST' });
  try {
    // Simulate fetching user details (you can replace this with your actual API call)
    const userDetails = { name: 'John Doe', id: 1 };
    dispatch({ type: 'FETCH_USER_DETAILS_SUCCESS', payload: userDetails });
  } catch (error) {
    dispatch({ type: 'FETCH_USER_DETAILS_FAILURE', payload: error.message });
  }
};
