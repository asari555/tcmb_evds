# tcmb_evds

A Rust crate for reaching the database of The Central Bank of The Republic of Turkey (CBRT).

## Table of Contents

+ **[About](#about)**
+ **[Install](#install)**
+ **[Usage](#usage)**
+ **[Details](#details)**
+ **[Comparison](#comparison)**
+ **[More](#more)**
+ **[References](#references)**

## About

Name of the crate comes from Turkish meaning of the abbreviations of CBRT and the electronic data delivery system. Let's move on the contents. The crate is designed via main two structures and provides sync-async modes for various purposes.

Operational functions of the crate is based on EVDS web services. These services are divided into two parts in this crate. `evds_basic` and `evds_currency` are the mentioned parts. The latter provides all of the web services operations without adequate control mechanisms compared to the former in terms of currency operations. It can easily be understood that `evds_currency` provides access to currency operations. Furthermore, type and validity control is much obvious in this part. The `evds_currency` includes additional self-control algorithms, although it seems a little bit complicated according to `evds_basic`. These algorithms, checks syntax and validity of the given data. It is hard to get error with this structure.

In addition, the overall structure of the crate changes with respect to selected mode. Sync and async modes are recommended for sync and async programmings respectively. The mode selection can be made using feature adjustments.

The next contents include details of the two main structures, mode selection, examples and detailed explanations of the crate.

## Install

Please, add **appropriate one** of the blow codes to your **Cargo.toml** to install the crate.

For *async_mode*, please add.

```text
[dependencies]
tcmb_evds = "0.1"
```

For *sync_mode*, please add.

```text
[dependencies]
tcmb_evds = {version = "0.1", default-features = false, features = ["sync_mode"]
```

## Usage

### evds_basic

An example of **get_data** getting series data.

```rust
    use tcmb_evds::*;
 

    // assigning required arguments.
    // another data series = "TP.DK.USD.A-TP.DK.USD.S-TP.DK.GBP.A-TP.DK.GBP.S"
    let data_series = "TP.DK.USD.A";

    let date = date::Date::from("13-12-2011")?;
    let date_preference = date::DatePreference::Single(date);

    let api_key = common::ApiKey::from("user_api_key".to_string())?;
    let return_format = common::ReturnFormat::Xml;
    let evds = common::Evds::from(api_key, return_format);


    // get data operation based on given series.
    let currency_data = evds_basic::get_data(data_series, &date_preference, &evds)?;
```

### evds_currency

An example of **get_data** getting currency series data.

```rust
    use tcmb_evds::*;
 

    // common elements
    let api_key = common::ApiKey::from("user_api_key".to_string())?; 
    let return_format = common::ReturnFormat::Json;
    let evds = common::Evds::from(api_key, return_format);

    let date = date::Date::from("13-12-2011")?;
    let date_preference = date::DatePreference::Single(date);


    // currency series creation
    let exchange_type = evds_currency::ExchangeType::new();
 
    let currency_code = evds_currency::CurrencyCode::Usd;
 
    // Ytl mode adds "YTL" to currency series, when it is true.
    let ytl_mode = true;
   
    let currency_series = 
        evds_currency::CurrencySeries::from(
            exchange_type, 
            currency_code, 
            date_preference, 
            ytl_mode
        );
 
 
    // get data operation based on created CurrencySeries.
    let currency_data = currency_series.get_data(&evds)?;
```

## Details

There are some details to clarify users' knowledge of how to use this tcmb_evds
API library.

### evds_basic

This part is the one of the two fundamental parts users face with. The most 
EVDS web service operations can be done with using evds_basic API functions. 
These functions are **get_data**, **get_data_group**, **get_categories**, 
**get_advanced_data_group** and **get_series_list**. The functionalities of 
these functions are explained lucidly in the documentation of the project. At 
the same time, users can learn more about these functions below. The 
**Function Names** and **Web Services as Functions** parts give extra 
information about these functions.

Functions of *evds_basic* uses some structures given below in supporting 
structures which are *date::DatePreference* and *common::Evds* that check 
whether the given data is valid or not when they are created. These supporters 
meet the requirement of common information that EVDS need as standard. So, the 
given data by users are checked and automatically converted to convenient urls 
with these supporters. However, there are some parameters, the mentioned 
functions requires for specific EVDS operations. These parameters are directly 
used in last url format for web service requesting. That means there is no data 
validity checking for these parameters. Therefore, user becomes responsible to 
supply valid data for mentioned parameters.

### evds_currency

This is the other part of the two fundamental parts users face with. The 
specific currency operations can be done with using evds_currency API 
functions. These functions are **get_data**, **get_advanced_data** and 
**get_multiple_data**. These functions are methods of **CurrencySeries** and 
**MultipleCurrencySeries** structures. How to use details of these functions 
are explained lucidly in the documentation of the project. At the same time, 
users can learn more about these functions below. The **Function Names** and 
**Web Services as Functions** parts give extra information about these 
functions.

Mentioned methods require some structures that are divided into two. 
These are main and supporting structures. The formers are called 
*CurrencySeries* and *MultipleCurrencySeries*. These structures implements the 
mentioned methods that are the main operation functions and contains important sub-structures explained in the crate documentation. The latter structures are 
called *common::Evds* and *AdvancedProcesses*. The first one supply common 
information that EVDS need as standard. The other one gives users more 
customization option to get specific currency data. 

The mentioned 3 methods use pre-created structures. When these structures are 
created, this crate checks validity and correctness of the given data. 
Therefore, usage of *evds_currency* methods are error free methods unlike 
*evds_basic*. Namely, *evds_currency* is responsible for checking the validity 
of the giving data.

### Function Names

The *evds_basic* API functions given in this crate:

  * **get_data** returns data about requested data series.

  * **get_data_group** returns requested data group.

  * **get_categories** returns all requested categories of EVDS.

  * **get_advanced_data_group** returns specified data groups.
  
  * **get_series_list** returns all usable series list.

The *evds_currency* API functions given in this crate:

  * **get_data** returns data about just one currency.

  * **get_advanced_data** returns data about just one currency with frequency 
  * formulas.

  * **get_multiple_data** returns data about more than one currency.

### Supporting Structures

**Common**: Fundamental structures used in user API functions for both 
*evds_basic* and *evds_currency*.

  * *DatePreference*: Most of the functions requires. 
  * *Evds*: Every function requires.

**For evds_basic**

  * *DatePreference*

**For evds_currency**
  
  + *AdvancedProcesses*: Includes advanced currency data configurations for 
    *get_advanced_data* function.

  + *Currency Series*: A composite structure containing below structures 
    to supply single and advanced (configured) currency data.

    * *ytl_mode*: This is a bool option to choose currency value wrt YTL 
    revolution.
    * *ExchangeType*: This is a structure to determine currency value is 
    selling/buying.
    * *CurrencyCode*: An enum of currency unit option.
    * *DatePreference*: Fundamental common structure.
    
  + *MultipleCurrencySeries*: A composite structure containing below structures 
    to supply multiple currencies data.

      * *ytl_mode*
      * *ExchangeType*
      * *CurrencyCodes*: A structure of currency unit options.
      * *DatePreference*
  
### Web Services as Functions

The web services, described in [`user guide`](https://evds2.tcmb.gov.tr/help/videos/EVDS_Web_Service_Usage_Guide.pdf) and [`kullanim kilavuzu`](https://evds2.tcmb.gov.tr/help/videos/EVDS_Web_Servis_Kullanim_Kilavuzu.pdf) in 
Turkish, correspond to following functions.

Format: 
    
    'function_sign' corresponds to 'a section of user guide'

evds_basic functions:  

* **get_data** corresponds to **1. EVDS Series Data Services** & **2.1. Level Values Requests**
* **get_data_group** corresponds to **3. All Series Data By Given Data Group**
* **get_categories** corresponds to **4.1. Category Service**
* **get_advanced_data_group** corresponds to **4.2.Data Group Service**
* **get_series_list** corresponds to **4.3. Series Service**

evds_currency methods:

* **get_data** corresponds to **1. EVDS Series Data Services**
* **get_advanced_data** corresponds to **2.2. ... series with frequecy formulas**
* **get_multiple_data** corresponds to **2.1. Level Values Requests**

## Comparison

*This section illustrates the fundamental differences of the two main structures.*

+ evds_basic
  + Provides most of the EVDS web service operations except currency value with frequency formulas service which is called advanced currency operations in this crate.
  + Users are responsible for ensuring validity of the given series and some data.
  + Less reliable for the currency service operations.

+ evds_currency
  + Specified for operating currency operations.
  + Given series and some data are automatically checked.
  + More reliable for the currency service operations.

## More

Detailed examples of usage and more are given in the documentation of the crate. User can visit [`crates.io`](https://crates.io/) or clone the repository and just enter `Cargo doc --open` command into the terminal.

## References

+ [`EVDS User Guide`](https://evds2.tcmb.gov.tr/help/videos/EVDS_Web_Service_Usage_Guide.pdf)

+ [`EVDS Kullanim Kilavuzu`](https://evds2.tcmb.gov.tr/help/videos/EVDS_Web_Servis_Kullanim_Kilavuzu.pdf)
