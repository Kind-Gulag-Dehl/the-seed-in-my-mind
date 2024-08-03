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
import NewVotePage from './pages/newvotepage';
import ChallengeDetailsPage from './pages/challengedetailspage';
import ImportancePage from './pages/importancepage'; // Import the ImportancePage component

const isAuthenticated = true; // This should be replaced with your actual authentication check

function App() {
  return (
    <Router>
      <NavigationBar />
      <Routes>
        <Route path="/" element={<ProtectedRoute isAuthenticated={isAuthenticated}><HomePage /></ProtectedRoute>} />
        <Route path="/submit-idea" element={<ProtectedRoute isAuthenticated={isAuthenticated}><IdeaSubmissionForm /></ProtectedRoute>} />
        <Route path="/ideas/:ideaId" element={<ProtectedRoute isAuthenticated={isAuthenticated}><IdeaDetailsPage /></ProtectedRoute>} />
        <Route path="/idea-details/:ideaId" element={<ProtectedRoute isAuthenticated={isAuthenticated}><SubDetailsPage /></ProtectedRoute>} />
        <Route path="/login" element={<Login />} />
        <Route path="/signup" element={<Signup />} />
        <Route path="/submit-idea-page" element={<ProtectedRoute isAuthenticated={isAuthenticated}><IdeaSubmission /></ProtectedRoute>} />
        <Route path="/login-page" element={<LoginPage />} />
        <Route path="/profile" element={<ProfilePage />} />
        <Route path="/newvote" element={<NewVotePage />} />
        <Route path="/challenge/:challengeId" element={<ChallengeDetailsPage />} />
        <Route path="/importance-page/:ideaId" element={<ProtectedRoute isAuthenticated={isAuthenticated}><ImportancePage /></ProtectedRoute>} /> {/* New route for the Importance Page */}
      </Routes>
    </Router>
  );
}

export default App;
