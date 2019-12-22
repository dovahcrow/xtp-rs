#![allow(unused_comparisons)]
use crate::sys::{
    self, XTPOrderInsertInfo__bindgen_ty_1, __BindgenUnionField, XTPMD, XTPOB, XTPQSI, XTPRI,
    XTPST, XTPTBT, XTPTPI, XTP_BUSINESS_TYPE, XTP_EXCHANGE_TYPE, XTP_FUND_TRANSFER_TYPE,
    XTP_LOG_LEVEL, XTP_MARKETDATA_TYPE, XTP_MARKET_TYPE, XTP_ORDER_ACTION_STATUS_TYPE,
    XTP_ORDER_STATUS_TYPE, XTP_ORDER_SUBMIT_STATUS_TYPE, XTP_POSITION_EFFECT_TYPE, XTP_PRICE_TYPE,
    XTP_PROTOCOL_TYPE, XTP_SIDE_TYPE, XTP_TBT_TYPE, XTP_TE_RESUME_TYPE, XTP_TICKER_TYPE,
};
use std::ffi::CStr;
use std::mem::transmute;

pub trait FromRaw<T> {
    unsafe fn from_raw(raw: T) -> Self;
}

macro_rules! impl_ffi_convert {
    ($rtype:ty, $ctype: ty, $lb: expr, $ub: expr) => {
        impl FromRaw<$ctype> for $rtype {
            unsafe fn from_raw(from: $ctype) -> Self {
                assert!($lb <= from as u32 && from as u32 <= $ub);
                transmute::<_, $rtype>(from)
            }
        }

        impl From<$rtype> for $ctype {
            fn from(r: $rtype) -> Self {
                unsafe { transmute::<_, $ctype>(r) }
            }
        }
    };
    ($rtype:ty, $ctype: ty, $ub: expr) => {
        impl_ffi_convert!($rtype, $ctype, 0, $ub);
    };
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum XTPLogLevel {
    Fatal = XTP_LOG_LEVEL::XTP_LOG_LEVEL_FATAL as u32,
    Error = XTP_LOG_LEVEL::XTP_LOG_LEVEL_ERROR as u32,
    Warning = XTP_LOG_LEVEL::XTP_LOG_LEVEL_WARNING as u32,
    Info = XTP_LOG_LEVEL::XTP_LOG_LEVEL_INFO as u32,
    Debug = XTP_LOG_LEVEL::XTP_LOG_LEVEL_DEBUG as u32,
    Trace = XTP_LOG_LEVEL::XTP_LOG_LEVEL_TRACE as u32,
}

impl_ffi_convert!(XTPLogLevel, XTP_LOG_LEVEL, 5);

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum XTPProtocolType {
    /// Use TCP transmission
    TCP = XTP_PROTOCOL_TYPE::XTP_PROTOCOL_TCP as u32,
    /// Use UDP transmission (only support market data)
    UDP = XTP_PROTOCOL_TYPE::XTP_PROTOCOL_UDP as u32,
}

impl_ffi_convert!(XTPProtocolType, XTP_PROTOCOL_TYPE, 1, 2);

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum XTPExchangeType {
    /// Shanghai Exchange
    SH = XTP_EXCHANGE_TYPE::XTP_EXCHANGE_SH as u32,
    /// Shenzhen Exchange
    SZ = XTP_EXCHANGE_TYPE::XTP_EXCHANGE_SZ as u32,
    /// Unknown
    Unknown = XTP_EXCHANGE_TYPE::XTP_EXCHANGE_UNKNOWN as u32,
}

impl_ffi_convert!(XTPExchangeType, XTP_EXCHANGE_TYPE, 1, 3);

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum XTPMarketType {
    /// Initializing or unknown
    MarketInit = XTP_MARKET_TYPE::XTP_MKT_INIT as u32,
    /// Shenzhen A
    SZA = XTP_MARKET_TYPE::XTP_MKT_SZ_A as u32,
    /// Shanghai A
    SHA = XTP_MARKET_TYPE::XTP_MKT_SH_A as u32,
    /// Unknown market type
    UNKNOWN = XTP_MARKET_TYPE::XTP_MKT_UNKNOWN as u32,
}
impl_ffi_convert!(XTPMarketType, XTP_MARKET_TYPE, 0, 3);

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum XTPMarketdataType {
    Actual = XTP_MARKETDATA_TYPE::XTP_MARKETDATA_ACTUAL as u32,
    Option = XTP_MARKETDATA_TYPE::XTP_MARKETDATA_OPTION as u32,
}
impl_ffi_convert!(XTPMarketdataType, XTP_MARKETDATA_TYPE, 0, 1);

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum XTPPriceType {
    /// 限价单-沪 / 深 / 沪期权（除普通股票业务外，其余业务均使用此种类型）
    Limit = XTP_PRICE_TYPE::XTP_PRICE_LIMIT as u32,
    /// 即时成交剩余转撤销，市价单-深 / 沪期权
    BestOrCancel = XTP_PRICE_TYPE::XTP_PRICE_BEST_OR_CANCEL as u32,
    /// 最优五档即时成交剩余转限价，市价单-沪
    BestsOrLimit = XTP_PRICE_TYPE::XTP_PRICE_BEST5_OR_LIMIT as u32,
    /// U最优5档即时成交剩余转撤销，市价单-沪深
    BestsOrCancel = XTP_PRICE_TYPE::XTP_PRICE_BEST5_OR_CANCEL as u32,
    /// 全部成交或撤销,市价单-深 / 沪期权
    AllOrCancel = XTP_PRICE_TYPE::XTP_PRICE_ALL_OR_CANCEL as u32,
    /// 本方最优，市价单-深
    ForwardBest = XTP_PRICE_TYPE::XTP_PRICE_FORWARD_BEST as u32,
    /// 对方最优剩余转限价，市价单-深 / 沪期权
    ReverseBestLimit = XTP_PRICE_TYPE::XTP_PRICE_REVERSE_BEST_LIMIT as u32,
    /// 期权限价申报FOK
    LimitOrCancel = XTP_PRICE_TYPE::XTP_PRICE_LIMIT_OR_CANCEL as u32,
    /// 未知或者无效价格类型
    TypeUnknown = XTP_PRICE_TYPE::XTP_PRICE_TYPE_UNKNOWN as u32,
}
impl_ffi_convert!(XTPPriceType, XTP_PRICE_TYPE, 1, 9);

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum XTPSideType {
    Buy = XTP_SIDE_TYPE::XTP_SIDE_BUY as u8,
    Sell = XTP_SIDE_TYPE::XTP_SIDE_SELL as u8,
    Purchase = XTP_SIDE_TYPE::XTP_SIDE_PURCHASE as u8,
    Redemption = XTP_SIDE_TYPE::XTP_SIDE_REDEMPTION as u8,
    Split = XTP_SIDE_TYPE::XTP_SIDE_SPLIT as u8,
    Merge = XTP_SIDE_TYPE::XTP_SIDE_MERGE as u8,
    Cover = XTP_SIDE_TYPE::XTP_SIDE_COVER as u8,
    Freeze = XTP_SIDE_TYPE::XTP_SIDE_FREEZE as u8,
    MarginTrade = XTP_SIDE_TYPE::XTP_SIDE_MARGIN_TRADE as u8,
    ShortSell = XTP_SIDE_TYPE::XTP_SIDE_SHORT_SELL as u8,
    RepayMargin = XTP_SIDE_TYPE::XTP_SIDE_REPAY_MARGIN as u8,
    RepayStock = XTP_SIDE_TYPE::XTP_SIDE_REPAY_STOCK as u8,
    StockRepayStock = XTP_SIDE_TYPE::XTP_SIDE_STOCK_REPAY_STOCK as u8,
    SurstkTrans = XTP_SIDE_TYPE::XTP_SIDE_SURSTK_TRANS as u8,
    GrtstkTransin = XTP_SIDE_TYPE::XTP_SIDE_GRTSTK_TRANSIN as u8,
    GrtstkTransout = XTP_SIDE_TYPE::XTP_SIDE_GRTSTK_TRANSOUT as u8,
    Unknown = XTP_SIDE_TYPE::XTP_SIDE_UNKNOWN as u8,
}

impl_ffi_convert!(XTPSideType, XTP_SIDE_TYPE, 1, 30);

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum XTPPositionEffectType {
    Init = XTP_POSITION_EFFECT_TYPE::XTP_POSITION_EFFECT_INIT as u8,
    Open = XTP_POSITION_EFFECT_TYPE::XTP_POSITION_EFFECT_OPEN as u8,
    Close = XTP_POSITION_EFFECT_TYPE::XTP_POSITION_EFFECT_CLOSE as u8,
    ForceClose = XTP_POSITION_EFFECT_TYPE::XTP_POSITION_EFFECT_FORCECLOSE as u8,
    CloseToday = XTP_POSITION_EFFECT_TYPE::XTP_POSITION_EFFECT_CLOSETODAY as u8,
    CloseYesterday = XTP_POSITION_EFFECT_TYPE::XTP_POSITION_EFFECT_CLOSEYESTERDAY as u8,
    ForceOff = XTP_POSITION_EFFECT_TYPE::XTP_POSITION_EFFECT_FORCEOFF as u8,
    LocalForceClose = XTP_POSITION_EFFECT_TYPE::XTP_POSITION_EFFECT_LOCALFORCECLOSE as u8,
    CreditForceCover = XTP_POSITION_EFFECT_TYPE::XTP_POSITION_EFFECT_CREDIT_FORCE_COVER as u8,
    CreditForceClear = XTP_POSITION_EFFECT_TYPE::XTP_POSITION_EFFECT_CREDIT_FORCE_CLEAR as u8,
    CreditForceDebt = XTP_POSITION_EFFECT_TYPE::XTP_POSITION_EFFECT_CREDIT_FORCE_DEBT as u8,
    CreditForceUncond = XTP_POSITION_EFFECT_TYPE::XTP_POSITION_EFFECT_CREDIT_FORCE_UNCOND as u8,
    Unknown = XTP_POSITION_EFFECT_TYPE::XTP_POSITION_EFFECT_UNKNOWN as u8,
}
impl_ffi_convert!(XTPPositionEffectType, XTP_POSITION_EFFECT_TYPE, 1, 12);

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum XTPOrderActionStatusType {
    Submitted = XTP_ORDER_ACTION_STATUS_TYPE::XTP_ORDER_ACTION_STATUS_SUBMITTED as u32,
    Accepted = XTP_ORDER_ACTION_STATUS_TYPE::XTP_ORDER_ACTION_STATUS_ACCEPTED as u32,
    Rejected = XTP_ORDER_ACTION_STATUS_TYPE::XTP_ORDER_ACTION_STATUS_REJECTED as u32,
}
impl_ffi_convert!(XTPOrderActionStatusType, XTP_ORDER_ACTION_STATUS_TYPE, 1, 3);

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum XTPOrderStatusType {
    Init = XTP_ORDER_STATUS_TYPE::XTP_ORDER_STATUS_INIT as u32,
    AllTraded = XTP_ORDER_STATUS_TYPE::XTP_ORDER_STATUS_ALLTRADED as u32,
    PartTradedQueueing = XTP_ORDER_STATUS_TYPE::XTP_ORDER_STATUS_PARTTRADEDQUEUEING as u32,
    PartTradedNotQueueing = XTP_ORDER_STATUS_TYPE::XTP_ORDER_STATUS_PARTTRADEDNOTQUEUEING as u32,
    NoTradeQueueing = XTP_ORDER_STATUS_TYPE::XTP_ORDER_STATUS_NOTRADEQUEUEING as u32,
    Canceled = XTP_ORDER_STATUS_TYPE::XTP_ORDER_STATUS_CANCELED as u32,
    Rejected = XTP_ORDER_STATUS_TYPE::XTP_ORDER_STATUS_REJECTED as u32,
    Unknown = XTP_ORDER_STATUS_TYPE::XTP_ORDER_STATUS_UNKNOWN as u32,
}
impl_ffi_convert!(XTPOrderStatusType, XTP_ORDER_STATUS_TYPE, 0, 7);

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum XTPOrderSubmitStatusType {
    InsertSubmitted = XTP_ORDER_SUBMIT_STATUS_TYPE::XTP_ORDER_SUBMIT_STATUS_INSERT_SUBMITTED as u32,
    InsertAccepted = XTP_ORDER_SUBMIT_STATUS_TYPE::XTP_ORDER_SUBMIT_STATUS_INSERT_ACCEPTED as u32,
    InsertRejected = XTP_ORDER_SUBMIT_STATUS_TYPE::XTP_ORDER_SUBMIT_STATUS_INSERT_REJECTED as u32,
    CancelSubmitted = XTP_ORDER_SUBMIT_STATUS_TYPE::XTP_ORDER_SUBMIT_STATUS_CANCEL_SUBMITTED as u32,
    CancelRejected = XTP_ORDER_SUBMIT_STATUS_TYPE::XTP_ORDER_SUBMIT_STATUS_CANCEL_REJECTED as u32,
    CancelAccepted = XTP_ORDER_SUBMIT_STATUS_TYPE::XTP_ORDER_SUBMIT_STATUS_CANCEL_ACCEPTED as u32,
}
impl_ffi_convert!(XTPOrderSubmitStatusType, XTP_ORDER_SUBMIT_STATUS_TYPE, 1, 6);

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum XTPTeResumeType {
    /// 从本交易日开始重传
    Restart = XTP_TE_RESUME_TYPE::XTP_TERT_RESTART as u32,
    /// 从从上次收到的续传（暂未支持）
    Resume = XTP_TE_RESUME_TYPE::XTP_TERT_RESUME as u32,
    /// 只传送登录后公有流（订单响应、成交回报）的内容"
    Quick = XTP_TE_RESUME_TYPE::XTP_TERT_QUICK as u32,
}
impl_ffi_convert!(XTPTeResumeType, XTP_TE_RESUME_TYPE, 0, 2);

// /// ETF_REPLACE_TYPE现金替代标识定义
// pub enum ETFReplaceType {
//     /// 禁止现金替代
//     Forbidden = 0,
//     /// 可以现金替代
//     Optional = 1,
//     /// 必须现金替代
//     Must = 2,
//     /// 深市退补现金替代
//     RecomputeInterSZ = 3,
//     /// 深市必须现金替代
//     MustInterSZ = 4,
//     /// 非沪深市场成分证券退补现金替代
//     RecomputeInterOther = 5,
//     /// 表示非沪深市场成份证券必须现金替代
//     MustInterOther = 6,
//     /// 无效值
//     Invalid = 7,
// }

/// 证券类型
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum XTPTickerType {
    /// 普通股票
    Stock = XTP_TICKER_TYPE::XTP_TICKER_TYPE_STOCK as u32,
    /// 指数
    Index = XTP_TICKER_TYPE::XTP_TICKER_TYPE_INDEX as u32,
    /// 基金
    Fund = XTP_TICKER_TYPE::XTP_TICKER_TYPE_FUND as u32,
    /// 债券
    Bond = XTP_TICKER_TYPE::XTP_TICKER_TYPE_BOND as u32,
    /// 期权
    Option = XTP_TICKER_TYPE::XTP_TICKER_TYPE_OPTION as u32,
    /// 科创板股票上海）
    TechStock = XTP_TICKER_TYPE::XTP_TICKER_TYPE_TECH_STOCK as u32,
    /// 未知类型
    Unknown = XTP_TICKER_TYPE::XTP_TICKER_TYPE_UNKNOWN as u32,
}
impl_ffi_convert!(XTPTickerType, XTP_TICKER_TYPE, 0, 6);

/// 证券业务类型
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum XTPBusinessType {
    /// 普通股票业务（股票买卖，ETF买卖等）
    Cash = XTP_BUSINESS_TYPE::XTP_BUSINESS_TYPE_CASH as u32,
    /// 新股申购业务（对应的price type需选择限价类型）
    Ipos = XTP_BUSINESS_TYPE::XTP_BUSINESS_TYPE_IPOS as u32,
    /// 回购业务 ( 对应的price type填为限价，side填为卖 )
    Repo = XTP_BUSINESS_TYPE::XTP_BUSINESS_TYPE_REPO as u32,
    /// ETF申赎业务
    ETF = XTP_BUSINESS_TYPE::XTP_BUSINESS_TYPE_ETF as u32,
    /// 融资融券业务（暂未支持）
    Margin = XTP_BUSINESS_TYPE::XTP_BUSINESS_TYPE_MARGIN as u32,
    /// 转托管（未支持）
    Designation = XTP_BUSINESS_TYPE::XTP_BUSINESS_TYPE_DESIGNATION as u32,
    /// 配股业务（对应的price type需选择限价类型,side填为买）
    Allotment = XTP_BUSINESS_TYPE::XTP_BUSINESS_TYPE_ALLOTMENT as u32,
    /// 分级基金申赎业务
    StructuredFundPurchaseRedemption =
        XTP_BUSINESS_TYPE::XTP_BUSINESS_TYPE_STRUCTURED_FUND_PURCHASE_REDEMPTION as u32,
    /// 分级基金拆分合并业务
    StructuredFundSplitMerge =
        XTP_BUSINESS_TYPE::XTP_BUSINESS_TYPE_STRUCTURED_FUND_SPLIT_MERGE as u32,
    /// 货币基金业务（暂未支持）
    MoneyFund = XTP_BUSINESS_TYPE::XTP_BUSINESS_TYPE_MONEY_FUND as u32,
    /// 期权业务
    Option = XTP_BUSINESS_TYPE::XTP_BUSINESS_TYPE_OPTION as u32,
    /// 行权
    Execute = XTP_BUSINESS_TYPE::XTP_BUSINESS_TYPE_EXECUTE as u32,
    /// 锁定解锁，暂不支持
    Freeze = XTP_BUSINESS_TYPE::XTP_BUSINESS_TYPE_FREEZE as u32,
    /// 未知类型
    Unknown = XTP_BUSINESS_TYPE::XTP_BUSINESS_TYPE_UNKNOWN as u32,
}
impl_ffi_convert!(XTPBusinessType, XTP_BUSINESS_TYPE, 0, 13);
// /// 账户类型
// pub enum XTPAccountType {
//     /// 普通账户
//     Normal = 0,
//     /// 信用账户
//     Credit = 1,
//     /// 衍生品账户
//     Derive = 2,
//     /// 未知账户类型
//     Unknown = 3,
// }

/// 资金流转方向类型
#[repr(u32)]
#[derive(Copy, Debug, Clone)]
pub enum XTPFundTransferType {
    /// 转出 从XTP转出到柜台
    TransferOut = XTP_FUND_TRANSFER_TYPE::XTP_FUND_TRANSFER_OUT as u32,
    /// 转入 从柜台转入XTP
    TransferIn = XTP_FUND_TRANSFER_TYPE::XTP_FUND_TRANSFER_IN as u32,
    /// 跨节点转出 从本XTP节点1，转出到对端XTP节点2，XTP服务器之间划拨，只能跨账户用户使用
    InterTransferOut = XTP_FUND_TRANSFER_TYPE::XTP_FUND_INTER_TRANSFER_OUT as u32,
    /// 跨节点转入 从对端XTP节点2，转入到本XTP节点1，XTP服务器之间划拨，只能跨账户用户使用
    InterTransferIn = XTP_FUND_TRANSFER_TYPE::XTP_FUND_INTER_TRANSFER_IN as u32,
    /// 未知类型
    TransferUnknown = XTP_FUND_TRANSFER_TYPE::XTP_FUND_TRANSFER_UNKNOWN as u32,
}
impl_ffi_convert!(XTPFundTransferType, XTP_FUND_TRANSFER_TYPE, 0, 4);

// /// XTP_FUND_OPER_STATUS柜台资金操作结果
// pub enum XTPFundOperStatus {
//     /// XTP已收到，正在处理中
//     PROCESSING = 0,
//     /// 成功
//     SUCCESS = 1,
//     /// 失败
//     FAILED = 2,
//     /// 已提交到集中柜台处理
//     SUBMITTED = 3,
//     /// 未知
//     UNKNOWN = 4,
// }

// /// XTP_SPLIT_MERGE_STATUS是一个基金当天拆分合并状态类型
// pub enum XTPSplitMergeStatus {
//     /// 允许拆分和合并
//     Allow = 0,
//     /// 只允许拆分，不允许合并
//     OnlySplit = 1,
//     /// 只允许合并，不允许拆分
//     OnlyMerge = 2,
//     /// 不允许拆分合并
//     Forbidden = 3,
// }

/// XTP_TBT_TYPE是一个逐笔回报类型
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum XTPTbtType {
    /// 逐笔委托
    ENTRUST = XTP_TBT_TYPE::XTP_TBT_ENTRUST as u32,
    /// 逐笔成交
    TRADE = XTP_TBT_TYPE::XTP_TBT_TRADE as u32,
}
impl_ffi_convert!(XTPTbtType, XTP_TBT_TYPE, 1, 2);

// /// XTP_OPT_CALL_OR_PUT_TYPE是一个认沽或认购类型
// pub enum XTPOptCallOrPutType {
//     /// 认购
//     CALL = 1,
//     /// 认沽
//     PUT = 2,
// }

// /// XTP_OPT_EXERCISE_TYPE_TYPE是一个行权方式类型

// pub enum XTPOptExerciseTypeType {
//     /// 欧式
//     EUR = 1,
//     /// 美式
//     AME = 2,
// }

// /// XTP_POSITION_DIRECTION_TYPE是一个持仓方向类型
// pub enum XTPPositionDirectionType {
//     /// 净
//     Net = 0,
//     /// 多（期权则为权利方）
//     Long = 1,
//     /// 空（期权则为义务方）
//     Short = 2,
//     /// 备兑（期权则为备兑义务方）
//     Covered = 3,
// }

// #[doc = "XTP_CRD_CASH_REPAY_STATUS是一个融资融券直接还款状态类型"]
// pub enum XTPCrdCrStatus {
//     ///  初始、未处理状态
//     INIT = 0,
//     ///  已成功处理状态
//     SUCCESS = 1,
//     ///  处理失败状态
//     FAILED = 2,
// }

// /// TXTPTradeTypeType是成交类型类型
// pub enum XTPTradeType {
//     Common = b'0' as isize,
//     Cash = b'1' as isize,
//     Primary = b'2' as isize,
//     CrossMktCash = b'3' as isize,
// }

// /// TXTPOrderTypeType是报单类型类型
// pub enum XTPOrderType {
//     Normal = '0' as isize,
//     DeriveFromQuote = '1' as isize,
//     DeriveFromCombination = '2' as isize,
//     Combination = '3' as isize,
//     ConditionalOrder = '4' as isize,
//     Swap = '5' as isize,
// }

// pub const XTP_ERR_MSG_LEN: u32 = 124;
// pub const XTP_ACCOUNT_PASSWORD_LEN: u32 = 64;

#[derive(Debug, Clone)]
pub struct XTPRspInfoStruct {
    pub error_id: i32,
    pub error_msg: String,
}

impl FromRaw<&XTPRI> for XTPRspInfoStruct {
    unsafe fn from_raw(
        XTPRI {
            error_id,
            error_msg,
        }: &XTPRI,
    ) -> Self {
        let error_msg = CStr::from_ptr(error_msg as *const [i8] as *const i8);
        XTPRspInfoStruct {
            error_id: *error_id,
            error_msg: error_msg.to_owned().to_string_lossy().to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct XTPSpecificTickerStruct {
    pub exchange_id: XTPExchangeType,
    pub ticker: String,
}

impl FromRaw<&XTPST> for XTPSpecificTickerStruct {
    unsafe fn from_raw(
        XTPST {
            exchange_id,
            ticker,
        }: &XTPST,
    ) -> Self {
        let exchange_id = XTPExchangeType::from_raw(*exchange_id);
        let ticker = CStr::from_ptr(ticker as *const [i8] as *const i8);
        let ticker = ticker.to_owned().to_string_lossy().to_string();
        XTPSpecificTickerStruct {
            exchange_id,
            ticker,
        }
    }
}

#[derive(Clone, Debug)]
pub struct OrderBookStruct {
    pub exchange_id: XTPExchangeType,
    pub ticker: String,
    pub last_price: f64,
    pub qty: i64,
    pub turnover: f64,
    pub trades_count: i64,
    pub bid: [f64; 10usize],
    pub ask: [f64; 10usize],
    pub bid_qty: [i64; 10usize],
    pub ask_qty: [i64; 10usize],
    pub data_time: i64,
}

impl FromRaw<&XTPOB> for OrderBookStruct {
    unsafe fn from_raw(ob: &XTPOB) -> Self {
        OrderBookStruct {
            exchange_id: XTPExchangeType::from_raw(ob.exchange_id),
            ticker: carray_to_string(&ob.ticker),
            last_price: ob.last_price,
            qty: ob.qty,
            turnover: ob.turnover,
            trades_count: ob.trades_count,
            bid: ob.bid,
            ask: ob.ask,
            bid_qty: ob.bid_qty,
            ask_qty: ob.ask_qty,
            data_time: ob.data_time,
        }
    }
}

#[derive(Clone, Debug)]
pub struct XTPTickByTickStruct {
    pub exchange_id: XTPExchangeType,
    pub ticker: String,
    pub seq: i64,
    pub data_time: i64,
    pub r#type: XTPTbtType,
}

impl FromRaw<&XTPTBT> for XTPTickByTickStruct {
    unsafe fn from_raw(tbt: &XTPTBT) -> Self {
        XTPTickByTickStruct {
            exchange_id: XTPExchangeType::from_raw(tbt.exchange_id),
            ticker: carray_to_string(&tbt.ticker),
            seq: tbt.seq,
            data_time: tbt.data_time,
            r#type: XTPTbtType::from_raw(tbt.type_),
        }
    }
}

#[derive(Clone, Debug)]
pub struct XTPQuoteStaticInfo {
    pub exchange_id: XTPExchangeType,
    pub ticker: String,
    pub ticker_name: String,
    pub ticker_type: XTPTickerType,
    pub pre_close_price: f64,
    pub upper_limit_price: f64,
    pub lower_limit_price: f64,
    pub price_tick: f64,
    pub buy_qty_unit: i32,
    pub sell_qty_unit: i32,
}

impl FromRaw<&XTPQSI> for XTPQuoteStaticInfo {
    unsafe fn from_raw(qsi: &XTPQSI) -> Self {
        XTPQuoteStaticInfo {
            exchange_id: XTPExchangeType::from_raw(qsi.exchange_id),
            ticker: carray_to_string(&qsi.ticker),
            ticker_name: carray_to_string(&qsi.ticker_name),
            ticker_type: XTPTickerType::from_raw(qsi.ticker_type),
            pre_close_price: qsi.pre_close_price,
            upper_limit_price: qsi.upper_limit_price,
            lower_limit_price: qsi.lower_limit_price,
            price_tick: qsi.price_tick,
            buy_qty_unit: qsi.buy_qty_unit,
            sell_qty_unit: qsi.sell_qty_unit,
        }
    }
}

#[derive(Debug, Clone)]
pub struct XTPTickerPriceInfo {
    pub exchange_id: XTPExchangeType,
    pub ticker: String,
    pub last_price: f64,
}

impl FromRaw<&XTPTPI> for XTPTickerPriceInfo {
    unsafe fn from_raw(tpi: &XTPTPI) -> Self {
        XTPTickerPriceInfo {
            exchange_id: XTPExchangeType::from_raw(tpi.exchange_id),
            ticker: carray_to_string(&tpi.ticker),
            last_price: tpi.last_price,
        }
    }
}

#[derive(Debug, Clone)]
pub struct XTPMarketDataStruct {
    pub exchange_id: XTPExchangeType,
    pub ticker: String,
    pub last_price: f64,
    pub pre_close_price: f64,
    pub open_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub close_price: f64,
    pub pre_total_long_positon: i64,
    pub total_long_positon: i64,
    pub pre_settl_price: f64,
    pub settl_price: f64,
    pub upper_limit_price: f64,
    pub lower_limit_price: f64,
    pub pre_delta: f64,
    pub curr_delta: f64,
    pub data_time: i64,
    pub qty: i64,
    pub turnover: f64,
    pub avg_price: f64,
    pub bid: [f64; 10usize],
    pub ask: [f64; 10usize],
    pub bid_qty: [i64; 10usize],
    pub ask_qty: [i64; 10usize],
    pub trades_count: i64,
    pub ticker_status: String,
    pub data_type: XTPMarketdataType,
}

impl FromRaw<&XTPMD> for XTPMarketDataStruct {
    unsafe fn from_raw(md: &XTPMD) -> Self {
        XTPMarketDataStruct {
            exchange_id: XTPExchangeType::from_raw(md.exchange_id),
            ticker: carray_to_string(&md.ticker),
            last_price: md.last_price,
            pre_close_price: md.pre_close_price,
            open_price: md.open_price,
            high_price: md.high_price,
            low_price: md.low_price,
            close_price: md.close_price,
            pre_total_long_positon: md.pre_total_long_positon,
            total_long_positon: md.total_long_positon,
            pre_settl_price: md.pre_close_price,
            settl_price: md.settl_price,
            upper_limit_price: md.upper_limit_price,
            lower_limit_price: md.low_price,
            pre_delta: md.pre_close_price,
            curr_delta: md.curr_delta,
            data_time: md.data_time,
            qty: md.qty,
            turnover: md.turnover,
            avg_price: md.avg_price,
            bid: md.bid,
            ask: md.ask,
            bid_qty: md.bid_qty,
            ask_qty: md.ask_qty,
            trades_count: md.trades_count,
            ticker_status: carray_to_string(&md.ticker_status),
            data_type: XTPMarketdataType::from_raw(md.data_type),
        }
    }
}

#[derive(Debug, Clone)]
pub struct XTPOrderInsertInfo {
    pub order_xtp_id: u64,
    pub order_client_id: u32,
    pub ticker: String,
    pub market: XTPMarketType,
    pub price: f64,
    pub stop_price: f64,
    pub quantity: i64,
    pub price_type: XTPPriceType,
    pub side: XTPSideType,
    pub position_effect: XTPPositionEffectType,
    pub business_type: XTPBusinessType,
}

impl From<&XTPOrderInsertInfo> for sys::XTPOrderInsertInfo {
    fn from(r: &XTPOrderInsertInfo) -> sys::XTPOrderInsertInfo {
        let union = unsafe {
            XTPOrderInsertInfo__bindgen_ty_1 {
                u32: __BindgenUnionField::new(),
                __bindgen_anon_1: __BindgenUnionField::new(),
                bindgen_union_field: transmute::<_, u32>((
                    XTP_SIDE_TYPE::from(r.side),
                    XTP_POSITION_EFFECT_TYPE::from(r.position_effect),
                    0u8,
                    0u8,
                )),
            }
        };

        sys::XTPOrderInsertInfo {
            order_xtp_id: r.order_xtp_id,
            order_client_id: r.order_client_id,
            ticker: string_to_carray16(&r.ticker),
            market: r.market.into(),
            price: r.price,
            stop_price: r.stop_price,
            quantity: r.quantity,
            price_type: r.price_type.into(),
            __bindgen_anon_1: union,
            business_type: r.business_type.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct XTPQueryOrderReq {
    pub ticker: String,
    pub begin_time: i64,
    pub end_time: i64,
}

impl From<&XTPQueryOrderReq> for sys::XTPQueryOrderReq {
    fn from(r: &XTPQueryOrderReq) -> sys::XTPQueryOrderReq {
        sys::XTPQueryOrderReq {
            ticker: string_to_carray16(&r.ticker),
            begin_time: r.begin_time,
            end_time: r.end_time,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct XTPQueryOrderByPageReq {
    pub req_count: i64,
    pub reference: i64,
    pub reserved: i64,
}

impl From<&XTPQueryOrderByPageReq> for sys::XTPQueryOrderByPageReq {
    fn from(r: &XTPQueryOrderByPageReq) -> sys::XTPQueryOrderByPageReq {
        sys::XTPQueryOrderByPageReq {
            req_count: r.req_count,
            reference: r.reference,
            reserved: r.reserved,
        }
    }
}

#[derive(Debug, Clone)]
pub struct XTPQueryTraderReq {
    pub ticker: String,
    pub begin_time: i64,
    pub end_time: i64,
}

impl From<&XTPQueryTraderReq> for sys::XTPQueryTraderReq {
    fn from(r: &XTPQueryTraderReq) -> sys::XTPQueryTraderReq {
        sys::XTPQueryTraderReq {
            ticker: string_to_carray16(&r.ticker),
            begin_time: r.begin_time,
            end_time: r.end_time,
        }
    }
}

#[derive(Debug, Clone)]
pub struct XTPQueryTraderByPageReq {
    pub req_count: i64,
    pub reference: i64,
    pub reserved: i64,
}

impl From<&XTPQueryTraderByPageReq> for sys::XTPQueryTraderByPageReq {
    fn from(r: &XTPQueryTraderByPageReq) -> sys::XTPQueryTraderByPageReq {
        sys::XTPQueryTraderByPageReq {
            req_count: r.req_count,
            reference: r.reference,
            reserved: r.reserved,
        }
    }
}

#[derive(Debug, Clone)]
pub struct XTPQueryStructuredFundInfoReq {
    pub exchange_id: XTPExchangeType,
    pub sf_ticker: String,
}

impl From<&XTPQueryStructuredFundInfoReq> for sys::XTPQueryStructuredFundInfoReq {
    fn from(r: &XTPQueryStructuredFundInfoReq) -> sys::XTPQueryStructuredFundInfoReq {
        sys::XTPQueryStructuredFundInfoReq {
            exchange_id: r.exchange_id.into(),
            sf_ticker: string_to_carray16(&r.sf_ticker),
        }
    }
}

#[derive(Clone)]
pub struct XTPFundTransferReq {
    pub serial_id: u64,
    pub fund_account: String,
    pub password: String,
    pub amount: f64,
    pub transfer_type: XTPFundTransferType,
}

impl From<&XTPFundTransferReq> for sys::XTPFundTransferReq {
    fn from(r: &XTPFundTransferReq) -> sys::XTPFundTransferReq {
        sys::XTPFundTransferReq {
            serial_id: r.serial_id,
            fund_account: string_to_carray16(&r.fund_account),
            password: string_to_carray64(&r.password),
            amount: r.amount,
            transfer_type: r.transfer_type.into(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct XTPQueryFundTransferLogReq {
    pub serial_id: u64,
}

impl From<&XTPQueryFundTransferLogReq> for sys::XTPQueryFundTransferLogReq {
    fn from(r: &XTPQueryFundTransferLogReq) -> sys::XTPQueryFundTransferLogReq {
        sys::XTPQueryFundTransferLogReq {
            serial_id: r.serial_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct XTPQueryETFBaseReq {
    pub market: XTPMarketType,
    pub ticker: String,
}

impl From<&XTPQueryETFBaseReq> for sys::XTPQueryETFBaseReq {
    fn from(r: &XTPQueryETFBaseReq) -> sys::XTPQueryETFBaseReq {
        sys::XTPQueryETFBaseReq {
            market: r.market.into(),
            ticker: string_to_carray16(&r.ticker),
        }
    }
}

#[derive(Debug, Clone)]
pub struct XTPQueryETFComponentReq {
    pub market: XTPMarketType,
    pub ticker: String,
}

impl From<&XTPQueryETFComponentReq> for sys::XTPQueryETFComponentReq {
    fn from(r: &XTPQueryETFComponentReq) -> sys::XTPQueryETFComponentReq {
        sys::XTPQueryETFComponentReq {
            market: r.market.into(),
            ticker: string_to_carray16(&r.ticker),
        }
    }
}

#[derive(Debug, Clone)]
pub struct XTPQueryOptionAuctionInfoReq {
    pub market: XTPMarketType,
    pub ticker: String,
}

impl From<&XTPQueryOptionAuctionInfoReq> for sys::XTPQueryOptionAuctionInfoReq {
    fn from(r: &XTPQueryOptionAuctionInfoReq) -> sys::XTPQueryOptionAuctionInfoReq {
        sys::XTPQueryOptionAuctionInfoReq {
            market: r.market.into(),
            ticker: string_to_carray16(&r.ticker),
        }
    }
}
// #[derive(Debug, Copy, Clone)]
// pub struct XTPTickByTickEntrust {
//     pub channel_no: i32,
//     pub seq: i64,
//     pub price: f64,
//     pub qty: i64,
//     pub side: char,
//     pub ord_type: char,
// }

// impl XTPTickByTickEntrust {
//     pub unsafe fn from_raw(ob: &XTPOB) -> Self {
//         XTPTickByTickEntrust {
//             channel_no: ob.channel_no,
//             seq: ob.seq,
//             price: ob.price,
//             qty: ob.qty,
//             side: ob.side,
//             ord_type: ob.ord_type,
//         }
//     }
// }

pub(crate) fn carray_to_string(ptr: &[i8]) -> String {
    let string = unsafe { CStr::from_ptr(ptr as *const [i8] as *const i8) };
    let string = string.to_owned().to_string_lossy().to_string();
    string
}

pub(crate) fn string_to_carray16(s: &str) -> [i8; 16] {
    let mut sarr = [0i8; 16];

    for (i, &byte) in s.as_bytes()[..16].into_iter().enumerate() {
        sarr[i] = byte as i8;
    }

    sarr
}

pub(crate) fn string_to_carray64(s: &str) -> [i8; 64] {
    let mut sarr = [0i8; 64];

    for (i, &byte) in s.as_bytes()[..64].into_iter().enumerate() {
        sarr[i] = byte as i8;
    }

    sarr
}
