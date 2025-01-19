FROM python:3.10-slim

WORKDIR /src

COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY ./src ./src

EXPOSE ${BIND_PORT}

CMD ["uvicorn", "src.main:app", "--host", "${BIND_IP}", "--port", "${BIND_PORT}"]