object:
    {
        "UserId": "",
        "UserPw": "",
        "AppVersion": "2.5.9",
        "Language": "de",
        "OsVersion": "28 8.0",
        "AppId": "uuid",
        "Device": "Spaceshuttle",
        "BundleId": "de.heinekingmedia.dsbmobile",
        "Date": "date now",
        "LastUpdate": "date now" 
    }

(all utf-8)
1: JSON stringify
2: gzip compress
2: Base64 encode

request object:
    {
        "req": {
            "Data": "base64 encoded gzip compressed json string",
            "DataType": 1,
        }
    }

request method: POST
request url: https://app.dsbcontrol.de/JsonHandler.ashx/GetData

header:
    Content-Type = application/json