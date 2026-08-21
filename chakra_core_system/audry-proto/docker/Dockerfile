FROM python:3.10-slim

WORKDIR /app

# Install shard-server requirements
COPY shard-server/requirements.txt requirements.txt
RUN pip install --no-cache-dir -r requirements.txt && rm requirements.txt

# Copy server code
COPY shard-server/ /app/

# Entrypoint script
COPY docker/shard-server-entry.sh /app/entry.sh
RUN chmod +x /app/entry.sh

EXPOSE 5000
ENTRYPOINT ["./entry.sh"]