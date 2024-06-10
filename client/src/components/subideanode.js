import React, { useState } from 'react';
import Modal from './modal';
import SubIdeaModalContent from './subideamodalcontent';
import UpDownVote from './updownvote';
import './subideanode.css';

const SubIdeaNode = ({ idea, tier = 1, color }) => {
    const [isModalOpen, setIsModalOpen] = useState(false);

    const baseWidth = 180;
    const baseMinHeight = 90;
    const baseFontSize = 10;
    const baseBorderRadius = 20;
    const sizeReductionFactor = 0.8;

    const currentWidth = baseWidth * Math.pow(sizeReductionFactor, tier - 1);
    const currentMinHeight = baseMinHeight * Math.pow(sizeReductionFactor, tier - 1);
    const currentFontSize = baseFontSize * Math.pow(sizeReductionFactor, tier - 1);
    const currentBorderRadius = baseBorderRadius - (tier - 1);  // Decrease border radius by 1pt per tier

    const style = {
        '--dynamicFontSize': `${currentFontSize}px`,
        width: `${currentWidth}px`,
        minHeight: `${currentMinHeight}px`,
        backgroundColor: color,
        cursor: 'pointer',
        borderRadius: `${currentBorderRadius}px`
    };

    const handleOpenModal = (e) => {
        e.stopPropagation();
        setIsModalOpen(true);
    };

    const handleCloseModal = () => {
        setIsModalOpen(false);
    };

    // Ensure that upvotes and downvotes are passed as numbers
    const upvotesCount = idea.upvotes ? idea.upvotes.length : 0;
    const downvotesCount = idea.downvotes ? idea.downvotes.length : 0;

// Inside SubIdeaNode component render method
<UpDownVote contentId={idea._id} onVote={(data) => console.log('Vote data:', data)} upvotes={idea.upvotes} downvotes={idea.downvotes} tier={tier} />


    return (
        <div className="sub-idea-wrapper">
            <div className="sub-idea-node" style={style} onClick={handleOpenModal}>
                <h3>{idea.title}</h3>
                {idea._id ? (
                    <UpDownVote
                        contentId={idea._id}
                        onVote={(data) => console.log('Vote data:', data)}
                        upvotes={upvotesCount}
                        downvotes={downvotesCount}
                    />
                ) : (
                    console.log('SubIdeaNode: _id is undefined')
                )}
            </div>
            {idea.subIdeas && idea.subIdeas.length > 0 && (
                <div className="sub-ideas-children">
                    {idea.subIdeas.map((childIdea, index) => (
                        <SubIdeaNode key={childIdea._id} idea={childIdea} tier={tier + 1} color={color} />
                    ))}
                </div>
            )}
            <Modal isOpen={isModalOpen} onClose={handleCloseModal}>
                <SubIdeaModalContent idea={idea} color={color} onClose={handleCloseModal} />
            </Modal>
        </div>
    );
};

export default SubIdeaNode;
