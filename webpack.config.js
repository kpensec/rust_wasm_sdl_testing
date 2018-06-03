'use strict';

const path = require('path');
const webpack = require('webpack');

module.exports = {
  module: {
    rules: [
      {
        oneOf: [
          {
            test: /\.js$/,
            exclude: /node_modules/,
            loader: 'babel-loader',

            options: {
              presets: ['env']
            }
          },
          {
            test: /\.css$/,

            use: [
              {
                loader: 'style-loader',

                options: {
                  sourceMap: true
                }
              },
              {
                loader: 'css-loader'
              }
            ]
          },
          //{
          //  test: /\.rs$/,
          //  use: {
          //    loader: 'rust-emscripten-loader',
          //    options: {
          //      path: '',
          //      target: 'wasm',
          //      outName: 'game.wasm'
          //    }
          //  }
          //}
        ]
      }
    ]
  },

  entry: './src',

  output: {
    filename: '[name].[chunkhash].js',
    chunkFilename: '[name].[chunkhash].js',
    path: path.resolve(__dirname, 'dist')
  },

  mode: 'development',
  node: {
      fs: "empty"
  }
};
