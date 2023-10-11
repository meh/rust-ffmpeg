use crate::ffi::{AVFieldOrder::*, *};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde_derive::Serialize, serde_derive::Deserialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde_", rename_all = "lowercase"))]
pub enum FieldOrder {
	Unknown,
	Progressive,
	TT,
	BB,
	TB,
	BT,
}

impl From<AVFieldOrder> for FieldOrder {
	fn from(value: AVFieldOrder) -> Self {
		match value {
			AV_FIELD_UNKNOWN => FieldOrder::Unknown,
			AV_FIELD_PROGRESSIVE => FieldOrder::Progressive,
			AV_FIELD_TT => FieldOrder::TT,
			AV_FIELD_BB => FieldOrder::BB,
			AV_FIELD_TB => FieldOrder::TB,
			AV_FIELD_BT => FieldOrder::BT,
		}
	}
}

impl Into<AVFieldOrder> for FieldOrder {
	fn into(self) -> AVFieldOrder {
		match self {
			FieldOrder::Unknown => AV_FIELD_UNKNOWN,
			FieldOrder::Progressive => AV_FIELD_PROGRESSIVE,
			FieldOrder::TT => AV_FIELD_TT,
			FieldOrder::BB => AV_FIELD_BB,
			FieldOrder::TB => AV_FIELD_TB,
			FieldOrder::BT => AV_FIELD_BT,
		}
	}
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod tests {
	use crate::FieldOrder;

	#[test]
	fn field_order() {
		assert_eq!(serde_json::to_string(&FieldOrder::TT).unwrap(), "\"tt\"");
		assert_eq!(serde_json::to_string(&FieldOrder::BB).unwrap(), "\"bb\"");
		assert_eq!(serde_json::to_string(&FieldOrder::TB).unwrap(), "\"tb\"");
		assert_eq!(serde_json::to_string(&FieldOrder::BT).unwrap(), "\"bt\"");
		assert_eq!(serde_json::to_string(&FieldOrder::Unknown).unwrap(), "\"unknown\"");
		assert_eq!(
			serde_json::to_string(&FieldOrder::Progressive).unwrap(),
			"\"progressive\""
		);
	}
}
