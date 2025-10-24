FROM public.ecr.aws/lambda/python:3.11

WORKDIR /

RUN yum install gcc -y

COPY . ${LAMBDA_TASK_ROOT}

COPY requirements.txt .

RUN pip install -r requirements.txt --target "${LAMBDA_TASK_ROOT}" -U --no-cache-dir


CMD ["main.handler"]