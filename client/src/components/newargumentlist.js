import React from 'react';

const NewArgumentList = ({ argumentsList, onArgumentSelect }) => {
  if (!argumentsList || argumentsList.length === 0) {
    return <p>No arguments provided.</p>;
  }

  return (
    <div style={{ margin: '20px 0' }}>
      <h4>Arguments:</h4>
      <ul style={{ listStyleType: 'none', padding: 0 }}>
        {argumentsList.map(arg => (
          <li key={arg._id}
              style={{
                padding: '10px',
                cursor: 'pointer',
                backgroundColor: arg.side === 'for' ? 'lightgreen' : 'lightcoral'
              }}
              onClick={() => onArgumentSelect(arg)}>
            <strong>{arg.title || 'No Title'}</strong>
            <p>{arg.sentenceDescription}</p>
          </li>
        ))}
      </ul>
    </div>
  );
};

export default NewArgumentList;
