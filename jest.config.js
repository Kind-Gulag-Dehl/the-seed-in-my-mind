// jest.config.js
module.exports = {
  projects: [
    {
      displayName: "server",
      testEnvironment: "node",
      setupFilesAfterEnv: ["<rootDir>/jest.server.setup.js"],
      testMatch: ["<rootDir>/tests/**/*.test.js"]
      // Additional server-specific configurations can be added here
    },
    {
      displayName: "client",
      testEnvironment: "jsdom",
      setupFilesAfterEnv: ["<rootDir>/jest.client.setup.js"],
      testMatch: ["<rootDir>/client/src/**/*.test.js"],
      moduleNameMapper: {
        '\\.(css|less)$': '<rootDir>/__mocks__/stylemock.js'
      }
      // Additional client-specific configurations can be added here
    }
  ]
};
