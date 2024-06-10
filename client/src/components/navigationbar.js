import React from 'react';
import { Link } from 'react-router-dom';
import './navigationbar.css'; // Correct import for the updated CSS styles

const NavigationBar = () => {
    return (
        <nav>
            <ul>
                <li><Link to="/" className="nav-link">Home</Link></li>
                <li><Link to="/profile" className="nav-link">Profile</Link></li>
                <li><Link to="/login" className="nav-link">Login</Link></li>
                <li><Link to="/newvote" className="nav-link">Vote</Link></li>  {/* Added link to the vote page */}
            </ul>
        </nav>
    );
};

export default NavigationBar;
