import React, { useState } from 'react';
import './App.css';
import ChallengesList from './components/challenges/ChallengesList';
import ChallengeView from './components/challenges/ChallengeView';

function App() {
  const [selectedChallenge, setSelectedChallenge] = useState(null);

  const handleSelectChallenge = (challenge) => {
    setSelectedChallenge(challenge);
  };

  const handleBackToList = () => {
    setSelectedChallenge(null);
  };

  return (
    <div className="App">
      <header className="App-header">
        <h1>ESP LED Hacking Tool</h1>
      </header>
      <main>
        {selectedChallenge ? (
          <ChallengeView 
            challenge={selectedChallenge} 
            onBack={handleBackToList} 
          />
        ) : (
          <ChallengesList onSelectChallenge={handleSelectChallenge} />
        )}
      </main>
      <footer>
        <p>Upload your C code to customize LED behavior</p>
      </footer>
    </div>
  );
}

export default App;
