pub enum HttpResponseStatusCode {
    Informational(Informational),
    Successful(Successful),
    Redirection(Redirection),
    ClientError(ClientError),
    ServerError(ServerError),
}

pub enum Informational {
    Continue = 100,
    SwitchingProtocols = 101,
    WebDavProcessing = 102,
    EarlyHints = 103,
}

pub enum Successful {
    Ok = 200,
    Created = 201,
    Accepted = 202,
    NonAuthorativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,
    WebDavMultiStatus = 207,
    WebDavAlreadyReported = 208,
    DeltaImUsed = 226,
}

pub enum Redirection {
    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    DeprecatedUseProxy = 305,
    ReservedUnused306 = 306,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,
}

pub enum ClientError {
    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthenticationRequired = 407,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    PayloadTooLarge = 413,
    UriTooLong = 414,
    UnsupportedMediaType = 415,
    RangeNotSatisfiable = 416,
    ExpectationFailed = 417,
    ImATeapot = 418,
    MisdirectedRequest = 421,
    WebDavUnprocessableContent = 422,
    WebDavLocked = 423,
    WebDavFailedDependency = 424,
    TooEarly = 425,
    UpgradeRequired = 426,
    PreconditionRequired = 428,
    TooManyRequests = 429,
    RequestHeaderFieldsTooLarge = 431,
    UnavailableForLegalReasons = 451,
}

pub enum ServerError {
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HttpVersionNotSupported = 505,
    VariantAlsoNegotiates = 506,
    WebDavInsufficientStorage = 507,
    WebDavLoopDetected = 508,
    NotExtended = 510,
    NetworkAuthenticationRequired = 511,
}
