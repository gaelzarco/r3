FROM node:latest

RUN npm install -g bun

WORKDIR /app

COPY package*.json ./

RUN bun install

COPY . .

RUN bun run build

EXPOSE 5179

# Command to serve the optimized build using serve
CMD ["serve", "-s", "build", "-l", "5179"]
