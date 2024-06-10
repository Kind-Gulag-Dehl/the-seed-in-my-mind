import React, { useState } from 'react';
import Modal from './modal';
import MainIdeaModalContent from './mainideamodalcontent';
import UpDownVote from './updownvote'; // Import the voting component
import './mainideanode.css';

const MainIdeaNode = ({ idea }) => {
    const [isModalOpen, setIsModalOpen] = useState(false);

    console.log('MainIdeaNode: Rendering with idea:', idea);  // Log the full idea object

    const handleOpenModal = () => setIsModalOpen(true);
    const handleCloseModal = () => setIsModalOpen(false);

    // Calculate upvotes and downvotes count
    const upvotesCount = idea.upvotes ? idea.upvotes.length : 0;
    const downvotesCount = idea.downvotes ? idea.downvotes.length : 0;

    return (
        <div className="main-idea-wrapper">
            <div className="main-idea-node" onClick={handleOpenModal} style={{ cursor: 'pointer' }}>
                <h3>{idea.title}</h3>
                {idea._id ? (
                    <UpDownVote
                        contentId={idea._id}
                        onVote={(data) => console.log('Vote data:', data)}
                        upvotes={upvotesCount}
                        downvotes={downvotesCount}
                    />
                ) : (
                    console.log('MainIdeaNode: _id is undefined') // This will log if _id is undefined
                )}
            </div>
            <Modal isOpen={isModalOpen} onClose={handleCloseModal}>
                <MainIdeaModalContent idea={idea} onClose={handleCloseModal} />
            </Modal>
        </div>
    );
};

export default MainIdeaNode;
