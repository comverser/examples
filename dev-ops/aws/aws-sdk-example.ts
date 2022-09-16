import * as path from "path";
import * as AWS from "aws-sdk";
import { load } from "ts-dotenv";

const env = load({
    ACCESS_KEY: String,
    SECRET_KEY: String,
    PROJECT_NO: String,
    DATA_NO: String,
    SUBACCOUNT_NAME: String,
    ENDPOINT_URL: String,
});

// Create S3 service object
const ep = new AWS.Endpoint(env.ENDPOINT_URL);
const s3 = new AWS.S3({
    accessKeyId: env.ACCESS_KEY,
    secretAccessKey: env.SECRET_KEY,
    endpoint: ep,
});

const dirs = {
    lv0: "088.주차 공간 탐색을 위한 차량 관점 복합 데이터",
    lv1: "06.품질검증",
    lv2: "1.Dataset",
    lv3: ["1_원천데이터", "2_라벨링데이터"],
};
const Bucket = env.SUBACCOUNT_NAME;
const Prefix = path.join(dirs.lv0, dirs.lv1, dirs.lv2);

const StartAfter = path.join(dirs.lv0, dirs.lv1, dirs.lv2, dirs.lv3[0]);
const Delimiter = "Camera";
const MaxKeys = 10;

const bucketParams = {
    Bucket,
    Prefix,
    StartAfter,
    Delimiter,
    MaxKeys,
};

// Call S3 to obtain a list of the objects in the bucket
s3.listObjectsV2(bucketParams, (err, data) => {
    if (err) {
        console.log("Error", err);
    } else {
        // console.log(Object.keys(data));
        const contents = data.Contents;
        contents.forEach((content) => console.log(content.Key));
    }
});
