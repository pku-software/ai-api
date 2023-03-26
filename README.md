# An simple web server for calling openai APIs

This is a simple web server for calling openai APIs. It is designed for PKU software course (aka. 软件设计实践) project.

** Warning: The project is finished in a harry, filled with awful practices. DO NOT LEARN FROM IT. **

## Installation

```bash
git clone https://github.com/pku-software/ai-api-server.git
cd ai-api-server
cargo build --release
```

## Usage

See the help message for more information.

```bash

ai-api-server -c config.toml -b 0.0.0.0:4399 --debug

```

## Configuration

You can configure the server by editing the `config.toml` file. 

## API

A token is needed for all requests. You should put the token in the `Authorization` header using the `Bearer` scheme.

For example, if your token is `dGVzdA==`, just put it in your header.

```http
Authorization: Basic dGVzdA==
```

### POST /api/ai/v1/translate

Translate text from one language to another.

#### Request

```json
{
    "text": "Hello world!",
    "from": "en",
    "to": "zh"
}
```

#### Response

##### Success

```json
{
    "status": "ok",
    "text": "你好，世界！"
}
```

##### Error

```json
{
    "status": "failed",
    "text": "Invalid language code"
}
```

### POST /api/ai/v1/chat

Interact with a chatbot. You should provide a prompt for the chatbot to generate a completition.

#### Request

```json
{
    "prompt": "Hello, I am a chatbot. How are you?",
    "max_tokens": 10,
    "temperature": 0.9,
    "top_p": 1,
    "frequency_penalty": 0,
    "presence_penalty": 0,
    "stop": ["\n", " Human:", " AI:"]
}
```

#### Response

##### Sucess

```json
{
    "status": "ok",
    "text": "Hello, I am a chatbot. How are you? I am fine. How are you?"
}
```

##### Error

```json
{
    "status": "failed",
    "text": "Network error!"
}
```

### POST /api/ai/v1/draw

Draw a picture. You should provide a prompt for the model to generate a completition.

#### Request

```json
{
    "prompt": "Apple",
    "number": 1,
    "width": 1024,
    "height": 1024
}
```

#### Response

#### Success

Return a base64 encoded image.

```json
{
    "status": "ok",
    "decoded_image": "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAgAAA"
}
```

#### Error

```json
{
    "status": "failed",
    "text": "Network error!"
}
```

### POST /api/ai/v1/wolfram

Ask a question to wolfram alpha, return the answer as a picture.

#### Request

```json
{
    "input": "What is the capital of China?"
}
```

#### Response

#### Success

Return a base64 encoded image.

```json
{
    "status": "ok",
    "decoded_image": "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAgAAA"
}
```

#### Error

```json
{
    "status": "failed",
    "text": "Network error!"
}
```

## License

[MIT](https://choosealicense.com/licenses/mit/)