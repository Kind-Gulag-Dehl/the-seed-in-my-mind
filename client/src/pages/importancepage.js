import React, { useState, useEffect } from 'react';
import axios from 'axios';
import { useParams } from 'react-router-dom';
import NavigationBar from '../components/navigationbar';
import TimeframeRankList from '../components/timeframeranklist';

const ImportancePage = () => {
    const { ideaId } = useParams();
    const [importanceData, setImportanceData] = useState({});
    const [error, setError] = useState('');

    useEffect(() => {
        if (!ideaId) {
            setError('No idea ID provided');
            return;
        }

        const fetchImportanceData = async () => {
            try {
                const response = await axios.get(`http://localhost:5000/api/ideas/${ideaId}/importance`);
                setImportanceData(response.data);
            } catch (err) {
                setError(`Failed to fetch importance data: ${err.message}`);
                console.error(err);
            }
        };

        fetchImportanceData();
    }, [ideaId]);

    if (error) {
        return (
            <div>
                <p>Error: {error}</p>
            </div>
        );
    }

    return (
        <div>
            <h1>Importance Ranks for Idea</h1>
            {importanceData && importanceData.immediate && (
                <TimeframeRankList
                    category="forIndividuals"
                    ideasData={importanceData.immediate.forIndividuals}
                />
            )}
        </div>
    );
};

export default ImportancePage;
