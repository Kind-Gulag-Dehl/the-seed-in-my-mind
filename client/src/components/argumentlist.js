import React, { useState, useEffect } from 'react';
import Modal from './modal';
import ArgumentForm from './argumentform';
import UpDownVote from './updownvote';

const ArgumentList = ({ argList, onArgumentSelect, onSubmitArgument }) => {
  const [isAdding, setIsAdding] = useState(false);
  const [newArgument, setNewArgument] = useState({
    title: '',
    sentenceDescription: '',
    paragraphDescription: '',
    fullDescription: '',
    stance: 'for'
  });
  const [argumentsList, setArgumentsList] = useState(argList);

  useEffect(() => {
    console.log("ArgumentList useEffect: Updating local state with new props", argList);
    // Ensure that each argument is initialized properly if the upvotes/downvotes are undefined
    setArgumentsList(argList.map(arg => ({
      ...arg,
      upvotes: arg.upvotes || [],
      downvotes: arg.downvotes || []
    })));
  }, [argList]);

  const handleAddArgument = () => {
    setIsAdding(true);
  };

  const handleSubmit = () => {
    onSubmitArgument(newArgument);
    setIsAdding(false);
    setNewArgument({
      title: '',
      sentenceDescription: '',
      paragraphDescription: '',
      fullDescription: '',
      stance: 'for'
    });
  };

  const updateNewArgument = field => value => {
    setNewArgument(prev => ({ ...prev, [field]: value }));
  };

  const handleVote = (updatedArgument) => {
    console.log('ArgumentList handleVote: Updated argument received', updatedArgument);
    setArgumentsList(prevArgs =>
      prevArgs.map(arg => arg._id === updatedArgument._id ? {
        ...arg,
        upvotes: updatedArgument.upvotes || [],
        downvotes: updatedArgument.downvotes || []
      } : arg)
    );
  };
  
  useEffect(() => {
    console.log("Full argument list state after update:", argumentsList);
  }, [argumentsList]);
  
  if (!argumentsList.length) {
    return <p>No arguments available.</p>;
  }
  

  return (
    <>
      <ul style={{ listStyleType: 'none', padding: 0 }}>
        {argumentsList.map(arg => {
          const upvotesCount = arg.upvotes ? arg.upvotes.length : 0;
          const downvotesCount = arg.downvotes ? arg.downvotes.length : 0;
          console.log(`Rendering argument ID: ${arg._id} with ${upvotesCount} upvotes and ${downvotesCount} downvotes`);

          return (
            <li key={arg._id}
                style={{
                  padding: '10px',
                  cursor: 'pointer',
                  backgroundColor: arg.side === 'for' ? 'lightgreen' : 'lightcoral',
                  borderRadius: '8px',
                  margin: '5px 0',
                  display: 'flex',
                  alignItems: 'center',
                  justifyContent: 'space-between'
                }}
                onClick={() => onArgumentSelect(arg)}>
              <span>{arg.title || 'No Title'}</span>
              <UpDownVote
                contentId={arg._id}
                onVote={handleVote}
                upvotes={upvotesCount}
                downvotes={downvotesCount}
                voteType="arguments"
              />
            </li>
          );
        })}
      </ul>
      <button onClick={handleAddArgument}>Add Argument</button>
      {isAdding && (
        <Modal isOpen={isAdding} onClose={() => setIsAdding(false)}>
          <ArgumentForm 
              argument={newArgument}
              updateArgument={updateNewArgument}
              toggleStance={() => setNewArgument({...newArgument, stance: newArgument.stance === 'for' ? 'against' : 'for'})}
          />
          <button onClick={handleSubmit}>Submit Argument</button>
        </Modal>
      )}
    </>
  );
};

export default ArgumentList;
