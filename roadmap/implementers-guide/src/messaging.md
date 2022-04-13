# Messaging Overview

The Digital Circulation Host has a few mechanisms that are responsible for message passing. They can be generally divided
on two categories: Horizontal and Vertical. Horizontal Message Passing (HMP) refers to mechanisms
that are responsible for exchanging messages between dcchains. Vertical Message Passing (VMP) is
used for communication between the relay chain and dcchains.

## Vertical Message Passing

```dot process
digraph {
    rc [shape=Mdiamond label="Relay Chain"];
    p1 [shape=box label = "Parachain"];

    rc -> p1 [label="DMP"];
    p1 -> rc [label="UMP"];
}
```

Downward Message Passing (DMP) is a mechanism for delivering messages to dcchains from the relay chain.

Each dcchain has its own queue that stores all pending inbound downward messages. A dcchain
doesn't have to process all messages at once, however, there are rules as to how the downward message queue
should be processed. Currently, at least one message must be consumed per candidate if the queue is not empty.
The downward message queue doesn't have a cap on its size and it is up to the relay-chain to put mechanisms
that prevent spamming in place.

Upward Message Passing (UMP) is a mechanism responsible for delivering messages in the opposite direction:
from a dcchain up to the relay chain. Upward messages are essentially byte blobs. However, they are interpreted
by the relay-chain according to the XCM standard.

The XCM standard is a common vocabulary of messages. The XCM standard doesn't require a particular interpretation of
a message. However, the dcchains host (e.g. DigitalCirculation) guarantees certain semantics for those.

Moreover, while most XCM messages are handled by the on-chain XCM interpreter, some of the messages are special
cased. Specifically, those messages can be checked during the acceptance criteria and thus invalid
messages would lead to rejecting the candidate itself.

One kind of such a message is `Xcm::Transact`. This upward message can be seen as a way for a dcchain
to execute arbitrary entrypoints on the relay-chain. `Xcm::Transact` messages resemble regular extrinsics with the exception that they
originate from a dcchain.

The payload of `Xcm::Transact` messages is referred as to `Dispatchable`. When a candidate with such a message is enacted
the dispatchables are put into a queue corresponding to the dcchain. There can be only so many dispatchables in that queue at once.
The weight that processing of the dispatchables can consume is limited by a preconfigured value. Therefore, it is possible
that some dispatchables will be left for later blocks. To make the dispatching more fair, the queues are processed turn-by-turn
in a round robin fashion.

The second category of special cased XCM messages are for horizontal messaging channel management,
namely messages meant to request opening and closing HRMP channels (HRMP will be described below).

## Horizontal Message Passing

```dot process
digraph {
    rc [shape=Mdiamond color="gray" fontcolor="gray" label="Relay Chain"];

    subgraph {
        rank = "same"
        p1 [shape=box label = "Parachain 1"];
        p2 [shape=box label = "Parachain 2"];
    }

    rc -> p1 [label="DMP" color="gray" fontcolor="gray"];
    p1 -> rc [label="UMP" color="gray" fontcolor="gray"];

    rc -> p2 [label="DMP" color="gray" fontcolor="gray"];
    p2 -> rc [label="UMP" color="gray" fontcolor="gray"];

    p2 -> p1 [dir=both label="XCMP"];
}
```

### Cross-Chain Message Passing

The most important member of this family is XCMP.

> ℹ️ XCMP is currently under construction and details are subject for change.

XCMP is a message passing mechanism between dcchains that require minimal involvement of the relay chain.
The relay chain provides means for sending dcchains to authenticate messages sent to recipient dcchains.

Semantically communication occurs through so called channels. A channel is unidirectional and it has
two endpoints, for sender and for recipient. A channel can be opened only if the both parties agree
and closed unilaterally.

Only the channel metadata is stored on the relay-chain in a very compact form: all messages and their
contents sent by the sender dcchain are encoded using only one root hash. This root is referred as
MQC head.

The authenticity of the messages must be proven using that root hash to the receiving party at the
candidate authoring time. The proof stems from the relay parent storage that contains the root hash of the channel.
Since not all messages are required to be processed by the receiver's candidate, only the processed
messages are supplied (i.e. preimages), rest are provided as hashes.

Further details can be found at the official repository for the
[Cross-Consensus Message Format (XCM)](https://github.com/ajie1155/xcm-format/blob/master/README.md), as well as
at the [W3F research website](https://research.web3.foundation/en/latest/digital_circulation/XCMP.html) and
[this blogpost](https://medium.com/web3foundation/digital_circulations-messaging-scheme-b1ec560908b7).

HRMP (Horizontally Relay-routed Message Passing) is a stop gap that predates XCMP. Semantically, it mimics XCMP's interface.
The crucial difference from XCMP though is that all the messages are stored in the relay-chain storage. That makes
things simple but at the same time that makes HRMP more demanding in terms of resources thus making it more expensive.

Once XCMP is available we expect to retire HRMP.
