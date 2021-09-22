//! Easy and ready access to the database of The Central Bank of The Republic of Turkey (CBRT).
//!
//! This crate provides three main separate mechanisms for acquiring data from the database:
//!
//! - [`evds_basic`](crate::evds_basic) includes functions making most of the web service operations except
//! currency operations with frequency formulas. 
//! - [`evds_currency`](crate::evds_currency) includes structure-based and implemented methods that make totally 
//! currency operations.
//! - [`evds_c`](crate::evds_c) includes FFI functions making all of the web service operations to make users able to
//! utilize these functions in **C language**. 
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
//! FFI functions for C language are given in the [`evds_c`](crate::evds_c) module. These functions supply all of the 
//! EVDS operations. In addition, most parameters of the functions are checked to make valid request. As a result of the 
//! checking, detailed and specified error types are returned to easily handle and fix the errors. Additionally, only C 
//! functions requires **ascii_mode** to convert a format having non-English characters of the response text to another 
//! format including only English characters. Moreover, the mode as an argument converts non-utf8 characters to ' * ' 
//! **if there is**. Therefore, the response text becomes safe against non-ascii characters. To use 
//! [`evds_c`](crate::evds_c) in C language, users should build the crate and use both built *libtcmb_evds.so* in the 
//! release folder and *tcmb_evds.h* file in the target folder. 
//!
//! This crate provides `async_mode`, `sync_mode` and `c_mode` as a feature. `async_mode` and `sync_mode` features
//! should be used with an async and a sync programming respectively. Furthermore, the `async_mode` feature is enabled 
//! by default. One of these two features should be used to work with this crate. In addition, `c_mode` is an optional
//! feature to create an applicable library with C language. This feature allows users to reach safety Rust codes from
//! a program written in C. 
//!
//! # Install
//!
//! Please, add **appropriate one** of the blow codes to your **Cargo.toml** to install the crate.
//! 
//!
//!
//! *sync_mode* and *c_mode* is included in default feature to use in C language.
//!
//! ```
//! [dependencies]
//! tcmb_evds = "0.1"
//! ```
//!
//! For *async_mode*, please add.
//!
//! ```
//! [dependencies]
//! tcmb_evds = {version = "0.1", default-features = false, features = ["async_mode"]}
//! ```
//!
//! For *sync_mode*, please add.
//!
//! ```
//! [dependencies]
//! tcmb_evds = {version = "0.1", default-features = false, features = ["sync_mode"]}
//! ```
//!
//! **Attention:** `c_mode` cannot be used alone without `sync_mode`.
//!
//! # Rust Usage
//!
//! For more and other function implementations and details, please go to [`evds_basic`](crate::evds_basic) module 
//! stage.
//!
//! ```
//! # use std::error::Error;
//! # use tcmb_evds::date;     
//! # use tcmb_evds::common;    
//! # use tcmb_evds::error::ReturnError;
//!     use tcmb_evds::evds_basic;
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
//!     let api_key = common::ApiKey::from("user_api_key")?;
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
//! #   use tcmb_evds::date::{Date, DatePreference};        
//! #   use tcmb_evds::evds_currency::{ExchangeType, CurrencyCode};
//! #   use tcmb_evds::common;
//!     use tcmb_evds::evds_currency;
//! 
//! # fn main() -> Result<(), Box<dyn Error>> {
//! 
//!     // common elements
//!     let api_key = common::ApiKey::from("user_api_key")?; 
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
//! # C Usage
//!
//! For more and other function implementations, please move on [`evds_c`](crate::evds_c) details.
//! 
//!
//! [`tcmb_evds_get_data`](crate::evds_c::tcmb_evds_get_data) usage example.
//! 
//! ```
//! #include "tcmb_evds.h"
//! 
//! int main() {
//!     // declaration of required arguments for get data operation.
//!     TcmbEvdsInput data_series;
//!     TcmbEvdsInput date;
//! 
//!     // declaring common variables for creating Evds common struct for each FFI function.
//!     TcmbEvdsInput api_key;
//!     TcmbEvdsReturnFormat return_format;
//! 
//!     bool ascii_mode;
//! 
//! 
//!     data_series.input_ptr = "TP.DK.USD.S";
//!     data_series.string_capacity = strlen(data_series.input_ptr);
//!
//!     date.input_ptr = "13-12-2011, 12-12-2012";
//!     date.string_capacity = strlen(date.input_ptr);
//!
//!     api_key.input_ptr = "VALID_API_KEY";
//!     api_key.string_capacity = strlen(api_key.input_ptr);
//! 
//!     return_format = Csv
//!
//!     ascii_mode = false;
//!
//! 
//!     // due to the fact that data_result may include error, please check it first.
//!     TcmbEvdsResult data_result = tcmb_evds_get_data(data_series, date, api_key, return_format, ascii_mode);
//! 
//!     return 0;
//! }
//! ```
//! 
//! 
//! [`tcmb_evds_get_advanced_data`](crate::evds_c::tcmb_evds_get_advanced_data) usage example.
//! 
//! ```
//! #include "tcmb_evds.h"
//! 
//! int main() {
//!     // declaration of required arguments for get data operation.
//!     TcmbEvdsInput data_series;
//!     TcmbEvdsInput date;
//!    
//!     // declaration of frequency formulas entities. 
//!     TcmbEvdsAggregationType aggregation_type;
//!     TcmbEvdsFormula formula;
//!     TcmbEvdsDataFrequency data_frequency;
//!    
//!     // declaring common variables for creating Evds common struct for each FFI function.
//!     TcmbEvdsInput api_key;
//!     TcmbEvdsReturnFormat return_format;
//!    
//!     bool ascii_mode;
//!    
//!     
//!     data_series.input_ptr = "TP.DK.USD.A";
//!     data_series.string_capacity = strlen(data_series.input_ptr);
//!    
//!     date.input_ptr = "13-12-2011";
//!     date.string_capacity = strlen(date.input_ptr);
//!      
//!     aggregation_type = End;
//!     formula = Level;
//!     data_frequency = Monthly;
//!    
//!     api_key.input_ptr = "VALID_API_KEY";
//!     api_key.string_capacity = strlen(api_key.input_ptr);
//!    
//!     return_format = Json;
//! 
//!     ascii_mode = false;
//! 
//!    
//!     // due to the fact that data_result may include error, please check it first.
//!     TcmbEvdsResult advanced_data_result = 
//!         tcmb_evds_get_advanced_data(
//!             data_series, 
//!             date, 
//!             aggregation_type, 
//!             formula, 
//!             data_frequency, 
//!             api_key, 
//!             return_format,
//!             ascii_mode
//!             );
//!
//!     return 0;
//! }
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
//!     - Rust specific.
//! 
//! - evds_currency
//!
//!     - Specified for operating **currency operations**.
//!     - Given series and some data are automatically checked.
//!     - More **reliable** for the **currency service operations**.
//!     - Rust specific.
//!
//! - evds_c 
//!
//!     - Provides all of the EVDS web service operations.
//!     - Most of the function parameters in the module are automatically checked inside the functions.
//!     - Users are responsible to give valid and correct arguments to related parameters not supporting auto control.
//!     - Includes functionalities of [`evds_basic`](crate::evds_basic) and [`evds_currency`](crate::evds_currency).
//!     - C specific.
//!
//! # User Guide
//!
//! First of all, please take a look at *benioku.md* for Turkish or *readme.md* files before using the crate.
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
/// # use tcmb_evds::common::{ApiKey, ReturnFormat, Evds};
/// # use tcmb_evds::error::ReturnError;
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
/// #
///     // Please use a valid key here.
///     let api_key = ApiKey::from("users_valid_key")?;
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
/// # use tcmb_evds::date::{Date, DateRange, DatePreference};
/// # use tcmb_evds::error::ReturnError;
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
/// #
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
/// Usage schematic and hierarchy of this module:
/// 
/// - [`CurrencySeries`] -> [`get_data`] ( [`Evds`] ) or [`get_advanced_data`] ( [`Evds`], [`AdvancedProcesses`] )
///
///     > `CurrencySeries` requires below struct as a difference:
///     > + [`CurrencyCode`]
///
/// - [`MultipleCurrencySeries`] -> [`get_multiple_data`] ( [`Evds`])
///
///     > `MultipleCurrencySeries` requires below struct as a difference:
///     > + [`CurrencyCodes`]
///
/// Both `CurrencySeries` and `MultipleCurrencySeries` requires common structures and a variable: 
///
/// - [`ExchangeType`]
/// - [`DatePreference`]
/// - `ytl_mode: bool`
///
/// Generic view of operational methods:
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


#[cfg(feature = "c_mode")]
/// provides an FFI to use abilities of the crate in C language.
///
/// This module has almost the same structural concept with the [`tcmb_evds`] crate. [`advanced_entities`], 
/// [`common_entities`] and [`error_handling`] corresponds [`frequency_formulas`] sub-module in [`evds_currency`], 
/// [`common`] and [`error`] in the crate respectively. These modules are responsible to supply required arguments for 
/// related parameters declared with various functions.
///
/// Each function, enum and struct of this module includes lucid explanation and its detailed usage example in its 
/// section.
///
/// Functions of the module has a parameter called ascii_mode that makes the returned string in ascii characters. 
/// In addition non-English characters is given as English characters and  non ascii characters are returned as '*'.
///
/// The related output library of the crate is built in the folder where the `Cargo build` command builds. To see the 
/// corresponding function names, enum and struct types in terms of **C**, please look at `target/tcmb_evds.h` 
/// automatically built C header file. In addition, to utilize the crate in C language, please link the built 
/// **libtcmb_evds.so** in `target/debug` or `target/release` depending on users build option and include **tcmb_evds.h** 
/// files.
///
/// To see the usage example of each function, please move on the sections of the functions.
///
/// [`tcmb_evds`]: crate
/// [`advanced_entities`]: crate::evds_c::advanced_entities
/// [`common_entities`]: crate::evds_c::common_entities
/// [`error_handling`]: crate::evds_c::error_handling
/// [`frequency_formulas`]: crate::evds_currency::frequency_formulas
/// [`evds_currency`]: crate::evds_currency
/// [`common`]: crate::common
/// [`error`]: crate::error
pub mod evds_c;
#[cfg(feature = "async_mode")]
mod request_async;
#[cfg(feature = "sync_mode")]
mod request_sync;


#[cfg(feature = "c_mode")]
extern crate libc;
