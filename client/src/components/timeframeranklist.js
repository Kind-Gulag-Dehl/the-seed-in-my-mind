import React from 'react';

const TimeframeRankList = ({ category, ideasData = [] }) => {
    return (
        <div>
            <h2>{`Immediate Importance for ${category}`}</h2>
            <ul>
                {ideasData.length > 0 ? (
                    ideasData.map((idea, index) => (
                        <li key={index}>
                            Title: {idea.title}, Current Rank: {idea.currentRank}, Previous Rank: {idea.previousNode}, Buffer Rank: {idea.bufferRank}
                        </li>
                    ))
                ) : (
                    <p>No ideas available for this category.</p>
                )}
            </ul>
        </div>
    );
};

export default TimeframeRankList;
