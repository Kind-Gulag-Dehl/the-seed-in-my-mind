import React, { useState } from 'react';
import axios from 'axios';

function ModSubmissionForm({ ideaId, onSubmissionSuccess }) {
  const [modType, setModType] = useState('');
  const [suggestion, setSuggestion] = useState('');
  const [explanation, setExplanation] = useState('');

  const handleFormSubmit = async (e) => {
    e.preventDefault();
    const token = localStorage.getItem('token');
    const config = {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    };

    const baseUrl = process.env.REACT_APP_API_BASE_URL || 'http://localhost:5000';
    const endpoint = `${baseUrl}/api/ideas/mods/${ideaId}`;

    const formData = {
      modType,
      suggestion,
      explanation,
    };

    try {
      await axios.post(endpoint, formData, config);
      setModType('');
      setSuggestion('');
      setExplanation('');

      if (onSubmissionSuccess) {
        onSubmissionSuccess();
      }
    } catch (error) {
      console.error("An error occurred:", error.response ? error.response.data : "Network Error");
    }
  };

  const getSuggestionMaxLength = () => {
    switch (modType) {
      case 'title': return 50;
      case 'sentenceDescription': return 150;
      case 'paragraphDescription': return 500;
      case 'fullDescription': return 2000;
      default: return 0;
    }
  };

  return (
    <form onSubmit={handleFormSubmit}>
      <label>
        Modification Type:
        <select value={modType} onChange={(e) => setModType(e.target.value)} required>
          <option value="">Select...</option>
          <option value="title">Title</option>
          <option value="sentenceDescription">Sentence Description</option>
          <option value="paragraphDescription">Paragraph Description</option>
          <option value="fullDescription">Full Description</option>
        </select>
      </label>
      <label>
        Suggestion (max {getSuggestionMaxLength()} chars):
        <textarea
          value={suggestion}
          onChange={(e) => setSuggestion(e.target.value)}
          maxLength={getSuggestionMaxLength()}
          required
        />
      </label>
      <label>
        Explanation:
        <textarea
          value={explanation}
          onChange={(e) => setExplanation(e.target.value)}
          maxLength={2000}
        />
      </label>
      <button type="submit">Submit Modification</button>
    </form>
  );
}

export default ModSubmissionForm;
