use atat::atat_derive::{AtatCmd, AtatEnum, AtatResp};
#[cfg(feature = "defmt")]
use defmt::Format;
use heapless::String;

use super::NoResponse;

#[cfg_attr(feature = "defmt", derive(Format))]
#[derive(Clone, AtatCmd)]
#[at_cmd("+UGPS", NoResponse)]
pub struct SetGps {
    #[at_arg(position = 0)]
    pub on: bool,
    #[at_arg(position = 1, len = 6)]
    pub gps_mode: GpsMode,
}

#[cfg_attr(feature = "defmt", derive(Format))]
#[derive(Debug, Clone, PartialEq, Eq, AtatEnum)]
pub enum GpsMode {
    NoAiding = 0,
    AutomaticLocalAiding = 1,
    AssistNowOffline = 2,
    AssistNowOnline = 4,
    AssistNowAutonomous = 8,
}

#[cfg_attr(feature = "defmt", derive(Format))]
#[derive(Clone, AtatCmd)]
#[at_cmd("+UGIND", NoResponse)]
pub struct SetGpsUrc {
    #[at_arg(position = 0)]
    pub on: bool,
}

#[cfg_attr(feature = "defmt", derive(Format))]
#[derive(Debug, Clone, AtatResp)]
pub struct GpsUrc {
    #[at_arg(position = 0)]
    pub aid_mode: GpsMode,
    #[at_arg(position = 1)]
    pub result: GpsResult,
}

#[cfg_attr(feature = "defmt", derive(Format))]
#[derive(Debug, Clone, PartialEq, Eq, AtatEnum)]
pub enum GpsResult {
    NoError = 0,
    WrongUrl = 1,
    HttpError = 2,
    CreateSocketError = 3,
    CloseSocketError = 4,
    WriteToSocketError = 5,
    ReadFromSocketError = 6,
    ConnectionDnsError = 7,
    FileSystemError = 8,
    GenericError = 9,
    NoAnswerFromGnss = 10,
    DataCollectionInProgress = 11,
    GnssConfigurationFailed = 12,
    RtcCalibrationFailed = 13,
    FeatureNotSupported = 14,
    FeaturePartiallySupported = 15,
    AuthenticationTokenMissing = 16,
}

#[cfg_attr(feature = "defmt", derive(Format))]
#[derive(Debug, Clone, AtatResp)]
pub struct UlocUrc {
    #[at_arg(position = 0)]
    pub date: String<10>,
    #[at_arg(position = 1)]
    pub time: String<12>,
    #[at_arg(position = 2)]
    pub lat: String<12>,
    #[at_arg(position = 3)]
    pub lon: String<13>,
    #[at_arg(position = 4)]
    pub alt: u32,
    #[at_arg(position = 5)]
    pub uncertainty: u32,
}
