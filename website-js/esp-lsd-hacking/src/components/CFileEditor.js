import React, { useState, useRef, useEffect } from 'react';
import './CFileEditor.css';

function CFileEditor() {
  const [code, setCode] = useState('// Write your C code here...');
  const [fileName] = useState('main.c');
  const [uploadStatus, setUploadStatus] = useState('');
  const [lineCount, setLineCount] = useState(1);
  const textareaRef = useRef(null);

  const handleCodeChange = (e) => {
    const newCode = e.target.value;
    setCode(newCode);

    // Update line count
    const lines = newCode.split('\n').length;
    setLineCount(lines);
  };

  // Initialize line count and handle textarea events
  useEffect(() => {
    const lines = code.split('\n').length;
    setLineCount(lines);

    // Ensure textarea and line numbers have the same height
    if (textareaRef.current) {
      const handleResize = () => {
        const lineNumbersElement = document.querySelector('.line-numbers');
        if (lineNumbersElement) {
          lineNumbersElement.style.height = `${textareaRef.current.scrollHeight}px`;
        }
      };

      // Initial sizing
      handleResize();

      // Add event listener for textarea resize
      textareaRef.current.addEventListener('input', handleResize);

      // Cleanup
      return () => {
        if (textareaRef.current) {
          textareaRef.current.removeEventListener('input', handleResize);
        }
      };
    }
  }, [code]);

  const uploadToServer = async () => {
    try {
      setUploadStatus('Uploading...');

      // Send the code to our server endpoint
      const response = await fetch('http://localhost:3002/api/save-code', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ content: code }),
      });

      const data = await response.json();

      if (data.success) {
        console.log(`File ${fileName} uploaded with content:`, code);
        console.log('Code written to /tmp/code.c');
        setUploadStatus('Upload successful!');
      } else {
        setUploadStatus(`Upload failed: ${data.message}`);
      }
    } catch (error) {
      console.error('Error uploading code:', error);
      setUploadStatus(`Error: ${error.message}`);
    }
  };

  // Generate line numbers
  const renderLineNumbers = () => {
    return Array.from({ length: lineCount }, (_, i) => i + 1).map(num => (
      <div key={num} className="line-number">
        {num}
      </div>
    ));
  };

  return (
    <div className="c-file-editor">
      <h2>C File Editor</h2>

      <div className="editor-container">
        <div className="editor-with-line-numbers">
          <div className="line-numbers">
            {renderLineNumbers()}
          </div>
          <textarea
            ref={textareaRef}
            className="code-editor"
            value={code}
            onChange={handleCodeChange}
            spellCheck="false"
            rows="20"
            onScroll={(e) => {
              if (document.querySelector('.line-numbers')) {
                document.querySelector('.line-numbers').scrollTop = e.target.scrollTop;
              }
            }}
          />
        </div>
      </div>

      <div className="button-container">
        <button onClick={uploadToServer} className="upload-button">
          Upload to Server
        </button>
        {uploadStatus && <div className="status-message">{uploadStatus}</div>}
      </div>
    </div>
  );
}

export default CFileEditor;
