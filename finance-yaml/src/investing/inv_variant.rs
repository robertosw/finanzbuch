use serde::Deserialize;
use serde::Serialize;
use std::str::FromStr;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum InvestmentVariant
{
    Stock,
    Fund,
    Etf,
    Bond,
    Option,
    Commoditiy,
    Crypto,
}
impl FromStr for InvestmentVariant
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        match s.to_lowercase().as_str() {
            "stock" => Ok(Self::Stock),
            "fund" => Ok(Self::Fund),
            "etf" => Ok(Self::Etf),
            "bond" => Ok(Self::Bond),
            "option" => Ok(Self::Option),
            "commoditiy" => Ok(Self::Commoditiy),
            "crypto" => Ok(Self::Crypto),
            _ => Err(format!("{s} is not a possible InvestmentVariant")),
        }
    }
}
impl std::fmt::Display for InvestmentVariant
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self {
            InvestmentVariant::Stock => write!(f, "Stock"),
            InvestmentVariant::Fund => write!(f, "Fund"),
            InvestmentVariant::Etf => write!(f, "Etf"),
            InvestmentVariant::Bond => write!(f, "Bond"),
            InvestmentVariant::Option => write!(f, "Option"),
            InvestmentVariant::Commoditiy => write!(f, "Commoditiy"),
            InvestmentVariant::Crypto => write!(f, "Crypto"),
        }
    }
}
