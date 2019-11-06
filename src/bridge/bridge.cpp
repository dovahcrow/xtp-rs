#include "bridge.hpp"

// *********************** QuoteApi **************************
extern "C" XTP::API::QuoteApi *CreateQuoteApi(uint8_t client_id, const char *save_file_path, XTP_LOG_LEVEL log_level)
{
    return XTP::API::QuoteApi::CreateQuoteApi(client_id, save_file_path, log_level);
}
extern "C" void QuoteApi_Release(XTP::API::QuoteApi *self)
{
    auto s = static_cast<XTP::API::QuoteApi *>(self);
    return s->Release();
}
extern "C" const char *QuoteApi_GetTradingDay(XTP::API::QuoteApi *self)
{
    auto s = static_cast<XTP::API::QuoteApi *>(self);
    return s->GetTradingDay();
}
extern "C" const char *QuoteApi_GetApiVersion(XTP::API::QuoteApi *self)
{
    auto s = static_cast<XTP::API::QuoteApi *>(self);
    return s->GetApiVersion();
}
extern "C" XTPRI *QuoteApi_GetApiLastError(XTP::API::QuoteApi *self)
{
    auto s = static_cast<XTP::API::QuoteApi *>(self);
    return s->GetApiLastError();
}
extern "C" void QuoteApi_SetUDPBufferSize(XTP::API::QuoteApi *self, uint32_t buff_size)
{
    auto s = static_cast<XTP::API::QuoteApi *>(self);
    return s->SetUDPBufferSize(buff_size);
}
extern "C" void QuoteApi_RegisterSpi(XTP::API::QuoteApi *self, XTP::API::QuoteSpi *spi)
{
    auto s = static_cast<XTP::API::QuoteApi *>(self);
    return s->RegisterSpi(spi);
}
extern "C" void QuoteApi_SetHeartBeatInterval(XTP::API::QuoteApi *self, uint32_t interval)
{
    auto s = static_cast<XTP::API::QuoteApi *>(self);
    return s->SetHeartBeatInterval(interval);
}
extern "C" int QuoteApi_SubscribeMarketData(XTP::API::QuoteApi *self, char *ticker[], int count, XTP_EXCHANGE_TYPE exchange_id)
{
    auto s = static_cast<XTP::API::QuoteApi *>(self);
    return s->SubscribeMarketData(ticker, count, exchange_id);
}
extern "C" int QuoteApi_UnSubscribeMarketData(XTP::API::QuoteApi *self, char *ticker[], int count, XTP_EXCHANGE_TYPE exchange_id)
{
    auto s = static_cast<XTP::API::QuoteApi *>(self);
    return s->UnSubscribeMarketData(ticker, count, exchange_id);
}
extern "C" int QuoteApi_SubscribeOrderBook(XTP::API::QuoteApi *self, char *ticker[], int count, XTP_EXCHANGE_TYPE exchange_id)
{
    auto s = static_cast<XTP::API::QuoteApi *>(self);
    return s->SubscribeOrderBook(ticker, count, exchange_id);
}
extern "C" int QuoteApi_UnSubscribeOrderBook(XTP::API::QuoteApi *self, char *ticker[], int count, XTP_EXCHANGE_TYPE exchange_id)
{
    auto s = static_cast<XTP::API::QuoteApi *>(self);
    return s->UnSubscribeOrderBook(ticker, count, exchange_id);
}
extern "C" int QuoteApi_SubscribeTickByTick(XTP::API::QuoteApi *self, char *ticker[], int count, XTP_EXCHANGE_TYPE exchange_id)
{
    auto s = static_cast<XTP::API::QuoteApi *>(self);
    return s->SubscribeTickByTick(ticker, count, exchange_id);
}
extern "C" int QuoteApi_UnSubscribeTickByTick(XTP::API::QuoteApi *self, char *ticker[], int count, XTP_EXCHANGE_TYPE exchange_id)
{
    auto s = static_cast<XTP::API::QuoteApi *>(self);
    return s->UnSubscribeTickByTick(ticker, count, exchange_id);
}
extern "C" int QuoteApi_SubscribeAllMarketData(XTP::API::QuoteApi *self, XTP_EXCHANGE_TYPE exchange_id)
{
    auto s = static_cast<XTP::API::QuoteApi *>(self);
    return s->SubscribeAllMarketData(exchange_id);
}
extern "C" int QuoteApi_UnSubscribeAllMarketData(XTP::API::QuoteApi *self, XTP_EXCHANGE_TYPE exchange_id)
{
    auto s = static_cast<XTP::API::QuoteApi *>(self);
    return s->UnSubscribeAllMarketData(exchange_id);
}
extern "C" int QuoteApi_SubscribeAllOrderBook(XTP::API::QuoteApi *self, XTP_EXCHANGE_TYPE exchange_id)
{
    auto s = static_cast<XTP::API::QuoteApi *>(self);
    return s->SubscribeAllOrderBook(exchange_id);
}
extern "C" int QuoteApi_UnSubscribeAllOrderBook(XTP::API::QuoteApi *self, XTP_EXCHANGE_TYPE exchange_id)
{
    auto s = static_cast<XTP::API::QuoteApi *>(self);
    return s->UnSubscribeAllOrderBook(exchange_id);
}
extern "C" int QuoteApi_SubscribeAllTickByTick(XTP::API::QuoteApi *self, XTP_EXCHANGE_TYPE exchange_id)
{
    auto s = static_cast<XTP::API::QuoteApi *>(self);
    return s->SubscribeAllTickByTick(exchange_id);
}
extern "C" int QuoteApi_UnSubscribeAllTickByTick(XTP::API::QuoteApi *self, XTP_EXCHANGE_TYPE exchange_id)
{
    auto s = static_cast<XTP::API::QuoteApi *>(self);
    return s->UnSubscribeAllTickByTick(exchange_id);
}
extern "C" int QuoteApi_Login(XTP::API::QuoteApi *self, const char *ip, int port, const char *user, const char *password, XTP_PROTOCOL_TYPE sock_type)
{
    auto s = static_cast<XTP::API::QuoteApi *>(self);
    return s->Login(ip, port, user, password, sock_type);
}
extern "C" int QuoteApi_Logout(XTP::API::QuoteApi *self)
{
    auto s = static_cast<XTP::API::QuoteApi *>(self);
    return s->Logout();
}
extern "C" int QuoteApi_QueryAllTickers(XTP::API::QuoteApi *self, XTP_EXCHANGE_TYPE exchange_id)
{
    auto s = static_cast<XTP::API::QuoteApi *>(self);
    return s->QueryAllTickers(exchange_id);
}
extern "C" int QuoteApi_QueryTickersPriceInfo(XTP::API::QuoteApi *self, char *ticker[], int count, XTP_EXCHANGE_TYPE exchange_id)
{
    auto s = static_cast<XTP::API::QuoteApi *>(self);
    return s->QueryTickersPriceInfo(ticker, count, exchange_id);
}
extern "C" int QuoteApi_QueryAllTickersPriceInfo(XTP::API::QuoteApi *self)
{
    auto s = static_cast<XTP::API::QuoteApi *>(self);
    return s->QueryAllTickersPriceInfo();
}
extern "C" int QuoteApi_SubscribeAllOptionMarketData(XTP::API::QuoteApi *self, XTP_EXCHANGE_TYPE exchange_id)
{
    auto s = static_cast<XTP::API::QuoteApi *>(self);
    return s->SubscribeAllOptionMarketData(exchange_id);
}
extern "C" int QuoteApi_UnSubscribeAllOptionMarketData(XTP::API::QuoteApi *self, XTP_EXCHANGE_TYPE exchange_id)
{
    auto s = static_cast<XTP::API::QuoteApi *>(self);
    return s->UnSubscribeAllOptionMarketData(exchange_id);
}
extern "C" int QuoteApi_SubscribeAllOptionOrderBook(XTP::API::QuoteApi *self, XTP_EXCHANGE_TYPE exchange_id)
{
    auto s = static_cast<XTP::API::QuoteApi *>(self);
    return s->SubscribeAllOptionOrderBook(exchange_id);
}
extern "C" int QuoteApi_UnSubscribeAllOptionOrderBook(XTP::API::QuoteApi *self, XTP_EXCHANGE_TYPE exchange_id)
{
    auto s = static_cast<XTP::API::QuoteApi *>(self);
    return s->UnSubscribeAllOptionOrderBook(exchange_id);
}
extern "C" int QuoteApi_SubscribeAllOptionTickByTick(XTP::API::QuoteApi *self, XTP_EXCHANGE_TYPE exchange_id)
{
    auto s = static_cast<XTP::API::QuoteApi *>(self);
    return s->SubscribeAllOptionTickByTick(exchange_id);
}
extern "C" int QuoteApi_UnSubscribeAllOptionTickByTick(XTP::API::QuoteApi *self, XTP_EXCHANGE_TYPE exchange_id)
{
    auto s = static_cast<XTP::API::QuoteApi *>(self);
    return s->UnSubscribeAllOptionTickByTick(exchange_id);
}

// *********************** TraderApi **************************
extern "C" XTP::API::TraderApi *CreateTraderApi(uint8_t client_id, const char *save_file_path, XTP_LOG_LEVEL log_level)
{
    return XTP::API::TraderApi::CreateTraderApi(client_id, save_file_path, log_level);
}
extern "C" void TraderApi_Release(XTP::API::TraderApi *self)
{
    auto s = static_cast<XTP::API::TraderApi *>(self);
    return s->Release();
}
extern "C" const char *GetTradingDay(XTP::API::TraderApi *self)
{
    auto s = static_cast<XTP::API::TraderApi *>(self);
    return s->GetTradingDay();
}
extern "C" void TraderApi_RegisterSpi(XTP::API::TraderApi *self, XTP::API::TraderSpi *spi)
{
    auto s = static_cast<XTP::API::TraderApi *>(self);
    return s->RegisterSpi(spi);
}
extern "C" XTPRI *GetApiLastError(XTP::API::TraderApi *self)
{
    auto s = static_cast<XTP::API::TraderApi *>(self);
    return s->GetApiLastError();
}
extern "C" const char *TraderApi_GetApiVersion(XTP::API::TraderApi *self)
{
    auto s = static_cast<XTP::API::TraderApi *>(self);
    return s->GetApiVersion();
}
extern "C" uint8_t TraderApi_GetClientIDByXTPID(XTP::API::TraderApi *self, uint64_t order_xtp_id)
{
    auto s = static_cast<XTP::API::TraderApi *>(self);
    return s->GetClientIDByXTPID(order_xtp_id);
}
extern "C" const char *TraderApi_GetAccountByXTPID(XTP::API::TraderApi *self, uint64_t order_xtp_id)
{
    auto s = static_cast<XTP::API::TraderApi *>(self);
    return s->GetAccountByXTPID(order_xtp_id);
}
extern "C" void TraderApi_SubscribePublicTopic(XTP::API::TraderApi *self, XTP_TE_RESUME_TYPE resume_type)
{
    auto s = static_cast<XTP::API::TraderApi *>(self);
    return s->SubscribePublicTopic(resume_type);
}
extern "C" void TraderApi_SetSoftwareVersion(XTP::API::TraderApi *self, const char *version)
{
    auto s = static_cast<XTP::API::TraderApi *>(self);
    return s->SetSoftwareVersion(version);
}
extern "C" void TraderApi_SetSoftwareKey(XTP::API::TraderApi *self, const char *key)
{
    auto s = static_cast<XTP::API::TraderApi *>(self);
    return s->SetSoftwareKey(key);
}
extern "C" void TraderApi_SetHeartBeatInterval(XTP::API::TraderApi *self, uint32_t interval)
{
    auto s = static_cast<XTP::API::TraderApi *>(self);
    return s->SetHeartBeatInterval(interval);
}
extern "C" uint64_t TraderApi_Login(XTP::API::TraderApi *self, const char *ip, int port, const char *user, const char *password, XTP_PROTOCOL_TYPE sock_type)
{
    auto s = static_cast<XTP::API::TraderApi *>(self);
    return s->Login(ip, port, user, password, sock_type);
}
extern "C" int TraderApi_Logout(XTP::API::TraderApi *self, uint64_t session_id)
{
    auto s = static_cast<XTP::API::TraderApi *>(self);
    return s->Logout(session_id);
}
extern "C" bool TraderApi_IsServerRestart(XTP::API::TraderApi *self, uint64_t session_id)
{
    auto s = static_cast<XTP::API::TraderApi *>(self);
    return s->IsServerRestart(session_id);
}
extern "C" uint64_t TraderApi_InsertOrder(XTP::API::TraderApi *self, XTPOrderInsertInfo *order, uint64_t session_id)
{
    auto s = static_cast<XTP::API::TraderApi *>(self);
    return s->InsertOrder(order, session_id);
}
extern "C" uint64_t TraderApi_CancelOrder(XTP::API::TraderApi *self, const uint64_t order_xtp_id, uint64_t session_id)
{
    auto s = static_cast<XTP::API::TraderApi *>(self);
    return s->CancelOrder(order_xtp_id, session_id);
}
extern "C" int TraderApi_QueryOrderByXTPID(XTP::API::TraderApi *self, const uint64_t order_xtp_id, uint64_t session_id, int request_id)
{
    auto s = static_cast<XTP::API::TraderApi *>(self);
    return s->QueryOrderByXTPID(order_xtp_id, session_id, request_id);
}
extern "C" int TraderApi_QueryOrders(XTP::API::TraderApi *self, const XTPQueryOrderReq *query_param, uint64_t session_id, int request_id)
{
    auto s = static_cast<XTP::API::TraderApi *>(self);
    return s->QueryOrders(query_param, session_id, request_id);
}
extern "C" int TraderApi_QueryOrdersByPage(XTP::API::TraderApi *self, const XTPQueryOrderByPageReq *query_param, uint64_t session_id, int request_id)
{
    auto s = static_cast<XTP::API::TraderApi *>(self);
    return s->QueryOrdersByPage(query_param, session_id, request_id);
}
extern "C" int TraderApi_QueryTradesByXTPID(XTP::API::TraderApi *self, const uint64_t order_xtp_id, uint64_t session_id, int request_id)
{
    auto s = static_cast<XTP::API::TraderApi *>(self);
    return s->QueryTradesByXTPID(order_xtp_id, session_id, request_id);
}
extern "C" int TraderApi_QueryTrades(XTP::API::TraderApi *self, XTPQueryTraderReq *query_param, uint64_t session_id, int request_id)
{
    auto s = static_cast<XTP::API::TraderApi *>(self);
    return s->QueryTrades(query_param, session_id, request_id);
}
extern "C" int TraderApi_QueryTradesByPage(XTP::API::TraderApi *self, const XTPQueryTraderByPageReq *query_param, uint64_t session_id, int request_id)
{
    auto s = static_cast<XTP::API::TraderApi *>(self);
    return s->QueryTradesByPage(query_param, session_id, request_id);
}
extern "C" int TraderApi_QueryPosition(XTP::API::TraderApi *self, const char *ticker, uint64_t session_id, int request_id)
{
    auto s = static_cast<XTP::API::TraderApi *>(self);
    return s->QueryPosition(ticker, session_id, request_id);
}
extern "C" int TraderApi_QueryAsset(XTP::API::TraderApi *self, uint64_t session_id, int request_id)
{
    auto s = static_cast<XTP::API::TraderApi *>(self);
    return s->QueryAsset(session_id, request_id);
}
extern "C" int TraderApi_QueryStructuredFund(XTP::API::TraderApi *self, XTPQueryStructuredFundInfoReq *query_param, uint64_t session_id, int request_id)
{
    auto s = static_cast<XTP::API::TraderApi *>(self);
    return s->QueryStructuredFund(query_param, session_id, request_id);
}
extern "C" uint64_t TraderApi_FundTransfer(XTP::API::TraderApi *self, XTPFundTransferReq *fund_transfer, uint64_t session_id)
{
    auto s = static_cast<XTP::API::TraderApi *>(self);
    return s->FundTransfer(fund_transfer, session_id);
}
extern "C" int TraderApi_QueryFundTransfer(XTP::API::TraderApi *self, XTPQueryFundTransferLogReq *query_param, uint64_t session_id, int request_id)
{
    auto s = static_cast<XTP::API::TraderApi *>(self);
    return s->QueryFundTransfer(query_param, session_id, request_id);
}
extern "C" int TraderApi_QueryETF(XTP::API::TraderApi *self, XTPQueryETFBaseReq *query_param, uint64_t session_id, int request_id)
{
    auto s = static_cast<XTP::API::TraderApi *>(self);
    return s->QueryETF(query_param, session_id, request_id);
}
extern "C" int TraderApi_QueryETFTickerBasket(XTP::API::TraderApi *self, XTPQueryETFComponentReq *query_param, uint64_t session_id, int request_id)
{
    auto s = static_cast<XTP::API::TraderApi *>(self);
    return s->QueryETFTickerBasket(query_param, session_id, request_id);
}
extern "C" int TraderApi_QueryIPOInfoList(XTP::API::TraderApi *self, uint64_t session_id, int request_id)
{
    auto s = static_cast<XTP::API::TraderApi *>(self);
    return s->QueryIPOInfoList(session_id, request_id);
}
extern "C" int TraderApi_QueryIPOQuotaInfo(XTP::API::TraderApi *self, uint64_t session_id, int request_id)
{
    auto s = static_cast<XTP::API::TraderApi *>(self);
    return s->QueryIPOQuotaInfo(session_id, request_id);
}
extern "C" int TraderApi_QueryOptionAuctionInfo(XTP::API::TraderApi *self, XTPQueryOptionAuctionInfoReq *query_param, uint64_t session_id, int request_id)
{
    auto s = static_cast<XTP::API::TraderApi *>(self);
    return s->QueryOptionAuctionInfo(query_param, session_id, request_id);
}

// *********************** QuoteSpiStub **************************
QuoteSpiStub::QuoteSpiStub(void *rust_object) : rust_object(rust_object) {}

void QuoteSpiStub::OnDisconnected(int reason)
{
    return QuoteSpiStub_Rust_OnDisconnected(this->rust_object, reason);
}

void QuoteSpiStub::OnError(XTPRI *error_info)
{
    return QuoteSpiStub_Rust_OnError(this->rust_object, error_info);
}

void QuoteSpiStub::OnSubMarketData(XTPST *ticker, XTPRI *error_info, bool is_last)
{
    return QuoteSpiStub_Rust_OnSubMarketData(this->rust_object, ticker, error_info, is_last);
}

void QuoteSpiStub::OnUnSubMarketData(XTPST *ticker, XTPRI *error_info, bool is_last)
{
    return QuoteSpiStub_Rust_OnSubMarketData(this->rust_object, ticker, error_info, is_last);
}

void QuoteSpiStub::OnDepthMarketData(XTPMD *market_data, int64_t bid1_qty[], int32_t bid1_count, int32_t max_bid1_count, int64_t ask1_qty[], int32_t ask1_count, int32_t max_ask1_count)
{
    return QuoteSpiStub_Rust_OnDepthMarketData(this->rust_object, market_data, bid1_qty, bid1_count, max_bid1_count, ask1_qty, ask1_count, max_ask1_count);
}
void QuoteSpiStub::OnSubOrderBook(XTPST *ticker, XTPRI *error_info, bool is_last)
{
    QuoteSpiStub_Rust_OnSubOrderBook(this->rust_object, ticker, error_info, is_last);
}
void QuoteSpiStub::OnUnSubOrderBook(XTPST *ticker, XTPRI *error_info, bool is_last)
{
    return QuoteSpiStub_Rust_OnUnSubOrderBook(this->rust_object, ticker, error_info, is_last);
}
void QuoteSpiStub::OnOrderBook(XTPOB *order_book)
{
    return QuoteSpiStub_Rust_OnOrderBook(this->rust_object, order_book);
}
void QuoteSpiStub::OnSubTickByTick(XTPST *ticker, XTPRI *error_info, bool is_last)
{
    return QuoteSpiStub_Rust_OnSubTickByTick(this->rust_object, ticker, error_info, is_last);
}
void QuoteSpiStub::OnUnSubTickByTick(XTPST *ticker, XTPRI *error_info, bool is_last)
{
    return QuoteSpiStub_Rust_OnUnSubTickByTick(this->rust_object, ticker, error_info, is_last);
}
void QuoteSpiStub::OnTickByTick(XTPTBT *tbt_data)
{
    return QuoteSpiStub_Rust_OnTickByTick(this->rust_object, tbt_data);
}
void QuoteSpiStub::OnSubscribeAllMarketData(XTP_EXCHANGE_TYPE exchange_id, XTPRI *error_info)
{
    return QuoteSpiStub_Rust_OnSubscribeAllMarketData(this->rust_object, exchange_id, error_info);
}
void QuoteSpiStub::OnUnSubscribeAllMarketData(XTP_EXCHANGE_TYPE exchange_id, XTPRI *error_info)
{
    return QuoteSpiStub_Rust_OnUnSubscribeAllMarketData(this->rust_object, exchange_id, error_info);
}
void QuoteSpiStub::OnSubscribeAllOrderBook(XTP_EXCHANGE_TYPE exchange_id, XTPRI *error_info)
{
    return QuoteSpiStub_Rust_OnSubscribeAllOrderBook(this->rust_object, exchange_id, error_info);
}
void QuoteSpiStub::OnUnSubscribeAllOrderBook(XTP_EXCHANGE_TYPE exchange_id, XTPRI *error_info)
{
    return QuoteSpiStub_Rust_OnUnSubscribeAllOrderBook(this->rust_object, exchange_id, error_info);
}
void QuoteSpiStub::OnSubscribeAllTickByTick(XTP_EXCHANGE_TYPE exchange_id, XTPRI *error_info)
{
    return QuoteSpiStub_Rust_OnSubscribeAllTickByTick(this->rust_object, exchange_id, error_info);
}
void QuoteSpiStub::OnUnSubscribeAllTickByTick(XTP_EXCHANGE_TYPE exchange_id, XTPRI *error_info)
{
    return QuoteSpiStub_Rust_OnUnSubscribeAllTickByTick(this->rust_object, exchange_id, error_info);
}
void QuoteSpiStub::OnQueryAllTickers(XTPQSI *ticker_info, XTPRI *error_info, bool is_last)
{
    return QuoteSpiStub_Rust_OnQueryAllTickers(this->rust_object, ticker_info, error_info, is_last);
}
void QuoteSpiStub::OnQueryTickersPriceInfo(XTPTPI *ticker_info, XTPRI *error_info, bool is_last)
{
    return QuoteSpiStub_Rust_OnQueryTickersPriceInfo(this->rust_object, ticker_info, error_info, is_last);
}
void QuoteSpiStub::OnSubscribeAllOptionMarketData(XTP_EXCHANGE_TYPE exchange_id, XTPRI *error_info)
{
    return QuoteSpiStub_Rust_OnSubscribeAllOptionMarketData(this->rust_object, exchange_id, error_info);
}
void QuoteSpiStub::OnUnSubscribeAllOptionMarketData(XTP_EXCHANGE_TYPE exchange_id, XTPRI *error_info)
{
    return QuoteSpiStub_Rust_OnUnSubscribeAllOptionMarketData(this->rust_object, exchange_id, error_info);
}
void QuoteSpiStub::OnSubscribeAllOptionOrderBook(XTP_EXCHANGE_TYPE exchange_id, XTPRI *error_info)
{
    return QuoteSpiStub_Rust_OnSubscribeAllOptionOrderBook(this->rust_object, exchange_id, error_info);
}
void QuoteSpiStub::OnUnSubscribeAllOptionOrderBook(XTP_EXCHANGE_TYPE exchange_id, XTPRI *error_info)
{
    return QuoteSpiStub_Rust_OnUnSubscribeAllOptionOrderBook(this->rust_object, exchange_id, error_info);
}
void QuoteSpiStub::OnSubscribeAllOptionTickByTick(XTP_EXCHANGE_TYPE exchange_id, XTPRI *error_info)
{
    return QuoteSpiStub_Rust_OnSubscribeAllOptionTickByTick(this->rust_object, exchange_id, error_info);
}
void QuoteSpiStub::OnUnSubscribeAllOptionTickByTick(XTP_EXCHANGE_TYPE exchange_id, XTPRI *error_info)
{
    return QuoteSpiStub_Rust_OnUnSubscribeAllOptionTickByTick(this->rust_object, exchange_id, error_info);
}
QuoteSpiStub::~QuoteSpiStub()
{
    QuoteSpiStub_Rust_Destructor(this->rust_object);
}

// *********************** TradeSpiStub **************************
TraderSpiStub::TraderSpiStub(void *rust_object) : rust_object(rust_object) {}
void TraderSpiStub::OnDisconnected(uint64_t session_id, int reason)
{
    return TraderSpiStub_Rust_OnDisconnected(this->rust_object, session_id, reason);
}
void TraderSpiStub::OnError(XTPRI *error_info)
{
    return TraderSpiStub_Rust_OnError(this->rust_object, error_info);
}
void TraderSpiStub::OnOrderEvent(XTPOrderInfo *order_info, XTPRI *error_info, uint64_t session_id)
{
    return TraderSpiStub_Rust_OnOrderEvent(this->rust_object, order_info, error_info, session_id);
}
void TraderSpiStub::OnTradeEvent(XTPTradeReport *trade_info, uint64_t session_id)
{
    return TraderSpiStub_Rust_OnTradeEvent(this->rust_object, trade_info, session_id);
}
void TraderSpiStub::OnCancelOrderError(XTPOrderCancelInfo *cancel_info, XTPRI *error_info, uint64_t session_id)
{
    return TraderSpiStub_Rust_OnCancelOrderError(this->rust_object, cancel_info, error_info, session_id);
}
void TraderSpiStub::OnQueryOrder(XTPQueryOrderRsp *order_info, XTPRI *error_info, int request_id, bool is_last, uint64_t session_id)
{
    return TraderSpiStub_Rust_OnQueryOrder(this->rust_object, order_info, error_info, request_id, is_last, session_id);
}
void TraderSpiStub::OnQueryOrderByPage(XTPQueryOrderRsp *order_info, int64_t req_count, int64_t order_sequence, int64_t query_reference, int request_id, bool is_last, uint64_t session_id)
{
    return TraderSpiStub_Rust_OnQueryOrderByPage(this->rust_object, order_info, req_count, order_sequence, query_reference, request_id, is_last, session_id);
}
void TraderSpiStub::OnQueryTrade(XTPQueryTradeRsp *trade_info, XTPRI *error_info, int request_id, bool is_last, uint64_t session_id)
{
    return TraderSpiStub_Rust_OnQueryTrade(this->rust_object, trade_info, error_info, request_id, is_last, session_id);
}
void TraderSpiStub::OnQueryTradeByPage(XTPQueryTradeRsp *trade_info, int64_t req_count, int64_t trade_sequence, int64_t query_reference, int request_id, bool is_last, uint64_t session_id)
{
    return TraderSpiStub_Rust_OnQueryTradeByPage(this->rust_object, trade_info, req_count, trade_sequence, query_reference, request_id, is_last, session_id);
}
void TraderSpiStub::OnQueryPosition(XTPQueryStkPositionRsp *position, XTPRI *error_info, int request_id, bool is_last, uint64_t session_id)
{
    return TraderSpiStub_Rust_OnQueryPosition(this->rust_object, position, error_info, request_id, is_last, session_id);
}
void TraderSpiStub::OnQueryAsset(XTPQueryAssetRsp *asset, XTPRI *error_info, int request_id, bool is_last, uint64_t session_id)
{
    return TraderSpiStub_Rust_OnQueryAsset(this->rust_object, asset, error_info, request_id, is_last, session_id);
}
void TraderSpiStub::OnQueryStructuredFund(XTPStructuredFundInfo *fund_info, XTPRI *error_info, int request_id, bool is_last, uint64_t session_id)
{
    return TraderSpiStub_Rust_OnQueryStructuredFund(this->rust_object, fund_info, error_info, request_id, is_last, session_id);
}
void TraderSpiStub::OnQueryFundTransfer(XTPFundTransferNotice *fund_transfer_info, XTPRI *error_info, int request_id, bool is_last, uint64_t session_id)
{
    return TraderSpiStub_Rust_OnQueryFundTransfer(this->rust_object, fund_transfer_info, error_info, request_id, is_last, session_id);
}
void TraderSpiStub::OnFundTransfer(XTPFundTransferNotice *fund_transfer_info, XTPRI *error_info, uint64_t session_id)
{
    return TraderSpiStub_Rust_OnFundTransfer(this->rust_object, fund_transfer_info, error_info, session_id);
}
void TraderSpiStub::OnQueryETF(XTPQueryETFBaseRsp *etf_info, XTPRI *error_info, int request_id, bool is_last, uint64_t session_id)
{
    return TraderSpiStub_Rust_OnQueryETF(this->rust_object, etf_info, error_info, request_id, is_last, session_id);
}
void TraderSpiStub::OnQueryETFBasket(XTPQueryETFComponentRsp *etf_component_info, XTPRI *error_info, int request_id, bool is_last, uint64_t session_id)
{
    return TraderSpiStub_Rust_OnQueryETFBasket(this->rust_object, etf_component_info, error_info, request_id, is_last, session_id);
}
void TraderSpiStub::OnQueryIPOInfoList(XTPQueryIPOTickerRsp *ipo_info, XTPRI *error_info, int request_id, bool is_last, uint64_t session_id)
{
    return TraderSpiStub_Rust_OnQueryIPOInfoList(this->rust_object, ipo_info, error_info, request_id, is_last, session_id);
}
void TraderSpiStub::OnQueryIPOQuotaInfo(XTPQueryIPOQuotaRsp *quota_info, XTPRI *error_info, int request_id, bool is_last, uint64_t session_id)
{
    return TraderSpiStub_Rust_OnQueryIPOQuotaInfo(this->rust_object, quota_info, error_info, request_id, is_last, session_id);
}
void TraderSpiStub::OnQueryOptionAuctionInfo(XTPQueryOptionAuctionInfoRsp *option_info, XTPRI *error_info, int request_id, bool is_last, uint64_t session_id)
{
    return TraderSpiStub_Rust_OnQueryOptionAuctionInfo(this->rust_object, option_info, error_info, request_id, is_last, session_id);
}
TraderSpiStub::~TraderSpiStub()
{
    TraderSpiStub_Rust_Destructor(this->rust_object);
}