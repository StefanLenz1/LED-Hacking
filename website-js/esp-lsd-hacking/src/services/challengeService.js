// Challenge Service
// This service handles reading and parsing the C code files from the ESP-LSD-Hacking folder

// Import required modules
import axios from 'axios';

// Base path to the ESP-LSD-Hacking folder
const BASE_PATH = '/ESP-LSD-Hacking';

/**
 * Get a list of all challenges
 * @returns {Promise<Array>} Array of challenge objects
 */
export const getChallenges = async () => {
  try {
    // In a real implementation, this would read the directory structure
    // For now, we'll return a hardcoded list based on our exploration
    return [
      {
        id: 1,
        title: 'LED Matrix Hacking (C Style)',
        difficulty: 'Medium',
        description: 'Bypass the LED matrix limitation by modifying the set_led_wrapper function in C style. The GRÜÜÜÜÜNEN have limited the LED matrix to 6x7, but the hardware supports 8x8. Your task is to modify the set_led_wrapper function to bypass this limitation.',
        taskPath: `${BASE_PATH}/src/task1/accessable/tips/task`,
        userCodePath: `${BASE_PATH}/src/task1/accessable/usercode.cpp`,
        contextCodePaths: [
          `${BASE_PATH}/src/task1/accessable/safe_led.h`,
          `${BASE_PATH}/src/task1/accessable/usercode.h`
        ],
        tipsPaths: [
          `${BASE_PATH}/src/task1/accessable/tips/tip1`,
          `${BASE_PATH}/src/task1/accessable/tips/tip2`,
          `${BASE_PATH}/src/task1/accessable/tips/tip3`,
          `${BASE_PATH}/src/task1/accessable/tips/tip4`,
          `${BASE_PATH}/src/task1/accessable/tips/tip5`,
          `${BASE_PATH}/src/task1/accessable/tips/tip6(solution)`
        ]
      },
      {
        id: 2,
        title: 'LED Matrix Hacking (C++ Style)',
        difficulty: 'Hard',
        description: 'Bypass the LED matrix limitation by modifying the set_led_wrapper function in C++ style. The GRÜÜÜÜÜNEN have limited the LED matrix to 6x7, but the hardware supports 8x8. Your task is to modify the set_led_wrapper function to bypass this limitation.',
        taskPath: `${BASE_PATH}/src/task2/accessable/tips/task`,
        userCodePath: `${BASE_PATH}/src/task2/accessable/usercode.cpp`,
        contextCodePaths: [
          `${BASE_PATH}/src/task2/accessable/safe_led.hpp`,
          `${BASE_PATH}/src/task2/accessable/usercode.h`
        ],
        tipsPaths: [
          `${BASE_PATH}/src/task2/accessable/tips/tip1`,
          `${BASE_PATH}/src/task2/accessable/tips/tip2`,
          `${BASE_PATH}/src/task2/accessable/tips/tip3`,
          `${BASE_PATH}/src/task2/accessable/tips/tip4`,
          `${BASE_PATH}/src/task2/accessable/tips/tip5`,
          `${BASE_PATH}/src/task2/accessable/tips/tip6(solution)`
        ]
      }
    ];
  } catch (error) {
    console.error('Error fetching challenges:', error);
    throw error;
  }
};

/**
 * Get the details of a specific challenge
 * @param {number} challengeId - The ID of the challenge
 * @returns {Promise<Object>} Challenge object
 */
export const getChallenge = async (challengeId) => {
  try {
    const challenges = await getChallenges();
    const challenge = challenges.find(c => c.id === challengeId);

    if (!challenge) {
      throw new Error(`Challenge with ID ${challengeId} not found`);
    }

    return challenge;
  } catch (error) {
    console.error(`Error fetching challenge ${challengeId}:`, error);
    throw error;
  }
};

/**
 * Read a file from the filesystem
 * @param {string} path - Path to the file
 * @returns {Promise<string>} File content
 */
const readFile = async (path) => {
  try {
    // Use axios to fetch the file content from the public directory
    // The path should be relative to the public directory
    const response = await axios.get(path, { 
      headers: { 'Cache-Control': 'no-cache' } 
    });
    return response.data;
  } catch (error) {
    console.error(`Error reading file ${path}:`, error);
    console.error(error.message);
    throw error;
  }
};

/**
 * Get the context code for a challenge
 * @param {number} challengeId - The ID of the challenge
 * @returns {Promise<Array>} Array of context code objects
 */
export const getContextCode = async (challengeId) => {
  try {
    const challenge = await getChallenge(challengeId);

    const contextCode = await Promise.all(
      challenge.contextCodePaths.map(async (path) => {
        const content = await readFile(path);
        const filename = path.split('/').pop();
        return { filename, content };
      })
    );

    // Add the user code file
    const userCodeContent = await readFile(challenge.userCodePath);
    contextCode.push({
      filename: challenge.userCodePath.split('/').pop(),
      content: userCodeContent
    });

    return contextCode;
  } catch (error) {
    console.error(`Error fetching context code for challenge ${challengeId}:`, error);
    throw error;
  }
};

/**
 * Get the hints for a challenge
 * @param {number} challengeId - The ID of the challenge
 * @returns {Promise<Array>} Array of hint objects
 */
export const getHints = async (challengeId) => {
  try {
    const challenge = await getChallenge(challengeId);

    const hints = await Promise.all(
      challenge.tipsPaths.map(async (path, index) => {
        let content = await readFile(path);
        const title = `Hint ${index + 1}`;
        return { id: index + 1, title, content };
      })
    );

    return hints;
  } catch (error) {
    console.error(`Error fetching hints for challenge ${challengeId}:`, error);
    throw error;
  }
};

/**
 * Get the task description for a challenge
 * @param {number} challengeId - The ID of the challenge
 * @returns {Promise<string>} Task description
 */
export const getTaskDescription = async (challengeId) => {
  try {
    const challenge = await getChallenge(challengeId);
    return await readFile(challenge.taskPath);
  } catch (error) {
    console.error(`Error fetching task description for challenge ${challengeId}:`, error);
    throw error;
  }
};
