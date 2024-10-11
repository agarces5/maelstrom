# Maelstrom

## Init

At the start of a test, Maelstrom issues a single init message to each node, like so:

```init
{
  "type":     "init",
  "msg_id":   1,
  "node_id":  "n3",
  "node_ids": ["n1", "n2", "n3"]
}
{"body":{"type":"init","msg_id":1,"node_id":"n3","node_ids":["n1","n2","n3"]},"dest":"n1","src":"c1"}
```

The node_id field indicates the ID of the node which is receiving this message: here, the node ID is "n3". Your node should remember this ID and include it as the src of any message it sends.

The node_ids field lists all nodes in the cluster, including the recipient. All nodes receive an identical list; you may use its order if you like.

In response to the init message, each node must respond with a message of type init_ok.

```init_ok
{
  "type":        "init_ok",
  "in_reply_to": 1
}
  
```

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
{"body":{"echo":"Please echo 35","msg_id":1,"type":"echo"},"dest":"n1","src":"c1"}
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
{"src":"n1","dest":"c1","body":{"type":"echo_ok","in_reply_to":1,"msg_id":1,"echo":"Please echo 35"}}
```
