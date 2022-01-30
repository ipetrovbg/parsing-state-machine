# parsing-state-machine
Parsing State Machine in Rust

#### Run
```shell
cargo run
```

#### Result
![image](https://user-images.githubusercontent.com/12900528/151717032-73735e4b-97b6-433a-bced-cf7310cbebda.png)

### Example Json definition

#### **convert**

```json
{
  "uuid": "2639f3c3-9e49-4802-92e3-7b8a68c25e4d",
  "next": "parse_html_body",
  "createdAt": "2022-01-30T14:47:25.869Z",
  "convert": {
    "from": "string",
    "to": "int"
  },
  "errorOnFail": "Failed to convert from string to int",
  "type": "convert"
}
```

#### **parse**

```json
{
  "uuid": "07124d66-f345-4c0b-90cc-5d45efbb0891",
  "createdAt": "2022-01-30T14:40:46.559Z",
  "errorOnFail": "Failed to parse the html",
  "next": "post_http_call",
  "name": "parse_html_body",
  "parse": {
    "type": "document",
    "content": "<html></html>"
  },
  "type": "parse"
}
```

#### **http**

```json
{
  "uuid": "a204db2d-3c36-43ed-98d1-7700a3ad622a",
  "createdAt": "2022-01-30T14:33:52.251Z",
  "errorOnFail": "My http step failed",
  "http": {
    "type": "GET",
    "url": "https://example.com"
  },
  "name": "make_http_request",
  "type": "http"
}
```

```json
{
  "uuid": "a204db2d-3c36-43ed-98d1-7700a3ad622a",
  "createdAt": "2022-01-30T14:33:52.251Z",
  "errorOnFail": "My http post step failed",
  "next": "make_http_request",
  "http": {
    "type": "POST",
    "url": "https://example.com",
    "body": "some body"
  },
  "name": "post_http_call",
  "type": "http"
}
```