use krpc;
use rpcfailure::RPCFailure;
use protobuf;
use protobuf::Message;

use std::io::Read;
use std::hash::Hash;
use std::collections::HashSet;
use std::collections::HashMap;


pub trait RPCExtractable: Sized {
    fn extract_value(client: &super::RPCClient, input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError>;
}

impl RPCExtractable for bool {
    fn extract_value(_client: &super::RPCClient, input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        input.read_bool()
    }
}

impl RPCExtractable for () {
    fn extract_value(_client: &super::RPCClient, _input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        Ok(())
    }
}

impl RPCExtractable for f64 {
    fn extract_value(_client: &super::RPCClient, input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        input.read_double()
    }
}

impl RPCExtractable for f32 {
    fn extract_value(_client: &super::RPCClient, input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        input.read_float()
    }
}

impl RPCExtractable for u64 {
    fn extract_value(_client: &super::RPCClient, input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        input.read_uint64()
    }
}

impl RPCExtractable for u32 {
    fn extract_value(_client: &super::RPCClient, input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        input.read_uint32()
    }
}

impl RPCExtractable for i64 {
    fn extract_value(_client: &super::RPCClient, input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        input.read_sint64()
    }
}

impl RPCExtractable for i32 {
    fn extract_value(_client: &super::RPCClient, input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        input.read_sint32()
    }
}

impl RPCExtractable for String {
    fn extract_value(_client: &super::RPCClient, input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        input.read_string()
    }
}

impl<T> RPCExtractable for Vec<T>
    where T: RPCExtractable
{
    fn extract_value(client: &super::RPCClient, input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let mut m = krpc::List::new();
        m.merge_from(input)?;

        let mut v = Vec::with_capacity(m.items.len());
        for item in &m.items {
            let mut i = protobuf::CodedInputStream::from_bytes(&item);
            v.push(RPCExtractable::extract_value(client, &mut i)?);
        }

        Ok(v)
    }
}

impl<T> RPCExtractable for HashSet<T>
    where T: RPCExtractable + Hash + Eq,
{
    fn extract_value(client: &super::RPCClient, input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let mut m = krpc::Set::new();
        m.merge_from(input)?;

        let mut s = HashSet::with_capacity(m.items.len());
        for item in &m.items {
            let mut i = protobuf::CodedInputStream::from_bytes(&item);
            s.insert(RPCExtractable::extract_value(client, &mut i)?);
        }

        Ok(s)
    }
}

impl<T, U> RPCExtractable for HashMap<T, U>
    where T: RPCExtractable + Hash + Eq,
          U: RPCExtractable
{
    fn extract_value(client: &super::RPCClient, input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let mut m = krpc::Dictionary::new();
        m.merge_from(input)?;

        let mut h = HashMap::with_capacity(m.entries.len());
        for entry in &m.entries {
            let mut i_k = protobuf::CodedInputStream::from_bytes(&entry.key);
            let mut i_v = protobuf::CodedInputStream::from_bytes(&entry.value);
            let key = RPCExtractable::extract_value(client, &mut i_k)?;
            let val = RPCExtractable::extract_value(client, &mut i_v)?;
            h.insert(key, val);
        }

        Ok(h)
    }
}

impl<T, U> RPCExtractable for (T, U)
    where T: RPCExtractable,
          U: RPCExtractable
{
    fn extract_value(client: &super::RPCClient, input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let t = RPCExtractable::extract_value(client, input)?;
        let u = RPCExtractable::extract_value(client, input)?;
        Ok((t, u))
    }
}

impl<T, U, V> RPCExtractable for (T, U, V)
    where T: RPCExtractable,
          U: RPCExtractable,
          V: RPCExtractable
{
    fn extract_value(client: &super::RPCClient, input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let t = RPCExtractable::extract_value(client, input)?;
        let u = RPCExtractable::extract_value(client, input)?;
        let v = RPCExtractable::extract_value(client, input)?;
        Ok((t, u, v))
    }
}

impl<T, U, V, W> RPCExtractable for (T, U, V, W)
    where T: RPCExtractable,
          U: RPCExtractable,
          V: RPCExtractable,
          W: RPCExtractable
{
    fn extract_value(client: &super::RPCClient, input: &mut protobuf::CodedInputStream) -> Result<Self, protobuf::ProtobufError> {
        let t = RPCExtractable::extract_value(client, input)?;
        let u = RPCExtractable::extract_value(client, input)?;
        let v = RPCExtractable::extract_value(client, input)?;
        let w = RPCExtractable::extract_value(client, input)?;
        Ok((t, u, v, w))
    }
}


pub trait RPCEncodable {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError>;
    fn encode_to_bytes(&self) -> Result<Vec<u8>, protobuf::ProtobufError> {
        let mut bytes = Vec::new();
        {
            let mut output = protobuf::CodedOutputStream::new(&mut bytes);
            self.encode(&mut output)?;
            output.flush()?;
        }
        Ok(bytes)
    }
}


impl RPCEncodable for bool {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        output.write_bool_no_tag(*self)
    }
}

impl RPCEncodable for f64 {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        output.write_double_no_tag(*self)
    }
}

impl RPCEncodable for f32 {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        output.write_float_no_tag(*self)
    }
}

impl RPCEncodable for u32 {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        output.write_uint32_no_tag(*self)
    }
}

impl RPCEncodable for i32 {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        output.write_sint32_no_tag(*self)
    }
}

impl RPCEncodable for u64 {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        output.write_uint64_no_tag(*self)
    }
}

impl RPCEncodable for i64 {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        output.write_sint64_no_tag(*self)
    }
}

impl RPCEncodable for String {
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        output.write_string_no_tag(self)
    }
}

impl<T> RPCEncodable for Vec<T>
    where T: RPCEncodable
{
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        let mut v = protobuf::RepeatedField::<Vec<u8>>::new();
        for e in self {
            v.push(e.encode_to_bytes()?);
        }

        let mut l = krpc::List::new();
        l.set_items(v);

        l.write_to(output)?;
        output.flush()?;

        Ok(())
    }
}

impl<T, U> RPCEncodable for (T, U)
    where T: RPCEncodable,
          U: RPCEncodable
{
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        let &(ref t, ref u) = self;
        t.encode(output)?;
        u.encode(output)?;
        output.flush()?;
        Ok(())
    }
}

impl<T, U, V> RPCEncodable for (T, U, V)
    where T: RPCEncodable,
          U: RPCEncodable,
          V: RPCEncodable
{
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        let &(ref t, ref u, ref v) = self;
        t.encode(output)?;
        u.encode(output)?;
        v.encode(output)?;
        output.flush()?;
        Ok(())
    }
}

impl<T, U, V, W> RPCEncodable for (T, U, V, W)
    where T: RPCEncodable,
          U: RPCEncodable,
          V: RPCEncodable,
          W: RPCEncodable
{
    fn encode(&self, output: &mut protobuf::CodedOutputStream) -> Result<(), protobuf::ProtobufError> {
        let &(ref t, ref u, ref v, ref w) = self;
        t.encode(output)?;
        u.encode(output)?;
        v.encode(output)?;
        w.encode(output)?;
        output.flush()?;
        Ok(())
    }
}

pub fn read_message<M>(sock: &mut Read) -> Result<M, protobuf::ProtobufError>
    where M: protobuf::Message + protobuf::MessageStatic
{
    let mut input_stream = protobuf::CodedInputStream::new(sock);
    protobuf::parse_length_delimited_from::<M>(&mut input_stream)
}


pub fn extract_single_result<T>(client: &super::RPCClient, response: &krpc::Response) -> Result<T, RPCFailure>
    where T: RPCExtractable
{
    if response.has_error() {
        Err(RPCFailure::ProcFailure(response.get_error().clone()))
    }
    else {
        extract_result(client, &response.results[0])
    }
}

pub fn extract_result<T>(client: &super::RPCClient, proc_result: &krpc::ProcedureResult) -> Result<T, RPCFailure>
    where T: RPCExtractable
{
    if proc_result.has_error() {
        Err(RPCFailure::ProcFailure(proc_result.get_error().clone()))
    }
    else {
        let mut input = protobuf::CodedInputStream::from_bytes(proc_result.get_value());
        RPCExtractable::extract_value(client, &mut input).map_err(RPCFailure::ProtobufFailure)
    }
}
