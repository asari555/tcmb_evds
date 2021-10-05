//! Easy and ready access to the database of The Central Bank of The Republic of Turkey (CBRT).
//!
//! This crate provides two main separate mechanisms for acquiring data from the database:
//!
//! - [`evds_basic`](crate::evds_basic) includes functions making most of the web service operations except
//! currency operations with frequency formulas. 
//! - [`evds_currency`](crate::evds_currency) includes structure-based and implemented methods that make totally 
//! currency operations.
//!
//! Useful functions of [`evds_basic`](crate::evds_basic) and [`evds_currency`](crate::evds_currency) 
//! require a number of common elements checking the validity of given inputs and creating appropriate requests formats 
//! for the database. These mentioned elements are included in [`common`], [`date`] and [`error`] modules.
//!
//! [`ApiKey`](struct@crate::common::ApiKey), which is a structure in [`common`], gives validity to all functions 
//! playing role to make requests for EVDS web services. Therefore, users need to utilize their keys to work with EVDS.
//!
//! The functions making most of EVDS web service operations are given inside the [`evds_basic`](crate::evds_basic) 
//! module except currency values with advanced features. In addition, [`evds_currency`](crate::evds_currency) has two 
//! structures containing required currency requesting elements. It is because the number of the elements are more than 
//! enough for using as parameter that these two struct supplies self methods to make only *currency* operations 
//! of EVDS web services.
//!
//! This crate provides `async_mode` and `sync_mode` as a feature. `async_mode` and `sync_mode` features
//! should be used with an async and a sync programming respectively. Furthermore, the `async_mode` feature is enabled 
//! by default. One of these two features should be used to work with this crate.
//!
//! # Install
//!
//! Please, add **appropriate one** of the blow codes to your **Cargo.toml** to install the crate.
//! 
//!
//! For *async_mode*, please add.
//!
//! ``` toml
//! [dependencies]
//! tcmb_evds = "0.1"
//! ```
//!
//! For *sync_mode*, please add.
//!
//! ``` toml
//! [dependencies]
//! tcmb_evds = {version = "0.1", default-features = false, features = ["sync_mode"]}
//! ```
//!
//! # Usage
//!
//! For more and other function implementations and details, please go to [`evds_basic`](crate::evds_basic) module 
//! stage.
//!
//! ```
//! # use std::error::Error;
//! # use tcmb_evds::error::ReturnError;
//!     use tcmb_evds::*;
//! #
//! # fn main() -> Result<(), Box<dyn Error>> {
//! 
//! 
//!     // assigning required arguments.
//!     // another data series = "TP.DK.USD.A-TP.DK.USD.S-TP.DK.GBP.A-TP.DK.GBP.S"
//!     let data_series = "TP.DK.USD.A";
//!
//!     let date = date::Date::from("13-12-2011")?;
//!     let date_preference = date::DatePreference::Single(date);
//!
//!     let api_key = common::ApiKey::from("user_api_key".to_string())?;
//!     let return_format = common::ReturnFormat::Xml;
//!     let evds = common::Evds::from(api_key, return_format);
//!
//!
//!     // get data operation based on given series.
//!     let currency_data = evds_basic::get_data(data_series, &date_preference, &evds)?;
//!
//! #   Ok(())
//! # }
//! ```
//! For more and other function implementations and details, please go to [`evds_currency`](crate::evds_currency) module 
//! stage.
//!
//! ```
//! # use std::error::Error;
//!     use tcmb_evds::*;
//! 
//! # fn main() -> Result<(), Box<dyn Error>> {
//! 
//!     // common elements
//!     let api_key = common::ApiKey::from("user_api_key".to_string())?; 
//!     let return_format = common::ReturnFormat::Json;
//!     let evds = common::Evds::from(api_key, return_format);
//!
//!     let date = date::Date::from("13-12-2011")?;
//!     let date_preference = date::DatePreference::Single(date);
//!
//!
//!     // currency series creation
//!     let exchange_type = evds_currency::ExchangeType::new();
//! 
//!     let currency_code = evds_currency::CurrencyCode::Usd;
//! 
//!     // Ytl mode adds "YTL" to currency series, when it is true.
//!     let ytl_mode = true;
//!   
//!     let currency_series = 
//!         evds_currency::CurrencySeries::from(
//!             exchange_type, 
//!             currency_code, 
//!             date_preference, 
//!             ytl_mode
//!         );
//! 
//! 
//!     // get data operation based on created CurrencySeries.
//!     let currency_data = currency_series.get_data(&evds)?;
//!
//! #   Ok(())
//! # }
//! ```
//!
//! # Comparison
//!
//! - evds_basic
//!
//!     - Provides **most of the EVDS web service operations** except currency value with frequency formulas
//!     service which is called advanced currency operations in this crate. 
//!     - Users are responsible for ensuring validity of the given series and some data.
//!     - Less reliable for the currency service operations.
//! 
//! - evds_currency
//!
//!     - Specified for operating **currency operations**.
//!     - Given series and some data are automatically checked.
//!     - More **reliable** for the **currency service operations**.
//!
//! # User Guide
//!
//! First of all, please take a look at *[README.md](https://github.com/asari555/tcmb_evds/blob/master/README.md)* file 
//! before using the crate.
//!
//! Additional resource for understanding some parameters, learning various data series and details of web services are 
//! mentioned in [`kullanim kilavuzu`] for Turkish and [`user guide`].
//!
//! 
//! [`user guide`]: <https://evds2.tcmb.gov.tr/help/videos/EVDS_Web_Service_Usage_Guide.pdf>
//! [`kullanim kilavuzu`]: <https://evds2.tcmb.gov.tr/help/videos/EVDS_Web_Servis_Kullanim_Kilavuzu.pdf>
//!
//! # Appendix
//!
//! To understand *ytl_mode* required in some functions and what is "YTL", please follow [`what is YTL?`].
//!
//! [`what is YTL?`]: <https://en.wikipedia.org/wiki/Revaluation_of_the_Turkish_lira>


// #[deny(missing_docs)]


/// contains two main elements that are used in operations of
/// [`evds_basic`](crate::evds_basic) and [`evds_currency`](crate::evds_currency).
/// 
/// [`ReturnFormat`](crate::common::ReturnFormat) specifies format of the database response and 
/// [`ApiKey`](crate::common::ApiKey) is created to supply valid key to the functions making request operations. 
/// [`Evds`](crate::common::Evds) becomes connection between the functions and both 
/// [`ReturnFormat`](crate::common::ReturnFormat) and [`ApiKey`](crate::common::ApiKey) variables.
///
/// # Usage
///
/// The each element is explained and exampled in their definition parts. This usage illustrates 
/// combined version of common elements on Evds structure to be used in related functions.
///
/// ```
/// # use std::error::Error;
///     use tcmb_evds::common::*;
///
/// # use tcmb_evds::error::ReturnError;
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
/// 
///     // Please use a valid key here.
///     let api_key = ApiKey::from("users_valid_key".to_string())?;
///
///     let return_format = ReturnFormat::Json;
///
///     let evds = Evds::from(api_key, return_format);
///
/// #   Ok(())
/// # }
/// ```
pub mod common;
/// contains date elements that are used in some functions of [`evds_basic`](crate::evds_basic) and 
/// [`evds_currency`](crate::evds_currency).
///
/// [`Date`](crate::date::Date) and [`DateRange`](crate::date::DateRange) are created to supply single date and multiple 
/// dates called date range. [`DatePreference`](crate::date::DatePreference) includes [`Date`](crate::date::Date) or 
/// [`DateRange`](crate::date::DateRange) and supplies date options to related functions of 
/// [`evds_basic`](crate::evds_basic) and [`evds_currency`](crate::evds_currency).
///
/// # Usage
///
/// The each element is explained and exampled in their definitions parts.
///
/// ```
/// # use std::error::Error;
///     use tcmb_evds::date::*;
///
/// # use tcmb_evds::error::ReturnError;
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
///
///     // Single date example.
///     let date = Date::from("13-12-2011")?;
///
///     let date_preference = DatePreference::Single(date);
///     
///     
///     // Multiple dates example.
///     let date_range = DateRange::from("13-12-2011", "13-12-2020")?;
///
///     let date_preference = DatePreference::Multiple(date_range);
///
/// #   Ok(())
/// # }
/// ```
pub mod date;
/// contains specified error options that are returned from the functions of 
/// [`evds_basic`](crate::evds_basic) and [`evds_currency`](crate::evds_currency) to illustrate why the error occurs.
///
/// One of the [`ReturnError`](crate::error::ReturnError) options is returned when something goes wrong with requesting 
/// data or giving parameter to the functions. Therefore, users are able to handle specified error types and to 
/// stringify them in a standard format.
pub mod error;
/// provides most of the EVDS web services except requesting advanced currency data that means currency data with 
/// frequency formulas.
pub mod evds_basic;
/// provides only currency operations with methods of [`CurrencySeries`] and [`MultipleCurrencySeries`].
///
/// This module built on two main structures and their methods operating currency services. The basic difference between
/// these structures is number of currency types. [`CurrencySeries`](crate::evds_currency::CurrencySeries) serves
/// methods using a currency type. In contrast, [`MultipleCurrencySeries`](crate::evds_currency::MultipleCurrencySeries)
/// serves a method using more than a currency type to get currencies data.   
///
/// `CurrencySeries` is composed of [`CurrencyCode`], `ExchangeType`, `DatePreference` and `ytl_mode` to supply 
/// adequate information to make requesting data about **a currency** via related **operational methods**. 
///
/// `MultipleCurrencySeries` is composed of [`CurrencyCodes`], `ExchangeType`, `DatePreference` and `ytl_mode` to 
/// supply adequate information to make requesting data about **given currencies** via **an operational method**. 
///
/// ### Usage schematic of operational functions and hierarchy of this module
/// 
/// - [`CurrencySeries`] -> [`get_data`] ( [`Evds`] ) or [`get_advanced_data`] ( [`Evds`], [`AdvancedProcesses`] )
///
///     > `CurrencySeries` requires below struct as a difference
///     > + [`CurrencyCode`]
///
/// - [`MultipleCurrencySeries`] -> [`get_multiple_data`] ( [`Evds`])
///
///     > `MultipleCurrencySeries` requires below struct as a difference:
///     > + [`CurrencyCodes`]
///
/// ### Both `CurrencySeries` and `MultipleCurrencySeries` requires common structures and a variable
///
/// - [`ExchangeType`]
/// - [`DatePreference`]
/// - `ytl_mode: bool`
///
/// ### Generic view of operational methods
///
/// - [`get_data`]
///
/// - [`get_advanced_data`]
/// 
/// - [`get_multiple_data`]
///
/// To use the operational methods, the related structures for the required operation should be built firstly. Then,
/// the methods can be used with the common structure that is [`Evds`].
///
/// *More details of operational functions and their usage are included in their stages.*
///
/// [`CurrencySeries`]: crate::evds_currency::CurrencySeries
/// [`MultipleCurrencySeries`]: crate::evds_currency::MultipleCurrencySeries
///
/// [`Evds`]: crate::common::Evds
/// [`DatePreference`]: crate::date::DatePreference
/// [`ExchangeType`]: crate::evds_currency::ExchangeType
///
/// [`CurrencySeries`]: crate::evds_currency::CurrencySeries
/// [`CurrencyCode`]: crate::evds_currency::CurrencyCode
/// [`AdvancedProcesses`]: crate::evds_currency::frequency_formulas::AdvancedProcesses
///
/// [`MultipleCurrencySeries`]: crate::evds_currency::MultipleCurrencySeries
/// [`CurrencyCodes`]: crate::evds_currency::CurrencyCodes
///
/// [`get_data`]: crate::evds_currency::CurrencySeries::get_data
/// [`get_advanced_data`]: crate::evds_currency::CurrencySeries::get_advanced_data
/// [`get_multiple_data`]: crate::evds_currency::MultipleCurrencySeries::get_multiple_data
pub mod evds_currency;
mod traits;

#[cfg(feature = "async_mode")]
mod request_async;
#[cfg(feature = "sync_mode")]
mod request_sync;
