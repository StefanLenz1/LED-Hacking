import React, { useState, useEffect } from 'react';
import './ChallengesList.css';

function ChallengesList({ onSelectChallenge }) {
  const [challenges, setChallenges] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    // In a real application, fetch challenges from the server
    fetchChallenges()
      .then(data => {
        setChallenges(data);
        setLoading(false);
      })
      .catch(err => {
        setError('Failed to load challenges');
        setLoading(false);
        console.error(err);
      });
  }, []);

  // Mock function to simulate fetching challenges from server
  const fetchChallenges = () => {
    return new Promise((resolve) => {
      // Simulate network delay
      setTimeout(() => {
        resolve([
          { id: 1, title: 'LED Blink Pattern', difficulty: 'Easy', description: 'Create a simple blinking pattern for the LED.' },
          { id: 2, title: 'RGB Color Cycle', difficulty: 'Medium', description: 'Program the RGB LED to cycle through colors.' },
          { id: 3, title: 'Sound Reactive LEDs', difficulty: 'Hard', description: 'Make LEDs react to sound input.' },
          { id: 4, title: 'Custom Animation', difficulty: 'Expert', description: 'Create a custom animation sequence for the LED strip.' }
        ]);
      }, 1000);
    });
  };

  const handleChallengeClick = (challenge) => {
    if (onSelectChallenge) {
      onSelectChallenge(challenge);
    }
  };

  if (loading) {
    return <div className="challenges-loading">Loading challenges...</div>;
  }

  if (error) {
    return <div className="challenges-error">{error}</div>;
  }

  return (
    <div className="challenges-list">
      <h2>Available Challenges</h2>
      {challenges.length === 0 ? (
        <p>No challenges available at the moment.</p>
      ) : (
        <ul>
          {challenges.map(challenge => (
            <li key={challenge.id} onClick={() => handleChallengeClick(challenge)}>
              <div className="challenge-card">
                <h3>{challenge.title}</h3>
                <span className={`difficulty ${challenge.difficulty.toLowerCase()}`}>
                  {challenge.difficulty}
                </span>
                <p>{challenge.description}</p>
                <button 
                  className="start-challenge-btn"
                  onClick={(e) => {
                    e.stopPropagation(); // Prevent the li onClick from firing
                    handleChallengeClick(challenge);
                  }}
                >
                  Start Challenge
                </button>
              </div>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}

export default ChallengesList;
