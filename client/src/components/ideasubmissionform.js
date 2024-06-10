import React, { useState } from 'react';
import axios from 'axios';

function IdeaSubmissionForm({ parentId, onSubmissionSuccess }) {
  const [title, setTitle] = useState('');
  const [sentenceDescription, setSentenceDescription] = useState('');
  const [paragraphDescription, setParagraphDescription] = useState('');
  const [fullDescription, setFullDescription] = useState('');

  const handleFormSubmit = async (e) => {
    e.preventDefault();
    const token = localStorage.getItem('token');
    const config = {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    };

    const baseUrl = process.env.REACT_APP_API_BASE_URL || 'http://localhost:5000';
    const endpoint = parentId ? `${baseUrl}/api/ideas/${parentId}/sub` : `${baseUrl}/api/ideas`;

    const formData = {
      title,
      sentenceDescription,
      paragraphDescription,
      fullDescription,
      isApproved: false, // Explicitly setting, but this is optional due to model default
      ...(parentId && { parentId }),
    };
    

    console.log("Preparing to submit:", formData, `to endpoint: ${endpoint}`);

    try {
      const response = await axios.post(endpoint, formData, config);
      console.log("Submission successful:", response.data);

      setTitle('');
      setSentenceDescription('');
      setParagraphDescription('');
      setFullDescription('');

      if (onSubmissionSuccess) {
        onSubmissionSuccess();
      }
    } catch (error) {
      console.error("An error occurred:", error.response ? error.response.data : "Network Error");
    }
  };

  return (
    <form onSubmit={handleFormSubmit}>
      <label>
      Title (max 50 chars):
  <input
    type="text"
    value={title}
    onChange={(e) => setTitle(e.target.value)}
    maxLength={50}
    required
        />
      </label>
      <label>
        Sentence Description (max 150 chars):
        <input
          type="text"
          value={sentenceDescription}
          onChange={(e) => setSentenceDescription(e.target.value)}
          maxLength={150}
          required
        />
      </label>
      <label>
        Paragraph Description (max 500 chars):
        <textarea
          value={paragraphDescription}
          onChange={(e) => setParagraphDescription(e.target.value)}
          maxLength={500}
        />
      </label>
      <label>
        Full Description (max 2000 chars):
        <textarea
          value={fullDescription}
          onChange={(e) => setFullDescription(e.target.value)}
          maxLength={2000}
        />
      </label>
      <button type="submit">Submit Idea</button>
    </form>
  );
}

export default IdeaSubmissionForm;
