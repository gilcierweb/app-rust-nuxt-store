use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentGatewayEnvironment {
    Test = 1,
    Production = 2,
}

impl PaymentGatewayEnvironment {
    pub fn from_i16(value: i16) -> Option<Self> {
        match value {
            1 => Some(Self::Test),
            2 => Some(Self::Production),
            _ => None,
        }
    }

    pub fn to_i16(self) -> i16 {
        self as i16
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Test => "test",
            Self::Production => "production",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentGatewayStatus {
    Inactive = 1,
    Active = 2,
    SandboxOnly = 3,
}

impl PaymentGatewayStatus {
    pub fn from_i16(value: i16) -> Option<Self> {
        match value {
            1 => Some(Self::Inactive),
            2 => Some(Self::Active),
            3 => Some(Self::SandboxOnly),
            _ => None,
        }
    }

    pub fn to_i16(self) -> i16 {
        self as i16
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Inactive => "inactive",
            Self::Active => "active",
            Self::SandboxOnly => "sandbox_only",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentMethodType {
    Card = 1,
    Wallet = 2,
    Pix = 3,
    Boleto = 4,
    BankTransfer = 5,
    Manual = 6,
    BuyNowPayLater = 7,
}

impl PaymentMethodType {
    pub fn from_i16(value: i16) -> Option<Self> {
        match value {
            1 => Some(Self::Card),
            2 => Some(Self::Wallet),
            3 => Some(Self::Pix),
            4 => Some(Self::Boleto),
            5 => Some(Self::BankTransfer),
            6 => Some(Self::Manual),
            7 => Some(Self::BuyNowPayLater),
            _ => None,
        }
    }

    pub fn to_i16(self) -> i16 {
        self as i16
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Card => "card",
            Self::Wallet => "wallet",
            Self::Pix => "pix",
            Self::Boleto => "boleto",
            Self::BankTransfer => "bank_transfer",
            Self::Manual => "manual",
            Self::BuyNowPayLater => "buy_now_pay_later",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentMethodDisplay {
    Frontend = 1,
    Backend = 2,
    Both = 3,
}

impl PaymentMethodDisplay {
    pub fn from_i16(value: i16) -> Option<Self> {
        match value {
            1 => Some(Self::Frontend),
            2 => Some(Self::Backend),
            3 => Some(Self::Both),
            _ => None,
        }
    }

    pub fn to_i16(self) -> i16 {
        self as i16
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Frontend => "front",
            Self::Backend => "back",
            Self::Both => "both",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentIntent {
    Purchase = 1,
    Authorize = 2,
    Capture = 3,
}

impl PaymentIntent {
    pub fn from_i16(value: i16) -> Option<Self> {
        match value {
            1 => Some(Self::Purchase),
            2 => Some(Self::Authorize),
            3 => Some(Self::Capture),
            _ => None,
        }
    }

    pub fn to_i16(self) -> i16 {
        self as i16
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Purchase => "purchase",
            Self::Authorize => "authorize",
            Self::Capture => "capture",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentAttemptStatus {
    Checkout = 1,
    Pending = 2,
    Processing = 3,
    Authorized = 4,
    Captured = 5,
    Failed = 6,
    Voided = 7,
    Cancelled = 8,
    Refunded = 9,
    PartiallyRefunded = 10,
}

impl PaymentAttemptStatus {
    pub fn from_i16(value: i16) -> Option<Self> {
        match value {
            1 => Some(Self::Checkout),
            2 => Some(Self::Pending),
            3 => Some(Self::Processing),
            4 => Some(Self::Authorized),
            5 => Some(Self::Captured),
            6 => Some(Self::Failed),
            7 => Some(Self::Voided),
            8 => Some(Self::Cancelled),
            9 => Some(Self::Refunded),
            10 => Some(Self::PartiallyRefunded),
            _ => None,
        }
    }

    pub fn to_i16(self) -> i16 {
        self as i16
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Checkout => "checkout",
            Self::Pending => "pending",
            Self::Processing => "processing",
            Self::Authorized => "authorized",
            Self::Captured => "captured",
            Self::Failed => "failed",
            Self::Voided => "voided",
            Self::Cancelled => "cancelled",
            Self::Refunded => "refunded",
            Self::PartiallyRefunded => "partially_refunded",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentSessionStatus {
    Pending = 1,
    Processing = 2,
    RequiresAction = 3,
    Completed = 4,
    Failed = 5,
    Cancelled = 6,
    Expired = 7,
}

impl PaymentSessionStatus {
    pub fn from_i16(value: i16) -> Option<Self> {
        match value {
            1 => Some(Self::Pending),
            2 => Some(Self::Processing),
            3 => Some(Self::RequiresAction),
            4 => Some(Self::Completed),
            5 => Some(Self::Failed),
            6 => Some(Self::Cancelled),
            7 => Some(Self::Expired),
            _ => None,
        }
    }

    pub fn to_i16(self) -> i16 {
        self as i16
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Processing => "processing",
            Self::RequiresAction => "requires_action",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
            Self::Expired => "expired",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentRefundStatus {
    Pending = 1,
    Processing = 2,
    Succeeded = 3,
    Failed = 4,
    Cancelled = 5,
}

impl PaymentRefundStatus {
    pub fn from_i16(value: i16) -> Option<Self> {
        match value {
            1 => Some(Self::Pending),
            2 => Some(Self::Processing),
            3 => Some(Self::Succeeded),
            4 => Some(Self::Failed),
            5 => Some(Self::Cancelled),
            _ => None,
        }
    }

    pub fn to_i16(self) -> i16 {
        self as i16
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Processing => "processing",
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentGatewayEventStatus {
    Received = 1,
    Processing = 2,
    Processed = 3,
    Failed = 4,
    Ignored = 5,
}

impl PaymentGatewayEventStatus {
    pub fn from_i16(value: i16) -> Option<Self> {
        match value {
            1 => Some(Self::Received),
            2 => Some(Self::Processing),
            3 => Some(Self::Processed),
            4 => Some(Self::Failed),
            5 => Some(Self::Ignored),
            _ => None,
        }
    }

    pub fn to_i16(self) -> i16 {
        self as i16
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Received => "received",
            Self::Processing => "processing",
            Self::Processed => "processed",
            Self::Failed => "failed",
            Self::Ignored => "ignored",
        }
    }
}
