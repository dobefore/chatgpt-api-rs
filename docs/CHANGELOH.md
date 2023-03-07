It takes me 17 minutes to create new crates and import crate openai-api-client locally.

Search how to set a proxy for a specific web application on Linux.

try to enable proxy settings in actix-web application,test it by access google.com using the proxy server extracted from vmess address.

Aim: A web server application that forwords respinse of chatgpt using api token.

divide into smaller tasks:
use actox-web to construct a server frame work
set env vars: api key.

copy actix-web related code from anki-sync-server-rs and crates.create server module to store actix-web code.copy server code from actix-web example repo. 

A handler to show welcome page: 
componets of welcome page: question input box and output box.
description of chatgpt: a html file where it contains a text input box and a triangle button and an output box.
When the triangle button is clicked,the string in the input box will be sent over to server with uri /ask.And output box will display the response string

create an error moudule in crate chatgpt-api allow it to be imported into current project

add a wrapper for the function completions to reduce unnecessary arguments like max token and model.

welcome page should be routed by /

Now test on windows,when enter /,html file is printed on browser in plain text.To fix it,import crate actix-files,but Specified path is not a directory: "/static/".It seems the static folder should be placed in the same dir as executable

encounter 405 errpr code.the request seems not to enter the handler,it is caused by my carelessness to set handler to go get method,so set post method.

chatgpt return undefined,through debug handler ask,chatgpt return normally,the problem is with js parse response in client. by using json.parse method to parse json object,finally I made it.

## handler module
create a handler module file to store handlers.
1. ask handler. make response to the request /ask. In it,chatgpt api will be requested (in home proxy will be used).Use the response as output response. make request to chatgpt and get its response
 
 parse api token of openai from env var


## error module
create an enum named Error.
copy code from anki-sync-server-rs


create a global var for vpi url
add str::parse error to ereror,find no error handling in it,so give up.
## new model gpt35-turbo
create a response struct to be deserilized.

Finish api converion in rust.

chatgpt api client error: Json parsing error: missing field `id` at line 8 column 1

know the reason from response from chatgpt \"message\": \"That model does not exist\",I mis-write the model name.

\"'messages' is a required property\",replace message with messages in request field.

\"message\": \"'introduce yourself' is not one of ['system', 'assistant', 'user'] - 'messages.0.role'\": add role and content to map 

\"message\": \"This model's maximum context length is 4096 tokens. However, you requested 4106 tokens (10 in the messages, 4096 in the completion). Please reduce the length of the messages or completion.\",\. allow this field to be None and will be skipped when serilizing.

chatgpt api client error: Json parsing error: invalid type: null, expected a string at line 1 column 616: It is weird, I work it out normally before,this time error appears. 