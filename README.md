# Maelstrom

## Echo [https://fly.io/dist-sys/1/](https://fly.io/dist-sys/1/)

In Maelstrom, we create a node which is a binary that receives JSON messages from STDIN and sends JSON messages to STDOUT.

```echo
{
  "src": "c1",
  "dest": "n1",
  "body": {
    "type": "echo",
    "msg_id": 1,
    "echo": "Please echo 35"
  }
}
```

Your job is to send a message with the same body back to the client but with a message type of "echo_ok". It should also associate itself with the original message by setting the "in_reply_to" field to the original message ID.

```echo_ok
{
  "src": "n1",
  "dest": "c1",
  "body": {
    "type": "echo_ok",
    "msg_id": 1,
    "in_reply_to": 1,
    "echo": "Please echo 35"
  }
}
```
