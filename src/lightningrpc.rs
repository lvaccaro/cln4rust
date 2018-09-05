//! High-level interface to c-lightning RPC
use serde::de::DeserializeOwned;
use serde::Serialize;
use strason::Json;

use client;
use error::Error;
use requests;
use responses;

/// Structure providing a high-level interface to the c-lightning daemon RPC
pub struct LightningRPC {
    client: client::Client,
}

impl LightningRPC {
    /// Create a new connection from a UNIX socket path
    pub fn new(sockname: String) -> LightningRPC {
        LightningRPC {
            client: client::Client::new(sockname),
        }
    }

    /// Generic call function for RPC calls
    fn call<T: Serialize, U: DeserializeOwned>(
        &mut self,
        method: &str,
        input: T,
    ) -> Result<U, Error> {
        let params = Json::from_serialize(input)?;
        let request = self.client.build_request(method.to_string(), params);
        self.client
            .send_request(&request)
            .and_then(|res| res.into_result::<U>())
    }

    /// Show information about this node
    pub fn getinfo(&mut self) -> Result<responses::GetInfo, Error> {
        self.call("getinfo", requests::GetInfo {})
    }

    /// Supply feerate estimates manually.
    pub fn feerates(&mut self, style: &str) -> Result<responses::FeeRates, Error> {
        self.call(
            "feerates",
            requests::FeeRates {
                style: style.to_string(),
            },
        )
    }

    /// Show current peers, if {level} is set, include {log}s"
    pub fn listpeers(
        &mut self,
        id: Option<String>,
        level: Option<String>,
    ) -> Result<responses::ListPeers, Error> {
        self.call("listpeers", requests::ListPeers { id, level })
    }

    /// Show invoice {label} (or all, if no {label))
    pub fn listinvoices(
        &mut self,
        label: Option<String>,
    ) -> Result<responses::ListInvoices, Error> {
        self.call("listinvoices", requests::ListInvoices { label })
    }

    /// Create an invoice for {msatoshi} with {label} and {description} with
    /// optional {expiry} seconds (default 1 hour)
    pub fn invoice(
        &mut self,
        msatoshi: i64,
        label: String,
        description: String,
        expiry: Option<i64>,
    ) -> Result<responses::Invoice, Error> {
        self.call(
            "invoice",
            requests::Invoice {
                msatoshi,
                label,
                description,
                expiry,
            },
        )
    }

    /// Create an invoice for {msatoshi} with {label} and {description} with
    /// optional {expiry} seconds (default 1 hour)
    pub fn delinvoice(
        &mut self,
        label: String,
        status: String,
    ) -> Result<responses::DelInvoice, Error> {
        self.call("delinvoice", requests::DelInvoice { label, status })
    }

    /// Send payment specified by {bolt11} with optional {msatoshi} (if and only if {bolt11} does
    /// not have amount)
    /// {description} (required if {bolt11} uses description hash)
    /// {riskfactor} (default 1.0)
    /// {maxfeepercent} (default 0.5) the maximum acceptable fee as a percentage (e.g. 0.5 =>
    /// 0.5%),
    /// {exemptfee} (default 5000 msat) disables the maxfeepercent check for fees below the
    /// threshold,
    /// {retry_for} (default 60) the integer number of seconds before we stop retrying, and
    /// {maxdelay} (default 500) the maximum number of blocks we allow the funds to possibly get
    /// locked
    pub fn pay(
        &mut self,
        bolt11: String,
        msatoshi: Option<i64>,
        description: Option<String>,
        riskfactor: Option<f64>,
        maxfeepercent: Option<f64>,
        exemptfee: Option<i64>,
        retry_for: Option<i64>,
        maxdelay: Option<i64>,
    ) -> Result<responses::Pay, Error> {
        self.call(
            "pay",
            requests::Pay {
                bolt11,
                msatoshi,
                description,
                riskfactor,
                maxfeepercent,
                exemptfee,
                retry_for,
                maxdelay,
            },
        )
    }

    /// Show outgoing payments
    pub fn listpayments(
        &mut self,
        bolt11: Option<String>,
        payment_hash: Option<String>,
    ) -> Result<responses::ListPayments, Error> {
        self.call(
            "listpayments",
            requests::ListPayments {
                bolt11,
                payment_hash,
            },
        )
    }

    /// Decode {bolt11}, using {description} if necessary
    pub fn decodepay(
        &mut self,
        bolt11: String,
        description: Option<String>,
    ) -> Result<responses::DecodePay, Error> {
        self.call(
            "decodepay",
            requests::DecodePay {
                bolt11,
                description,
            },
        )
    }

    /// Show route to {id} for {msatoshi}, using {riskfactor} and optional {cltv} (default 9). If
    /// specified search from {fromid} otherwise use this node as source. Randomize the route with
    /// up to {fuzzpercent} (0.0 -> 100.0, default 5.0) using {seed} as an arbitrary-size string
    /// seed.
    pub fn getroute(
        &mut self,
        id: String,
        msatoshi: i64,
        riskfactor: f64,
        cltv: Option<i64>,
        fromid: Option<String>,
        fuzzpercent: Option<f64>,
        seed: Option<String>,
    ) -> Result<responses::GetRoute, Error> {
        self.call(
            "getroute",
            requests::GetRoute {
                id,
                msatoshi,
                riskfactor,
                cltv,
                fromid,
                fuzzpercent,
                seed,
            },
        )
    }

    /// Connect to {id} at {host} (which can end in ':port' if not default). {id} can also be of
    /// the form id@host
    pub fn connect(
        &mut self,
        id: String,
        host: Option<String>,
    ) -> Result<responses::Connect, Error> {
        self.call("connect", requests::Connect { id, host })
    }

    /// Disconnect from peer with {peer_id}
    pub fn disconnect(&mut self, id: String) -> Result<responses::Disconnect, Error> {
        self.call("disconnect", requests::Disconnect { id })
    }

    /// Shut down the lightningd process
    pub fn stop(&mut self) -> Result<responses::Stop, Error> {
        self.call("stop", requests::Stop {})
    }
}
