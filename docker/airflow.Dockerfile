# docker\airflow.Dockerfile
FROM apache/airflow:3.1.8

USER root

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        curl \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

USER airflow

RUN pip install --no-cache-dir \
    requests==2.32.3