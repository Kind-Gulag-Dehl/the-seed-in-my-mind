import React from 'react';
import { useLocation } from 'react-router-dom';
import IdeaSubmissionForm from '../components/ideasubmissionform';

const IdeaSubmission = () => {
  // Helper function to parse query parameters
  const useQuery = () => {
    return new URLSearchParams(useLocation().search);
  };

  const query = useQuery();
  const parentId = query.get('parentId'); // Retrieves 'parentId' from the query parameters

  return (
    <div>
      <h1>{parentId ? 'Submit New Sub-Idea' : 'Submit New Idea'}</h1>
      <IdeaSubmissionForm parentId={parentId} />
    </div>
  );
};

export default IdeaSubmission;
