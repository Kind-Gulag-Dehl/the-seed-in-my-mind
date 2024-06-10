import React, { useState, useEffect } from 'react';
import axios from 'axios';
import { Link } from 'react-router-dom'; // Import Link

function IdeasList() {
  const [ideas, setIdeas] = useState([]);
  const [error, setError] = useState(null);

  useEffect(() => {
    // Updated to use the environment variable for the base URL
    const baseUrl = process.env.REACT_APP_API_BASE_URL || 'http://localhost:5000';

    const fetchIdeas = async () => {
      try {
        // Now fetching main ideas only
        const fetchUrl = `${baseUrl}/api/ideas/main`;
        console.log(`Fetching main ideas from ${fetchUrl}`); // Log the URL being hit
        const response = await axios.get(fetchUrl);
        console.log('Fetched main ideas:', response.data); // Log the fetched data
        if (Array.isArray(response.data)) {
          setIdeas(response.data);
          setError(null); // Reset the error state on successful fetch
        } else {
          setError('The data format is incorrect.');
        }
      } catch (error) {
        console.error("An error occurred while fetching main ideas:", error); // Log any errors
        setError("An error occurred while fetching main ideas.");
      }
    };
    
    fetchIdeas();
  }, []);

  if (error) {
    return <p>Error: {error}</p>;
  }

  return (
    <div>
      <h2>Main Ideas List</h2>
      {ideas.length > 0 ? (
        <ul>
          {ideas.map((idea) => (
            <li key={idea._id}>
              <Link to={`/ideas/${idea._id}`}>
                <h3>{idea.title}</h3>
              </Link>
              {/* Updated to reflect main ideas might not have a 'description' but 'sentenceDescription' */}
              <p>{idea.sentenceDescription}</p>
            </li>
          ))}
        </ul>
      ) : (
        <p>No main ideas submitted yet.</p>
      )}
    </div>
  );
}

export default IdeasList;
