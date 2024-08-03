const cron = require('node-cron');
const axios = require('axios'); // Axios will be used to make HTTP requests.

// Function to trigger the ranking recalculation
async function triggerRankRecalculation() {
    try {
        const response = await axios.post('http://localhost:5000/api/ideas/rank/recalculate');
        console.log('Ranking recalculated successfully:', response.data);
    } catch (error) {
        console.error('Error triggering rank recalculation:', error.response ? error.response.data : error.message);
    }
}

// Schedule the ranking recalculation to run every week
cron.schedule('0 0 * * 0', () => {
    console.log('Running scheduled task to recalculate rankings');
    triggerRankRecalculation();
}, {
    scheduled: true,
    timezone: "America/New_York"
});

module.exports = { triggerRankRecalculation };
