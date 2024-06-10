import React, { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import axios from 'axios';
import dateFormat from 'dateformat'; // Assuming you use 'dateformat' for formatting dates

const SubDetailsPage = () => {
    const { ideaId } = useParams();
    const [subIdeaDetails, setSubIdeaDetails] = useState(null);
    const [parentIdeas, setParentIdeas] = useState('');

    useEffect(() => {
        const fetchSubIdeaDetails = async () => {
            try {
                const response = await axios.get(`http://localhost:5000/api/ideas/${ideaId}`);
                setSubIdeaDetails(response.data);

                // Assume `response.data` includes a `parentHierarchy` array with titles of parent ideas
                // Example of parentHierarchy: ["Main Idea Title", "Tier 2 Sub Idea"]
                const parentHierarchy = response.data.parentHierarchy || [];
                setParentIdeas(parentHierarchy.join(' > '));
            } catch (error) {
                console.error("Error fetching sub-idea details:", error);
            }
        };
        fetchSubIdeaDetails();
    }, [ideaId]);

    if (!subIdeaDetails) return <div>Loading...</div>;

    return (
        <div>
           {parentIdeas && <h4>Parent Ideas: {parentIdeas} {'>'} {subIdeaDetails.title}</h4>}

            <h1>Idea (max characters):</h1>
            <p>{subIdeaDetails.title}</p>
            <h2>Created by:</h2>
            <p>{subIdeaDetails.authorName /* Assuming `authorName` is part of your idea details */}</p>
            <h2>Date created:</h2>
            <p>{dateFormat(subIdeaDetails.createdAt, "mmmm dS, yyyy")}</p>
            <h2>Sentence Description:</h2>
            <p>{subIdeaDetails.sentenceDescription}</p>
            <h2>Paragraph Description:</h2>
            <p>{subIdeaDetails.paragraphDescription}</p>
            <h2>Full Description:</h2>
            <p>{subIdeaDetails.fullDescription}</p>
        </div>
    );
};

export default SubDetailsPage;
