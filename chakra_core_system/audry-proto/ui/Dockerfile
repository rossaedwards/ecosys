# Use official Node.js image
FROM node:18-alpine

# Set working directory
WORKDIR /app

# Install dependencies first (better caching)
COPY package.json package-lock.json* ./
RUN npm install

# Copy the rest of the UI source
COPY . .

# Expose React dev server port
EXPOSE 3000

# Start the React app
CMD ["npm", "start"]