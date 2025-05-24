import React, { useState, useEffect } from 'react';
import './ChallengesList.css';
import { getChallenges } from '../../services/challengeService';

function ChallengesList({ onSelectChallenge }) {
  const [challenges, setChallenges] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    // Fetch challenges from the ESP-LSD-Hacking folder
    getChallenges()
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
