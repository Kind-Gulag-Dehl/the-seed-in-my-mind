
import React, { useEffect, useState } from 'react';
import axios from 'axios';
import { useParams } from 'react-router-dom';
import MainIdeaNode from '../components/mainideanode';
import SubIdeaNode from '../components/subideanode';
import ProposedIdeaList from '../components/proposedidealist';


import './ideadetailspage.css';


const IdeaDetailsPage = () => {
  const { ideaId } = useParams();
  const [ideaDetails, setIdeaDetails] = useState(null);
  const [proposedMainIdeas, setProposedMainIdeas] = useState([]); // State to track proposed main ideas
  const [showProposedForMain, setShowProposedForMain] = useState(false); // State to control the visibility of proposed main ideas
  const [error, setError] = useState('');
  const [isSidebarOpen, setIsSidebarOpen] = useState(false);
  const baseColors = ['#FF6347', '#4682B4', '#FFD700', '#6A5ACD', '#FF69B4', '#DAA520', '#800080', '#008B8B', '#FF4500', '#32CD32'];

  useEffect(() => {
    const fetchData = async () => {
      try {
        const detailsResponse = await axios.get(`http://localhost:5000/api/ideas/${ideaId}`);
        setIdeaDetails(detailsResponse.data);

        const proposedResponse = await axios.get(`http://localhost:5000/api/ideas/proposed?parentId=${ideaId}`);
        setProposedMainIdeas(proposedResponse.data);
      } catch (err) {
        console.error('Error while fetching data:', err);
        setError('An error occurred while fetching idea details.');
      }
    };

    fetchData();
  }, [ideaId]);

  // Function to toggle the display of proposed ideas for the main idea
  const toggleShowProposedForMain = () => {
    setShowProposedForMain(!showProposedForMain);
  };

  const renderSubIdeasForSidebar = (subIdeas, tier = 2, parentColorIndex = 0) => {
    if (!subIdeas || subIdeas.length === 0) return null;

    const tierHeaders = {};

    const generateTierContent = (ideas, currentTier, colorIndex) => {
      return ideas.map((subIdea, index) => {
        const newColorIndex = currentTier === 2 ? index % baseColors.length : colorIndex; // Tier 2 ideas get new colors, deeper tiers inherit
        if (!tierHeaders[currentTier]) {
          tierHeaders[currentTier] = [<h2 key={`tier-${currentTier}-header`}>Tier {currentTier}</h2>];
        }

        tierHeaders[currentTier].push(
          <div key={subIdea._id} className="sidebar-sub-idea-title" style={{ backgroundColor: baseColors[newColorIndex] }}>
            {subIdea.title}
            <ProposedIdeaList ideaId={subIdea._id} parentTier={currentTier} />
          </div>
        );

        if (subIdea.subIdeas) {
          generateTierContent(subIdea.subIdeas, currentTier + 1, newColorIndex); // Pass the color index down to children
        }
      });
    };

    generateTierContent(subIdeas, tier, parentColorIndex);

    return Object.values(tierHeaders).flat();
  };

  return (
    <div className="page-layout">
      <button className="sidebar-toggle" onClick={() => setIsSidebarOpen(!isSidebarOpen)}>☰</button>
      <div className={`sidebar ${isSidebarOpen ? 'open' : ''}`}>
        <div className="sidebar-content">
          {ideaDetails && (
            <>
              <h2>Main Idea</h2>
              <div className="sidebar-main-idea-title">
                {ideaDetails.title}
                {/* Directly render ProposedIdeaList for the main idea */}
                <ProposedIdeaList ideaId={ideaId} parentTier={1} />
              </div>
              {/* Rest of the sidebar content for sub-ideas */}
              {ideaDetails.subIdeas && renderSubIdeasForSidebar(ideaDetails.subIdeas)}
            </>
          )}
          {error && <p className="error">{error}</p>}
        </div>
      </div>
      <div className="idea-details-container">
        {ideaDetails ? <MainIdeaNode idea={ideaDetails} /> : <p>Loading main idea...</p>}
        {/* Sub-ideas visualization */}
        <div className="sub-ideas-container">
          {ideaDetails?.subIdeas?.map((subIdea, index) => (
            <SubIdeaNode
              key={subIdea._id}
              idea={subIdea}
              color={baseColors[index % baseColors.length]}
              tier={2} // start from tier 2 for sub-ideas
            />
          ))}
        </div>
      </div>
      <div className="horizontal-scrollbar-placeholder" />
    </div>
  );
};

export default IdeaDetailsPage;
