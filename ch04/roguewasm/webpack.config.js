const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');

module.exports = {
    entry: './index.ts',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'index.js',
    },
    devtool: 'inline-source-map',
    module: {
        rules: [{
            test: /\.ts?$/,
            use: 'ts-loader',
            exclude: /node_modules/,
        }]
    },
    resolve: {
        extensions: ['tsx', '.ts', '.js']
    },
    experiments: {
        asyncWebAssembly: true,
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: path.resolve(__dirname, 'index.html')
        }),
        // This helps some versions of MS Edge with don't have TextEncoder or TextDecoder.
        new webpack.ProvidePlugin({
            TextDecoder: ['text-encoding', 'TextDecoder'],
            TextEncoder: ['text-encoding', 'TextEncoder']
        })
    ],
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'index.js',
    },
    mode: 'development'
};
