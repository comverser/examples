# dataset
- Total Objects !== files number
```bash
aws --endpoint-url=https://kr.object.ncloudstorage.com s3 ls --summarize s3://aidata-2021-01-052/"088.주차 공간 탐색을 위한 차량 관점 복합 데이터"/06.품질검증/1.Dataset/
```
```bash
aws --endpoint-url=https://kr.object.ncloudstorage.com s3 sync s3://aidata-2021-01-052/"088.주차 공간 탐색을 위한 차량 관점 복합 데이터"/06.품질검증/1.Dataset/ /data/original/raw/
```
```bash
aws --endpoint-url=https://kr.object.ncloudstorage.com s3 ls s3://aidata-2021-01-052/"088.주차 공간 탐색을 위한 차량 관점 복합 데이터"/06.품질검증/01.Dataset/ | grep PRE | wc -l
```


# S3
```bash
aws s3 sync <source> <target> [--options]
```


# EC2
## create free tier instance
```bash
aws ec2 run-instances --image-id ami-0eb14fe5735c13eb5 --instance-type t2.micro --key-name <key pairs>
```

# Doc
https://docs.aws.amazon.com/index.html


# Mis
```bash
aws sts get-caller-identity --profile <profile_name>
aws configure list --profile <profile_name>
```

# install
https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html

# configure
```bash
aws configure --profile <profile_name>
```

# setting dir
```bash
cd ~/.aws
```
