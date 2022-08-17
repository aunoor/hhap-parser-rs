#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum State {
    Dead,

    StartReqOrRes,
    ResOrRespH,
    StartRes,
    ResH,
    ResHT,
    ResHTT,
    ResHTTP,
    ResE,
    ResEV,
    ResEVE,
    ResEVEN,
    ResEVENT,
    ResFirstHttpMajor,
    ResHttpMajor,
    ResFirstHttpMinor,
    ResHttpMinor,
    ResFirstStatusCode,
    ResStatusCode,
    ResStatusStart,
    ResStatus,
    ResLineAlmostDone,

    StartReq,

    ReqMethod,
    ReqSpacesBeforeUrl,
    ReqSchema,
    ReqSchemaSlash,
    ReqSchemaSlashSlash,
    ReqServerStart,
    ReqServer,
    ReqServerWithAt,
    ReqPath,
    ReqQueryStringStart,
    ReqQueryString,
    ReqFragmentStart,
    ReqFragment,
    ReqHttpStart,
    ReqHttpH,
    ReqHttpHT,
    ReqHttpHTT,
    ReqHttpHTTP,
    ReqFirstHttpMajor,
    ReqHttpMajor,
    ReqFirstHttpMinor,
    ReqHttpMinor,
    ReqLineAlmostDone,

    HeaderFieldStart,
    HeaderField,
    HeaderValueDiscardWs,
    HeaderValueDiscardWsAlmostDone,
    HeaderValueDiscardLws,
    HeaderValueStart,
    HeaderValue,
    HeaderValueLws,

    HeaderAlmostDone,

    ChunkSizeStart,
    ChunkSize,
    ChunkParameters,
    ChunkSizeAlmostDone,

    HeadersAlmostDone,
    HeadersDone,

    ChunkData,
    ChunkDataAlmostDone,
    ChunkDataDone,

    BodyIdentity,
    BodyIdentityEof,

    MessageDone
}

impl State {
    pub fn is_header_state(self) -> bool {
        self <= State::HeadersDone
    }
}

pub enum HeaderState {
    General,
    C,
    CO,
    CON,

    MatchingConnection,
    MatchingProxyConnection,
    MatchingContentLength,
    MatchingTransferEncoding,
    MatchingUpgrade,

    Connection,
    ContentLength,
    TransferEncoding,
    Upgrade,

    MatchingTransferEncodingChunked,
    MatchingConnectionKeepAlive,
    MatchingConnectionClose,

    TransferEncodingChunked,
    ConnectionKeepAlive,
    ConnectionClose,
}
