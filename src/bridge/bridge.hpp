#ifndef XTP_RS_BRIDGE_H_
#define XTP_RS_BRIDGE_H_

#include "../../sdk/bin/include/xtp_trader_api.h"
#include "../../sdk/bin/include/xtp_quote_api.h"
#include "../../sdk/bin/include/xtp_api_struct.h"
#include "../../sdk/bin/include/xoms_api_fund_struct.h"
#include "../../sdk/bin/include/xoms_api_struct.h"
#include "../../sdk/bin/include/xquote_api_struct.h"
#include "../../sdk/bin/include/xtp_api_data_type.h"
#include "../../sdk/bin/include/xtp_api_struct_common.h"

extern "C" XTP::API::QuoteApi *CreateQuoteApi(uint8_t client_id, const char *save_file_path, XTP_LOG_LEVEL log_level = XTP_LOG_LEVEL_DEBUG);
extern "C" void QuoteApi_Release(XTP::API::QuoteApi *self);
extern "C" const char *QuoteApi_GetTradingDay(XTP::API::QuoteApi *self);
extern "C" const char *QuoteApi_GetApiVersion(XTP::API::QuoteApi *self);
extern "C" XTPRI *QuoteApi_GetApiLastError(XTP::API::QuoteApi *self);
extern "C" void QuoteApi_SetUDPBufferSize(XTP::API::QuoteApi *self, uint32_t buff_size);
extern "C" void QuoteApi_RegisterSpi(XTP::API::QuoteApi *self, XTP::API::QuoteSpi *spi);
extern "C" void QuoteApi_SetHeartBeatInterval(XTP::API::QuoteApi *self, uint32_t interval);
extern "C" int QuoteApi_SubscribeMarketData(XTP::API::QuoteApi *self, char *ticker[], int count, XTP_EXCHANGE_TYPE exchange_id);
extern "C" int QuoteApi_UnSubscribeMarketData(XTP::API::QuoteApi *self, char *ticker[], int count, XTP_EXCHANGE_TYPE exchange_id);
extern "C" int QuoteApi_SubscribeOrderBook(XTP::API::QuoteApi *self, char *ticker[], int count, XTP_EXCHANGE_TYPE exchange_id);
extern "C" int QuoteApi_UnSubscribeOrderBook(XTP::API::QuoteApi *self, char *ticker[], int count, XTP_EXCHANGE_TYPE exchange_id);
extern "C" int QuoteApi_SubscribeTickByTick(XTP::API::QuoteApi *self, char *ticker[], int count, XTP_EXCHANGE_TYPE exchange_id);
extern "C" int QuoteApi_UnSubscribeTickByTick(XTP::API::QuoteApi *self, char *ticker[], int count, XTP_EXCHANGE_TYPE exchange_id);
extern "C" int QuoteApi_SubscribeAllMarketData(XTP::API::QuoteApi *self, XTP_EXCHANGE_TYPE exchange_id = XTP_EXCHANGE_UNKNOWN);
extern "C" int QuoteApi_UnSubscribeAllMarketData(XTP::API::QuoteApi *self, XTP_EXCHANGE_TYPE exchange_id = XTP_EXCHANGE_UNKNOWN);
extern "C" int QuoteApi_SubscribeAllOrderBook(XTP::API::QuoteApi *self, XTP_EXCHANGE_TYPE exchange_id = XTP_EXCHANGE_UNKNOWN);
extern "C" int QuoteApi_UnSubscribeAllOrderBook(XTP::API::QuoteApi *self, XTP_EXCHANGE_TYPE exchange_id = XTP_EXCHANGE_UNKNOWN);
extern "C" int QuoteApi_SubscribeAllTickByTick(XTP::API::QuoteApi *self, XTP_EXCHANGE_TYPE exchange_id = XTP_EXCHANGE_UNKNOWN);
extern "C" int QuoteApi_UnSubscribeAllTickByTick(XTP::API::QuoteApi *self, XTP_EXCHANGE_TYPE exchange_id = XTP_EXCHANGE_UNKNOWN);
extern "C" int QuoteApi_Login(XTP::API::QuoteApi *self, const char *ip, int port, const char *user, const char *password, XTP_PROTOCOL_TYPE sock_type);
extern "C" int QuoteApi_Logout(XTP::API::QuoteApi *self);
extern "C" int QuoteApi_QueryAllTickers(XTP::API::QuoteApi *self, XTP_EXCHANGE_TYPE exchange_id);
extern "C" int QuoteApi_QueryTickersPriceInfo(XTP::API::QuoteApi *self, char *ticker[], int count, XTP_EXCHANGE_TYPE exchange_id);
extern "C" int QuoteApi_QueryAllTickersPriceInfo(XTP::API::QuoteApi *self);
extern "C" int QuoteApi_SubscribeAllOptionMarketData(XTP::API::QuoteApi *self, XTP_EXCHANGE_TYPE exchange_id = XTP_EXCHANGE_UNKNOWN);
extern "C" int QuoteApi_UnSubscribeAllOptionMarketData(XTP::API::QuoteApi *self, XTP_EXCHANGE_TYPE exchange_id = XTP_EXCHANGE_UNKNOWN);
extern "C" int QuoteApi_SubscribeAllOptionOrderBook(XTP::API::QuoteApi *self, XTP_EXCHANGE_TYPE exchange_id = XTP_EXCHANGE_UNKNOWN);
extern "C" int QuoteApi_UnSubscribeAllOptionOrderBook(XTP::API::QuoteApi *self, XTP_EXCHANGE_TYPE exchange_id = XTP_EXCHANGE_UNKNOWN);
extern "C" int QuoteApi_SubscribeAllOptionTickByTick(XTP::API::QuoteApi *self, XTP_EXCHANGE_TYPE exchange_id = XTP_EXCHANGE_UNKNOWN);
extern "C" int QuoteApi_UnSubscribeAllOptionTickByTick(XTP::API::QuoteApi *self, XTP_EXCHANGE_TYPE exchange_id = XTP_EXCHANGE_UNKNOWN);

extern "C" XTP::API::TraderApi *CreateTraderApi(uint8_t client_id, const char *save_file_path, XTP_LOG_LEVEL log_level = XTP_LOG_LEVEL_DEBUG);
extern "C" void TraderApi_Release(XTP::API::TraderApi *self);
extern "C" const char *TraderApi_GetTradingDay(XTP::API::TraderApi *self);
extern "C" void TraderApi_RegisterSpi(XTP::API::TraderApi *self, XTP::API::TraderSpi *spi);
extern "C" XTPRI *TraderApi_GetApiLastError(XTP::API::TraderApi *self);
extern "C" const char *TraderApi_GetApiVersion(XTP::API::TraderApi *self);
extern "C" uint8_t TraderApi_GetClientIDByXTPID(XTP::API::TraderApi *self, uint64_t order_xtp_id);
extern "C" const char *TraderApi_GetAccountByXTPID(XTP::API::TraderApi *self, uint64_t order_xtp_id);
extern "C" void TraderApi_SubscribePublicTopic(XTP::API::TraderApi *self, XTP_TE_RESUME_TYPE resume_type);
extern "C" void TraderApi_SetSoftwareVersion(XTP::API::TraderApi *self, const char *version);
extern "C" void TraderApi_SetSoftwareKey(XTP::API::TraderApi *self, const char *key);
extern "C" void TraderApi_SetHeartBeatInterval(XTP::API::TraderApi *self, uint32_t interval);
extern "C" uint64_t TraderApi_Login(XTP::API::TraderApi *self, const char *ip, int port, const char *user, const char *password, XTP_PROTOCOL_TYPE sock_type);
extern "C" int TraderApi_Logout(XTP::API::TraderApi *self, uint64_t session_id);
extern "C" bool TraderApi_IsServerRestart(XTP::API::TraderApi *self, uint64_t session_id);
extern "C" uint64_t TraderApi_InsertOrder(XTP::API::TraderApi *self, XTPOrderInsertInfo *order, uint64_t session_id);
extern "C" uint64_t TraderApi_CancelOrder(XTP::API::TraderApi *self, const uint64_t order_xtp_id, uint64_t session_id);
extern "C" int TraderApi_QueryOrderByXTPID(XTP::API::TraderApi *self, const uint64_t order_xtp_id, uint64_t session_id, int request_id);
extern "C" int TraderApi_QueryOrders(XTP::API::TraderApi *self, const XTPQueryOrderReq *query_param, uint64_t session_id, int request_id);
extern "C" int TraderApi_QueryOrdersByPage(XTP::API::TraderApi *self, const XTPQueryOrderByPageReq *query_param, uint64_t session_id, int request_id);
extern "C" int TraderApi_QueryTradesByXTPID(XTP::API::TraderApi *self, const uint64_t order_xtp_id, uint64_t session_id, int request_id);
extern "C" int TraderApi_QueryTrades(XTP::API::TraderApi *self, XTPQueryTraderReq *query_param, uint64_t session_id, int request_id);
extern "C" int TraderApi_QueryTradesByPage(XTP::API::TraderApi *self, const XTPQueryTraderByPageReq *query_param, uint64_t session_id, int request_id);
extern "C" int TraderApi_QueryPosition(XTP::API::TraderApi *self, const char *ticker, uint64_t session_id, int request_id);
extern "C" int TraderApi_QueryAsset(XTP::API::TraderApi *self, uint64_t session_id, int request_id);
extern "C" int TraderApi_QueryStructuredFund(XTP::API::TraderApi *self, XTPQueryStructuredFundInfoReq *query_param, uint64_t session_id, int request_id);
extern "C" uint64_t TraderApi_FundTransfer(XTP::API::TraderApi *self, XTPFundTransferReq *fund_transfer, uint64_t session_id);
extern "C" int TraderApi_QueryFundTransfer(XTP::API::TraderApi *self, XTPQueryFundTransferLogReq *query_param, uint64_t session_id, int request_id);
extern "C" int TraderApi_QueryETF(XTP::API::TraderApi *self, XTPQueryETFBaseReq *query_param, uint64_t session_id, int request_id);
extern "C" int TraderApi_QueryETFTickerBasket(XTP::API::TraderApi *self, XTPQueryETFComponentReq *query_param, uint64_t session_id, int request_id);
extern "C" int TraderApi_QueryIPOInfoList(XTP::API::TraderApi *self, uint64_t session_id, int request_id);
extern "C" int TraderApi_QueryIPOQuotaInfo(XTP::API::TraderApi *self, uint64_t session_id, int request_id);
extern "C" int TraderApi_QueryOptionAuctionInfo(XTP::API::TraderApi *self, XTPQueryOptionAuctionInfoReq *query_param, uint64_t session_id, int request_id);

class QuoteSpiStub : XTP::API::QuoteSpi
{
public:
    void *rust_object;
    QuoteSpiStub(void *rust_object);
    void OnDisconnected(int reason);
    void OnError(XTPRI *error_info);
    void OnSubMarketData(XTPST *ticker, XTPRI *error_info, bool is_last);
    void OnUnSubMarketData(XTPST *ticker, XTPRI *error_info, bool is_last);
    void OnDepthMarketData(XTPMD *market_data, int64_t bid1_qty[], int32_t bid1_count, int32_t max_bid1_count, int64_t ask1_qty[], int32_t ask1_count, int32_t max_ask1_count);
    void OnSubOrderBook(XTPST *ticker, XTPRI *error_info, bool is_last);
    void OnUnSubOrderBook(XTPST *ticker, XTPRI *error_info, bool is_last);
    void OnOrderBook(XTPOB *order_book);
    void OnSubTickByTick(XTPST *ticker, XTPRI *error_info, bool is_last);
    void OnUnSubTickByTick(XTPST *ticker, XTPRI *error_info, bool is_last);
    void OnTickByTick(XTPTBT *tbt_data);
    void OnSubscribeAllMarketData(XTP_EXCHANGE_TYPE exchange_id, XTPRI *error_info);
    void OnUnSubscribeAllMarketData(XTP_EXCHANGE_TYPE exchange_id, XTPRI *error_info);
    void OnSubscribeAllOrderBook(XTP_EXCHANGE_TYPE exchange_id, XTPRI *error_info);
    void OnUnSubscribeAllOrderBook(XTP_EXCHANGE_TYPE exchange_id, XTPRI *error_info);
    void OnSubscribeAllTickByTick(XTP_EXCHANGE_TYPE exchange_id, XTPRI *error_info);
    void OnUnSubscribeAllTickByTick(XTP_EXCHANGE_TYPE exchange_id, XTPRI *error_info);
    void OnQueryAllTickers(XTPQSI *ticker_info, XTPRI *error_info, bool is_last);
    void OnQueryTickersPriceInfo(XTPTPI *ticker_info, XTPRI *error_info, bool is_last);
    void OnSubscribeAllOptionMarketData(XTP_EXCHANGE_TYPE exchange_id, XTPRI *error_info);
    void OnUnSubscribeAllOptionMarketData(XTP_EXCHANGE_TYPE exchange_id, XTPRI *error_info);
    void OnSubscribeAllOptionOrderBook(XTP_EXCHANGE_TYPE exchange_id, XTPRI *error_info);
    void OnUnSubscribeAllOptionOrderBook(XTP_EXCHANGE_TYPE exchange_id, XTPRI *error_info);
    void OnSubscribeAllOptionTickByTick(XTP_EXCHANGE_TYPE exchange_id, XTPRI *error_info);
    void OnUnSubscribeAllOptionTickByTick(XTP_EXCHANGE_TYPE exchange_id, XTPRI *error_info);
    ~QuoteSpiStub();
};

extern "C" void QuoteSpiStub_Rust_OnDisconnected(void *rust_object, int reason);
extern "C" void QuoteSpiStub_Rust_OnError(void *rust_object, XTPRI *error_info);
extern "C" void QuoteSpiStub_Rust_OnSubMarketData(void *rust_object, XTPST *ticker, XTPRI *error_info, bool is_last);
extern "C" void QuoteSpiStub_Rust_OnUnSubMarketData(void *rust_object, XTPST *ticker, XTPRI *error_info, bool is_last);
extern "C" void QuoteSpiStub_Rust_OnDepthMarketData(void *rust_object, XTPMD *market_data, int64_t bid1_qty[], int32_t bid1_count, int32_t max_bid1_count, int64_t ask1_qty[], int32_t ask1_count, int32_t max_ask1_count);
extern "C" void QuoteSpiStub_Rust_OnSubOrderBook(void *rust_object, XTPST *ticker, XTPRI *error_info, bool is_last);
extern "C" void QuoteSpiStub_Rust_OnUnSubOrderBook(void *rust_object, XTPST *ticker, XTPRI *error_info, bool is_last);
extern "C" void QuoteSpiStub_Rust_OnOrderBook(void *rust_object, XTPOB *order_book);
extern "C" void QuoteSpiStub_Rust_OnSubTickByTick(void *rust_object, XTPST *ticker, XTPRI *error_info, bool is_last);
extern "C" void QuoteSpiStub_Rust_OnUnSubTickByTick(void *rust_object, XTPST *ticker, XTPRI *error_info, bool is_last);
extern "C" void QuoteSpiStub_Rust_OnTickByTick(void *rust_object, XTPTBT *tbt_data);
extern "C" void QuoteSpiStub_Rust_OnSubscribeAllMarketData(void *rust_object, XTP_EXCHANGE_TYPE exchange_id, XTPRI *error_info);
extern "C" void QuoteSpiStub_Rust_OnUnSubscribeAllMarketData(void *rust_object, XTP_EXCHANGE_TYPE exchange_id, XTPRI *error_info);
extern "C" void QuoteSpiStub_Rust_OnSubscribeAllOrderBook(void *rust_object, XTP_EXCHANGE_TYPE exchange_id, XTPRI *error_info);
extern "C" void QuoteSpiStub_Rust_OnUnSubscribeAllOrderBook(void *rust_object, XTP_EXCHANGE_TYPE exchange_id, XTPRI *error_info);
extern "C" void QuoteSpiStub_Rust_OnSubscribeAllTickByTick(void *rust_object, XTP_EXCHANGE_TYPE exchange_id, XTPRI *error_info);
extern "C" void QuoteSpiStub_Rust_OnUnSubscribeAllTickByTick(void *rust_object, XTP_EXCHANGE_TYPE exchange_id, XTPRI *error_info);
extern "C" void QuoteSpiStub_Rust_OnQueryAllTickers(void *rust_object, XTPQSI *ticker_info, XTPRI *error_info, bool is_last);
extern "C" void QuoteSpiStub_Rust_OnQueryTickersPriceInfo(void *rust_object, XTPTPI *ticker_info, XTPRI *error_info, bool is_last);
extern "C" void QuoteSpiStub_Rust_OnSubscribeAllOptionMarketData(void *rust_object, XTP_EXCHANGE_TYPE exchange_id, XTPRI *error_info);
extern "C" void QuoteSpiStub_Rust_OnUnSubscribeAllOptionMarketData(void *rust_object, XTP_EXCHANGE_TYPE exchange_id, XTPRI *error_info);
extern "C" void QuoteSpiStub_Rust_OnSubscribeAllOptionOrderBook(void *rust_object, XTP_EXCHANGE_TYPE exchange_id, XTPRI *error_info);
extern "C" void QuoteSpiStub_Rust_OnUnSubscribeAllOptionOrderBook(void *rust_object, XTP_EXCHANGE_TYPE exchange_id, XTPRI *error_info);
extern "C" void QuoteSpiStub_Rust_OnSubscribeAllOptionTickByTick(void *rust_object, XTP_EXCHANGE_TYPE exchange_id, XTPRI *error_info);
extern "C" void QuoteSpiStub_Rust_OnUnSubscribeAllOptionTickByTick(void *rust_object, XTP_EXCHANGE_TYPE exchange_id, XTPRI *error_info);
extern "C" void QuoteSpiStub_Rust_Destructor(void *rust_object);

class TraderSpiStub : XTP::API::TraderSpi
{
public:
    void *rust_object;
    TraderSpiStub(void *rust_object);
    void OnDisconnected(uint64_t session_id, int reason);
    void OnError(XTPRI *error_info);
    void OnOrderEvent(XTPOrderInfo *order_info, XTPRI *error_info, uint64_t session_id);
    void OnTradeEvent(XTPTradeReport *trade_info, uint64_t session_id);
    void OnCancelOrderError(XTPOrderCancelInfo *cancel_info, XTPRI *error_info, uint64_t session_id);
    void OnQueryOrder(XTPQueryOrderRsp *order_info, XTPRI *error_info, int request_id, bool is_last, uint64_t session_id);
    void OnQueryOrderByPage(XTPQueryOrderRsp *order_info, int64_t req_count, int64_t order_sequence, int64_t query_reference, int request_id, bool is_last, uint64_t session_id);
    void OnQueryTrade(XTPQueryTradeRsp *trade_info, XTPRI *error_info, int request_id, bool is_last, uint64_t session_id);
    void OnQueryTradeByPage(XTPQueryTradeRsp *trade_info, int64_t req_count, int64_t trade_sequence, int64_t query_reference, int request_id, bool is_last, uint64_t session_id);
    void OnQueryPosition(XTPQueryStkPositionRsp *position, XTPRI *error_info, int request_id, bool is_last, uint64_t session_id);
    void OnQueryAsset(XTPQueryAssetRsp *asset, XTPRI *error_info, int request_id, bool is_last, uint64_t session_id);
    void OnQueryStructuredFund(XTPStructuredFundInfo *fund_info, XTPRI *error_info, int request_id, bool is_last, uint64_t session_id);
    void OnQueryFundTransfer(XTPFundTransferNotice *fund_transfer_info, XTPRI *error_info, int request_id, bool is_last, uint64_t session_id);
    void OnFundTransfer(XTPFundTransferNotice *fund_transfer_info, XTPRI *error_info, uint64_t session_id);
    void OnQueryETF(XTPQueryETFBaseRsp *etf_info, XTPRI *error_info, int request_id, bool is_last, uint64_t session_id);
    void OnQueryETFBasket(XTPQueryETFComponentRsp *etf_component_info, XTPRI *error_info, int request_id, bool is_last, uint64_t session_id);
    void OnQueryIPOInfoList(XTPQueryIPOTickerRsp *ipo_info, XTPRI *error_info, int request_id, bool is_last, uint64_t session_id);
    void OnQueryIPOQuotaInfo(XTPQueryIPOQuotaRsp *quota_info, XTPRI *error_info, int request_id, bool is_last, uint64_t session_id);
    void OnQueryOptionAuctionInfo(XTPQueryOptionAuctionInfoRsp *option_info, XTPRI *error_info, int request_id, bool is_last, uint64_t session_id);
    ~TraderSpiStub();
};

extern "C" void TraderSpiStub_Rust_OnDisconnected(void *rust_object, uint64_t session_id, int reason);
extern "C" void TraderSpiStub_Rust_OnError(void *rust_object, XTPRI *error_info);
extern "C" void TraderSpiStub_Rust_OnOrderEvent(void *rust_object, XTPOrderInfo *order_info, XTPRI *error_info, uint64_t session_id);
extern "C" void TraderSpiStub_Rust_OnTradeEvent(void *rust_object, XTPTradeReport *trade_info, uint64_t session_id);
extern "C" void TraderSpiStub_Rust_OnCancelOrderError(void *rust_object, XTPOrderCancelInfo *cancel_info, XTPRI *error_info, uint64_t session_id);
extern "C" void TraderSpiStub_Rust_OnQueryOrder(void *rust_object, XTPQueryOrderRsp *order_info, XTPRI *error_info, int request_id, bool is_last, uint64_t session_id);
extern "C" void TraderSpiStub_Rust_OnQueryOrderByPage(void *rust_object, XTPQueryOrderRsp *order_info, int64_t req_count, int64_t order_sequence, int64_t query_reference, int request_id, bool is_last, uint64_t session_id);
extern "C" void TraderSpiStub_Rust_OnQueryTrade(void *rust_object, XTPQueryTradeRsp *trade_info, XTPRI *error_info, int request_id, bool is_last, uint64_t session_id);
extern "C" void TraderSpiStub_Rust_OnQueryTradeByPage(void *rust_object, XTPQueryTradeRsp *trade_info, int64_t req_count, int64_t trade_sequence, int64_t query_reference, int request_id, bool is_last, uint64_t session_id);
extern "C" void TraderSpiStub_Rust_OnQueryPosition(void *rust_object, XTPQueryStkPositionRsp *position, XTPRI *error_info, int request_id, bool is_last, uint64_t session_id);
extern "C" void TraderSpiStub_Rust_OnQueryAsset(void *rust_object, XTPQueryAssetRsp *asset, XTPRI *error_info, int request_id, bool is_last, uint64_t session_id);
extern "C" void TraderSpiStub_Rust_OnQueryStructuredFund(void *rust_object, XTPStructuredFundInfo *fund_info, XTPRI *error_info, int request_id, bool is_last, uint64_t session_id);
extern "C" void TraderSpiStub_Rust_OnQueryFundTransfer(void *rust_object, XTPFundTransferNotice *fund_transfer_info, XTPRI *error_info, int request_id, bool is_last, uint64_t session_id);
extern "C" void TraderSpiStub_Rust_OnFundTransfer(void *rust_object, XTPFundTransferNotice *fund_transfer_info, XTPRI *error_info, uint64_t session_id);
extern "C" void TraderSpiStub_Rust_OnQueryETF(void *rust_object, XTPQueryETFBaseRsp *etf_info, XTPRI *error_info, int request_id, bool is_last, uint64_t session_id);
extern "C" void TraderSpiStub_Rust_OnQueryETFBasket(void *rust_object, XTPQueryETFComponentRsp *etf_component_info, XTPRI *error_info, int request_id, bool is_last, uint64_t session_id);
extern "C" void TraderSpiStub_Rust_OnQueryIPOInfoList(void *rust_object, XTPQueryIPOTickerRsp *ipo_info, XTPRI *error_info, int request_id, bool is_last, uint64_t session_id);
extern "C" void TraderSpiStub_Rust_OnQueryIPOQuotaInfo(void *rust_object, XTPQueryIPOQuotaRsp *quota_info, XTPRI *error_info, int request_id, bool is_last, uint64_t session_id);
extern "C" void TraderSpiStub_Rust_OnQueryOptionAuctionInfo(void *rust_object, XTPQueryOptionAuctionInfoRsp *option_info, XTPRI *error_info, int request_id, bool is_last, uint64_t session_id);
extern "C" void TraderSpiStub_Rust_Destructor(void *rust_object);

#endif