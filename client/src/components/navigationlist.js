// navigationlist.js

import React from 'react';

const NavigationList = ({ ideas, onSelectIdea }) => {
  const renderIdeas = (ideas, level = 0) => {
    return (
      <ul style={{ paddingLeft: `${level * 20}px` }}> {/* Indent sub-ideas */}
        {ideas.map((idea) => (
          <li key={idea.id} style={{ color: idea.color }} onClick={() => onSelectIdea(idea.id)}>
            {idea.title} - Rank: {idea.importanceRank}
            {idea.subIdeas && renderIdeas(idea.subIdeas, level + 1)}
          </li>
        ))}
      </ul>
    );
  };

  return <nav>{renderIdeas(ideas)}</nav>;
};

export default NavigationList;
