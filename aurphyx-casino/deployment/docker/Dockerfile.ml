FROM python:3.10-slim

WORKDIR /app

COPY python/requirements.txt ./
RUN pip install --no-cache-dir -r requirements.txt

COPY python ./python

EXPOSE 8001

CMD ["uvicorn", "python.ml.service:app", "--host", "0.0.0.0", "--port", "8001"]

