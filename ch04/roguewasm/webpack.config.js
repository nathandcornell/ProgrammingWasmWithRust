const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');

module.exports = {
    entry: './index.ts',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'index.js',
    },
    rules: {
        use: 'ts-loader',
        exclude: /node_modules/,
    },
    resolve: {
        extensions: ['tsx', '.ts', '.js']
    },
    experiments: {
        asyncWebAssembly: true,
    },
    plugins: [
        new HtmlWebpackPlugin(),
        // This helps some versions of MS Edge with don't have TextEncoder or TextDecoder.
        new webpack.ProvidePlugin({
            TextDecoder: ['text-encoding', 'TextDecoder'],
            TextEncoder: ['text-encoding', 'TextEncoder']
        })
    ],
    mode: 'development'
};
