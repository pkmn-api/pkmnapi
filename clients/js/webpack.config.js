// libs
const path = require('path');

// variables
const SRC_DIR = path.resolve(__dirname, './src');
const BUILD_DIR = path.resolve(__dirname, './dist');

// configs
const baseConfig = {
    entry: path.resolve(SRC_DIR, './index.ts'),
    output: {
        filename: 'Pkmnapi.js',
        path: BUILD_DIR,
        library: 'Pkmnapi',
        libraryTarget: 'umd'
    },
    module: {
        rules: [
            {
                test: /\.[jt]s$/,
                exclude: /node_modules/,
                use: 'ts-loader'
            }
        ]
    },
    resolve: {
        extensions: ['.ts', '.js']
    }
};

const nodeConfig = {
    ...baseConfig,
    target: 'node',
    output: {
        ...baseConfig.output,
        filename: 'Pkmnapi.node.js'
    }
};

const webConfig = {
    ...baseConfig
};

module.exports = [
    nodeConfig,
    webConfig
];
