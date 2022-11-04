response object:
    {
        "d": "base64 encoded gzip compressed json string"
    }

d -> single string

1. gzip decompress
2. (parse json)
3. get value from "d"