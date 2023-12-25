//! Defines Non-Fungible Token Transfer (ICS-721) event types.
use ibc_core::channel::types::acknowledgement::AcknowledgementStatus;
use ibc_core::primitives::prelude::*;
use ibc_core::primitives::Signer;
use ibc_core::router::types::event::ModuleEvent;

use super::Memo;
use crate::{PrefixedClass, TokenId, MODULE_ID_STR};

const EVENT_TYPE_PACKET: &str = "nft_packet";
const EVENT_TYPE_TIMEOUT: &str = "timeout";
const EVENT_TYPE_CLASS_TRACE: &str = "nft_class_trace";
const EVENT_TYPE_TRANSFER: &str = "ibc_nft_transfer";

/// Contains all events variants that can be emitted from the NFT transfer application
pub enum Event {
    Recv(RecvEvent),
    Ack(AckEvent),
    AckStatus(AckStatusEvent),
    Timeout(TimeoutEvent),
    ClassTrace(ClassTraceEvent),
    Transfer(TransferEvent),
}

/// Event emitted by the `onRecvPacket` module callback to indicate the that the
/// `RecvPacket` message was processed
pub struct RecvEvent {
    pub sender: Signer,
    pub receiver: Signer,
    pub class: PrefixedClass,
    pub token: TokenId,
    pub memo: Memo,
    pub success: bool,
}

impl From<RecvEvent> for ModuleEvent {
    fn from(ev: RecvEvent) -> Self {
        let RecvEvent {
            sender,
            receiver,
            class,
            token,
            memo,
            success,
        } = ev;
        Self {
            kind: EVENT_TYPE_PACKET.to_string(),
            attributes: vec![
                ("module", MODULE_ID_STR).into(),
                ("sender", sender).into(),
                ("receiver", receiver).into(),
                ("class", class).into(),
                ("token", token).into(),
                ("memo", memo).into(),
                ("success", success).into(),
            ],
        }
    }
}

/// Event emitted in the `onAcknowledgePacket` module callback
pub struct AckEvent {
    pub sender: Signer,
    pub receiver: Signer,
    pub class: PrefixedClass,
    pub token: TokenId,
    pub memo: Memo,
    pub acknowledgement: AcknowledgementStatus,
}

impl From<AckEvent> for ModuleEvent {
    fn from(ev: AckEvent) -> Self {
        let AckEvent {
            sender,
            receiver,
            class,
            token,
            memo,
            acknowledgement,
        } = ev;
        Self {
            kind: EVENT_TYPE_PACKET.to_string(),
            attributes: vec![
                ("module", MODULE_ID_STR).into(),
                ("sender", sender).into(),
                ("receiver", receiver).into(),
                ("class", class).into(),
                ("token", token).into(),
                ("memo", memo).into(),
                ("acknowledgement", acknowledgement).into(),
            ],
        }
    }
}

/// Event emitted in the `onAcknowledgePacket` module callback to indicate
/// whether the acknowledgement is a success or a failure
pub struct AckStatusEvent {
    pub acknowledgement: AcknowledgementStatus,
}

impl From<AckStatusEvent> for ModuleEvent {
    fn from(ev: AckStatusEvent) -> Self {
        let AckStatusEvent { acknowledgement } = ev;
        let attr_label = match acknowledgement {
            AcknowledgementStatus::Success(_) => "success",
            AcknowledgementStatus::Error(_) => "error",
        };

        Self {
            kind: EVENT_TYPE_PACKET.to_string(),
            attributes: vec![(attr_label, acknowledgement.to_string()).into()],
        }
    }
}

/// Event emitted in the `onTimeoutPacket` module callback
pub struct TimeoutEvent {
    pub refund_receiver: Signer,
    pub refund_class: PrefixedClass,
    pub refund_token: TokenId,
    pub memo: Memo,
}

impl From<TimeoutEvent> for ModuleEvent {
    fn from(ev: TimeoutEvent) -> Self {
        let TimeoutEvent {
            refund_receiver,
            refund_class,
            refund_token,
            memo,
        } = ev;
        Self {
            kind: EVENT_TYPE_TIMEOUT.to_string(),
            attributes: vec![
                ("module", MODULE_ID_STR).into(),
                ("refund_receiver", refund_receiver).into(),
                ("refund_class", refund_class).into(),
                ("refund_token", refund_token).into(),
                ("memo", memo).into(),
            ],
        }
    }
}

/// Event emitted in the `onRecvPacket` module callback when new tokens are minted
pub struct ClassTraceEvent {
    pub trace_hash: Option<String>,
    pub class: PrefixedClass,
}

impl From<ClassTraceEvent> for ModuleEvent {
    fn from(ev: ClassTraceEvent) -> Self {
        let ClassTraceEvent { trace_hash, class } = ev;
        let mut ev = Self {
            kind: EVENT_TYPE_CLASS_TRACE.to_string(),
            attributes: vec![("class", class).into()],
        };
        if let Some(hash) = trace_hash {
            ev.attributes.push(("trace_hash", hash).into());
        }
        ev
    }
}

/// Event emitted after a successful `sendTransfer`
pub struct TransferEvent {
    pub sender: Signer,
    pub receiver: Signer,
    pub class: PrefixedClass,
    pub token: TokenId,
    pub memo: Memo,
}

impl From<TransferEvent> for ModuleEvent {
    fn from(ev: TransferEvent) -> Self {
        let TransferEvent {
            sender,
            receiver,
            class,
            token,
            memo,
        } = ev;

        Self {
            kind: EVENT_TYPE_TRANSFER.to_string(),
            attributes: vec![
                ("sender", sender).into(),
                ("receiver", receiver).into(),
                ("class", class).into(),
                ("token", token).into(),
                ("memo", memo).into(),
            ],
        }
    }
}

impl From<Event> for ModuleEvent {
    fn from(ev: Event) -> Self {
        match ev {
            Event::Recv(ev) => ev.into(),
            Event::Ack(ev) => ev.into(),
            Event::AckStatus(ev) => ev.into(),
            Event::Timeout(ev) => ev.into(),
            Event::ClassTrace(ev) => ev.into(),
            Event::Transfer(ev) => ev.into(),
        }
    }
}
