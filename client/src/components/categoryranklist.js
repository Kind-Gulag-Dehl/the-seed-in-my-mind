import React from 'react'; // Make sure React is imported

const CategoryRankList = ({ timeframe, category, ideas }) => {
    return (
        <div>
            <h2>{`${timeframe} importance ${category}`}</h2>
            <ul>
                {Array.isArray(ideas) ? ideas.map(idea => (
                    <li key={idea._id}>
                        {idea.title} - Rank: {idea.importanceRanks[timeframe][category].currentRank}  // Assuming 'currentRank' needs to be displayed
                    </li>
                )) : <li>No ideas available</li>}
            </ul>
        </div>
    );
};

export default CategoryRankList;
