use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum OrderStatus {
    Pending = 1,
    Confirmed = 2,
    Processing = 3,
    Shipped = 4,
    Delivered = 5,
    Cancelled = 6,
}

impl OrderStatus {
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::Pending),
            2 => Some(Self::Confirmed),
            3 => Some(Self::Processing),
            4 => Some(Self::Shipped),
            5 => Some(Self::Delivered),
            6 => Some(Self::Cancelled),
            _ => None,
        }
    }

    pub fn to_i32(&self) -> i32 {
        *self as i32
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Confirmed => "confirmed",
            Self::Processing => "processing",
            Self::Shipped => "shipped",
            Self::Delivered => "delivered",
            Self::Cancelled => "cancelled",
        }
    }

    pub fn valid_transitions(&self) -> &'static [OrderStatus] {
        match self {
            Self::Pending => &[Self::Confirmed, Self::Cancelled],
            Self::Confirmed => &[Self::Processing, Self::Cancelled],
            Self::Processing => &[Self::Shipped, Self::Cancelled],
            Self::Shipped => &[Self::Delivered],
            Self::Delivered => &[],
            Self::Cancelled => &[],
        }
    }

    pub fn can_transition_to(&self, new: OrderStatus) -> bool {
        self.valid_transitions().contains(&new)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum PaymentStatus {
    Unpaid = 1,
    Paid = 2,
    Refunded = 3,
    PartiallyRefunded = 4,
}

impl PaymentStatus {
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::Unpaid),
            2 => Some(Self::Paid),
            3 => Some(Self::Refunded),
            4 => Some(Self::PartiallyRefunded),
            _ => None,
        }
    }

    pub fn to_i32(&self) -> i32 {
        *self as i32
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Unpaid => "unpaid",
            Self::Paid => "paid",
            Self::Refunded => "refunded",
            Self::PartiallyRefunded => "partially_refunded",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum FulfillmentStatus {
    Unfulfilled = 1,
    Fulfilled = 2,
    Partial = 3,
}

impl FulfillmentStatus {
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::Unfulfilled),
            2 => Some(Self::Fulfilled),
            3 => Some(Self::Partial),
            _ => None,
        }
    }

    pub fn to_i32(&self) -> i32 {
        *self as i32
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Unfulfilled => "unfulfilled",
            Self::Fulfilled => "fulfilled",
            Self::Partial => "partial",
        }
    }
}
