import React, { useState, useEffect } from 'react';
import axios from 'axios';
import Modal from './modal';
import { useSelector } from 'react-redux';

const IdeaApprovalChallengeForm = ({ ideaId, isOpen, onClose }) => {
    const [responses, setResponses] = useState({
        functionalNecessity: '',
        importance: '',
        justification: ''
    });
    const [isSubmitting, setIsSubmitting] = useState(false);
    const [error, setError] = useState('');

    // Get the user ID from Redux state
    const userId = useSelector(state => state.user.user?.userId);
    console.log("Fetched user ID from Redux state:", userId);

    // Log the entire user state to ensure the data structure is as expected
    const userState = useSelector(state => state.user);
    console.log("Current user state from Redux:", userState);

    // Check how often your component re-renders and the user ID fetched each time
    useEffect(() => {
        console.log("Component re-rendered. Current userId:", userId);
    });

    const handleChange = (e) => {
        setResponses({ ...responses, [e.target.name]: e.target.value });
    };

    const handleSubmit = async (e) => {
        e.preventDefault();
        setIsSubmitting(true);
        setError('');
    
        try {
            const requestData = {
                associatedIdea: ideaId,
                initiator: userId,  // Use the user ID from Redux
                challengeType: 'approve',
                ...responses
            };
        
            console.log("Request data being sent:", requestData);
        
            const response = await axios.post('http://localhost:5000/api/challenges/initiate', requestData);
            console.log("Challenge response:", response.data);
            alert('Challenge for approval submitted successfully!');
            onClose();  // Close the modal on success
            setResponses({
                functionalNecessity: '',
                importance: '',
                justification: ''
            });
        } catch (err) {
            setError('Failed to submit approval challenge. Please try again.');
            console.error("Error submitting challenge:", err);
            console.log("Error details:", err.response?.data);  // Log backend error response
        }
        setIsSubmitting(false);
    };

    return (
        <Modal isOpen={isOpen} onClose={onClose}>
            <form onSubmit={handleSubmit}>
                <h3>Approval Challenge Form</h3>
                <div>
                    <label htmlFor="functionalNecessity">Could a functional version of the main idea be completed without this proposal?</label>
                    <textarea
                        id="functionalNecessity"
                        name="functionalNecessity"
                        value={responses.functionalNecessity}
                        onChange={handleChange}
                        required
                    />
                </div>
                <div>
                    <label htmlFor="importance">Is this idea more important than at least one other idea in its tier?</label>
                    <textarea
                        id="importance"
                        name="importance"
                        value={responses.importance}
                        onChange={handleChange}
                        required
                    />
                </div>
                <div>
                    <label htmlFor="justification">Why do you think this idea should be added/why do you think it will make the project better?</label>
                    <textarea
                        id="justification"
                        name="justification"
                        value={responses.justification}
                        onChange={handleChange}
                        required
                    />
                </div>
                {error && <p className="error">{error}</p>}
                <button type="submit" disabled={isSubmitting}>
                    {isSubmitting ? 'Submitting...' : 'Submit Approval Challenge'}
                </button>
            </form>
        </Modal>
    );
};

export default IdeaApprovalChallengeForm;
