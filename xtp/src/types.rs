pub enum XTPLogLevel {
    Fatal = 0,
    Error = 1,
    Warning = 2,
    Info = 3,
    Debug = 4,
    Trace = 5,
}

pub enum XTPProtocolType {
    /// Use TCP transmission
    TCP = 1,
    /// Use UDP transmission (only support market data)
    UDP = 2,
}

pub enum XTPExchangeType {
    /// Shanghai Exchange
    SH = 1,
    /// Shenzhen Exchange
    SZ = 2,
    /// Unknown
    Unknown = 3,
}

pub enum XTPMarketType {
    /// Initializing or unknown
    MarketInit = 0,
    /// Shenzhen A
    SZA = 1,
    /// Shanghai A
    SHA = 2,
    /// Unknown market type
    UNKNOWN = 3,
}

pub enum XTPPriceType {
    /// 限价单-沪 / 深 / 沪期权（除普通股票业务外，其余业务均使用此种类型）
    Limit = 1,
    /// 即时成交剩余转撤销，市价单-深 / 沪期权
    BestOrCancel = 2,
    /// 最优五档即时成交剩余转限价，市价单-沪
    BestsOrLimit = 3,
    /// U最优5档即时成交剩余转撤销，市价单-沪深
    BestsOrCancel = 4,
    /// 全部成交或撤销,市价单-深 / 沪期权
    AllOrCancel = 5,
    /// 本方最优，市价单-深
    ForwardBest = 6,
    /// 对方最优剩余转限价，市价单-深 / 沪期权
    ReverseBestLimit = 7,
    /// 期权限价申报FOK
    LimitOrCancel = 8,
    /// 未知或者无效价格类型
    TypeUnknown = 9,
}

pub enum XTPSideType {
    Buy = 1,
    Sell = 2,
    Purchase = 7,
    Redemption = 8,
    Split = 9,
    Merge = 10,
    Cover = 11,
    Freeze = 12,
    MarginTrade = 21,
    ShortSell = 22,
    RepayMargin = 23,
    RepayStock = 24,
    StockRepayStock = 26,
    SurstkTrans = 27,
    GrtstkTransin = 28,
    GrtstkTransout = 29,
    Unknown = 30,
}

pub enum XTPPositionEffectType {
    Init = 0,
    Open = 1,
    Close = 2,
    ForceClose = 3,
    CloseToday = 4,
    CloseYesterday = 5,
    ForceOff = 6,
    LocalForceClose = 7,
    CreditForceCover = 8,
    CreditForceClear = 9,
    CreditForceDebt = 10,
    CreditForceUncond = 11,
    Unknown = 12,
}

pub enum XtpOrderActionStatusType {
    Submitted = 1,
    Accepted = 2,
    Rejected = 3,
}

pub enum XTPOrderStatusType {
    Init = 0,
    AllTraded = 1,
    PartTradedQueueing = 2,
    PartTradedNotQueueing = 3,
    NoTradeQueueing = 4,
    Canceled = 5,
    Rejected = 6,
    Unknown = 7,
}

pub enum XTPOrderSubmitStatusType {
    InsertSubmitted = 1,
    InsertAccepted = 2,
    InsertRejected = 3,
    CancelSubmitted = 4,
    CancelRejected = 5,
    CancelAccepted = 6,
}
