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
{"src":"n1","dest":"c1","body":{"type":"init_ok","msg_id":1}}
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

# Generate ID [https://fly.io/dist-sys/2/](https://fly.io/dist-sys/2/)

In this challenge, you’ll need to implement a globally-unique ID generation system that runs against Maelstrom’s unique-ids workload. Your service should be totally available, meaning that it can continue to operate even in the face of network partitions.

Your node will receive a request message body that looks like this:

```generate
{
  "type": "generate"
}
{"src":"c1","dest":"n1","body":{"type":"generate","msg_id":1}}
```

and it will need to return a "generate_ok" message with a unique ID:

```generate_ok
{
  "type": "generate_ok",
  "id": 123
}
{"src":"n1","dest":"c1","body":{"type":"generate_ok","in_reply_to":1,"msg_id":1,"id":"n1-1"}}
```

IDs may be of any type–strings, booleans, integers, floats, arrays, etc.

# Broadcast [https://fly.io/dist-sys/3/](https://fly.io/dist-sys/3/)

In this challenge, you’ll need to implement a broadcast system that gossips messages between all nodes in the cluster.

## Single-node broadcast

Your node will need to handle the "broadcast" workload which has 3 RPC message types: broadcast, read, & topology. Your node will need to store the set of integer values that it sees from broadcast messages so that they can be returned later via the read message RPC.

Data can be stored in-memory as node processes are not killed by Maelstrom.

### RPC: broadcast

This message requests that a value be broadcast out to all nodes in the cluster. The value is always an integer and it is unique for each message from Maelstrom.

Your node will receive a request message body that looks like this:

```broadcast
{
  "type": "broadcast",
  "message": 1000
}
{"src":"c1","dest":"n1","body":{"type":"broadcast","message":1}}
```

It should store the "message" value locally so it can be read later. In response, it should send an acknowledge with a broadcast_ok message:

```broadcast_ok
{
  "type": "broadcast_ok"
}
{"src":"n1","dest":"c1","body":{"type":"broadcast_ok"}}
```

### RPC: read

This message requests that a node return all values that it has seen.

Your node will receive a request message body that looks like this:

```read
{
  "type": "read"
}
{"src":"c1","dest":"n1","body":{"type":"read"}}
```

In response, it should return a read_ok message with a list of values it has seen:

```read_ok
{
  "type": "read_ok",
  "messages": [1, 8, 72, 25]
}
{"src":"n1","dest":"c1","body":{"type":"read_ok","messages":[1, 8, 72, 25]}}
```

The order of the returned values does not matter.

### RPC: topology

This message informs the node of who its neighboring nodes are. Maelstrom has multiple topologies available or you can ignore this message and make your own topology from the list of nodes in the Node.NodeIDs() method. All nodes can communicate with each other regardless of the topology passed in.

Your node will receive a request message body that looks like this:

```topology
{
  "type": "topology",
  "topology": {
    "n1": ["n2", "n3"],
    "n2": ["n1"],
    "n3": ["n1"]
  }
}
{"src":"c1","dest":"n1","body":{"type":"topology","topology":{"n1":["n2","n3"],"n2":["n1"],"n3":["n1"]}}}
```

In response, your node should return a topology_ok message body:

```topology_ok
{
  "type": "topology_ok"
}
{"src":"n1","dest":"c1","body":{"type":"topology_ok"}}
```

### Broadcast: Multi Node

Your node should propagate values it sees from broadcast messages to the other nodes in the cluster. It can use the topology passed to your node in the topology message or you can build your own topology.

The simplest approach is to simply send a node’s entire data set on every message, however, this is not practical in a real-world system. Instead, try to send data more efficiently as if you were building a real broadcast system.

Values should propagate to all other nodes within a few seconds.
