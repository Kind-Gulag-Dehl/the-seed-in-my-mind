import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import NavigationBar from './components/navigationbar'; // Added Navigation Bar to all pages
import ProtectedRoute from './utils/protectedroute';
import HomePage from './pages/homepage';
import IdeaSubmissionForm from './components/ideasubmissionform';
import IdeaDetailsPage from './pages/ideadetailspage';
import SubDetailsPage from './pages/subdetailspage';
import Login from './components/login';
import Signup from './components/signup';
import IdeaSubmission from './pages/ideasubmissionpage';
import LoginPage from './pages/loginpage';
import ProfilePage from './pages/profilepage';
import NewVotePage from './pages/newvotepage'; // Added VotePage component
import ChallengeDetailsPage from './pages/challengedetailspage';

// Assuming you have a way to check if the user is authenticated
const isAuthenticated = true; // This should be replaced with your actual authentication check

function App() {
  return (
    <Router>
      <NavigationBar />
      <Routes>
        {/* Home Page */}
        <Route path="/" element={<ProtectedRoute isAuthenticated={isAuthenticated}><HomePage /></ProtectedRoute>} />

        {/* Idea Submission - Assuming this needs to be protected */}
        <Route path="/submit-idea" element={<ProtectedRoute isAuthenticated={isAuthenticated}><IdeaSubmissionForm /></ProtectedRoute>} />

        {/* Idea Details Page - Assuming it needs to be protected */}
        <Route path="/ideas/:ideaId" element={<ProtectedRoute isAuthenticated={isAuthenticated}><IdeaDetailsPage /></ProtectedRoute>} />

        {/* Detailed Idea Page - Protected route */}
        <Route path="/idea-details/:ideaId" element={<ProtectedRoute isAuthenticated={isAuthenticated}><SubDetailsPage /></ProtectedRoute>} />

        {/* Login Page - No protection needed */}
        <Route path="/login" element={<Login />} />

        {/* Signup Page - No protection needed */}
        <Route path="/signup" element={<Signup />} />

        {/* Additional Routes */}
        <Route path="/submit-idea-page" element={<ProtectedRoute isAuthenticated={isAuthenticated}><IdeaSubmission /></ProtectedRoute>} />
        <Route path="/login-page" element={<LoginPage />} />
        <Route path="/profile" element={<ProfilePage />} />
        <Route path="/newvote" element={<NewVotePage />} /> {/* Updated the path and component for the vote page */}
        <Route path="/challenge/:challengeId" element={<ChallengeDetailsPage />} />

      </Routes>
    </Router>
  );
}

export default App;
