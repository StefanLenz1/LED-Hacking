import React, { useState, useEffect } from 'react';
import CFileEditor from '../CFileEditor';
import './ChallengeView.css';
import { getContextCode, getHints } from '../../services/challengeService';

function ChallengeView({ challenge, onBack }) {
  const [activeTab, setActiveTab] = useState('editor');
  const [contextCode, setContextCode] = useState([]);
  const [hints, setHints] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    if (!challenge) return;

    // Fetch challenge data (context code and hints) from the ESP-LSD-Hacking folder
    Promise.all([
      getContextCode(challenge.id),
      getHints(challenge.id)
    ])
      .then(([codeData, hintsData]) => {
        setContextCode(codeData);
        setHints(hintsData);
        setLoading(false);
      })
      .catch(err => {
        setError('Failed to load challenge data');
        setLoading(false);
        console.error(err);
      });
  }, [challenge]);

  if (loading) {
    return <div className="challenge-loading">Loading challenge data...</div>;
  }

  if (error) {
    return <div className="challenge-error">{error}</div>;
  }

  return (
    <div className="challenge-view">
      <div className="challenge-header">
        <button className="back-button" onClick={onBack}>← Back to Challenges</button>
        <h2>{challenge.title}</h2>
        <span className={`difficulty ${challenge.difficulty.toLowerCase()}`}>
          {challenge.difficulty}
        </span>
      </div>

      <p className="challenge-description">{challenge.description}</p>

      <div className="tabs">
        <button 
          className={`tab-button ${activeTab === 'editor' ? 'active' : ''}`}
          onClick={() => setActiveTab('editor')}
        >
          Code Editor
        </button>
        <button 
          className={`tab-button ${activeTab === 'context' ? 'active' : ''}`}
          onClick={() => setActiveTab('context')}
        >
          Context Code
        </button>
        <button 
          className={`tab-button ${activeTab === 'hints' ? 'active' : ''}`}
          onClick={() => setActiveTab('hints')}
        >
          Hints
        </button>
      </div>

      <div className="tab-content">
        {activeTab === 'editor' && (
          <div className="editor-tab">
            <CFileEditor />
          </div>
        )}

        {activeTab === 'context' && (
          <div className="context-tab">
            <h3>Context Code</h3>
            <p>Your code will be embedded within the following files:</p>

            <div className="context-files">
              {contextCode.map((file, index) => (
                <div key={index} className="context-file">
                  <h4>{file.filename}</h4>
                  <pre className="code-block">{file.content}</pre>
                </div>
              ))}
            </div>
          </div>
        )}

        {activeTab === 'hints' && (
          <div className="hints-tab">
            <h3>Hints</h3>
            <div className="hints-list">
              {hints.map(hint => (
                <details key={hint.id} className="hint-item">
                  <summary>{hint.title}</summary>
                  <p dangerouslySetInnerHTML={{ __html: hint.content }}></p>
                </details>
              ))}
            </div>
          </div>
        )}
      </div>
    </div>
  );
}

export default ChallengeView;
