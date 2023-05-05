use esp_idf_sys::*;
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};

/// Bluetooth Device address type
pub enum BLEAddressType {
  Public = BLE_ADDR_PUBLIC as _,
  Random = BLE_ADDR_RANDOM as _,
  PublicID = BLE_ADDR_PUBLIC_ID as _,
  RandomID = BLE_ADDR_RANDOM_ID as _,
}

#[derive(Copy, Clone)]
pub struct BLEAddress {
  pub(crate) value: esp_idf_sys::ble_addr_t,
}

impl BLEAddress {
  pub fn new(val: [u8; 6], addr_type: BLEAddressType) -> Self {
    let mut ret = Self {
      value: esp_idf_sys::ble_addr_t {
        val,
        type_: addr_type as _,
      },
    };
    ret.value.val.reverse();
    ret
  }
}

impl From<esp_idf_sys::ble_addr_t> for BLEAddress {
  fn from(value: esp_idf_sys::ble_addr_t) -> Self {
    Self { value }
  }
}

impl core::fmt::Display for BLEAddress {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(
      f,
      "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
      self.value.val[5],
      self.value.val[4],
      self.value.val[3],
      self.value.val[2],
      self.value.val[1],
      self.value.val[0]
    )
  }
}

impl core::fmt::Debug for BLEAddress {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(f, "{self}")
  }
}

impl Serialize for BLEAddress {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut bytes = [0; 7];
    bytes[0] = self.value.type_ as u8;
    bytes[1..].copy_from_slice(&self.value.val);
    serializer.serialize_bytes(&bytes)
  }
}

impl<'de> Deserialize<'de> for BLEAddress {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct AddrVisitor;

    impl<'de> Visitor<'de> for AddrVisitor {
      type Value = BLEAddress;

      fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("8 bytes")
      }

      fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        let ty = match v[0] {
          _ if v[0] == BLEAddressType::Public as u8 => BLEAddressType::Public,
          _ if v[0] == BLEAddressType::Random as u8 => BLEAddressType::RandomID,
          _ if v[0] == BLEAddressType::PublicID as u8 => BLEAddressType::PublicID,
          _ if v[0] == BLEAddressType::RandomID as u8 => BLEAddressType::RandomID,
          _ => return Err(E::custom("invalid BLEAddressType")),
        };
        let val = <[u8; 6]>::try_from(&v[1..7])
          .map_err(|_| E::custom("unexpected end of byte sequence"))?;
        Ok(BLEAddress::new(val, ty))
      }
    }

    deserializer.deserialize_bytes(AddrVisitor)
  }
}
