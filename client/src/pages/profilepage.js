import React from 'react';
import { useSelector } from 'react-redux';

const ProfilePage = () => {
    // Using useSelector to pull user data from Redux store
    const user = useSelector(state => state.user);

    if (!user) {
        return <div>Loading user data...</div>; // or any other loading state representation
    }

    return (
        <div className="profile-page">
            <h1>Profile</h1>
            <div className="profile-content">
                <p><strong>Username:</strong> {user.username}</p>
                <p><strong>Email:</strong> {user.email}</p>
                <p><strong>Bio:</strong> {user.bio}</p>
            </div>
        </div>
    );
};

export default ProfilePage;
