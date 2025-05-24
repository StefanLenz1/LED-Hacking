import React, { useState, useEffect } from 'react';
import CFileEditor from '../CFileEditor';
import './ChallengeView.css';

function ChallengeView({ challenge, onBack }) {
  const [activeTab, setActiveTab] = useState('editor');
  const [contextCode, setContextCode] = useState([]);
  const [hints, setHints] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    if (!challenge) return;

    // Fetch challenge data (context code and hints) from server
    Promise.all([
      fetchContextCode(challenge.id),
      fetchHints(challenge.id)
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

  // Mock function to simulate fetching context code from server
  const fetchContextCode = (challengeId) => {
    return new Promise((resolve) => {
      // Simulate network delay
      setTimeout(() => {
        resolve([
          { 
            filename: 'led_driver.h', 
            content: '// LED Driver Header\n#ifndef LED_DRIVER_H\n#define LED_DRIVER_H\n\n#include <stdint.h>\n\nvoid led_init(void);\nvoid led_set_color(uint8_t r, uint8_t g, uint8_t b);\nvoid led_set_brightness(uint8_t brightness);\n\n#endif // LED_DRIVER_H' 
          },
          { 
            filename: 'led_driver.c', 
            content: '// LED Driver Implementation\n#include "led_driver.h"\n#include "hardware.h"\n\nvoid led_init(void) {\n  // Initialize LED hardware\n  hw_gpio_init();\n  hw_pwm_init();\n}\n\nvoid led_set_color(uint8_t r, uint8_t g, uint8_t b) {\n  // Set RGB color values\n  hw_pwm_set_channel(0, r);\n  hw_pwm_set_channel(1, g);\n  hw_pwm_set_channel(2, b);\n}\n\nvoid led_set_brightness(uint8_t brightness) {\n  // Set overall brightness\n  hw_pwm_set_global_brightness(brightness);\n}' 
          },
          { 
            filename: 'main.c', 
            content: '// Main application\n#include "led_driver.h"\n#include <stdio.h>\n\n// Your code will be inserted here\n\nint main(void) {\n  // Initialize system\n  led_init();\n  \n  // Your main loop will be called here\n  \n  return 0;\n}' 
          }
        ]);
      }, 800);
    });
  };

  // Mock function to simulate fetching hints from server
  const fetchHints = (challengeId) => {
    return new Promise((resolve) => {
      // Simulate network delay
      setTimeout(() => {
        resolve([
          { id: 1, title: 'Getting Started', content: 'Begin by initializing the LED system using the provided led_init() function.' },
          { id: 2, title: 'Setting Colors', content: 'Use led_set_color(r, g, b) to set the RGB values for the LED. Each parameter should be a value between 0-255.' },
          { id: 3, title: 'Creating Patterns', content: 'To create blinking patterns, you can use delay functions between color changes.' },
          { id: 4, title: 'Advanced Tip', content: 'For smooth transitions, gradually change color values in small increments rather than jumping directly to the target color.' }
        ]);
      }, 600);
    });
  };

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
                  <p>{hint.content}</p>
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