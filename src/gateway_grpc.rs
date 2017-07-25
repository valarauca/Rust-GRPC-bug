// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use super::grpc;

// interface

pub trait Gateway {
    fn RemoteLogging(&self, o: ::grpc::GrpcRequestOptions, p: super::gateway::SetLogLevel) -> ::grpc::GrpcStreamingResponse<super::gateway::LogMessage>;
}

// client

pub struct GatewayClient {
    grpc_client: ::grpc::client::GrpcClient,
    method_RemoteLogging: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::gateway::SetLogLevel, super::gateway::LogMessage>>,
}

impl GatewayClient {
    pub fn with_client(grpc_client: ::grpc::client::GrpcClient) -> Self {
        GatewayClient {
            grpc_client: grpc_client,
            method_RemoteLogging: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/gateway.Gateway/RemoteLogging".to_string(),
                streaming: ::grpc::method::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new(host: &str, port: u16, tls: bool, conf: ::grpc::client::GrpcClientConf) -> ::grpc::result::GrpcResult<Self> {
        ::grpc::client::GrpcClient::new(host, port, tls, conf).map(|c| {
            GatewayClient::with_client(c)
        })
    }
}

impl Gateway for GatewayClient {
    fn RemoteLogging(&self, o: ::grpc::GrpcRequestOptions, p: super::gateway::SetLogLevel) -> ::grpc::GrpcStreamingResponse<super::gateway::LogMessage> {
        self.grpc_client.call_server_streaming(o, p, self.method_RemoteLogging.clone())
    }
}

// server

pub struct GatewayServer {
    pub grpc_server: ::grpc::server::GrpcServer,
}

impl ::std::ops::Deref for GatewayServer {
    type Target = ::grpc::server::GrpcServer;

    fn deref(&self) -> &Self::Target {
        &self.grpc_server
    }
}

impl GatewayServer {
    pub fn new<A : ::std::net::ToSocketAddrs, H : Gateway + 'static + Sync + Send + 'static>(addr: A, conf: ::grpc::server::GrpcServerConf, h: H) -> Self {
        let service_definition = GatewayServer::new_service_def(h);
        GatewayServer {
            grpc_server: ::grpc::server::GrpcServer::new_plain(addr, conf, service_definition),
        }
    }

    pub fn new_pool<A : ::std::net::ToSocketAddrs, H : Gateway + 'static + Sync + Send + 'static>(addr: A, conf: ::grpc::server::GrpcServerConf, h: H, cpu_pool: ::futures_cpupool::CpuPool) -> Self {
        let service_definition = GatewayServer::new_service_def(h);
        GatewayServer {
            grpc_server: ::grpc::server::GrpcServer::new_plain_pool(addr, conf, service_definition, cpu_pool),
        }
    }

    pub fn new_service_def<H : Gateway + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::server::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::server::ServerServiceDefinition::new(
            vec![
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/gateway.Gateway/RemoteLogging".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerServerStreaming::new(move |o, p| handler_copy.RemoteLogging(o, p))
                    },
                ),
            ],
        )
    }
}
