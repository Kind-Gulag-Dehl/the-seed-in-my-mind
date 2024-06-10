const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack'); // Import webpack
require('dotenv').config(); // Require dotenv to load the .env variables

module.exports = {
  entry: './src/index.js', // Entry point relative to the location of this file
  output: {
    path: path.resolve(__dirname, 'build'), // Output directory
    filename: 'bundle.js', // Output bundle file
    publicPath: '/', // Ensures all assets are served from the root
  },
  module: {
    rules: [
      {
        test: /\.(js|jsx)$/, // Matches JavaScript and JSX files
        exclude: /node_modules/, // Excludes the node_modules directory
        use: {
          loader: 'babel-loader', // Uses Babel to transpile JS and JSX files
          options: {
            presets: ['@babel/preset-env', '@babel/preset-react'], // Transpilation presets for modern JS and React
          },
        },
      },
      {
        test: /\.css$/, // Matches CSS files
        use: ['style-loader', 'css-loader'], // Uses style-loader and css-loader for CSS files
      },
      // Add other loaders for different file types as needed
    ],
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: './src/index.html', // Specifies the HTML template to use
      filename: 'index.html', // Output filename in the build folder
    }),
    new webpack.DefinePlugin({
      'process.env': {
        REACT_APP_API_BASE_URL: JSON.stringify(process.env.REACT_APP_API_BASE_URL),
      },
    }),
  ],
  devServer: {
    static: {
      directory: path.join(__dirname, 'build'), // Updated from 'contentBase' to 'static.directory'
    },
    compress: true, // Enables gzip compression
    port: 3000, // Port to serve the application on
    historyApiFallback: true, // Essential for single-page applications using routing
    open: true, // Automatically opens the browser when the server starts
    hot: true, // Enables hot module replacement
  },
};
