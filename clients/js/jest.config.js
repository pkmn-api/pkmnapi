module.exports = {
    roots: [
        '<rootDir>/tests'
    ],
    testMatch: [
        '**/__tests__/**/*.+(ts|js)',
        '**/?(*.)+(spec|test).+(ts|js)'
    ],
    transform: {
        '^.+\\.[jt]s$': 'ts-jest'
    },
    testEnvironment: 'node'
};
