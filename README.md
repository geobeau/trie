

```mermaid
stateDiagram-v2
    node1: 1001 / max
    node2: 1001 / max
    node3: 1001 / max
    [*] --> node1

    node1 --> node2
    node1 --> [*]

    node2 --> node3
    node2 --> [*]

```

1001 1011 1111

1101 1111 1101


```mermaid
stateDiagram-v2
    node1: 1001 / 1 / depth-0

    node2: 1011 / 8 / depth-1
    node3: 1111 / 8 / depth-2

    node4: 1101 / 8 / depth-0
    node5: 1111 / 8 / depth-1
    node6: 1101 / 8 / depth-2
    [*] --> node1

    node1 --> node2
    node1 --> node4

    node2 --> node3
    node2 --> [*]
    node4 --> node5
    node5 --> node6

```

node1 xor 1101 = 0100
offset = 1

------------

1001 1011 1111

1101 1111 1101

1011 0101 1001

```mermaid
stateDiagram-v2
    node1: 1001 / 1 / depth-0
    node1bis: 1001 / 2 / depth-0

    node2: 1011 / 8 / depth-1
    node3: 1111 / 8 / depth-2

    node4: 1101 / 8 / depth-0
    node5: 1111 / 8 / depth-1
    node6: 1101 / 8 / depth-2

    node7: 1011 / 8 / depth-0
    [*] --> node1

    node1 --> node1bis
    node1bis --> node2
    node1bis --> node7
    node1 --> node4

    node2 --> node3
    node2 --> [*]
    node4 --> node5
    node5 --> node6

```


Attention node1 -> node2 how to change depth?

1001 xor 1011 = 0010
offset = 2

