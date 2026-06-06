# Dockerfile for server app
FROM python:3.9
WORKDIR /app
COPY ../shard-server /app
RUN pip install flask
CMD ["python", "shard_server.py"]