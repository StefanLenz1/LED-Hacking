import React, { useState } from 'react';
import './CFileEditor.css';

function CFileEditor() {
  const [code, setCode] = useState('// Write your C code here...');
  const [fileName] = useState('main.c');
  const [uploadStatus, setUploadStatus] = useState('');

  const handleCodeChange = (e) => {
    setCode(e.target.value);
  };

  const uploadToServer = async () => {
    try {
      setUploadStatus('Uploading...');

      // In a real application, you would use fetch or axios to send the data to your server
      // This is a mock implementation
      const response = await mockServerUpload(fileName, code);

      if (response.success) {
        setUploadStatus('Upload successful!');
      } else {
        setUploadStatus(`Upload failed: ${response.error}`);
      }
    } catch (error) {
      setUploadStatus(`Error: ${error.message}`);
    }
  };

  // Mock function to simulate server upload
  const mockServerUpload = (fileName, content) => {
    return new Promise((resolve) => {
      // Simulate network delay
      setTimeout(() => {
        console.log(`File ${fileName} uploaded with content:`, content);
        resolve({ success: true });
      }, 1000);
    });
  };

  return (
    <div className="c-file-editor">
      <h2>C File Editor</h2>

      <div className="editor-container">
        <textarea
          className="code-editor"
          value={code}
          onChange={handleCodeChange}
          spellCheck="false"
          rows="20"
        />
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
