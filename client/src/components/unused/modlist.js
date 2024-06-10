import React, { useState, useEffect } from 'react';
import axios from 'axios';

const ModList = ({ ideaId }) => {
  const [mods, setMods] = useState([]);
  const [isCollapsed, setIsCollapsed] = useState(true);

  useEffect(() => {
    const fetchMods = async () => {
      try {
        const response = await axios.get(`http://localhost:5000/api/ideas/mods/list/${ideaId}`);
        setMods(response.data);
      } catch (error) {
        console.error("Error fetching modifications:", error);
      }
    };

    fetchMods();
  }, [ideaId]);

  const toggleCollapse = () => {
    setIsCollapsed(!isCollapsed);
  };

  // Group modifications by type
  const groupedMods = mods.reduce((acc, mod) => {
    const { modType } = mod;
    if (!acc[modType]) {
      acc[modType] = [];
    }
    acc[modType].push(mod);
    return acc;
  }, {});

  return (
    <div>
      <button onClick={toggleCollapse}>
        {isCollapsed ? 'Suggested Modifications' : 'Collapse'}
      </button>
      {!isCollapsed && (
        <div style={{ maxHeight: '300px', overflowY: 'scroll' }}>
          {Object.keys(groupedMods).length > 0 ? (
            Object.entries(groupedMods).map(([modType, mods]) => (
              mods.map((mod, index) => (
                <div key={`${modType}-${index}`}>
                  {/* Display mod type and index + 1 (for human-readable numbering) */}
                  <p>{`${modType} ${index + 1}`}</p>
                </div>
              ))
            ))
          ) : (
            <p>No modifications suggested yet.</p>
          )}
        </div>
      )}
    </div>
  );
};

export default ModList;
