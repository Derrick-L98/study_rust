#![allow(dead_code)]
use std::io;

use actix::prelude::*;
use actix_codec::{Decoder, Encoder};
use actix_web::web::{BufMut, BytesMut};
use byteorder::{BigEndian, ByteOrder};
use serde::{Deserialize, Serialize};
use serde_json as json;

/// Client request
#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "()")]
#[serde(tag = "cmd", content = "data")]
pub enum ChatRequest {
    /// List rooms
    List,
    /// Join rooms
    Join(String),
    /// Send message
    Message(String),
    /// Ping
    Ping,
}

/// Server response
#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "()")]
#[serde(tag = "cmd", content = "data")]
pub enum ChatResponse {
    Ping,

    /// List of rooms
    /// 房间列表
    Rooms(Vec<String>),

    /// Joined
    /// 加入
    Joined(String),

    /// Message
    /// 消息
    Message(String),
}

/// Codec for Client -> Server transport
pub struct ChatCodec;
//解码
impl Decoder for ChatCodec {
    type Item = ChatRequest;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let size = {//获取消息长度
            if src.len() < 2 {
                return Ok(None);
            }
            BigEndian::read_u16(src.as_ref()) as usize//两个字节
        };

        if src.len() >= size + 2 {
            let _ = src.split_to(2);//在给定索引处将缓冲区一分为二。然后self包含元素[at，len），返回的BytesMut包含元素[0，at）。
            let buf = src.split_to(size);//截取消息
            Ok(Some(json::from_slice::<ChatRequest>(&buf)?))//解析
        } else {
            Ok(None)
        }
    }
}

//编码
impl Encoder<ChatResponse> for ChatCodec {
    type Error = io::Error;
    //包头
    fn encode(&mut self, msg: ChatResponse, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let msg: String = json::to_string(&msg).unwrap();
        let msg_ref: &[u8] = msg.as_ref();

        dst.reserve(msg_ref.len() + 2);//重置len
        dst.put_u16(msg_ref.len() as u16);//按大端字节顺序将一个无符号的16位整数写入self。
        dst.put(msg_ref);//追加消息

        Ok(())
    }
}

/// Codec for Server -> Client transport
pub struct ClientChatCodec;

impl Decoder for ClientChatCodec {
    type Item = ChatResponse;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let size = {
            if src.len() < 2 {
                return Ok(None);
            }
            BigEndian::read_u16(src.as_ref()) as usize
        };

        if src.len() >= size + 2 {
            let _ = src.split_to(2);
            let buf = src.split_to(size);
            Ok(Some(json::from_slice::<ChatResponse>(&buf)?))
        } else {
            Ok(None)
        }
    }
}

impl Encoder<ChatRequest> for ClientChatCodec {
    type Error = io::Error;

    fn encode(&mut self, msg: ChatRequest, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let msg = json::to_string(&msg).unwrap();
        let msg_ref: &[u8] = msg.as_ref();

        dst.reserve(msg_ref.len() + 2);
        dst.put_u16(msg_ref.len() as u16);
        dst.put(msg_ref);

        Ok(())
    }
}
