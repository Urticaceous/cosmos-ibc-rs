use crate::core::handler::{ExecutionHandler, ValidationHandler};
use crate::core::ics02_client::handler as client_handler;
use crate::core::ics03_connection::handler as conn_handler;
use crate::core::{ContextError, KeeperContext, ReaderContext};
use crate::prelude::*;

use ibc_proto::google::protobuf::Any;

use crate::core::ics02_client::msgs::{create_client, update_client, upgrade_client, ClientMsg};
use crate::core::ics03_connection::msgs::{
    conn_open_ack, conn_open_confirm, conn_open_init, conn_open_try, ConnectionMsg,
};
use crate::core::ics04_channel::msgs::{
    acknowledgement, chan_close_confirm, chan_close_init, chan_open_ack, chan_open_confirm,
    chan_open_init, chan_open_try, recv_packet, timeout, timeout_on_close, ChannelMsg, PacketMsg,
};
use crate::core::ics26_routing::error::RouterError;
use ibc_proto::protobuf::Protobuf;

/// Enumeration of all messages that the local ICS26 module is capable of routing.
#[derive(Clone, Debug)]
pub enum MsgEnvelope {
    Client(ClientMsg),
    Connection(ConnectionMsg),
    Channel(ChannelMsg),
    Packet(PacketMsg),
}

impl TryFrom<Any> for MsgEnvelope {
    type Error = RouterError;

    fn try_from(any_msg: Any) -> Result<Self, Self::Error> {
        match any_msg.type_url.as_str() {
            // ICS2 messages
            create_client::TYPE_URL => {
                // Pop out the message and then wrap it in the corresponding type.
                let domain_msg = create_client::MsgCreateClient::decode_vec(&any_msg.value)
                    .map_err(RouterError::MalformedMessageBytes)?;
                Ok(MsgEnvelope::Client(ClientMsg::CreateClient(domain_msg)))
            }
            update_client::TYPE_URL => {
                let domain_msg = update_client::MsgUpdateClient::decode_vec(&any_msg.value)
                    .map_err(RouterError::MalformedMessageBytes)?;
                Ok(MsgEnvelope::Client(ClientMsg::UpdateClient(domain_msg)))
            }
            upgrade_client::TYPE_URL => {
                let domain_msg = upgrade_client::MsgUpgradeClient::decode_vec(&any_msg.value)
                    .map_err(RouterError::MalformedMessageBytes)?;
                Ok(MsgEnvelope::Client(ClientMsg::UpgradeClient(domain_msg)))
            }

            // ICS03
            conn_open_init::TYPE_URL => {
                let domain_msg = conn_open_init::MsgConnectionOpenInit::decode_vec(&any_msg.value)
                    .map_err(RouterError::MalformedMessageBytes)?;
                Ok(MsgEnvelope::Connection(ConnectionMsg::OpenInit(domain_msg)))
            }
            conn_open_try::TYPE_URL => {
                let domain_msg = conn_open_try::MsgConnectionOpenTry::decode_vec(&any_msg.value)
                    .map_err(RouterError::MalformedMessageBytes)?;
                Ok(MsgEnvelope::Connection(ConnectionMsg::OpenTry(domain_msg)))
            }
            conn_open_ack::TYPE_URL => {
                let domain_msg = conn_open_ack::MsgConnectionOpenAck::decode_vec(&any_msg.value)
                    .map_err(RouterError::MalformedMessageBytes)?;
                Ok(MsgEnvelope::Connection(ConnectionMsg::OpenAck(domain_msg)))
            }
            conn_open_confirm::TYPE_URL => {
                let domain_msg =
                    conn_open_confirm::MsgConnectionOpenConfirm::decode_vec(&any_msg.value)
                        .map_err(RouterError::MalformedMessageBytes)?;
                Ok(MsgEnvelope::Connection(ConnectionMsg::OpenConfirm(
                    domain_msg,
                )))
            }

            // ICS04 channel messages
            chan_open_init::TYPE_URL => {
                let domain_msg = chan_open_init::MsgChannelOpenInit::decode_vec(&any_msg.value)
                    .map_err(RouterError::MalformedMessageBytes)?;
                Ok(MsgEnvelope::Channel(ChannelMsg::OpenInit(domain_msg)))
            }
            chan_open_try::TYPE_URL => {
                let domain_msg = chan_open_try::MsgChannelOpenTry::decode_vec(&any_msg.value)
                    .map_err(RouterError::MalformedMessageBytes)?;
                Ok(MsgEnvelope::Channel(ChannelMsg::OpenTry(domain_msg)))
            }
            chan_open_ack::TYPE_URL => {
                let domain_msg = chan_open_ack::MsgChannelOpenAck::decode_vec(&any_msg.value)
                    .map_err(RouterError::MalformedMessageBytes)?;
                Ok(MsgEnvelope::Channel(ChannelMsg::OpenAck(domain_msg)))
            }
            chan_open_confirm::TYPE_URL => {
                let domain_msg =
                    chan_open_confirm::MsgChannelOpenConfirm::decode_vec(&any_msg.value)
                        .map_err(RouterError::MalformedMessageBytes)?;
                Ok(MsgEnvelope::Channel(ChannelMsg::OpenConfirm(domain_msg)))
            }
            chan_close_init::TYPE_URL => {
                let domain_msg = chan_close_init::MsgChannelCloseInit::decode_vec(&any_msg.value)
                    .map_err(RouterError::MalformedMessageBytes)?;
                Ok(MsgEnvelope::Channel(ChannelMsg::CloseInit(domain_msg)))
            }
            chan_close_confirm::TYPE_URL => {
                let domain_msg =
                    chan_close_confirm::MsgChannelCloseConfirm::decode_vec(&any_msg.value)
                        .map_err(RouterError::MalformedMessageBytes)?;
                Ok(MsgEnvelope::Channel(ChannelMsg::CloseConfirm(domain_msg)))
            }
            // ICS04 packet messages
            recv_packet::TYPE_URL => {
                let domain_msg = recv_packet::MsgRecvPacket::decode_vec(&any_msg.value)
                    .map_err(RouterError::MalformedMessageBytes)?;
                Ok(MsgEnvelope::Packet(PacketMsg::Recv(domain_msg)))
            }
            acknowledgement::TYPE_URL => {
                let domain_msg = acknowledgement::MsgAcknowledgement::decode_vec(&any_msg.value)
                    .map_err(RouterError::MalformedMessageBytes)?;
                Ok(MsgEnvelope::Packet(PacketMsg::Ack(domain_msg)))
            }
            timeout::TYPE_URL => {
                let domain_msg = timeout::MsgTimeout::decode_vec(&any_msg.value)
                    .map_err(RouterError::MalformedMessageBytes)?;
                Ok(MsgEnvelope::Packet(PacketMsg::Timeout(domain_msg)))
            }
            timeout_on_close::TYPE_URL => {
                let domain_msg = timeout_on_close::MsgTimeoutOnClose::decode_vec(&any_msg.value)
                    .map_err(RouterError::MalformedMessageBytes)?;
                Ok(MsgEnvelope::Packet(PacketMsg::TimeoutOnClose(domain_msg)))
            }
            _ => Err(RouterError::UnknownMessageTypeUrl {
                url: any_msg.type_url,
            }),
        }
    }
}

impl<Ctx> ValidationHandler<MsgEnvelope> for Ctx
where
    Ctx: ReaderContext,
{
    fn validate(&self, msg: &MsgEnvelope) -> Result<(), ContextError> {
        match msg {
            MsgEnvelope::Client(msg) => match msg {
                ClientMsg::CreateClient(msg) => self.validate(msg),
                ClientMsg::UpdateClient(msg) => self.validate(msg),
                ClientMsg::Misbehaviour(_msg) => todo!(),
                ClientMsg::UpgradeClient(_msg) => todo!(),
            },
            MsgEnvelope::Connection(msg) => match msg {
                ConnectionMsg::OpenInit(_msg) => todo!(),
                ConnectionMsg::OpenTry(_msg) => todo!(),
                ConnectionMsg::OpenAck(_msg) => todo!(),
                ConnectionMsg::OpenConfirm(ref _msg) => todo!(),
            },
            MsgEnvelope::Channel(msg) => match msg {
                ChannelMsg::OpenInit(msg) => self.validate(msg),
                ChannelMsg::OpenTry(_msg) => todo!(),
                ChannelMsg::OpenAck(_msg) => todo!(),
                ChannelMsg::OpenConfirm(_msg) => todo!(),
                ChannelMsg::CloseInit(_msg) => todo!(),
                ChannelMsg::CloseConfirm(_msg) => todo!(),
            },
            MsgEnvelope::Packet(msg) => match msg {
                PacketMsg::Recv(_msg) => todo!(),
                PacketMsg::Ack(_msg) => todo!(),
                PacketMsg::Timeout(_msg) => {
                    todo!()
                }
                PacketMsg::TimeoutOnClose(_msg) => todo!(),
            },
        }
    }
}

impl<Ctx> ExecutionHandler<MsgEnvelope> for Ctx
where
    Ctx: KeeperContext,
{
    fn execute(&mut self, msg: &MsgEnvelope) -> Result<(), ContextError> {
        match msg {
            MsgEnvelope::Client(msg) => match msg {
                ClientMsg::CreateClient(msg) => self.execute(msg),
                ClientMsg::UpdateClient(msg) => self.execute(msg),
                ClientMsg::Misbehaviour(msg) => {
                    client_handler::misbehaviour::execute(self, msg.clone())
                }
                ClientMsg::UpgradeClient(msg) => {
                    client_handler::upgrade_client::execute(self, msg.clone())
                }
            },
            MsgEnvelope::Connection(msg) => match msg {
                ConnectionMsg::OpenInit(msg) => {
                    conn_handler::conn_open_init::execute(self, msg.clone())
                }
                ConnectionMsg::OpenTry(msg) => {
                    conn_handler::conn_open_try::execute(self, msg.clone())
                }
                ConnectionMsg::OpenAck(msg) => {
                    conn_handler::conn_open_ack::execute(self, msg.clone())
                }
                ConnectionMsg::OpenConfirm(ref msg) => {
                    conn_handler::conn_open_confirm::execute(self, msg)
                }
            },
            MsgEnvelope::Channel(msg) => match msg {
                ChannelMsg::OpenInit(msg) => self.execute(msg),
                ChannelMsg::OpenTry(_msg) => todo!(),
                ChannelMsg::OpenAck(_msg) => todo!(),
                ChannelMsg::OpenConfirm(_msg) => {
                    todo!()
                }
                ChannelMsg::CloseInit(_msg) => todo!(),
                ChannelMsg::CloseConfirm(_msg) => {
                    todo!()
                }
            },
            MsgEnvelope::Packet(msg) => match msg {
                PacketMsg::Recv(_msg) => todo!(),
                PacketMsg::Ack(_msg) => todo!(),
                PacketMsg::Timeout(_msg) => {
                    todo!()
                }
                PacketMsg::TimeoutOnClose(_msg) => todo!(),
            },
        }
    }
}
