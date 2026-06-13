use bigdecimal::{BigDecimal, ToPrimitive};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Currency {
    BRL,
    USD,
    EUR,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Money {
    pub amount_minor: i64, // menor unidade (centavos)
    pub currency: Currency,
}

impl Money {
    pub fn new(amount_minor: i64, currency: Currency) -> Self {
        Self {
            amount_minor,
            currency,
        }
    }

    /// Converte para decimal (ex: 1050 -> 10.50)
    pub fn to_decimal(&self) -> BigDecimal {
        BigDecimal::from(self.amount_minor) / BigDecimal::from(100)
    }

    /// Cria a partir de decimal (ex: 10.50 -> 1050)
    #[allow(dead_code)]
    pub fn from_decimal(value: BigDecimal, currency: Currency) -> Self {
        let scaled = value * BigDecimal::from(100);

        // arredondamento seguro (half-up)
        let cents = scaled
            .with_scale(0)
            .to_i64()
            .expect("Erro ao converter BigDecimal para i64");

        Self {
            amount_minor: cents,
            currency,
        }
    }

    /// Aplica porcentagem usando basis points (ex: 2000 = 20%)
    #[allow(dead_code)]
    pub fn apply_basis_points(&self, basis_points: i64) -> Self {
        let result = (self.amount_minor * basis_points) / 10_000;
        Self {
            amount_minor: result,
            currency: self.currency,
        }
    }

    /// Soma segura (mesma moeda)
    pub fn checked_add(self, other: Self) -> Result<Self, String> {
        if self.currency != other.currency {
            return Err(format!(
                "Cannot add different currencies: {:?} and {:?}",
                self.currency, other.currency
            ));
        }

        Ok(Self {
            amount_minor: self.amount_minor + other.amount_minor,
            currency: self.currency,
        })
    }

    /// Subtração segura
    pub fn checked_sub(self, other: Self) -> Result<Self, String> {
        if self.currency != other.currency {
            return Err(format!(
                "Cannot subtract different currencies: {:?} and {:?}",
                self.currency, other.currency
            ));
        }

        Ok(Self {
            amount_minor: self.amount_minor - other.amount_minor,
            currency: self.currency,
        })
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let decimal = self.to_decimal();
        write!(f, "{} {:?}", decimal, self.currency)
    }
}
