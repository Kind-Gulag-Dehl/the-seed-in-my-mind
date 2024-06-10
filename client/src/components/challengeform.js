import React, { useState } from 'react';
import axios from 'axios';
import Modal from './modal';
import { useSelector } from 'react-redux';

const ChallengeForm = ({ ideaId, isOpen, onClose, challengeType }) => {
    const [argumentsList, setArgumentsList] = useState([]);
    const [isSubmitting, setIsSubmitting] = useState(false);
    const [error, setError] = useState('');

    const userId = useSelector(state => state.user.user?.userId);

    const addArgument = () => {
        setArgumentsList([...argumentsList, { title: '', sentenceDescription: '', paragraphDescription: '', fullDescription: '', stance: 'for' }]);
    };

    const updateArgument = (index, field, value) => {
        const newArguments = [...argumentsList];
        newArguments[index][field] = value;
        setArgumentsList(newArguments);
    };

    const toggleStance = (index) => {
        const newArguments = [...argumentsList];
        newArguments[index].stance = newArguments[index].stance === 'for' ? 'against' : 'for';
        setArgumentsList(newArguments);
    };

    const handleSubmit = async (e) => {
        e.preventDefault();
        setIsSubmitting(true);
        setError('');
    
        console.log("UserId fetched from Redux:", userId);
    
        if (!userId) {
            console.error('No userId available, cancelling submission');
            setError('No user information available, please log in.');
            setIsSubmitting(false);
            return;
        }
    
        const requestData = {
            associatedIdea: ideaId,
            initiator: userId,
            challengeType: challengeType,
            argumentsFor: argumentsList.filter(arg => arg.stance === 'for').map(arg => ({
                title: arg.title,
                sentenceDescription: arg.sentenceDescription,
                paragraphDescription: arg.paragraphDescription,
                fullDescription: arg.fullDescription
            })),
            argumentsAgainst: argumentsList.filter(arg => arg.stance === 'against').map(arg => ({
                title: arg.title,
                sentenceDescription: arg.sentenceDescription,
                paragraphDescription: arg.paragraphDescription,
                fullDescription: arg.fullDescription
            }))
        };
    
        console.log("Final request data being sent:", requestData);
    
        try {
            const response = await axios.post('http://localhost:5000/api/challenges/initiate', requestData);
            console.log('Challenge submitted successfully:', response.data);
            alert('Challenge submitted successfully!');
            onClose();
            setArgumentsList([]);
        } catch (err) {
            console.error("Error submitting challenge:", err.response ? err.response.data : err);
            setError('Failed to submit challenge. Please try again.');
        } finally {
            setIsSubmitting(false);
        }
    };
    
    

    return (
        <Modal isOpen={isOpen} onClose={onClose}>
            <form onSubmit={handleSubmit}>
                <h3>{challengeType.charAt(0).toUpperCase() + challengeType.slice(1)} Challenge Form</h3>
                {argumentsList.map((argument, index) => (
                    <div key={index} style={{ backgroundColor: argument.stance === 'for' ? 'lightgreen' : 'lightcoral', padding: '10px', margin: '5px' }}>
                        <div>
                            <label>Title:</label>
                            <input type="text" value={argument.title} onChange={(e) => updateArgument(index, 'title', e.target.value)} maxLength="50" required />
                            <small>{argument.title.length}/50</small>
                        </div>
                        <div>
                            <label>Sentence Description:</label>
                            <input type="text" value={argument.sentenceDescription} onChange={(e) => updateArgument(index, 'sentenceDescription', e.target.value)} maxLength="150" required />
                            <small>{argument.sentenceDescription.length}/150</small>
                        </div>
                        <div>
                            <label>Paragraph Description:</label>
                            <textarea value={argument.paragraphDescription} onChange={(e) => updateArgument(index, 'paragraphDescription', e.target.value)} maxLength="500" required />
                            <small>{argument.paragraphDescription.length}/500</small>
                        </div>
                        <div>
                            <label>Full Description:</label>
                            <textarea value={argument.fullDescription} onChange={(e) => updateArgument(index, 'fullDescription', e.target.value)} maxLength="2000" required />
                            <small>{argument.fullDescription.length}/2000</small>
                        </div>
                        <button type="button" onClick={() => toggleStance(index)}>Toggle For/Against</button>
                    </div>
                ))}
                <button type="button" onClick={addArgument}>Add Argument</button>
                {error && <p className="error">{error}</p>}
                <button type="submit" disabled={isSubmitting || argumentsList.length === 0}>
                    {isSubmitting ? 'Submitting...' : 'Submit Challenge'}
                </button>
            </form>
        </Modal>
    );
};

export default ChallengeForm;
