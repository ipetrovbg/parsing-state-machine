# Parsing + State machine
Parsing JSON with serde which become input steps to a state machine written in Rust

#### Run
```shell
cargo run
```

#### Result
<img width="819" alt="Screenshot 2022-02-03 at 23 21 39" src="https://user-images.githubusercontent.com/12900528/152431201-266a5873-de16-4614-9322-459b8c30b0c1.png">



### Example Json definition

#### **convert**

```json
{
  "uuid": "2639f3c3-9e49-4802-92e3-7b8a68c25e4d",
  "next": "point_to_some_other_step_or_leave_empty_to_end",
  "name": "convert_from_string_to_int",
  "createdAt": "2022-01-30T14:47:25.869Z",
  "convert": {
    "from": "string",
    "to": "int",
    "source": "6"
  },
  "errorOnFail": "Failed to convert a string to int",
  "type": "convert"
}
```
```json
{
  "uuid": "2639f3c3-9e49-4802-92e3-7b8a68c25e4d",
  "next": "",
  "name": "convert_from_int_to_string",
  "createdAt": "2022-01-30T14:47:25.869Z",
  "convert": {
    "from": "int",
    "to": "string",
    "source": 6
  },
  "errorOnFail": "Failed to convert an int to string",
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
  "errorOnFail": "My http GET step failed.",
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
  "errorOnFail": "My http POST request step failed.",
  "next": "make_http_request",
  "http": {
    "type": "POST",
    "url": "https://example.com",
    "body": "some body"
  },
  "name": "http_post_call",
  "type": "http"
}
```
