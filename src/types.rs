#![allow(unused_comparisons)]
use crate::sys::{
    self, TXTPOrderTypeType, TXTPTradeTypeType, XTPOrderInfo__bindgen_ty_1__bindgen_ty_1,
    XTPOrderInsertInfo__bindgen_ty_1, __BindgenUnionField, ETF_REPLACE_TYPE, XTPMD, XTPOB, XTPQSI,
    XTPRI, XTPST, XTPTBT, XTPTPI, XTP_ACCOUNT_TYPE, XTP_BUSINESS_TYPE, XTP_EXCHANGE_TYPE,
    XTP_FUND_OPER_STATUS, XTP_FUND_TRANSFER_TYPE, XTP_LOG_LEVEL, XTP_MARKETDATA_TYPE,
    XTP_MARKET_TYPE, XTP_OPT_CALL_OR_PUT_TYPE, XTP_OPT_EXERCISE_TYPE_TYPE,
    XTP_ORDER_ACTION_STATUS_TYPE, XTP_ORDER_STATUS_TYPE, XTP_ORDER_SUBMIT_STATUS_TYPE,
    XTP_POSITION_DIRECTION_TYPE, XTP_POSITION_EFFECT_TYPE, XTP_PRICE_TYPE, XTP_PROTOCOL_TYPE,
    XTP_SIDE_TYPE, XTP_SPLIT_MERGE_STATUS, XTP_TBT_TYPE, XTP_TE_RESUME_TYPE, XTP_TICKER_TYPE,
};
use libc::c_char;
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

pub type XTPOrderInfoUnion = XTPOrderInfo__bindgen_ty_1__bindgen_ty_1;

/// XTP_LOG_LEVEL是日志输出级别类型
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum XTPLogLevel {
    /// 严重错误级别
    Fatal = XTP_LOG_LEVEL::XTP_LOG_LEVEL_FATAL as u32,
    /// 错误级别
    Error = XTP_LOG_LEVEL::XTP_LOG_LEVEL_ERROR as u32,
    /// 警告级别
    Warning = XTP_LOG_LEVEL::XTP_LOG_LEVEL_WARNING as u32,
    /// info级别
    Info = XTP_LOG_LEVEL::XTP_LOG_LEVEL_INFO as u32,
    /// debug级别
    Debug = XTP_LOG_LEVEL::XTP_LOG_LEVEL_DEBUG as u32,
    /// trace级别
    Trace = XTP_LOG_LEVEL::XTP_LOG_LEVEL_TRACE as u32,
}
impl_ffi_convert!(XTPLogLevel, XTP_LOG_LEVEL, 5);

/// XTP_PROTOCOL_TYPE是通讯传输协议方式
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum XTPProtocolType {
    /// Use TCP transmission
    TCP = XTP_PROTOCOL_TYPE::XTP_PROTOCOL_TCP as u32,
    /// Use UDP transmission (only support market data)
    UDP = XTP_PROTOCOL_TYPE::XTP_PROTOCOL_UDP as u32,
}
impl_ffi_convert!(XTPProtocolType, XTP_PROTOCOL_TYPE, 1, 2);

/// XTP_EXCHANGE_TYPE是交易所类型
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

/// XTP_MARKET_TYPE市场类型
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum XTPMarketType {
    /// Initializing or unknown
    MarketInit = XTP_MARKET_TYPE::XTP_MKT_INIT as u32,
    /// Shenzhen A-shares
    SZA = XTP_MARKET_TYPE::XTP_MKT_SZ_A as u32,
    /// Shanghai A-shares
    SHA = XTP_MARKET_TYPE::XTP_MKT_SH_A as u32,
    /// Unknown market type
    UNKNOWN = XTP_MARKET_TYPE::XTP_MKT_UNKNOWN as u32,
}
impl_ffi_convert!(XTPMarketType, XTP_MARKET_TYPE, 3);

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum XTPMarketdataType {
    /// 现货(股票/基金/债券等)
    Actual = XTP_MARKETDATA_TYPE::XTP_MARKETDATA_ACTUAL as u32,
    /// 期权
    Option = XTP_MARKETDATA_TYPE::XTP_MARKETDATA_OPTION as u32,
}
impl_ffi_convert!(XTPMarketdataType, XTP_MARKETDATA_TYPE, 1);

/// XTP_PRICE_TYPE是价格类型
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

/// XTP_SIDE_TYPE是买卖方向类型
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum XTPSideType {
    /// 买（新股申购，ETF买，配股，信用交易中担保品买）
    Buy = XTP_SIDE_TYPE::XTP_SIDE_BUY as u8,
    /// 卖（逆回购，ETF卖，信用交易中担保品卖）
    Sell = XTP_SIDE_TYPE::XTP_SIDE_SELL as u8,
    /// 申购
    Purchase = XTP_SIDE_TYPE::XTP_SIDE_PURCHASE as u8,
    /// 赎回
    Redemption = XTP_SIDE_TYPE::XTP_SIDE_REDEMPTION as u8,
    /// 拆分
    Split = XTP_SIDE_TYPE::XTP_SIDE_SPLIT as u8,
    /// 合并
    Merge = XTP_SIDE_TYPE::XTP_SIDE_MERGE as u8,
    /// 改版之后的side的备兑，暂不支持
    Cover = XTP_SIDE_TYPE::XTP_SIDE_COVER as u8,
    /// 改版之后的side锁定（对应开平标识为开）/解锁（对应开平标识为平）
    Freeze = XTP_SIDE_TYPE::XTP_SIDE_FREEZE as u8,
    /// 融资买入
    MarginTrade = XTP_SIDE_TYPE::XTP_SIDE_MARGIN_TRADE as u8,
    /// 融券卖出
    ShortSell = XTP_SIDE_TYPE::XTP_SIDE_SHORT_SELL as u8,
    /// 卖券还款
    RepayMargin = XTP_SIDE_TYPE::XTP_SIDE_REPAY_MARGIN as u8,
    /// 买券还券
    RepayStock = XTP_SIDE_TYPE::XTP_SIDE_REPAY_STOCK as u8,
    /// 现券还券
    StockRepayStock = XTP_SIDE_TYPE::XTP_SIDE_STOCK_REPAY_STOCK as u8,
    /// 余券划转
    SurstkTrans = XTP_SIDE_TYPE::XTP_SIDE_SURSTK_TRANS as u8,
    /// 担保品转入
    GrtstkTransin = XTP_SIDE_TYPE::XTP_SIDE_GRTSTK_TRANSIN as u8,
    /// 担保品转出
    GrtstkTransout = XTP_SIDE_TYPE::XTP_SIDE_GRTSTK_TRANSOUT as u8,
    ///未知或者无效买卖方向
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
impl_ffi_convert!(XTPOrderStatusType, XTP_ORDER_STATUS_TYPE, 7);

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
impl_ffi_convert!(XTPTeResumeType, XTP_TE_RESUME_TYPE, 2);

/// ETF_REPLACE_TYPE现金替代标识定义
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ETFReplaceType {
    /// 禁止现金替代
    Forbidden = ETF_REPLACE_TYPE::ERT_CASH_FORBIDDEN as u32,
    /// 可以现金替代
    Optional = ETF_REPLACE_TYPE::ERT_CASH_OPTIONAL as u32,
    /// 必须现金替代
    Must = ETF_REPLACE_TYPE::ERT_CASH_MUST as u32,
    /// 深市退补现金替代
    RecomputeInterSZ = ETF_REPLACE_TYPE::ERT_CASH_RECOMPUTE_INTER_SZ as u32,
    /// 深市必须现金替代
    MustInterSZ = ETF_REPLACE_TYPE::ERT_CASH_MUST_INTER_SZ as u32,
    /// 非沪深市场成分证券退补现金替代
    RecomputeInterOther = ETF_REPLACE_TYPE::ERT_CASH_RECOMPUTE_INTER_OTHER as u32,
    /// 表示非沪深市场成份证券必须现金替代
    MustInterOther = ETF_REPLACE_TYPE::ERT_CASH_MUST_INTER_OTHER as u32,
    /// 无效值
    Invalid = ETF_REPLACE_TYPE::EPT_INVALID as u32,
}
impl_ffi_convert!(ETFReplaceType, ETF_REPLACE_TYPE, 7);

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
impl_ffi_convert!(XTPTickerType, XTP_TICKER_TYPE, 6);

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
impl_ffi_convert!(XTPBusinessType, XTP_BUSINESS_TYPE, 13);

/// 账户类型
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum XTPAccountType {
    /// 普通账户
    Normal = XTP_ACCOUNT_TYPE::XTP_ACCOUNT_NORMAL as u32,
    /// 信用账户
    Credit = XTP_ACCOUNT_TYPE::XTP_ACCOUNT_CREDIT as u32,
    /// 衍生品账户
    Derive = XTP_ACCOUNT_TYPE::XTP_ACCOUNT_DERIVE as u32,
    /// 未知账户类型
    Unknown = XTP_ACCOUNT_TYPE::XTP_ACCOUNT_UNKNOWN as u32,
}
impl_ffi_convert!(XTPAccountType, XTP_ACCOUNT_TYPE, 3);

/// 资金流转方向类型
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
impl_ffi_convert!(XTPFundTransferType, XTP_FUND_TRANSFER_TYPE, 4);

/// XTP_FUND_OPER_STATUS柜台资金操作结果
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum XTPFundOperStatus {
    /// XTP已收到，正在处理中
    PROCESSING = XTP_FUND_OPER_STATUS::XTP_FUND_OPER_PROCESSING as u32,
    /// 成功
    SUCCESS = XTP_FUND_OPER_STATUS::XTP_FUND_OPER_SUCCESS as u32,
    /// 失败
    FAILED = XTP_FUND_OPER_STATUS::XTP_FUND_OPER_FAILED as u32,
    /// 已提交到集中柜台处理
    SUBMITTED = XTP_FUND_OPER_STATUS::XTP_FUND_OPER_SUBMITTED as u32,
    /// 未知
    UNKNOWN = XTP_FUND_OPER_STATUS::XTP_FUND_OPER_UNKNOWN as u32,
}
impl_ffi_convert!(XTPFundOperStatus, XTP_FUND_OPER_STATUS, 4);

/// XTP_SPLIT_MERGE_STATUS是一个基金当天拆分合并状态类型
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum XTPSplitMergeStatus {
    /// 允许拆分和合并
    Allow = XTP_SPLIT_MERGE_STATUS::XTP_SPLIT_MERGE_STATUS_ALLOW as u32,
    /// 只允许拆分，不允许合并
    OnlySplit = XTP_SPLIT_MERGE_STATUS::XTP_SPLIT_MERGE_STATUS_ONLY_SPLIT as u32,
    /// 只允许合并，不允许拆分
    OnlyMerge = XTP_SPLIT_MERGE_STATUS::XTP_SPLIT_MERGE_STATUS_ONLY_MERGE as u32,
    /// 不允许拆分合并
    Forbidden = XTP_SPLIT_MERGE_STATUS::XTP_SPLIT_MERGE_STATUS_FORBIDDEN as u32,
}
impl_ffi_convert!(XTPSplitMergeStatus, XTP_SPLIT_MERGE_STATUS, 3);

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

/// XTP_OPT_CALL_OR_PUT_TYPE是一个认沽或认购类型
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum XTPOptCallOrPutType {
    /// 认购
    CALL = XTP_OPT_CALL_OR_PUT_TYPE::XTP_OPT_CALL as u32,
    /// 认沽
    PUT = XTP_OPT_CALL_OR_PUT_TYPE::XTP_OPT_PUT as u32,
}
impl_ffi_convert!(XTPOptCallOrPutType, XTP_OPT_CALL_OR_PUT_TYPE, 1, 2);

/// XTP_OPT_EXERCISE_TYPE_TYPE是一个行权方式类型
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum XTPOptExerciseTypeType {
    EUR = XTP_OPT_EXERCISE_TYPE_TYPE::XTP_OPT_EXERCISE_TYPE_EUR as u32,
    AME = XTP_OPT_EXERCISE_TYPE_TYPE::XTP_OPT_EXERCISE_TYPE_AME as u32,
}
impl_ffi_convert!(XTPOptExerciseTypeType, XTP_OPT_EXERCISE_TYPE_TYPE, 1, 2);

/// XTP_POSITION_DIRECTION_TYPE是一个持仓方向类型
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum XTPPositionDirectionType {
    /// 净
    Net = XTP_POSITION_DIRECTION_TYPE::XTP_POSITION_DIRECTION_NET as u32,
    /// 多（期权则为权利方）
    Long = XTP_POSITION_DIRECTION_TYPE::XTP_POSITION_DIRECTION_LONG as u32,
    /// 空（期权则为义务方）
    Short = XTP_POSITION_DIRECTION_TYPE::XTP_POSITION_DIRECTION_SHORT as u32,
    /// 备兑（期权则为备兑义务方）
    Covered = XTP_POSITION_DIRECTION_TYPE::XTP_POSITION_DIRECTION_COVERED as u32,
}
impl_ffi_convert!(XTPPositionDirectionType, XTP_POSITION_DIRECTION_TYPE, 3);

#[derive(Debug, Clone)]
pub struct XTPRspInfoStruct {
    pub error_id: i32,
    pub error_msg: String,
}

impl<'a> FromRaw<&'a XTPRI> for XTPRspInfoStruct {
    unsafe fn from_raw(
        XTPRI {
            error_id,
            error_msg,
        }: &'a XTPRI,
    ) -> Self {
        let error_msg = FromCBuf::from_c_buf(error_msg.as_ref());
        XTPRspInfoStruct {
            error_id: *error_id,
            error_msg: error_msg,
        }
    }
}

#[derive(Clone, Debug)]
pub struct XTPSpecificTickerStruct {
    pub exchange_id: XTPExchangeType,
    pub ticker: String,
}

impl<'a> FromRaw<&'a XTPST> for XTPSpecificTickerStruct {
    unsafe fn from_raw(
        XTPST {
            exchange_id,
            ticker,
        }: &'a XTPST,
    ) -> Self {
        XTPSpecificTickerStruct {
            exchange_id: XTPExchangeType::from_raw(*exchange_id),
            ticker: FromCBuf::from_c_buf(ticker.as_ref()),
        }
    }
}

#[derive(Debug, Clone)]
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

impl<'a> FromRaw<&'a XTPOB> for OrderBookStruct {
    unsafe fn from_raw(ob: &'a XTPOB) -> Self {
        OrderBookStruct {
            exchange_id: XTPExchangeType::from_raw(ob.exchange_id),
            ticker: FromCBuf::from_c_buf(ob.ticker.as_ref()),
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

impl<'a> FromRaw<&'a XTPTBT> for XTPTickByTickStruct {
    unsafe fn from_raw(tbt: &'a XTPTBT) -> Self {
        XTPTickByTickStruct {
            exchange_id: XTPExchangeType::from_raw(tbt.exchange_id),
            ticker: FromCBuf::from_c_buf(tbt.ticker.as_ref()),
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

impl<'a> FromRaw<&'a XTPQSI> for XTPQuoteStaticInfo {
    unsafe fn from_raw(qsi: &'a XTPQSI) -> Self {
        XTPQuoteStaticInfo {
            exchange_id: XTPExchangeType::from_raw(qsi.exchange_id),
            ticker: FromCBuf::from_c_buf(qsi.ticker.as_ref()),
            ticker_name: FromCBuf::from_c_buf(qsi.ticker_name.as_ref()),
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

#[derive(Clone, Debug)]
pub struct XTPTickerPriceInfo {
    pub exchange_id: XTPExchangeType,
    pub ticker: String,
    pub last_price: f64,
}

impl<'a> FromRaw<&'a XTPTPI> for XTPTickerPriceInfo {
    unsafe fn from_raw(tpi: &'a XTPTPI) -> Self {
        XTPTickerPriceInfo {
            exchange_id: XTPExchangeType::from_raw(tpi.exchange_id),
            ticker: FromCBuf::from_c_buf(tpi.ticker.as_ref()),
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

impl<'a> FromRaw<&'a XTPMD> for XTPMarketDataStruct {
    unsafe fn from_raw(md: &'a XTPMD) -> Self {
        XTPMarketDataStruct {
            exchange_id: XTPExchangeType::from_raw(md.exchange_id),
            ticker: FromCBuf::from_c_buf(md.ticker.as_ref()),
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
            ticker_status: FromCBuf::from_c_buf(md.ticker_status.as_ref()),
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

impl<'a> From<&'a XTPOrderInsertInfo> for sys::XTPOrderInsertInfo {
    fn from(r: &'a XTPOrderInsertInfo) -> sys::XTPOrderInsertInfo {
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
            ticker: r.ticker.to_c_buf16(),
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

impl<'a> From<&'a XTPQueryOrderReq> for sys::XTPQueryOrderReq {
    fn from(r: &XTPQueryOrderReq) -> sys::XTPQueryOrderReq {
        sys::XTPQueryOrderReq {
            ticker: r.ticker.to_c_buf16(),
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

impl<'a> From<&'a XTPQueryTraderReq> for sys::XTPQueryTraderReq {
    fn from(r: &XTPQueryTraderReq) -> sys::XTPQueryTraderReq {
        sys::XTPQueryTraderReq {
            ticker: r.ticker.to_c_buf16(),
            begin_time: r.begin_time,
            end_time: r.end_time,
        }
    }
}

#[derive(Debug, Copy, Clone)]
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

impl<'a> From<&'a XTPQueryStructuredFundInfoReq> for sys::XTPQueryStructuredFundInfoReq {
    fn from(r: &'a XTPQueryStructuredFundInfoReq) -> sys::XTPQueryStructuredFundInfoReq {
        sys::XTPQueryStructuredFundInfoReq {
            exchange_id: r.exchange_id.into(),
            sf_ticker: r.sf_ticker.to_c_buf16(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct XTPFundTransferReq {
    pub serial_id: u64,
    pub fund_account: String,
    pub password: String,
    pub amount: f64,
    pub transfer_type: XTPFundTransferType,
}

impl<'a> From<&'a XTPFundTransferReq> for sys::XTPFundTransferReq {
    fn from(r: &'a XTPFundTransferReq) -> sys::XTPFundTransferReq {
        sys::XTPFundTransferReq {
            serial_id: r.serial_id,
            fund_account: r.fund_account.to_c_buf16(),
            password: r.password.to_c_buf64(),
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

impl<'a> From<&'a XTPQueryETFBaseReq> for sys::XTPQueryETFBaseReq {
    fn from(r: &XTPQueryETFBaseReq) -> sys::XTPQueryETFBaseReq {
        sys::XTPQueryETFBaseReq {
            market: r.market.into(),
            ticker: r.ticker.to_c_buf16(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct XTPQueryETFComponentReq {
    pub market: XTPMarketType,
    pub ticker: String,
}

impl<'a> From<&'a XTPQueryETFComponentReq> for sys::XTPQueryETFComponentReq {
    fn from(r: &XTPQueryETFComponentReq) -> sys::XTPQueryETFComponentReq {
        sys::XTPQueryETFComponentReq {
            market: r.market.into(),
            ticker: r.ticker.to_c_buf16(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct XTPQueryOptionAuctionInfoReq {
    pub market: XTPMarketType,
    pub ticker: String,
}

impl<'a> From<&'a XTPQueryOptionAuctionInfoReq> for sys::XTPQueryOptionAuctionInfoReq {
    fn from(r: &XTPQueryOptionAuctionInfoReq) -> sys::XTPQueryOptionAuctionInfoReq {
        sys::XTPQueryOptionAuctionInfoReq {
            market: r.market.into(),
            ticker: r.ticker.to_c_buf16(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct XTPOrderInfo {
    pub order_xtp_id: u64,
    pub order_client_id: u32,
    pub order_cancel_client_id: u32,
    pub order_cancel_xtp_id: u64,
    pub ticker: String,
    pub market: XTPMarketType,
    pub price: f64,
    pub quantity: i64,
    pub price_type: XTPPriceType,
    pub side: XTPSideType,
    pub position_effect: XTPPositionEffectType,
    pub business_type: XTPBusinessType,
    pub qty_traded: i64,
    pub qty_left: i64,
    pub insert_time: i64,
    pub update_time: i64,
    pub cancel_time: i64,
    pub trade_amount: f64,
    pub order_local_id: String,
    pub order_status: XTPOrderStatusType,
    pub order_submit_status: XTPOrderSubmitStatusType,
    pub order_type: TXTPOrderTypeType,
}

impl<'a> FromRaw<&'a sys::XTPOrderInfo> for XTPOrderInfo {
    unsafe fn from_raw(r: &sys::XTPOrderInfo) -> XTPOrderInfo {
        let union = transmute::<_, &XTPOrderInfoUnion>(&r.__bindgen_anon_1);

        XTPOrderInfo {
            order_xtp_id: r.order_xtp_id,
            order_client_id: r.order_client_id,
            order_cancel_client_id: r.order_cancel_client_id,
            order_cancel_xtp_id: r.order_cancel_xtp_id,
            ticker: FromCBuf::from_c_buf(r.ticker.as_ref()),
            market: XTPMarketType::from_raw(r.market),
            price: r.price,
            quantity: r.quantity,
            price_type: XTPPriceType::from_raw(r.price_type),
            side: XTPSideType::from_raw(union.side),
            position_effect: XTPPositionEffectType::from_raw(union.position_effect),
            business_type: XTPBusinessType::from_raw(r.business_type),
            qty_traded: r.qty_traded,
            qty_left: r.qty_left,
            insert_time: r.insert_time,
            update_time: r.update_time,
            cancel_time: r.cancel_time,
            trade_amount: r.trade_amount,
            order_local_id: FromCBuf::from_c_buf(r.order_local_id.as_ref()),
            order_status: XTPOrderStatusType::from_raw(r.order_status),
            order_submit_status: XTPOrderSubmitStatusType::from_raw(r.order_submit_status),
            order_type: r.order_type,
        }
    }
}

#[derive(Debug, Clone)]
pub struct XTPTradeReport {
    pub order_xtp_id: u64,
    pub order_client_id: u32,
    pub ticker: String,
    pub market: XTPMarketType,
    pub local_order_id: u64,
    pub exec_id: String,
    pub price: f64,
    pub quantity: i64,
    pub trade_time: i64,
    pub trade_amount: f64,
    pub report_index: u64,
    pub order_exch_id: String,
    pub trade_type: TXTPTradeTypeType,
    pub side: XTPSideType,
    pub position_effect: XTPPositionEffectType,
    pub business_type: XTPBusinessType,
    pub branch_pbu: String,
}

impl<'a> FromRaw<&'a sys::XTPTradeReport> for XTPTradeReport {
    unsafe fn from_raw(r: &sys::XTPTradeReport) -> XTPTradeReport {
        let union = transmute::<_, &XTPOrderInfoUnion>(&r.__bindgen_anon_1);

        XTPTradeReport {
            order_xtp_id: r.order_xtp_id,
            order_client_id: r.order_client_id,
            ticker: FromCBuf::from_c_buf(r.ticker.as_ref()),
            market: XTPMarketType::from_raw(r.market),
            local_order_id: r.local_order_id,
            exec_id: FromCBuf::from_c_buf(r.exec_id.as_ref()),
            price: r.price,
            quantity: r.quantity,
            trade_time: r.trade_time,
            trade_amount: r.trade_amount,
            report_index: r.report_index,
            order_exch_id: FromCBuf::from_c_buf(r.order_exch_id.as_ref()),
            trade_type: r.trade_type,
            side: XTPSideType::from_raw(union.side),
            position_effect: XTPPositionEffectType::from_raw(union.position_effect),
            business_type: XTPBusinessType::from_raw(r.business_type),
            branch_pbu: FromCBuf::from_c_buf(r.branch_pbu.as_ref()),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct XTPOrderCancelInfo {
    pub order_cancel_xtp_id: u64,
    pub order_xtp_id: u64,
}

impl FromRaw<&sys::XTPOrderCancelInfo> for XTPOrderCancelInfo {
    unsafe fn from_raw(r: &sys::XTPOrderCancelInfo) -> XTPOrderCancelInfo {
        XTPOrderCancelInfo {
            order_cancel_xtp_id: r.order_cancel_xtp_id,
            order_xtp_id: r.order_xtp_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct XTPQueryStkPositionRsp {
    pub ticker: String,
    pub ticker_name: String,
    pub market: XTPMarketType,
    pub total_qty: i64,
    pub sellable_qty: i64,
    pub avg_price: f64,
    pub unrealized_pnl: f64,
    pub yesterday_position: i64,
    pub purchase_redeemable_qty: i64,
    pub position_direction: XTPPositionDirectionType,
    pub reserved1: u32,
    pub executable_option: i64,
    pub lockable_position: i64,
    pub executable_underlying: i64,
    pub locked_position: i64,
    pub usable_locked_position: i64,
}

impl<'a> FromRaw<&'a sys::XTPQueryStkPositionRsp> for XTPQueryStkPositionRsp {
    unsafe fn from_raw(r: &sys::XTPQueryStkPositionRsp) -> XTPQueryStkPositionRsp {
        XTPQueryStkPositionRsp {
            ticker: FromCBuf::from_c_buf(r.ticker.as_ref()),
            ticker_name: FromCBuf::from_c_buf(r.ticker_name.as_ref()),
            market: XTPMarketType::from_raw(r.market),
            total_qty: r.total_qty,
            sellable_qty: r.sellable_qty,
            avg_price: r.avg_price,
            unrealized_pnl: r.unrealized_pnl,
            yesterday_position: r.yesterday_position,
            purchase_redeemable_qty: r.purchase_redeemable_qty,
            position_direction: XTPPositionDirectionType::from_raw(r.position_direction),
            reserved1: r.reserved1,
            executable_option: r.executable_option,
            lockable_position: r.lockable_position,
            executable_underlying: r.executable_underlying,
            locked_position: r.locked_position,
            usable_locked_position: r.usable_locked_position,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct XTPQueryAssetRsp {
    pub total_asset: f64,
    pub buying_power: f64,
    pub security_asset: f64,
    pub fund_buy_amount: f64,
    pub fund_buy_fee: f64,
    pub fund_sell_amount: f64,
    pub fund_sell_fee: f64,
    pub withholding_amount: f64,
    pub account_type: XTPAccountType,
    pub frozen_margin: f64,
    pub frozen_exec_cash: f64,
    pub frozen_exec_fee: f64,
    pub pay_later: f64,
    pub preadva_pay: f64,
    pub orig_banlance: f64,
    pub banlance: f64,
    pub deposit_withdraw: f64,
    pub trade_netting: f64,
    pub captial_asset: f64,
    pub force_freeze_amount: f64,
    pub preferred_amount: f64,
    pub repay_stock_aval_banlance: f64,
}

impl FromRaw<&sys::XTPQueryAssetRsp> for XTPQueryAssetRsp {
    unsafe fn from_raw(r: &sys::XTPQueryAssetRsp) -> XTPQueryAssetRsp {
        XTPQueryAssetRsp {
            total_asset: r.total_asset,
            buying_power: r.buying_power,
            security_asset: r.security_asset,
            fund_buy_amount: r.fund_buy_amount,
            fund_buy_fee: r.fund_buy_fee,
            fund_sell_amount: r.fund_sell_amount,
            fund_sell_fee: r.fund_sell_fee,
            withholding_amount: r.withholding_amount,
            account_type: XTPAccountType::from_raw(r.account_type),
            frozen_margin: r.frozen_margin,
            frozen_exec_cash: r.frozen_exec_cash,
            frozen_exec_fee: r.frozen_exec_fee,
            pay_later: r.pay_later,
            preadva_pay: r.preadva_pay,
            orig_banlance: r.orig_banlance,
            banlance: r.banlance,
            deposit_withdraw: r.deposit_withdraw,
            trade_netting: r.trade_netting,
            captial_asset: r.captial_asset,
            force_freeze_amount: r.force_freeze_amount,
            preferred_amount: r.preferred_amount,
            repay_stock_aval_banlance: r.repay_stock_aval_banlance,
        }
    }
}

#[derive(Debug, Clone)]
pub struct XTPStructuredFundInfo {
    pub exchange_id: XTPExchangeType,
    pub sf_ticker: String,
    pub sf_ticker_name: String,
    pub ticker: String,
    pub ticker_name: String,
    pub split_merge_status: XTPSplitMergeStatus,
    pub ratio: u32,
    pub min_split_qty: u32,
    pub min_merge_qty: u32,
    pub net_price: f64,
}

impl<'a> FromRaw<&'a sys::XTPStructuredFundInfo> for XTPStructuredFundInfo {
    unsafe fn from_raw(r: &sys::XTPStructuredFundInfo) -> XTPStructuredFundInfo {
        XTPStructuredFundInfo {
            exchange_id: XTPExchangeType::from_raw(r.exchange_id),
            sf_ticker: FromCBuf::from_c_buf(r.sf_ticker.as_ref()),
            sf_ticker_name: FromCBuf::from_c_buf(r.sf_ticker_name.as_ref()),
            ticker: FromCBuf::from_c_buf(r.ticker.as_ref()),
            ticker_name: FromCBuf::from_c_buf(r.ticker_name.as_ref()),
            split_merge_status: XTPSplitMergeStatus::from_raw(r.split_merge_status),
            ratio: r.ratio,
            min_split_qty: r.min_split_qty,
            min_merge_qty: r.min_merge_qty,
            net_price: r.net_price,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct XTPFundTransferNotice {
    pub serial_id: u64,
    pub transfer_type: XTPFundTransferType,
    pub amount: f64,
    pub oper_status: XTPFundOperStatus,
    pub transfer_time: u64,
}

impl<'a> FromRaw<&'a sys::XTPFundTransferNotice> for XTPFundTransferNotice {
    unsafe fn from_raw(r: &sys::XTPFundTransferNotice) -> XTPFundTransferNotice {
        XTPFundTransferNotice {
            serial_id: r.serial_id,
            transfer_type: XTPFundTransferType::from_raw(r.transfer_type),
            amount: r.amount,
            oper_status: XTPFundOperStatus::from_raw(r.oper_status),
            transfer_time: r.transfer_time,
        }
    }
}

#[derive(Debug, Clone)]
pub struct XTPQueryETFBaseRsp {
    pub market: XTPMarketType,
    pub etf: String,
    pub subscribe_redemption_ticker: String,
    pub unit: i32,
    pub subscribe_status: i32,
    pub redemption_status: i32,
    pub max_cash_ratio: f64,
    pub estimate_amount: f64,
    pub cash_component: f64,
    pub net_value: f64,
    pub total_amount: f64,
}

impl<'a> FromRaw<&'a sys::XTPQueryETFBaseRsp> for XTPQueryETFBaseRsp {
    unsafe fn from_raw(r: &sys::XTPQueryETFBaseRsp) -> XTPQueryETFBaseRsp {
        XTPQueryETFBaseRsp {
            market: XTPMarketType::from_raw(r.market),
            etf: FromCBuf::from_c_buf(r.etf.as_ref()),
            subscribe_redemption_ticker: FromCBuf::from_c_buf(
                r.subscribe_redemption_ticker.as_ref(),
            ),
            unit: r.unit,
            subscribe_status: r.subscribe_status,
            redemption_status: r.redemption_status,
            max_cash_ratio: r.max_cash_ratio,
            estimate_amount: r.estimate_amount,
            cash_component: r.cash_component,
            net_value: r.net_value,
            total_amount: r.total_amount,
        }
    }
}

#[derive(Debug, Clone)]
pub struct XTPQueryETFComponentRsp {
    pub market: XTPMarketType,
    pub ticker: String,
    pub component_ticker: String,
    pub component_name: String,
    pub quantity: i64,
    pub component_market: XTPMarketType,
    pub replace_type: ETFReplaceType,
    pub premium_ratio: f64,
    pub amount: f64,
}

impl<'a> FromRaw<&'a sys::XTPQueryETFComponentRsp> for XTPQueryETFComponentRsp {
    unsafe fn from_raw(r: &sys::XTPQueryETFComponentRsp) -> XTPQueryETFComponentRsp {
        XTPQueryETFComponentRsp {
            market: XTPMarketType::from_raw(r.market),
            ticker: FromCBuf::from_c_buf(r.ticker.as_ref()),
            component_ticker: FromCBuf::from_c_buf(r.component_ticker.as_ref()),
            component_name: FromCBuf::from_c_buf(r.component_name.as_ref()),
            quantity: r.quantity,
            component_market: XTPMarketType::from_raw(r.component_market),
            replace_type: ETFReplaceType::from_raw(r.replace_type),
            premium_ratio: r.premium_ratio,
            amount: r.amount,
        }
    }
}

#[derive(Debug, Clone)]
pub struct XTPQueryIPOTickerRsp {
    pub market: XTPMarketType,
    pub ticker: String,
    pub ticker_name: String,
    pub ticker_type: XTPTickerType,
    pub price: f64,
    pub unit: i32,
    pub qty_upper_limit: i32,
}

impl<'a> FromRaw<&'a sys::XTPQueryIPOTickerRsp> for XTPQueryIPOTickerRsp {
    unsafe fn from_raw(r: &sys::XTPQueryIPOTickerRsp) -> XTPQueryIPOTickerRsp {
        XTPQueryIPOTickerRsp {
            market: XTPMarketType::from_raw(r.market),
            ticker: FromCBuf::from_c_buf(r.ticker.as_ref()),
            ticker_name: FromCBuf::from_c_buf(r.ticker_name.as_ref()),
            ticker_type: XTPTickerType::from_raw(r.ticker_type),
            price: r.price,
            unit: r.unit,
            qty_upper_limit: r.qty_upper_limit,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct XTPQueryIPOQuotaRsp {
    pub market: XTPMarketType,
    pub quantity: i32,
    pub tech_quantity: i32,
    pub unused: i32,
}

impl<'a> FromRaw<&'a sys::XTPQueryIPOQuotaRsp> for XTPQueryIPOQuotaRsp {
    unsafe fn from_raw(r: &sys::XTPQueryIPOQuotaRsp) -> XTPQueryIPOQuotaRsp {
        XTPQueryIPOQuotaRsp {
            market: XTPMarketType::from_raw(r.market),
            quantity: r.quantity,
            tech_quantity: r.tech_quantity,
            unused: r.unused,
        }
    }
}

#[derive(Debug, Clone)]
pub struct XTPQueryOptionAuctionInfoRsp {
    pub ticker: String,
    pub security_id_source: XTPMarketType,
    pub symbol: String,
    pub contract_id: String,
    pub underlying_security_id: String,
    pub underlying_security_id_source: XTPMarketType,
    pub list_date: u32,
    pub last_trade_date: u32,
    pub ticker_type: XTPTickerType,
    pub day_trading: i32,
    pub call_or_put: XTPOptCallOrPutType,
    pub delivery_day: u32,
    pub delivery_month: u32,
    pub exercise_type: XTPOptExerciseTypeType,
    pub exercise_begin_date: u32,
    pub exercise_end_date: u32,
    pub exercise_price: f64,
    pub qty_unit: i64,
    pub contract_unit: i64,
    pub contract_position: i64,
    pub prev_close_price: f64,
    pub prev_clearing_price: f64,
    pub lmt_buy_max_qty: i64,
    pub lmt_buy_min_qty: i64,
    pub lmt_sell_max_qty: i64,
    pub lmt_sell_min_qty: i64,
    pub mkt_buy_max_qty: i64,
    pub mkt_buy_min_qty: i64,
    pub mkt_sell_max_qty: i64,
    pub mkt_sell_min_qty: i64,
    pub price_tick: f64,
    pub upper_limit_price: f64,
    pub lower_limit_price: f64,
    pub sell_margin: f64,
    pub margin_ratio_param1: f64,
    pub margin_ratio_param2: f64,
}

impl<'a> FromRaw<&'a sys::XTPQueryOptionAuctionInfoRsp> for XTPQueryOptionAuctionInfoRsp {
    unsafe fn from_raw(r: &sys::XTPQueryOptionAuctionInfoRsp) -> XTPQueryOptionAuctionInfoRsp {
        XTPQueryOptionAuctionInfoRsp {
            ticker: FromCBuf::from_c_buf(r.ticker.as_ref()),
            security_id_source: XTPMarketType::from_raw(r.security_id_source),
            symbol: FromCBuf::from_c_buf(r.symbol.as_ref()),
            contract_id: FromCBuf::from_c_buf(r.contract_id.as_ref()),
            underlying_security_id: FromCBuf::from_c_buf(r.underlying_security_id.as_ref()),
            underlying_security_id_source: XTPMarketType::from_raw(r.underlying_security_id_source),
            list_date: r.list_date,
            last_trade_date: r.last_trade_date,
            ticker_type: XTPTickerType::from_raw(r.ticker_type),
            day_trading: r.day_trading,
            call_or_put: XTPOptCallOrPutType::from_raw(r.call_or_put),
            delivery_day: r.delivery_day,
            delivery_month: r.delivery_month,
            exercise_type: XTPOptExerciseTypeType::from_raw(r.exercise_type),
            exercise_begin_date: r.exercise_begin_date,
            exercise_end_date: r.exercise_end_date,
            exercise_price: r.exercise_price,
            qty_unit: r.qty_unit,
            contract_unit: r.contract_unit,
            contract_position: r.contract_position,
            prev_close_price: r.prev_close_price,
            prev_clearing_price: r.prev_clearing_price,
            lmt_buy_max_qty: r.lmt_buy_max_qty,
            lmt_buy_min_qty: r.lmt_buy_min_qty,
            lmt_sell_max_qty: r.lmt_sell_max_qty,
            lmt_sell_min_qty: r.lmt_sell_min_qty,
            mkt_buy_max_qty: r.mkt_buy_max_qty,
            mkt_buy_min_qty: r.mkt_buy_min_qty,
            mkt_sell_max_qty: r.mkt_sell_max_qty,
            mkt_sell_min_qty: r.mkt_sell_min_qty,
            price_tick: r.price_tick,
            upper_limit_price: r.upper_limit_price,
            lower_limit_price: r.lower_limit_price,
            sell_margin: r.sell_margin,
            margin_ratio_param1: r.margin_ratio_param1,
            margin_ratio_param2: r.margin_ratio_param2,
        }
    }
}

trait FromCBuf<'a> {
    fn from_c_buf(b: &'a [c_char]) -> Self;
}

impl<'a> FromCBuf<'a> for &'a CStr {
    fn from_c_buf(b: &'a [c_char]) -> Self {
        // convert from &[i8] to &[u8]
        let b = unsafe { &*(b as *const _ as *const [u8]) };
        match b.iter().position(|&c| c == 0u8) {
            Some(pos) => unsafe { CStr::from_bytes_with_nul_unchecked(&b[..pos + 1]) },
            None => {
                let s = String::from_utf8(b.to_vec());
                println!("{:?}", s);
                unreachable!("String without null end"); // TODO: not sure if XTP follows this
            }
        }
    }
}

impl<'a> FromCBuf<'a> for String {
    fn from_c_buf(b: &'a [c_char]) -> Self {
        // convert from &[i8] to &[u8]
        let b = unsafe { &*(b as *const _ as *const [u8]) };
        let slice = match b.iter().position(|&c| c == 0u8) {
            Some(pos) => &b[..pos + 1],
            None => b,
        };
        unsafe { String::from_utf8_unchecked(slice.to_vec()) }
    }
}

trait ToCBuf {
    fn to_c_buf16(&self) -> [c_char; 16usize];
    fn to_c_buf64(&self) -> [c_char; 64usize];
}

impl ToCBuf for &CStr {
    fn to_c_buf16(&self) -> [c_char; 16usize] {
        let mut sarr = [0i8; 16];

        for (i, &byte) in self.to_bytes()[..16].into_iter().enumerate() {
            sarr[i] = byte as i8;
        }

        sarr
    }
    fn to_c_buf64(&self) -> [c_char; 64usize] {
        let mut sarr = [0i8; 64];

        for (i, &byte) in self.to_bytes()[..64].into_iter().enumerate() {
            sarr[i] = byte as i8;
        }

        sarr
    }
}

impl ToCBuf for String {
    fn to_c_buf16(&self) -> [c_char; 16usize] {
        let mut sarr = [0i8; 16];

        for (i, &byte) in self.as_bytes()[..16].into_iter().enumerate() {
            sarr[i] = byte as i8;
        }

        sarr
    }
    fn to_c_buf64(&self) -> [c_char; 64usize] {
        let mut sarr = [0i8; 64];

        for (i, &byte) in self.as_bytes()[..64].into_iter().enumerate() {
            sarr[i] = byte as i8;
        }

        sarr
    }
}
