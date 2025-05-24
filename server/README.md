# ESP-LSD-Hacking Server

This is a simple Express server that handles code submissions from the ESP-LSD-Hacking web application and writes them to `/tmp/code.c`.

## Setup

1. Install dependencies:
   ```
   cd server
   npm install
   ```

2. Start the server:
   ```
   npm start
   ```

   Or for development with auto-restart:
   ```
   npm run dev
   ```

## API Endpoints

### POST /api/save-code

Saves the submitted code to `/tmp/code.c`.

**Request Body:**
```json
{
  "content": "// C code content here"
}
```

**Response:**
```json
{
  "success": true,
  "message": "Code saved successfully"
}
```

## Client Integration

The client application is configured to send code to this server when the "Upload to Server" button is clicked. The server runs on port 3001 by default.