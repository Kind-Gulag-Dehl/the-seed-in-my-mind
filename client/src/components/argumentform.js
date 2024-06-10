import React from 'react';

const ArgumentForm = ({ argument, updateArgument, toggleStance }) => {
    return (
        <div style={{ backgroundColor: argument.stance === 'for' ? 'lightgreen' : 'lightcoral', padding: '10px', margin: '5px' }}>
            <div>
                <label>Title:</label>
                <input 
                    type="text" 
                    value={argument.title} 
                    onChange={(e) => updateArgument('title')(e.target.value)} // Correct usage
                    maxLength="50" 
                    required 
                />
                <small>{argument.title.length}/50</small>
            </div>
            <div>
                <label>Sentence Description:</label>
                <input 
                    type="text" 
                    value={argument.sentenceDescription} 
                    onChange={(e) => updateArgument('sentenceDescription')(e.target.value)} // Correct usage
                    maxLength="150" 
                    required 
                />
                <small>{argument.sentenceDescription.length}/150</small>
            </div>
            <div>
                <label>Paragraph Description:</label>
                <textarea 
                    value={argument.paragraphDescription} 
                    onChange={(e) => updateArgument('paragraphDescription')(e.target.value)} // Correct usage
                    maxLength="500" 
                    required 
                />
                <small>{argument.paragraphDescription.length}/500</small>
            </div>
            <div>
                <label>Full Description:</label>
                <textarea 
                    value={argument.fullDescription} 
                    onChange={(e) => updateArgument('fullDescription')(e.target.value)} // Correct usage
                    maxLength="2000" 
                    required 
                />
                <small>{argument.fullDescription.length}/2000</small>
            </div>
            <button type="button" onClick={toggleStance}>Toggle For/Against</button>
        </div>
    );
};

export default ArgumentForm;
