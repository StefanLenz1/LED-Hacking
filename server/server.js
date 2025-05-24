const express = require('express');
const fs = require('fs');
const path = require('path');
const cors = require('cors');
const bodyParser = require('body-parser');
const { exec } = require('child_process');

const app = express();
const PORT = process.env.PORT || 3002;

// Middleware
app.use(cors());
app.use(bodyParser.json());

// Route to handle code submission
app.post('/api/save-code', (req, res) => {
  try {
    const { content } = req.body;

    // Write the code to /tmp/code.c
    fs.writeFileSync('/tmp/code.c', content, 'utf8');

    console.log('Code written to /tmp/code.c');
    res.status(200).json({ success: true, message: 'Code saved successfully' });
  } catch (error) {
    console.error('Error writing to file:', error);
    res.status(500).json({ success: false, message: 'Error saving code', error: error.message });
  }
});

// Route to handle microcontroller reset
app.post('/api/reset-microcontroller', (req, res) => {
  try {
    // Execute a bash command to reset the microcontroller
    // This is a placeholder command - replace with the actual command needed for your microcontroller
    exec('echo "Resetting microcontroller..." > /tmp/did && sleep 1', (error, stdout, stderr) => {
      if (error) {
        console.error(`Error executing reset command: ${error}`);
        return res.status(500).json({ success: false, message: 'Error resetting microcontroller', error: error.message });
      }

      console.log('Microcontroller reset command executed');
      console.log('stdout:', stdout);
      console.log('stderr:', stderr);

      res.status(200).json({ success: true, message: 'Microcontroller reset successfully' });
    });
  } catch (error) {
    console.error('Error resetting microcontroller:', error);
    res.status(500).json({ success: false, message: 'Error resetting microcontroller', error: error.message });
  }
});

// Start the server
app.listen(PORT, () => {
  console.log(`Server running on port ${PORT}`);
});
